//===============================================================

use std::collections::HashMap;

use crate::textures::{RendererTexture, Texture};

use brackens_assets::{Handle, HandleID};
pub use model_renderer::{ModelRenderer, RawMeshInstance, RawMeshVertex};

pub mod assets;
pub mod model_renderer;

//===============================================================

pub type MaterialID = HandleID<RendererMaterial>;
pub type MeshID = HandleID<RendererMesh>;
pub type TextureID = HandleID<RendererTexture>;

//===============================================================

pub struct MaterialBindGroupLayout {
    inner: wgpu::BindGroupLayout,
    creation:
}

//===============================================================

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

pub struct RawMesh {
    pub positions: Vec<[f32; 3]>,
    pub vertex_color: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub texcoords: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

pub struct RawMaterial {
    pub name: String,

    pub ambient: Option<[f32; 3]>,
    pub diffuse: Option<[f32; 3]>,
    pub specular: Option<[f32; 3]>,
    pub shininess: Option<f32>,
    pub dissolve: Option<f32>,
    pub optical_density: Option<f32>,

    pub ambient_texture: Option<TextureID>,
    pub diffuse_texture: Option<TextureID>,
    pub specular_texture: Option<TextureID>,
    pub normal_texture: Option<TextureID>,
    pub shininess_texture: Option<TextureID>,
    pub dissolve_texture: Option<TextureID>,

    pub illumination_model: Option<u8>,
    // pub unknown_param: HashMap<String, String>,
}

//===============================================================
