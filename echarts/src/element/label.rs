use serde::Serialize;

use super::{color::Color, line_style::LineStyle};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Position {
    Top,
    Left,
    Right,
    Bottom,
    Inside,
    InsideLeft,
    InsideRight,
    InsideTop,
    InsideBottom,
    InsideTopLeft,
    InsideBottomLeft,
    InsideTopRight,
    InsideBottomRight,
    Start,
    Outside,
    Middle,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rotate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<(f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formatter: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    font_size: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    padding: Option<(f64, f64, f64, f64)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    align: Option<Align>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_align: Option<VerticalAlign>,

    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
}

impl Label {
    pub fn new() -> Self {
        Self {
            show: None,
            position: None,
            distance: None,
            rotate: None,
            offset: None,
            formatter: None,
            color: None,
            font_size: None,
            padding: None,
            align: None,
            vertical_align: None,
            silent: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    pub fn distance(mut self, distance: f64) -> Self {
        self.distance = Some(distance);
        self
    }

    pub fn rotate<S: Into<String>>(mut self, rotate: S) -> Self {
        self.rotate = Some(rotate.into());
        self
    }

    pub fn offset(mut self, offset: (f64, f64)) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn formatter<S: Into<String>>(mut self, formatter: S) -> Self {
        self.formatter = Some(formatter.into());
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn font_size(mut self, font_size: f64) -> Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn padding<F: Into<f64>>(mut self, padding: (F, F, F, F)) -> Self {
        self.padding = Some((
            padding.0.into(),
            padding.1.into(),
            padding.2.into(),
            padding.3.into(),
        ));
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    pub fn vertical_align(mut self, vertical_align: VerticalAlign) -> Self {
        self.vertical_align = Some(vertical_align);
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = Some(silent);
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    show_above: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    length2: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    smooth: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_turn_angle: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    line_style: Option<LineStyle>,
}

impl LabelLine {
    pub fn new() -> Self {
        Self {
            show: None,
            show_above: None,
            length2: None,
            smooth: None,
            min_turn_angle: None,
            line_style: None,
        }
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = Some(show);
        self
    }

    pub fn show_above(mut self, show_above: bool) -> Self {
        self.show_above = Some(show_above);
        self
    }

    pub fn length2<F: Into<f64>>(mut self, length2: F) -> Self {
        self.length2 = Some(length2.into());
        self
    }

    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = Some(smooth);
        self
    }

    pub fn min_turn_angle<F: Into<f64>>(mut self, min_turn_angle: F) -> Self {
        self.min_turn_angle = Some(min_turn_angle.into());
        self
    }

    pub fn line_style<S: Into<LineStyle>>(mut self, line_style: S) -> Self {
        self.line_style = Some(line_style.into());
        self
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelLayout {
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_overlap: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    overlap: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rotate: Option<f64>,
}

impl LabelLayout {
    pub fn new() -> Self {
        Self {
            hide_overlap: None,
            overlap: None,
            rotate: None,
        }
    }

    pub fn hide_overlap(mut self, hide_overlap: bool) -> Self {
        self.hide_overlap = Some(hide_overlap);
        self
    }

    pub fn overlap<S: Into<String>>(mut self, overlap: S) -> Self {
        self.overlap = Some(overlap.into());
        self
    }

    pub fn rotate<F: Into<f64>>(mut self, rotate: F) -> Self {
        self.rotate = Some(rotate.into());
        self
    }
}