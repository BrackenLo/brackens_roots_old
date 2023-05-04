//===============================================================

use log::info;

use super::PipelineBuilderDescriptor;

//===============================================================

pub struct RawPipeline {
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

        Self { pipeline }
    }
}

//===============================================================
