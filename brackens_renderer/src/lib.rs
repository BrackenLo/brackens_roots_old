//===============================================================

use log::info;

pub mod pipelines;
pub mod renderer_2d;
pub mod renderer_3d;

pub use bytemuck;
pub use image;
pub use wgpu;

//===============================================================

#[derive(Clone, Copy)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

//===============================================================

pub struct RenderPrefs {
    pub backends: wgpu::Backends,
    pub dx12_compiler: wgpu::Dx12Compiler,
    pub power_preferences: wgpu::PowerPreference,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
    pub present_mode: wgpu::PresentMode,
}
impl Default for RenderPrefs {
    fn default() -> Self {
        Self {
            backends: wgpu::Backends::all(),
            dx12_compiler: wgpu::Dx12Compiler::default(),
            power_preferences: wgpu::PowerPreference::default(),
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            present_mode: wgpu::PresentMode::default(),
        }
    }
}

//===============================================================

pub struct RenderComponents {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub size: Size<u32>,
}

impl RenderComponents {
    pub fn new<
        W: raw_window_handle::HasRawWindowHandle + raw_window_handle::HasRawDisplayHandle,
    >(
        prefs: RenderPrefs,
        window: &W,
        size: Size<u32>,
    ) -> Self {
        info!("Creating new wgpu components");

        //----------------------------------------------

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: prefs.backends,
            dx12_shader_compiler: prefs.dx12_compiler,
        });

        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter =
            match pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: prefs.power_preferences,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })) {
                Some(adapter) => adapter,
                None => panic!(
                "Error creating wgpu adapter. No adapter available that matches adapter options."
            ),
            };

        //----------------------------------------------

        let (device, queue) = match pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Default Device"),
                features: prefs.features,
                limits: prefs.limits,
            },
            None,
        )) {
            Ok(val) => val,
            Err(e) => panic!("Error creating wgpu device and queue: {}", e),
        };

        let capabilities = surface.get_capabilities(&adapter);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: capabilities.formats[0],
            width: size.width,
            height: size.height,
            present_mode: prefs.present_mode,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        //----------------------------------------------

        info!("Successfully created wgpu components");

        Self {
            device,
            queue,
            surface,
            config,
            size,
        }

        //----------------------------------------------
    }
}

//===============================================================

pub mod render_tools {
    use log::warn;

    use crate::Size;

    //===============================================================

    pub struct RenderPassTools {
        pub surface_texture: wgpu::SurfaceTexture,
        pub surface_view: wgpu::TextureView,
        pub encoder: wgpu::CommandEncoder,
    }

    //===============================================================

    pub fn start_render_pass(
        device: &wgpu::Device,
        surface: &wgpu::Surface,
    ) -> Result<RenderPassTools, wgpu::SurfaceError> {
        let surface_data = match surface.get_current_texture() {
            Ok(output) => {
                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                Ok((output, view))
            }
            Err(e) => Err(e),
        };

        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Command Encoder"),
        });

        match surface_data {
            Ok(surface_data) => Ok(RenderPassTools {
                surface_texture: surface_data.0,
                surface_view: surface_data.1,
                encoder,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn end_render_pass(queue: &wgpu::Queue, render_pass: RenderPassTools) {
        queue.submit(Some(render_pass.encoder.finish()));
        render_pass.surface_texture.present();
    }

    //===============================================================

    pub fn resize(
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        config: &mut wgpu::SurfaceConfiguration,
        new_size: Size<u32>,
    ) {
        if new_size.width > 0 && new_size.height > 0 {
            config.width = new_size.width;
            config.height = new_size.height;
            surface.configure(device, &config);
        } else {
            warn!(
                "Warning: Attempting to resize surface however provided dimensions are both zero."
            );
        }
    }

    pub fn clear_background(render_tools: &mut RenderPassTools, clear_color: [f64; 3]) {
        render_tools
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Color Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &render_tools.surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: clear_color[0],
                            g: clear_color[1],
                            b: clear_color[2],
                            a: 1.,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
    }

    //===============================================================
}

//===============================================================
