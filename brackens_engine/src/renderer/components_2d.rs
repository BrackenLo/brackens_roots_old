//===============================================================

use std::collections::HashMap;

use brackens_assets::Handle;
use brackens_renderer::{
    render_tools,
    renderer_2d::{self, tools::TextureProcessor, RawTextureInstance, RendererTexture, TextureID},
    wgpu, Size,
};
use brackens_tools::glam::{self, Vec2};
use shipyard::{Borrow, Component, EntitiesViewMut, EntityId, IntoBorrow, Unique, ViewMut};

use crate::{assets::AssetStorage, prelude::Transform, spatial_tools::TransformBundleViewMut};

use super::components::Visible;

//===============================================================

// Texture Rendering

#[derive(Unique)]
pub struct TextureRenderer {
    renderer: renderer_2d::TextureRenderer,
    processor: TextureProcessor,
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
            processor: TextureProcessor::default(),
        }
    }

    #[inline]
    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        self.renderer.get_texture_layout()
    }

    //--------------------------------------------------

    #[inline]
    pub(crate) fn resize_depth(&mut self, device: &wgpu::Device, new_size: Size<u32>) {
        self.renderer.resize_depth(device, new_size);
    }

    #[inline]
    pub(crate) fn resize_projection(&mut self, queue: &wgpu::Queue, matrix: &glam::Mat4) {
        self.renderer.set_projection(queue, matrix);
    }

    pub(crate) fn resize_both(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        new_size: Size<u32>,
    ) {
        self.renderer
            .resize_depth_projection(device, queue, new_size);
    }

    //--------------------------------------------------

    #[inline]
    pub(crate) fn get_unprocessed_mut(
        &mut self,
    ) -> &mut HashMap<TextureID, Vec<RawTextureInstance>> {
        self.processor.get_unprocessed_mut()
    }

    #[inline]
    pub(crate) fn process_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.processor.process_texture(device, queue);
    }

    //--------------------------------------------------

    pub(crate) fn render(
        &mut self,
        texture_storage: &AssetStorage<RendererTexture>,
        render_tools: &mut render_tools::RenderPassTools,
    ) {
        let draw = self
            .processor
            .get_draw_data()
            .iter()
            .map(|(id, buffer)| {
                let bind_group = &texture_storage.get_data(id).unwrap().bind_group;

                (bind_group, buffer)
            })
            .collect::<Vec<_>>();

        self.renderer.render(render_tools, &draw);

        // let draw = self
        //     .should_render
        //     .iter()
        //     .map(|val| {
        //         let bind_group = &self.texture_data.get(val).unwrap().get().bind_group;
        //         let draw_data = self.draw_data.get(val).unwrap();

        //         (bind_group, draw_data)
        //     })
        //     .collect::<Vec<_>>();

        // self.renderer.render(render_tools, &draw);
    }

    //--------------------------------------------------
}

//--------------------------------------------------
// Texture Components

#[derive(Component, Clone)]
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
    pub fn new_texture(
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
