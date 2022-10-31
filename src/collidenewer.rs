use bevy::ecs::component::Component;
use bevy::math::{Vec2, Vec3};
pub struct Shape {
    pub vertices: Vec<Vec3>,
    pub origin: Vec3,
    pub radius: f32,
} //vertices that will define the polygon

pub struct CollisionInfo {
    shape_a: RB,
    shape_b: RB,
    distance: f32, //distance of origins on shortest path
    vector: Vec2,  //direction to move it
    contain_a: bool,
    contain_b: bool,
    seperation: Vec2, //contains the direction and distance to push the thing outside
}

#[derive(Component)]
pub struct RB {
    position: Vec3,

    velocity: Vec2,
    acceleration: Vec2,
    inertia: f32,

    angle: f32,
    angular_velocity: f32,

    force: Vec2,
    torque: Vec2,

    mass: f32,
    restitution: f32,
    area: f32,

    collider: Shape,
    is_circle: bool,

    is_static: bool,
    temp_static: bool,
}

impl RB {
    fn new(position: Vec3, mass: f32, restitution: f32, collider: Shape, is_static: bool) -> RB {
        let mut area: f32 = 0.0;
        let is_circle = collider.vertices.len() == 0;
        if !is_circle {
            let length: f32 = collider.vertices[0].distance(collider.vertices[1]).abs(); //taking pretty big assuptions of regularity
            let sides: usize = collider.vertices.len();
            area = shape_area_approximation(sides, length);
        } else {
            //is a circle
            area = std::f32::consts::PI * collider.radius * collider.radius;
        }
        RB {
            position: position,

            velocity: Vec2 { x: 0.0, y: 0.0 },
            acceleration: Vec2 { x: 0.0, y: 0.0 },
            inertia: calc_inertia(&collider, &mass),

            angle: 0.0,
            angular_velocity: 0.0,

            force: Vec2 { x: 0.0, y: 0.0 },
            torque: Vec2 { x: 0.0, y: 0.0 },

            mass: mass,
            restitution: restitution,
            area: area,

            collider: collider,
            is_circle: is_circle,

            is_static: is_static,
            temp_static: true,
        }
    }
}

impl CollisionInfo {
    fn new(rb_a: RB, rb_b: RB) -> CollisionInfo {
        CollisionInfo {
            //setup stuff for resolution
            shape_a: rb_a,
            shape_b: rb_b,
            distance: 0.0,
            vector: Vec2 { x: 0.0, y: 0.0 },
            contain_a: true,
            contain_b: true,
            seperation: Vec2 { x: 0.0, y: 0.0 },
        }
    }
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
            radius: radius,
        }
    }
}

//https://en.wikipedia.org/wiki/List_of_moments_of_inertia
fn calc_inertia(shape: &Shape, mass: &f32) -> f32 {
    if shape.vertices.len() == 0 {
        return mass * shape.radius * shape.radius;
    }
    let mut denominator: f32 = 0.0;
    let mut numerator = 0.0;

    for n in 1..shape.vertices.len() {
        let p1: Vec2 = shape.vertices[n - 1].truncate();
        let p2: Vec2 = shape.vertices[n].truncate();
        let mag = (p1 * p2).length();
        denominator += mag;
        numerator += mag * ((p1.dot(p1)) + (p1.dot(p2)) + (p2.dot(p2)));
    }
    mass * (numerator / (6.0 * denominator))
}

fn shape_area_approximation(sides: usize, length: f32) -> f32 {
    //dont care how this works for
    (length * length * (sides as f32))
        / (4.0 * ((180.0 / (sides as f32)) * std::f32::consts::PI / 180.0).tan())
}

pub fn rotate(shape: &mut Shape, angle: f32) -> &mut Shape {
    for mut vert in shape.vertices.iter_mut() {
        *vert -= shape.origin;
        let mut temp_vert: Vec2 = vert.truncate();
        temp_vert = Vec2::from_angle(angle).rotate(temp_vert);
        *vert = Vec3 {
            x: temp_vert.x,
            y: temp_vert.y,
            z: vert.z,
        };
        *vert += shape.origin;
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

pub fn sat_polygon_polygon(shape_a: RB, shape_b: RB) -> Option<CollisionInfo> {
    let a_vertices: Vec<Vec3> = shape_a.collider.vertices.to_vec();
    let b_vertices: Vec<Vec3> = shape_b.collider.vertices.to_vec();
    let a_pos: Vec3 = shape_a.position;
    let b_pos: Vec3 = shape_b.position;
    println!("{}", a_pos);
    let mut axes: Vec<Vec2> = vec![Default::default(); 0]; //perpindicular axes to project onto
    let mut poly_a = Vec::<Vec2>::with_capacity(6);
    let mut poly_b = Vec::<Vec2>::with_capacity(6);
    let mut shortest: f32 = f32::MAX;

    let mut col: CollisionInfo = CollisionInfo::new(shape_a, shape_b);

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

    col.seperation = Vec2 {
        //how to get the shape outside
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
        //project all vertices onto the axis
        let dot = axis.dot(shape[i]);
        min_val = min_val.min(dot);
        max_val = max_val.max(dot);
    }

    (min_val, max_val)
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

pub fn resolve(info: &mut CollisionInfo, dt: bevy::utils::Duration) {
    //calc torque and force. kinda be lookin like newtons second if you know what i mean yuh yuh yuh yuh google picture of newtowns second law and come back to me B) yup thats right its fucking sweet right?
    let mut rb_a: &RB = &info.shape_a;
    let mut rb_b: &RB = &info.shape_b;

    info.vector;
}

pub fn add_forces(rb: RB) {}
