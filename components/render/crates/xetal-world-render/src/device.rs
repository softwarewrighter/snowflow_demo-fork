use crate::{pipeline, resources};
use web_sys::HtmlCanvasElement;

pub struct Renderer {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) pipeline: wgpu::RenderPipeline,
    pub(crate) uniform: wgpu::Buffer,
    pub(crate) group: wgpu::BindGroup,
    pub(crate) depth: wgpu::TextureView,
    pub(crate) shadow: xetal_world_shadow::Shadow,
    pub(crate) mesh: Option<resources::GpuMesh>,
}

impl Renderer {
    pub async fn new(canvas: HtmlCanvasElement, report: fn(String)) -> Result<Self, String> {
        let (surface, device, queue, config) = connect(&canvas).await?;
        device.on_uncaptured_error(Box::new(move |error| report(error.to_string())));
        let (uniform, group, layout) = resources::uniforms(&device);
        let shadow = xetal_world_shadow::Shadow::new(&device);
        let pipeline = pipeline::create(&device, config.format, &layout, &shadow.layout);
        let depth = resources::depth(&device, &config);
        Ok(Self {
            device,
            queue,
            surface,
            config,
            pipeline,
            uniform,
            group,
            depth,
            shadow,
            mesh: None,
        })
    }
}

type Connection = (
    wgpu::Surface<'static>,
    wgpu::Device,
    wgpu::Queue,
    wgpu::SurfaceConfiguration,
);

async fn connect(canvas: &HtmlCanvasElement) -> Result<Connection, String> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(|e| e.to_string())?;
    let adapter = adapter(&instance, &surface).await?;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .map_err(|e| e.to_string())?;
    let config = surface
        .get_default_config(&adapter, canvas.width().max(1), canvas.height().max(1))
        .ok_or("unsupported surface")?;
    surface.configure(&device, &config);
    Ok((surface, device, queue, config))
}

async fn adapter(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface<'_>,
) -> Result<wgpu::Adapter, String> {
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .ok_or_else(|| "WebGPU adapter unavailable".to_string())
}
