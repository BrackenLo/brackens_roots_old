//===============================================================

use std::{collections::HashMap, fs::File, io::Read, sync::Arc};

use crate::{
    handle_x::{Handle, HandleID, HandleInner},
    Asset, AssetFileLoadable, AssetFileLoadableData,
};

//===============================================================

#[derive(Debug)]
pub enum AssetStorageError {
    AssetNotExist,
    InvalidType,
    AssetExistsDifferentType,
}
impl std::fmt::Display for AssetStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AssetStorageError::AssetNotExist => todo!(),
            AssetStorageError::InvalidType => {
                write!(
                    f,
                    "AssetStorageError: The given type doesn't match the stored type."
                )
            }
            AssetStorageError::AssetExistsDifferentType => todo!(),
        }
    }
}
impl std::error::Error for AssetStorageError {}

//===============================================================

pub enum ReferenceCountSignalX {
    Increase(HandleInner),
    Decrease(HandleInner),
}

//===============================================================

pub struct AssetStorageX {
    sender: crossbeam::channel::Sender<ReferenceCountSignalX>,
    receiver: crossbeam::channel::Receiver<ReferenceCountSignalX>,

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

    just_added: Vec<HandleInner>,
    removed_assets: Vec<HandleInner>,
}

impl AssetStorageX {
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
        self.just_added.push(id);

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
                AssetStorageError::InvalidType => Err(AssetStorageError::AssetExistsDifferentType)?,
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
    pub fn load_from_data<T: AssetFileLoadableData, F: Fn(&[u8]) -> T>(
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
    fn get_handle<T: Asset>(&self, id: HandleInner) -> Result<Handle<T>, AssetStorageError> {
        let val = self
            .loaded
            .get(&id)
            // If asset doesn't exist, return custom error
            .ok_or(AssetStorageError::AssetNotExist)?
            .clone();

        let inner: Arc<T> = val
            .into_any_arc()
            .downcast()
            .map_err(|_| AssetStorageError::InvalidType)?;

        let id = HandleID::new(id);

        let handle = Handle::new(id, self.sender.clone(), inner);
        Ok(handle)
    }

    //----------------------------------------------
}

//===============================================================
