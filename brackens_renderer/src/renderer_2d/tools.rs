//===============================================================

use std::collections::{HashMap, HashSet};

use wgpu::util::DeviceExt;

use super::{RawTextureInstance, TextureDrawBuffer, TextureID};

//===============================================================

#[derive(Default)]
pub struct TextureProcessor {
    should_render: HashSet<TextureID>,
    unprocessed_draw_data: HashMap<TextureID, Vec<RawTextureInstance>>,

    draw_data: HashMap<TextureID, TextureDrawBuffer>,
}

impl TextureProcessor {
    //----------------------------------------------

    pub fn get_unprocessed_mut(&mut self) -> &mut HashMap<TextureID, Vec<RawTextureInstance>> {
        &mut self.unprocessed_draw_data
    }

    pub fn get_draw_data(&self) -> &HashMap<TextureID, TextureDrawBuffer> {
        &self.draw_data
    }

    //----------------------------------------------

    pub fn process_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.should_render.clear();

        for (id, data) in self.unprocessed_draw_data.iter() {
            let data_count = data.len() as u32;

            if data_count == 0 {
                continue;
            }
            self.should_render.insert(*id);

            // Get or create buffers
            self.draw_data
                .entry(*id)
                // Buffer exists. Set the new data.
                .and_modify(|buffer| {
                    // Buffer is too small to hold new data. Create a new, bigger one
                    if data_count > buffer.instance_count {
                        *buffer = Self::create_instance_buffer(device, data, &id.to_string());
                        ()
                    }
                    // Buffer is big enough. Write the new data to it
                    queue.write_buffer(&buffer.instance_buffer, 0, bytemuck::cast_slice(data));
                })
                // Buffer doesn't exist. Create a new one.
                .or_insert_with(|| Self::create_instance_buffer(device, data, &id.to_string()));
        }

        // Removed unused data
        self.draw_data.retain(|k, _| self.should_render.contains(k));
        self.unprocessed_draw_data.clear();
    }

    fn create_instance_buffer(
        device: &wgpu::Device,
        data: &[RawTextureInstance],
        label: &str,
    ) -> TextureDrawBuffer {
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("Texture Draw Buffer - {}", label)),
            contents: bytemuck::cast_slice(data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        TextureDrawBuffer {
            instance_buffer,
            instance_count: data.len() as u32,
        }
    }

    //----------------------------------------------
}

//===============================================================
