/*
// use pathfinding::prelude::*;
use spade::delaunay::*;
use spade::kernels::*;

pub type CoordNum = f64;

pub struct Location {
    x: CoordNum,
    y: CoordNum,
}

struct Ray {
    o: [CoordNum; 2],
    d: [CoordNum; 2],
    l: CoordNum,
}

pub struct NavMesh {
    triangulation: ConstrainedDelaunayTriangulation<[CoordNum; 2], FloatKernel, DelaunayWalkLocate>,
    last_point: [CoordNum; 2],
    nodes: Vec<[CoordNum; 2]>,
}

impl NavMesh {
    pub fn new() -> Self {
        NavMesh {
            triangulation: ConstrainedDelaunayTriangulation::with_walk_locate(),
            last_point: [0.,0.],
            nodes: vec![],
        }
    }

    pub fn from_points(points: &[[CoordNum; 2]]) -> Self {
        let mut navmesh = NavMesh {
            triangulation: ConstrainedDelaunayTriangulation::with_walk_locate(),
            last_point: [0.,0.],
            nodes: vec![],
        };
        for point in points {
            navmesh.add_point(*point);
        }
        navmesh
    }

    pub fn clear(&mut self) {
        self.triangulation = ConstrainedDelaunayTriangulation::with_walk_locate();
        self.nodes.clear();
    }

    pub fn add_point(&mut self, point: [CoordNum; 2]) {
        if self.triangulation.num_vertices() == 0 {
            self.triangulation.insert(point);
        } else {
            self.triangulation.add_constraint_edge(self.last_point, point);
        }
        self.last_point = point;
    }

    pub fn num_vertices(&self) -> usize {
        self.triangulation.num_vertices()
    }

    pub fn is_empty(&self) -> bool {
        self.triangulation.num_triangles() > 0
    }

    pub fn edges(&self) -> Vec<[[CoordNum; 2]; 2]> {
        self.triangulation.edges()
        .map(|e| {
            let from = e.from();
            let to = e.to();
            [*from, *to]
        })
        .collect()
    }

    pub fn deleteme_centroids(&self) -> Vec<[CoordNum; 2]> {
        self.triangulation.triangles()
        .map(|t| {
            get_centroid(&t.as_triangle())
        })
        .filter(|p| {
            self.point_inside(*p)
        })
        .collect()
    }

    // construct nodes from the triangulation
    pub fn bake(&mut self) {
        for edge in self.triangulation.edges() {
            let from = edge.from();
            let to = edge.to();
            // find midpoint of the edge and add node
            println!("edge found from {:?} to {:?}", *from, *to);
        }
    }

    pub fn point_inside(&self, point: [CoordNum; 2]) -> bool {
        // take point definitely outside. If there is intersection with a hull line, it's outside
        let out_ray = make_ray_between(point, get_outside_point());
        let mut num_intersections: u32 = 0;
        for edge in self.triangulation.edges() {
            if !self.triangulation.is_constraint_edge(edge.fix()) {
                continue;
            }
            let from = edge.from();
            let to = edge.to();
            let edge_ray = make_ray_between(*from, *to);
            if rays_intersect(&out_ray, &edge_ray) {
                num_intersections = num_intersections + 1;
            }
        }
        num_intersections % 2 != 0
    }

    // fn count_constraint_intersections(&self, p: [CoordNum; 2]) -> u32 {

    // }
}

// subdivisions of 0 results in nodes at vertices, and one halfway along each edge
// 1 subdivision would give the halfway one, then two more nodes for each edge, halfway from
// the original halfway point and so on.
// fn nodes_from_triangulation(triangulation: &Triangulation, subdivisions: u8) -> Vec<Location> {
//     // for e in 0..triangulation.triangles.len() {
//     //     // add the point at vertext to the 
//     //     if triangulation.halfedges[e] != EMPTY && e < triangulation.halfedges[e] {
//     //         let p = &points[triangulation.triangles[e]];
//     //         let q = &points[triangulation.triangles[next_halfedge(e)]];
//     //         println!("edge between p: {:?} and q: {:?}", p, q);
//     //     } else {
//     //         println!("no edge found, skipping");
//     //     }
//     // }
//     vec![]
// }

fn get_outside_point() -> [CoordNum; 2] {
    [i32::MAX as CoordNum, i32::MAX as CoordNum]
}

fn get_centroid<T>(t: &[VertexHandle<[CoordNum; 2], T>; 3]) -> [CoordNum; 2] {
    [
        (t[0][0] + t[1][0] + t[2][0]) / 3.,
        (t[0][1] + t[1][1] + t[2][1]) / 3.
    ]
}

fn make_ray_between(from: [CoordNum; 2], to: [CoordNum; 2]) -> Ray {
    let dir = [to[0] - from[0], to[1] - from[1]];
    let l = (dir[0].powi(2) + dir[1].powi(2)).sqrt();
    Ray {
        d: dir,
        o: from,
        l: l
    }    
}

fn rays_intersect(a: &Ray, b: &Ray) -> bool {
    let dx = b.o[0] - a.o[0];
    let dy = b.o[1] - a.o[1];
    let det = b.d[0] * a.d[1] - b.d[1] * a.d[0];
    let u = (dy * b.d[0] - dx * b.d[1]) / det;
    let v = (dy * a.d[0] - dx * a.d[1]) / det;
    u > 0. && v > 0. && v <= b.l
}
*/