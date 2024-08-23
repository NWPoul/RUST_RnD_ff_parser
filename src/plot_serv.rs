

use gnuplot::{
    Figure,
    Caption,
    Color,
};


use crate::utils::u_serv::Vector3d;
use crate::analise::{parser_data_to_sma_list, parser_data_to_t_sma_xyz_list};

use crate::PLOT_RAW;



pub fn format_data_for_plot(data: &[f64]) -> (Vec<f64>, Vec<f64>) {
    let mut t: Vec<f64> = vec![0.0];
    let mut y = Vec::new();

    for (i, tdata) in data.iter().enumerate() {
        t.push(i as f64 * 0.005);
        y.push(*tdata);
    }
    (t, y)
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
            let x_data = &smaxyz_data.1.iter().map(|vector| vector.x).collect::<Vec<f64>>();
            let y_data = &smaxyz_data.1.iter().map(|vector| vector.y).collect::<Vec<f64>>();
            let z_data = &smaxyz_data.1.iter().map(|vector| vector.z).collect::<Vec<f64>>();
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