

pub fn prompt_to(sys_msg: &str, msg: &str) -> bool {
    println!("{msg}\n{sys_msg}\n");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().is_empty()
}

pub fn prompt_to_exit(msg: &str) {
    let sys_msg = "Press 'enter' to exit...";
    prompt_to(sys_msg, msg);
}

pub fn prompt_to_continue(msg: &str) -> bool {
    let sys_msg = "Press 'enter' to continue...";
    prompt_to(sys_msg, msg)
}



pub fn abs_max(f_prev: f64, f_new: f64) -> f64 {
    f_prev.abs().max(f_new.abs())
}

pub fn remove_symbols(input: &str, symbols: &str) -> String {
    input.chars().filter(|c| !symbols.contains(*c)).collect()
}
