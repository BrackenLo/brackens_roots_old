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
// Standard implementation accounts for:
// - Vertex Position
// - Texture Coords
// - Instance position
// - rect color

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
pub struct RawTextureInstance {
    pub transform: [f32; 16],
    pub color: [f32; 4],
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

// Basic Vertices and Indices for rect

pub const TEXTURE_VERTICES: [RawTextureVertex; 4] = [
    // Bottom Left
    RawTextureVertex {
        position: [-0.5, -0.5, 0.0],
        tex_coord: [0.0, 1.0],
    },
    // Bottom Right
    RawTextureVertex {
        position: [0.5, -0.5, 0.0],
        tex_coord: [1.0, 1.0],
    },
    // Top Right
    RawTextureVertex {
        position: [0.5, 0.5, 0.0],
        tex_coord: [1.0, 0.0],
    },
    // Top Left
    RawTextureVertex {
        position: [-0.5, 0.5, 0.0],
        tex_coord: [0.0, 0.0],
    },
];
pub const TEXTURE_INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

//===============================================================

pub struct TextureDrawBuffer {
    pub instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

//===============================================================
