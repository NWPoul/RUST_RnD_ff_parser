pub fn promt_to_exit(msg: &str) {
    println!("{}\nPress 'enter' to exit...\n", {msg});
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}


pub fn abs_max(f_prev: f64, f_new: f64) -> f64 {
    f_prev.abs().max(f_new.abs())
}