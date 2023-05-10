//===============================================================

use std::collections::HashMap;

use crate::textures::Texture;

pub mod model_renderer;

use brackens_assets::{Handle, HandleID};
pub use model_renderer::{ModelRenderer, RawMeshInstance, RawMeshVertex};

//===============================================================

// pub struct RendererMaterial {
//     pub name: String,
//     pub albedo: Texture,
//     // pub specular: Texture,
//     pub bind_group: wgpu::BindGroup,
// }
// impl RendererMaterial {
//     pub fn load(
//         device: &wgpu::Device,
//         name: String,
//         albedo: Texture,
//         // specular: Texture,
//         layout: &wgpu::BindGroupLayout,
//     ) -> Self {
//         let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label: Some(&format!("Loaded Model {}", name)),
//             layout,
//             entries: &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: wgpu::BindingResource::TextureView(&albedo.view),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: wgpu::BindingResource::Sampler(&albedo.sampler),
//                 },
//                 // wgpu::BindGroupEntry {
//                 //     binding: 2,
//                 //     resource: wgpu::BindingResource::TextureView(&specular.view),
//                 // },
//                 // wgpu::BindGroupEntry {
//                 //     binding: 3,
//                 //     resource: wgpu::BindingResource::Sampler(&specular.sampler),
//                 // },
//             ],
//         });

//         Self {
//             name,
//             albedo,
//             // specular: todo!(),
//             bind_group,
//         }
//     }
// }

// pub struct RendererMesh {
//     pub vertices: wgpu::Buffer,
//     pub indices: wgpu::Buffer,
//     pub index_count: u32,
// }

//===============================================================

// pub struct Material {
//     pub name: String,
//     pub ambient: Option<[f64; 3]>,
//     pub diffuse: Option<[f64; 3]>,
//     pub specular: Option<[f64; 3]>,
//     pub shininess: Option<f64>,
//     pub dissolve: Option<f64>,
//     pub optical_density: Option<f64>,
//     pub ambient_texture: Option<String>,
//     pub diffuse_texture: Option<String>,
//     pub specular_texture: Option<String>,
//     pub normal_texture: Option<String>,
//     pub shininess_texture: Option<String>,
//     pub dissolve_texture: Option<String>,
//     pub illumination_model: Option<u8>,
//     pub unknown_param: AHashMap<String, String>,
// }

// pub struct Mesh {
//     pub positions: Vec<f64>,
//     pub vertex_color: Vec<f64>,
//     pub normals: Vec<f64>,
//     pub texcoords: Vec<f64>,
//     pub indices: Vec<u32>,
//     pub face_arities: Vec<u32>,
//     pub vertex_color_indices: Vec<u32>,
//     pub texcoord_indices: Vec<u32>,
//     pub normal_indices: Vec<u32>,
//     pub material_id: Option<usize>,
// }

pub type MaterialID = HandleID;
pub type MeshID = HandleID;

pub struct RendererMaterial {
    pub name: String,
    pub diffuse: Handle<Texture>,

    pub bind_group: wgpu::BindGroup,
}

pub struct RendererMesh {
    pub vertices: wgpu::Buffer,
    pub indices: wgpu::Buffer,
    pub index_count: u32,
}

pub struct RendererModel {
    pub meshes: HashMap<MaterialID, MeshID>,
}

//===============================================================
