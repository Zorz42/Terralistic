//! Deferred deletion of GPU objects.
//!
//! Recording a frame instead of drawing it immediately opens a gap: a `Texture` or a
//! `VertexBuffer` can be dropped after a command referring to it has been recorded but
//! before that command runs. The toolkit does that routinely and always has -
//! `login.rs` builds a text texture inside `render_inner` and lets it fall out of scope at
//! the end, and every `RectArray` in the world renderer is replaced wholesale by
//! `self.rect_array = RectArray::new()` when its chunk changes. Deleting the OpenGL object
//! at that point would leave the recorded command pointing at nothing.
//!
//! So `Drop` parks the object's name here instead, and the backend deletes the whole batch
//! once the frame's commands have executed. The window is one frame, and the memory is
//! bounded by how much a single frame creates and throws away.
//!
//! This also closes a subtler hole. OpenGL reuses names as soon as they are freed, so an
//! immediate delete could hand the same name straight back to the next `glGenTextures` and
//! a stale command would silently draw *someone else's* texture rather than nothing at all.
//! Holding the name until the frame ends makes that impossible.
//!
//! A `Mutex` rather than a `thread_local!` because `Texture` is `Send`: the game only ever
//! touches OpenGL from the main thread, but nothing in the type system says a texture has to
//! be dropped there, and dropping one on a worker must not lose the name.

use std::sync::{Mutex, PoisonError};

/// Names waiting to be handed back to OpenGL, split by object type because each needs its
/// own delete call.
static PENDING_TEXTURES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
static PENDING_BUFFERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
static PENDING_VERTEX_ARRAYS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

pub(super) fn delete_texture_later(name: u32) {
    PENDING_TEXTURES.lock().unwrap_or_else(PoisonError::into_inner).push(name);
}

pub(super) fn delete_buffer_later(name: u32) {
    PENDING_BUFFERS.lock().unwrap_or_else(PoisonError::into_inner).push(name);
}

pub(super) fn delete_vertex_array_later(name: u32) {
    PENDING_VERTEX_ARRAYS.lock().unwrap_or_else(PoisonError::into_inner).push(name);
}

/// Hands every parked name back to OpenGL.
///
/// Only the backend calls this, and only once the frame's commands have run, because until
/// then some of these names are still being drawn from. It needs the OpenGL context to be
/// current on the calling thread.
pub(super) fn collect() {
    let textures = std::mem::take(&mut *PENDING_TEXTURES.lock().unwrap_or_else(PoisonError::into_inner));
    let buffers = std::mem::take(&mut *PENDING_BUFFERS.lock().unwrap_or_else(PoisonError::into_inner));
    let vertex_arrays = std::mem::take(&mut *PENDING_VERTEX_ARRAYS.lock().unwrap_or_else(PoisonError::into_inner));

    unsafe {
        if !textures.is_empty() {
            gl::DeleteTextures(textures.len() as i32, textures.as_ptr());
        }
        if !buffers.is_empty() {
            gl::DeleteBuffers(buffers.len() as i32, buffers.as_ptr());
        }
        if !vertex_arrays.is_empty() {
            gl::DeleteVertexArrays(vertex_arrays.len() as i32, vertex_arrays.as_ptr());
        }
    }
}

/// How many names are currently parked, for tests.
///
/// The counts are process wide, so a test may only assert that they *grew* by what it
/// dropped, never on an absolute value.
#[cfg(test)]
pub fn get_pending_counts() -> (usize, usize, usize) {
    (
        PENDING_TEXTURES.lock().unwrap_or_else(PoisonError::into_inner).len(),
        PENDING_BUFFERS.lock().unwrap_or_else(PoisonError::into_inner).len(),
        PENDING_VERTEX_ARRAYS.lock().unwrap_or_else(PoisonError::into_inner).len(),
    )
}
