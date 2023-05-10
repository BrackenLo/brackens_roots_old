//===============================================================

use std::collections::{HashMap, HashSet};

use brackens_renderer::{
    bytemuck,
    models::{self, model_renderer::InstanceData, RawMeshInstance, RendererMaterial, RendererMesh},
    render_tools,
    textures::{self, RawTextureInstance},
    wgpu::{self, util::DeviceExt},
    Size,
};

use brackens_assets::{Handle, HandleID};

use brackens_tools::glam::Vec2;
use shipyard::{Component, Unique};

pub use brackens_renderer::{
    textures::RendererTexture, textures::TextureDrawCall as FinalTextureDrawCall,
};

use crate::assets::AssetStorage;

//===============================================================
// Core rendering Uniques

#[derive(Unique)]
pub struct RenderPassTools(pub(crate) render_tools::RenderPassTools);

#[derive(Unique)]
pub struct ClearColor(pub [f64; 3]);

//===============================================================
// Shared Rendering Components

#[derive(Component)]
pub struct Visible {
    pub visible: bool,
}

//===============================================================
// Texture Rendering

#[derive(Unique)]
pub struct TextureRenderer {
    renderer: textures::TextureRenderer,

    should_render: HashSet<HandleID>,
    unprocessed_draw_data: HashMap<HandleID, Vec<RawTextureInstance>>,

    texture_data: HashMap<HandleID, Handle<RendererTexture>>,
    draw_data: HashMap<HandleID, FinalTextureDrawCall>,
}

impl TextureRenderer {
    //--------------------------------------------------

    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            renderer: textures::TextureRenderer::new(device, config.format),
            should_render: HashSet::new(),
            unprocessed_draw_data: HashMap::new(),

            texture_data: HashMap::new(),
            draw_data: HashMap::new(),
        }
    }

    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        self.renderer.get_texture_layout()
    }

    //--------------------------------------------------

    pub(crate) fn resize(&mut self, queue: &wgpu::Queue, new_size: Size<u32>) {
        self.renderer.resize(queue, new_size)
    }

    //--------------------------------------------------

    pub(crate) fn add_texture(&mut self, handle: Handle<RendererTexture>) {
        let handle = handle.clone_weak();
        self.texture_data.insert(handle.id(), handle);
    }

    pub(crate) fn remove_texture(&mut self, id: HandleID) {
        self.should_render.remove(&id);
        self.unprocessed_draw_data.remove(&id);

        self.texture_data.remove(&id);
        self.draw_data.remove(&id);
    }

    //--------------------------------------------------

    pub(crate) fn draw_texture(&mut self, handle_id: HandleID, instance: RawTextureInstance) {
        match self.unprocessed_draw_data.get_mut(&handle_id) {
            Some(val) => val.push(instance),
            None => {
                self.unprocessed_draw_data.insert(handle_id, vec![instance]);
            }
        };
    }

    //--------------------------------------------------

    pub(crate) fn process_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.should_render.clear();

        for unprocessed in self.unprocessed_draw_data.iter() {
            let data_count = unprocessed.1.len() as u32;

            if data_count == 0 {
                continue;
            }
            self.should_render.insert(unprocessed.0.clone());

            if let Some(instance) = self.draw_data.get_mut(&unprocessed.0) {
                // Buffer is too small to hold new data. Need to create bigger buffer
                if data_count > instance.instance_count {
                    let FinalTextureDrawCall {
                        instances,
                        instance_count,
                    } = Self::create_instance_buffer(device, unprocessed.1);

                    instance.instances = instances;
                    instance.instance_count = instance_count;
                    continue;
                } else {
                    // Buffer is big enough. Just write new data to it
                    queue.write_buffer(&instance.instances, 0, bytemuck::cast_slice(unprocessed.1));
                    continue;
                }
            }

            // Data doesn't exist yet. Create it and add it
            let instance = Self::create_instance_buffer(device, unprocessed.1);
            self.draw_data.insert(unprocessed.0.clone(), instance);
        }

        self.unprocessed_draw_data.clear();
    }

    fn create_instance_buffer(
        device: &wgpu::Device,
        data: &[RawTextureInstance],
    ) -> FinalTextureDrawCall {
        let instances = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Texture Instance Buffer")),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        FinalTextureDrawCall {
            instances,
            instance_count: data.len() as u32,
        }
    }

    //--------------------------------------------------

    pub(crate) fn render(&mut self, render_tools: &mut render_tools::RenderPassTools) {
        let draw = self
            .should_render
            .iter()
            .map(|val| {
                let bind_group = &self.texture_data.get(val).unwrap().get().bind_group;
                let draw_data = self.draw_data.get(val).unwrap();

                (bind_group, draw_data)
            })
            .collect::<Vec<_>>();

        self.renderer.render(render_tools, &draw);
    }

    //--------------------------------------------------
}

//--------------------------------------------------
// Texture Components

#[derive(Component)]
pub struct Texture {
    pub size: Vec2,
    pub handle: Handle<RendererTexture>,
    pub color: [f32; 4],
}

//===============================================================
//===============================================================
//===============================================================
// Model Rendering

type MaterialID = HandleID;
type MeshID = HandleID;

#[derive(Component)]
pub struct Model {
    meshes: HashMap<MeshID, MaterialID>,
}

//--------------------------------------------------

#[derive(Unique)]
pub struct ModelRenderer {
    renderer: models::ModelRenderer,

    instance_id: u16,
    instance_data: HashMap<u16, RawMeshInstance>,
    material_data: HashMap<MaterialID, HashMap<MeshID, Vec<u16>>>,

    processed_data: Vec<(
        Handle<RendererMaterial>,
        Vec<(Handle<RendererMesh>, InstanceData)>,
    )>,
}
impl ModelRenderer {
    //--------------------------------------------------

    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            renderer: models::ModelRenderer::new(device, config),
            instance_id: 0,
            instance_data: HashMap::new(),
            material_data: HashMap::new(),
            processed_data: vec![],
        }
    }

    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        self.renderer.get_material_layout()
    }

    //--------------------------------------------------

    pub(crate) fn resize(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        new_size: Size<u32>,
    ) {
        self.renderer.resize(device, queue, new_size)
    }

    //--------------------------------------------------

    pub(crate) fn render_model(&mut self, model: &Model, instance: RawMeshInstance) {
        let instance_id = self.instance_id;
        self.instance_id += 1;

        self.instance_data.insert(instance_id, instance);

        for (mesh, material) in &model.meshes {
            if let Some(meshes) = self.material_data.get_mut(material) {
                if let Some(instances) = meshes.get_mut(mesh) {
                    instances.push(instance_id);
                    continue;
                }

                meshes.insert(mesh.clone(), vec![instance_id]);
            }

            let mut hashmap = HashMap::new();
            hashmap.insert(mesh.clone(), vec![instance_id]);

            self.material_data.insert(material.clone(), hashmap);
        }
    }

    //--------------------------------------------------

    pub(crate) fn process_data(
        &mut self,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
        mesh_storage: &AssetStorage<RendererMesh>,
        material_storage: &AssetStorage<RendererMaterial>,
    ) {
        self.processed_data.clear();

        for (material_id, meshes) in &self.material_data {
            let material = material_storage.get_handle(material_id).unwrap();

            let mut mesh_data = Vec::new();

            for (mesh_id, instance_id) in meshes {
                let mesh = mesh_storage.get_handle(mesh_id).unwrap();

                let instances = instance_id
                    .iter()
                    .map(|id| *self.instance_data.get(id).unwrap())
                    .collect::<Vec<_>>();

                let instances = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("Model Instance Buffer")),
                    contents: bytemuck::cast_slice(&instances),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });

                let instance_data = InstanceData {
                    instances,
                    instance_count: instance_id.len() as u32,
                };

                mesh_data.push((mesh, instance_data));
            }

            self.processed_data.push((material, mesh_data));
        }
    }

    pub(crate) fn render(&mut self, render_tools: &mut render_tools::RenderPassTools) {
        let draw = self
            .processed_data
            .iter()
            .map(|(material, meshes)| {
                let mesh_data = meshes
                    .iter()
                    .map(|(mesh, instances)| (mesh.get(), instances))
                    .collect::<Vec<_>>();

                (&material.get().bind_group, mesh_data)
            })
            .collect::<Vec<_>>();

        self.renderer.render(render_tools, draw.as_slice());
    }
}

//===============================================================
