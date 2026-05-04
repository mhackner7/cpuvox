use crate::ray::Ray;

struct Tree64 {
    pub nodes: Vec<Node>, // root node at nodes[0]
    pub bricks: Vec<Brick>,
    pub indices: Vec<u32>,
}

#[derive(Debug, Copy, Clone)]
struct Node {
    pub children: u64,
    pub pointer: i32, // pos: child pointer, neg: is a leaf node
    pub depth: u32,
}

impl Node {
    fn traverse(self, ray: Ray) -> Material {
        //todo
        0
    }
}

const BRICK_WIDTH: usize = 8;
const BRICK_SIZE: usize = BRICK_WIDTH * BRICK_WIDTH * BRICK_WIDTH;
type Material = u8;

struct Brick {
    v: [Material; BRICK_SIZE],
}

impl Brick {
    #[inline(always)]
    const fn index(i: (u8, u8, u8)) -> usize {
        i.0 as usize + (i.1 as usize * BRICK_WIDTH) + (i.2 as usize * BRICK_WIDTH * BRICK_WIDTH)
    }

    #[inline(always)]
    pub fn at(&self, i: (u8, u8, u8)) -> Material {
        let index = Self::index(i);
        self.v[index]
    }

    #[inline(always)]
    pub fn change_at(&mut self, i: (u8, u8, u8), mat: Material) {
        let index = Self::index(i);
        self.v[index] = mat;
    }

    #[inline(always)]
    pub fn traverse(&mut self, ray: Ray) -> Material {
        // todo
        0
    }
}
