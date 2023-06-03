//===============================================================

use std::collections::HashMap;

use brackens_assets::Handle;
use brackens_renderer::{
    bytemuck, render_tools,
    renderer_3d::{
        self, model_renderer::InstanceData, MaterialID, MeshID, RawMeshInstance, RendererMaterial,
        RendererMesh,
    },
    wgpu::{self, util::DeviceExt},
    Size,
};
use shipyard::{Component, Unique};

use crate::assets::AssetStorage;

//===============================================================

//===============================================================

// Model Rendering

#[derive(Component)]
pub struct Model {
    meshes: HashMap<MeshID, MaterialID>,
}

//--------------------------------------------------

#[derive(Unique)]
pub struct ModelRenderer {
    renderer: renderer_3d::ModelRenderer,

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
            renderer: renderer_3d::ModelRenderer::new(device, config),
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
