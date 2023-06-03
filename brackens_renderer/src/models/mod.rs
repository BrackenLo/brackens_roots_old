//===============================================================

use crate::textures::RendererTexture;

use brackens_assets::HandleID;

use self::assets::{RendererMaterial, RendererMesh};

pub mod assets;
pub mod model_renderer;

pub use model_renderer::{ModelRenderer, RawMeshInstance, RawMeshVertex};

//===============================================================

pub type MaterialID = HandleID<RendererMaterial>;
pub type MeshID = HandleID<RendererMesh>;
pub type TextureID = HandleID<RendererTexture>;

//===============================================================

// pub struct MaterialBindGroupLayout {
//     inner: wgpu::BindGroupLayout,
//     creation:
// }

//===============================================================
