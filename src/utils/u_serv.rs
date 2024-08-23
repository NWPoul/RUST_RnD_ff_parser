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

pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    pub fn dot_product(a: &Vector3d, b: &Vector3d) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn angle_between(a: &Vector3d, b: &Vector3d) -> f64 {
        let cos_theta = Self::dot_product(a, b) / (a.magnitude() * b.magnitude());
        cos_theta.acos()
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
