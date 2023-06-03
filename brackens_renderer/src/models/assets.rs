//===============================================================

use anyhow::Result;
use brackens_assets::{file_loading::load_string, Asset, AssetStorage, HandleID};
use std::{
    fs::File,
    io::{BufReader, Cursor},
};
use tobj::load_mtl_buf;

use crate::textures::RendererTexture;

use super::{RawMaterial, RawMesh, RendererMaterial, RendererMesh, RendererModel};

//===============================================================

impl Asset for RawMesh {
    fn asset_name() -> &'static str {
        "Raw Mesh"
    }
}
impl Asset for RawMaterial {
    fn asset_name() -> &'static str {
        "Raw Material"
    }
}

//===============================================================

impl Asset for RendererMesh {
    fn asset_name() -> &'static str {
        "Renderer Model"
    }
}

impl Asset for RendererMaterial {
    fn asset_name() -> &'static str {
        "Renderer Material"
    }
}

impl Asset for RendererModel {
    fn asset_name() -> &'static str {
        "Renderer Model"
    }
}

impl RendererModel {
    pub fn load_model(
        device: &wgpu::Device,
        queue: &wgpu::Queue,

        texture_storage: &mut AssetStorage<RendererTexture>,
        material_storage: AssetStorage<RendererMaterial>,
        model_storage: AssetStorage<RendererModel>,

        sampler: &wgpu::SamplerDescriptor,
        bind_group_layout: &wgpu::BindGroupLayout,

        path: String,
    ) -> Result<Self> {
        let txt = load_string(&path)?;

        let cursor = Cursor::new(txt);
        let mut reader = BufReader::new(cursor);

        let (models, model_materials) = tobj::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
            |p| {
                let file = File::open(p).unwrap();
                let mut reader = BufReader::new(file);

                load_mtl_buf(&mut reader)
            },
        )?;

        let mut raw_materials = Vec::new();
        for tobj::Material {
            name,
            ambient,
            diffuse,
            specular,
            shininess,
            dissolve,
            optical_density,
            ambient_texture,
            diffuse_texture,
            specular_texture,
            normal_texture,
            shininess_texture,
            dissolve_texture,
            illumination_model,
            unknown_param,
        } in model_materials?
        {
            let ambient_texture = load_texture(
                device,
                queue,
                texture_storage,
                sampler,
                bind_group_layout,
                &name,
                ambient_texture,
            )?;

            let diffuse_texture = load_texture(
                device,
                queue,
                texture_storage,
                sampler,
                bind_group_layout,
                &name,
                diffuse_texture,
            )?;

            let specular_texture = load_texture(
                device,
                queue,
                texture_storage,
                sampler,
                bind_group_layout,
                &name,
                specular_texture,
            )?;

            let normal_texture = load_texture(
                device,
                queue,
                texture_storage,
                sampler,
                bind_group_layout,
                &name,
                normal_texture,
            )?;
            let shininess_texture = load_texture(
                device,
                queue,
                texture_storage,
                sampler,
                bind_group_layout,
                &name,
                shininess_texture,
            )?;
            let dissolve_texture = load_texture(
                device,
                queue,
                texture_storage,
                sampler,
                bind_group_layout,
                &name,
                dissolve_texture,
            )?;

            let material = RawMaterial {
                name,
                ambient,
                diffuse,
                specular,
                shininess,
                dissolve,
                optical_density,
                ambient_texture,
                diffuse_texture,
                specular_texture,
                normal_texture,
                shininess_texture,
                dissolve_texture,
                illumination_model,
            };

            raw_materials.push(material);
        }

        todo!()
    }
}

fn load_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture_storage: &mut AssetStorage<RendererTexture>,
    sampler: &wgpu::SamplerDescriptor,
    bind_group_layout: &wgpu::BindGroupLayout,

    name: &str,
    texture: Option<String>,
) -> Result<Option<HandleID<RendererTexture>>> {
    let texture = match texture {
        Some(path) => match texture_storage.get_loaded_file(&path) {
            Some(handle) => Some(handle.id()),
            None => {
                let texture = RendererTexture::from_file(
                    device,
                    queue,
                    &path,
                    &name,
                    sampler,
                    bind_group_layout,
                )?;

                Some(texture_storage.add_asset_file(texture, path).id())
            }
        },
        None => None,
    };

    Ok(texture)
}

//===============================================================
