use std::collections::HashMap;
use std::path::PathBuf;
use svg::node::Value;
use svg::{node::element::tag::Type, parser::Event};
use crate::writer::IndentationWriter;

//const VALID_DIOXUS_SVG_ATTRIBUTES: [&str; 10] = ["id", "class", "style", "xmlns", "view_box", "fill", "x", "y", "width", "height"];
const VALID_ICON_PROPS: [&str; 14] = ["id", "class", "style", "xmlns", "view_box", "fill", "x", "y", "width", "height", "stroke", "stroke_width", "stroke_linecap", "stroke_linejoin"];

pub(crate) struct Attribute {
    pub name: String,
    pub value: Value,
}

impl Attribute {
    pub fn new(name: String, value: Value) -> Self {
        Self {
            name,
            value,
        }
    }
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        format!("{}: \"{}\",", process_attribute_name(&self.name), self.value)
    }
}

/// Process the attribute name and make it valid for rust and dioxus
/// If dioxus does not directly support the attribute, then we need to wrap it in quotes
/// Example: xmlns:xlink -> "xmlns:xlink"
/// Example: viewBox => view_box
pub(crate) fn process_attribute_name(name: &str) -> String {
    if name.contains(":") {
        format!("\"{}\"", name)
    } else {
        heck::AsSnakeCase(name).to_string()
    }
}

/// If value is Some, then the svg source had a value for the attribute
/// If value is None, then the svg source had no value for the attribute
/// 
/// If IconProps supports the attribute and the svg source has a value, then we use the prop value but fallback to the source value
/// If IconProps supports the attribute and the svg source does not have a value, then we use the prop value
/// If IconProps does not support the attribute, then we use the svg source value
pub(crate) fn optional_prop_value(opt: Option<String>, name: &str) -> String {

    // is name in IconProps?
    if VALID_ICON_PROPS.contains(&name) {
        match opt {
            Some(val) => format!("props.{name}.unwrap_or(\"{val}\".to_string())"),
            None => format!("props.{name}"),
        }
    } else {
        opt.unwrap_or_default()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct SvgAttributes {
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub xmlns: Option<String>,
    pub view_box: Option<String>,
    pub fill: Option<String>,
    pub x: Option<String>,
    pub y: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<String>,
    pub stroke_linecap: Option<String>,
    pub stroke_linejoin: Option<String>,
    pub misc: HashMap<String, String>,
}

pub(crate) fn process_svg_file(path: PathBuf, icon_name: String) -> Result<String, svg::parser::Error> {

    let mut content = String::new();
    let svg_file = svg::open(&path, &mut content).expect("Failed to open svg file");

    let mut writer = IndentationWriter::new();

    writer.indent_after_line(format!("pub fn {icon_name}(props: IconProps) -> Element {{"));
    writer.indent_after_line("rsx! {");

    for event in svg_file.into_iter() {
        match event {
            Event::Error(e) => return Err(e),
            Event::Text(text) => writer.write_line(format!("\"{text}\"")),
            Event::Tag(tag, typ, context) => {
                match typ {
                    Type::Start => {
                        writer.indent_after_line(format!("{tag} {{"));

                        if !context.is_empty() {
                            if tag == "svg" {
                                let mut attrs = SvgAttributes::default();

                                for (name, value) in context.into_iter() {
                                    match name.as_str() {
                                        "id" => attrs.id = Some(value.to_string()),
                                        "class" => attrs.class = Some(value.to_string()),
                                        "style" => attrs.style = Some(value.to_string()),
                                        "xmlns" => attrs.xmlns = Some(value.to_string()),
                                        "viewBox" => attrs.view_box = Some(value.to_string()),
                                        "fill" => attrs.fill = Some(value.to_string()),
                                        "x" => attrs.x = Some(value.to_string()),
                                        "y" => attrs.y = Some(value.to_string()),
                                        "width" => attrs.width = Some(value.to_string()),
                                        "height" => attrs.height = Some(value.to_string()),
                                        "stroke" => attrs.stroke = Some(value.to_string()),
                                        "stroke-width" => attrs.stroke_width = Some(value.to_string()),
                                        "stroke-linecap" => attrs.stroke_linecap = Some(value.to_string()),
                                        "stroke-linejoin" => attrs.stroke_linejoin = Some(value.to_string()),
                                        _ => _ = attrs.misc.insert(name.to_string(), value.to_string()),
                                    } 
                                }

                                // write the svg attributes
                                writer.write_line(format!("id: {},", optional_prop_value(attrs.id, "id")));
                                writer.write_line(format!("class: {},", optional_prop_value(attrs.class, "class")));
                                writer.write_line(format!("style: {},", optional_prop_value(attrs.style, "style")));
                                writer.write_line(format!("xmlns: {},", optional_prop_value(attrs.xmlns, "xmlns")));
                                writer.write_line(format!("view_box: {},", optional_prop_value(attrs.view_box, "view_box")));
                                writer.write_line(format!("fill: {},", optional_prop_value(attrs.fill, "fill")));
                                writer.write_line(format!("x: {},", optional_prop_value(attrs.x, "x")));
                                writer.write_line(format!("y: {},", optional_prop_value(attrs.y, "y")));
                                writer.write_line(format!("width: {},", optional_prop_value(attrs.width, "width")));
                                writer.write_line(format!("height: {},", optional_prop_value(attrs.height, "height")));
                                writer.write_line(format!("stroke: {},", optional_prop_value(attrs.stroke, "stroke")));
                                writer.write_line(format!("stroke_width: {},", optional_prop_value(attrs.stroke_width, "stroke_width")));
                                writer.write_line(format!("stroke_linecap: {},", optional_prop_value(attrs.stroke_linecap, "stroke_linecap")));
                                writer.write_line(format!("stroke_linejoin: {},", optional_prop_value(attrs.stroke_linejoin, "stroke_linejoin")));

                                // write the misc attributes
                                for (name, value) in attrs.misc.into_iter() {
                                    writer.write_line(format!("{}: \"{}\",", process_attribute_name(&name), value));
                                }
                            } else {
                                for (name, value) in context.into_iter() {
                                    let attr = Attribute::new(name, value);
                                    writer.write_line(attr.to_string());
                                }
                            }

                        }
                    },
                    Type::End => {
                        writer.dedent_before_line("}");
                    },
                    Type::Empty => {
                        writer.indent_after_line(format!("{tag} {{"));

                        for (name, value) in context.into_iter() {
                            let attr = Attribute::new(name, value);
                            writer.write_line(attr.to_string());
                        }

                        writer.dedent_before_line("}");
                    },
                }
            },
            _ => {} // we ignore instructions, declarations and comments
        }
    };

    // for rsx
    writer.dedent_before_line("}");

    // for fn
    writer.dedent_before_line("}");

    Ok(writer.to_string())
}