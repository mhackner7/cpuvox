use crate::ivec3::IVec3;
use crate::multigrid::MultiGrid;
use gxhash::{HashMap, HashMapExt};

type ChunkType = MultiGrid;

pub struct World {
    map: HashMap<IVec3, ChunkType>,
}

impl World {
    pub fn new(render_distance: usize) -> World {
        let initial_capacity = render_distance.pow(3);

        let map: HashMap<IVec3, ChunkType> = HashMap::with_capacity(initial_capacity);

        World { map }
    }

    pub fn chunk_at(&self, vec: IVec3) -> Option<&ChunkType> {
        self.map.get(&vec)
    }

    pub fn chunk_at_mut(&mut self, vec: IVec3) -> Option<&mut ChunkType> {
        self.map.get_mut(&vec)
    }
}
