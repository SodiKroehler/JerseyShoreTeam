//! Utilities for detecting if and on which side two axis-aligned bounding boxes (AABB) collide.
use bevy::math::Vec2;
use bevy::math::Vec3;

#[derive(Debug)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Inside,
}

pub fn circle_collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<Collision> {
    let distance = a_pos.truncate() - b_pos.truncate();
    let size = a_size + b_size;
    let a_max = a_pos.truncate() + a_size;
    let a_min = a_pos.truncate() - a_size;
    let b_max = b_pos.truncate() + b_size;
    let b_min = b_pos.truncate() - b_size;

    if distance.x.abs() <= size.x && distance.y.abs() <= size.y {
        let (x_collision, x_depth) = {
            if distance.x < 0.0 && a_max.x > b_min.x && a_max.x < b_max.x
            //left side collision
            {
                (Collision::Left, b_min.x - a_max.x)
            } else if distance.x > 0.0 && a_min.x < b_max.x && a_max.x > b_max.x {
                (Collision::Right, a_min.x - b_min.x)
            } else
            //is inside of the other circle
            {
                (Collision::Inside, -f32::INFINITY)
            }
        };

        let (y_collision, y_depth) = {
            if distance.y < 0.0 && a_max.y > b_min.y && a_max.y < b_max.y {
                (Collision::Bottom, b_min.y - a_max.y)
            } else if distance.y > 0.0 && a_min.y < b_max.y && a_max.y > b_max.y {
                (Collision::Top, a_min.y - b_max.y)
            } else {
                (Collision::Inside, -f32::INFINITY)
            }
        };

        if y_depth.abs() < x_depth.abs() {
            return Some(y_collision);
        } else {
            return Some(x_collision);
        }
    }
    return None;
}

//adapted from http://jeffreythompson.org/collision-detection/circle-rect.php
pub fn rectangle_circle_collide(
    c_pos: Vec3,
    c_size: Vec2,
    r_pos: Vec3,
    r_size: Vec2,
) -> Option<Collision> {
    let c_max = c_pos.truncate() + c_size;
    let c_min = c_pos.truncate() - c_size;
    let r_max = r_pos.truncate() + r_size / 2.0;
    let r_min = r_pos.truncate() - r_size / 2.0;

    let close: Vec2 = {
        //find the closest rectangle edge
        if c_min.x < r_min.x && c_max.y < r_max.y
        //top left
        {
            Vec2 {
                x: r_min.x,
                y: r_max.y,
            }
        } else if c_min.x < r_min.x && c_max.y > r_min.y
        //bottom left
        {
            Vec2 {
                x: r_min.x,
                y: r_min.y,
            }
        } else if c_min.x > r_max.x && c_max.y < r_max.y
        //top right
        {
            Vec2 {
                x: r_max.x,
                y: r_max.y,
            }
        } else
        //if c_min.x > r_max.x && c_max.y > r_min.y  //bottom right
        {
            Vec2 {
                x: r_max.x,
                y: r_min.y,
            }
        }
    };

    let dist = Vec2 {
        x: c_pos.x - close.x,
        y: c_pos.y - close.y,
    };
    if dist.x.powi(2) + dist.y.powi(2) <= c_size.x.powi(2) {
        let (x_collision, x_depth) = {
            if c_min.x < r_min.x && c_max.x > r_min.x && c_max.x < r_max.x {
                (Collision::Left, r_min.x - c_max.x)
            } else if c_min.x > r_min.x && c_min.x < r_max.x && c_max.x > r_max.x {
                (Collision::Right, c_min.x - r_max.x)
            } else {
                (Collision::Inside, -f32::INFINITY)
            }
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = {
            if c_min.y < r_min.y && c_max.y > r_min.y && c_max.y < r_max.y {
                (Collision::Bottom, r_min.y - c_max.y)
            } else if c_min.y > r_min.y && c_min.y < r_max.y && c_max.y > r_max.y {
                (Collision::Top, c_min.y - r_max.y)
            } else {
                (Collision::Inside, -f32::INFINITY)
            }
        };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        if y_depth.abs() < x_depth.abs() {
            return Some(y_collision);
        } else {
            return Some(x_collision);
        }
    }
    return None;
}

// TODO: ideally we can remove this once bevy gets a physics system
/// Axis-aligned bounding box collision with "side" detection
/// * `a_pos` and `b_pos` are the center positions of the rectangles, typically obtained by
/// extracting the `translation` field from a `Transform` component
/// * `a_size` and `b_size` are the dimensions (width and height) of the rectangles.
pub fn collide(a_pos: Vec3, a_size: Vec2, b_pos: Vec3, b_size: Vec2) -> Option<Collision> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
        {
            (Collision::Left, b_min.x - a_max.x)
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Collision::Right, a_min.x - b_max.x)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
        {
            (Collision::Bottom, b_min.y - a_max.y)
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Collision::Top, a_min.y - b_max.y)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        if y_depth.abs() < x_depth.abs() {
            Some(y_collision)
        } else {
            Some(x_collision)
        }
    } else {
        None
    }
}
