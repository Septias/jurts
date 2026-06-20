use std::ops::Index;

use bevy::math::Vec2;

struct Edge {
    start: usize, // indexes into vertices
    occupied: bool,
}

struct Jurt2d {
    edges: Vec<Edge>,
    vertices: Vec<Vec2>,
    center: Vec2,
    radius: f32, // added to not have to recompute
}

trait Visualizable {
    // creates a bevy instance of the jurt
    fn instatiate();
}

trait Construction: Index<usize> {
    fn lies_inside(&self, pos: Vec2) -> bool;
    fn index(&self, pos: usize);
    fn centers(&self) -> Vec<Vec2>;
}

impl Index<usize> for Jurt2d {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        todo!()
    }
}

impl Construction for Jurt2d {
    fn lies_inside(&self, pos: Vec2) -> bool {
        self.center.distance(pos) > self.radius
    }

    fn index(&self, pos: usize) -> &Vec2 {
        &self.vertices[pos]
    }

    fn centers(&self) -> Vec<Vec2> {
        todo!()
    }
}

fn create_jurt(sides: usize) -> Jurt2d {
    todo!()
}
