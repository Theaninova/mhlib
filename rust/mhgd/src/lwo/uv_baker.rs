use godot::builtin::{Transform3D, Vector2, Vector3};

enum Axis {
    X,
    Y,
    Z,
}

pub fn project_planar(vertex: Vector3, axis: u16, transform: Transform3D) -> Vector2 {
    by_axis(
        transform * vertex,
        match axis {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => panic!(),
        },
    )
}

pub fn project_cubic(vertex: Vector3, normal: Vector3, transform: Transform3D) -> Vector2 {
    let p = transform * vertex;
    let n = (transform.basis * normal).abs().normalized();
    let axis = if n.x > n.y && n.x > n.z {
        Axis::X
    } else if n.y > n.x && n.y > n.z {
        Axis::Y
    } else {
        Axis::Z
    };
    by_axis(p, axis)
}

fn by_axis(v: Vector3, axis: Axis) -> Vector2 {
    match axis {
        Axis::X => Vector2 {
            x: v.z + 0.5,
            y: v.y + 0.5,
        },
        Axis::Y => Vector2 {
            x: v.z + 0.5,
            y: v.x + 0.5,
        },
        Axis::Z => Vector2 {
            x: v.x + 0.5,
            y: v.y + 0.5,
        },
    }
}
