//! GLSL シェーダのコンパイルとリンク。

use glow::HasContext;

/// 頂点シェーダとフラグメントシェーダからプログラムを作る。
pub fn create_program(
    gl: &glow::Context,
    vertex_source: &str,
    fragment_source: &str,
) -> Result<glow::Program, String> {
    unsafe {
        let vertex = compile(gl, glow::VERTEX_SHADER, vertex_source)?;
        let fragment = compile(gl, glow::FRAGMENT_SHADER, fragment_source)?;

        let program = gl.create_program()?;
        gl.attach_shader(program, vertex);
        gl.attach_shader(program, fragment);
        gl.link_program(program);

        // リンク後はシェーダオブジェクトは不要。
        gl.detach_shader(program, vertex);
        gl.detach_shader(program, fragment);
        gl.delete_shader(vertex);
        gl.delete_shader(fragment);

        if gl.get_program_link_status(program) {
            Ok(program)
        } else {
            let log = gl.get_program_info_log(program);
            gl.delete_program(program);
            Err(format!("シェーダのリンクに失敗しました: {log}"))
        }
    }
}

unsafe fn compile(gl: &glow::Context, kind: u32, source: &str) -> Result<glow::Shader, String> {
    unsafe {
        let shader = gl.create_shader(kind)?;
        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if gl.get_shader_compile_status(shader) {
            Ok(shader)
        } else {
            let log = gl.get_shader_info_log(shader);
            gl.delete_shader(shader);
            Err(format!("シェーダのコンパイルに失敗しました: {log}"))
        }
    }
}
