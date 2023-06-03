//===============================================================

use std::marker::PhantomData;

use wgpu::util::DeviceExt;

//===============================================================

pub enum BindGroupEntryTypes<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    Buffer(BufferTemplate<T>),
    Sampler,
    TextureView,
}

pub enum BindGroupEntry<'a, T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    Buffer(T),
    Sampler(&'a wgpu::Sampler),
    TextureView(&'a wgpu::TextureView),
}

//===============================================================

pub struct BindGroupTemplate<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    layout: wgpu::BindGroupLayout,
    label: String,
    entries: Vec<BindGroupEntryTypes<T>>,
}

impl<T> BindGroupTemplate<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        data: Vec<BindGroupEntry<T>>,
    ) -> wgpu::BindGroup {
        let mut binding = 0;
        let mut entries = Vec::new();

        for entry in &self.entries {
            // Get the value for this entry
            let value = match data.get(binding) {
                Some(val) => val,
                None => {
                    panic!("Error: Invalid number of parameters for create bind group template")
                }
            };

            let resource = match (entry, value) {
                (BindGroupEntryTypes::Buffer(template), BindGroupEntry::Buffer(data)) => {
                    // wgpu::BindingResource::Buffer()
                    todo!()
                }
                (BindGroupEntryTypes::Sampler, BindGroupEntry::Sampler(sampler)) => {
                    wgpu::BindingResource::Sampler(sampler)
                }
                (BindGroupEntryTypes::TextureView, BindGroupEntry::TextureView(view)) => {
                    wgpu::BindingResource::TextureView(view)
                }
                _ => panic!("Error: Incompatible Parameter type for create bind group template"),
            };

            entries.push(wgpu::BindGroupEntry {
                binding: binding as u32,
                resource,
            });
            binding += 1;
        }

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&self.label),
            layout: &self.layout,
            entries: entries.as_slice(),
        })
    }
}

//===============================================================

pub struct BufferTemplate<T: bytemuck::Pod + bytemuck::Zeroable> {
    phantom: PhantomData<T>,
    label: String,
    usage: wgpu::BufferUsages,
}

impl<T> Default for BufferTemplate<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            label: "Unnamed Template Buffer".into(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    }
}

impl<T> BufferTemplate<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn new<Name: AsRef<str>>(label: Name) -> Self {
        Self {
            label: label.as_ref().into(),
            ..Default::default()
        }
    }
    pub fn new_usages<Name: AsRef<str>>(label: Name, usage: wgpu::BufferUsages) -> Self {
        Self {
            phantom: PhantomData,
            label: label.as_ref().into(),
            usage,
        }
    }

    pub fn create_buffer(&self, device: &wgpu::Device, data: T) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Buffer Template: {}", &self.label)),
            contents: bytemuck::cast_slice(&[data]),
            usage: self.usage,
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
