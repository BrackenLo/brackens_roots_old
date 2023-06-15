//===============================================================

pub mod bind_group_templates;
pub mod instance_pipeline;
pub mod raw_pipeline;

pub use {
    bind_group_templates::{
        BindGroupEntry, BindGroupEntryLayout, BindGroupEntryType, BindGroupTemplate,
    },
    instance_pipeline::InstancePipeline,
    raw_pipeline::RawPipeline,
};

//===============================================================

pub trait Vertex: bytemuck::Pod {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}

pub struct PipelineBuilderDescriptor<'a> {
    pub name: String,
    pub bind_group_layouts: Option<Vec<&'a wgpu::BindGroupLayout>>,
    pub shader: wgpu::ShaderModule,
    pub primitive: wgpu::PrimitiveState,
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    pub multisample: wgpu::MultisampleState,
    pub fragment_targets: Vec<Option<wgpu::ColorTargetState>>,
    pub multiview: Option<std::num::NonZeroU32>,
}

//===============================================================

//===============================================================
