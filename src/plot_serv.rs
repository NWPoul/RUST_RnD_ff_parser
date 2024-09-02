
use std::ops::{Add, Div};

use gnuplot::{
    AxesCommon, Caption, Color, Figure, PlotOption
};


use crate::utils::u_serv::Vector3d;
use crate::analise::{
    // data_to_stat_vecs,
    v3d_list_to_magnitude_sma_list,
    v3d_list_to_magnitude_smaspr_list,
    v3d_list_to_plainsum_sma_list,
    v3d_list_to_ts_sma_v3d_list,
};

use crate::PLOT_RAW;



pub fn format_single_series_for_plot<T: Into<f64> + Add<Output=T> + Div<Output=T> + Copy>(data: &[T], tick: &f64) -> (Vec<f64>, Vec<f64>) {
    let mut t: Vec<f64> = vec![0.0];
    let mut y: Vec<f64> = Vec::new();

    for (i, tdata) in data.iter().enumerate() {
        t.push(i as f64 * tick);
        y.push((*tdata).clone().into());
    }
    (t, y)
}


pub fn gnu_plot_single_ts_data_series(ts: &[f64], data: &[f64], title: &str) {
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title(title, &[])
        .lines(ts, data, &[Color("black")]);

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}


pub fn gnu_plot_single_series<T: Into<f64> + Add<Output=T> + Div<Output=T> + Copy>(data: &[T], tick: &f64, title: &str) {
    let (t,y) = format_single_series_for_plot(data, tick);
    gnu_plot_single_ts_data_series(&t, &y, title);
}

// pub fn gnu_plot_single_sma(data: &[f64], tick: &f64, base: usize, title: &str) {
//     let stat_data = data_to_stat_vecs(data.into(), base);
//     gnu_plot_single(&stat_data.sma, tick, title);
// }
// pub fn gnu_plot_single_spr(data: &[f64], tick: &f64, base: usize, title: &str) {
//     let stat_data = data_to_stat_vecs(data.into(), base);
//     gnu_plot_single(&stat_data.spr, tick, title);
// }


pub fn gnu_plot_v3d_series_and_stats(data: &[Vector3d], series_props: &[usize]) {
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
                Color("brown"),
            ]
        );
    }

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}

pub fn gnu_plot_stats_for_v3d_data(data: &[Vector3d], base_series: &[usize]) {
    let mut sma_magnitude_series: Vec<(Vec<f64>, Vec<f64>, String, String)> = Vec::new();
    let mut spr_magnitude_series: Vec<(Vec<f64>, Vec<f64>, String, String)> = Vec::new();

    fn base_to_label(base: usize) -> String {
        format!("{} pt", base)
    }

    for base in base_series {
        let cur_stats = v3d_list_to_magnitude_smaspr_list(data, *base);
        // let cur_base_label = base_to_label(*base);
        sma_magnitude_series.push((cur_stats.0.clone(), cur_stats.1, base_to_label(*base), "black".into()));
        spr_magnitude_series.push((cur_stats.0        , cur_stats.2, base_to_label(*base), "brown".into()));
    }

    gnu_plot_multi_ts_data(
        [sma_magnitude_series, spr_magnitude_series].concat()
    );
}




/// (ts, data, label, color)
pub fn get_lines_for_ts_data(ndata: &[(Vec<f64>, Vec<f64>, String, String)]) -> Vec<(Vec<f64>, Vec<f64>, &[PlotOption<&'static str>])> {
    let mut lines: Vec<(Vec<f64>, Vec<f64>, &'static [PlotOption<&'static str>])> = Vec::new();
    
    for data in ndata {
        let (cur_ts, cur_data, cur_label, cur_color) = data;
        let cur_label:&'static String = &cur_label.to_string();
        // Create PlotOption<&str> slice
        let plot_options: &[PlotOption<&'static str>] = &[
            Caption(&cur_label),
            Color(cur_color.as_str())
        ];
        
        lines.push((cur_ts.clone(), cur_data.clone(), plot_options));
    }
    
    lines
}



pub fn gnu_plot_multi_ts_data(multi_ts_data: Vec<(Vec<f64>, Vec<f64>, String, String)>) {
    let mut fg: Figure = Figure::new();
    let fg_2d = fg.axes2d();
    
    let lines_data = get_lines_for_ts_data(&multi_ts_data);
    
    for line_data in lines_data {
        let (x, y, opt) = line_data;
        fg_2d.lines(x, y, opt);
    }

    std::thread::spawn(move || {
        fg.show().unwrap();
    });

}