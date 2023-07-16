use serde::Serialize;

use crate::{
    datatype::CompositeValue,
    element::{Color, Orient, TextStyle},
};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualMapType {
    Continuous,
    Piecewise,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualMapPiece {
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lt: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lte: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gt: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gte: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
}

impl VisualMapPiece {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            lt: None,
            lte: None,
            gt: None,
            gte: None,
            label: None,
            color: None,
        }
    }

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn lt<F: Into<f64>>(mut self, lt: F) -> Self {
        self.lt = Some(lt.into());
        self
    }

    pub fn lte<F: Into<f64>>(mut self, lte: F) -> Self {
        self.lte = Some(lte.into());
        self
    }

    pub fn gt<F: Into<f64>>(mut self, gt: F) -> Self {
        self.gt = Some(gt.into());
        self
    }

    pub fn gte<F: Into<f64>>(mut self, gte: F) -> Self {
        self.gte = Some(gte.into());
        self
    }

    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: C) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl From<(f64, f64)> for VisualMapPiece {
    fn from((min, max): (f64, f64)) -> Self {
        Self::new().min(min).max(max)
    }
}

impl From<(i64, i64)> for VisualMapPiece {
    fn from((min, max): (i64, i64)) -> Self {
        Self::new().min(min as f64).max(max as f64)
    }
}

impl From<(f64, f64, &str)> for VisualMapPiece {
    fn from((min, max, label): (f64, f64, &str)) -> Self {
        Self::new().min(min).max(max).label(label)
    }
}

impl From<(i64, i64, &str)> for VisualMapPiece {
    fn from((min, max, label): (i64, i64, &str)) -> Self {
        Self::new().min(min as f64).max(max as f64).label(label)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualMapChannel {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    color: Vec<Color>,
}

impl VisualMapChannel {
    pub fn new() -> Self {
        Self { color: vec![] }
    }

    pub fn color<C: Into<Color>>(mut self, color: Vec<C>) -> Self {
        self.color = color.into_iter().map(|c| c.into()).collect();
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    type_: Option<VisualMapType>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    color: Vec<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dimension: Option<CompositeValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    series_index: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<f64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    categories: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calculable: Option<bool>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    text_style: Option<TextStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<(f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    realtime: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    inverse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    precision: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_width: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    item_height: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    in_range: Option<VisualMapChannel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    out_range: Option<VisualMapChannel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pieces: Option<Vec<VisualMapPiece>>,
}

impl VisualMap {
    pub fn new() -> Self {
        Self {
            type_: None,
            color: vec![],
            show: None,
            dimension: None,
            series_index: None,
            min: None,
            max: None,
            categories: vec![],
            calculable: None,
            orient: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            text_style: None,
            range: None,
            realtime: None,
            inverse: None,
            precision: None,
            item_width: None,
            item_height: None,
            in_range: None,
            out_range: None,
            pieces: None,
        }
    }

    pub fn type_<S: Into<VisualMapType>>(mut self, type_: S) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn color<C: Into<Color>>(mut self, color: Vec<C>) -> Self {
        self.color = color.into_iter().map(|c| c.into()).collect();
        self
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn dimension<C: Into<CompositeValue>>(mut self, dimension: C) -> Self {
        self.dimension = Some(dimension.into());
        self
    }

    pub fn series_index<F: Into<f64>>(mut self, series_index: F) -> Self {
        self.series_index = Some(series_index.into());
        self
    }

    pub fn min<F: Into<f64>>(mut self, min: F) -> Self {
        self.min = Some(min.into());
        self
    }

    pub fn max<F: Into<f64>>(mut self, max: F) -> Self {
        self.max = Some(max.into());
        self
    }

    pub fn categories<S: Into<String>>(mut self, categories: Vec<S>) -> Self {
        self.categories = categories.into_iter().map(|c| c.into()).collect();
        self
    }

    pub fn calculable(mut self, calculable: bool) -> Self {
        self.calculable = Some(calculable);
        self
    }

    pub fn orient(mut self, orient: Orient) -> Self {
        self.orient = Some(orient);
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

    pub fn text_style<T: Into<TextStyle>>(mut self, text_style: T) -> Self {
        self.text_style = Some(text_style.into());
        self
    }

    pub fn range<F: Into<f64>>(mut self, range: (F, F)) -> Self {
        self.range = Some((range.0.into(), range.1.into()));
        self
    }

    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = Some(inverse);
        self
    }

    pub fn precision<F: Into<f64>>(mut self, precision: F) -> Self {
        self.precision = Some(precision.into());
        self
    }

    pub fn item_width<F: Into<f64>>(mut self, item_width: F) -> Self {
        self.item_width = Some(item_width.into());
        self
    }

    pub fn item_height<F: Into<f64>>(mut self, item_height: F) -> Self {
        self.item_height = Some(item_height.into());
        self
    }

    pub fn in_range<V: Into<VisualMapChannel>>(mut self, in_range: V) -> Self {
        self.in_range = Some(in_range.into());
        self
    }

    pub fn out_range<V: Into<VisualMapChannel>>(mut self, out_range: V) -> Self {
        self.out_range = Some(out_range.into());
        self
    }

    pub fn pieces(mut self, pieces: Vec<VisualMapPiece>) -> Self {
        self.pieces = Some(pieces);
        self
    }
}
