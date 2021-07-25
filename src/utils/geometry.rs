use bevy::prelude::*;

// a bunch of utility functions; can be split into submodules later

pub struct Ray {
    o: Vec2,
    d: Vec2,
    l: f32,
}

impl Ray {
    pub fn from_points(a: &Vec2, b: &Vec2) -> Self {
        let diff = Vec2::new(b.x - a.x, b.y - a.y);
        let length = diff.length();
        Ray {
            o: a.clone(),
            d: diff.normalize(),
            l: length,
        }
    }
}

// check if a point is inside the polygon formed by points
pub fn point_inside_polygon(point: &Vec2, polygon: &[Vec2]) -> bool {
    let point_outside = get_outside_point(polygon);
    let out = Ray::from_points(point, &point_outside);
    let mut intersections = 0;
    for e in polygon.iter().enumerate() {
        let mut next_index = e.0 + 1;
        if e.0 == polygon.len() - 1 {
            next_index = 0;
        }
        let next = polygon.get(next_index).unwrap();
        if do_lines_intersect(&out, &Ray::from_points(e.1, next)) {
            intersections += 1;
        }
    }
    intersections % 2 != 0
}

pub fn point_inside_sprite(point: &Vec2, sprite: &Sprite, transform: &Transform) -> bool {
    let halfx = sprite.size.x * 0.5;
    let halfy = sprite.size.y * 0.5;

    point.x > transform.translation.x - halfx
        && point.x <= transform.translation.x + halfx
        && point.y > transform.translation.y - halfy
        && point.y <= transform.translation.y + halfy
}

pub fn do_lines_intersect(a: &Ray, b: &Ray) -> bool {
    let dx = b.o.x - a.o.x;
    let dy = b.o.y - a.o.y;
    let det = b.d.x * a.d.y - b.d.y * a.d.x;
    let u = (dy * b.d.x - dx * b.d.y) / det;
    let v = (dy * a.d.x - dx * a.d.y) / det;
    u > 0. && v > 0. && v <= b.l
}

pub fn get_outside_point(points: &[Vec2]) -> Vec2 {
    let mut max_x = f32::NEG_INFINITY;
    let mut furthest = &Vec2::ZERO;
    for p in points.iter() {
        if p.x > max_x {
            max_x = p.x;
            furthest = p;
        }
    }
    Vec2::new(furthest.x + 10.0, furthest.y)
}

// [top, left, bottom, right]
pub fn get_bounding_box(points: &[Vec2]) -> [f32; 4] {
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;

    for p in points.iter() {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.y < min_y {
            min_y = p.y;
        }
        if p.x > max_x {
            max_x = p.x;
        }
        if p.y > max_y {
            max_y = p.y;
        }
    }
    [max_y, min_x, min_y, max_x]
}

pub fn max_polygon_width(points: &[Vec2]) -> f32 {
    let bb = get_bounding_box(points);
    let height = f32::abs(bb[0] - bb[2]);
    let width = f32::abs(bb[1] - bb[3]);
    if height > width {
        return height;
    }
    width
}
