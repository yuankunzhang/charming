use charming::{
    component::{Axis, Grid, Title},
    datatype::{DataFrame, DataPoint, DataPointItem},
    element::{
        font_settings::FontWeight, AxisPointer, AxisPointerType, AxisType, ItemStyle, SplitArea,
        SplitLine, TextStyle, Tooltip, Trigger,
    },
    series::{Boxplot, Scatter},
    Chart,
};

pub fn chart() -> Chart {
    let data: DataFrame = vec![
        vec![655, 850, 940, 980, 1175].into(),
        vec![672.5, 800.0, 845.0, 885.0, 1012.5].into(),
        vec![780, 840, 855, 880, 940].into(),
        vec![621.25, 767.5, 815.0, 865.0, 1011.25].into(),
        DataPoint::Item(
            DataPointItem::new(vec![713.75, 807.5, 830.0, 870.0, 963.75])
                .item_style(ItemStyle::new().border_color("red")),
        ),
    ];
    let outlier = vec![vec![2, 960], vec![2, 980], vec![2, 750], vec![4, 980]];
    Chart::new()
        .title(
            Title::new()
                .text("Michelson-Morley Experiment")
                .left("center"),
        )
        .title(
            Title::new()
                .text("upper: Q3 + 1.5 * IQR \nlower: Q1 - 1.5 * IQR")
                .border_color("#999")
                .border_width(1)
                .text_style(
                    TextStyle::new()
                        .font_weight(FontWeight::Normal)
                        .font_size(14)
                        .line_height(20),
                )
                .left("10%")
                .top("90%"),
        )
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Item)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Shadow)),
        )
        .grid(Grid::new().left("10%").right("10%").bottom("15%"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(true)
                .name_gap(30)
                .split_area(SplitArea::new().show(false))
                .split_line(SplitLine::new().show(false)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .name("km/s minus 299,000")
                .split_area(SplitArea::new().show(true)),
        )
        .series(Boxplot::new().name("boxplot").data(data))
        .series(Scatter::new().name("outlier").data(outlier))
}
