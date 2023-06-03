//===============================================================

use crate::pipelines::{instance_pipeline::RawInstancePipeline, PipelineBuilderDescriptor};

use super::{
    renderer_components::{RawTextureVertex, TEXTURE_INDICES, TEXTURE_VERTICES},
    RawTextureInstance,
};

//===============================================================

pub struct Renderer2D {
    // Underlying pipeline
    pipeline: RawInstancePipeline,

    // Bind Group and buffer containing constant data (projection, time, etc.):
    global_bind_group: wgpu::BindGroup,
    global_uniform_buffer: wgpu::Buffer,

    // Bind Group Layout textures (and texture instance stuff) must adhere to
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl Renderer2D {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,

        global_bind_group_layout: wgpu::BindGroupLayout,
        global_bind_group: wgpu::BindGroup,
        global_uniform_buffer: wgpu::Buffer,

        shader: wgpu::ShaderSource,
        label: &str,
    ) -> Self {
        //----------------------------------------------

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(&format!("Renderer2D: {}", label)),
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
            name: format!("Renderer2D: {}", label),
            bind_group_layouts: Some(vec![&global_bind_group_layout, &texture_bind_group_layout]),
            shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(&format!("Renderer2D: {} - Shader", label)),
                source: shader,
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

        //----------------------------------------------

        let pipeline = RawInstancePipeline::new::<RawTextureVertex, RawTextureInstance>(
            device,
            builder,
            &TEXTURE_VERTICES,
            &TEXTURE_INDICES,
        );

        Self {
            pipeline,
            global_bind_group,
            global_uniform_buffer,
            texture_bind_group_layout,
        }

        //----------------------------------------------
    }
}

//===============================================================
