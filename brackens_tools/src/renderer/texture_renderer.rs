//===============================================================

use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use super::{
    pipelines::{PipelineBuilderDescriptor, RawInstancePipeline, Vertex},
    render_tools::RenderPassTools,
};

//===============================================================

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawTextureVertex {
    position: [f32; 3],
    tex_coord: [f32; 2],
}
impl Vertex for RawTextureVertex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawTextureVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct RawTextureInstance {
    transform: [f32; 16],
    color: [f32; 4],
}
impl Vertex for RawTextureInstance {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawTextureInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
            ],
        }
    }
}

//===============================================================

pub const TEXTURE_VERTICES: [RawTextureVertex; 4] = [
    RawTextureVertex {
        position: [-0.5, -0.5, 0.0],
        tex_coord: [0.0, 0.0],
    },
    RawTextureVertex {
        position: [0.5, -0.5, 0.0],
        tex_coord: [1.0, 1.0],
    },
    RawTextureVertex {
        position: [0.5, 0.5, 0.0],
        tex_coord: [1.0, 0.0],
    },
    RawTextureVertex {
        position: [-0.5, 0.5, 0.0],
        tex_coord: [0.0, 1.0],
    },
];
pub const TEXTURE_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

//===============================================================

pub struct TextureRenderer {
    pipeline: RawInstancePipeline,

    projection_bind_group: wgpu::BindGroup,
    projection_uniform_buffer: wgpu::Buffer,

    texture_bind_group_layout: wgpu::BindGroupLayout,
}
impl TextureRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        //----------------------------------------------

        let projection_matrix = glam::Mat4::orthographic_rh(0., 640., 0., 360., 0., 100.);
        let projection_uniform_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Texture Renderer Projection Uniform Buffer"),
                contents: bytemuck::cast_slice(&projection_matrix.to_cols_array()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let projection_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Renderer Projection Bind Group Layout"),
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
            label: Some("Texture Renderer Bind Group"),
            layout: &projection_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    projection_uniform_buffer.as_entire_buffer_binding(),
                ),
            }],
        });

        //----------------------------------------------

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Renderer Bind Group Layout"),
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

        let builder = PipelineBuilderDescriptor {
            name: "Texture".into(),
            bind_group_layouts: Some(vec![]),
            shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Texture Renderer Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("texture_shader.wgsl").into()),
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment_targets: vec![Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            multiview: None,
        };

        let pipeline = RawInstancePipeline::new::<RawTextureVertex, RawTextureInstance>(
            device,
            builder,
            &TEXTURE_VERTICES,
            &TEXTURE_INDICES,
        );

        //----------------------------------------------

        Self {
            pipeline,
            projection_bind_group,
            projection_uniform_buffer,
            texture_bind_group_layout,
        }

        //----------------------------------------------
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, new_size: PhysicalSize<u32>) {
        queue.write_buffer(
            &self.projection_uniform_buffer,
            0,
            bytemuck::cast_slice(
                &glam::Mat4::orthographic_rh(
                    0.,
                    new_size.width as f32,
                    0.,
                    new_size.height as f32,
                    0.,
                    100.,
                )
                .to_cols_array(),
            ),
        )
    }

    pub fn get_texture_layout(&self) -> &wgpu::BindGroupLayout {
        &self.texture_bind_group_layout
    }
}

pub struct TextureDrawCall {
    texture_bind_group: wgpu::BindGroup,
    instances: wgpu::Buffer,
    instance_count: u32,
}

impl TextureRenderer {
    pub fn render(&self, render_tools: &mut RenderPassTools, draw_calls: Vec<TextureDrawCall>) {
        let mut render_pass = self.pipeline.start_render_pass(render_tools, None);

        render_pass.set_bind_group(0, &self.projection_bind_group);
        for draw_call in &draw_calls {
            render_pass.set_bind_group(1, &draw_call.texture_bind_group);
            render_pass.draw_instanced(Some(&draw_call.instances), draw_call.instance_count);
        }
    }
}

//===============================================================
