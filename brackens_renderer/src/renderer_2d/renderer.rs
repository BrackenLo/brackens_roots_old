//===============================================================

use std::marker::PhantomData;

use wgpu::util::DeviceExt;

use crate::{
    pipelines::{
        bind_group_templates::{
            BindGroupEntry, BindGroupEntryLayout, BindGroupEntryType, BindGroupTemplate,
            BufferTemplate,
        },
        instance_pipeline::RawInstancePipeline,
        PipelineBuilderDescriptor,
    },
    render_tools::RenderPassTools,
    Size,
};

use super::{
    renderer_components::{
        RawTextureInstance, RawTextureVertex, RendererDescriptor2D, TextureDrawBuffer,
        TEXTURE_INDICES, TEXTURE_VERTICES,
    },
    tools::TEXTURE_SHADER,
    RendererTexture, Texture,
};

//===============================================================

pub struct Renderer2D<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    // Underlying pipeline
    pipeline: RawInstancePipeline,

    // Bind Group Layout textures (and texture instance stuff) must adhere to
    texture_bind_group_layout: wgpu::BindGroupLayout,

    // Bind Group and buffer containing constant data (projection, time, etc.):
    phantom_data: PhantomData<T>,
    global_bind_group: wgpu::BindGroup,
    global_uniform_buffers: Option<wgpu::Buffer>,
}

impl<T> Renderer2D<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    //===============================================================

    pub fn new(descriptor: RendererDescriptor2D<T>) -> Self {
        let RendererDescriptor2D {
            device,
            format,
            global_bind_group_template,
            data,
            shader,
            use_depth_texture,
            label,
        } = descriptor;

        //----------------------------------------------

        let depth_stencil = if use_depth_texture {
            Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            })
        } else {
            None
        };

        let builder = PipelineBuilderDescriptor {
            name: format!("Renderer2D: {}", label),
            bind_group_layouts: None,
            shader: device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(&format!("Renderer2D: {} - Shader", label)),
                source: shader,
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil,
            multisample: wgpu::MultisampleState::default(),
            fragment_targets: vec![Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            multiview: None,
        };

        //----------------------------------------------

        Self::new_custom(device, global_bind_group_template, &data, builder, label)

        //----------------------------------------------
    }

    //===============================================================

    pub fn new_custom(
        device: &wgpu::Device,
        global_bind_group_template: BindGroupTemplate<T>,
        data: &[BindGroupEntry<T>],
        builder: PipelineBuilderDescriptor,
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

        let layout = global_bind_group_template.get_layout();
        let (global_bind_group, global_uniform_buffers) =
            global_bind_group_template.create_bind_group(device, data);

        //----------------------------------------------

        let PipelineBuilderDescriptor {
            name,
            // bind_group_layouts,
            shader,
            primitive,
            depth_stencil,
            multisample,
            fragment_targets,
            multiview,
            ..
        } = builder;

        let new_builder = PipelineBuilderDescriptor {
            name,
            bind_group_layouts: Some(vec![layout, &texture_bind_group_layout]),
            shader,
            primitive,
            depth_stencil,
            multisample,
            fragment_targets,
            multiview,
        };

        let pipeline = RawInstancePipeline::new::<RawTextureVertex, RawTextureInstance>(
            device,
            new_builder,
            &TEXTURE_VERTICES,
            &TEXTURE_INDICES,
        );

        Self {
            pipeline,
            texture_bind_group_layout,
            phantom_data: PhantomData,
            global_bind_group,
            global_uniform_buffers,
        }
    }

    //===============================================================

    pub fn update_global_buffer(&mut self, queue: &wgpu::Queue, data: &[T]) {
        if let Some(buffer) = &self.global_uniform_buffers {
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(data))
        }
    }

    pub fn set_global_buffer(&mut self, device: &wgpu::Device, data: &[T]) {
        let name = self.name().to_string();

        if let Some(buffer) = &mut self.global_uniform_buffers {
            *buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("Renderer2D: {} - Uniform Buffer", name)),
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        }
    }

    //===============================================================

    pub fn name(&self) -> &str {
        &self.pipeline.name()
    }

    pub fn get_texture_layout(&self) -> &wgpu::BindGroupLayout {
        &self.texture_bind_group_layout
    }

    pub fn load_texture(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: &str,
        path: &str,
        sampler: wgpu::SamplerDescriptor,
    ) -> RendererTexture {
        RendererTexture::from_file(
            device,
            queue,
            path,
            label,
            &sampler,
            &self.texture_bind_group_layout,
        )
        .unwrap()
    }

    //===============================================================

    pub fn render(
        &self,
        render_tools: &mut RenderPassTools,
        draw_calls: &[(&wgpu::BindGroup, &TextureDrawBuffer)],
        depth_texture: Option<wgpu::RenderPassDepthStencilAttachment>,
    ) {
        let mut render_pass = self.pipeline.start_render_pass(render_tools, depth_texture);

        render_pass.set_bind_group(0, &self.global_bind_group);
        for draw_call in draw_calls {
            render_pass.set_bind_group(1, &draw_call.0);
            render_pass.draw_instanced(
                Some(&draw_call.1.instance_buffer),
                draw_call.1.instance_count,
            );
        }
    }
}

//===============================================================

pub struct TextureRenderer {
    inner: Renderer2D<[f32; 16]>,
    depth_texture: Texture,
}

impl TextureRenderer {
    //----------------------------------------------

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, window_size: Size<u32>) -> Self {
        //----------------------------------------------

        let buffer_template = BufferTemplate::new("Texture Renderer");
        let bind_group_template = BindGroupTemplate::new(
            device,
            "Texture Renderer",
            vec![BindGroupEntryLayout {
                entry_type: BindGroupEntryType::Buffer(buffer_template),
                visibility: wgpu::ShaderStages::VERTEX,
            }],
        );

        let projection_matrix = glam::Mat4::orthographic_lh(
            0.,
            window_size.width as f32,
            0.,
            window_size.height as f32,
            0.,
            100.,
        )
        .to_cols_array();

        let data = BindGroupEntry::Buffer(projection_matrix);

        //----------------------------------------------

        let inner = Renderer2D::new(RendererDescriptor2D {
            device,
            format,
            global_bind_group_template: bind_group_template,
            data: vec![data],
            shader: wgpu::ShaderSource::Wgsl(TEXTURE_SHADER.into()),
            use_depth_texture: true,
            label: "Texture Renderer",
        });

        //----------------------------------------------

        let depth_texture = Texture::create_depth_texture(
            device,
            Size::new(window_size.width, window_size.height),
            "Texture Renderer",
        );

        Self {
            inner,
            depth_texture,
        }

        //----------------------------------------------
    }

    //----------------------------------------------

    pub fn resize_depth(&mut self, device: &wgpu::Device, new_size: Size<u32>) {
        self.depth_texture = Texture::create_depth_texture(device, new_size, "Texture Renderer");
    }

    pub fn set_projection(&mut self, queue: &wgpu::Queue, matrix: &glam::Mat4) {
        self.inner
            .update_global_buffer(queue, bytemuck::cast_slice(&matrix.to_cols_array()));
    }

    pub fn resize_depth_projection(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        new_size: Size<u32>,
    ) {
        self.resize_depth(device, new_size);
        self.set_projection(
            queue,
            &glam::Mat4::orthographic_lh(
                0.,
                new_size.width as f32,
                0.,
                new_size.height as f32,
                0.,
                100.,
            ),
        );
    }

    pub fn get_texture_layout(&self) -> &wgpu::BindGroupLayout {
        &self.inner.get_texture_layout()
    }

    //----------------------------------------------

    pub fn render(
        &self,
        render_tools: &mut RenderPassTools,
        draw_calls: &[(&wgpu::BindGroup, &TextureDrawBuffer)],
    ) {
        self.inner.render(
            render_tools,
            draw_calls,
            Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        );
    }

    //----------------------------------------------
}

//===============================================================
