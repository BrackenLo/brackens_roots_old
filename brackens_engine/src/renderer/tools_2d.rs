//===============================================================

use brackens_assets::Handle;
use brackens_renderer::renderer_2d::RendererTexture;
use shipyard::World;

use crate::{
    assets::AssetStorage,
    core_components::{Device, Queue},
    UV, UVM,
};

use super::components_2d::TextureRenderer;

//===============================================================

pub fn load_texture<T: AsRef<str>>(
    world: &mut World,
    path: T,
    label: T,
    sampler: Option<brackens_renderer::wgpu::SamplerDescriptor>,
) -> Handle<RendererTexture> {
    world.run_with_data(
        |data: (T, T),
         mut texture_storage: UVM<AssetStorage<RendererTexture>>,
         renderer: UV<TextureRenderer>,
         device: UV<Device>,
         queue: UV<Queue>| {
            //--------------------------------------------------

            let layout = renderer.get_layout();
            let sampler = match sampler {
                Some(val) => val,
                None => brackens_renderer::wgpu::SamplerDescriptor::default(),
            };

            let texture = brackens_renderer::renderer_2d::Texture::from_file(
                &device.0,
                &queue.0,
                data.0.as_ref(),
                data.1.as_ref(),
                &sampler,
            )
            .unwrap();

            let loaded_texture = RendererTexture::from_texture(&device.0, texture, layout);
            texture_storage.add_asset(loaded_texture)

            //--------------------------------------------------
        },
        (path, label),
    )
}

//===============================================================
