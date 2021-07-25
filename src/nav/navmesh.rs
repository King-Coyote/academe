use crate::{
    utils::geometry::*,
};
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
    outside_point: Point,
}

impl NavMesh {
    pub fn with_boundary(boundary: &[Vec2]) -> Option<Self> {
        let mut navmesh = NavMesh {
            triangulation: ConstrainedDelaunayTriangulation::with_walk_locate(),
            last_point: raw_from_vec2(*boundary.get(0)?),
            verts: boundary.to_vec(),
            mid_points: vec![],
            outside_point: raw_from_vec2(get_outside_point(boundary)),
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

    // returns an iterator over all triangles that are within the boundary of the navmesh
    pub fn interior_triangles(&self) -> TrianglesIterator {
        TrianglesIterator::new(&self.verts, self.triangulation.triangles())
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

impl<'a> Iterator for EdgesIterator<'a> {
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

pub struct TrianglesIterator<'a> {
    vertices: &'a [Vec2],
    iter: Box<dyn Iterator<Item=FaceHandle<'a, Point, spade::delaunay::CdtEdge>> + 'a>,
}

impl<'a> TrianglesIterator<'a> {
    pub fn new(
        vertices: &'a [Vec2],
        iter: impl Iterator<Item=FaceHandle<'a, Point, spade::delaunay::CdtEdge>> + 'a
    ) -> Self {
        TrianglesIterator {
            vertices,
            iter: Box::new(iter)
        }
    }
}

impl<'a> Iterator for TrianglesIterator<'a> {
    type Item = [Vec2; 3];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let handle = self.iter.next()?;
            let vertices = handle.as_triangle();
            if !point_inside_polygon(&get_centroid(&vertices), self.vertices) {
                continue;
            }
            return Some([
                vec2_from_raw(&vertices[0]),
                vec2_from_raw(&vertices[1]),
                vec2_from_raw(&vertices[2])
            ]);
        }
    }
}

// UTIL
fn raw_from_vec2(vec: Vec2) -> Point {
    [vec.x, vec.y]
}

fn vec2_from_raw(raw: &Point) -> Vec2 {
    Vec2::new(raw[0], raw[1])
}

fn get_centroid<T>(t: &[VertexHandle<Point, T>; 3]) -> Vec2 {
    Vec2::new(
        (t[0][0] + t[1][0] + t[2][0]) / 3.,
        (t[0][1] + t[1][1] + t[2][1]) / 3.
    )
}