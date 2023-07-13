use charming::{
    component::{
        Axis, Brush, BrushType, DataZoom, DataZoomType, Feature, Grid, Legend, Toolbox,
        ToolboxDataZoom,
    },
    element::{
        AxisLine, AxisPointer, AxisPointerLink, AxisPointerType, AxisType, DimensionEncode,
        SplitArea, SplitLine, Tooltip, Trigger,
    },
    series::Custom,
    Chart,
};

pub fn chart() -> Chart {
    let raw_data: Vec<(String, f64, f64, f64, f64, f64)> =
        serde_json::from_str(include_str!("../../asset/stock-dji.json")).unwrap();
    let category_data = raw_data
        .iter()
        .map(|(x, _, _, _, _, _)| x)
        .collect::<Vec<_>>();
    let data = raw_data
        .iter()
        .enumerate()
        .map(|(i, v)| vec![i as f64, v.1, v.2, v.3, v.4])
        .collect::<Vec<_>>();

    Chart::new()
        .legend(
            Legend::new()
                .bottom(10)
                .left("center")
                .data(vec!["Dow-Jones index"]),
        )
        .tooltip(
            Tooltip::new()
                .trigger(Trigger::Axis)
                .axis_pointer(AxisPointer::new().type_(AxisPointerType::Cross)),
        )
        .axis_pointer(AxisPointer::new().link(vec![AxisPointerLink::new().x_axis_index("all")]))
        .toolbox(
            Toolbox::new().feature(
                Feature::new()
                    .data_zoom(ToolboxDataZoom::new().y_axis_index("none"))
                    .brush(Brush::new().type_(vec![BrushType::LineX, BrushType::Clear])),
            ),
        )
        .grid(Grid::new().left("10%").right("8%").bottom(150))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(category_data)
                .boundary_gap(false)
                .axis_line(AxisLine::new().on_zero(false))
                .split_line(SplitLine::new().show(false))
                .min("dataMin")
                .max("dataMax")
                .axis_pointer(AxisPointer::new().z(100)),
        )
        .y_axis(
            Axis::new()
                .scale(true)
                .split_area(SplitArea::new().show(true)),
        )
        .data_zoom(
            DataZoom::new()
                .type_(DataZoomType::Inside)
                .start(98)
                .end(100)
                .min_value_span(10),
        )
        .data_zoom(
            DataZoom::new()
                .type_(DataZoomType::Slider)
                .bottom(60)
                .start(98)
                .start(98)
                .end(100)
                .min_value_span(10),
        )
        .series(
            Custom::new()
                .name("Dow-Jones index")
                .dimensions(vec!["-", "open", "close", "lowest", "highest"])
                .encode(
                    DimensionEncode::new()
                        .x(0)
                        .y(vec![1, 2, 3, 4])
                        .tooltip(vec![1, 2, 3, 4]),
                )
                .render_item(RENDER_ITEM)
                .data(data),
        )
}

static RENDER_ITEM: &str = r#"
function (params, api) {
  var xValue = api.value(0);
  var openPoint = api.coord([xValue, api.value(1)]);
  var closePoint = api.coord([xValue, api.value(2)]);
  var lowPoint = api.coord([xValue, api.value(3)]);
  var highPoint = api.coord([xValue, api.value(4)]);
  var halfWidth = api.size([1, 0])[0] * 0.35;
  var style = api.style({
    stroke: api.visual('color')
  });
  return {
    type: 'group',
    children: [
      {
        type: 'line',
        shape: {
          x1: lowPoint[0],
          y1: lowPoint[1],
          x2: highPoint[0],
          y2: highPoint[1]
        },
        style: style
      },
      {
        type: 'line',
        shape: {
          x1: openPoint[0],
          y1: openPoint[1],
          x2: openPoint[0] - halfWidth,
          y2: openPoint[1]
        },
        style: style
      },
      {
        type: 'line',
        shape: {
          x1: closePoint[0],
          y1: closePoint[1],
          x2: closePoint[0] + halfWidth,
          y2: closePoint[1]
        },
        style: style
      }
    ]
  };
}
"#;
