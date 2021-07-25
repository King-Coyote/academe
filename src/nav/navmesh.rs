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

pub struct NavMeshBuilder<'a> {
    boundary: Option<&'a [Vec2]>,
    holes: Vec<&'a [Vec2]>,
}

impl<'a> NavMeshBuilder<'a> {
    pub fn new() -> Self {
        NavMeshBuilder {
            boundary: None,
            holes: vec![],
        }
    }

    pub fn with_boundary(&mut self, boundary: &'a [Vec2]) -> &mut Self {
        self.boundary = Some(boundary);
        self
    }

    pub fn with_hole(&mut self, hole: &'a [Vec2]) -> &mut Self {
        self.holes.push(hole);
        self
    }

    pub fn build(self) -> Option<NavMesh> {
        let first_point = self.boundary?.get(0)?;
        let boundary = self.boundary?;
        let mut navmesh = NavMesh {
            boundary: boundary.to_vec(),
            holes: self.holes.iter().map(|v| v.to_vec()).collect(),
            triangulation: ConstrainedDelaunayTriangulation::with_walk_locate(),
            last_point: raw_from_vec2(*first_point),
            mid_points: vec![],
            outside_point: raw_from_vec2(get_outside_point(boundary)),
        };
        navmesh.add_boundary(&boundary);
        for hole in self.holes {
            navmesh.add_boundary(hole);
        }
        Some(navmesh)
    }
}

// needs to give navagents a path they can walk on
// so it only needs to expose the medial axis?????
pub struct NavMesh {
    boundary: Vec<Vec2>,
    holes: Vec<Vec<Vec2>>,
    mid_points: Vec<Vec2>,
    last_point: Point,
    triangulation: ConstrainedDelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>,
    outside_point: Point,
}

impl NavMesh {
    // adds some boundary, hole or otherwise. Only the triangulation cares about this
    fn add_boundary(&mut self, vertices: &[Vec2]) {
        let mut last_point: Option<Point> = None;
        for vert in vertices {
            let point = raw_from_vec2(*vert);
            match last_point {
                Some(last) => {
                    self.triangulation.add_constraint_edge(last, point);
                },
                None => {
                    self.triangulation.insert(point);
                },
            }
            last_point = Some(point);
        }
    }

    pub fn edges(&self) -> EdgesIterator {
        EdgesIterator::new(self.triangulation.edges())
    }

    // returns an iterator over all triangles that are within the boundary of the navmesh
    pub fn interior_triangles(&self) -> TrianglesIterator {
        TrianglesIterator::new(
            &self.boundary,
            &self.holes,
            self.triangulation.triangles()
        )
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
    boundary: &'a [Vec2],
    holes: &'a [Vec<Vec2>],
    iter: Box<dyn Iterator<Item=FaceHandle<'a, Point, spade::delaunay::CdtEdge>> + 'a>,
}

impl<'a> TrianglesIterator<'a> {
    pub fn new(
        boundary: &'a [Vec2],
        holes: &'a [Vec<Vec2>],
        iter: impl Iterator<Item=FaceHandle<'a, Point, spade::delaunay::CdtEdge>> + 'a
    ) -> Self {
        TrianglesIterator {
            boundary,
            holes,
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
            let centroid = get_centroid(&vertices);
            if !point_inside_polygon(&centroid, self.boundary) 
            || self.holes.iter().any(|hole| point_inside_polygon(&centroid, &hole))
            {
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