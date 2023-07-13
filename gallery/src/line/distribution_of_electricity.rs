use charming::{
    component::{Axis, Feature, SaveAsImage, Title, Toolbox, VisualMap, VisualMapPiece},
    element::{
        AxisLabel, AxisPointer, AxisPointerType, AxisType, ItemStyle, MarkArea, MarkAreaData,
        Tooltip, Trigger,
    },
    series::Line,
    Chart,
};

pub fn chart() -> Chart {
    Chart::new()
        .title(
            Title::new()
                .text("Distribution of Electricity")
                .subtext("Fake Data"),
        )
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Cross)),
        )
        .toolbox(
            Toolbox::new()
                .show(true)
                .feature(Feature::new().save_as_image(SaveAsImage::new().show(true))),
        )
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .boundary_gap(false)
                .data(vec![
                    "00:00", "01:15", "02:30", "03:45", "05:00", "06:15", "07:30", "08:45",
                    "10:00", "11:15", "12:30", "13:45", "15:00", "16:15", "17:30", "18:45",
                    "20:00", "21:15", "22:30", "23:45",
                ]),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Value)
                .axis_label(AxisLabel::new().formatter("{value} W"))
                .axis_pointer(AxisPointer::new().snap(true)),
        )
        .visual_map(VisualMap::new().show(false).dimension(0).pieces(vec![
            VisualMapPiece::new().lte(6).color("green"),
            VisualMapPiece::new().gt(6).lte(8).color("red"),
            VisualMapPiece::new().gt(8).lte(14).color("green"),
            VisualMapPiece::new().gt(14).lte(17).color("red"),
            VisualMapPiece::new().gt(17).color("green"),
        ]))
        .series(
            Line::new()
                .name("Electricity")
                .smooth(0.5)
                .data(vec![
                    300, 280, 250, 260, 270, 300, 550, 500, 400, 390, 380, 390, 400, 500, 600, 750,
                    800, 700, 600, 400,
                ])
                .mark_area(
                    MarkArea::new()
                        .item_style(ItemStyle::new().color("rgba(255, 173, 177, 0.4)"))
                        .data(vec![
                            (
                                MarkAreaData::new().name("Morning Peak").x_axis("07:30"),
                                MarkAreaData::new().x_axis("10:00"),
                            ),
                            (
                                MarkAreaData::new().name("Evening Peak").x_axis("17:30"),
                                MarkAreaData::new().x_axis("21:15"),
                            ),
                        ]),
                ),
        )
}
