
use std::ops::{Add, Div};

use gnuplot::{
    AxesCommon, Caption, Color, Figure
};


use crate::utils::u_serv::Vector3d;
use crate::analise::{data_to_sma, parser_data_to_sma_list, parser_data_to_t_sma_xyz_list};

use crate::PLOT_RAW;



pub fn format_data_for_plot<T: Into<f64> + Add<Output=T> + Div<Output=T> + Copy>(data: &[T], tick: &f64) -> (Vec<f64>, Vec<f64>) {
    let mut t: Vec<f64> = vec![0.0];
    let mut y: Vec<f64> = Vec::new();

    for (i, tdata) in data.iter().enumerate() {
        t.push(i as f64 * tick);
        y.push((*tdata).clone().into());
    }
    (t, y)
}


pub fn gnu_plot_ts_data(ts: &[f64], data: &[f64], title: &str) {
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(title, &[])
        .lines(ts, data, &[Color("black")]);

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}


pub fn gnu_plot_single<T: Into<f64> + Add<Output=T> + Div<Output=T> + Copy>(data: &[T], tick: &f64, title: &str) {
    let (t,y) = format_data_for_plot(data, tick);
    gnu_plot_ts_data(&t, &y, title);
}

pub fn gnu_plot_single_sma(data: &[f64], tick: &f64, base: usize, title: &str) {
    let data = data_to_sma(data.into(), base);
    gnu_plot_single(&data, tick, title);
}



pub fn gnu_plot_series(data: &[Vector3d], base_series: &[usize]) {
    let mut sma_series   : Vec<(Vec<f64>, Vec<f64>     , usize)> = Vec::new();
    let mut smaxyz_series: Vec<(Vec<f64>, Vec<Vector3d>, usize)> = Vec::new();

    for base in base_series {
        let cur_sma = parser_data_to_sma_list(data, *base);
        let cur_smaxyz = parser_data_to_t_sma_xyz_list(data, *base);
        sma_series.push((cur_sma.0, cur_sma.1, *base));
        smaxyz_series.push((cur_smaxyz.0, cur_smaxyz.1, *base));
    }

    let mut fg: Figure = Figure::new();
    let fg_2d = fg.axes2d();
    
    if PLOT_RAW {
        for smaxyz_data in smaxyz_series.iter() {
            let label = format!("{} pt", smaxyz_data.2);
            let x_data: Vec<f64> = smaxyz_data.1.iter().map(|vector| vector.x).collect();
            let y_data: Vec<f64> = smaxyz_data.1.iter().map(|vector| vector.y).collect();
            let z_data: Vec<f64> = smaxyz_data.1.iter().map(|vector| vector.z).collect();
            fg_2d
                .lines(&smaxyz_data.0, x_data, &[Color("green"), Caption(&format!("raw_x {}",  &label))])
                .lines(&smaxyz_data.0, y_data, &[Color("red")  , Caption(&format!("raw_y {}",  &label))])
                .lines(&smaxyz_data.0, z_data, &[Color("blue") , Caption(&format!("raw_z {}",  &label))]);
        }
    }

    for sma_data in sma_series.iter() {
        let label = format!("{} pt", sma_data.2);
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