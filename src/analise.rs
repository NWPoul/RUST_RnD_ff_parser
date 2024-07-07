use gnuplot::{
    Figure,
    // Caption,
    Color,
};



pub fn parser_data_to_sma_list(data: &[(f64, f64, f64)], _base: usize) -> (Vec<f64>, Vec<f64>) {
    let n_base = *crate::SMA_BASE.lock().unwrap();
    let mut sma_vec = vec![0.];
    let mut sma_t = vec![0.];

    for i in n_base..data.len() {
        sma_t.push(i as f64 * 0.005);
        let cur_data = &data[i - n_base..i];
        let cur_sma_x: f64 = cur_data.iter().map(|(x, _, _)| x).sum();
        let cur_sma_y: f64 = cur_data.iter().map(|(_, y, _)| y).sum();
        let cur_sma_z: f64 = cur_data.iter().map(|(_, _, z)| z).sum();

        sma_vec.push(
            f64::sqrt(cur_sma_x.powi(2) + cur_sma_y.powi(2) + cur_sma_z.powi(2)) / n_base as f64,
        );
    }

    (sma_t, sma_vec)
}

pub fn get_max_vec_data(data: &[f64]) -> (f64, f64) {
    let (max_i, max_vec) = data
        .iter()
        .enumerate()
        .max_by(
            |prev, next| 
                prev.1
                    .partial_cmp(next.1)
                    .unwrap_or(std::cmp::Ordering::Greater)
        )
        .unwrap_or((0,&0.));
    ((max_i as f64 * 0.005).round(), *max_vec)
}








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
    // let t: Vec<f64> = (0..data.len()).map(
    //     |i| i as f64 * 0.005
    // ).collect();
    
    let mut sma_series: Vec<(Vec<f64>,Vec<f64>)> = Vec::new(); 
    
    for base in base_series {
        let cur_sma = parser_data_to_sma_list(data, *base);
        sma_series.push(cur_sma);
    }

    let mut fg = Figure::new();
    for sma_data in sma_series.iter() {
        fg.axes2d()
           .lines(&sma_data.0, &sma_data.1, &[Color("black")]);
    }

    std::thread::spawn(move || {
        fg.show().unwrap();
    });
}