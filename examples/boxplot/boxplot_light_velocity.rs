use echarts::{
    component::{axis, dataset, grid, title, tooltip},
    element::{axis_type, split_area, split_line, text_style},
    series::{boxplot, scatter, Series},
    Chart,
};

fn main() {
    let data = vec![
        vec![
            850, 740, 900, 1070, 930, 850, 950, 980, 980, 880, 1000, 980, 930, 650, 760, 810, 1000,
            1000, 960, 960,
        ],
        vec![
            960, 940, 960, 940, 880, 800, 850, 880, 900, 840, 830, 790, 810, 880, 880, 830, 800,
            790, 760, 800,
        ],
        vec![
            880, 880, 880, 860, 720, 720, 620, 860, 970, 950, 880, 910, 850, 870, 840, 840, 850,
            840, 840, 840,
        ],
        vec![
            890, 810, 810, 820, 800, 770, 760, 740, 750, 760, 910, 920, 890, 860, 880, 720, 840,
            850, 850, 780,
        ],
        vec![
            890, 840, 780, 810, 760, 810, 790, 810, 820, 850, 870, 870, 810, 740, 810, 940, 950,
            800, 810, 870,
        ],
    ];

    let ds = dataset::Dataset::new()
        .source(data)
        .transform(r#"{ "type": "boxplot", "config": { "itemNameFormatter": "expr {value}" } }"#)
        .transform(
            dataset::Transform::new()
                .from_dataset_index(1)
                .from_transform_result(1),
        );

    let chart = Chart::new()
        .title(
            title::Title::new()
                .text("Michelson-Morley Experiment")
                .left("center"),
        )
        .title(
            title::Title::new()
                .text("upper: Q3 + 1.5 * IQR \nlower: Q1 - 1.5 * IQR")
                .border_color("#999")
                .border_width(1)
                .text_style(
                    text_style::TextStyle::new()
                        .font_weight("normal")
                        .font_size(14)
                        .line_height(20),
                )
                .left("10%")
                .top("90%"),
        )
        .dataset(ds)
        .tooltip(
            tooltip::Tooltip::new()
                .trigger(tooltip::Trigger::Item)
                .axis_pointer(tooltip::AxisPointer::new().type_(tooltip::AxisPointerType::Shadow)),
        )
        .grid(grid::Grid::new().left("10%").right("10%").bottom("15%"))
        .x_axis(
            axis::Axis::new()
                .type_(axis_type::AxisType::Category)
                .boundary_gap(true)
                .name_gap(30)
                .split_area(split_area::SplitArea::new().show(false))
                .split_line(split_line::SplitLine::new().show(false)),
        )
        .y_axis(
            axis::Axis::new()
                .type_(axis_type::AxisType::Value)
                .name("km/s minus 299,000")
                .split_area(split_area::SplitArea::new().show(true)),
        )
        .series(Series::Boxplot(
            boxplot::Boxplot::new().name("boxplot").dataset_index(1),
        ))
        .series(Series::Scatter(
            scatter::Scatter::new().name("outlier").dataset_index(2),
        ));
    println!("{}", chart.to_string());
}
