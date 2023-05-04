//===============================================================

use crate::{
    pipelines::{instance_pipeline::RawInstancePipeline, Vertex},
    render_tools::RenderPassTools,
    textures::{LoadedTexture, Texture},
    Size,
};

//===============================================================

pub struct Material {
    pub name: String,
    pub texture: LoadedTexture,
}

pub struct Model {}

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
pub struct RawModelInstance {
    pub transform: [f32; 16],
    // Bones?
}
impl Vertex for RawModelInstance {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawModelInstance>() as wgpu::BufferAddress,
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
    pipeline: RawInstancePipeline,

    projection_bind_group: wgpu::BindGroup,
    projection_uniform_buffer: wgpu::Buffer,

    material_bind_group_layout: wgpu::BindGroupLayout,

    depth_texture: Texture,
}
impl ModelRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        todo!()
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

    pub fn render(
        &self,
        render_tools: &mut RenderPassTools,
        draw_calls: &[(&wgpu::BindGroup, &ModelDrawCall)],
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
        for draw_call in draw_calls {}
    }
}

pub struct ModelDrawCall {
    pub vertices: wgpu::Buffer,

    pub indices: wgpu::Buffer,
    pub index_count: u32,

    pub instances: wgpu::Buffer,
    pub instance_count: u32,
}

//===============================================================
