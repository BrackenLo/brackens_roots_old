//===============================================================

use brackens_assets::Handle;
use brackens_renderer::{renderer_2d::RendererTexture, wgpu};
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
    sampler: Option<wgpu::SamplerDescriptor>,
) -> Handle<RendererTexture> {
    world.run(
        |mut texture_storage: UVM<AssetStorage<RendererTexture>>,
         renderer: UV<TextureRenderer>,
         device: UV<Device>,
         queue: UV<Queue>| {
            //--------------------------------------------------

            let layout = renderer.get_layout();
            let sampler = match sampler {
                Some(val) => val,
                None => wgpu::SamplerDescriptor::default(),
            };

            let loaded_texture = RendererTexture::from_file(
                &device.0,
                &queue.0,
                path.as_ref(),
                label.as_ref(),
                &sampler,
                layout,
            )
            .unwrap();
            texture_storage.add_asset(loaded_texture)

            //--------------------------------------------------
        },
    )
}

pub fn load_texture_custom_layout<T: AsRef<str>>(
    world: &mut World,
    path: T,
    label: T,
    sampler: Option<wgpu::SamplerDescriptor>,
    layout: &wgpu::BindGroupLayout,
) -> Handle<RendererTexture> {
    world.run(
        |mut texture_storage: UVM<AssetStorage<RendererTexture>>,
         device: UV<Device>,
         queue: UV<Queue>| {
            let sampler = match sampler {
                Some(val) => val,
                None => wgpu::SamplerDescriptor::default(),
            };

            let loaded_texture = RendererTexture::from_file(
                &device.0,
                &queue.0,
                path.as_ref(),
                label.as_ref(),
                &sampler,
                layout,
            )
            .unwrap();

            texture_storage.add_asset(loaded_texture)
        },
    )
}

pub fn load_blank_texture<T: AsRef<str>>(
    world: &mut World,
    label: T,
    color: [f32; 3],
    sampler: Option<wgpu::SamplerDescriptor>,
) -> Handle<RendererTexture> {
    world.run(
        |mut texture_storage: UVM<AssetStorage<RendererTexture>>,
         renderer: UV<TextureRenderer>,
         device: UV<Device>,
         queue: UV<Queue>| {
            let sampler = match sampler {
                Some(val) => val,
                None => wgpu::SamplerDescriptor::default(),
            };
            let layout = renderer.get_layout();

            let r = (color[0].clamp(0., 1.) * 255.) as u8;
            let g = (color[1].clamp(0., 1.) * 255.) as u8;
            let b = (color[2].clamp(0., 1.) * 255.) as u8;

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
                Some(label.as_ref()),
                &sampler,
                layout,
            )
            .unwrap();

            texture_storage.add_asset(loaded_texture)
        },
    )
}

pub fn load_blank_texture_custom_layout<T: AsRef<str>>(
    world: &mut World,
    label: T,
    color: [f32; 3],
    sampler: Option<wgpu::SamplerDescriptor>,
    layout: &wgpu::BindGroupLayout,
) -> Handle<RendererTexture> {
    world.run(
        |mut texture_storage: UVM<AssetStorage<RendererTexture>>,
         device: UV<Device>,
         queue: UV<Queue>| {
            let sampler = match sampler {
                Some(val) => val,
                None => wgpu::SamplerDescriptor::default(),
            };

            let r = (color[0].clamp(0., 1.) * 255.) as u8;
            let g = (color[1].clamp(0., 1.) * 255.) as u8;
            let b = (color[2].clamp(0., 1.) * 255.) as u8;

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
                Some(label.as_ref()),
                &sampler,
                layout,
            )
            .unwrap();

            texture_storage.add_asset(loaded_texture)
        },
    )
}

//===============================================================
