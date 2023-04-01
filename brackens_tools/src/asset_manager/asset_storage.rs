//===============================================================

use crossbeam::channel::TryRecvError;

use std::{collections::HashMap, sync::Arc};

use log::info;

use super::{
    asset_handle::{Handle, HandleID},
    Asset, AssetLoadable,
};

//===============================================================

pub type SenderType<T> = crossbeam::channel::Sender<T>;
pub type ReceiverType<T> = crossbeam::channel::Receiver<T>;

pub enum ReferenceCountSignal {
    Increase(HandleID),
    Decrease(HandleID),
}

//===============================================================

pub struct AssetStorage<T: Asset> {
    sender: SenderType<ReferenceCountSignal>,
    receiver: ReceiverType<ReferenceCountSignal>,

    current_id: HandleID,

    // The currently loaded data
    loaded: HashMap<HandleID, Arc<T>>,
    // Hashmap containing path to asset as a key. Used to check if data
    // is already loaded and if so, create a handle to it.
    loaded_paths: HashMap<String, HandleID>,
    // Hashmap that is the opposite of loaded_paths used to access the file
    // path when unloading the data.
    // If a collection did key <-> key instead of key -> value exists, using
    // that would be preferable.
    asset_paths: HashMap<HandleID, String>,

    // Keep track of how many strong handles in existance
    asset_count: HashMap<HandleID, u32>,
    just_added: Vec<HandleID>,
    pending_removal: Vec<HandleID>,

    load_path: String,
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
            current_id: HandleID(0),
            loaded: HashMap::new(),
            loaded_paths: HashMap::new(),
            asset_paths: HashMap::new(),
            asset_count: HashMap::new(),
            just_added: Vec::new(),
            pending_removal: Vec::new(),

            load_path: "".into(),
        }
    }

    //----------------------------------------------

    #[inline]
    fn get_next_id(&mut self) -> HandleID {
        let to_return = self.current_id;
        self.current_id.0 += 1;
        to_return
    }

    pub fn load_asset(&mut self, asset: T) -> Handle<T> {
        let next_id = self.get_next_id();
        let data_access = Arc::new(asset);

        self.loaded.insert(next_id, data_access.clone());
        self.asset_count.insert(next_id, 0);
        self.just_added.push(next_id);

        info!("Loaded new {} asset with id {}", T::asset_name(), next_id);

        Handle::strong(next_id, self.sender.clone(), data_access)
    }

    //----------------------------------------------

    pub fn tick(&mut self) {
        self.check_asset_changes();
        self.remove_pending_assets();

        self.just_added.clear();
    }

    pub fn check_asset_changes(&mut self) {
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
                        self.pending_removal.push(id);
                    }
                }
            }
        }
    }

    pub fn remove_pending_assets(&mut self) {
        for to_remove in &self.pending_removal {
            info!(
                "Unloading {} asset with handle id {}",
                T::asset_name(),
                to_remove
            );

            self.loaded.remove(&to_remove); //Remove Asset
            self.asset_count.remove(&to_remove); //Remove Counter

            // Remove id -> path and then path -> id
            self.loaded_paths
                .remove(&self.asset_paths.remove(&to_remove).unwrap());
        }
        self.pending_removal.clear();
    }

    //----------------------------------------------

    // pub fn get_pending_removal(&self) -> Vec<Handle<T>> {
    //     self.pending_removal
    //         .iter()
    //         .map(|id| Handle::weak(*id, self.loaded.get(id).unwrap().clone()))
    //         .collect()
    // }

    pub fn get_just_added(&self) -> Vec<Handle<T>> {
        self.just_added
            .iter()
            .map(|id| Handle::weak(*id, self.loaded.get(id).unwrap().clone()))
            .collect()
    }

    //----------------------------------------------
}
impl<T> AssetStorage<T>
where
    T: AssetLoadable,
{
    pub fn load_from_file(&mut self, path: String) -> Handle<T> {
        info!("Loading new {} asset from path {}", T::asset_name(), path);

        // Check if file is already loaded. If so, we can create an new handle to
        // the existing data.
        if let Some(id) = self.loaded_paths.get(&path) {
            info!("Asset already loaded with id {}", id);
            let data_access = self.loaded.get(id).unwrap().clone();
            return Handle::strong(*id, self.sender.clone(), data_access);
        }

        // Otherwise, load and store the data accordingly
        let data = T::load_from_file(format!("{}{}", self.load_path, path.clone()));
        let next_id = self.get_next_id();
        let data_access = Arc::new(data);

        self.loaded.insert(next_id, data_access.clone());
        self.asset_count.insert(next_id, 0);
        self.just_added.push(next_id);

        self.loaded_paths.insert(path.clone(), next_id);
        self.asset_paths.insert(next_id, path);

        info!("Loaded {} asset with id {}", T::asset_name(), next_id,);

        Handle::strong(next_id, self.sender.clone(), data_access)
    }
}

//===============================================================
