//===============================================================

use brackens_assets::Handle;
use brackens_renderer::{
    renderer_2d::RendererTexture,
    renderer_3d::{RendererMaterial, RendererMesh},
};
use shipyard::{UniqueView, UniqueViewMut, World};

use crate::{
    assets::{AssetLoadable, AssetStorage},
    core_components::{Device, Queue},
};

use super::{components_2d::TextureRenderer, components_3d::Model};

//===============================================================

#[allow(unused)]
pub fn load_texture_bytes<T: AsRef<str>>(
    world: &mut World,
    bytes: &[u8],
    label: T,
    sampler: Option<brackens_renderer::wgpu::SamplerDescriptor>,
) -> Handle<RendererTexture> {
    world.run_with_data(
        |data: (&[u8], T),
         mut texture_storage: UniqueViewMut<AssetStorage<RendererTexture>>,
         renderer: UniqueView<TextureRenderer>,
         device: UniqueView<Device>,
         queue: UniqueView<Queue>| {
            //--------------------------------------------------

            let layout = renderer.get_layout();
            let sampler = match sampler {
                Some(val) => val,
                None => brackens_renderer::wgpu::SamplerDescriptor::default(),
            };

            let texture = brackens_renderer::renderer_2d::Texture::from_bytes(
                &device.0,
                &queue.0,
                data.0,
                data.1.as_ref(),
                &sampler,
            )
            .unwrap();

            let loaded_texture = RendererTexture::from_texture(&device.0, texture, layout);
            texture_storage.add_asset(loaded_texture)

            //--------------------------------------------------
        },
        (bytes, label),
    )
}

//===============================================================

impl AssetLoadable for RendererMaterial {
    fn load_asset(_world: &World, _path: &str) -> Result<Self, crate::assets::AssetLoadError>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl AssetLoadable for RendererMesh {
    fn load_asset(_world: &World, _path: &str) -> Result<Self, crate::assets::AssetLoadError>
    where
        Self: Sized,
    {
        todo!()
    }
}

pub fn load_model_path<T: AsRef<str>>(_world: &mut World, _label: T, _path: T) -> Model {
    todo!()
}

//===============================================================
