//! 面ごとに色を付けた立方体のメッシュと描画。

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use glow::HasContext;

use crate::shader::create_program;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 3],
}

pub struct Cube {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    index_count: i32,
    model_location: glow::UniformLocation,
    view_projection_location: glow::UniformLocation,
}

impl Cube {
    pub fn new(gl: &glow::Context) -> Result<Self, String> {
        let program = create_program(
            gl,
            include_str!("../shaders/cube.vert"),
            include_str!("../shaders/cube.frag"),
        )?;
        let (vertices, indices) = build_mesh();

        unsafe {
            let model_location = gl
                .get_uniform_location(program, "u_model")
                .ok_or("uniform u_model が見つかりません")?;
            let view_projection_location = gl
                .get_uniform_location(program, "u_view_projection")
                .ok_or("uniform u_view_projection が見つかりません")?;

            let vertex_array = gl.create_vertex_array()?;
            gl.bind_vertex_array(Some(vertex_array));

            let vertex_buffer = gl.create_buffer()?;
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertices),
                glow::STATIC_DRAW,
            );

            // インデックスバッファの紐付けは VAO に記録される。
            let index_buffer = gl.create_buffer()?;
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indices),
                glow::STATIC_DRAW,
            );

            let stride = size_of::<Vertex>() as i32;
            let offsets = [
                std::mem::offset_of!(Vertex, position),
                std::mem::offset_of!(Vertex, normal),
                std::mem::offset_of!(Vertex, color),
            ];
            for (location, offset) in offsets.into_iter().enumerate() {
                gl.enable_vertex_attrib_array(location as u32);
                gl.vertex_attrib_pointer_f32(
                    location as u32,
                    3,
                    glow::FLOAT,
                    false,
                    stride,
                    offset as i32,
                );
            }

            gl.bind_vertex_array(None);

            Ok(Self {
                program,
                vertex_array,
                index_count: indices.len() as i32,
                model_location,
                view_projection_location,
            })
        }
    }

    pub fn draw(&self, gl: &glow::Context, model: Mat4, view_projection: Mat4) {
        unsafe {
            gl.use_program(Some(self.program));
            gl.uniform_matrix_4_f32_slice(
                Some(&self.model_location),
                false,
                &model.to_cols_array(),
            );
            gl.uniform_matrix_4_f32_slice(
                Some(&self.view_projection_location),
                false,
                &view_projection.to_cols_array(),
            );
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_SHORT, 0);
            gl.bind_vertex_array(None);
        }
    }
}

/// 法線を面ごとに分けるため、1面4頂点 × 6面 = 24頂点で作る。
fn build_mesh() -> (Vec<Vertex>, Vec<u16>) {
    // (面の法線, 面に沿った軸, 色)
    let faces = [
        (Vec3::X, Vec3::NEG_Z, [0.91, 0.30, 0.24]),
        (Vec3::NEG_X, Vec3::Z, [0.20, 0.60, 0.86]),
        (Vec3::Y, Vec3::X, [0.18, 0.80, 0.44]),
        (Vec3::NEG_Y, Vec3::X, [0.95, 0.77, 0.06]),
        (Vec3::Z, Vec3::X, [0.61, 0.35, 0.71]),
        (Vec3::NEG_Z, Vec3::NEG_X, [0.90, 0.49, 0.13]),
    ];

    let mut vertices = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    for (normal, u, color) in faces {
        // u × v = normal となる v を取ると、外から見て反時計回りになる。
        let v = normal.cross(u);
        let base = vertices.len() as u16;

        for (su, sv) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            let position = (normal + u * su + v * sv) * 0.5;
            vertices.push(Vertex {
                position: position.to_array(),
                normal: normal.to_array(),
                color,
            });
        }
        indices.extend([0, 1, 2, 0, 2, 3].map(|i| base + i));
    }

    (vertices, indices)
}
