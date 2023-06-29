use crate::chart::series::*;

pub enum TextAlign {
    Auto,
    Left,
    Right,
    Center,
}

pub enum TextVerticalAlign {
    Auto,
    Top,
    Bottom,
    Middle,
}

pub struct Title {
    pub show: bool,
    pub text: String,
    pub link: String,
    pub target: String,
    pub subtext: String,
    pub sublink: String,
    pub subtarget: String,
    pub text_align: TextAlign,
    pub text_vertical_align: TextVerticalAlign,
    pub padding: u32,
    pub item_gap: u32,
    pub left: String,
    pub top: String,
    pub right: String,
    pub bottom: String,
    pub background_color: String,
    pub border_color: String,
    pub border_width: u32,
    pub border_radius: u32,
}

impl Default for Title {
    fn default() -> Self {
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
}

pub enum LegendStyle {
    Plain,
    Scroll,
}

pub enum LegendAlign {
    Auto,
    Left,
    Right,
}

pub struct Legend {
    pub type_: LegendStyle,
    pub show: bool,
    pub left: String,
    pub top: String,
    pub right: String,
    pub bottom: String,
    pub orient: String,
    pub align: LegendAlign,
    pub padding: u32,
    pub item_gap: u32,
    pub item_width: u32,
    pub item_height: u32,
    pub background_color: String,
    pub border_color: String,
    pub border_width: u32,
    pub border_radius: u32,
}

impl Default for Legend {
    fn default() -> Self {
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
        }
    }
}

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

impl Default for Grid {
    fn default() -> Self {
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

pub enum AxisType {
    Value,
    Category,
    Time,
    Log,
}

pub struct Axis {
    pub show: bool,
    pub grid_index: u32,
    pub offset: i32,
    pub type_: AxisType,
    pub data: Vec<String>,
}

impl Default for Axis {
    fn default() -> Self {
        Self {
            show: true,
            grid_index: 0,
            offset: 0,
            type_: AxisType::Category,
            data: vec![],
        }
    }
}

pub struct Option {
    pub title: Title,
    pub legend: Legend,
    pub grid: Grid,
    pub x_axis: Axis,
    pub y_axis: Axis,
    pub series: Box<dyn Series>,
}

impl Option {
    pub fn new() -> Self {
        Self {
            title: Title::default(),
            legend: Legend::default(),
            grid: Grid::default(),
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            series: Box::new(DummySeries {}),
        }
    }

    pub fn title(mut self, title: Title) -> Self {
        self.title = title;
        self
    }

    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = legend;
        self
    }

    pub fn x_axis(mut self, x_axis: Axis) -> Self {
        self.x_axis = x_axis;
        self
    }

    pub fn y_axis(mut self, y_axis: Axis) -> Self {
        self.y_axis = y_axis;
        self
    }
}
