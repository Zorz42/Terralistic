use anyhow::{bail, Result};

/// This compiles fragment and vertex shader and returns the compiled shader program
pub fn compile_shader(vertex_code: &str, fragment_code: &str) -> Result<u32> {
    // Safety: This is safe because we are using the opengl api correctly
    unsafe {
        let vertex_id = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment_id = gl::CreateShader(gl::FRAGMENT_SHADER);

        let mut temp = std::ffi::CString::new(vertex_code)?;
        gl::ShaderSource(vertex_id, 1, &temp.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_id);

        // Check for vertex shader compile errors
        let mut success = 0;
        gl::GetShaderiv(vertex_id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(vertex_id, gl::INFO_LOG_LENGTH, &mut len);

            // create a buffer with the correct size
            let mut buffer = vec![0; len as usize];

            gl::GetShaderInfoLog(
                vertex_id,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr().cast::<i8>(),
            );

            bail!(
                "Vertex shader compilation failed: {}",
                std::str::from_utf8(&buffer)?
            );
        }

        temp = std::ffi::CString::new(fragment_code)?;
        gl::ShaderSource(fragment_id, 1, &temp.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_id);

        // Check for fragment shader compile errors
        success = 0;
        gl::GetShaderiv(fragment_id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(fragment_id, gl::INFO_LOG_LENGTH, &mut len);

            // create a buffer with the correct size
            let mut buffer = vec![0; len as usize];

            gl::GetShaderInfoLog(
                fragment_id,
                len,
                std::ptr::null_mut(),
                buffer.as_mut_ptr().cast::<i8>(),
            );
            bail!(
                "Fragment shader compilation failed: {}",
                std::str::from_utf8(&buffer)?
            );
        }

        let program_id = gl::CreateProgram();

        gl::AttachShader(program_id, vertex_id);
        gl::AttachShader(program_id, fragment_id);
        gl::LinkProgram(program_id);

        gl::DetachShader(program_id, vertex_id);
        gl::DetachShader(program_id, fragment_id);

        gl::DeleteShader(vertex_id);
        gl::DeleteShader(fragment_id);

        Ok(program_id)
    }
}
