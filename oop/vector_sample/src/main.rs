use std::ops::{Add, Sub, Mul, Div};


#[derive(Debug, Clone, Copy)]
struct Vector3d {
    x: f64,
    y: f64,
    z: f64,
}


impl Add for Vector3d {
    type Output = Vector3d;
    fn add(self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vector3d {
    type Output = Vector3d;
    fn sub(self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul<f64> for Vector3d {
    type Output = Vector3d;
    fn mul(self, scalar: f64) -> Vector3d {
        Vector3d {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}
impl Mul<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn mul(self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Div<f64> for Vector3d {
    type Output = Vector3d;
    fn div(self, scalar: f64) -> Vector3d {
        Vector3d {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}
impl Div<Vector3d> for Vector3d {
    type Output = Vector3d;
    fn div(self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl Vector3d {
    fn cross(&self, other: &Vector3d) -> Vector3d {
        Vector3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x, 
        }
    }
}

fn main() {

    let  v1:Vector3d=Vector3d { x: 3.0, y: 1.0, z: 2.0 };
    let  v2:Vector3d=Vector3d { x: 4.0, y: 5.0, z: 12.0 };
    let  v3:Vector3d=Vector3d { x: 24.0, y: 15.0, z: 10.0 };

    println!("{:?}",(v1+v2)*v3);

}
