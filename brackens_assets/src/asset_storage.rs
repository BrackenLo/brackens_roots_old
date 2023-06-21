//===============================================================

use std::{collections::HashMap, fs::File, io::Read, sync::Arc};

use crossbeam::channel::TryRecvError;

use crate::{
    handle::{Handle, HandleID, HandleInner},
    Asset, AssetFileLoadable,
};

//===============================================================

#[derive(Debug)]
pub enum AssetStorageError {
    AssetNotExist,
    AssetIsDifferentType,
}
impl std::fmt::Display for AssetStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AssetStorageError::AssetNotExist => {
                write!(f, "AssetStorageError: The requested asset does not exist")
            }
            AssetStorageError::AssetIsDifferentType => write!(
                f,
                "AssetStorageError: The given type doesn't match the stored type"
            ),
        }
    }
}
impl std::error::Error for AssetStorageError {}

//===============================================================

pub enum ReferenceCountSignal {
    Increase(HandleInner),
    Decrease(HandleInner),
}

//===============================================================

pub struct AssetStorage {
    sender: crossbeam::channel::Sender<ReferenceCountSignal>,
    receiver: crossbeam::channel::Receiver<ReferenceCountSignal>,

    current_id: HandleInner,
    loaded: HashMap<HandleInner, Arc<dyn Asset>>,

    // Hashmap containing path to asset as a key. Used to check if data is already loaded
    // and if so, create a handle to it.
    loaded_paths: HashMap<String, HandleInner>,
    // Hashmap that is the opposite of loaded_paths, used to access the file path when
    // unloading the data.
    asset_paths: HashMap<HandleInner, String>,

    // Path to load assets from
    load_path: String,

    // Keep track of how many handles in existance so we can unload when finished with.
    handle_count: HashMap<HandleInner, u32>,

    removed_assets: Vec<HandleInner>,
}

impl Default for AssetStorage {
    fn default() -> Self {
        let (sender, receiver) = crossbeam::channel::unbounded();
        Self {
            sender,
            receiver,
            current_id: HandleInner::from_id(0),
            loaded: HashMap::new(),

            loaded_paths: HashMap::new(),
            asset_paths: HashMap::new(),
            load_path: "".into(),

            handle_count: HashMap::new(),
            removed_assets: Vec::new(),
        }
    }
}

impl AssetStorage {
    //----------------------------------------------

    pub fn new() -> Self {
        Self::default()
    }

    //----------------------------------------------

    fn get_next_id(&mut self) -> HandleInner {
        let id = self.current_id;
        self.current_id.next();
        id
    }

    //----------------------------------------------

    fn insert_data_path<T: Asset>(&mut self, data: T, path: &str) -> Handle<T> {
        let handle = self.insert_data(data);

        self.loaded_paths.insert(path.into(), handle.inner_id());
        self.asset_paths.insert(handle.inner_id(), path.into());

        handle
    }

    fn insert_data<T: Asset>(&mut self, data: T) -> Handle<T> {
        let id = self.get_next_id();

        let handle_inner = Arc::new(data);

        // Add references to data to storage
        self.loaded.insert(id, handle_inner.clone());
        self.handle_count.insert(id, 0);

        // Construct Handle and return
        let handle_id = HandleID::new(id);
        let handle = Handle::new(handle_id, self.sender.clone(), handle_inner);
        handle
    }

    //----------------------------------------------

    /// Check if asset is already loaded. If so, get a new handle and return it.
    /// If problem exists type casting asset, return custom error.
    /// Otherwise, return None
    fn load_file_get_already_loaded<T: Asset>(
        &mut self,
        path: &str,
    ) -> Result<Option<Handle<T>>, AssetStorageError> {
        //
        match self.get_file_loaded(path) {
            Ok(handle) => return Ok(Some(handle)),
            Err(e) => match e {
                // Only error if the type cast is invalid
                // AssetStorageError::InvalidType => Err(AssetStorageError::AssetExistsDifferentType)?,
                AssetStorageError::AssetIsDifferentType => Err(e)?,
                _ => {}
            },
        };

        return Ok(None);
    }

    /// Load an asset from a given file path
    pub fn load_from_file<T: AssetFileLoadable>(
        &mut self,
        path: &str,
    ) -> anyhow::Result<Handle<T>> {
        // Check if file is already loaded. If so, we can create a new handle to the existing data.
        if let Some(handle) = self.load_file_get_already_loaded(path)? {
            return Ok(handle);
        }

        let data = T::load_from_file(&format!("{}{}", self.load_path, &path));
        let handle = self.insert_data_path(data, path);
        Ok(handle)
    }

    /// Load an asset from a given file path and function
    pub fn load_from_data<T: Asset, F: Fn(&[u8]) -> T>(
        &mut self,
        path: &str,
        load_data: F,
    ) -> anyhow::Result<Handle<T>> {
        // Check if file is already loaded. If so, we can create a new handle to the existing data.
        if let Some(handle) = self.load_file_get_already_loaded(path)? {
            return Ok(handle);
        }

        // Load the file into bytes
        let mut file = File::open(path)?;
        let mut bytes = vec![];
        let _count = file.read_to_end(&mut bytes)?;

        // Pass the loaded bytes into provided function and get data
        let data = load_data(&bytes);
        // Insert new data to get handle
        let handle = self.insert_data_path(data, path);

        Ok(handle)
    }

    //----------------------------------------------

    pub fn is_file_loaded(&self, path: &str) -> bool {
        self.loaded_paths.contains_key(path)
    }

    pub fn get_file_loaded<T: Asset>(&self, path: &str) -> Result<Handle<T>, AssetStorageError> {
        if let Some(id) = self.loaded_paths.get(path) {
            return self.get_handle(*id);
        }
        Err(AssetStorageError::AssetNotExist)
    }

    //----------------------------------------------

    /// Get and cast an id into a handle
    pub fn get_handle<T: Asset, HI: Into<HandleInner>>(
        &self,
        id: HI,
    ) -> Result<Handle<T>, AssetStorageError> {
        let id = id.into();

        let val = self
            .loaded
            .get(&id)
            // If asset doesn't exist, return custom error
            .ok_or(AssetStorageError::AssetNotExist)?
            .clone();

        let inner: Arc<T> = val
            .into_any_arc()
            .downcast()
            .map_err(|_| AssetStorageError::AssetIsDifferentType)?;

        let id = HandleID::new(id);

        let handle = Handle::new(id, self.sender.clone(), inner);
        Ok(handle)
    }

    pub fn get_data<T: Asset>(&self, id: HandleID<T>) -> Result<&T, AssetStorageError> {
        self.get_data_raw(id)
    }

    pub fn get_data_raw<T: Asset, HI: Into<HandleInner>>(
        &self,
        id: HI,
    ) -> Result<&T, AssetStorageError> {
        match self.loaded.get(&id.into()) {
            Some(val) => match val.as_ref().as_any().downcast_ref() {
                Some(of_type) => Ok(of_type),
                None => Err(AssetStorageError::AssetIsDifferentType)?,
            },
            None => Err(AssetStorageError::AssetNotExist)?,
        }
    }

    //----------------------------------------------

    pub fn tick(&mut self) {
        self.check_asset_changes();
        self.removed_pending_assets();
    }

    pub(crate) fn check_asset_changes(&mut self) {
        self.removed_assets.clear();

        // Loop through each recieved signal and act accordingly
        loop {
            let data = match self.receiver.try_recv() {
                Ok(data) => data,
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    panic!("Error: Handle has been disconnected from Asset Storage")
                }
            };

            match data {
                ReferenceCountSignal::Increase(id) => match self.handle_count.get_mut(&id) {
                    Some(count) => *count += 1,
                    None => panic!(
                        "Error: Handle amount increased but asset with id {:?} doesn't exist.",
                        id
                    ),
                },
                ReferenceCountSignal::Decrease(id) => match self.handle_count.get_mut(&id) {
                    Some(count) => {
                        *count -= 1;
                        if *count == 0 {
                            self.removed_assets.push(id);
                        }
                    }
                    None => panic!(
                        "Error: Handle amount decreased but asset with id {:?} doesn't exist.",
                        id
                    ),
                },
            }
        }
    }

    pub(crate) fn removed_pending_assets(&mut self) {
        for to_remove in &self.removed_assets {
            self.loaded.remove(&to_remove);
            self.handle_count.remove(&to_remove);

            if let Some(val) = &self.asset_paths.remove(&to_remove) {
                self.loaded_paths.remove(val);
            }
        }
    }

    //----------------------------------------------
}

//===============================================================
