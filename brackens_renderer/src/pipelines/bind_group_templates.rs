//===============================================================

use std::marker::PhantomData;

use wgpu::util::DeviceExt;

//===============================================================

pub enum BindGroupEntryType<T>
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

pub struct BindGroupEntryLayout<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub entry_type: BindGroupEntryType<T>,
    pub visibility: wgpu::ShaderStages,
}

//===============================================================

pub struct BindGroupTemplate<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    label: String,
    layout: wgpu::BindGroupLayout,
    layout_entries: Vec<BindGroupEntryType<T>>,
}

impl<T> BindGroupTemplate<T>
where
    T: bytemuck::Pod + bytemuck::Zeroable,
{
    pub fn new(
        device: &wgpu::Device,
        label: &str,
        new_layout: Vec<BindGroupEntryLayout<T>>,
    ) -> Self {
        let mut entries = Vec::new();
        let mut binding = 0;

        let mut layout_entries = Vec::new();

        for entry in new_layout {
            let ty = match entry.entry_type {
                BindGroupEntryType::Buffer(val) => {
                    layout_entries.push(BindGroupEntryType::Buffer(val));

                    wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    }
                }
                BindGroupEntryType::Sampler => {
                    layout_entries.push(BindGroupEntryType::Sampler);
                    wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
                }
                BindGroupEntryType::TextureView => {
                    layout_entries.push(BindGroupEntryType::TextureView);
                    wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    }
                }
            };

            entries.push(wgpu::BindGroupLayoutEntry {
                binding,
                visibility: entry.visibility,
                ty,
                count: None,
            });
            binding += 1;
        }

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&format!("{} - Bind Group Template Layout", label)),
            entries: &entries,
        });

        Self {
            label: label.into(),
            layout,
            layout_entries,
        }
    }

    pub fn get_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    pub fn create_bind_group(
        &self,
        device: &wgpu::Device,
        data: Vec<BindGroupEntry<T>>,
    ) -> (wgpu::BindGroup, Option<wgpu::Buffer>) {
        let mut binding = 0;
        let mut entries = Vec::new();

        let mut buffer = None;

        // Create buffers ahead of time due to immutible borrow errors
        for value in &data {
            let entry = match self.layout_entries.get(binding) {
                Some(val) => val,
                None => {
                    panic!(
                        "Error: Invalid number of parameters for create bind group template - {}",
                        self.label
                    )
                }
            };

            match (entry, value) {
                (BindGroupEntryType::Buffer(template), BindGroupEntry::Buffer(data)) => {
                    if buffer.is_some() {
                        panic!(
                            "Error: Only one uniform buffer is allowed in bind group template - {}",
                            self.label
                        )
                    }
                    buffer = Some(template.create_buffer(device, data));
                }
                _ => {}
            }
        }

        // Go through all values, make sure they're of the right type and create the wgpu equivelent
        for value in &data {
            let entry = match self.layout_entries.get(binding) {
                Some(val) => val,
                None => {
                    panic!(
                        "Error: Invalid number of parameters for create bind group template - {}",
                        self.label
                    )
                }
            };

            let resource = match (entry, value) {
                (BindGroupEntryType::Buffer(_), BindGroupEntry::Buffer(_)) => {
                    wgpu::BindingResource::Buffer(
                        buffer.as_ref().unwrap().as_entire_buffer_binding(),
                    )
                }
                (BindGroupEntryType::Sampler, BindGroupEntry::Sampler(sampler)) => {
                    wgpu::BindingResource::Sampler(sampler)
                }
                (BindGroupEntryType::TextureView, BindGroupEntry::TextureView(view)) => {
                    wgpu::BindingResource::TextureView(view)
                }
                _ => panic!(
                    "Error: Incompatible Parameter type for create bind group template {}",
                    self.label
                ),
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
        (bind_group, buffer)
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
