use serde::Serialize;

use super::series::Series;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TextVerticalAlign {
    Auto,
    Top,
    Bottom,
    Middle,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    show: bool,
    text: String,
    link: String,
    target: String,
    subtext: String,
    sublink: String,
    subtarget: String,
    text_align: TextAlign,
    text_vertical_align: TextVerticalAlign,
    padding: u32,
    item_gap: u32,
    left: String,
    top: String,
    right: String,
    bottom: String,
    background_color: String,
    border_color: String,
    border_width: u32,
    border_radius: u32,
}

impl Title {
    pub fn new() -> Self {
        Self {
            show: true,
            text: "".to_string(),
            link: "".to_string(),
            target: "blank".to_string(),
            subtext: "".to_string(),
            sublink: "".to_string(),
            subtarget: "blank".to_string(),
            text_align: TextAlign::Auto,
            text_vertical_align: TextVerticalAlign::Auto,
            padding: 5,
            item_gap: 10,
            left: "auto".to_string(),
            top: "auto".to_string(),
            right: "auto".to_string(),
            bottom: "auto".to_string(),
            background_color: "transparent".to_string(),
            border_color: "#ccc".to_string(),
            border_width: 1,
            border_radius: 0,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LegendStyle {
    Plain,
    Scroll,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LegendAlign {
    Auto,
    Left,
    Right,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Legend {
    #[serde(rename = "type")]
    type_: LegendStyle,
    show: bool,
    left: String,
    top: String,
    right: String,
    bottom: String,
    orient: String,
    align: LegendAlign,
    padding: u32,
    item_gap: u32,
    item_width: u32,
    item_height: u32,
    background_color: String,
    border_color: String,
    border_width: u32,
    border_radius: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<String>>,
}

impl Legend {
    pub fn new() -> Self {
        Self {
            type_: LegendStyle::Plain,
            show: true,
            left: "auto".to_string(),
            top: "auto".to_string(),
            right: "auto".to_string(),
            bottom: "auto".to_string(),
            orient: "horizontal".to_string(),
            align: LegendAlign::Auto,
            padding: 5,
            item_gap: 10,
            item_width: 25,
            item_height: 14,
            background_color: "transparent".to_string(),
            border_color: "#ccc".to_string(),
            border_width: 1,
            border_radius: 0,
            data: None,
        }
    }

    pub fn data(mut self, data: Vec<String>) -> Self {
        self.data = Some(data);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
    pub show: bool,
    pub left: String,
    pub top: String,
    pub right: String,
    pub bottom: String,
    pub contain_label: bool,
    pub background_color: String,
    pub border_color: String,
    pub border_width: u32,
    pub border_radius: u32,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            show: false,
            left: "10%".to_string(),
            top: "60".to_string(),
            right: "10%".to_string(),
            bottom: "60".to_string(),
            contain_label: false,
            background_color: "transparent".to_string(),
            border_color: "#ccc".to_string(),
            border_width: 1,
            border_radius: 0,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisType {
    Value,
    Category,
    Time,
    Log,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Axis {
    show: bool,
    grid_index: u32,
    offset: i32,
    #[serde(rename = "type")]
    type_: AxisType,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<String>>,
}

impl Axis {
    pub fn new() -> Self {
        Self {
            show: true,
            grid_index: 0,
            offset: 0,
            type_: AxisType::Category,
            data: None,
        }
    }

    pub fn type_(mut self, type_: AxisType) -> Self {
        self.type_ = type_;
        self
    }

    pub fn data(mut self, data: Vec<String>) -> Self {
        self.data = Some(data);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    legend: Option<Legend>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grid: Option<Grid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x_axis: Option<Axis>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis: Option<Axis>,
    series: Vec<Series>,
}

impl ChartOption {
    pub fn new() -> Self {
        Self {
            title: None,
            legend: None,
            grid: None,
            x_axis: None,
            y_axis: None,
            series: vec![],
        }
    }

    pub fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }

    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = Some(legend);
        self
    }

    pub fn grid(mut self, grid: Grid) -> Self {
        self.grid = Some(grid);
        self
    }

    pub fn x_axis(mut self, x_axis: Axis) -> Self {
        self.x_axis = Some(x_axis);
        self
    }

    pub fn y_axis(mut self, y_axis: Axis) -> Self {
        self.y_axis = Some(y_axis);
        self
    }

    pub fn series(mut self, series: Series) -> Self {
        self.series.push(series);
        self
    }
}
