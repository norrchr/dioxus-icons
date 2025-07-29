use dioxus::prelude::*;

pub mod icons;

#[derive(PartialEq, Eq, Clone, Default, Props)]
pub struct IconProps {
    id: Option<String>,
    class: Option<String>,
    style: Option<String>,
    xmlns: Option<String>,
    view_box: Option<String>,
    fill: Option<String>,
    x: Option<String>,
    y: Option<String>,
    width: Option<String>,
    height: Option<String>,
    stroke: Option<String>,
    stroke_width: Option<String>,
    stroke_linecap: Option<String>,
    stroke_linejoin: Option<String>,
}
