//===============================================================

use wgpu::util::DeviceExt;

use crate::{
    pipelines::{PipelineBuilderDescriptor, RawPipeline, Vertex},
    render_tools::RenderPassTools,
    textures::Texture,
    Size,
};

use super::RendererMesh;

//===============================================================

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawMeshVertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coord: [f32; 2],
}
impl Vertex for RawMeshVertex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawMeshVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawMeshInstance {
    pub transform: [f32; 16],
    // Bones?
}
impl Vertex for RawMeshInstance {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawMeshInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
            ],
        }
    }
}

//===============================================================

pub struct ModelRenderer {
    pipeline: RawPipeline,

    projection_bind_group: wgpu::BindGroup,
    projection_uniform_buffer: wgpu::Buffer,

    // Currently Contains:
    // Binding1 - Texture
    // Binding2 - Sampler
    material_bind_group_layout: wgpu::BindGroupLayout,

    depth_texture: Texture,
}
impl ModelRenderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        //----------------------------------------------

        let projection_matrix = glam::Mat4::orthographic_rh(0., 640., 0., 360., 0., 100.);
        let projection_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Model Renderer Projection Uniform Buffer"),
                contents: bytemuck::cast_slice(&projection_matrix.to_cols_array()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let projection_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Model Renderer Projection Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let projection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Model Renderer Bind Group"),
            layout: &projection_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    projection_uniform_buffer.as_entire_buffer_binding(),
                ),
            }],
        });

        //----------------------------------------------

        let material_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Model Renderer Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        //----------------------------------------------

        let size = Size {
            width: config.width,
            height: config.height,
        };

        let depth_texture = Texture::create_depth_texture(device, size, "Model Depth Texture");

        //----------------------------------------------

        let builder = PipelineBuilderDescriptor {
            name: "Model".into(),
            bind_group_layouts: Some(vec![
                &projection_bind_group_layout,
                &material_bind_group_layout,
            ]),
            shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Model Renderer Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("model_shader.wgsl").into()),
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment_targets: vec![Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            multiview: None,
        };

        let pipeline = RawPipeline::new(
            device,
            &[
                RawMeshVertex::buffer_layout(),
                RawMeshInstance::buffer_layout(),
            ],
            builder,
        );

        //----------------------------------------------

        Self {
            pipeline,
            projection_bind_group,
            projection_uniform_buffer,
            material_bind_group_layout,
            depth_texture,
        }

        //----------------------------------------------
    }

    pub fn resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, new_size: Size<u32>) {
        let projection_data = glam::Mat4::orthographic_rh(
            0.,
            new_size.width as f32,
            0.,
            new_size.height as f32,
            0.,
            100.,
        )
        .to_cols_array();

        queue.write_buffer(
            &self.projection_uniform_buffer,
            0,
            bytemuck::cast_slice(&projection_data),
        );

        self.depth_texture = Texture::create_depth_texture(device, new_size, "ModelDepthTexture");
    }

    pub fn get_material_layout(&self) -> &wgpu::BindGroupLayout {
        &self.material_bind_group_layout
    }

    pub fn render(
        &self,
        render_tools: &mut RenderPassTools,
        draw_calls: &[(&wgpu::BindGroup, Vec<(&RendererMesh, &InstanceData)>)],
    ) {
        let depth_stencil_attachment = wgpu::RenderPassDepthStencilAttachment {
            view: &self.depth_texture.view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        };

        let mut render_pass = self
            .pipeline
            .start_render_pass(render_tools, Some(depth_stencil_attachment));

        render_pass.set_bind_group(0, &self.projection_bind_group);

        for draw_call in draw_calls {
            // Set material for all meshes using this
            render_pass.set_bind_group(1, &draw_call.0);

            // Render all the meshes using the set material
            for instance in &draw_call.1 {
                let RendererMesh {
                    vertices,
                    indices,
                    index_count,
                } = instance.0;

                let InstanceData {
                    instances,
                    instance_count,
                } = &instance.1;

                render_pass.set_vertex_buffers([vertices, instances], 0);
                render_pass.set_index_buffer(indices);

                render_pass.draw_index(0..*index_count, 0..*instance_count);
            }
        }
    }
}

pub struct InstanceData {
    pub instances: wgpu::Buffer,
    pub instance_count: u32,
}

//===============================================================
