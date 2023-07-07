//===============================================================

use anyhow::Result;
use brackens_assets::Asset;
use image::GenericImageView;

use crate::Size;

//===============================================================

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Asset for Texture {
    fn asset_name(&self) -> &str {
        "Wgpu Texture"
    }
    // fn asset_name() -> &'static str {
    //     "Wgpu Texture"
    // }
}

impl Texture {
    pub fn from_file(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        label: &str,
        sampler: &wgpu::SamplerDescriptor,
    ) -> Result<Self> {
        let image = image::io::Reader::open(path)?.decode()?;
        Self::from_image(device, queue, &image, Some(label), sampler)
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
        sampler: &wgpu::SamplerDescriptor,
    ) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, Some(label), sampler)
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
        sampler: &wgpu::SamplerDescriptor,
    ) -> Result<Self> {
        //----------------------------------------------

        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        //----------------------------------------------

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        //----------------------------------------------

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: None,
            },
            size,
        );

        //----------------------------------------------

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(sampler);

        //----------------------------------------------

        Ok(Self {
            texture,
            view,
            sampler,
        })

        //----------------------------------------------
    }

    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    pub fn create_depth_texture(
        device: &wgpu::Device,
        window_size: Size<u32>,
        label: &str,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: window_size.width,
            height: window_size.height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("{} - Depth Texture", label)),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[wgpu::TextureFormat::Depth32Float],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(label),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}

//----------------------------------------------

pub struct RendererTexture {
    pub texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

impl Asset for RendererTexture {
    fn asset_name(&self) -> &str {
        "Renderer Texture"
    }
}

impl RendererTexture {
    pub fn from_color_f32(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: [f32; 3],
        label: &str,
        sampler: &wgpu::SamplerDescriptor,
        layout: &wgpu::BindGroupLayout,
    ) -> Result<Self> {
        let r = (color[0].clamp(0., 1.) * 255.) as u8;
        let g = (color[1].clamp(0., 1.) * 255.) as u8;
        let b = (color[2].clamp(0., 1.) * 255.) as u8;

        Self::from_color(device, queue, [r, g, b], label, sampler, layout)
    }

    pub fn from_color(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: [u8; 3],
        label: &str,
        sampler: &wgpu::SamplerDescriptor,
        layout: &wgpu::BindGroupLayout,
    ) -> Result<Self> {
        let mut rgb = image::RgbImage::new(1, 1);
        rgb.pixels_mut().for_each(|mut pixel| {
            pixel.0[0] = color[0];
            pixel.0[1] = color[1];
            pixel.0[2] = color[2];
        });
        let rgba = image::DynamicImage::from(rgb);

        Self::from_image(device, queue, &rgba, Some(label), sampler, layout)
    }

    pub fn from_file(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: &str,
        label: &str,
        sampler: &wgpu::SamplerDescriptor,
        layout: &wgpu::BindGroupLayout,
    ) -> Result<Self> {
        match Texture::from_file(device, queue, path, label, sampler) {
            Ok(texture) => Ok(Self::from_texture(device, texture, layout)),
            Err(e) => Err(e),
        }
    }

    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
        sampler: &wgpu::SamplerDescriptor,
        layout: &wgpu::BindGroupLayout,
    ) -> Result<Self> {
        match Texture::from_bytes(device, queue, bytes, label, sampler) {
            Ok(texture) => Ok(Self::from_texture(device, texture, layout)),
            Err(e) => Err(e),
        }
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
        sampler: &wgpu::SamplerDescriptor,
        layout: &wgpu::BindGroupLayout,
    ) -> Result<Self> {
        match Texture::from_image(device, queue, img, label, sampler) {
            Ok(texture) => Ok(Self::from_texture(device, texture, layout)),
            Err(e) => Err(e),
        }
    }

    pub fn from_texture(
        device: &wgpu::Device,
        texture: Texture,
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("Loaded wgpu texture")),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Self {
            texture,
            bind_group,
        }
    }
}

//===============================================================
