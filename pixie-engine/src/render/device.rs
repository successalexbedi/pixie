use wgpu::{Instance, InstanceDescriptor, DeviceDescriptor, RequestAdapterOptions};

pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter: wgpu::Adapter,
}

impl GpuContext {
    pub async fn new(surface: &wgpu::Surface<'_>) -> GpuContext {
        let instance = Instance::new(&InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("no compatible GPU adapter found");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await
            .expect("failed to create GPU device");

        GpuContext { device, queue, adapter }
    }

    /// Configures (or reconfigures, on resize) a surface against this
    /// device — picks a real sRGB format if the surface offers one,
    /// falls back to whatever it does offer otherwise. Returns the
    /// format chosen, since the render pipeline (once built) needs to
    /// know it.
    pub fn configure_surface(&self, surface: &wgpu::Surface, width: u32, height: u32) -> wgpu::TextureFormat {
        let caps = surface.get_capabilities(&self.adapter);
        let format = caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: width.max(1),
            height: height.max(1),
            present_mode: caps.present_modes[0],
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&self.device, &config);
        format
    }
}
