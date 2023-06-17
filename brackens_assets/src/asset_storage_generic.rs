//===============================================================

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fs::File,
    io::Read,
    sync::Arc,
};

use crate::Asset;

// use crate::Asset;

//===============================================================

// use crate::Asset;

pub struct FromData {}
impl FromData {
    pub fn create_data(&self) -> MyData {
        todo!()
    }
}

pub struct MyData {}
impl Asset for MyData {
    fn asset_name(&self) -> &str {
        todo!()
    }
}

//===============================================================

pub struct Handle<T: Asset> {
    handle_id: HandleInner,
    asset: Arc<T>,
}
// impl

#[derive(std::hash::Hash, PartialEq, Eq)]
pub struct HandleInner(u32);

//===============================================================

pub enum ReferenceCountSignalX {
    Increase(HandleInner),
    Decrease(HandleInner),
}

pub struct StorageInner {
    sender: crossbeam::channel::Sender<ReferenceCountSignalX>,
    receiver: crossbeam::channel::Receiver<ReferenceCountSignalX>,

    current_id: HandleInner,

    loaded: HashMap<HandleInner, Arc<dyn Asset>>,
}
impl StorageInner {
    pub fn create_handle<T: Asset>(&self, id: HandleInner) -> anyhow::Result<Handle<T>> {
        // let val = self.loaded.get(&id).unwrap?;

        let val = self.loaded.get(&id).unwrap();
        let inner = val.clone();

        // let inner = inner.into_any_arc();
        let inner: Arc<T> = match inner.into_any_arc().downcast() {
            Ok(val) => val,
            Err(_) => panic!("Error"),
        };

        let handle = Handle {
            handle_id: id,
            asset: inner,
        };

        Ok(handle)
    }
}

//===============================================================

pub struct AssetStorageX {
    inner: HashMap<TypeId, StorageInner>,
}

impl AssetStorageX {
    pub fn add_type<T: Asset>(&mut self) {
        let type_id = TypeId::of::<T>();
    }

    pub fn load_from_data<T: Asset, F: Fn(&[u8]) -> T>(
        &mut self,
        path: &str,
        load_data: F,
    ) -> anyhow::Result<Handle<T>> {
        let mut file = File::open(path)?;
        let mut bytes = vec![];

        let _count = file.read_to_end(&mut bytes)?;

        let data = load_data(&bytes);

        todo!();
    }
}

//===============================================================
