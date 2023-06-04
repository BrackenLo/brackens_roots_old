//===============================================================

use std::collections::{HashMap, HashSet};

use brackens_assets::{Handle, HandleID};
use brackens_renderer::{
    bytemuck, render_tools,
    renderer_2d::{
        self, RawTextureInstance, RendererTexture, TextureDrawCall as FinalTextureDrawCall,
    },
    wgpu::{self, util::DeviceExt},
    Size,
};
use brackens_tools::glam::Vec2;
use shipyard::{Borrow, Component, EntitiesViewMut, EntityId, IntoBorrow, Unique, ViewMut};

use crate::{prelude::Transform, spatial_components::TransformBundleViewMut};

use super::components::Visible;

//===============================================================

// Texture Rendering

#[derive(Unique)]
pub struct TextureRenderer {
    renderer: renderer_2d::TextureRenderer,

    should_render: HashSet<HandleID<RendererTexture>>,
    unprocessed_draw_data: HashMap<HandleID<RendererTexture>, Vec<RawTextureInstance>>,

    texture_data: HashMap<HandleID<RendererTexture>, Handle<RendererTexture>>,
    draw_data: HashMap<HandleID<RendererTexture>, FinalTextureDrawCall>,
}

impl TextureRenderer {
    //--------------------------------------------------

    pub fn new(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        window_size: Size<u32>,
    ) -> Self {
        Self {
            renderer: renderer_2d::TextureRenderer::new(device, config.format, window_size),
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

    pub(crate) fn resize(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        new_size: Size<u32>,
    ) {
        self.renderer.resize(device, queue, new_size)
    }

    //--------------------------------------------------

    pub(crate) fn add_texture(&mut self, handle: Handle<RendererTexture>) {
        let handle = handle.clone_weak();
        self.texture_data.insert(handle.id(), handle);
    }

    pub(crate) fn remove_texture(&mut self, id: HandleID<RendererTexture>) {
        self.should_render.remove(&id);
        self.unprocessed_draw_data.remove(&id);

        self.texture_data.remove(&id);
        self.draw_data.remove(&id);
    }

    //--------------------------------------------------

    pub(crate) fn draw_texture(
        &mut self,
        handle_id: HandleID<RendererTexture>,
        instance: RawTextureInstance,
    ) {
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

//--------------------------------------------------
// Texture Components

#[derive(Component)]
pub struct Texture {
    pub size: Vec2,
    pub handle: Handle<RendererTexture>,
    pub color: [f32; 4],
}
impl Texture {
    pub fn new(handle: Handle<RendererTexture>, width: f32, height: f32) -> Self {
        Texture {
            size: Vec2::new(width, height),
            handle,
            color: [1., 1., 1., 1.],
        }
    }
    pub fn new_color(
        handle: Handle<RendererTexture>,
        width: f32,
        height: f32,
        color: [f32; 4],
    ) -> Self {
        Texture {
            size: Vec2::new(width, height),
            handle,
            color,
        }
    }
}

//===============================================================

pub struct TextureBundleViewMut<'v> {
    vm_transform_bundle: TransformBundleViewMut<'v>,
    vm_visible: ViewMut<'v, Visible>,
    vm_texture: ViewMut<'v, Texture>,
}
impl<'v> TextureBundleViewMut<'v> {
    pub fn create_texture(
        &mut self,
        entities: &mut EntitiesViewMut,
        transform: Transform,
        texture: Texture,
    ) -> EntityId {
        let id = self
            .vm_transform_bundle
            .create_transform(entities, transform);

        entities.add_component(
            id,
            (&mut self.vm_visible, &mut self.vm_texture),
            (Visible::default(), texture),
        );

        id
    }

    pub fn add_texture(
        &mut self,
        entities: &mut EntitiesViewMut,
        entity: EntityId,
        transform: Transform,
        texture: Texture,
    ) {
        self.vm_transform_bundle
            .add_transform(entities, entity, transform);

        entities.add_component(
            entity,
            (&mut self.vm_visible, &mut self.vm_texture),
            (Visible::default(), texture),
        );
    }
}

pub struct TextureBundleViewMutBorrower;
impl<'v> IntoBorrow for TextureBundleViewMut<'_> {
    type Borrow = TextureBundleViewMutBorrower;
}

type TextureBundleViewMutComponents<'v> = (
    TransformBundleViewMut<'v>,
    ViewMut<'v, Visible>,
    ViewMut<'v, Texture>,
);

impl<'v> Borrow<'v> for TextureBundleViewMutBorrower {
    type View = TextureBundleViewMut<'v>;

    fn borrow(
        world: &'v shipyard::World,
        last_run: Option<u32>,
        current: u32,
    ) -> Result<Self::View, shipyard::error::GetStorage> {
        let (vm_transform_bundle, vm_visible, vm_texture) =
            <TextureBundleViewMutComponents as IntoBorrow>::Borrow::borrow(
                world, last_run, current,
            )?;

        Ok(TextureBundleViewMut {
            vm_transform_bundle,
            vm_visible,
            vm_texture,
        })
    }
}

//===============================================================
