use charming::{
    component::{
        Axis, DataZoom, DataZoomType, Feature, Restore, SaveAsImage, Title, Toolbox,
        ToolboxDataZoom,
    },
    element::{AreaStyle, AxisType, Color, ColorStop, LineStyle, Symbol, Tooltip, Trigger},
    series::Line,
    Chart,
};
use chrono::{Days, NaiveDate};
use rand::Rng;

pub fn chart() -> Chart {
    let base_date = NaiveDate::parse_from_str("1968-09-03", "%Y-%m-%d").unwrap();
    let mut rng = rand::thread_rng();
    let mut dates = vec![];
    let mut values = vec![rng.gen::<f64>() * 300.];

    for i in 1..20001 {
        let date = base_date.checked_add_days(Days::new(i)).unwrap();
        dates.push(date.format("%Y/%m/%d").to_string());
        values.push((rng.gen::<f64>() - 0.5) * 20. + values[(i - 1) as usize]);
    }

    Chart::new()
        .tooltip(Tooltip::new().trigger(Trigger::Axis))
        .title(Title::new().left("center").text("Large Area Chart"))
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_zoom(ToolboxDataZoom::new().y_axis_index("none"))
                    .restore(Restore::new())
                    .save_as_image(SaveAsImage::new()),
            ),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(dates),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .data_zoom(DataZoom::new().type_(DataZoomType::Inside).start(0).end(10))
        .data_zoom(DataZoom::new().start(0).end(10))
        .series(
            Line::new()
                .name("Fake Data")
                .symbol(Symbol::None)
                .line_style(LineStyle::new().color("rgb(255, 70, 131"))
                .area_style(AreaStyle::new().color(Color::LinearGradient {
                    x: 0.,
                    y: 0.,
                    x2: 0.,
                    y2: 1.,
                    color_stops: vec![
                        ColorStop::new(0, "rgb(255, 158, 68)"),
                        ColorStop::new(1, "rgb(255, 70, 131)"),
                    ],
                }))
                .data(values),
        )
}
