//! winit のウィンドウと OpenGL コンテキストをまとめて管理する。

use std::num::NonZeroU32;

use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, GlProfile, NotCurrentGlContext, PossiblyCurrentContext,
    Version,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow as _};
use raw_window_handle::HasWindowHandle;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

/// フィールドは宣言順に破棄されるので、ウィンドウに依存するものを先に並べる。
pub struct GlWindow {
    pub gl: glow::Context,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    pub window: Window,
}

impl GlWindow {
    pub fn new(event_loop: &ActiveEventLoop, title: &str) -> Self {
        let window_attributes = Window::default_attributes()
            .with_title(title)
            .with_inner_size(LogicalSize::new(960.0, 640.0));

        let template = ConfigTemplateBuilder::new().with_depth_size(24);
        let (window, config) = DisplayBuilder::new()
            .with_window_attributes(Some(window_attributes))
            .build(event_loop, template, pick_config)
            .expect("ウィンドウの作成に失敗しました");
        let window = window.expect("ウィンドウが作成されませんでした");

        let display = config.display();
        let raw_handle = window
            .window_handle()
            .expect("ウィンドウハンドルの取得に失敗しました")
            .as_raw();

        // macOS で使える OpenGL は 4.1 Core Profile が上限。
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 1))))
            .with_profile(GlProfile::Core)
            .build(Some(raw_handle));
        let context = unsafe { display.create_context(&config, &context_attributes) }
            .expect("OpenGL コンテキストの作成に失敗しました");

        let surface_attributes = window
            .build_surface_attributes(Default::default())
            .expect("サーフェス属性の作成に失敗しました");
        let surface = unsafe { display.create_window_surface(&config, &surface_attributes) }
            .expect("ウィンドウサーフェスの作成に失敗しました");

        let context = context
            .make_current(&surface)
            .expect("OpenGL コンテキストを有効化できませんでした");

        // 垂直同期。失敗しても描画はできるので警告だけ出す。
        if let Err(error) = surface.set_swap_interval(&context, SwapInterval::Wait(NonZeroU32::MIN))
        {
            eprintln!("垂直同期を有効化できませんでした: {error}");
        }

        let gl = unsafe {
            glow::Context::from_loader_function_cstr(|symbol| display.get_proc_address(symbol))
        };

        Self {
            gl,
            surface,
            context,
            window,
        }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        if let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        {
            self.surface.resize(&self.context, width, height);
        }
    }

    pub fn swap_buffers(&self) {
        self.surface
            .swap_buffers(&self.context)
            .expect("バッファの入れ替えに失敗しました");
    }
}

/// 候補のうち、マルチサンプル数が最も多い設定を選ぶ。
fn pick_config(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .max_by_key(glutin::config::GlConfig::num_samples)
        .expect("利用できる OpenGL 設定がありません")
}
