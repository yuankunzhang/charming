use serde::Serialize;

use crate::{datatype::CompositeValue, element::Orient};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolboxFeatureSaveAsImageType {
    Png,
    Jpg,
    Svg,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolboxFeatureSaveAsImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<ToolboxFeatureSaveAsImageType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
}

impl ToolboxFeatureSaveAsImage {
    pub fn new() -> Self {
        Self {
            show: None,
            type_: None,
            name: None,
            background_color: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn type_(mut self, type_: ToolboxFeatureSaveAsImageType) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn background_color<S: Into<String>>(mut self, background_color: S) -> Self {
        self.background_color = Some(background_color.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolboxFeatureRestore {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl ToolboxFeatureRestore {
    pub fn new() -> Self {
        Self {
            show: None,
            title: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolboxFeatureDataView {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<bool>,
}

impl ToolboxFeatureDataView {
    pub fn new() -> Self {
        Self {
            show: None,
            title: None,
            read_only: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = Some(read_only);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolboxFeatureMagicTypeType {
    /// For line charts.
    Line,
    /// For bar charts.
    Bar,
    /// For stacked charts.
    Stack,
}

impl From<&str> for ToolboxFeatureMagicTypeType {
    fn from(s: &str) -> Self {
        match s {
            "line" => Self::Line,
            "bar" => Self::Bar,
            "stack" => Self::Stack,
            _ => panic!("Invalid magic type type: {}", s),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolboxFeatureMagicType {
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<Vec<ToolboxFeatureMagicTypeType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl ToolboxFeatureMagicType {
    pub fn new() -> Self {
        Self {
            type_: None,
            title: None,
        }
    }

    pub fn type_(mut self, type_: Vec<ToolboxFeatureMagicTypeType>) -> Self {
        self.type_ = Some(type_);
        self
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ToolboxFeatureBrushType {
    Rect,
    Polygon,
    LineX,
    LineY,
    Keep,
    Clear,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolboxFeatureBrush {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    type_: Vec<ToolboxFeatureBrushType>,
}

impl ToolboxFeatureBrush {
    pub fn new() -> Self {
        Self { type_: vec![] }
    }

    pub fn type_(mut self, type_: Vec<ToolboxFeatureBrushType>) -> Self {
        self.type_ = type_;
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolboxFeatureDataZoom {
    #[serde(skip_serializing_if = "Option::is_none")]
    y_axis_index: Option<bool>,
}

impl ToolboxFeatureDataZoom {
    pub fn new() -> Self {
        Self { y_axis_index: None }
    }

    pub fn y_axis_index(mut self, y_axis_index: bool) -> Self {
        self.y_axis_index = Some(y_axis_index);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolboxFeature {
    #[serde(skip_serializing_if = "Option::is_none")]
    save_as_image: Option<ToolboxFeatureSaveAsImage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    restore: Option<ToolboxFeatureRestore>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data_view: Option<ToolboxFeatureDataView>,

    #[serde(skip_serializing_if = "Option::is_none")]
    magic_type: Option<ToolboxFeatureMagicType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data_zoom: Option<ToolboxFeatureDataZoom>,

    #[serde(skip_serializing_if = "Option::is_none")]
    brush: Option<ToolboxFeatureBrush>,
}

impl ToolboxFeature {
    pub fn new() -> Self {
        Self {
            save_as_image: None,
            restore: None,
            data_view: None,
            magic_type: None,
            data_zoom: None,
            brush: None,
        }
    }

    pub fn save_as_image(mut self, save_as_image: ToolboxFeatureSaveAsImage) -> Self {
        self.save_as_image = Some(save_as_image);
        self
    }

    pub fn restore(mut self, restore: ToolboxFeatureRestore) -> Self {
        self.restore = Some(restore);
        self
    }

    pub fn data_view(mut self, data_view: ToolboxFeatureDataView) -> Self {
        self.data_view = Some(data_view);
        self
    }

    pub fn magic_type(mut self, magic_type: ToolboxFeatureMagicType) -> Self {
        self.magic_type = Some(magic_type);
        self
    }

    pub fn data_zoom(mut self, data_zoom: ToolboxFeatureDataZoom) -> Self {
        self.data_zoom = Some(data_zoom);
        self
    }

    pub fn brush(mut self, brush: ToolboxFeatureBrush) -> Self {
        self.brush = Some(brush);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Toolbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    feature: Option<ToolboxFeature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    orient: Option<Orient>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom: Option<CompositeValue>,
}

impl Toolbox {
    pub fn new() -> Self {
        Self {
            show: None,
            feature: None,
            orient: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn feature<T: Into<ToolboxFeature>>(mut self, feature: T) -> Self {
        self.feature = Some(feature.into());
        self
    }

    pub fn orient<O: Into<Orient>>(mut self, orient: O) -> Self {
        self.orient = Some(orient.into());
        self
    }

    pub fn left<C: Into<CompositeValue>>(mut self, left: C) -> Self {
        self.left = Some(left.into());
        self
    }

    pub fn top<C: Into<CompositeValue>>(mut self, top: C) -> Self {
        self.top = Some(top.into());
        self
    }

    pub fn right<C: Into<CompositeValue>>(mut self, right: C) -> Self {
        self.right = Some(right.into());
        self
    }

    pub fn bottom<C: Into<CompositeValue>>(mut self, bottom: C) -> Self {
        self.bottom = Some(bottom.into());
        self
    }
}
