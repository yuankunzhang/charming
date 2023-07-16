use charming::{
    component::{Axis, Feature, Grid, Legend, Toolbox, ToolboxDataZoom},
    datatype::{CompositeValue, Dataset},
    element::{AxisLabel, AxisType, DimensionEncode, Tooltip},
    series::Scatter,
    Chart,
};

pub fn chart() -> Chart {
    let data: Vec<Vec<CompositeValue>> =
        serde_json::from_str(include_str!("../../asset/life-expectancy-table.json")).unwrap();

    Chart::new()
        .legend(Legend::new())
        .tooltip(Tooltip::new())
        .toolbox(
            Toolbox::new()
                .left("center")
                .feature(Feature::new().data_zoom(ToolboxDataZoom::new())),
        )
        .grid(Grid::new().right("57%").bottom("57%"))
        .grid(Grid::new().left("57%").bottom("57%"))
        .grid(Grid::new().right("57%").top("57%"))
        .grid(Grid::new().left("57%").top("57%"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .grid_index(0)
                .name("Income")
                .axis_label(AxisLabel::new().rotate(50).interval(0)),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .grid_index(1)
                .name("Country")
                .boundary_gap(false)
                .axis_label(AxisLabel::new().rotate(50).interval(0)),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .grid_index(2)
                .name("Income")
                .axis_label(AxisLabel::new().rotate(50).interval(0)),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Value)
                .grid_index(3)
                .name("Life Expectancy")
                .axis_label(AxisLabel::new().rotate(50).interval(0)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .grid_index(0)
                .name("Life Expectancy"),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .grid_index(1)
                .name("Income"),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .grid_index(2)
                .name("Population"),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .grid_index(3)
                .name("Population"),
        )
        .dataset(Dataset::new().source(data))
        .series(
            Scatter::new()
                .symbol_size(2.5)
                .x_axis_index(0)
                .y_axis_index(0)
                .encode(
                    DimensionEncode::new()
                        .x("Income")
                        .y("Life Expectancy")
                        .tooltip(vec![
                            "Income",
                            "Life Expectancy",
                            "Population",
                            "Country",
                            "Year",
                        ]),
                ),
        )
        .series(
            Scatter::new()
                .symbol_size(2.5)
                .x_axis_index(1)
                .y_axis_index(1)
                .encode(
                    DimensionEncode::new()
                        .x("Country")
                        .y("Income")
                        .tooltip(vec![
                            "Income",
                            "Life Expectancy",
                            "Population",
                            "Country",
                            "Year",
                        ]),
                ),
        )
        .series(
            Scatter::new()
                .symbol_size(2.5)
                .x_axis_index(2)
                .y_axis_index(2)
                .encode(
                    DimensionEncode::new()
                        .x("Income")
                        .y("Population")
                        .tooltip(vec![
                            "Income",
                            "Life Expectancy",
                            "Population",
                            "Country",
                            "Year",
                        ]),
                ),
        )
        .series(
            Scatter::new()
                .symbol_size(2.5)
                .x_axis_index(3)
                .y_axis_index(3)
                .encode(
                    DimensionEncode::new()
                        .x("Life Expectancy")
                        .y("Population")
                        .tooltip(vec![
                            "Income",
                            "Life Expectancy",
                            "Population",
                            "Country",
                            "Year",
                        ]),
                ),
        )
}
