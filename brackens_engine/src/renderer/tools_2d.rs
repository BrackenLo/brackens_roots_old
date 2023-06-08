//===============================================================

use brackens_assets::Handle;
use brackens_renderer::{renderer_2d::RendererTexture, wgpu};
use shipyard::{Borrow, EntitiesViewMut, EntityId, IntoBorrow, UniqueView, UniqueViewMut, ViewMut};

use crate::{
    assets::AssetStorage,
    core_components::{Device, Queue},
    prelude::Transform,
    spatial_tools::TransformBundleViewMut,
};

use super::{
    components::Visible,
    components_2d::{Texture, TextureRenderer},
};

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

pub struct LoadTextureDescriptor<'a> {
    pub label: &'a str,
    pub path: &'a str,
    pub sampler: Option<wgpu::SamplerDescriptor<'a>>,
}

//--------------------------------------------------

pub fn load_texture(
    data: LoadTextureDescriptor,
    mut texture_storage: UniqueViewMut<AssetStorage<RendererTexture>>,
    renderer: UniqueView<TextureRenderer>,
    device: UniqueView<Device>,
    queue: UniqueView<Queue>,
) -> Handle<RendererTexture> {
    let layout = renderer.get_layout();
    let sampler = match data.sampler {
        Some(val) => val,
        None => wgpu::SamplerDescriptor::default(),
    };

    let loaded_texture =
        RendererTexture::from_file(&device.0, &queue.0, data.path, data.label, &sampler, layout)
            .unwrap();
    texture_storage.add_asset(loaded_texture)
}

pub fn load_texture_custom_layout(
    data: (LoadTextureDescriptor, &wgpu::BindGroupLayout),
    mut texture_storage: UniqueViewMut<AssetStorage<RendererTexture>>,
    device: UniqueView<Device>,
    queue: UniqueView<Queue>,
) -> Handle<RendererTexture> {
    let sampler = match data.0.sampler {
        Some(val) => val,
        None => wgpu::SamplerDescriptor::default(),
    };

    let loaded_texture = RendererTexture::from_file(
        &device.0,
        &queue.0,
        data.0.path,
        data.0.label,
        &sampler,
        data.1,
    )
    .unwrap();

    texture_storage.add_asset(loaded_texture)
}

//===============================================================

pub struct BlankTextureDescriptor<'a> {
    pub label: &'a str,
    pub color: [f32; 3],
    pub sampler: Option<wgpu::SamplerDescriptor<'a>>,
}
impl<'a> Default for BlankTextureDescriptor<'a> {
    fn default() -> Self {
        Self {
            label: "Blank Texture",
            color: [1., 1., 1.],
            sampler: None,
        }
    }
}

//--------------------------------------------------

/// Run using world.run_with_data where data is a BlankTextureDescriptor struct
pub fn load_blank_texture(
    data: BlankTextureDescriptor,
    mut texture_storage: UniqueViewMut<AssetStorage<RendererTexture>>,
    renderer: UniqueView<TextureRenderer>,
    device: UniqueView<Device>,
    queue: UniqueView<Queue>,
) -> Handle<RendererTexture> {
    let sampler = match data.sampler {
        Some(val) => val,
        None => wgpu::SamplerDescriptor::default(),
    };
    let layout = renderer.get_layout();

    let r = (data.color[0].clamp(0., 1.) * 255.) as u8;
    let g = (data.color[1].clamp(0., 1.) * 255.) as u8;
    let b = (data.color[2].clamp(0., 1.) * 255.) as u8;

    let mut rgb = brackens_renderer::image::RgbImage::new(11, 1);

    for pixel in rgb.pixels_mut() {
        pixel.0[0] = r;
        pixel.0[1] = g;
        pixel.0[2] = b;
    }
    let rgba = brackens_renderer::image::DynamicImage::from(rgb);

    let loaded_texture = RendererTexture::from_image(
        &device.0,
        &queue.0,
        &rgba,
        Some(data.label.as_ref()),
        &sampler,
        layout,
    )
    .unwrap();

    texture_storage.add_asset(loaded_texture)
}

/// Run using world.run_with_data where data is a tuple with a BlankTextureDescriptor struct with a bind group layout
pub fn load_blank_texture_custom_layout(
    data: (BlankTextureDescriptor, &wgpu::BindGroupLayout),
    mut texture_storage: UniqueViewMut<AssetStorage<RendererTexture>>,
    device: UniqueView<Device>,
    queue: UniqueView<Queue>,
) -> Handle<RendererTexture> {
    let sampler = match data.0.sampler {
        Some(val) => val,
        None => wgpu::SamplerDescriptor::default(),
    };

    let r = (data.0.color[0].clamp(0., 1.) * 255.) as u8;
    let g = (data.0.color[1].clamp(0., 1.) * 255.) as u8;
    let b = (data.0.color[2].clamp(0., 1.) * 255.) as u8;

    let mut rgb = brackens_renderer::image::RgbImage::new(11, 1);

    for pixel in rgb.pixels_mut() {
        pixel.0[0] = r;
        pixel.0[1] = g;
        pixel.0[2] = b;
    }
    let rgba = brackens_renderer::image::DynamicImage::from(rgb);

    let loaded_texture = RendererTexture::from_image(
        &device.0,
        &queue.0,
        &rgba,
        Some(data.0.label.as_ref()),
        &sampler,
        data.1,
    )
    .unwrap();

    texture_storage.add_asset(loaded_texture)
}

//===============================================================
