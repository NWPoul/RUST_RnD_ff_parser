use gnuplot::{
    Figure,
    // Caption,
    Color,
};

pub fn format_data_for_plot(data: &Vec<(f64, f64, f64)>) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut t: Vec<f64> = vec![0.0];
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();

    for (i, tdata) in data.iter().enumerate() {
        t.push(i as f64 * 0.005);
        let (xv, yv, zv) = tdata;
        x.push(*xv);
        y.push(*yv);
        z.push(*zv);
    }
    (t, x, y, z)
}

pub fn get_sma_vec(data: (&Vec<f64>, &Vec<f64>, &Vec<f64>, &Vec<f64>), base: usize) -> (Vec<f64>, Vec<f64>) {
    let (t, x, y, z) = data;
    let mut sma_x = Vec::new();
    let mut sma_y = Vec::new();
    let mut sma_z = Vec::new();

    let mut sma_vec = vec![0.];
    let mut sma_t = vec![0.];

    for i in base..t.len() {
        let cur_sma_x: f64 = x[i - base..i].iter().sum::<f64>();
        let cur_sma_y: f64 = y[i - base..i].iter().sum::<f64>();
        let cur_sma_z: f64 = z[i - base..i].iter().sum::<f64>();
        sma_x.push(cur_sma_x);
        sma_y.push(cur_sma_y);
        sma_z.push(cur_sma_z);

        sma_vec.push(
            f64::sqrt(cur_sma_x.powi(2) + cur_sma_y.powi(2) + cur_sma_z.powi(2)) / base as f64
        );
        sma_t.push(i as f64 * 0.005);
    }

    (sma_t, sma_vec)
}

pub fn get_max_vec_data(data: (Vec<f64>, Vec<f64>)) -> (f64, f64) {
    let (max_t, max_vec) = data.1
        .iter()
        .enumerate()
        .max_by(
            |prev, next| prev.1.partial_cmp(next.1).unwrap_or(std::cmp::Ordering::Greater)
        )
        .unwrap_or((0,&0.));
    (max_t as f64 * 0.005, *max_vec)
}


pub fn gnu_plot_test(data: &Vec<(f64, f64, f64)>) {
    let (t, x, y, z) = format_data_for_plot(data);
    let (sma_t, sma_v) = get_sma_vec((&t, &x, &y, &z), 50);
    let (sma_t2, sma_v2) = get_sma_vec((&t, &x, &y, &z), 100);
    let (sma_t3, sma_v3) = get_sma_vec((&t, &x, &y, &z), 200);

    let mut fg = Figure::new();
    let mut fg2 = Figure::new();
    fg.axes2d()
        .lines(&t, &x, &[Color("green")])
        .lines(&t, &y, &[Color("red")])
        .lines(&t, &z, &[Color("blue")])
        .lines(&sma_t, &sma_v, &[Color("black")]);

    fg2.axes2d()
        .lines(&sma_t, &sma_v, &[Color("black")])
        .lines(&sma_t2, &sma_v2, &[Color("blue")])
        .lines(&sma_t3, &sma_v3, &[Color("green")]);

    std::thread::spawn(move || {
        fg.show().unwrap();
    });

    std::thread::spawn(move || {
        fg2.show().unwrap();
    });
}
