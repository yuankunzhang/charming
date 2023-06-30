use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveAsImageType {
    Png,
    Jpg,
    Svg,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveAsImage {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<SaveAsImageType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<String>,
}

impl SaveAsImage {
    pub fn new() -> Self {
        Self {
            type_: None,
            name: None,
            background_color: None,
        }
    }

    pub fn type_(mut self, type_: SaveAsImageType) -> Self {
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
pub struct Restore {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataView {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<bool>,
}

impl DataView {
    pub fn new() -> Self {
        Self {
            title: None,
            read_only: None,
        }
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
pub enum MagicTypeType {
    /// For line charts.
    Line,
    /// For bar charts.
    Bar,
    /// For stacked charts.
    Stack,
}

impl From<&str> for MagicTypeType {
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
pub struct MagicType {
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<Vec<MagicTypeType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl MagicType {
    pub fn new() -> Self {
        Self {
            type_: None,
            title: None,
        }
    }

    pub fn type_(mut self, type_: Vec<MagicTypeType>) -> Self {
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
pub struct Feature {
    #[serde(skip_serializing_if = "Option::is_none")]
    save_as_image: Option<SaveAsImage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    restore: Option<Restore>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data_view: Option<DataView>,

    #[serde(skip_serializing_if = "Option::is_none")]
    magic_type: Option<MagicType>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            save_as_image: None,
            restore: None,
            data_view: None,
            magic_type: None,
        }
    }

    pub fn save_as_image(mut self, save_as_image: SaveAsImage) -> Self {
        self.save_as_image = Some(save_as_image);
        self
    }

    pub fn restore(mut self, restore: Restore) -> Self {
        self.restore = Some(restore);
        self
    }

    pub fn data_view(mut self, data_view: DataView) -> Self {
        self.data_view = Some(data_view);
        self
    }

    pub fn magic_type(mut self, magic_type: MagicType) -> Self {
        self.magic_type = Some(magic_type);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Toolbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    feature: Option<Feature>,
}

impl Toolbox {
    pub fn new() -> Self {
        Self { feature: None }
    }

    pub fn feature(mut self, feature: Feature) -> Self {
        self.feature = Some(feature);
        self
    }
}
