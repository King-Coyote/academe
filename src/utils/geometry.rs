use bevy::prelude::*;
use intersect2d::*;

// check if a point is inside the polygon formed by points
pub fn point_inside_polygon(point: &Vec2, polygon: &[Vec2]) -> bool {
    let point_outside = get_outside_point(polygon);
    // let out = Ray::from_points(point, &point_outside);
    num_intersections_between(&point_outside, point, polygon) % 2 != 0
}

pub fn num_intersections_between(a: &Vec2, b: &Vec2, boundary: &[Vec2]) -> u32 {
    let mut intersections = 0;
    let line = [*a, *b];
    for e in boundary.iter().enumerate() {
        let mut next_index = e.0 + 1;
        if e.0 == boundary.len() - 1 {
            next_index = 0;
        }
        let next = boundary.get(next_index).unwrap();
        if let Some(pt) = lines_intersect(&line, &[*e.1, *next]) {
            intersections += 1;
        }
    }
    intersections
}

pub fn any_intersections_between(a: &Vec2, b: &Vec2, boundary: &[Vec2]) -> bool {
    let line = [*a, *b];
    for e in boundary.iter().enumerate() {
        let mut next_index = e.0 + 1;
        if e.0 == boundary.len() - 1 {
            next_index = 0;
        }
        let next = boundary.get(next_index).unwrap();
        if let Some(pt) = lines_intersect(&line, &[*e.1, *next]) {
            return true;
        }
    }
    false
}

pub fn point_inside_sprite(point: &Vec2, sprite: &Sprite, transform: &Transform) -> bool {
    let halfx = sprite.size.x * 0.5;
    let halfy = sprite.size.y * 0.5;

    point.x > transform.translation.x - halfx
        && point.x <= transform.translation.x + halfx
        && point.y > transform.translation.y - halfy
        && point.y <= transform.translation.y + halfy
}

pub fn lines_intersect(a: &[Vec2; 2], b: &[Vec2; 2]) -> Option<Vec2> {
    let a: geo::Line::<f32> = destructure_vec2_line(a).into();
    let b: geo::Line::<f32> = destructure_vec2_line(b).into();
    if let Some(sect) = intersect(&a, &b) {
        let pt = sect.single();
        return Some(Vec2::new(
            pt.x,
            pt.y
        ));
    }
    None
}

fn destructure_vec2(v: &Vec2) -> (f32, f32) {
    (v.x, v.y)
}

fn destructure_vec2_line(v: &[Vec2; 2]) -> [(f32, f32); 2] {
    [
        (v[0].x, v[0].y),
        (v[1].x, v[1].y)
    ]
}

fn same_signs(a: f32, b: f32) -> bool {
    (a > 0. && b > 0.) ||
    (a < 0. && b < 0.)
}

fn is_zero(val: f32) -> bool {
    f32::abs(val) <= f32::EPSILON
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
