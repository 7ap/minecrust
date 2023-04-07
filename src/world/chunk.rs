use std::rc::Weak;

use crate::prelude::*;

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    pub position: IVec3,
    pub blocks: [[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}
