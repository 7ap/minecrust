pub mod block;
pub mod chunk;

use bevy::utils::HashMap;

use crate::prelude::*;

pub use block::*;
pub use chunk::*;

pub struct World {
    pub chunks: HashMap<IVec3, Chunk>,
}
