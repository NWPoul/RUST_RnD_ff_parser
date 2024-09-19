use std::f64;



pub fn prompt_to_exit(msg: &str) {
    println!("{}\nPress 'enter' to exit...\n", {msg});
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

pub fn prompt_to_continue(msg: &str) {
    println!("{}\nPress 'enter' to continue...\n", {msg});
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}


pub fn abs_max(f_prev: f64, f_new: f64) -> f64 {
    f_prev.abs().max(f_new.abs())
}





pub fn ends_with_one(value: usize) -> bool {
    let value_str = value.to_string();
    value_str.chars().last() == Some('1')
}



pub enum Axis3d { X, Y, Z }
#[derive(Clone)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub const AXIS: [Axis3d; 3] = [Axis3d::X, Axis3d::Y, Axis3d::Z];
    
    pub fn new(x:f64, y:f64, z:f64) -> Self {
        Self { x, y, z }
    }
    
    pub fn get_axis_val(&self, i: &Axis3d) -> f64 {
        match i {
            Axis3d::X => self.x,
            Axis3d::Y => self.y,
            Axis3d::Z => self.z,
        }
    }

    pub fn axis_iter() -> std::slice::Iter<'static, Axis3d> {
        Self::AXIS.iter()
    }

    pub fn apply_for_all_axis<F: Fn(f64)->f64>(&self, f: F) -> Self {
        Self {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    pub fn sum_axis(&self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn dot_product(&self, b: &Self) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    fn v3sum(&self, other: &Self, neg: bool) -> Self {
        let sign = if neg {-1.0} else {1.0};
        Self {
            x: self.x + sign * other.x,
            y: self.y + sign * other.y,
            z: self.z + sign * other.z,
        }
    }
    pub fn v3sub(&self, sub: &Self) -> Self { self.v3sum(sub, true ) }
    pub fn v3add(&self, add: &Self) -> Self { self.v3sum(add, false) }
}

impl From<(f64, f64, f64)> for Vector3d {
    fn from(input: (f64, f64, f64)) -> Self {
        Self { x: input.0, y: input.1, z: input.2 }
    }
}
impl From<[f64; 3]> for Vector3d {
    fn from(input: [f64; 3]) -> Self {
        // if input.len() < 3 {
        //     panic!("Input array/slice must have at least three elements");
        // }
        Self { x: input[0], y: input[1], z: input[2] }
    }
}

