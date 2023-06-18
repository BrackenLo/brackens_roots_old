//===============================================================

use std::{hash::Hash, marker::PhantomData, sync::Arc};

use crate::{asset_storage_generic::ReferenceCountSignalX, Asset, SenderType};

//===============================================================

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct HandleInner(u32);
impl HandleInner {
    pub(crate) fn next(&mut self) {
        self.0 += 1;
    }
}

impl<T: Asset> From<HandleID<T>> for HandleInner {
    fn from(value: HandleID<T>) -> Self {
        value.id
    }
}

//===============================================================

pub struct HandleID<T: Asset> {
    id: HandleInner,
    data: PhantomData<T>,
}
impl<T: Asset> HandleID<T> {
    pub(crate) fn new(id: HandleInner) -> Self {
        Self {
            id,
            data: PhantomData,
        }
    }
    pub fn id(&self) -> HandleInner {
        self.id
    }
}

//----------------------------------------------

impl<T: Asset> Hash for HandleID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T: Asset> Clone for HandleID<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: Asset> Copy for HandleID<T> {}

impl<T: Asset> PartialEq for HandleID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: Asset> Eq for HandleID<T> {}

//===============================================================

pub struct Handle<T: Asset> {
    handle_id: HandleID<T>,
    sender: SenderType<ReferenceCountSignalX>,
    asset: Arc<T>,
}

impl<T: Asset> Handle<T> {
    pub(crate) fn new(
        id: HandleID<T>,
        sender: SenderType<ReferenceCountSignalX>,
        asset: Arc<T>,
    ) -> Self {
        sender.send(ReferenceCountSignalX::Increase(id.id)).unwrap();

        Self {
            handle_id: id,
            sender,
            asset,
        }
    }

    pub fn id(&self) -> HandleID<T> {
        self.handle_id
    }
    pub fn inner_id(&self) -> HandleInner {
        self.handle_id.id
    }
}

impl<T: Asset> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self::new(self.handle_id, self.sender.clone(), self.asset.clone())
    }
}

impl<T: Asset> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.handle_id.id == other.handle_id.id
    }
}

//===============================================================
