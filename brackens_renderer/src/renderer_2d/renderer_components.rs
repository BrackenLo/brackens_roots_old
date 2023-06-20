//===============================================================

use crate::pipelines::{
    bind_group_templates::{BindGroupEntry, BindGroupTemplate},
    Vertex,
};

//===============================================================

pub struct RendererDescriptor2D<'a, T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub device: &'a wgpu::Device,
    pub format: wgpu::TextureFormat,

    // How data used for all render passes should be laid out
    pub global_bind_group_template: BindGroupTemplate<T>,
    // The data used for all render passes. Should follow the template above.
    pub data: Vec<BindGroupEntry<'a, T>>,

    pub shader: wgpu::ShaderSource<'a>,
    pub use_depth_texture: bool,
    pub label: &'a str,
}

//===============================================================

// Rect vertex and instance layouts

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawTextureVertex {
    position: [f32; 3],
}
impl Vertex for RawTextureVertex {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawTextureVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            }],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawTextureInstance {
    pub tex_coord_top_left: [f32; 2],
    pub tex_coord_bottom_right: [f32; 2],
    pub transform: [f32; 16],
    pub color: [f32; 4],
}
impl Default for RawTextureInstance {
    fn default() -> Self {
        Self {
            tex_coord_top_left: [0., 0.],
            tex_coord_bottom_right: [1., 1.],
            transform: glam::Mat4::IDENTITY.to_cols_array(),
            color: [1., 1., 1., 1.],
        }
    }
}
impl Vertex for RawTextureInstance {
    fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RawTextureInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // Tex Coord f32x2
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 1,
                },
                // Tex Coord f32x2
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 2,
                },
                // Transform 1 f32x4
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                },
                // Transform 2 f32x4
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                },
                // Transform 3 f32x4
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
                // Transform 4 f32x4
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
                // Color f32x4
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<[f32; 20]>() as wgpu::BufferAddress,
                    shader_location: 7,
                },
            ],
        }
    }
}

//===============================================================

// Basic Vertices and Indices for rect

pub const TEXTURE_VERTICES: [RawTextureVertex; 4] = [
    // Bottom Left
    RawTextureVertex {
        position: [-0.5, -0.5, 0.0],
    },
    // Bottom Right
    RawTextureVertex {
        position: [0.5, -0.5, 0.0],
    },
    // Top Right
    RawTextureVertex {
        position: [0.5, 0.5, 0.0],
    },
    // Top Left
    RawTextureVertex {
        position: [-0.5, 0.5, 0.0],
    },
];
pub const TEXTURE_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

//===============================================================

pub struct TextureDrawBuffer {
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

//===============================================================
