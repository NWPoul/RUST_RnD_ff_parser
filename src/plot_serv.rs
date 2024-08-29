
use std::ops::{Add, Div};

use gnuplot::{
    AxesCommon, Caption, Color, Figure
};


use crate::utils::u_serv::Vector3d;
use crate::analise::{
    data_to_sma_spread_data, v3d_list_to_magnitude_sma_list, v3d_list_to_magnitude_smaspr_list, v3d_list_to_plainsum_sma_list, v3d_list_to_ts_sma_v3d_list
};

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
    let stat_data = data_to_sma_spread_data(data.into(), base);
    gnu_plot_single(&stat_data.0, tick, title);
}
pub fn gnu_plot_single_spr(data: &[f64], tick: &f64, base: usize, title: &str) {
    let stat_data = data_to_sma_spread_data(data.into(), base);
    gnu_plot_single(&stat_data.1, tick, title);
}


pub fn gnu_plot_series(data: &[Vector3d], series_props: &[usize]) {
    let mut sma_magnitude_series: Vec<(Vec<f64>, Vec<f64>     , usize)> = Vec::new();
    let mut sma_plainsum_series : Vec<(Vec<f64>, Vec<f64>     , usize)> = Vec::new();
    let mut sma_raw_series      : Vec<(Vec<f64>, Vec<Vector3d>, usize)> = Vec::new();

    for base in series_props {
        let cur_magnitude_sma = v3d_list_to_magnitude_sma_list(data, *base);
        let cur_plainsum_sma  = v3d_list_to_plainsum_sma_list(data, *base);
        let cur_v3d_sma       = v3d_list_to_ts_sma_v3d_list(data, *base);
        sma_magnitude_series.push((cur_magnitude_sma.0, cur_magnitude_sma.1, *base));
        sma_plainsum_series.push((cur_plainsum_sma.0, cur_plainsum_sma.1, *base));
        sma_raw_series.push((cur_v3d_sma.0, cur_v3d_sma.1, *base));
    }

    let mut fg: Figure = Figure::new();
    let fg_2d = fg.axes2d();
    
    if PLOT_RAW {
        for smaxyz_data in sma_raw_series.iter() {
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

    for sma_data in sma_magnitude_series.iter() {
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
    for sma_data in sma_plainsum_series.iter() {
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


pub fn gnu_plot_stats_for_data(data: &[Vector3d], base_series: &[usize]) {
    let mut sma_magnitude_series: Vec<(Vec<f64>, Vec<f64>, usize)> = Vec::new();
    let mut spr_magnitude_series: Vec<(Vec<f64>, Vec<f64>, usize)> = Vec::new();

    for base in base_series {
        let cur_stats = v3d_list_to_magnitude_smaspr_list(data, *base);
        sma_magnitude_series.push((cur_stats.0.clone(), cur_stats.1, *base));
        spr_magnitude_series.push((cur_stats.0, cur_stats.2, *base));
    }

    let mut fg: Figure = Figure::new();
    let fg_2d = fg.axes2d();

    for sma_data in sma_magnitude_series.iter() {
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
    for spr_data in spr_magnitude_series.iter() {
        let label = format!("{} pt", spr_data.2);
        fg_2d.lines(
            &spr_data.0,
            &spr_data.1,
            &[
                Caption(&label),
                Color("brown"),
            ]
        );
    }

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}