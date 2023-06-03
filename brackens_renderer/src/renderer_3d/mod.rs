//===============================================================

use brackens_assets::HandleID;

pub mod assets;
pub mod model_renderer;

pub use {
    assets::{RendererMaterial, RendererMesh},
    model_renderer::{ModelRenderer, RawMeshInstance, RawMeshVertex},
};

//===============================================================

pub type MaterialID = HandleID<RendererMaterial>;
pub type MeshID = HandleID<RendererMesh>;

//===============================================================

// pub struct MaterialBindGroupLayout {
//     inner: wgpu::BindGroupLayout,
//     creation:
// }

//===============================================================
