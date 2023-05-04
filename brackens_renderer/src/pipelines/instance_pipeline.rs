//===============================================================

use log::info;
use wgpu::util::DeviceExt;

use crate::render_tools::RenderPassTools;

use super::{raw_pipeline::PipelineRenderPass, PipelineBuilderDescriptor, RawPipeline, Vertex};

//===============================================================

pub struct RawInstancePipeline {
    pipeline: RawPipeline,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl RawInstancePipeline {
    pub fn new<VB: Vertex, IB: Vertex>(
        device: &wgpu::Device,
        builder: PipelineBuilderDescriptor,
        vertex_data: &[VB],
        index_data: &[u16],
    ) -> Self {
        info!("Creating new instance pipeline '{}'", &builder.name);

        //----------------------------------------------

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Vertex Buffer", &builder.name)),
            contents: bytemuck::cast_slice(vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Index Buffer", &builder.name)),
            contents: bytemuck::cast_slice(index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let index_count = index_data.len() as u32;

        //----------------------------------------------

        let pipeline =
            RawPipeline::new(device, &[VB::buffer_layout(), IB::buffer_layout()], builder);

        //----------------------------------------------

        // info!(
        //     "Successfully created new instance Pipeline '{}'",
        //     &builder.name
        // );

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            index_count,
        }

        //----------------------------------------------
    }

    pub fn name(&self) -> &str {
        &self.pipeline.name()
    }

    pub fn start_render_pass<'a: 'b, 'b>(
        &'a self,
        render_tools: &'a mut RenderPassTools,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    ) -> InstancePipelineRenderPass<'b> {
        let render_pass = self
            .pipeline
            .start_render_pass(render_tools, depth_stencil_attachment);

        InstancePipelineRenderPass::new(
            render_pass,
            &self.vertex_buffer,
            &self.index_buffer,
            self.index_count,
        )
    }

    pub fn render<
        'a,
        InstanceBuffers: IntoIterator<Item = &'a wgpu::Buffer>,
        BindGroups: IntoIterator<Item = &'a wgpu::BindGroup>,
    >(
        &'a self,
        render_tools: &'a mut RenderPassTools,
        instance_buffers: InstanceBuffers,
        instance_count: u32,
        bind_groups: BindGroups,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    ) {
        let mut render_pass = self.start_render_pass(render_tools, depth_stencil_attachment);

        let mut index = 0;
        for bind_group in bind_groups.into_iter() {
            render_pass.set_bind_group(index, bind_group);
            index += 1;
        }

        render_pass.draw_instanced(instance_buffers, instance_count);
    }
}

//===============================================================

pub struct InstancePipelineRenderPass<'a> {
    render_pass: PipelineRenderPass<'a>,
    index_count: u32,
}
impl<'a> InstancePipelineRenderPass<'a> {
    pub fn new(
        mut render_pass: PipelineRenderPass<'a>,
        vertex_buffer: &'a wgpu::Buffer,
        index_buffer: &'a wgpu::Buffer,
        index_count: u32,
    ) -> Self {
        render_pass.set_vertex_buffers(Some(vertex_buffer), 0);
        render_pass.set_index_buffer(index_buffer);

        Self {
            render_pass,
            index_count,
        }
    }

    pub fn set_bind_group(&mut self, index: u32, bind_group: &'a wgpu::BindGroup) {
        self.render_pass.set_bind_group(index, bind_group);
    }

    pub fn draw_instanced<InstanceBuffers: IntoIterator<Item = &'a wgpu::Buffer>>(
        &mut self,
        instance_buffers: InstanceBuffers,
        instance_count: u32,
    ) {
        self.render_pass.set_vertex_buffers(instance_buffers, 1);
        self.render_pass
            .draw_index(0..self.index_count, 0..instance_count);
    }
}

//===============================================================
//===============================================================
//===============================================================

pub struct InstancePipeline {
    pipeline: RawInstancePipeline,

    instance_buffer: wgpu::Buffer,
    instance_count: u32,
}

impl InstancePipeline {
    pub fn new<VB: Vertex, IB: Vertex>(
        device: &wgpu::Device,
        builder: PipelineBuilderDescriptor,
        vertex_data: &[VB],
        index_data: &[u16],
    ) -> Self {
        //----------------------------------------------

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{} Instance Buffer", &builder.name)),
            size: 0,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let instance_count = 0;

        let pipeline = RawInstancePipeline::new::<VB, IB>(device, builder, vertex_data, index_data);

        //----------------------------------------------

        Self {
            pipeline,
            instance_buffer,
            instance_count,
        }

        //----------------------------------------------
    }

    pub fn update_instance<IB: Vertex>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[IB],
    ) {
        if data.len() == 0 {
            return;
        } else if data.len() <= self.instance_count as usize {
            queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(data));
            return;
        }

        // Data has outgrown the instance buffer. We need to make a new, larger buffer.
        self.instance_count = data.len() as u32;
        self.instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Instance Buffer", &self.pipeline.name())),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
    }

    pub fn render<'a, BindGroups: IntoIterator<Item = &'a wgpu::BindGroup>>(
        &'a self,
        render_tools: &'a mut RenderPassTools,
        bind_groups: BindGroups,
        depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'a>>,
    ) {
        self.pipeline.render(
            render_tools,
            Some(&self.instance_buffer),
            self.instance_count,
            bind_groups,
            depth_stencil_attachment,
        );
    }
}

//===============================================================
