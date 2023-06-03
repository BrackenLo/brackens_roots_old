//===============================================================

use std::ops::Range;

use log::info;

use crate::render_tools::RenderPassTools;

use super::PipelineBuilderDescriptor;

//===============================================================

pub struct RawPipeline {
    name: String,
    pipeline: wgpu::RenderPipeline,
}
impl RawPipeline {
    pub fn new(
        device: &wgpu::Device,
        buffers: &[wgpu::VertexBufferLayout],
        builder: PipelineBuilderDescriptor,
    ) -> Self {
        info!("Creating new pipeline '{}'", &builder.name);

        //----------------------------------------------

        let bind_group_layouts = match builder.bind_group_layouts {
            Some(val) => val,
            None => vec![],
        };

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&format!("{} Pipeline Layout", &builder.name)),
            bind_group_layouts: bind_group_layouts.as_slice(),
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{} Render Pipeline", &builder.name)),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &builder.shader,
                entry_point: "vs_main",
                buffers,
            },
            primitive: builder.primitive,
            depth_stencil: builder.depth_stencil,
            multisample: builder.multisample,
            fragment: Some(wgpu::FragmentState {
                module: &builder.shader,
                entry_point: "fs_main",
                targets: &builder.fragment_targets,
            }),
            multiview: builder.multiview,
        });

        //----------------------------------------------

        info!("Successfully created new pipeline '{}'", &builder.name);

        //----------------------------------------------

        Self {
            name: builder.name,
            pipeline,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start_render_pass<'a: 'b, 'b>(
        &'a self,
        render_tools: &'a mut RenderPassTools,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    ) -> PipelineRenderPass<'b> {
        let mut render_pass = render_tools
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(&format!("{} Render Pass", &self.name)),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &render_tools.surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment,
            });

        render_pass.set_pipeline(&self.pipeline);

        PipelineRenderPass { render_pass }
    }
}

//===============================================================

pub struct PipelineRenderPass<'a> {
    render_pass: wgpu::RenderPass<'a>,
}
impl<'a> PipelineRenderPass<'a> {
    //----------------------------------------------

    pub fn set_bind_group(&mut self, index: u32, bind_group: &'a wgpu::BindGroup) {
        self.render_pass.set_bind_group(index, bind_group, &[]);
    }

    pub fn set_vertex_buffers<VertexBuffers: IntoIterator<Item = &'a wgpu::Buffer>>(
        &mut self,
        vertex_buffers: VertexBuffers,
        start_index: u32,
    ) {
        let mut index = start_index;

        for val in vertex_buffers.into_iter() {
            self.render_pass.set_vertex_buffer(index, val.slice(..));
            index += 1;
        }
    }

    pub fn set_index_buffer(&mut self, index_buffer: &'a wgpu::Buffer) {
        self.render_pass
            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    }

    //----------------------------------------------

    pub fn inner(&mut self) -> &mut wgpu::RenderPass<'a> {
        &mut self.render_pass
    }

    //----------------------------------------------

    pub fn draw<VertexBuffers: IntoIterator<Item = &'a wgpu::Buffer>>(
        &mut self,
        vertices: Range<u32>,
        instances: Range<u32>,
    ) {
        self.render_pass.draw(vertices, instances);
    }

    pub fn draw_index(&mut self, indices: Range<u32>, instances: Range<u32>) {
        self.render_pass.draw_indexed(indices, 0, instances);
    }

    //----------------------------------------------
}

//===============================================================
