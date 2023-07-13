use charming::{
    component::{Axis, Grid, VisualMap},
    datatype::{CompositeValue, DataFrame},
    df,
    element::{AxisType, Emphasis, ItemStyle, Label, Orient, SplitArea, Tooltip},
    series::Heatmap,
    Chart,
};

pub fn chart() -> Chart {
    let data = vec![
        vec![0, 0, 5],
        vec![0, 1, 1],
        vec![0, 2, 0],
        vec![0, 3, 0],
        vec![0, 4, 0],
        vec![0, 5, 0],
        vec![0, 6, 0],
        vec![0, 7, 0],
        vec![0, 8, 0],
        vec![0, 9, 0],
        vec![0, 10, 0],
        vec![0, 11, 2],
        vec![0, 12, 4],
        vec![0, 13, 1],
        vec![0, 14, 1],
        vec![0, 15, 3],
        vec![0, 16, 4],
        vec![0, 17, 6],
        vec![0, 18, 4],
        vec![0, 19, 4],
        vec![0, 20, 3],
        vec![0, 21, 3],
        vec![0, 22, 2],
        vec![0, 23, 5],
        vec![1, 0, 7],
        vec![1, 1, 0],
        vec![1, 2, 0],
        vec![1, 3, 0],
        vec![1, 4, 0],
        vec![1, 5, 0],
        vec![1, 6, 0],
        vec![1, 7, 0],
        vec![1, 8, 0],
        vec![1, 9, 0],
        vec![1, 10, 5],
        vec![1, 11, 2],
        vec![1, 12, 2],
        vec![1, 13, 6],
        vec![1, 14, 9],
        vec![1, 15, 11],
        vec![1, 16, 6],
        vec![1, 17, 7],
        vec![1, 18, 8],
        vec![1, 19, 12],
        vec![1, 20, 5],
        vec![1, 21, 5],
        vec![1, 22, 7],
        vec![1, 23, 2],
        vec![2, 0, 1],
        vec![2, 1, 1],
        vec![2, 2, 0],
        vec![2, 3, 0],
        vec![2, 4, 0],
        vec![2, 5, 0],
        vec![2, 6, 0],
        vec![2, 7, 0],
        vec![2, 8, 0],
        vec![2, 9, 0],
        vec![2, 10, 3],
        vec![2, 11, 2],
        vec![2, 12, 1],
        vec![2, 13, 9],
        vec![2, 14, 8],
        vec![2, 15, 10],
        vec![2, 16, 6],
        vec![2, 17, 5],
        vec![2, 18, 5],
        vec![2, 19, 5],
        vec![2, 20, 7],
        vec![2, 21, 4],
        vec![2, 22, 2],
        vec![2, 23, 4],
        vec![3, 0, 7],
        vec![3, 1, 3],
        vec![3, 2, 0],
        vec![3, 3, 0],
        vec![3, 4, 0],
        vec![3, 5, 0],
        vec![3, 6, 0],
        vec![3, 7, 0],
        vec![3, 8, 1],
        vec![3, 9, 0],
        vec![3, 10, 5],
        vec![3, 11, 4],
        vec![3, 12, 7],
        vec![3, 13, 14],
        vec![3, 14, 13],
        vec![3, 15, 12],
        vec![3, 16, 9],
        vec![3, 17, 5],
        vec![3, 18, 5],
        vec![3, 19, 10],
        vec![3, 20, 6],
        vec![3, 21, 4],
        vec![3, 22, 4],
        vec![3, 23, 1],
        vec![4, 0, 1],
        vec![4, 1, 3],
        vec![4, 2, 0],
        vec![4, 3, 0],
        vec![4, 4, 0],
        vec![4, 5, 1],
        vec![4, 6, 0],
        vec![4, 7, 0],
        vec![4, 8, 0],
        vec![4, 9, 2],
        vec![4, 10, 4],
        vec![4, 11, 4],
        vec![4, 12, 2],
        vec![4, 13, 4],
        vec![4, 14, 4],
        vec![4, 15, 14],
        vec![4, 16, 12],
        vec![4, 17, 1],
        vec![4, 18, 8],
        vec![4, 19, 5],
        vec![4, 20, 3],
        vec![4, 21, 7],
        vec![4, 22, 3],
        vec![4, 23, 0],
        vec![5, 0, 2],
        vec![5, 1, 1],
        vec![5, 2, 0],
        vec![5, 3, 3],
        vec![5, 4, 0],
        vec![5, 5, 0],
        vec![5, 6, 0],
        vec![5, 7, 0],
        vec![5, 8, 2],
        vec![5, 9, 0],
        vec![5, 10, 4],
        vec![5, 11, 1],
        vec![5, 12, 5],
        vec![5, 13, 10],
        vec![5, 14, 5],
        vec![5, 15, 7],
        vec![5, 16, 11],
        vec![5, 17, 6],
        vec![5, 18, 0],
        vec![5, 19, 5],
        vec![5, 20, 3],
        vec![5, 21, 4],
        vec![5, 22, 2],
        vec![5, 23, 0],
        vec![6, 0, 1],
        vec![6, 1, 0],
        vec![6, 2, 0],
        vec![6, 3, 0],
        vec![6, 4, 0],
        vec![6, 5, 0],
        vec![6, 6, 0],
        vec![6, 7, 0],
        vec![6, 8, 0],
        vec![6, 9, 0],
        vec![6, 10, 1],
        vec![6, 11, 0],
        vec![6, 12, 2],
        vec![6, 13, 1],
        vec![6, 14, 3],
        vec![6, 15, 4],
        vec![6, 16, 0],
        vec![6, 17, 0],
        vec![6, 18, 0],
        vec![6, 19, 0],
        vec![6, 20, 1],
        vec![6, 21, 2],
        vec![6, 22, 2],
        vec![6, 23, 6],
    ];
    let data: Vec<DataFrame> = data
        .into_iter()
        .map(|d| {
            df![
                d[1],
                d[0],
                if d[2] == 0 {
                    CompositeValue::from("-")
                } else {
                    CompositeValue::from(d[2])
                }
            ]
        })
        .collect();

    Chart::new()
        .tooltip(Tooltip::new().position("top"))
        .grid(Grid::new().height("50%").top("10%"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec![
                    "12a", "1a", "2a", "3a", "4a", "5a", "6a", "7a", "8a", "9a", "10a", "11a",
                    "12p", "1p", "2p", "3p", "4p", "5p", "6p", "7p", "8p", "9p", "10p", "11p",
                ])
                .split_area(SplitArea::new().show(true)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec![
                    "Saturday",
                    "Friday",
                    "Thursday",
                    "Wednesday",
                    "Tuesday",
                    "Monday",
                    "Sunday",
                ])
                .split_area(SplitArea::new().show(true)),
        )
        .visual_map(
            VisualMap::new()
                .min(0)
                .max(10)
                .calculable(true)
                .orient(Orient::Horizontal)
                .left("center")
                .bottom("15%"),
        )
        .series(
            Heatmap::new()
                .name("Punch Card")
                .label(Label::new().show(true))
                .emphasis(
                    Emphasis::new().item_style(
                        ItemStyle::new()
                            .shadow_blur(10)
                            .shadow_color("rgba(0, 0, 0, 0.5)"),
                    ),
                )
                .data(data),
        )
}
