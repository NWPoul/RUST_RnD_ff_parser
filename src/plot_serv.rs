
// use std::ops::{Add, Div};
use std::fmt::Display;

use gnuplot::{
    Axes2D, AxesCommon, Caption, Color, Figure
};


use crate::utils::u_serv::Vector3d;

// Into<f64> + Add<Output=T> + Div<Output=T> + Copy>



pub fn format_single_series_for_plot(data: &[f64], tick: &f64) -> (Vec<f64>, Vec<f64>) {
    let mut t: Vec<f64> = vec![0.0];
    let mut y: Vec<f64> = Vec::new();

    for (i, tdata) in data.iter().enumerate() {
        t.push(i as f64 * tick);
        y.push((*tdata).clone().into());
    }
    (t, y)
}


pub fn gnu_plot_single_ts_data(ts: &[f64], data: &[f64], title: &str) {
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(title, &[])
        .lines(ts, data, &[Color("black")]);

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}

pub fn gnu_plot_single_data(data: &[f64], tick: &f64, title: &str) {
    let (t,y) = format_single_series_for_plot(data, tick);
    gnu_plot_single_ts_data(&t, &y, title);
}



/// fg_2d, &[([ts], [data], label, color)]
fn add_lines_for_multi_ts_data(fg_2d: &mut Axes2D, ndata: &[(Vec<f64>, Vec<f64>, impl Display, impl Display)]) {
    for data in ndata {
        let (cur_ts, cur_data, cur_label, cur_color) = data;
        fg_2d.lines(
            cur_ts,
            cur_data,
            &[
                Caption(&format!("{}", cur_label)),
                Color(&format!("{}", cur_color)),
            ]
        );
    };
}

fn add_lines_for_v3d(fg_2d: &mut Axes2D, raw_series: &[(Vec<f64>, Vec<Vector3d>, impl Display)]) {
    for smaxyz_data in raw_series.iter() {
        let label = format!("{} pt", smaxyz_data.2);
        let x_data: Vec<f64> = smaxyz_data.1.iter().map(|vector| vector.x).collect();
        let y_data: Vec<f64> = smaxyz_data.1.iter().map(|vector| vector.y).collect();
        let z_data: Vec<f64> = smaxyz_data.1.iter().map(|vector| vector.z).collect();
        fg_2d
            .lines(&smaxyz_data.0, x_data, &[Color("green"), Caption(&format!("x {}",  &label))])
            .lines(&smaxyz_data.0, y_data, &[Color("red")  , Caption(&format!("y {}",  &label))])
            .lines(&smaxyz_data.0, z_data, &[Color("blue") , Caption(&format!("z {}",  &label))]);
    }
}


pub fn gnu_plot_multi_ts_data(multi_ts_data: &[(Vec<f64>, Vec<f64>, impl Display, impl Display)], title: &str) {
    let mut fg: Figure = Figure::new();
    let fg_2d = fg.axes2d().set_title(title, &[]);
    
    add_lines_for_multi_ts_data(fg_2d, multi_ts_data);
    
    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}

pub fn gnu_plot_v3d_and_multi_ts_data(
    v3d_data     : &[(Vec<f64>, Vec<Vector3d>, impl Display)],
    multi_ts_data: &[(Vec<f64>, Vec<f64>     , impl Display, impl Display)],
    title        : &str,
) {
    let mut fg: Figure = Figure::new();
    let fg_2d = fg.axes2d().set_title(title, &[]);

    add_lines_for_v3d(fg_2d, v3d_data);
    add_lines_for_multi_ts_data(fg_2d, &multi_ts_data);
    
    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}




