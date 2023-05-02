//===============================================================

use std::collections::{HashMap, HashSet};

use brackens_renderer::{
    bytemuck, render_tools,
    texture_renderer::{self, RawTextureInstance},
    wgpu::{self, util::DeviceExt},
};

use brackens_assets::{Handle, HandleID};

use brackens_tools::{glam::Vec2, winit::dpi::PhysicalSize};
use shipyard::{Component, Unique};

pub use brackens_renderer::{
    texture::LoadedTexture, texture_renderer::TextureDrawCall as FinalTextureDrawCall,
};

//===============================================================

#[derive(Unique)]
pub struct RenderPassTools(pub(crate) render_tools::RenderPassTools);

#[derive(Unique)]
pub struct ClearColor(pub [f64; 3]);

//===============================================================

#[derive(Unique)]
pub struct TextureRenderer {
    renderer: texture_renderer::TextureRenderer,

    should_render: HashSet<HandleID>,
    unprocessed_draw_data: HashMap<HandleID, Vec<RawTextureInstance>>,

    texture_data: HashMap<HandleID, Handle<LoadedTexture>>,
    draw_data: HashMap<HandleID, FinalTextureDrawCall>,
}

impl TextureRenderer {
    //--------------------------------------------------

    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            renderer: texture_renderer::TextureRenderer::new(device, config.format),
            should_render: HashSet::new(),
            unprocessed_draw_data: HashMap::new(),

            texture_data: HashMap::new(),
            draw_data: HashMap::new(),
        }
    }

    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        self.renderer.get_texture_layout()
    }

    //--------------------------------------------------

    pub(crate) fn resize(&mut self, queue: &wgpu::Queue, new_size: PhysicalSize<u32>) {
        let new_size = brackens_renderer::Size {
            width: new_size.width,
            height: new_size.height,
        };
        self.renderer.resize(queue, new_size)
    }

    //--------------------------------------------------

    pub(crate) fn add_texture(&mut self, handle: Handle<LoadedTexture>) {
        let handle = handle.clone_weak();
        self.texture_data.insert(handle.id(), handle);
    }

    pub(crate) fn remove_texture(&mut self, id: HandleID) {
        self.should_render.remove(&id);
        self.unprocessed_draw_data.remove(&id);

        self.texture_data.remove(&id);
        self.draw_data.remove(&id);
    }

    //--------------------------------------------------

    pub(crate) fn draw_texture(&mut self, handle_id: HandleID, instance: RawTextureInstance) {
        match self.unprocessed_draw_data.get_mut(&handle_id) {
            Some(val) => val.push(instance),
            None => {
                self.unprocessed_draw_data.insert(handle_id, vec![instance]);
            }
        };
    }

    //--------------------------------------------------

    pub(crate) fn process_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.should_render.clear();

        for unprocessed in self.unprocessed_draw_data.iter() {
            let data_count = unprocessed.1.len() as u32;

            if data_count == 0 {
                continue;
            }
            self.should_render.insert(unprocessed.0.clone());

            if let Some(instance) = self.draw_data.get_mut(&unprocessed.0) {
                // Buffer is too small to hold new data. Need to create bigger buffer
                if data_count > instance.instance_count {
                    let FinalTextureDrawCall {
                        instances,
                        instance_count,
                    } = Self::create_instance_buffer(device, unprocessed.1);

                    instance.instances = instances;
                    instance.instance_count = instance_count;
                    continue;
                } else {
                    // Buffer is big enough. Just write new data to it
                    queue.write_buffer(&instance.instances, 0, bytemuck::cast_slice(unprocessed.1));
                    continue;
                }
            }

            // Data doesn't exist yet. Create it and add it
            let instance = Self::create_instance_buffer(device, unprocessed.1);
            self.draw_data.insert(unprocessed.0.clone(), instance);
        }

        self.unprocessed_draw_data.clear();
    }

    fn create_instance_buffer(
        device: &wgpu::Device,
        data: &[RawTextureInstance],
    ) -> FinalTextureDrawCall {
        let instances = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Texture Instance Buffer")),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        FinalTextureDrawCall {
            instances,
            instance_count: data.len() as u32,
        }
    }

    //--------------------------------------------------

    pub(crate) fn render(&mut self, render_tools: &mut render_tools::RenderPassTools) {
        if self.should_render.len() == 0 {
            return;
        }

        let draw = self
            .should_render
            .iter()
            .map(|val| {
                let bind_group = &self.texture_data.get(val).unwrap().get().bind_group;
                let draw_data = self.draw_data.get(val).unwrap();

                (bind_group, draw_data)
            })
            .collect::<Vec<_>>();

        self.renderer.render(render_tools, &draw);
    }

    //--------------------------------------------------
}

//===============================================================

#[derive(Component)]
pub struct Visible {
    pub visible: bool,
}

#[derive(Component)]
pub struct Texture {
    pub size: Vec2,
    pub handle: Handle<LoadedTexture>,
    pub color: [f32; 4],
}

//===============================================================
