//===============================================================

use std::{marker::PhantomData, sync::Arc};

use super::{
    asset_storage::{ReferenceCountSignal, SenderType},
    Asset,
};

//===============================================================

pub struct HandleID<T: Asset> {
    pub(crate) id: u32,
    data: PhantomData<T>,
}

impl<T> HandleID<T>
where
    T: Asset,
{
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            data: PhantomData,
        }
    }
}

//----------------------------------------------

impl<T> std::hash::Hash for HandleID<T>
where
    T: Asset,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

//----------------------------------------------

impl<T> Clone for HandleID<T>
where
    T: Asset,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for HandleID<T> where T: Asset {}

//----------------------------------------------

impl<T> PartialEq for HandleID<T>
where
    T: Asset,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for HandleID<T> where T: Asset {}

//----------------------------------------------

impl<T> std::fmt::Display for HandleID<T>
where
    T: Asset,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.id)
    }
}

//===============================================================

pub enum HandleType<T: Asset> {
    Weak,
    Strong(SenderType<ReferenceCountSignal<T>>),
}

//===============================================================

pub struct Handle<T: 'static + Asset> {
    handle_id: HandleID<T>,
    handle_type: HandleType<T>,
    asset: Arc<T>,
}
impl<T> Handle<T>
where
    T: Asset,
{
    //----------------------------------------------

    #[inline]
    pub fn id(&self) -> HandleID<T> {
        self.handle_id
        // todo!()
    }
    #[inline]
    pub fn type_name(&self) -> &'static str {
        T::asset_name()
    }
    #[inline]
    pub fn get(&self) -> &T {
        &self.asset
    }

    //----------------------------------------------

    /// Create a new strong handle. If you want to clone a handle or clone a weak handle use clone or clone_weak
    pub fn strong(
        handle_id: HandleID<T>,
        sender: SenderType<ReferenceCountSignal<T>>,
        asset: Arc<T>,
    ) -> Self {
        // When creating a new handle, we need to tell the asset storage
        // that another reference is now in use
        sender
            .send(ReferenceCountSignal::Increase(handle_id))
            .unwrap();
        let handle_type = HandleType::Strong(sender);

        Self {
            handle_id,
            handle_type,
            asset,
        }
    }

    pub fn weak(handle_id: HandleID<T>, asset: Arc<T>) -> Self {
        Self {
            handle_id,
            handle_type: HandleType::Weak,
            asset,
        }
    }

    //----------------------------------------------

    /// Create a new weak handle from an existing one
    #[inline]
    pub fn clone_weak(&self) -> Self {
        Self::weak(self.handle_id, self.asset.clone())
    }

    /// Convert a strong handle into a weak one. This action cannot currently be reversed.
    pub fn to_weak(&mut self) {
        match &mut self.handle_type {
            HandleType::Weak => {}
            HandleType::Strong(sender) => {
                sender
                    .send(ReferenceCountSignal::Decrease(self.handle_id))
                    .unwrap();
                self.handle_type = HandleType::Weak;
            }
        }
    }

    //----------------------------------------------
}

impl<T: Asset> Clone for Handle<T> {
    fn clone(&self) -> Self {
        match &self.handle_type {
            HandleType::Weak => self.clone_weak(),
            HandleType::Strong(sender) => {
                Self::strong(self.handle_id, sender.clone(), self.asset.clone())
            }
        }
    }
}
impl<T: Asset> Drop for Handle<T> {
    fn drop(&mut self) {
        if let HandleType::Strong(sender) = &mut self.handle_type {
            // Removing a handle, decrease the total existing handles
            sender
                .send(ReferenceCountSignal::Decrease(self.handle_id))
                .unwrap();
        }
    }
}

impl<T> PartialEq for Handle<T>
where
    T: Asset,
{
    fn eq(&self, other: &Self) -> bool {
        self.handle_id.id == other.handle_id.id
    }
}

//===============================================================

// pub struct HandleMut<T: 'static + Asset> {
//     handle_id: HandleID,
//     handle_type: HandleType,
//     asset: Arc<RwLock<T>>,
//     // asset: Arc<Mutex<T>>,
// }
// impl<T> HandleMut<T>
// where
//     T: Asset,
// {
//     //----------------------------------------------

//     #[inline]
//     pub fn id(&self) -> HandleID {
//         self.handle_id
//     }
//     #[inline]
//     pub fn get(&self) -> RwLockReadGuard<T> {
//         self.asset.read()
//     }

//     //----------------------------------------------

//     /// Create a new strong handle. If you want to clone a handle, use clone or clone_weak
//     pub fn strong(
//         handle_id: HandleID,
//         sender: SenderType<ReferenceCountSignal>,
//         asset: Arc<RwLock<T>>,
//     ) -> Self {
//         // When creating a new handle, we need to tell the asset storage
//         // that another reference is now in use
//         sender
//             .send(ReferenceCountSignal::Increase(handle_id))
//             .unwrap();
//         let handle_type = HandleType::Strong(sender);

//         Self {
//             handle_id,
//             handle_type,
//             asset,
//         }
//     }

//     pub fn weak(handle_id: HandleID, asset: Arc<RwLock<T>>) -> Self {
//         Self {
//             handle_id,
//             handle_type: HandleType::Weak,
//             asset,
//         }
//     }

//     //----------------------------------------------

//     /// Create a new weak handle from an existing one
//     #[inline]
//     pub fn clone_weak(&self) -> Self {
//         Self::weak(self.handle_id, self.asset.clone())
//     }

//     /// Convert a strong handle into a weak one. This action cannot currently be reversed.
//     pub fn to_weak(&mut self) {
//         match &mut self.handle_type {
//             HandleType::Weak => {}
//             HandleType::Strong(sender) => {
//                 sender
//                     .send(ReferenceCountSignal::Decrease(self.handle_id))
//                     .unwrap();
//                 self.handle_type = HandleType::Weak;
//             }
//         }
//     }

//     //----------------------------------------------
// }

// impl<T: Asset> Clone for HandleMut<T> {
//     fn clone(&self) -> Self {
//         match &self.handle_type {
//             HandleType::Weak => self.clone_weak(),
//             HandleType::Strong(sender) => {
//                 Self::strong(self.handle_id, sender.clone(), self.asset.clone())
//             }
//         }
//     }
// }
// impl<T: Asset> Drop for HandleMut<T> {
//     fn drop(&mut self) {
//         if let HandleType::Strong(sender) = &mut self.handle_type {
//             // Removing a handle, decrease the total existing handles
//             sender
//                 .send(ReferenceCountSignal::Decrease(self.handle_id))
//                 .unwrap();
//         }
//     }
// }

//===============================================================
