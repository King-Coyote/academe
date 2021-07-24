use spade::PointN;
use spade::{
    delaunay::*,
};
use spade::kernels::*;
use bevy::prelude::*;

type CoordNum = f32;
type Point = [CoordNum; 2];

// needs to give navagents a path they can walk on
// so it only needs to expose the medial axis?????
pub struct NavMesh {
    pub verts: Vec<Vec2>,
    pub mid_points: Vec<Vec2>,
    last_point: Point,
    triangulation: ConstrainedDelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>,
}

impl NavMesh {
    pub fn with_boundary(boundary: &[Vec2]) -> Option<Self> {
        let mut navmesh = NavMesh {
            triangulation: ConstrainedDelaunayTriangulation::with_walk_locate(),
            last_point: raw_from_vec2(*boundary.get(0)?),
            verts: boundary.to_vec(),
            mid_points: vec![]
        };
        for vec in boundary {
            navmesh.add_point(raw_from_vec2(*vec));
        }
        Some(navmesh)
    }

    fn add_point(&mut self, point: Point) {
        if self.triangulation.num_vertices() == 0 {
            self.triangulation.insert(point);
        } else {
            self.triangulation.add_constraint_edge(self.last_point, point);
        }
        self.last_point = point;
    }

    pub fn edges(&self) -> EdgesIterator {
        EdgesIterator::new(self.triangulation.edges())
    }
}

// look at this fuckin stupid shit that I have to do to make this iterable
// I am implementing an adaptor for the EdgesIterator deep inside Spade.
pub struct EdgesIterator<'a> {
    iter: Box<dyn Iterator<Item=EdgeHandle<'a, Point, spade::delaunay::CdtEdge>> + 'a>,
}

impl<'a> EdgesIterator<'a> {
    pub fn new(iter: impl Iterator<Item=EdgeHandle<'a, Point, spade::delaunay::CdtEdge>> + 'a) -> Self {
        EdgesIterator {iter: Box::new(iter)}
    }
}

impl<'a> Iterator for EdgesIterator<'a>
{
    type Item = [Vec2; 2];
    fn next(&mut self) -> Option<Self::Item> {
        let handle = self.iter.next()?;
        let from = handle.from();
        let to = handle.to();
        Some([
            Vec2::new(from[0], from[1]),
            Vec2::new(to[0], to[1])
        ])
    }
}

// UTIL

fn raw_from_vec2(vec: Vec2) -> Point {
    [vec.x, vec.y]
}