use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use pixie_node::PixelNode;
use crate::render::device::GpuContext;
use crate::render::pipeline::QuadPipeline;
use crate::render::quad::QuadBatch;
use crate::text::GlyphAtlas;
use cosmic_text::{FontSystem, SwashCache};

pub trait PixieApp {
    fn build(&mut self, width: u32, height: u32) -> PixelNode;
}

pub struct App<T: PixieApp> {
    content: T,
    instance: Option<wgpu::Instance>,
    window: Option<Arc<Window>>,
    gpu: Option<GpuContext>,
    // Deliberately separate from `gpu` — the GPU device can outlive
    // a single foreground session, but on Android the native window
    // (and therefore the Surface built from it) genuinely cannot.
    // Android can hand out a NEW native window on every resume, most
    // commonly right when the splash screen hands off to real
    // content — treating the Surface as "create once, keep forever"
    // is what caused the crash loop this fixes.
    surface: Option<wgpu::Surface<'static>>,
    quad_pipeline: Option<QuadPipeline>,
    batch: QuadBatch,
    font_system: FontSystem,
    swash_cache: SwashCache,
    atlas: GlyphAtlas,
}

impl<T: PixieApp> App<T> {
    pub fn new(content: T) -> Self {
        App {
            content,
            instance: None,
            window: None, gpu: None, surface: None, quad_pipeline: None,
            batch: QuadBatch::new(),
            font_system: crate::text::new_font_system(),
            swash_cache: SwashCache::new(),
            atlas: GlyphAtlas::new(512, 512),
        }
    }

    fn rebuild(&mut self, width: u32, height: u32) {
        let tree = self.content.build(width, height);

        let mut bridge = crate::layout::TreeBridge::new();
        let root_id = bridge.build(&tree, &mut self.font_system);
        bridge.compute(
            root_id,
            taffy::prelude::Size {
                width: taffy::prelude::AvailableSpace::Definite(width as f32),
                height: taffy::prelude::AvailableSpace::Definite(height as f32),
            },
        );

        self.batch.clear();
        crate::render::paint::paint_tree(
            &tree, &bridge, root_id, 0.0, 0.0,
            &mut self.batch, &mut self.font_system, &mut self.swash_cache, &mut self.atlas,
        );
    }
}

impl<T: PixieApp> ApplicationHandler for App<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Window creation is idempotent — reuse it if this is a
        // resume after a suspend, not a truly fresh launch.
        let window = match &self.window {
            Some(w) => Arc::clone(w),
            None => {
                let attrs = Window::default_attributes().with_title("Pixie");
                let w = Arc::new(event_loop.create_window(attrs).expect("failed to create window"));
                self.window = Some(Arc::clone(&w));
                w
            }
        };

        // Instance persists once created — only the Surface gets
        // rebuilt every resume, since it's the piece tied to a
        // specific native window that Android can swap out.
        if self.instance.is_none() {
            self.instance = Some(wgpu::Instance::new(&wgpu::InstanceDescriptor::default()));
        }
        let instance = self.instance.as_ref().unwrap();

        let surface = instance
            .create_surface(Arc::clone(&window))
            .expect("failed to create GPU surface");

        // GPU device also persists once created — request_adapter
        // needs a compatible surface just to pick hardware, it
        // doesn't tie the Device itself to that specific surface.
        if self.gpu.is_none() {
            self.gpu = Some(pollster::block_on(GpuContext::new(&surface)));
        }
        let gpu = self.gpu.as_ref().unwrap();

        let size = window.inner_size();
        let format = gpu.configure_surface(&surface, size.width, size.height);

        if self.quad_pipeline.is_none() {
            self.quad_pipeline = Some(QuadPipeline::new(&gpu.device, format));
        }

        self.surface = Some(surface);

        self.rebuild(size.width, size.height);

        if let (Some(gpu), Some(pipeline)) = (self.gpu.as_ref(), self.quad_pipeline.as_ref()) {
            pipeline.upload_atlas(&gpu.queue, &self.atlas);
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    /// The native window is going away — drop the Surface built from
    /// it immediately, since using a Surface after this point is
    /// exactly the dangling-handle bug this whole fix addresses.
    /// Everything else (GPU device, pipeline, window handle itself)
    /// survives, since only the underlying native surface is invalid.
    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.surface = None;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let (Some(gpu), Some(surface)) = (self.gpu.as_ref(), self.surface.as_ref()) {
                    gpu.configure_surface(surface, size.width, size.height);
                }
                self.rebuild(size.width, size.height);
                if let (Some(gpu), Some(pipeline)) = (self.gpu.as_ref(), self.quad_pipeline.as_ref()) {
                    pipeline.upload_atlas(&gpu.queue, &self.atlas);
                }
            }
            WindowEvent::RedrawRequested => {
                let size = self.window.as_ref().map(|w| w.inner_size());

                if let (Some(gpu), Some(surface), Some(pipeline), Some(size)) =
                    (self.gpu.as_ref(), self.surface.as_ref(), self.quad_pipeline.as_mut(), size)
                {
                    pipeline.upload(&gpu.device, &gpu.queue, &self.batch, size.width as f32, size.height as f32);

                    // If the surface is stale (window swapped without
                    // us catching a suspended() call yet), just skip
                    // this frame instead of crashing — the next
                    // resumed() will rebuild it properly.
                    let frame = match surface.get_current_texture() {
                        Ok(f) => f,
                        Err(_) => return,
                    };
                    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                    {
                        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("pixie pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.02, g: 0.05, b: 0.2, a: 1.0 }),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            timestamp_writes: None,
                            occlusion_query_set: None,
                        });
                        pipeline.draw(&mut pass);
                    }
                    gpu.queue.submit(Some(encoder.finish()));
                    frame.present();
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

pub fn run_desktop<T: PixieApp>(content: T) {
    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(content);
    event_loop.run_app(&mut app).expect("event loop error");
}

#[cfg(target_os = "android")]
pub fn run_android<T: PixieApp>(content: T, android_app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let event_loop = EventLoop::builder()
        .with_android_app(android_app)
        .build()
        .expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(content);
    event_loop.run_app(&mut app).expect("event loop error");
}
