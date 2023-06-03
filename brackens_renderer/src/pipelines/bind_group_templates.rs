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
    ) -> (wgpu::BindGroup, Vec<wgpu::Buffer>) {
        let mut binding = 0;
        let mut entries = Vec::new();

        let mut buffer_index = 0;
        let mut buffers = Vec::new();

        // Create buffers ahead of time due to immutible borrow errors
        for value in &data {
            let entry = match self.entries.get(binding) {
                Some(val) => val,
                None => {
                    panic!("Error: Invalid number of parameters for create bind group template")
                }
            };

            match (entry, value) {
                (BindGroupEntryTypes::Buffer(template), BindGroupEntry::Buffer(data)) => {
                    let buffer = template.create_buffer(device, data);
                    buffers.push(buffer);
                }
                _ => {}
            }
        }

        // Go through all values, make sure they're of the right type and create the wgpu equivelent
        for value in &data {
            let entry = match self.entries.get(binding) {
                Some(val) => val,
                None => {
                    panic!("Error: Invalid number of parameters for create bind group template")
                }
            };

            let resource = match (entry, value) {
                (BindGroupEntryTypes::Buffer(_), BindGroupEntry::Buffer(_)) => {
                    buffer_index += 1;
                    wgpu::BindingResource::Buffer(
                        buffers[buffer_index - 1].as_entire_buffer_binding(),
                    )
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

        // Create the bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&self.label),
            layout: &self.layout,
            entries: entries.as_slice(),
        });

        // Return the bind group and any used buffers
        (bind_group, buffers)
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

    pub fn create_buffer(&self, device: &wgpu::Device, data: &T) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Buffer Template: {}", &self.label)),
            contents: bytemuck::cast_slice(&[*data]),
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
