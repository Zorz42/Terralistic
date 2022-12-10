/**
This compiles fragment and vertex shader and returns the compiled shader program
 */
pub(crate) fn compile_shader(vertex_code: &str, fragment_code: &str) -> u32 {
    unsafe {
        let vertex_id = gl::CreateShader(gl::VERTEX_SHADER);
        let fragment_id = gl::CreateShader(gl::FRAGMENT_SHADER);

        let temp = std::ffi::CString::new(vertex_code).unwrap();
        gl::ShaderSource(vertex_id, 1, &temp.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex_id);

        let temp = std::ffi::CString::new(fragment_code).unwrap();
        gl::ShaderSource(fragment_id, 1, &temp.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment_id);

        let program_id = gl::CreateProgram();

        gl::AttachShader(program_id, vertex_id);
        gl::AttachShader(program_id, fragment_id);
        gl::LinkProgram(program_id);

        gl::DetachShader(program_id, vertex_id);
        gl::DetachShader(program_id, fragment_id);

        gl::DeleteShader(vertex_id);
        gl::DeleteShader(fragment_id);

        program_id
    }
}