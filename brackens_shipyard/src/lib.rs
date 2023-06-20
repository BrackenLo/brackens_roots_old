//===============================================================

use shipyard::Label;
use std::hash::Hash;

//===============================================================

pub use shipyard;

#[cfg(feature = "assets")]
pub mod assets;
#[cfg(feature = "tools")]
pub mod tools;

//===============================================================

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum WorkloadPhase {
    Start,
    PreUpdate,
    Update,
    PostUpdate,
    PreRender,
    Render,
    PostRender,
    End,
}
impl Label for WorkloadPhase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn dyn_eq(&self, other: &dyn Label) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn dyn_hash(&self, mut state: &mut dyn std::hash::Hasher) {
        Self::hash(self, &mut state);
    }

    fn dyn_clone(&self) -> Box<dyn Label> {
        Box::new(self.clone())
    }

    fn dyn_debug(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

//===============================================================
