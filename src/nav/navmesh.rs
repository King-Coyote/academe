use super::graph::*;
use crate::{
    utils::geometry::*,
    utils::math,
    utils::data_struct,
};
use spade::{
    delaunay::*,
    kernels::*,
};
use bevy::prelude::*;
use std::collections::{HashMap};
use pathfinding::directed::astar::astar;
use serde::{Deserialize, Serialize};


type CoordNum = f32;
type Point = [CoordNum; 2];
type EH<'a> = EdgeHandle<'a, Point, CdtEdge>;
type Triangulation = ConstrainedDelaunayTriangulation<Point, FloatKernel, DelaunayWalkLocate>;

// pub struct NavMeshBuilder<'a> {
//     boundary: Option<&'a [Vec2]>,
//     holes: Vec<&'a [Vec2]>,
// }

// impl<'a> NavMeshBuilder<'a> {
//     pub fn new() -> Self {
//         NavMeshBuilder {
//             boundary: None,
//             holes: vec![],
//         }
//     }

//     pub fn with_boundary(&mut self, boundary: &'a [Vec2]) -> &mut Self {
//         self.boundary = Some(boundary);
//         self
//     }

//     pub fn with_hole(&mut self, hole: &'a [Vec2]) -> &mut Self {
//         self.holes.push(hole);
//         self
//     }

//     pub fn build(self) -> Option<NavMesh> {
//         let first_point = self.boundary?.get(0)?;
//         let boundary = self.boundary?;
//         let mut triangulation = ConstrainedDelaunayTriangulation::with_walk_locate();
//         add_triangulation_boundary(&mut triangulation, &boundary);
//         for hole in self.holes.iter() {
//             add_triangulation_boundary(&mut triangulation, hole);
//         }
//         let graph = build_medial_graph(&triangulation, &boundary, &self.holes);
//         let mut navmesh = NavMesh {
//             boundary: boundary.to_vec(),
//             holes: self.holes.iter().map(|v| v.to_vec()).collect(),
//             medial_graph: graph,
//             triangulation: triangulation,
//         };
//         Some(navmesh)
//     }
// }

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct NavMesh {
    boundary: Vec<Vec2>,
    holes: Vec<Vec<Vec2>>,
    #[reflect(ignore)]
    medial_graph: Graph<Vec2>,
    #[reflect(ignore)]
    triangulation: Triangulation,
}

impl NavMesh {
    /// consumes boundary and hole vectors
    pub fn new(boundary: Vec<Vec2>, holes: Vec<Vec<Vec2>>) -> Option<Self> {
        let mut triangulation = ConstrainedDelaunayTriangulation::with_walk_locate();
        add_triangulation_boundary(&mut triangulation, &boundary);
        for hole in holes.iter() {
            add_triangulation_boundary(&mut triangulation, hole);
        }
        let medial_graph = build_medial_graph(&triangulation, &boundary, &holes);
        let mut navmesh = NavMesh {
            boundary,
            holes,
            medial_graph,
            triangulation,
        };
        Some(navmesh)
    }

    pub fn build(&mut self) {
        let mut triangulation = ConstrainedDelaunayTriangulation::with_walk_locate();
        add_triangulation_boundary(&mut triangulation, &self.boundary);
        for hole in self.holes.iter() {
            add_triangulation_boundary(&mut triangulation, hole);
        }
        self.medial_graph = build_medial_graph(&triangulation, &self.boundary, &self.holes);
        self.triangulation = triangulation;
    }

    pub fn edges(&self) -> EdgesIterator {
        EdgesIterator::new(self.triangulation.edges())
    }

    pub fn graph_nodes_iter(&self) -> impl Iterator<Item = &Vec2> {
        self.medial_graph.nodes_iter()
    }

    pub fn graph_edges(&self) -> Vec<(&Vec2, &Vec2)> {
        self.medial_graph.edges()
    }

    // returns an iterator over all triangles that are within the boundary of the navmesh
    pub fn interior_triangles(&self) -> TrianglesIterator {
        TrianglesIterator::new(
            &self.boundary,
            &self.holes,
            self.triangulation.triangles()
        )
    }

    pub fn points_have_los(&self, a: &Vec2, b: &Vec2) -> bool {
        !any_intersections_between(a, b, &self.boundary)
        && !self.holes.iter().any(|hole| any_intersections_between(a, b, hole))
    }

    pub fn find_path(&self, a: Vec2, b: Vec2) -> Option<Vec<Vec2>> {
        // for some point, get the closest node's index with LOS
        let mut path: Vec<Vec2> = vec![b];
        if self.points_have_los(&a, &b) {
            return Some(path);
        }
        let mut min_dist = f32::INFINITY;
        let start = self.medial_graph.nodes
            .iter()
            .enumerate()
            .fold(0, |closest, (index, node)| {
                let dist = (*node - a).length();
                if dist < min_dist && self.points_have_los(&a, node) {
                    min_dist = dist;
                    return index
                }
                closest
            });
        // the closer the euclidean distance, the better
        let heuristic = |n: &usize| {
            let pt = self.medial_graph.get(*n).unwrap();
            (b - *pt).length().round() as i32
        };
        let success = |n: &usize| {
            let pt = self.medial_graph.get(*n).unwrap();
            self.points_have_los(pt, &b)
        };
        astar(
            &start,
            |n| self.medial_graph.succ(*n).unwrap().iter().map(|s| (*s, 1)),
            heuristic,
            success
        ).map(|path_indices| {
            path.extend(path_indices.0
                .iter()
                .rev() // want this to have last point as index 0 (since we'll be popping from it)
                .map(|i| *self.medial_graph.get(*i).unwrap())
            );
            // let durr = path.iter().enumerate().rev().find(|e| e.1
            let mut succ_had_los = false;
            loop {
                if path.len() < 2 {
                    break;
                }
                let this_has_los = match succ_had_los {
                    true => true,
                    false => self.points_have_los(&a, path.last().unwrap())
                };
                let successor = path.get(path.len() - 2).unwrap();
                succ_had_los = self.points_have_los(&a, &successor);
                if succ_had_los && this_has_los {
                    path.pop();
                } else {
                    break;
                }
            }
            path
        })
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

fn get_medial_point(e: &EH<'_>) -> Vec2 {
    Vec2::new(
        (e.from()[0] + e.to()[0]) / 2.0,
        (e.from()[1] + e.to()[1]) / 2.0
    )
}

fn add_triangulation_boundary(triangulation: &mut Triangulation, vertices: &[Vec2]) {
    let mut last_point: Option<Point> = None;
    for vert in vertices {
        let point = raw_from_vec2(*vert);
        match last_point {
            Some(last) => {
                triangulation.add_constraint_edge(last, point);
            },
            None => {
                triangulation.insert(point);
            },
        }
        last_point = Some(point);
    }
}

fn build_medial_graph(triangulation: &Triangulation, boundary: &[Vec2], holes: &[Vec<Vec2>]) -> Graph<Vec2> {
    let mut visited: HashMap<(u32, u32), usize> = HashMap::new();
    let mut graph: Graph<Vec2> = Graph::default();

    for face in triangulation.triangles() {
        let medial_indices = face.adjacent_edges()
            .filter(|e| !triangulation.is_constraint_edge(e.fix()))
            .map(|e| {
                let point = get_medial_point(&e);
                Vec2::new(
                    math::round_to(point.x, 3),
                    math::round_to(point.y, 3)
                )
            })
            .filter(|p| {
                point_inside_polygon(&p, boundary)
                && holes.iter().all(|hole| !point_inside_polygon(&p, hole))
            })
            .map(|p| {
                let encoded = data_struct::decode_vec2(&p);
                match visited.get(&encoded) {
                    Some(i) => *i,
                    None => {
                        let new_index = graph.add_node(p);
                        visited.insert(encoded, new_index);
                        new_index
                    }
                }
            })
            .collect::<Vec<usize>>();
        for index in medial_indices.iter() {
            for other_index in medial_indices.iter().filter(|i| *i != index) {
                graph.add_edge(*index, *other_index);
            }
        }
    }

    info!("Graph generated with {} nodes, {} edges", graph.num_nodes(), graph.num_edges());
    graph
}