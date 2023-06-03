//===============================================================

use std::marker::PhantomData;

use wgpu::util::DeviceExt;

//===============================================================

pub struct BindGroupTemplate {}

//===============================================================

pub struct BufferTemplate<T: bytemuck::Pod + bytemuck::Zeroable> {
    phantom: PhantomData<T>,
    buffer_name: String,
    buffer_usage: wgpu::BufferUsages,
}

impl<T> Default for BufferTemplate<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            buffer_name: "Unnamed Template Buffer".into(),
            buffer_usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    }
}

impl<T> BufferTemplate<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn new<Name: AsRef<str>>(buffer_name: Name) -> Self {
        Self {
            buffer_name: buffer_name.as_ref().into(),
            ..Default::default()
        }
    }
    pub fn new_usages(buffer_name: String, buffer_usage: wgpu::BufferUsages) -> Self {
        Self {
            phantom: PhantomData,
            buffer_name,
            buffer_usage,
        }
    }

    pub fn create_buffer(&self, device: &wgpu::Device, data: T) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Buffer Template: {}", &self.buffer_name)),
            contents: bytemuck::cast_slice(&[data]),
            usage: self.buffer_usage,
        })
    }

    pub fn update_buffer(&self, queue: &wgpu::Queue, buffer: &wgpu::Buffer, data: T) {
        queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[data]));
    }
}

// Example ?
// pub fn create_projection(device: &wgpu::Device, matrix: glam::Mat4) -> wgpu::Buffer {
//     let template = BufferTemplate::<[f32; 16]>::new("Projection Buffer");
//     template.create_buffer(device, matrix.to_cols_array());

//     todo!()
// }

//===============================================================
