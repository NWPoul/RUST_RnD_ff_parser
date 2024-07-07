

use gnuplot::{
    Figure,
    Caption,
    Color,
};

use crate::analise::parser_data_to_sma_list;





pub fn format_data_for_plot(data: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut t: Vec<f64> = vec![0.0];
    let mut y = Vec::new();

    for (i, tdata) in data.iter().enumerate() {
        t.push(i as f64 * 0.005);
        y.push(*tdata);
    }
    (t, y)
}



pub fn gnu_plot_xyz(data: &Vec<(f64, f64, f64)>) {
    let mut t: Vec<f64> = vec![0.0];
    let mut xy: Vec<f64> = Vec::new();
    let mut yy: Vec<f64> = Vec::new();
    let mut zy: Vec<f64> = Vec::new();

    for (i, tdata) in data.iter().enumerate() {
        t.push(i as f64 * 0.005);
        xy.push(tdata.0);
        yy.push(tdata.1);
        zy.push(tdata.2);
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&t, &xy, &[Color("green")])
        .lines(&t, &yy, &[Color("red")])
        .lines(&t, &zy, &[Color("blue")]);

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}

pub fn gnu_plot_single(data: &[f64]) {
    let (t,y) = format_data_for_plot(data);

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&t, &y, &[Color("black")]);

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}




pub fn gnu_plot_series(data: &Vec<(f64, f64, f64)>, base_series: &[usize]) {
    let mut sma_series: Vec<(Vec<f64>,Vec<f64>, usize)> = Vec::new();

    for base in base_series {
        let cur_sma = parser_data_to_sma_list(data, *base);
        sma_series.push((cur_sma.0, cur_sma.1, *base));
    }

    let mut fg = Figure::new();
    let fg_2d = fg.axes2d();
    for sma_data in sma_series.iter() {
        let label = format!("{} ms", sma_data.2);
        fg_2d.lines(
            &sma_data.0,
            &sma_data.1,
            &[
                Caption(&label),
                Color("black"),
            ]
        );
    }

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}