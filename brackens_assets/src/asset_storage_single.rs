//===============================================================

use crossbeam::channel::TryRecvError;

use std::{collections::HashMap, sync::Arc};

// use log::info;

use crate::{asset_storage::ReferenceCountSignal, handle::HandleInner, ReceiverType, SenderType};

use super::{
    handle::{Handle, HandleID},
    Asset,
};

//===============================================================

pub struct AssetStorageSingle<T: Asset> {
    sender: SenderType<ReferenceCountSignal>,
    receiver: ReceiverType<ReferenceCountSignal>,

    current_id: HandleInner,

    // The currently loaded data
    loaded: HashMap<HandleInner, Arc<T>>,

    // Keep track of how many strong handles in existance
    handle_count: HashMap<HandleInner, u32>,
    just_added: Vec<HandleInner>,
    removed_assets: Vec<HandleInner>,

    // Hashmap containing path to asset as a key. Used to check if data
    // is already loaded and if so, create a handle to it.
    loaded_paths: HashMap<String, HandleInner>,
    // Hashmap that is the opposite of loaded_paths used to access the file
    // path when unloading the data.
    // If a collection did key <-> key instead of key -> value exists, using
    // that would be preferable.
    asset_paths: HashMap<HandleInner, String>,
}

impl<T> Default for AssetStorageSingle<T>
where
    T: Asset,
{
    fn default() -> Self {
        // info!("Creating new {} asset storage", T::asset_name());

        let (sender, receiver) = crossbeam::channel::unbounded();
        Self {
            sender,
            receiver,
            current_id: HandleInner::from_id(0),
            loaded: HashMap::new(),
            handle_count: HashMap::new(),
            just_added: Vec::new(),
            removed_assets: Vec::new(),

            loaded_paths: HashMap::new(),
            asset_paths: HashMap::new(),
        }
    }
}

impl<T> AssetStorageSingle<T>
where
    T: Asset,
{
    //----------------------------------------------

    pub fn new() -> Self {
        Self::default()
    }

    //----------------------------------------------

    fn get_next_id(&mut self) -> HandleInner {
        let to_return = self.current_id;
        self.current_id.next();
        to_return
    }

    pub fn add_asset(&mut self, asset: T) -> Handle<T> {
        let id = self.get_next_id();
        let handle_inner = Arc::new(asset);

        self.loaded.insert(id, handle_inner.clone());
        self.handle_count.insert(id, 0);
        self.just_added.push(id);

        let handle_id = HandleID::new(id);
        Handle::new(handle_id, self.sender.clone(), handle_inner)
    }

    pub fn add_asset_file<P: AsRef<str>>(&mut self, asset: T, path: P) -> Handle<T> {
        if let Some(handle) = self.get_loaded_file(path.as_ref()) {
            return handle;
        }

        let handle = self.add_asset(asset);

        let path = path.as_ref().to_string();

        self.loaded_paths.insert(path.clone(), handle.inner_id());
        self.asset_paths.insert(handle.inner_id(), path);

        handle
    }

    //----------------------------------------------

    pub fn get_handle<HI: Into<HandleInner>>(&self, id: HI) -> Option<Handle<T>> {
        let id = id.into();
        match self.loaded.get(&id) {
            Some(inner) => {
                let handle_id = HandleID::new(id);
                Some(
                    Handle::new(handle_id, self.sender.clone(), inner.clone()), // Handle::strong(
                                                                                // id.clone(),
                                                                                // self.sender.clone(),
                                                                                // data.clone(),
                )
            }
            None => None,
        }
    }

    pub fn is_file_loaded(&self, path: &str) -> bool {
        self.loaded_paths.contains_key(path)
    }

    pub fn get_loaded_file(&self, path: &str) -> Option<Handle<T>> {
        if let Some(id) = self.loaded_paths.get(path) {
            // info!(
            //     "Retrieving previously loaded {} asset with id {}",
            //     T::asset_name(),
            //     id
            // );
            let handle_inner = self.loaded.get(id).unwrap().clone();
            let handle_id = HandleID::new(*id);
            return Some(Handle::new(handle_id, self.sender.clone(), handle_inner));
        }

        None
    }

    pub fn get_data<HI: Into<HandleInner>>(&self, id: HI) -> Option<&T> {
        let id = id.into();
        match self.loaded.get(&id) {
            Some(val) => Some(val.as_ref()),
            None => None,
        }
    }

    //----------------------------------------------

    pub fn tick(&mut self) {
        self.check_asset_changes();
        self.remove_pending_assets();

        self.clear_just_added();
    }

    pub fn check_asset_changes(&mut self) {
        self.removed_assets.clear();

        // Loop through each recieved signal and act accordingly
        loop {
            let data = match self.receiver.try_recv() {
                Ok(data) => data,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    // panic!(
                    //     "Error, {} Asset Storage channels have been disconnected",
                    //     T::asset_name()
                    // );
                    todo!();
                }
            };

            match data {
                ReferenceCountSignal::Increase(id) => {
                    *self.handle_count.get_mut(&id).unwrap() += 1;
                }
                ReferenceCountSignal::Decrease(id) => {
                    let count = self.handle_count.get_mut(&id).unwrap();
                    *count -= 1;
                    if *count == 0 {
                        self.removed_assets.push(id);
                    }
                }
            }
        }
    }

    pub fn remove_pending_assets(&mut self) {
        for to_remove in &self.removed_assets {
            // info!(
            //     "Unloading {} asset with handle id {}",
            //     T::asset_name(),
            //     to_remove
            // );

            self.loaded.remove(&to_remove); //Remove Asset
            self.handle_count.remove(&to_remove); //Remove Counter
        }

        for to_remove in &self.removed_assets {
            match &self.asset_paths.remove(&to_remove) {
                Some(val) => {
                    self.loaded_paths.remove(val);
                }
                None => {}
            }
        }
    }

    pub fn clear_just_added(&mut self) {
        self.just_added.clear();
    }

    //----------------------------------------------

    pub fn get_just_added(&self) -> Vec<Handle<T>> {
        self.just_added
            .iter()
            .map(|id| {
                let handle_id = HandleID::new(*id);
                let inner = self.loaded.get(id).unwrap().clone();
                Handle::new(handle_id, self.sender.clone(), inner)
            })
            .collect()
    }

    pub fn get_removed_assets(&self) -> &Vec<HandleInner> {
        &self.removed_assets
    }

    //----------------------------------------------
}

//===============================================================
