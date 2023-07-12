use charming::{component::Axis, series::Scatter, Chart};

pub fn chart() -> Chart {
    Chart::new().x_axis(Axis::new()).y_axis(Axis::new()).series(
        Scatter::new().symbol_size(20).data(vec![
            vec![10.0, 8.04],
            vec![8.07, 6.95],
            vec![13.0, 7.58],
            vec![9.05, 8.81],
            vec![11.0, 8.33],
            vec![14.0, 7.66],
            vec![13.4, 6.81],
            vec![10.0, 6.33],
            vec![14.0, 8.96],
            vec![12.5, 6.82],
            vec![9.15, 7.2],
            vec![11.5, 7.2],
            vec![3.03, 4.23],
            vec![12.2, 7.83],
            vec![2.02, 4.47],
            vec![1.05, 3.33],
            vec![4.05, 4.96],
            vec![6.03, 7.24],
            vec![12.0, 6.26],
            vec![12.0, 8.84],
            vec![7.08, 5.82],
            vec![5.02, 5.68],
        ]),
    )
}
