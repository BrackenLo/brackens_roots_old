//===============================================================

use crossbeam::channel::TryRecvError;

use std::{collections::HashMap, sync::Arc};

use log::info;

use super::{
    asset_handle::{Handle, HandleID},
    Asset,
};

//===============================================================

pub type SenderType<T> = crossbeam::channel::Sender<T>;
pub type ReceiverType<T> = crossbeam::channel::Receiver<T>;

pub enum ReferenceCountSignal<T: Asset> {
    Increase(HandleID<T>),
    Decrease(HandleID<T>),
}

//===============================================================

pub struct AssetStorage<T: Asset> {
    sender: SenderType<ReferenceCountSignal<T>>,
    receiver: ReceiverType<ReferenceCountSignal<T>>,

    current_id: HandleID<T>,

    // The currently loaded data
    loaded: HashMap<HandleID<T>, Arc<T>>,

    // Keep track of how many strong handles in existance
    asset_count: HashMap<HandleID<T>, u32>,
    just_added: Vec<HandleID<T>>,
    removed_assets: Vec<HandleID<T>>,

    // Hashmap containing path to asset as a key. Used to check if data
    // is already loaded and if so, create a handle to it.
    loaded_paths: HashMap<String, HandleID<T>>,
    // Hashmap that is the opposite of loaded_paths used to access the file
    // path when unloading the data.
    // If a collection did key <-> key instead of key -> value exists, using
    // that would be preferable.
    asset_paths: HashMap<HandleID<T>, String>,
}

impl<T> AssetStorage<T>
where
    T: Asset,
{
    //----------------------------------------------

    pub fn new() -> Self {
        info!("Creating new {} asset storage", T::asset_name());

        let (sender, receiver) = crossbeam::channel::unbounded();
        Self {
            sender,
            receiver,
            current_id: HandleID::new(0),
            loaded: HashMap::new(),
            asset_count: HashMap::new(),
            just_added: Vec::new(),
            removed_assets: Vec::new(),

            loaded_paths: HashMap::new(),
            asset_paths: HashMap::new(),
        }
    }

    pub fn get_sender(&self) -> &SenderType<ReferenceCountSignal<T>> {
        &self.sender
    }

    pub fn get_loaded(&self) -> &HashMap<HandleID<T>, Arc<T>> {
        &self.loaded
    }

    //----------------------------------------------

    #[inline]
    fn get_next_id(&mut self) -> HandleID<T> {
        let to_return = self.current_id;
        self.current_id.id += 1;
        to_return
    }

    pub fn add_asset(&mut self, asset: T) -> Handle<T> {
        let next_id = self.get_next_id();
        let data_access = Arc::new(asset);

        self.loaded.insert(next_id, data_access.clone());
        self.asset_count.insert(next_id, 0);
        self.just_added.push(next_id);

        info!("Loaded new {} asset with id {}", T::asset_name(), next_id);

        Handle::strong(next_id, self.sender.clone(), data_access)
    }

    pub fn add_asset_file(&mut self, asset: T, path: String) -> Handle<T> {
        let handle = self.add_asset(asset);

        self.loaded_paths.insert(path.clone(), handle.id());
        self.asset_paths.insert(handle.id(), path);

        handle
    }

    pub fn get_handle(&self, id: &HandleID<T>) -> Option<Handle<T>> {
        match self.loaded.get(&id) {
            Some(data) => Some(Handle::strong(
                id.clone(),
                self.sender.clone(),
                data.clone(),
            )),
            None => None,
        }
    }

    pub fn get_loaded_file(&self, path: &str) -> Option<Handle<T>> {
        if let Some(id) = self.loaded_paths.get(path) {
            info!(
                "Retrieving previously loaded {} asset with id {}",
                T::asset_name(),
                id
            );
            let data_access = self.loaded.get(id).unwrap().clone();
            return Some(Handle::strong(*id, self.sender.clone(), data_access));
        }

        None
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
                    panic!(
                        "Error, {} Asset Storage channels have been disconnected",
                        T::asset_name()
                    );
                }
            };

            match data {
                ReferenceCountSignal::Increase(id) => {
                    *self.asset_count.get_mut(&id).unwrap() += 1;
                }
                ReferenceCountSignal::Decrease(id) => {
                    let count = self.asset_count.get_mut(&id).unwrap();
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
            info!(
                "Unloading {} asset with handle id {}",
                T::asset_name(),
                to_remove
            );

            self.loaded.remove(&to_remove); //Remove Asset
            self.asset_count.remove(&to_remove); //Remove Counter
        }

        for to_remove in &self.removed_assets {
            self.loaded_paths
                .remove(&self.asset_paths.remove(&to_remove).unwrap());
        }
    }

    pub fn clear_just_added(&mut self) {
        self.just_added.clear();
    }

    //----------------------------------------------

    pub fn get_just_added(&self) -> Vec<Handle<T>> {
        self.just_added
            .iter()
            .map(|id| Handle::weak(*id, self.loaded.get(id).unwrap().clone()))
            .collect()
    }

    pub fn get_removed_assets(&self) -> &Vec<HandleID<T>> {
        &self.removed_assets
    }

    //----------------------------------------------
}

//===============================================================
