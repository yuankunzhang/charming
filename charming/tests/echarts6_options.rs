use charming::{
    Chart,
    component::{
        Aria, AriaData, AriaLabel, Axis, Calendar, Grid, Matrix, MatrixCell, MatrixCells,
        MatrixDimension, MatrixDimensionCell, MatrixDimensionLevel, Toolbox,
    },
    datatype::DataPointItem,
    element::{
        AxisLabel, AxisType, CoordinateSystem, CoordinateSystemUsage, Cursor, Formatter,
        JsFunction, TimeFormatterEntry, TimeFormatterValue, TimeUnit,
    },
    series::{
        Candlestick, Chord, ChordEdge, ChordNode, Custom, Line, LineEventTarget, Lines, Map,
        MapData, Treemap, TreemapNode,
    },
};
use serde_json::{Value, json};
use std::collections::BTreeMap;

fn roundtrip(chart: Chart) -> Value {
    let json = serde_json::to_value(&chart).unwrap();
    let restored: Chart = serde_json::from_value(json.clone()).unwrap();
    assert_eq!(serde_json::to_value(&restored).unwrap(), json);
    // Exercise the renderer's typed source walk as well as ordinary JSON serde.
    restored.to_javascript().unwrap();
    json
}

#[test]
fn matrix_hierarchy_cells_and_calendar_placement_roundtrip() {
    let json = roundtrip(
        Chart::new()
            .calendar(Calendar::new().id("days").range("2026-11"))
            .calendar(Calendar::new().range("2026-12"))
            .matrix(
                Matrix::new()
                    .id("table")
                    .trigger_event(true)
                    .x(MatrixDimension::new()
                        .data(vec![MatrixDimensionCell::new().value("Group").children(
                            vec![
                                MatrixDimensionCell::from("A"),
                                MatrixDimensionCell::from("B"),
                            ],
                        )])
                        .levels(vec![Some(MatrixDimensionLevel::new().level_size(30))]))
                    .y(MatrixDimension::new().show(false).length(2usize))
                    .body(MatrixCells::new().data(vec![
                            MatrixCell::new()
                                .coord(vec![vec![0, 1], vec![0, 1]])
                                .merge_cells(true)
                                .value("Merged"),
                        ])),
            )
            .grid(
                Grid::new()
                    .coordinate_system(CoordinateSystem::Matrix)
                    .coordinate_system_usage(CoordinateSystemUsage::Box)
                    .matrix_id("table")
                    .coord(vec![0, 0]),
            )
            .toolbox(
                Toolbox::new()
                    .coordinate_system(CoordinateSystem::Calendar)
                    .calendar_id("days")
                    .coord("2026-11-04"),
            ),
    );
    assert_eq!(json["calendar"].as_array().unwrap().len(), 2);
    assert_eq!(
        json["matrix"][0]["x"]["data"][0]["children"][1]["value"],
        "B"
    );
    assert_eq!(
        json["matrix"][0]["body"]["data"][0]["coord"],
        json!([[0, 1], [0, 1]])
    );
    assert_eq!(json["grid"][0]["coordinateSystemUsage"], "box");
    assert_eq!(json["toolbox"]["calendarId"], "days");
}

#[test]
fn named_custom_and_explicit_function_keep_distinct_json_semantics() {
    let callback = JsFunction::from("function(){ return {type:'circle'}; }");
    let payload = json!({"text":"function(){ globalThis.noExecution = true; }", "nested":{"type":"function","source":"ordinary data"}});
    let json = roundtrip(
        Chart::new()
            .series(
                Custom::new()
                    .render_item("violin")
                    .item_payload(payload.as_object().unwrap().clone()),
            )
            .series(Custom::new().render_item(callback.clone()))
            .series(Custom::new().render_item("function(){ this is a name }")),
    );
    assert_eq!(json["series"][0]["renderItem"], "violin");
    assert_eq!(json["series"][0]["itemPayload"], payload);
    assert_eq!(
        json["series"][1]["renderItem"],
        serde_json::to_value(callback).unwrap()
    );
    assert_eq!(
        json["series"][2]["renderItem"],
        "function(){ this is a name }"
    );
}

#[test]
fn series_dispatch_time_dictionary_and_accessibility_preserve_new_branches() {
    let formatter = Formatter::from(BTreeMap::from([
        (TimeUnit::Year, TimeFormatterEntry::from("{yyyy}")),
        (
            TimeUnit::Day,
            TimeFormatterEntry::Upper(BTreeMap::from([(
                TimeUnit::Month,
                TimeFormatterValue::Levels(vec!["{MMM} {dd}".into(), "{dd}".into()]),
            )])),
        ),
    ]));
    let json = roundtrip(
        Chart::new()
            .x_axis(
                Axis::new()
                    .type_(AxisType::Time)
                    .axis_label(AxisLabel::new().formatter(formatter)),
            )
            .aria(
                Aria::new().label(
                    AriaLabel::new()
                        .enabled(true)
                        .data(AriaData::new().exclude_dimension_id(vec![1usize])),
                ),
            )
            .series(
                Chord::new()
                    .data(vec![ChordNode::new().name("A"), ChordNode::new().name("B")])
                    .links(vec![ChordEdge::new().source("A").target("B").value(2)]),
            )
            .series(Lines::new().data(vec![vec![vec![0.0, 0.0], vec![1.0, 1.0]]]))
            .series(
                Map::new()
                    .map("map")
                    .data(vec![MapData::new().name("Region").value(1).silent(true)]),
            )
            .series(Line::new().trigger_event(LineEventTarget::Line))
            .series(Line::new().trigger_event(LineEventTarget::Area))
            .series(Line::new().trigger_event(true))
            .series(Candlestick::new().data(vec![
                DataPointItem::new(vec![1, 2, 0, 3]).cursor(Cursor::Crosshair),
            ]))
            .series(
                Treemap::new().data(vec![TreemapNode::new().name("Root").children(vec![
            TreemapNode::new().name("Leaf").value(2).cursor(Cursor::Pointer)])]),
            ),
    );
    assert_eq!(
        json["xAxis"]["axisLabel"]["formatter"]["day"]["month"],
        json!(["{MMM} {dd}", "{dd}"])
    );
    assert_eq!(
        json["aria"]["label"]["data"]["excludeDimensionId"],
        json!([1])
    );
    assert_eq!(json["series"][0]["type"], "chord");
    assert_eq!(
        json["series"][1]["data"][0]["coords"],
        json!([[0.0, 0.0], [1.0, 1.0]])
    );
    assert_eq!(json["series"][2]["data"][0]["silent"], true);
    assert_eq!(json["series"][3]["triggerEvent"], "line");
    assert_eq!(json["series"][4]["triggerEvent"], "area");
    assert_eq!(json["series"][5]["triggerEvent"], true);
    assert_eq!(json["series"][6]["data"][0]["cursor"], "crosshair");
    assert_eq!(
        json["series"][7]["data"][0]["children"][0]["cursor"],
        "pointer"
    );
}
