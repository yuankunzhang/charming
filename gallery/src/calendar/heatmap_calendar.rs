use charming::{
    component::{Calendar, Title, VisualMap, VisualMapType},
    datatype::DataFrame,
    element::{CoordinateSystem, ItemStyle, Orient, Tooltip},
    series::Heatmap,
    Chart,
};
use chrono::NaiveDate;
use rand::{Rng, SeedableRng};

pub fn chart() -> Chart {
    Chart::new()
        .title(Title::new().top(30).left("center").text("Daily Step Count"))
        .tooltip(Tooltip::new())
        .visual_map(
            VisualMap::new()
                .min(0)
                .max(10000)
                .type_(VisualMapType::Piecewise)
                .orient(Orient::Horizontal)
                .left("center")
                .top(65),
        )
        .calendar(
            Calendar::new()
                .top(120)
                .range(("2017-01-01", "2017-05-31"))
                .item_style(ItemStyle::new().border_width(0.5)),
        )
        .series(
            Heatmap::new()
                .coordinate_system(CoordinateSystem::Calendar)
                .data(make_data()),
        )
}

fn make_data() -> Vec<DataFrame> {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(1337);
    let naive_date = NaiveDate::from_ymd_opt(2017, 1, 1).unwrap();
    let mut data: Vec<DataFrame> = Vec::with_capacity(151);
    for day in naive_date.iter_days().take(151) {
        let value: f64 = rng.random_range(0.0..10000.0);
        data.push(vec![
            day.format("%Y-%m-%d").to_string().into(),
            value.floor().into(),
        ]);
    }
    data
}
