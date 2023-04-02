//===============================================================

use log::info;
use wgpu::util::DeviceExt;

use super::render_tools::RenderPassTools;

//===============================================================

pub trait Vertex: bytemuck::Pod {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a>;
}

pub struct PipelineBuilderDescriptor {
    pub name: String,
    pub bind_group_layouts: Option<Vec<wgpu::BindGroupLayout>>,
    pub shader: wgpu::ShaderModule,
    pub primitive: wgpu::PrimitiveState,
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    pub multisample: wgpu::MultisampleState,
    pub fragment_targets: Vec<Option<wgpu::ColorTargetState>>,
    pub multiview: Option<std::num::NonZeroU32>,
}

//===============================================================

//===============================================================

pub struct RawInstancePipeline {
    name: String,
    pipeline: wgpu::RenderPipeline,

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
        info!("Creating new pipeline '{}'", &builder.name);

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

        let bind_group_layouts = if let Some(bind_group_layouts) = &builder.bind_group_layouts {
            bind_group_layouts.iter().collect::<Vec<_>>()
        } else {
            vec![]
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
                buffers: &[VB::buffer_layout(), IB::buffer_layout()],
            },
            primitive: builder.primitive,
            depth_stencil: builder.depth_stencil.clone(),
            multisample: builder.multisample,
            fragment: Some(wgpu::FragmentState {
                module: &builder.shader,
                entry_point: "fs_main",
                targets: builder.fragment_targets.as_slice(),
            }),
            multiview: builder.multiview,
        });

        //----------------------------------------------

        info!("Successfully created new Pipeline '{}'", &builder.name);

        Self {
            name: builder.name,
            pipeline,
            vertex_buffer,
            index_buffer,
            index_count,
        }

        //----------------------------------------------
    }
}

pub struct PipelineRenderPass<'a> {
    render_pass: wgpu::RenderPass<'a>,
    index_count: u32,
}
impl<'a> PipelineRenderPass<'a> {
    pub fn set_bind_group(&mut self, index: u32, bind_group: &'a wgpu::BindGroup) {
        self.render_pass.set_bind_group(index, bind_group, &[]);
    }

    pub fn draw_instanced<InstanceBuffers: IntoIterator<Item = &'a wgpu::Buffer>>(
        &mut self,
        instance_buffers: InstanceBuffers,
        instance_count: u32,
    ) {
        let mut index = 1;
        for val in instance_buffers.into_iter() {
            self.render_pass.set_vertex_buffer(index, val.slice(..));
            index += 1;
        }

        self.render_pass
            .draw_indexed(0..self.index_count, 0, 0..instance_count);
    }
}

impl RawInstancePipeline {
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
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        PipelineRenderPass {
            render_pass,
            index_count: self.index_count,
        }
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
            label: Some(&format!("{} Instance Buffer", &self.pipeline.name)),
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
