mod cube;
mod gl_window;
mod shader;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use glam::camera::rh::proj::opengl::perspective;
use glam::camera::rh::view::look_at_mat4;
use glam::{Mat4, Vec3};
use glow::HasContext;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowId;

use cube::Cube;
use gl_window::GlWindow;

/// `--screenshot` 指定時に保存するフレームの既定値。`--frame` で変更できる。
const DEFAULT_SCREENSHOT_FRAME: u32 = 30;

struct Scene {
    gl_window: GlWindow,
    cube: Cube,
    started_at: Instant,
    frame: u32,
}

impl Scene {
    fn new(event_loop: &ActiveEventLoop) -> Self {
        let gl_window = GlWindow::new(event_loop, "poc-rust-opengl");
        let gl = &gl_window.gl;

        unsafe {
            println!("OpenGL: {}", gl.get_parameter_string(glow::VERSION));
            println!("GPU:    {}", gl.get_parameter_string(glow::RENDERER));
            gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::CULL_FACE);
            gl.clear_color(0.08, 0.09, 0.12, 1.0);
        }

        let cube = Cube::new(gl).expect("立方体の初期化に失敗しました");

        Self {
            gl_window,
            cube,
            started_at: Instant::now(),
            frame: 0,
        }
    }

    fn render(&mut self) {
        let gl = &self.gl_window.gl;
        let size = self.gl_window.size();
        let aspect = size.width as f32 / size.height.max(1) as f32;
        let time = self.started_at.elapsed().as_secs_f32();

        let model = Mat4::from_rotation_y(time) * Mat4::from_rotation_x(time * 0.6);
        let view = look_at_mat4(Vec3::new(0.0, 0.8, 3.0), Vec3::ZERO, Vec3::Y);
        let projection = perspective(45f32.to_radians(), aspect, 0.1, 100.0);

        unsafe {
            gl.viewport(0, 0, size.width as i32, size.height as i32);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
        self.cube.draw(gl, model, projection * view);
        self.frame += 1;
    }
}

struct App {
    scene: Option<Scene>,
    screenshot_path: Option<PathBuf>,
    screenshot_frame: u32,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.scene.is_none() {
            self.scene = Some(Scene::new(event_loop));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some(scene) = self.scene.as_mut() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::Resized(size) => scene.gl_window.resize(size),
            WindowEvent::RedrawRequested => {
                scene.render();

                if scene.frame == self.screenshot_frame
                    && let Some(path) = &self.screenshot_path
                {
                    let size = scene.gl_window.size();
                    save_ppm(&scene.gl_window.gl, size.width, size.height, path)
                        .expect("スクリーンショットの保存に失敗しました");
                    println!("保存しました: {}", path.display());
                    event_loop.exit();
                }

                scene.gl_window.swap_buffers();
                scene.gl_window.window.request_redraw();
            }
            _ => {}
        }
    }
}

/// 現在のフレームバッファを PPM 形式で保存する。
fn save_ppm(gl: &glow::Context, width: u32, height: u32, path: &Path) -> std::io::Result<()> {
    let row_size = width as usize * 3;
    let mut pixels = vec![0u8; row_size * height as usize];

    unsafe {
        gl.pixel_store_i32(glow::PACK_ALIGNMENT, 1);
        gl.read_pixels(
            0,
            0,
            width as i32,
            height as i32,
            glow::RGB,
            glow::UNSIGNED_BYTE,
            glow::PixelPackData::Slice(Some(&mut pixels)),
        );
    }

    let mut file = BufWriter::new(File::create(path)?);
    write!(file, "P6\n{width} {height}\n255\n")?;
    // OpenGL の原点は左下なので、下の行から順に書き出す。
    for row in pixels.chunks_exact(row_size).rev() {
        file.write_all(row)?;
    }
    file.flush()
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut app = App {
        scene: None,
        screenshot_path: None,
        screenshot_frame: DEFAULT_SCREENSHOT_FRAME,
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--screenshot" => {
                let path = args.next().expect("--screenshot には保存先が必要です");
                app.screenshot_path = Some(path.into());
            }
            "--frame" => {
                app.screenshot_frame = args
                    .next()
                    .and_then(|frame| frame.parse().ok())
                    .expect("--frame にはフレーム番号が必要です");
            }
            other => panic!("不明な引数です: {other}"),
        }
    }

    let event_loop = EventLoop::new().expect("イベントループの作成に失敗しました");
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut app)
        .expect("実行中にエラーが発生しました");
}
