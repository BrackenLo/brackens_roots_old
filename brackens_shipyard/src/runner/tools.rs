//===============================================================

use brackens_assets::Handle;
use brackens_renderer::{renderer_2d::RendererTexture, wgpu};
use shipyard::{Borrow, BorrowInfo, IntoBorrow, UniqueView, UniqueViewMut};

use crate::{
    assets::AssetStorage,
    renderer::{Device, Queue},
};

//===============================================================

pub struct TextureLoader<'v> {
    assets: UniqueViewMut<'v, AssetStorage>,
    device: UniqueView<'v, Device>,
    queue: UniqueView<'v, Queue>,
}
impl<'v> TextureLoader<'v> {
    pub fn load_texture(
        &mut self,
        layout: &wgpu::BindGroupLayout,
        label: &str,
        path: &str,
        sampler: Option<wgpu::SamplerDescriptor>,
    ) -> Handle<RendererTexture> {
        let sampler = match sampler {
            Some(sampler) => sampler,
            None => wgpu::SamplerDescriptor::default(),
        };

        self.assets
            .load_from_data(path, |bytes| {
                RendererTexture::from_bytes(
                    self.device.inner(),
                    self.queue.inner(),
                    bytes,
                    label,
                    &sampler,
                    layout,
                )
                .unwrap()
            })
            .unwrap()
    }

    pub fn load_color(
        &mut self,
        layout: &wgpu::BindGroupLayout,
        label: &str,
        color: [u8; 3],
        sampler: Option<wgpu::SamplerDescriptor>,
    ) -> Handle<RendererTexture> {
        let sampler = match sampler {
            Some(sampler) => sampler,
            None => wgpu::SamplerDescriptor::default(),
        };

        let texture = RendererTexture::from_color(
            self.device.inner(),
            self.queue.inner(),
            color,
            label,
            &sampler,
            layout,
        )
        .unwrap();

        self.assets.insert_data(texture)
    }
}

//--------------------------------------------------

pub type TextureLoaderComponents<'v> = (
    UniqueViewMut<'v, AssetStorage>,
    UniqueView<'v, Device>,
    UniqueView<'v, Queue>,
);

pub struct TextureLoaderBorrower;
impl<'v> IntoBorrow for TextureLoader<'_> {
    type Borrow = TextureLoaderBorrower;
}

impl<'v> Borrow<'v> for TextureLoaderBorrower {
    type View = TextureLoader<'v>;

    fn borrow(
        world: &'v shipyard::World,
        last_run: Option<u32>,
        current: u32,
    ) -> Result<Self::View, shipyard::error::GetStorage> {
        let (assets, device, queue) =
            <TextureLoaderComponents as IntoBorrow>::Borrow::borrow(world, last_run, current)?;

        Ok(TextureLoader {
            assets,
            device,
            queue,
        })
    }
}

unsafe impl BorrowInfo for TextureLoader<'_> {
    fn borrow_info(info: &mut Vec<shipyard::info::TypeInfo>) {
        TextureLoaderComponents::borrow_info(info);
    }
}

//===============================================================
