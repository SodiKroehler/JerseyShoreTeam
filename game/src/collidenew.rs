use bevy::math::{Vec2, Vec3};
use bevy::ecs::component::Component;
#[derive(Component)]
pub struct Shape {
    vertices: Vec<Vec3>,
    origin: Vec3,
    rotation: f32,
} //vertices that will define the polygon

pub struct CollisionInfo {
    shape_a: Shape,
    shape_b: Shape,
    distance: f32,
    vector: Vec2,
    contain_a: bool,
    contain_b: bool,
    seperation: Vec2,
}

#[derive(Component)]
struct RB {
    pos_x: i32,
}

pub fn rotate(shape: &mut Shape, angle: f32) -> &mut Shape{
    for mut vert in shape.vertices.iter_mut() {
        *vert -= shape.origin;
        let mut temp_vert: Vec2 = vert.truncate();
        temp_vert = Vec2::from_angle(angle).rotate(temp_vert);
        *vert = Vec3{
        	x: temp_vert.x,
        	y: temp_vert.y,
        	z: 0.0,
        };
        *vert += shape.origin;
    }
    return shape;
}

pub fn sat(a_pos: &Vec3, a_vertices: &Vec<Vec3>, b_pos: &Vec3, b_vertices: &Vec<Vec3>) -> Option<CollisionInfo> {
    let mut axes: Vec<Vec2> = vec![Default::default(); a_vertices.len() + b_vertices.len()]; //perpindicular axes to project onto
    let mut poly_a = Vec::<Vec2>::with_capacity(6);
    let mut poly_b = Vec::<Vec2>::with_capacity(6);
    let mut shortest = u32::MAX;


    let mut col = CollisionInfo {
        shape_a : Shape {
            vertices: a_vertices.to_vec(),
            origin: *a_pos,
            rotation: 0.0
        },
        shape_b : Shape {
            vertices: b_vertices.to_vec(),
            origin: *b_pos,
            rotation: 0.0
        },
        distance: 0.0,
        vector: Vec2{
        	x: 0.0,
        	y: 0.0,
        },
        contain_a: true,
        contain_b: true,
        seperation: Vec2{
        	x: 0.0,
        	y: 0.0,
        },
    };

    for a in a_vertices.iter() {
        poly_a.push(a.truncate());
    }
    for b in b_vertices.iter() {
        poly_b.push(b.truncate());
    }

    for i in 0..poly_a.len() {
        axes[i] = Vec2 {
            //get perpindicular to axis
            x: poly_a[i].y - poly_a[(i + 1) % poly_a.len()].y,
            y: poly_a[(i + 1) % poly_a.len()].x - poly_a[i].x,
        }
        .normalize_or_zero();
    }
    for i in 0..poly_b.len() {
        axes[i + poly_a.len()] = Vec2 {
            //get perpindicular
            x: poly_b[i].y - poly_b[(i + 1) % poly_b.len()].y,
            y: poly_b[(i + 1) % poly_b.len()].x - poly_b[i].x,
        }
        .normalize_or_zero();
    }

    let v_offset: Vec2 = Vec2 {
        x: a_pos.x - b_pos.x,
        y: a_pos.y - b_pos.y,
    };

    for i in 0..axes.len() {
        let mut poly_a_range = project_shape(&poly_a, &axes[i]);
        let mut poly_b_range = project_shape(&poly_b, &axes[i]);

        let offset = axes[i].dot(v_offset);
        poly_a_range.0 += offset;
        poly_a_range.1 += offset;

        if (poly_a_range.0 - poly_b_range.1 > 0.0) || (poly_b_range.0 - poly_a_range.1 > 0.0) {
            //gap, do not need to keep checking definitely not colliding
            return None;
        }

        let min_dist: f32 = -(poly_b_range.1 - poly_a_range.1);
        let abs_min: f32 = if min_dist < 0.0 {
            min_dist * -1.0
        } else {
            min_dist
        };

        if abs_min < shortest as f32 {
            shortest = abs_min as u32;
            col.distance = min_dist;
            col.vector = axes[i];
        }
    }

    col.seperation = Vec2{
    	x: col.vector.x * col.distance, 
    	y: col.vector.y * col.distance,
    };

    return Some(col);
}

pub fn project_shape(shape: &Vec<Vec2>, axis: &Vec2) -> (f32, f32) {
    //do dot product for first vector onto axis
    let mut min_val = axis.dot(shape[0]);
    let mut max_val = min_val;

    for i in 1..shape.len() {
        let dot = axis.dot(shape[i]);
        min_val = min_val.min(dot);
        max_val = max_val.max(dot);
    }

    (min_val, max_val)
}

pub fn checkRange(range_a: (f32, f32), range_b: (f32, f32), invert: bool) -> (bool, bool) {
    //sees if shapes are contained with another
    let mut contain_a: bool = true;
    let mut contain_b: bool = true;
    if invert {
        if range_a.1 < range_b.1 || range_a.0 > range_b.0 {
            contain_a = false
        }
        if range_b.1 < range_a.1 || range_b.0 > range_a.0 {
            contain_b = false
        }
    } else {
        if range_a.1 > range_b.1 || range_a.0 < range_b.0 {
            contain_a = false
        }
        if range_b.1 > range_a.1 || range_b.0 < range_a.0 {
            contain_b = false
        }
    }

    (contain_a, contain_b)
}
