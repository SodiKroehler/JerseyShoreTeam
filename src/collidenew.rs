use bevy::ecs::component::Component;
use bevy::math::{Vec2, Vec3};
#[derive(Component)]
pub struct Shape {
    pub vertices: Vec<Vec3>,
    pub origin: Vec3,
} //vertices that will define the polygon

pub struct CollisionInfo {
    pub shape_a: Shape,
    pub shape_b: Shape,
    pub distance: f32,
    pub vector: Vec2,
    pub contain_a: bool,
    pub contain_b: bool,
    pub separation: Vec2,
}

#[derive(Component)]
struct RB {
    pos_x: f32,
    pos_y: f32,

    velocity: Vec2,
    acceleration: Vec2,

    rotation: f32,
    torque: f32,

    mass: f32,
    restitution: f32,

    is_static: bool,
}

pub(crate) trait RegularPolygon {
    fn new(sides: usize, radius: f32, origin: Vec3) -> Self;
}

impl RegularPolygon for Shape {
    fn new(sides: usize, radius: f32, origin: Vec3) -> Shape {
        let mut vertices: Vec<Vec3> = vec![Default::default(); 0];

        for i in 0..sides {
            let x: f32 = ((i as f32) / (sides as f32) * 2.0 * std::f32::consts::PI).cos() * radius
                + origin.x;
            let y: f32 = ((i as f32) / (sides as f32) * 2.0 * std::f32::consts::PI).sin() * radius
                + origin.y;
            let z = origin.z;
            vertices.push(Vec3 { x: x, y: y, z: z });
            //println!("x{}, y{}, z{}", x, y, origin.z);
        }

        Shape {
            vertices: vertices,
            origin: origin,
        }
    }
}

pub fn rotate(shape: &mut Shape, angle: f32) -> &mut Shape {
    for mut vert in shape.vertices.iter_mut() {
        //*vert -= shape.origin;
        let mut temp_vert: Vec2 = vert.truncate();
        temp_vert = Vec2::from_angle(angle).rotate(temp_vert);
        *vert = Vec3 {
            x: temp_vert.x,
            y: temp_vert.y,
            z: vert.z,
        };
        //*vert += shape.origin;
    }
    return shape;
}

pub fn move_shape(shape: &mut Shape, direction: Vec3) -> &mut Shape {
    for mut vert in shape.vertices.iter_mut() {
        vert.x += direction.x;
        vert.y += direction.y;
        vert.z += direction.z;
    }
    shape.origin += direction;
    return shape;
}

pub fn sat(shape_a: &Shape, shape_b: &Shape) -> Option<CollisionInfo> {
    let a_vertices: Vec<Vec3> = shape_a.vertices.to_vec();
    let b_vertices: Vec<Vec3> = shape_b.vertices.to_vec();
    let a_pos: Vec3 = shape_a.origin;
    let b_pos: Vec3 = shape_b.origin;
    //println!("{}", a_pos);
    let mut axes: Vec<Vec2> = vec![Default::default(); 0]; //perpindicular axes to project onto
    let mut poly_a = Vec::<Vec2>::with_capacity(6);
    let mut poly_b = Vec::<Vec2>::with_capacity(6);
    let mut shortest: f32 = f32::MAX;

    let mut col = CollisionInfo {
        //setup stuff for resolution
        shape_a: Shape{
        	vertices: shape_a.vertices.clone(),
        	origin: shape_a.origin,
        },
        shape_b: Shape{
        	vertices: shape_b.vertices.clone(),
        	origin: shape_b.origin,
        },
        distance: 0.0,
        vector: Vec2 { x: 0.0, y: 0.0 },
        contain_a: true,
        contain_b: true,
        separation: Vec2 { x: 0.0, y: 0.0 },
    };

    for a in a_vertices.iter() {
        //remove z axis for calculations
        poly_a.push(a.truncate());
    }
    for b in b_vertices.iter() {
        poly_b.push(b.truncate());
    }

    for i in 0..poly_a.len() {
        axes.push(
            Vec2 {
                //get perpindicular to axis
                x: poly_a[i].y - poly_a[(i + 1) % poly_a.len()].y,
                y: poly_a[(i + 1) % poly_a.len()].x - poly_a[i].x,
            }
            .normalize_or_zero(),
        );
    }
    for i in 0..poly_b.len() {
        axes.push(
            Vec2 {
                //get perpindicular
                x: poly_b[i].y - poly_b[(i + 1) % poly_b.len()].y,
                y: poly_b[(i + 1) % poly_b.len()].x - poly_b[i].x,
            }
            .normalize_or_zero(),
        );
    }

    let v_offset: Vec2 = Vec2 {
        //offset of shape origins
        x: a_pos.x - b_pos.x,
        y: a_pos.y - b_pos.y,
    };

    for i in 0..axes.len() {
        let mut poly_a_range: (f32, f32) = project_shape(&poly_a, &axes[i]);
        let poly_b_range: (f32, f32) = project_shape(&poly_b, &axes[i]);

        let offset = axes[i].dot(v_offset); //project the shape offset onto this axis
        poly_a_range.0 += offset; //put shape A onto this offset
        poly_a_range.1 += offset;

        if (poly_a_range.0 - poly_b_range.1 > 0.0) || (poly_b_range.0 - poly_a_range.1 > 0.0) {
            //gap, do not need to keep checking definitely not colliding
            return None;
        }

        let checked: (bool, bool) = check_range(poly_a_range, poly_b_range, false);
        col.contain_a = checked.0;
        col.contain_b = checked.1;

        let min_dist: f32 = -(poly_b_range.1 - poly_a_range.0); //collision distance on this axis
        let abs_min: f32 = min_dist.abs();

        if abs_min < shortest {
            //finds axis with the shortest collision, meaning furthest inside
            shortest = abs_min;
            col.distance = min_dist;
            col.vector = axes[i];
        }
    }

    col.separation = Vec2 {
        //how to get the shape outside
        x: col.vector.x * col.distance,
        y: col.vector.y * col.distance,
    };

    return Some(col);
}

pub fn poly_circle_collide(polygon: &Shape, circle_pos: &Vec2, circle_rad: &f32) -> Option<CollisionInfo> {//shape, radius
    let mut shortest = f32::MIN;
    let vertices: Vec<Vec3> = line_work(&mut polygon.vertices.to_vec());
    let mut poly: Vec<Vec2> = Vec::<Vec2>::with_capacity(vertices.len());
    let mut axes: Vec<Vec2> = vec![Default::default(); 0];


    let mut col = CollisionInfo {
        //setup stuff for resolution
        shape_a: Shape{
        	vertices: polygon.vertices.clone(),
        	origin: polygon.origin,
        },
        shape_b: Shape{
        	vertices: polygon.vertices.clone(),
        	origin: polygon.origin,
        },
        distance: 0.0,
        vector: Vec2 { x: 0.0, y: 0.0 },
        contain_a: true,
        contain_b: true,
        separation: Vec2 { x: 0.0, y: 0.0 },
    };
    
    for a in vertices.iter() {
        poly.push(a.truncate());
    }

    let v_offset: Vec2 = (polygon.origin.truncate() - *circle_pos);

    let mut close: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    for v in poly.iter() {
        let t = *v + polygon.origin.truncate();
        let distance = circle_pos.distance(t);
        if distance < shortest {
            shortest = distance;
            close.x = polygon.origin.x + v.x;
            close.y = polygon.origin.y + v.y;
        }
    }

    let axis: Vec2 = Vec2 {
        x: close.x - circle_pos.x,
        y: close.y - circle_pos.y,
    };

    let mut p_range_temp = project_shape(&poly, &axis);
    let s_offset_temp = axis.dot(v_offset.clone());
    p_range_temp.0 += s_offset_temp;
    p_range_temp.1 += s_offset_temp;

    let c_range_temp = project_circle(&circle_rad, &axis);
    if (p_range_temp.0 - c_range_temp.1 > 0.0) || (c_range_temp.0 - p_range_temp.1 > 0.0) {
        return None;
    }

    let distance_min = c_range_temp.1 - p_range_temp.0;
    shortest = distance_min.abs();

    col.distance = distance_min;
    col.vector = axis;

    (col.contain_a, col.contain_b) = check_range(p_range_temp, c_range_temp, false);

    axes = make_normal(&poly, &mut axes);

    for i in 0..poly.len() {
        let mut p_range = project_shape(&poly, &axes[i]);

        let s_offset = axes[i].dot(v_offset);
        p_range.0 += s_offset;
        p_range.1 += s_offset;

        let c_range = project_circle(&circle_rad, &axes[i]);

        if (p_range.0 - c_range.1 > 0.0) || (c_range.0 - p_range.1 > 0.0) {
            return None;
        }

        (col.contain_a, col.contain_b) = check_range(p_range, c_range, false);

        let distance_min_temp = c_range.1 - p_range.0;

        if distance_min_temp.abs() < shortest {
            shortest = distance_min_temp.abs();

            col.distance = distance_min_temp;
            col.vector = axes[i];
        }
    }

    col.separation = col.vector * col.distance;

    return Some(col);
}

pub fn project_shape(shape: &Vec<Vec2>, axis: &Vec2) -> (f32, f32) {
    //do dot product for first vector onto axis
    let mut min_val = axis.dot(shape[0]);
    let mut max_val = min_val;

    for i in 1..shape.len() {
        //project all vertices onto the axis
        let dot = axis.dot(shape[i]);
        min_val = min_val.min(dot);
        max_val = max_val.max(dot);
    }

    (min_val, max_val)
}
pub fn project_circle(radius: &f32, axis: &Vec2) -> (f32, f32) {
    let projection = axis.dot(Vec2 { x: 0.0, y: 0.0 });
    (projection - radius, projection + radius)
}

fn line_work(verts: &mut Vec<Vec3>) -> Vec<Vec3> {
    if verts.len() == 2 {
        let p1: Vec3 = verts[0];
        let p2: Vec3 = verts[1];
        let mut p3: Vec3 = Vec3 {
            x: p1.y - p2.y,
            y: p2.x - p1.x,
            z: p1.z,
        };
        p3 = p3.normalize() * 0.00001;
        verts.push(p3);
    }
    verts.to_vec()
}
fn make_normal(verts: &Vec<Vec2>, axes: &mut Vec<Vec2>) -> Vec<Vec2> {
    for i in 0..verts.len() {
        axes.push(
            Vec2 {
                //get perpindicular to axis
                x: verts[i].y - verts[(i + 1) % verts.len()].y,
                y: verts[(i + 1) % verts.len()].x - verts[i].x,
            }
            .normalize_or_zero(),
        );
    }
    axes.to_vec()
}

pub fn check_range(range_a: (f32, f32), range_b: (f32, f32), invert: bool) -> (bool, bool) {
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
