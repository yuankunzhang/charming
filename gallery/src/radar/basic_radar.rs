use charming::{
    component::{Legend, RadarCoordinate, Title},
    series::Radar,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().text("Basic Radar Chart"))
        .legend(Legend::new().data(vec!["Allocated Budget", "Actual Spending"]))
        .radar(RadarCoordinate::new().indicator(vec![
            ("Sales", 0, 6500),
            ("Administration", 0, 16000),
            ("Information Technology", 0, 30000),
            ("Customer Support", 0, 38000),
            ("Development", 0, 52000),
            ("Marketing", 0, 25000),
        ]))
        .series(Radar::new().name("Budget vs spending").data(vec![
            (
                vec![4200, 3000, 20000, 35000, 50000, 18000],
                "Allocated Budget",
            ),
            (
                vec![5000, 14000, 28000, 26000, 42000, 21000],
                "Actual Spending",
            ),
        ]))
}
