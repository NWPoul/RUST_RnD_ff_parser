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

#[derive(Clone)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
pub enum Index3D {
    X,
    Y,
    Z,
}


impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn get_axis_val_by_index(&self, i: &Index3D) -> f64 {
        match i {
            Index3D::X => self.x,
            Index3D::Y => self.y,
            Index3D::Z => self.z,
            // _ => panic!("index can be only 0, 1, 2")
        }
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

    pub fn plain_sum(&self) -> f64 {
        self.x + self.y + self.z
    }

    pub fn dot_product(a: &Self, b: &Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn substract(&self, sub: &Self) -> Self {
        Self {
            x: self.x - sub.x,
            y: self.y - sub.y,
            z: self.z - sub.z,
        }
    }
    pub fn add_v3d(&self, add: &Self) -> Self {
        Self {
            x: self.x + add.x,
            y: self.y + add.y,
            z: self.z + add.z,
        }
    }
}

impl From<(f64, f64, f64)> for Vector3d {
    fn from(tuple: (f64, f64, f64)) -> Self {
        Self { x: tuple.0, y: tuple.1, z: tuple.2 }
    }
}



pub fn ends_with_one(value: usize) -> bool {
    let value_str = value.to_string();
    value_str.chars().last() == Some('1')
}
