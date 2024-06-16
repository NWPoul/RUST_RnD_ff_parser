use gnuplot::{
    Figure,
    // Caption,
    Color,
};

pub fn format_data_for_plot(data: &Vec<(f64, f64, f64)>) -> (Vec<f64>, Vec<f64>) {
    let mut t = vec![0.0];
    let mut xp = vec![];

    let mut i = 0;
    for (xv, yv, zvli) in data {
        t.push(i as f64 * 0.005);
        xp.push(*xv);
        i += 1;
    };
    (t, xp)
}

pub fn gnu_plot_test(data: &Vec<(f64, f64, f64)>) {
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

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(t.clone(), x.clone(), &[Color("black")])
        .lines(t.clone(), y.clone(), &[Color("red")])
        .lines(t.clone(), z.clone(), &[Color("blue")]);

    
    _ = fg.show().unwrap();

    fg.axes2d()
        .lines(x, y, &[Color("black")]);
    _ = fg.show().unwrap();
}
