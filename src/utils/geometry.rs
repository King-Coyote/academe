use bevy::prelude::*;

// a bunch of utility functions; can be split into submodules later

pub struct Ray {
    o: Vec2,
    d: Vec2,
    l: f32,
}

pub struct Line {
    start: Vec2,
    end: Vec2,
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
    // let out = Ray::from_points(point, &point_outside);
    num_intersections_between(&point_outside, point, polygon) % 2 != 0
}

pub fn num_intersections_between(a: &Vec2, b: &Vec2, boundary: &[Vec2]) -> u32 {
    let mut intersections = 0;
    let line = Ray::from_points(b, a);
    for e in boundary.iter().enumerate() {
        let mut next_index = e.0 + 1;
        if e.0 == boundary.len() - 1 {
            next_index = 0;
        }
        let next = boundary.get(next_index).unwrap();
        if do_lines_intersect(&line, &Ray::from_points(e.1, next)) {
            intersections += 1;
        }
    }
    info!("num intersects is {}", intersections);
    intersections
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
    if f32::abs(det) <= f32::EPSILON {
        return false;
    }
    let u = (dy * b.d.x - dx * b.d.y) / det;
    let v = (dy * a.d.x - dx * a.d.y) / det;
    u > 0. && v > 0. && v <= b.l
}

// doesn't seem to work rn
pub fn do_lines_intersect_experiment(a: &Line, b: &Line) -> bool {
    let ( x1, y1 ) = destructure_vec2(&a.start);
    let ( x2, y2 ) = destructure_vec2(&a.end);
    let ( x3, y3 ) = destructure_vec2(&b.start);
    let ( x4, y4 ) = destructure_vec2(&b.end);

    // First line coefficients where "a1 x  +  b1 y  +  c1  =  0"
    let a1 = y2 - y1;
    let b1 = x1 - x2;
    let c1 = x2 * y1 - x1 * y2;

    // Second line coefficients
    let a2 = y4 - y3;
    let b2 = x3 - x4;
    let c2 = x4 * y3 - x3 * y4;

    let denom = a1 * b2 - a2 * b1;

    // Lines are colinear
    if is_zero(denom) {
        return false;
    }

    // Compute sign values
    let r3 = a1 * x3 + b1 * y3 + c1;
    let r4 = a1 * x4 + b1 * y4 + c1;

    // Sign values for second line
    let r1 = a2 * x1 + b2 * y1 + c2;
    let r2 = a2 * x2 + b2 * y2 + c2;

    // Flag denoting whether intersection point is on passed line segments. If this is false,
    // the intersection occurs somewhere along the two mathematical, infinite lines instead.
    //
    // Check signs of r3 and r4.  If both point 3 and point 4 lie on same side of line 1, the
    // line segments do not intersect.
    //
    // Check signs of r1 and r2.  If both point 1 and point 2 lie on same side of second line
    // segment, the line segments do not intersect.
    let is_on_segments = (!is_zero(r3) && !is_zero(r4) && same_signs(r3, r4))
        || (!is_zero(r1) && !is_zero(r2) && same_signs(r1, r2));

    // If we got here, line segments intersect. Compute intersection point using method similar
    // to that described here: http://paulbourke.net/geometry/pointlineplane/#i2l

    // The denom/2 is to get rounding instead of truncating. It is added or subtracted to the
    // numerator, depending upon the sign of the numerator.
    // let offset = if denom < 0 { -denom / 2 } else { denom / 2 };

    // let num = b1 * c2 - b2 * c1;
    // let x = if num < 0 { num - offset } else { num + offset } / denom;

    // let num = a2 * c1 - a1 * c2;
    // let y = if num < 0 { num - offset } else { num + offset } / denom;
    is_on_segments
}

fn destructure_vec2(v: &Vec2) -> (f32, f32) {
    (v.x, v.y)
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
