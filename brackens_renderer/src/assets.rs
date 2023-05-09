//===============================================================

use brackens_assets::{Asset, AssetLoadable};

use crate::{
    models::{RendererMaterial, RendererMesh},
    textures::{RendererTexture, Texture},
};

//===============================================================

impl Asset for Texture {
    fn asset_name() -> &'static str {
        "WgpuTexture"
    }
}

type TheDataWeNeed<'a> = (wgpu::Device, wgpu::Queue, &'a wgpu::SamplerDescriptor<'a>);

impl<'a> AssetLoadable<TheDataWeNeed<'a>> for Texture {
    fn load_from_file(path: String, data: TheDataWeNeed<'a>) -> Self {
        todo!()
    }

    fn load_default(data: TheDataWeNeed<'a>) -> Self {
        todo!()
    }
}

impl Asset for RendererTexture {
    fn asset_name() -> &'static str {
        "WgpuLoadedTexture"
    }
}

impl Asset for RendererMaterial {
    fn asset_name() -> &'static str {
        "WgpuMaterial"
    }
}

impl Asset for RendererMesh {
    fn asset_name() -> &'static str {
        "WgpuModel"
    }
}

//===============================================================
