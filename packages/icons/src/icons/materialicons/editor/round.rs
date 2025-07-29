use dioxus::prelude::*;
use crate::IconProps;
pub fn add_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M16,17c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1s-1,0.45-1,1v2C15,16.55,15.45,17,16,17z",
                    }
                    path {
                        d: "M20,10c-0.55,0-1,0.45-1,1v8H5V5h8c0.55,0,1-0.45,1-1s-0.45-1-1-1H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14 c1.1,0,2-0.9,2-2v-8C21,10.45,20.55,10,20,10z",
                    }
                    path {
                        d: "M7,11v5c0,0.55,0.45,1,1,1s1-0.45,1-1v-5c0-0.55-0.45-1-1-1S7,10.45,7,11z",
                    }
                    path {
                        d: "M11,8v8c0,0.55,0.45,1,1,1s1-0.45,1-1V8c0-0.55-0.45-1-1-1S11,7.45,11,8z",
                    }
                    path {
                        d: "M16,7h1v1c0,0.55,0.45,1,1,1s1-0.45,1-1V7h1c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1V4c0-0.55-0.45-1-1-1s-1,0.45-1,1v1h-1 c-0.55,0-1,0.45-1,1S15.45,7,16,7z",
                    }
                }
            }
        }
    }
}

pub fn add_comment_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M22 4c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4V4zm-6 7h-3v3c0 .55-.45 1-1 1s-1-.45-1-1v-3H8c-.55 0-1-.45-1-1s.45-1 1-1h3V6c0-.55.45-1 1-1s1 .45 1 1v3h3c.55 0 1 .45 1 1s-.45 1-1 1z",
            }
        }
    }
}

pub fn align_horizontal_center_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M12,2L12,2c0.55,0,1,0.45,1,1v4l6.5,0C20.33,7,21,7.67,21,8.5v0c0,0.83-0.67,1.5-1.5,1.5H13v4h3.5c0.83,0,1.5,0.67,1.5,1.5 v0c0,0.83-0.67,1.5-1.5,1.5H13v4c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-4H7.5C6.67,17,6,16.33,6,15.5v0C6,14.67,6.67,14,7.5,14 H11v-4H4.5C3.67,10,3,9.33,3,8.5v0C3,7.67,3.67,7,4.5,7H11l0-4C11,2.45,11.45,2,12,2z",
            }
        }
    }
}

pub fn align_horizontal_left_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                fill: "none",
                height: "24",
                width: "24",
            }
            path {
                d: "M3,22L3,22c-0.55,0-1-0.45-1-1V3c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v18C4,21.55,3.55,22,3,22z M20.5,7h-13 C6.67,7,6,7.67,6,8.5v0C6,9.33,6.67,10,7.5,10h13c0.83,0,1.5-0.67,1.5-1.5v0C22,7.67,21.33,7,20.5,7z M14.5,14h-7 C6.67,14,6,14.67,6,15.5v0C6,16.33,6.67,17,7.5,17h7c0.83,0,1.5-0.67,1.5-1.5v0C16,14.67,15.33,14,14.5,14z",
            }
        }
    }
}

pub fn align_horizontal_right_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M21,2L21,2c0.55,0,1,0.45,1,1v18c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V3C20,2.45,20.45,2,21,2z M3.5,10h13 c0.83,0,1.5-0.67,1.5-1.5v0C18,7.67,17.33,7,16.5,7h-13C2.67,7,2,7.67,2,8.5v0C2,9.33,2.67,10,3.5,10z M9.5,17h7 c0.83,0,1.5-0.67,1.5-1.5v0c0-0.83-0.67-1.5-1.5-1.5h-7C8.67,14,8,14.67,8,15.5v0C8,16.33,8.67,17,9.5,17z",
            }
        }
    }
}

pub fn align_vertical_bottom_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                height: "24",
                fill: "none",
                width: "24",
            }
            path {
                d: "M21,22H3c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h18c0.55,0,1,0.45,1,1v0C22,21.55,21.55,22,21,22z M8.5,2L8.5,2 C7.67,2,7,2.67,7,3.5v13C7,17.33,7.67,18,8.5,18h0c0.83,0,1.5-0.67,1.5-1.5v-13C10,2.67,9.33,2,8.5,2z M15.5,8L15.5,8 C14.67,8,14,8.67,14,9.5v7c0,0.83,0.67,1.5,1.5,1.5h0c0.83,0,1.5-0.67,1.5-1.5v-7C17,8.67,16.33,8,15.5,8z",
            }
        }
    }
}

pub fn align_vertical_center_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                fill: "none",
                width: "24",
                height: "24",
            }
            path {
                d: "M21,11h-4V7.5C17,6.67,16.33,6,15.5,6h0C14.67,6,14,6.67,14,7.5V11h-4V4.5C10,3.67,9.33,3,8.5,3h0C7.67,3,7,3.67,7,4.5V11 H2.84c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1H7v6.5C7,20.33,7.67,21,8.5,21h0c0.83,0,1.5-0.67,1.5-1.5V13h4v3.5 c0,0.83,0.67,1.5,1.5,1.5h0c0.83,0,1.5-0.67,1.5-1.5V13h4c0.55,0,1-0.45,1-1v0C22,11.45,21.55,11,21,11z",
            }
        }
    }
}

pub fn align_vertical_top_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M22,3L22,3c0,0.55-0.45,1-1,1H3C2.45,4,2,3.55,2,3v0c0-0.55,0.45-1,1-1h18C21.55,2,22,2.45,22,3z M8.5,22L8.5,22 c0.83,0,1.5-0.67,1.5-1.5v-13C10,6.67,9.33,6,8.5,6h0C7.67,6,7,6.67,7,7.5v13C7,21.33,7.67,22,8.5,22z M15.5,16L15.5,16 c0.83,0,1.5-0.67,1.5-1.5v-7C17,6.67,16.33,6,15.5,6h0C14.67,6,14,6.67,14,7.5v7C14,15.33,14.67,16,15.5,16z",
            }
        }
    }
}

pub fn area_chart_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                width: "20",
                fill: "none",
                height: "20",
            }
            path {
                d: "M6.68,11.39L3.9,9.3C3.63,9.1,3.32,9,3,9l0-2.8c0.19,0,0.39,0.06,0.55,0.17L6,8l3.39-4.24c0.34-0.42,0.95-0.5,1.38-0.18 L14,6h2c0.55,0,1,0.45,1,1v6.08l-6.06-4.86c-0.67-0.53-1.65-0.4-2.15,0.29L6.68,11.39z M17,15l-6.18-4.96 c-0.44-0.36-1.1-0.27-1.43,0.19L7,13.5l-3.4-2.55c-0.18-0.13-0.39-0.2-0.6-0.2L3,15c0,0.55,0.45,1,1,1h12C16.55,16,17,15.55,17,15z",
            }
        }
    }
}

pub fn area_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M8,17l3.39-4.66c0.33-0.46,0.98-0.55,1.42-0.2L21,18.5V19c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1l0-5.72 c0.22,0,0.44,0.07,0.62,0.22L8,17z M3,11c0.44,0,0.88,0.15,1.25,0.44l3.37,2.69l2.77-3.81c0.66-0.91,1.95-1.1,2.85-0.4L21,15.97V8 c0-0.55-0.45-1-1-1h-3l-4.18-3.34c-0.45-0.36-1.1-0.27-1.44,0.2L7,10L3.6,7.45C3.42,7.32,3.21,7.25,3,7.25L3,11z",
            }
        }
    }
}

pub fn attach_file_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M16.5 6.75v10.58c0 2.09-1.53 3.95-3.61 4.15-2.39.23-4.39-1.64-4.39-3.98V5.14c0-1.31.94-2.5 2.24-2.63 1.5-.15 2.76 1.02 2.76 2.49v10.5c0 .55-.45 1-1 1s-1-.45-1-1V6.75c0-.41-.34-.75-.75-.75s-.75.34-.75.75v8.61c0 1.31.94 2.5 2.24 2.63 1.5.15 2.76-1.02 2.76-2.49V5.17c0-2.09-1.53-3.95-3.61-4.15C9.01.79 7 2.66 7 5v12.27c0 2.87 2.1 5.44 4.96 5.71 3.29.3 6.04-2.26 6.04-5.48V6.75c0-.41-.34-.75-.75-.75s-.75.34-.75.75z",
            }
        }
    }
}

pub fn attach_money_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M11.8 10.9c-2.27-.59-3-1.2-3-2.15 0-1.09 1.01-1.85 2.7-1.85 1.42 0 2.13.54 2.39 1.4.12.4.45.7.87.7h.3c.66 0 1.13-.65.9-1.27-.42-1.18-1.4-2.16-2.96-2.54V4.5c0-.83-.67-1.5-1.5-1.5S10 3.67 10 4.5v.66c-1.94.42-3.5 1.68-3.5 3.61 0 2.31 1.91 3.46 4.7 4.13 2.5.6 3 1.48 3 2.41 0 .69-.49 1.79-2.7 1.79-1.65 0-2.5-.59-2.83-1.43-.15-.39-.49-.67-.9-.67h-.28c-.67 0-1.14.68-.89 1.3.57 1.39 1.9 2.21 3.4 2.53v.67c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5v-.65c1.95-.37 3.5-1.5 3.5-3.55 0-2.84-2.43-3.81-4.7-4.4z",
            }
        }
    }
}

pub fn auto_graph_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                height: "20",
                fill: "none",
                width: "20",
            }
            path {
                d: "M12.05,9l-0.33-0.72L11,7.95c-0.39-0.18-0.39-0.73,0-0.91l0.72-0.33L12.05,6c0.18-0.39,0.73-0.39,0.91,0l0.33,0.72L14,7.05 c0.39,0.18,0.39,0.73,0,0.91l-0.72,0.33L12.95,9C12.78,9.39,12.22,9.39,12.05,9z M3.95,11l0.33-0.72L5,9.95 c0.39-0.18,0.39-0.73,0-0.91L4.28,8.72L3.95,8C3.78,7.61,3.22,7.61,3.05,8L2.72,8.72L2,9.05C1.61,9.22,1.61,9.78,2,9.95l0.72,0.33 L3.05,11C3.22,11.39,3.78,11.39,3.95,11z M7.45,7l0.49-1.06L9,5.45c0.39-0.18,0.39-0.73,0-0.91L7.94,4.06L7.45,3 C7.28,2.61,6.72,2.61,6.55,3L6.06,4.06L5,4.55C4.61,4.72,4.61,5.28,5,5.45l1.06,0.49L6.55,7C6.72,7.39,7.28,7.39,7.45,7z M18.44,7 c-0.31-0.28-0.78-0.25-1.06,0.06l-5.11,5.75l-2.56-2.56c-0.39-0.39-1.02-0.39-1.41,0l-4.76,4.77c-0.29,0.29-0.29,0.77,0,1.06 c0.29,0.29,0.77,0.29,1.06,0L9,11.67l2.58,2.58c0.41,0.41,1.07,0.39,1.45-0.04l5.47-6.15C18.78,7.75,18.75,7.27,18.44,7z",
            }
        }
    }
}

pub fn auto_graph_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                fill: "none",
                height: "24",
                width: "24",
            }
            g {
                path {
                    d: "M14.06,9.94L13,9.45c-0.39-0.18-0.39-0.73,0-0.91l1.06-0.49L14.55,7c0.18-0.39,0.73-0.39,0.91,0l0.49,1.06L17,8.55 c0.39,0.18,0.39,0.73,0,0.91l-1.06,0.49L15.45,11c-0.18,0.39-0.73,0.39-0.91,0L14.06,9.94z M4.45,13l0.49-1.06L6,11.45 c0.39-0.18,0.39-0.73,0-0.91l-1.06-0.49L4.45,9C4.28,8.61,3.72,8.61,3.55,9l-0.49,1.06L2,10.55c-0.39,0.18-0.39,0.73,0,0.91 l1.06,0.49L3.55,13C3.72,13.39,4.28,13.39,4.45,13z M8.96,7.99l0.63-1.4l1.4-0.63c0.39-0.18,0.39-0.73,0-0.91l-1.4-0.63l-0.63-1.4 c-0.18-0.39-0.73-0.39-0.91,0l-0.63,1.4l-1.4,0.63c-0.39,0.18-0.39,0.73,0,0.91l1.4,0.63l0.63,1.4C8.22,8.38,8.78,8.38,8.96,7.99z M22.34,8.27c-0.4-0.4-1.07-0.39-1.45,0.04l-6.39,7.18l-3.29-3.29c-0.39-0.39-1.02-0.39-1.41,0l-6.04,6.05 c-0.41,0.41-0.41,1.09,0,1.5c0.41,0.41,1.09,0.41,1.5,0l5.25-5.26l3.25,3.25c0.41,0.41,1.07,0.39,1.45-0.04l7.17-8.07 C22.73,9.24,22.71,8.64,22.34,8.27z",
                }
            }
        }
    }
}

pub fn bar_chart_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M13,11.5v3c0,0.83,0.67,1.5,1.5,1.5h0c0.83,0,1.5-0.67,1.5-1.5v-3c0-0.83-0.67-1.5-1.5-1.5h0C13.67,10,13,10.67,13,11.5z",
                    }
                    path {
                        d: "M5.5,16L5.5,16C6.33,16,7,15.33,7,14.5v-5C7,8.67,6.33,8,5.5,8h0C4.67,8,4,8.67,4,9.5v5C4,15.33,4.67,16,5.5,16z",
                    }
                    path {
                        d: "M10,16L10,16c0.83,0,1.5-0.67,1.5-1.5v-9C11.5,4.67,10.83,4,10,4h0C9.17,4,8.5,4.67,8.5,5.5v9C8.5,15.33,9.17,16,10,16z",
                    }
                }
            }
        }
    }
}

pub fn bar_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M6,20L6,20c1.1,0,2-0.9,2-2v-7c0-1.1-0.9-2-2-2h0c-1.1,0-2,0.9-2,2v7C4,19.1,4.9,20,6,20z",
                    }
                    path {
                        d: "M16,15v3c0,1.1,0.9,2,2,2h0c1.1,0,2-0.9,2-2v-3c0-1.1-0.9-2-2-2h0C16.9,13,16,13.9,16,15z",
                    }
                    path {
                        d: "M12,20L12,20c1.1,0,2-0.9,2-2V6c0-1.1-0.9-2-2-2h0c-1.1,0-2,0.9-2,2v12C10,19.1,10.9,20,12,20z",
                    }
                }
            }
        }
    }
}

pub fn border_all_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M3 5v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2zm8 14H6c-.55 0-1-.45-1-1v-5h5c.55 0 1 .45 1 1v5zm-1-8H5V6c0-.55.45-1 1-1h5v5c0 .55-.45 1-1 1zm8 8h-5v-5c0-.55.45-1 1-1h5v5c0 .55-.45 1-1 1zm1-8h-5c-.55 0-1-.45-1-1V5h5c.55 0 1 .45 1 1v5z",
            }
        }
    }
}

pub fn border_bottom_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M9 11H7v2h2v-2zm4 4h-2v2h2v-2zM9 3H7v2h2V3zm4 8h-2v2h2v-2zM5 3H3v2h2V3zm8 4h-2v2h2V7zm4 4h-2v2h2v-2zm-4-8h-2v2h2V3zm4 0h-2v2h2V3zm2 10h2v-2h-2v2zm0 4h2v-2h-2v2zM5 7H3v2h2V7zm14-4v2h2V3h-2zm0 6h2V7h-2v2zM5 11H3v2h2v-2zM4 21h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm1-6H3v2h2v-2z",
            }
        }
    }
}

pub fn border_clear_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M7 5h2V3H7v2zm0 8h2v-2H7v2zm0 8h2v-2H7v2zm4-4h2v-2h-2v2zm0 4h2v-2h-2v2zm-8 0h2v-2H3v2zm0-4h2v-2H3v2zm0-4h2v-2H3v2zm0-4h2V7H3v2zm0-4h2V3H3v2zm8 8h2v-2h-2v2zm8 4h2v-2h-2v2zm0-4h2v-2h-2v2zm0 8h2v-2h-2v2zm0-12h2V7h-2v2zm-8 0h2V7h-2v2zm8-6v2h2V3h-2zm-8 2h2V3h-2v2zm4 16h2v-2h-2v2zm0-8h2v-2h-2v2zm0-8h2V3h-2v2z",
            }
        }
    }
}

pub fn border_color_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M20,24H4c-1.1,0-2-0.9-2-2v0c0-1.1,0.9-2,2-2h16c1.1,0,2,0.9,2,2v0C22,23.1,21.1,24,20,24z M13.06,5.19l3.75,3.75l-8.77,8.77C7.86,17.9,7.6,18,7.34,18H5c-0.55,0-1-0.45-1-1v-2.34c0-0.27,0.11-0.52,0.29-0.71L13.06,5.19z M17.88,7.87l-3.75-3.75l1.83-1.83c0.39-0.39,1.02-0.39,1.41,0l2.34,2.34c0.39,0.39,0.39,1.02,0,1.41L17.88,7.87z",
                    enable_background: "new",
                }
            }
        }
    }
}

pub fn border_horizontal_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M3 21h2v-2H3v2zM5 7H3v2h2V7zM3 17h2v-2H3v2zm4 4h2v-2H7v2zM5 3H3v2h2V3zm4 0H7v2h2V3zm8 0h-2v2h2V3zm-4 4h-2v2h2V7zm0-4h-2v2h2V3zm6 14h2v-2h-2v2zm-8 4h2v-2h-2v2zm-7-8h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zM19 3v2h2V3h-2zm0 6h2V7h-2v2zm-8 8h2v-2h-2v2zm4 4h2v-2h-2v2zm4 0h2v-2h-2v2z",
            }
        }
    }
}

pub fn border_inner_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M3 21h2v-2H3v2zm4 0h2v-2H7v2zM5 7H3v2h2V7zM3 17h2v-2H3v2zM9 3H7v2h2V3zM5 3H3v2h2V3zm12 0h-2v2h2V3zm2 6h2V7h-2v2zm0-6v2h2V3h-2zm-4 18h2v-2h-2v2zM12 3c-.55 0-1 .45-1 1v7H4c-.55 0-1 .45-1 1s.45 1 1 1h7v7c0 .55.45 1 1 1s1-.45 1-1v-7h7c.55 0 1-.45 1-1s-.45-1-1-1h-7V4c0-.55-.45-1-1-1zm7 18h2v-2h-2v2zm0-4h2v-2h-2v2z",
            }
        }
    }
}

pub fn border_left_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M11 21h2v-2h-2v2zm0-4h2v-2h-2v2zm0-12h2V3h-2v2zm0 4h2V7h-2v2zm0 4h2v-2h-2v2zm-4 8h2v-2H7v2zM7 5h2V3H7v2zm0 8h2v-2H7v2zm-3 8c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1s-1 .45-1 1v16c0 .55.45 1 1 1zM19 9h2V7h-2v2zm-4 12h2v-2h-2v2zm4-4h2v-2h-2v2zm0-14v2h2V3h-2zm0 10h2v-2h-2v2zm0 8h2v-2h-2v2zm-4-8h2v-2h-2v2zm0-8h2V3h-2v2z",
            }
        }
    }
}

pub fn border_outer_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M13 7h-2v2h2V7zm0 4h-2v2h2v-2zm4 0h-2v2h2v-2zM3 5v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2zm15 14H6c-.55 0-1-.45-1-1V6c0-.55.45-1 1-1h12c.55 0 1 .45 1 1v12c0 .55-.45 1-1 1zm-5-4h-2v2h2v-2zm-4-4H7v2h2v-2z",
            }
        }
    }
}

pub fn border_right_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M7 21h2v-2H7v2zM3 5h2V3H3v2zm4 0h2V3H7v2zm0 8h2v-2H7v2zm-4 8h2v-2H3v2zm8 0h2v-2h-2v2zm-8-8h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm8 8h2v-2h-2v2zm4-4h2v-2h-2v2zm4-9v16c0 .55.45 1 1 1s1-.45 1-1V4c0-.55-.45-1-1-1s-1 .45-1 1zm-4 17h2v-2h-2v2zm0-16h2V3h-2v2zm-4 8h2v-2h-2v2zm0-8h2V3h-2v2zm0 4h2V7h-2v2z",
            }
        }
    }
}

pub fn border_style_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M15 21h2v-2h-2v2zm4 0h2v-2h-2v2zM7 21h2v-2H7v2zm4 0h2v-2h-2v2zm8-4h2v-2h-2v2zm0-4h2v-2h-2v2zM3 5v15c0 .55.45 1 1 1s1-.45 1-1V6c0-.55.45-1 1-1h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-1.1 0-2 .9-2 2zm16 4h2V7h-2v2z",
            }
        }
    }
}

pub fn border_top_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M7 21h2v-2H7v2zm0-8h2v-2H7v2zm4 0h2v-2h-2v2zm0 8h2v-2h-2v2zm-8-4h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2v-2H3v2zm0-4h2V7H3v2zm8 8h2v-2h-2v2zm8-8h2V7h-2v2zm0 4h2v-2h-2v2zM3 4c0 .55.45 1 1 1h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1zm16 13h2v-2h-2v2zm-4 4h2v-2h-2v2zM11 9h2V7h-2v2zm8 12h2v-2h-2v2zm-4-8h2v-2h-2v2z",
            }
        }
    }
}

pub fn border_vertical_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M3 9h2V7H3v2zm0-4h2V3H3v2zm4 16h2v-2H7v2zm0-8h2v-2H7v2zm-4 0h2v-2H3v2zm0 8h2v-2H3v2zm0-4h2v-2H3v2zM7 5h2V3H7v2zm12 12h2v-2h-2v2zm-7 4c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1s-1 .45-1 1v16c0 .55.45 1 1 1zm7 0h2v-2h-2v2zm0-8h2v-2h-2v2zm0-10v2h2V3h-2zm0 6h2V7h-2v2zm-4-4h2V3h-2v2zm0 16h2v-2h-2v2zm0-8h2v-2h-2v2z",
            }
        }
    }
}

pub fn bubble_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            circle {
                cy: "14.4",
                r: "3.2",
                cx: "7.2",
            }
            circle {
                cx: "14.8",
                r: "2",
                cy: "18",
            }
            circle {
                cx: "15.2",
                cy: "8.8",
                r: "4.8",
            }
        }
    }
}

pub fn candlestick_chart_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M6.5,3L6.5,3C6.09,3,5.75,3.34,5.75,3.75V5H5C4.45,5,4,5.45,4,6v8c0,0.55,0.45,1,1,1h0.75v1.25 C5.75,16.66,6.09,17,6.5,17h0c0.41,0,0.75-0.34,0.75-0.75V15H8c0.55,0,1-0.45,1-1V6c0-0.55-0.45-1-1-1H7.25V3.75 C7.25,3.34,6.91,3,6.5,3z",
                        }
                    }
                    g {
                        path {
                            d: "M15,7h-0.75V3.75C14.25,3.34,13.91,3,13.5,3h0c-0.41,0-0.75,0.34-0.75,0.75V7H12c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1 h0.75v3.25c0,0.41,0.34,0.75,0.75,0.75h0c0.41,0,0.75-0.34,0.75-0.75V13H15c0.55,0,1-0.45,1-1V8C16,7.45,15.55,7,15,7z",
                        }
                    }
                }
            }
        }
    }
}

pub fn candlestick_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M8,4L8,4C7.45,4,7,4.45,7,5v1H6C5.45,6,5,6.45,5,7v10c0,0.55,0.45,1,1,1h1v1c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-1h1 c0.55,0,1-0.45,1-1V7c0-0.55-0.45-1-1-1H9V5C9,4.45,8.55,4,8,4z",
                        }
                    }
                    g {
                        path {
                            d: "M18,8h-1V5c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v3h-1c-0.55,0-1,0.45-1,1v5c0,0.55,0.45,1,1,1h1v4c0,0.55,0.45,1,1,1 h0c0.55,0,1-0.45,1-1v-4h1c0.55,0,1-0.45,1-1V9C19,8.45,18.55,8,18,8z",
                        }
                    }
                }
            }
        }
    }
}

pub fn checklist_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                fill: "none",
                height: "20",
                width: "20",
            }
            g {
                path {
                    d: "M11,6.75C11,6.34,11.34,6,11.75,6h5.5C17.66,6,18,6.34,18,6.75c0,0.41-0.34,0.75-0.75,0.75h-5.5C11.34,7.5,11,7.16,11,6.75 z M18,13.25c0-0.41-0.34-0.75-0.75-0.75h-5.5c-0.41,0-0.75,0.34-0.75,0.75S11.34,14,11.75,14h5.5C17.66,14,18,13.66,18,13.25z M8.89,3.87c-0.29-0.29-0.77-0.29-1.06,0L4.83,6.88L3.59,5.64c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.77,0,1.06l1.59,1.59 c0.39,0.39,1.02,0.39,1.41,0l3.36-3.36C9.19,4.64,9.19,4.17,8.89,3.87z M8.89,10.37c-0.29-0.29-0.77-0.29-1.06,0l-3.01,3.01 l-1.24-1.24c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.77,0,1.06l1.59,1.59c0.39,0.39,1.02,0.39,1.41,0l3.36-3.36 C9.19,11.14,9.19,10.67,8.89,10.37z",
                }
            }
        }
    }
}

pub fn checklist_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                fill: "none",
                height: "24",
                width: "24",
            }
            g {
                path {
                    d: "M22,8c0-0.55-0.45-1-1-1h-7c-0.55,0-1,0.45-1,1s0.45,1,1,1h7C21.55,9,22,8.55,22,8z M13,16c0,0.55,0.45,1,1,1h7 c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-7C13.45,15,13,15.45,13,16z M10.47,4.63c0.39,0.39,0.39,1.02,0,1.41l-4.23,4.25 c-0.39,0.39-1.02,0.39-1.42,0L2.7,8.16c-0.39-0.39-0.39-1.02,0-1.41c0.39-0.39,1.02-0.39,1.41,0l1.42,1.42l3.54-3.54 C9.45,4.25,10.09,4.25,10.47,4.63z M10.48,12.64c0.39,0.39,0.39,1.02,0,1.41l-4.23,4.25c-0.39,0.39-1.02,0.39-1.42,0L2.7,16.16 c-0.39-0.39-0.39-1.02,0-1.41s1.02-0.39,1.41,0l1.42,1.42l3.54-3.54C9.45,12.25,10.09,12.25,10.48,12.64L10.48,12.64z",
                }
            }
        }
    }
}

pub fn checklist_rtl_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                fill: "none",
                height: "20",
                width: "20",
            }
            path {
                d: "M9,6.75C9,7.16,8.66,7.5,8.25,7.5h-5.5C2.34,7.5,2,7.16,2,6.75S2.34,6,2.75,6h5.5C8.66,6,9,6.34,9,6.75z M9,13.25 c0-0.41-0.34-0.75-0.75-0.75h-5.5C2.34,12.5,2,12.84,2,13.25C2,13.66,2.34,14,2.75,14h5.5C8.66,14,9,13.66,9,13.25z M17.47,3.87 c-0.29-0.29-0.77-0.29-1.06,0L13.4,6.88l-1.24-1.24c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.77,0,1.06l1.59,1.59 c0.39,0.39,1.02,0.39,1.41,0l3.36-3.36C17.76,4.64,17.76,4.17,17.47,3.87z M17.47,10.37c-0.29-0.29-0.77-0.29-1.06,0l-3.01,3.01 l-1.24-1.24c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.77,0,1.06l1.59,1.59c0.39,0.39,1.02,0.39,1.41,0l3.36-3.36 C17.76,11.14,17.76,10.67,17.47,10.37z",
            }
        }
    }
}

pub fn checklist_rtl_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M11,8c0-0.55-0.45-1-1-1H3C2.45,7,2,7.45,2,8s0.45,1,1,1h7C10.55,9,11,8.55,11,8z M11,16c0-0.55-0.45-1-1-1H3 c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h7C10.55,17,11,16.55,11,16z M17.05,10.29c-0.39,0.39-1.02,0.39-1.41,0l-2.12-2.12 c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0l1.41,1.41l3.54-3.54c0.39-0.39,1.02-0.39,1.41,0l0,0 c0.39,0.39,0.39,1.02,0,1.41L17.05,10.29z M17.05,18.29c-0.39,0.39-1.02,0.39-1.41,0l-2.12-2.12c-0.39-0.39-0.39-1.02,0-1.41l0,0 c0.39-0.39,1.02-0.39,1.41,0l1.41,1.41l3.54-3.54c0.39-0.39,1.02-0.39,1.41,0l0,0c0.39,0.39,0.39,1.02,0,1.41L17.05,18.29z",
            }
        }
    }
}

pub fn data_array_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M4,5.5v9C4,15.33,4.67,16,5.5,16h1.75C7.66,16,8,15.66,8,15.25v0c0-0.41-0.34-0.75-0.75-0.75H5.5v-9h1.75 C7.66,5.5,8,5.16,8,4.75v0C8,4.34,7.66,4,7.25,4H5.5C4.67,4,4,4.67,4,5.5z",
                    }
                    path {
                        d: "M12,4.75L12,4.75c0,0.41,0.34,0.75,0.75,0.75h1.75v9h-1.75c-0.41,0-0.75,0.34-0.75,0.75v0c0,0.41,0.34,0.75,0.75,0.75 h1.75c0.83,0,1.5-0.67,1.5-1.5v-9C16,4.67,15.33,4,14.5,4h-1.75C12.34,4,12,4.34,12,4.75z",
                    }
                }
            }
        }
    }
}

pub fn data_array_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M15,5L15,5c0,0.55,0.45,1,1,1h2v12h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2c1.1,0,2-0.9,2-2V6c0-1.1-0.9-2-2-2h-2 C15.45,4,15,4.45,15,5z",
                    }
                    path {
                        d: "M6,20h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6V6h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6C4.9,4,4,4.9,4,6v12 C4,19.1,4.9,20,6,20z",
                    }
                }
            }
        }
    }
}

pub fn data_object_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M4,6.25v1.5C4,8.16,3.66,8.5,3.25,8.5h0C2.56,8.5,2,9.06,2,9.75v0.5c0,0.69,0.56,1.25,1.25,1.25h0 C3.66,11.5,4,11.84,4,12.25v1.5C4,14.99,5.01,16,6.25,16h1C7.66,16,8,15.66,8,15.25v0c0-0.41-0.34-0.75-0.75-0.75h-1 c-0.41,0-0.75-0.34-0.75-0.75v-1.5c0-1.16-0.88-2.11-2-2.24V9.99c1.12-0.12,2-1.08,2-2.24v-1.5c0-0.41,0.34-0.75,0.75-0.75h1 C7.66,5.5,8,5.16,8,4.75v0C8,4.34,7.66,4,7.25,4h-1C5.01,4,4,5.01,4,6.25z",
                    }
                    path {
                        d: "M16.75,8.5C16.34,8.5,16,8.16,16,7.75v-1.5C16,5.01,14.99,4,13.75,4h-1C12.34,4,12,4.34,12,4.75v0 c0,0.41,0.34,0.75,0.75,0.75h1c0.41,0,0.75,0.34,0.75,0.75v1.5c0,1.16,0.88,2.11,2,2.24v0.03c-1.12,0.12-2,1.08-2,2.24v1.5 c0,0.41-0.34,0.75-0.75,0.75h-1c-0.41,0-0.75,0.34-0.75,0.75v0c0,0.41,0.34,0.75,0.75,0.75h1c1.24,0,2.25-1.01,2.25-2.25v-1.5 c0-0.41,0.34-0.75,0.75-0.75h0c0.69,0,1.25-0.56,1.25-1.25v-0.5C18,9.06,17.44,8.5,16.75,8.5L16.75,8.5z",
                    }
                }
            }
        }
    }
}

pub fn data_object_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M4,7v2c0,0.55-0.45,1-1,1h0c-0.55,0-1,0.45-1,1v2c0,0.55,0.45,1,1,1h0c0.55,0,1,0.45,1,1v2c0,1.66,1.34,3,3,3h2 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H7c-0.55,0-1-0.45-1-1v-2c0-1.3-0.84-2.42-2-2.83v-0.34C5.16,11.42,6,10.3,6,9V7 c0-0.55,0.45-1,1-1h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H7C5.34,4,4,5.34,4,7z",
                    }
                    path {
                        d: "M21,10c-0.55,0-1-0.45-1-1V7c0-1.66-1.34-3-3-3h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2c0.55,0,1,0.45,1,1v2 c0,1.3,0.84,2.42,2,2.83v0.34c-1.16,0.41-2,1.52-2,2.83v2c0,0.55-0.45,1-1,1h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2 c1.66,0,3-1.34,3-3v-2c0-0.55,0.45-1,1-1h0c0.55,0,1-0.45,1-1v-2C22,10.45,21.55,10,21,10L21,10z",
                    }
                }
            }
        }
    }
}

pub fn drag_handle_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M19 9H5c-.55 0-1 .45-1 1s.45 1 1 1h14c.55 0 1-.45 1-1s-.45-1-1-1zM5 15h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1 .45-1 1s.45 1 1 1z",
            }
        }
    }
}

pub fn draw_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                fill: "none",
                height: "20",
                width: "20",
            }
            path {
                d: "M15.35,8.83l0.71-0.71c0.59-0.59,0.59-1.54,0-2.12L15,4.94c-0.59-0.59-1.54-0.59-2.12,0l-0.71,0.71L15.35,8.83z M11.11,6.71 l-6.96,6.96C4.05,13.77,4,13.89,4,14.03v2.47C4,16.78,4.22,17,4.5,17h2.47c0.13,0,0.26-0.05,0.35-0.15l6.96-6.96L11.11,6.71z M4.51,11.18C3.59,10.76,3,10.16,3,9.25c0-1.31,1.39-1.99,2.61-2.59C6.45,6.24,7.5,5.73,7.5,5.25C7.5,4.91,6.83,4.5,6,4.5 c-0.94,0-1.36,0.46-1.38,0.48C4.35,5.29,3.88,5.33,3.57,5.07C3.26,4.81,3.21,4.35,3.46,4.03C3.55,3.93,4.34,3,6,3 c1.47,0,3,0.84,3,2.25C9,6.66,7.55,7.37,6.27,8C5.56,8.35,4.5,8.87,4.5,9.25c0,0.3,0.48,0.56,1.17,0.78L4.51,11.18z M14.14,12.16 c0.83,0.48,1.36,1.14,1.36,2.09c0,1.94-2.44,2.75-3.75,2.75C11.34,17,11,16.66,11,16.25s0.34-0.75,0.75-0.75 c0.77,0,2.25-0.49,2.25-1.25c0-0.39-0.38-0.71-0.97-0.97L14.14,12.16z",
            }
        }
    }
}

pub fn draw_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                height: "24",
                fill: "none",
                width: "24",
            }
            path {
                d: "M18.85,10.39l1.06-1.06c0.78-0.78,0.78-2.05,0-2.83L18.5,5.09c-0.78-0.78-2.05-0.78-2.83,0l-1.06,1.06L18.85,10.39z M13.19,7.56l-9.05,9.05C4.05,16.7,4,16.83,4,16.96v3.54C4,20.78,4.22,21,4.5,21h3.54c0.13,0,0.26-0.05,0.35-0.15l9.05-9.05 L13.19,7.56z M19,17.5c0,2.19-2.54,3.5-5,3.5c-0.55,0-1-0.45-1-1s0.45-1,1-1c1.54,0,3-0.73,3-1.5c0-0.47-0.48-0.87-1.23-1.2 l1.48-1.48C18.32,15.45,19,16.29,19,17.5z M4.58,13.35C3.61,12.79,3,12.06,3,11c0-1.8,1.89-2.63,3.56-3.36C7.59,7.18,9,6.56,9,6 c0-0.41-0.78-1-2-1C5.74,5,5.2,5.61,5.17,5.64C4.82,6.05,4.19,6.1,3.77,5.76C3.36,5.42,3.28,4.81,3.62,4.38C3.73,4.24,4.76,3,7,3 c2.24,0,4,1.32,4,3c0,1.87-1.93,2.72-3.64,3.47C6.42,9.88,5,10.5,5,11c0,0.31,0.43,0.6,1.07,0.86L4.58,13.35z",
            }
        }
    }
}

pub fn edit_note_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                width: "20",
                fill: "none",
                height: "20",
            }
            path {
                d: "M12,5.75c0,0.41-0.34,0.75-0.75,0.75h-7.5C3.34,6.5,3,6.16,3,5.75C3,5.34,3.34,5,3.75,5h7.5C11.66,5,12,5.34,12,5.75z M9,12 c0,0.41-0.34,0.75-0.75,0.75h-4.5C3.34,12.75,3,12.41,3,12c0-0.41,0.34-0.75,0.75-0.75h4.5C8.66,11.25,9,11.59,9,12z M12,8.87 c0,0.41-0.34,0.75-0.75,0.75h-7.5C3.34,9.62,3,9.28,3,8.87c0-0.41,0.34-0.75,0.75-0.75h7.5C11.66,8.12,12,8.46,12,8.87z M16.78,11.99l0.65-0.65c0.29-0.29,0.29-0.77,0-1.06l-0.71-0.71c-0.29-0.29-0.77-0.29-1.06,0l-0.65,0.65L16.78,11.99z M16.19,12.58 l-4.27,4.27C11.82,16.95,11.7,17,11.56,17H10.5c-0.28,0-0.5-0.22-0.5-0.5v-1.06c0-0.13,0.05-0.26,0.15-0.35l4.27-4.27L16.19,12.58z",
            }
        }
    }
}

pub fn edit_note_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                height: "24",
                fill: "none",
                width: "24",
            }
            path {
                d: "M14,11c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1s0.45-1,1-1h9C13.55,10,14,10.45,14,11z M3,7c0,0.55,0.45,1,1,1h9 c0.55,0,1-0.45,1-1s-0.45-1-1-1H4C3.45,6,3,6.45,3,7z M10,15c0-0.55-0.45-1-1-1H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h5 C9.55,16,10,15.55,10,15z M18.01,12.87l0.71-0.71c0.39-0.39,1.02-0.39,1.41,0l0.71,0.71c0.39,0.39,0.39,1.02,0,1.41l-0.71,0.71 L18.01,12.87z M17.3,13.58l-5.16,5.16C12.05,18.83,12,18.95,12,19.09v1.41c0,0.28,0.22,0.5,0.5,0.5h1.41c0.13,0,0.26-0.05,0.35-0.15 l5.16-5.16L17.3,13.58z",
            }
        }
    }
}

pub fn format_align_center_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M7 16c0 .55.45 1 1 1h8c.55 0 1-.45 1-1s-.45-1-1-1H8c-.55 0-1 .45-1 1zm-3 5h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm0-8h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm3-5c0 .55.45 1 1 1h8c.55 0 1-.45 1-1s-.45-1-1-1H8c-.55 0-1 .45-1 1zM3 4c0 .55.45 1 1 1h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn format_align_justify_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M4 21h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm0-4h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm0-4h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm0-4h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zM3 4c0 .55.45 1 1 1h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn format_align_left_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M14 15H4c-.55 0-1 .45-1 1s.45 1 1 1h10c.55 0 1-.45 1-1s-.45-1-1-1zm0-8H4c-.55 0-1 .45-1 1s.45 1 1 1h10c.55 0 1-.45 1-1s-.45-1-1-1zM4 13h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm0 8h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zM3 4c0 .55.45 1 1 1h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn format_align_right_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M4 21h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm6-4h10c.55 0 1-.45 1-1s-.45-1-1-1H10c-.55 0-1 .45-1 1s.45 1 1 1zm-6-4h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm6-4h10c.55 0 1-.45 1-1s-.45-1-1-1H10c-.55 0-1 .45-1 1s.45 1 1 1zM3 4c0 .55.45 1 1 1h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn format_bold_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M15.6 10.79c.97-.67 1.65-1.77 1.65-2.79 0-2.26-1.75-4-4-4H8c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h5.78c2.07 0 3.96-1.69 3.97-3.77.01-1.53-.85-2.84-2.15-3.44zM10 6.5h3c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-3v-3zm3.5 9H10v-3h3.5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5z",
            }
        }
    }
}

pub fn format_clear_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M18.5 8c.83 0 1.5-.67 1.5-1.5S19.33 5 18.5 5H6.39l3 3h1.83l-.55 1.28 2.09 2.09L14.21 8h4.29zm-1.06 10.88L4.12 5.56c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l6.26 6.26-1.65 3.84c-.39.92.28 1.93 1.27 1.93.55 0 1.05-.33 1.27-.84l1.21-2.83 4.95 4.95c.39.39 1.02.39 1.41 0 .4-.38.4-1.01.01-1.4z",
            }
        }
    }
}

pub fn format_color_fill_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                width: "20",
                fill: "none",
                height: "20",
            }
            path {
                d: "M13.5,12.25c0-1.16,1.75-3.06,1.75-3.06S17,11.09,17,12.25c0,0.96-0.79,1.75-1.75,1.75S13.5,13.21,13.5,12.25z M2,18L2,18 c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v0c0-1.1-0.9-2-2-2H4C2.9,16,2,16.9,2,18z M3.4,8.96c-0.53-0.53-0.53-1.4,0-1.93l3.54-3.54 l-1.9-1.9c-0.29-0.29-0.29-0.77,0-1.06c0.29-0.29,0.77-0.29,1.06,0l6.5,6.5c0.53,0.53,0.53,1.4,0,1.93L8.97,12.6 C8.7,12.87,8.35,13,8,13s-0.7-0.13-0.97-0.4L3.4,8.96z M4.56,8h6.89L8,4.56L4.56,8z",
            }
        }
    }
}

pub fn format_color_fill_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M8.94,16.56C9.23,16.85,9.62,17,10,17s0.77-0.15,1.06-0.44l5.5-5.5 c0.59-0.58,0.59-1.53,0-2.12L8.32,0.7c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l1.68,1.68L3.44,8.94 c-0.59,0.59-0.59,1.54,0,2.12L8.94,16.56z M10,5.21L14.79,10H5.21L10,5.21z",
                        enable_background: "new",
                    }
                    path {
                        d: "M19,17c1.1,0,2-0.9,2-2c0-1.33-2-3.5-2-3.5s-2,2.17-2,3.5C17,16.1,17.9,17,19,17z",
                        enable_background: "new",
                    }
                    path {
                        enable_background: "new",
                        d: "M20,20H4c-1.1,0-2,0.9-2,2s0.9,2,2,2h16c1.1,0,2-0.9,2-2S21.1,20,20,20z",
                    }
                }
            }
        }
    }
}

pub fn format_color_reset_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0zm0 0h24v24H0V0zm0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M18 14c0-3.09-3.6-7.88-5.23-9.87-.4-.49-1.15-.49-1.55 0-.46.57-1.08 1.36-1.73 2.27l8.44 8.44c.04-.28.07-.56.07-.84zm1.29 5.01L6.12 5.84c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l2.61 2.61C6.55 11.33 6 12.79 6 14c0 3.31 2.69 6 6 6 1.52 0 2.9-.57 3.95-1.5l1.92 1.92c.39.39 1.02.39 1.41 0 .4-.38.4-1.02.01-1.41z",
            }
        }
    }
}

pub fn format_color_text_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M16,16H4c-1.1,0-2,0.9-2,2s0.9,2,2,2h12c1.1,0,2-0.9,2-2S17.1,16,16,16z",
                    }
                    path {
                        d: "M6.51,13L6.51,13c0.34,0,0.65-0.21,0.76-0.53l0.72-2.02h4.04l0.71,2.02c0.11,0.32,0.42,0.54,0.76,0.54 c0.56,0,0.95-0.56,0.75-1.09l-3.03-8.08C11.02,3.33,10.54,3,10,3S8.98,3.33,8.79,3.84l-3.03,8.08C5.56,12.44,5.95,13,6.51,13z M9.57,6.02l0.39-1.16h0.08l0.39,1.16l1.06,2.98H8.51L9.57,6.02z",
                    }
                }
            }
        }
    }
}

pub fn format_color_text_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M20,20H4c-1.1,0-2,0.9-2,2s0.9,2,2,2h16c1.1,0,2-0.9,2-2S21.1,20,20,20z",
                    }
                    path {
                        d: "M7.11,17L7.11,17c0.48,0,0.91-0.3,1.06-0.75l1.01-2.83h5.65l0.99,2.82C15.98,16.7,16.41,17,16.89,17 c0.79,0,1.33-0.79,1.05-1.52L13.69,4.17C13.43,3.47,12.75,3,12,3s-1.43,0.47-1.69,1.17L6.06,15.48C5.78,16.21,6.33,17,7.11,17z M11.94,5.6h0.12l2.03,5.79H9.91L11.94,5.6z",
                    }
                }
            }
        }
    }
}

pub fn format_indent_decrease_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 17h8c.55 0 1-.45 1-1s-.45-1-1-1h-8c-.55 0-1 .45-1 1s.45 1 1 1zm-8.65-4.65l2.79 2.79c.32.32.86.1.86-.35V9.21c0-.45-.54-.67-.85-.35l-2.79 2.79c-.2.19-.2.51-.01.7zM4 21h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zM3 4c0 .55.45 1 1 1h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1zm9 5h8c.55 0 1-.45 1-1s-.45-1-1-1h-8c-.55 0-1 .45-1 1s.45 1 1 1zm0 4h8c.55 0 1-.45 1-1s-.45-1-1-1h-8c-.55 0-1 .45-1 1s.45 1 1 1z",
            }
        }
    }
}

pub fn format_indent_increase_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M4 21h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zM3 9.21v5.59c0 .45.54.67.85.35l2.79-2.79c.2-.2.2-.51 0-.71l-2.79-2.8c-.31-.31-.85-.09-.85.36zM12 17h8c.55 0 1-.45 1-1s-.45-1-1-1h-8c-.55 0-1 .45-1 1s.45 1 1 1zM3 4c0 .55.45 1 1 1h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1zm9 5h8c.55 0 1-.45 1-1s-.45-1-1-1h-8c-.55 0-1 .45-1 1s.45 1 1 1zm0 4h8c.55 0 1-.45 1-1s-.45-1-1-1h-8c-.55 0-1 .45-1 1s.45 1 1 1z",
            }
        }
    }
}

pub fn format_italic_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M10 5.5c0 .83.67 1.5 1.5 1.5h.71l-3.42 8H7.5c-.83 0-1.5.67-1.5 1.5S6.67 18 7.5 18h5c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5h-.71l3.42-8h1.29c.83 0 1.5-.67 1.5-1.5S17.33 4 16.5 4h-5c-.83 0-1.5.67-1.5 1.5z",
            }
        }
    }
}

pub fn format_line_spacing_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M7.29 7c.45 0 .67-.54.35-.85l-2.29-2.3c-.2-.2-.51-.2-.71 0l-2.29 2.3c-.31.31-.09.85.36.85H4v10H2.71c-.45 0-.67.54-.35.85l2.29 2.29c.2.2.51.2.71 0l2.29-2.29c.31-.31.09-.85-.36-.85H6V7h1.29zM11 7h10c.55 0 1-.45 1-1s-.45-1-1-1H11c-.55 0-1 .45-1 1s.45 1 1 1zm10 10H11c-.55 0-1 .45-1 1s.45 1 1 1h10c.55 0 1-.45 1-1s-.45-1-1-1zm0-6H11c-.55 0-1 .45-1 1s.45 1 1 1h10c.55 0 1-.45 1-1s-.45-1-1-1z",
            }
        }
    }
}

pub fn format_list_bulleted_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M4 10.5c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm0-6c-.83 0-1.5.67-1.5 1.5S3.17 7.5 4 7.5 5.5 6.83 5.5 6 4.83 4.5 4 4.5zm0 12c-.83 0-1.5.68-1.5 1.5s.68 1.5 1.5 1.5 1.5-.68 1.5-1.5-.67-1.5-1.5-1.5zM8 19h12c.55 0 1-.45 1-1s-.45-1-1-1H8c-.55 0-1 .45-1 1s.45 1 1 1zm0-6h12c.55 0 1-.45 1-1s-.45-1-1-1H8c-.55 0-1 .45-1 1s.45 1 1 1zM7 6c0 .55.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1H8c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn format_list_numbered_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M8 7h12c.55 0 1-.45 1-1s-.45-1-1-1H8c-.55 0-1 .45-1 1s.45 1 1 1zm12 10H8c-.55 0-1 .45-1 1s.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1zm0-6H8c-.55 0-1 .45-1 1s.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1zM4.5 16h-2c-.28 0-.5.22-.5.5s.22.5.5.5H4v.5h-.5c-.28 0-.5.22-.5.5s.22.5.5.5H4v.5H2.5c-.28 0-.5.22-.5.5s.22.5.5.5h2c.28 0 .5-.22.5-.5v-3c0-.28-.22-.5-.5-.5zm-2-11H3v2.5c0 .28.22.5.5.5s.5-.22.5-.5v-3c0-.28-.22-.5-.5-.5h-1c-.28 0-.5.22-.5.5s.22.5.5.5zm2 5h-2c-.28 0-.5.22-.5.5s.22.5.5.5h1.3l-1.68 1.96c-.08.09-.12.21-.12.32v.22c0 .28.22.5.5.5h2c.28 0 .5-.22.5-.5s-.22-.5-.5-.5H3.2l1.68-1.96c.08-.09.12-.21.12-.32v-.22c0-.28-.22-.5-.5-.5z",
            }
        }
    }
}

pub fn format_list_numbered_rtl_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M20.5 16h-2c-.28 0-.5.22-.5.5s.22.5.5.5H20v.5h-.5c-.28 0-.5.22-.5.5s.22.5.5.5h.5v.5h-1.5c-.28 0-.5.22-.5.5s.22.5.5.5h2c.28 0 .5-.22.5-.5v-3c0-.28-.22-.5-.5-.5zm-2-11h.5v2.5c0 .28.22.5.5.5s.5-.22.5-.5v-3c0-.28-.22-.5-.5-.5h-1c-.28 0-.5.22-.5.5s.22.5.5.5zm2.5 5.72v-.22c0-.28-.22-.5-.5-.5h-2c-.28 0-.5.22-.5.5s.22.5.5.5h1.3l-1.68 1.96c-.08.09-.12.21-.12.32v.22c0 .28.22.5.5.5h2c.28 0 .5-.22.5-.5s-.22-.5-.5-.5h-1.3l1.68-1.96c.08-.09.12-.21.12-.32zM15 5H3c-.55 0-1 .45-1 1s.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1zm0 12H3c-.55 0-1 .45-1 1s.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1zm0-6H3c-.55 0-1 .45-1 1s.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1z",
            }
        }
    }
}

pub fn format_paint_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M18 4V3c0-.55-.45-1-1-1H5c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h12c.55 0 1-.45 1-1V6h1v4h-9c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h2c.55 0 1-.45 1-1v-9h7c.55 0 1-.45 1-1V5c0-.55-.45-1-1-1h-2z",
            }
        }
    }
}

pub fn format_quote_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M7.17 17c.51 0 .98-.29 1.2-.74l1.42-2.84c.14-.28.21-.58.21-.89V8c0-.55-.45-1-1-1H5c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h2l-1.03 2.06c-.45.89.2 1.94 1.2 1.94zm10 0c.51 0 .98-.29 1.2-.74l1.42-2.84c.14-.28.21-.58.21-.89V8c0-.55-.45-1-1-1h-4c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h2l-1.03 2.06c-.45.89.2 1.94 1.2 1.94z",
            }
        }
    }
}

pub fn format_shapes_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M23 6V2c0-.55-.45-1-1-1h-4c-.55 0-1 .45-1 1v1H7V2c0-.55-.45-1-1-1H2c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h1v10H2c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-1h10v1c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-4c0-.55-.45-1-1-1h-1V7h1c.55 0 1-.45 1-1zM3 3h2v2H3V3zm2 18H3v-2h2v2zm12-2H7v-1c0-.55-.45-1-1-1H5V7h1c.55 0 1-.45 1-1V5h10v1c0 .55.45 1 1 1h1v10h-1c-.55 0-1 .45-1 1v1zm4 2h-2v-2h2v2zM19 5V3h2v2h-2zm-6.06 2.65c-.15-.39-.53-.65-.95-.65-.42 0-.8.26-.94.65l-2.77 7.33c-.19.49.17 1.02.7 1.02.32 0 .6-.2.71-.5l.55-1.5h3.49l.56 1.51c.11.29.39.49.71.49h.01c.53 0 .89-.53.71-1.02l-2.78-7.33zm-2.25 5.09L12 8.91l1.3 3.83h-2.61z",
            }
        }
    }
}

pub fn format_size_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M9 5.5c0 .83.67 1.5 1.5 1.5H14v10.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5V7h3.5c.83 0 1.5-.67 1.5-1.5S21.33 4 20.5 4h-10C9.67 4 9 4.67 9 5.5zM4.5 12H6v5.5c0 .83.67 1.5 1.5 1.5S9 18.33 9 17.5V12h1.5c.83 0 1.5-.67 1.5-1.5S11.33 9 10.5 9h-6C3.67 9 3 9.67 3 10.5S3.67 12 4.5 12z",
            }
        }
    }
}

pub fn format_strikethrough_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 19c1.1 0 2-.9 2-2v-1h-4v1c0 1.1.9 2 2 2zM5 5.5C5 6.33 5.67 7 6.5 7H10v3h4V7h3.5c.83 0 1.5-.67 1.5-1.5S18.33 4 17.5 4h-11C5.67 4 5 4.67 5 5.5zM4 14h16c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1z",
            }
        }
    }
}

pub fn format_textdirection_l_to_r_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M9 10v4c0 .55.45 1 1 1s1-.45 1-1V4h2v10c0 .55.45 1 1 1s1-.45 1-1V4h1c.55 0 1-.45 1-1s-.45-1-1-1H9.17C7.08 2 5.22 3.53 5.02 5.61 4.79 7.99 6.66 10 9 10zm11.65 7.65l-2.79-2.79c-.32-.32-.86-.1-.86.35V17H6c-.55 0-1 .45-1 1s.45 1 1 1h11v1.79c0 .45.54.67.85.35l2.79-2.79c.2-.19.2-.51.01-.7z",
            }
        }
    }
}

pub fn format_textdirection_r_to_l_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M10 10v4c0 .55.45 1 1 1s1-.45 1-1V4h2v10c0 .55.45 1 1 1s1-.45 1-1V4h1c.55 0 1-.45 1-1s-.45-1-1-1h-6.83C8.08 2 6.22 3.53 6.02 5.61 5.79 7.99 7.66 10 10 10zm-2 7v-1.79c0-.45-.54-.67-.85-.35l-2.79 2.79c-.2.2-.2.51 0 .71l2.79 2.79c.31.31.85.09.85-.36V19h11c.55 0 1-.45 1-1s-.45-1-1-1H8z",
            }
        }
    }
}

pub fn format_underlined_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M12.79 16.95c3.03-.39 5.21-3.11 5.21-6.16V4.25C18 3.56 17.44 3 16.75 3s-1.25.56-1.25 1.25v6.65c0 1.67-1.13 3.19-2.77 3.52-2.25.47-4.23-1.25-4.23-3.42V4.25C8.5 3.56 7.94 3 7.25 3S6 3.56 6 4.25V11c0 3.57 3.13 6.42 6.79 5.95zM5 20c0 .55.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1H6c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn functions_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M16.5 4H7.56C6.7 4 6 4.7 6 5.56c0 .28.12.55.32.74L12.5 12l-6.18 5.7c-.2.19-.32.46-.32.74C6 19.3 6.7 20 7.56 20h8.94c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5H11l3.59-3.59c.78-.78.78-2.05 0-2.83L11 7h5.5c.83 0 1.5-.67 1.5-1.5S17.33 4 16.5 4z",
            }
        }
    }
}

pub fn height_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                }
                path {
                    d: "M11,7h1.79c0.45,0,0.67-0.54,0.35-0.85l-2.79-2.78c-0.2-0.19-0.51-0.19-0.71,0L6.86,6.15C6.54,6.46,6.76,7,7.21,7H9v6H7.21 c-0.45,0-0.67,0.54-0.35,0.85l2.79,2.78c0.2,0.19,0.51,0.19,0.71,0l2.79-2.78c0.32-0.31,0.09-0.85-0.35-0.85H11V7z",
                }
            }
        }
    }
}

pub fn height_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                }
                path {
                    d: "M13,6.99h1.79c0.45,0,0.67-0.54,0.35-0.85l-2.79-2.78c-0.2-0.19-0.51-0.19-0.71,0L8.86,6.14C8.54,6.45,8.76,6.99,9.21,6.99 H11v10.02H9.21c-0.45,0-0.67,0.54-0.35,0.85l2.79,2.78c0.2,0.19,0.51,0.19,0.71,0l2.79-2.78c0.32-0.31,0.09-0.85-0.35-0.85H13V6.99 z",
                }
            }
        }
    }
}

pub fn hexagon_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M5.57,3.76l-3.14,5.5c-0.26,0.46-0.26,1.03,0,1.49l3.14,5.5C5.84,16.71,6.33,17,6.87,17h6.26c0.54,0,1.04-0.29,1.3-0.76 l3.14-5.5c0.26-0.46,0.26-1.03,0-1.49l-3.14-5.5C14.16,3.29,13.67,3,13.13,3H6.87C6.33,3,5.84,3.29,5.57,3.76z",
                }
            }
        }
    }
}

pub fn hexagon_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M16.05,3H7.95C7.24,3,6.58,3.38,6.22,4l-4.04,7c-0.36,0.62-0.36,1.38,0,2l4.04,7c0.36,0.62,1.02,1,1.73,1h8.09 c0.71,0,1.37-0.38,1.73-1l4.04-7c0.36-0.62,0.36-1.38,0-2l-4.04-7C17.42,3.38,16.76,3,16.05,3z",
                }
            }
        }
    }
}

pub fn highlight_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M6.29 14.29L9 17v4c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-4l2.71-2.71c.19-.19.29-.44.29-.71V10c0-.55-.45-1-1-1H7c-.55 0-1 .45-1 1v3.59c0 .26.11.52.29.7zM12 2c.55 0 1 .45 1 1v1c0 .55-.45 1-1 1s-1-.45-1-1V3c0-.55.45-1 1-1zM4.21 5.17c.39-.39 1.02-.39 1.42 0l.71.71c.39.39.39 1.02 0 1.41-.39.39-1.02.39-1.41 0l-.72-.71c-.39-.39-.39-1.02 0-1.41zm13.46.71l.71-.71c.39-.39 1.02-.39 1.41 0 .39.39.39 1.02 0 1.41l-.71.71c-.39.39-1.02.39-1.41 0-.39-.39-.39-1.02 0-1.41z",
            }
        }
    }
}

pub fn horizontal_distribute_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                height: "24",
                fill: "none",
                width: "24",
            }
            path {
                d: "M3,22L3,22c-0.55,0-1-0.45-1-1V3c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v18C4,21.55,3.55,22,3,22z M21,2L21,2 c-0.55,0-1,0.45-1,1v18c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V3C22,2.45,21.55,2,21,2z M12,7L12,7c-0.83,0-1.5,0.67-1.5,1.5v7 c0,0.83,0.67,1.5,1.5,1.5h0c0.83,0,1.5-0.67,1.5-1.5v-7C13.5,7.67,12.83,7,12,7z",
            }
        }
    }
}

pub fn horizontal_rule_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
                path {
                    d: "M16,11H4c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h12c0.55,0,1,0.45,1,1v0C17,10.55,16.55,11,16,11z",
                }
            }
        }
    }
}

pub fn horizontal_rule_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    fill_rule: "evenodd",
                    fill: "none",
                    height: "24",
                }
                g {
                    path {
                        fill_rule: "evenodd",
                        d: "M19,13H5c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h14c0.55,0,1,0.45,1,1v0 C20,12.55,19.55,13,19,13z",
                    }
                }
            }
        }
    }
}

pub fn insert_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM8 17c-.55 0-1-.45-1-1v-5c0-.55.45-1 1-1s1 .45 1 1v5c0 .55-.45 1-1 1zm4 0c-.55 0-1-.45-1-1V8c0-.55.45-1 1-1s1 .45 1 1v8c0 .55-.45 1-1 1zm4 0c-.55 0-1-.45-1-1v-2c0-.55.45-1 1-1s1 .45 1 1v2c0 .55-.45 1-1 1z",
            }
        }
    }
}

pub fn insert_chart_outlined_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M8 17c-.55 0-1-.45-1-1v-5c0-.55.45-1 1-1s1 .45 1 1v5c0 .55-.45 1-1 1zm4 0c-.55 0-1-.45-1-1V8c0-.55.45-1 1-1s1 .45 1 1v8c0 .55-.45 1-1 1zm4 0c-.55 0-1-.45-1-1v-2c0-.55.45-1 1-1s1 .45 1 1v2c0 .55-.45 1-1 1zm2 2H6c-.55 0-1-.45-1-1V6c0-.55.45-1 1-1h12c.55 0 1 .45 1 1v12c0 .55-.45 1-1 1zm1-16H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn insert_comment_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M20 2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4V4c0-1.1-.9-2-2-2zm-3 12H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1zm0-3H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1zm0-3H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1z",
            }
        }
    }
}

pub fn insert_drive_file_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M6 2c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8.83c0-.53-.21-1.04-.59-1.41l-4.83-4.83c-.37-.38-.88-.59-1.41-.59H6zm7 6V3.5L18.5 9H14c-.55 0-1-.45-1-1z",
            }
        }
    }
}

pub fn insert_emoticon_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M11.99,2C6.47,2,2,6.48,2,12s4.47,10,9.99,10C17.52,22,22,17.52,22,12S17.52,2,11.99,2z M8.5,8C9.33,8,10,8.67,10,9.5 S9.33,11,8.5,11S7,10.33,7,9.5S7.67,8,8.5,8z M16.75,14.75C15.8,16.39,14.03,17.5,12,17.5s-3.8-1.11-4.75-2.75 C7.06,14.42,7.31,14,7.69,14h8.62C16.7,14,16.94,14.42,16.75,14.75z M15.5,11c-0.83,0-1.5-0.67-1.5-1.5S14.67,8,15.5,8 S17,8.67,17,9.5S16.33,11,15.5,11z",
            }
        }
    }
}

pub fn insert_invitation_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M16 12h-3c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1h3c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1zm0-10v1H8V2c0-.55-.45-1-1-1s-1 .45-1 1v1H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2h-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm2 17H6c-.55 0-1-.45-1-1V8h14v10c0 .55-.45 1-1 1z",
            }
        }
    }
}

pub fn insert_link_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M3.96 11.38C4.24 9.91 5.62 8.9 7.12 8.9h2.93c.52 0 .95-.43.95-.95S10.57 7 10.05 7H7.22c-2.61 0-4.94 1.91-5.19 4.51C1.74 14.49 4.08 17 7 17h3.05c.52 0 .95-.43.95-.95s-.43-.95-.95-.95H7c-1.91 0-3.42-1.74-3.04-3.72zM9 13h6c.55 0 1-.45 1-1s-.45-1-1-1H9c-.55 0-1 .45-1 1s.45 1 1 1zm7.78-6h-2.83c-.52 0-.95.43-.95.95s.43.95.95.95h2.93c1.5 0 2.88 1.01 3.16 2.48.38 1.98-1.13 3.72-3.04 3.72h-3.05c-.52 0-.95.43-.95.95s.43.95.95.95H17c2.92 0 5.26-2.51 4.98-5.49-.25-2.6-2.59-4.51-5.2-4.51z",
            }
        }
    }
}

pub fn insert_page_break_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M4,16.5C4,17.33,4.67,18,5.5,18h9c0.83,0,1.5-0.67,1.5-1.5v-3H4V16.5z",
                    }
                    path {
                        d: "M15.56,5.56l-3.12-3.12C12.16,2.16,11.78,2,11.38,2H5.5C4.67,2,4,2.67,4,3.5V10h12V6.62C16,6.22,15.84,5.84,15.56,5.56z M11,6V3l4,4h-3C11.45,7,11,6.55,11,6z",
                    }
                    path {
                        d: "M12,11.75L12,11.75c0-0.41-0.34-0.75-0.75-0.75h-2.5C8.34,11,8,11.34,8,11.75v0c0,0.41,0.34,0.75,0.75,0.75h2.5 C11.66,12.5,12,12.16,12,11.75z",
                    }
                    path {
                        d: "M5.25,11h-2.5C2.34,11,2,11.34,2,11.75v0c0,0.41,0.34,0.75,0.75,0.75h2.5C5.66,12.5,6,12.16,6,11.75v0 C6,11.34,5.66,11,5.25,11z",
                    }
                    path {
                        d: "M14,11.75L14,11.75c0,0.41,0.34,0.75,0.75,0.75h2.5c0.41,0,0.75-0.34,0.75-0.75v0c0-0.41-0.34-0.75-0.75-0.75h-2.5 C14.34,11,14,11.34,14,11.75z",
                    }
                }
            }
        }
    }
}

pub fn insert_page_break_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M4,20c0,1.1,0.89,2,1.99,2H18c1.1,0,2-0.9,2-2v-3H4L4,20z",
                    }
                    path {
                        d: "M19.41,7.41l-4.83-4.83C14.21,2.21,13.7,2,13.17,2H6C4.9,2,4.01,2.89,4.01,3.99l0,7.01H20V8.83 C20,8.3,19.79,7.79,19.41,7.41z M13,8V3.5L18.5,9H14C13.45,9,13,8.55,13,8z",
                    }
                    path {
                        d: "M15,14L15,14c0-0.55-0.45-1-1-1h-4c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h4C14.55,15,15,14.55,15,14z",
                    }
                    path {
                        d: "M17,14L17,14c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-4C17.45,13,17,13.45,17,14z",
                    }
                    path {
                        d: "M6,13H2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v0C7,13.45,6.55,13,6,13z",
                    }
                }
            }
        }
    }
}

pub fn insert_photo_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.9 13.98l2.1 2.53 3.1-3.99c.2-.26.6-.26.8.01l3.51 4.68c.25.33.01.8-.4.8H6.02c-.42 0-.65-.48-.39-.81L8.12 14c.19-.26.57-.27.78-.02z",
            }
        }
    }
}

pub fn linear_scale_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                fill: "none",
                height: "24",
                width: "24",
            }
            path {
                d: "M17,7c-2.41,0-4.43,1.72-4.9,4H6.79C6.4,10.12,5.52,9.5,4.5,9.5C3.12,9.5,2,10.62,2,12s1.12,2.5,2.5,2.5 c1.02,0,1.9-0.62,2.29-1.5h5.31c0.46,2.28,2.48,4,4.9,4c2.76,0,5-2.24,5-5S19.76,7,17,7z M17,15c-1.65,0-3-1.35-3-3s1.35-3,3-3 s3,1.35,3,3S18.65,15,17,15z",
            }
        }
    }
}

pub fn line_axis_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M14.68,10.61l2.82-3.17c0.28-0.31,0.25-0.78-0.06-1.06l0,0c-0.31-0.28-0.78-0.25-1.06,0.06l-2.76,3.11L9.06,5.05 C8.47,4.47,7.53,4.47,6.95,5.06L2.53,9.48c-0.29,0.29-0.29,0.77,0,1.06l0,0c0.29,0.29,0.77,0.29,1.06,0L8,6.12l4.62,4.56 l-1.35,1.52L9.06,9.99c-0.59-0.59-1.54-0.59-2.12,0l-4.41,4.42c-0.29,0.29-0.29,0.77,0,1.06l0,0c0.29,0.29,0.77,0.29,1.06,0 L8,11.05l2.21,2.21c0.61,0.61,1.61,0.58,2.18-0.06l1.3-1.46l2.78,2.75c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06 L14.68,10.61z",
                }
            }
        }
    }
}

pub fn line_axis_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M21.34,6.77L21.34,6.77c-0.4-0.4-1.07-0.39-1.45,0.04l-3.33,3.74l-5.65-5.24C10.12,4.58,8.9,4.6,8.14,5.36L2.7,10.81 c-0.39,0.39-0.39,1.02,0,1.41l0.09,0.09c0.39,0.39,1.02,0.39,1.41,0l5.44-5.45l5.59,5.19l-1.73,1.95l-2.58-2.58 c-0.78-0.78-2.05-0.78-2.83,0L2.7,16.8c-0.39,0.39-0.39,1.02,0,1.41L2.8,18.3c0.39,0.39,1.02,0.39,1.41,0l5.3-5.3l2.5,2.5 c0.81,0.81,2.14,0.77,2.91-0.09l1.78-2.01l3.19,2.96c0.39,0.36,1,0.35,1.38-0.03l0.01-0.01c0.4-0.4,0.39-1.05-0.03-1.43l-3.22-2.99 l3.35-3.77C21.73,7.74,21.71,7.14,21.34,6.77z",
                }
            }
        }
    }
}

pub fn margin_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5z M9,8c0,0.55-0.45,1-1,1S7,8.55,7,8 s0.45-1,1-1S9,7.45,9,8z M13,8c0,0.55-0.45,1-1,1s-1-0.45-1-1s0.45-1,1-1S13,7.45,13,8z M17,8c0,0.55-0.45,1-1,1 c-0.55,0-1-0.45-1-1s0.45-1,1-1C16.55,7,17,7.45,17,8z M17,12c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1s0.45-1,1-1 C16.55,11,17,11.45,17,12z M13,12c0,0.55-0.45,1-1,1s-1-0.45-1-1s0.45-1,1-1S13,11.45,13,12z M9,12c0,0.55-0.45,1-1,1s-1-0.45-1-1 s0.45-1,1-1S9,11.45,9,12z",
                }
            }
        }
    }
}

pub fn merge_type_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M17.7 19.7c.39-.39.39-1.02 0-1.41l-2.7-2.7L13.59 17l2.7 2.7c.39.39 1.03.39 1.41 0zM8.71 8H11v5.59l-4.71 4.7c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0l4.71-4.7c.38-.38.59-.88.59-1.41V8h2.29c.45 0 .67-.54.35-.85l-3.29-3.29c-.2-.2-.51-.2-.71 0L8.35 7.15c-.31.31-.09.85.36.85z",
            }
        }
    }
}

pub fn mode_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M3 17.46v3.04c0 .28.22.5.5.5h3.04c.13 0 .26-.05.35-.15L17.81 9.94l-3.75-3.75L3.15 17.1c-.1.1-.15.22-.15.36zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z",
            }
        }
    }
}

pub fn mode_comment_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M22 4c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4V4z",
            }
        }
    }
}

pub fn mode_edit_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M3,17.46l0,3.04C3,20.78,3.22,21,3.5,21h3.04c0.13,0,0.26-0.05,0.35-0.15L17.81,9.94l-3.75-3.75L3.15,17.1 C3.05,17.2,3,17.32,3,17.46z",
                        }
                    }
                    g {
                        path {
                            d: "M20.71,5.63l-2.34-2.34c-0.39-0.39-1.02-0.39-1.41,0l-1.83,1.83l3.75,3.75l1.83-1.83C21.1,6.65,21.1,6.02,20.71,5.63z",
                        }
                    }
                }
            }
        }
    }
}

pub fn mode_edit_outline_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    polygon {
                        points: "11.62,5.38 3,14 3,17 6,17 14.62,8.38",
                    }
                    path {
                        d: "M16.71,4.88l-1.59-1.59c-0.39-0.39-1.02-0.39-1.41,0l-1.03,1.02l3,3l1.03-1.02C17.1,5.9,17.1,5.27,16.71,4.88z",
                    }
                }
            }
        }
    }
}

pub fn mode_edit_outline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M3,17.46l0,3.04C3,20.78,3.22,21,3.5,21h3.04c0.13,0,0.26-0.05,0.35-0.15L17.81,9.94l-3.75-3.75L3.15,17.1 C3.05,17.2,3,17.32,3,17.46z",
                        }
                    }
                    g {
                        path {
                            d: "M20.71,5.63l-2.34-2.34c-0.39-0.39-1.02-0.39-1.41,0l-1.83,1.83l3.75,3.75l1.83-1.83C21.1,6.65,21.1,6.02,20.71,5.63z",
                        }
                    }
                }
            }
        }
    }
}

pub fn monetization_on_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1.41 16.09v.58c0 .73-.6 1.33-1.33 1.33h-.01c-.73 0-1.33-.6-1.33-1.33v-.6c-1.33-.28-2.51-1.01-3.01-2.24-.23-.55.2-1.16.8-1.16h.24c.37 0 .67.25.81.6.29.75 1.05 1.27 2.51 1.27 1.96 0 2.4-.98 2.4-1.59 0-.83-.44-1.61-2.67-2.14-2.48-.6-4.18-1.62-4.18-3.67 0-1.72 1.39-2.84 3.11-3.21v-.6c0-.73.6-1.33 1.33-1.33h.01c.73 0 1.33.6 1.33 1.33v.62c1.38.34 2.25 1.2 2.63 2.26.2.55-.22 1.13-.81 1.13h-.26c-.37 0-.67-.26-.77-.62-.23-.76-.86-1.25-2.12-1.25-1.5 0-2.4.68-2.4 1.64 0 .84.65 1.39 2.67 1.91s4.18 1.39 4.18 3.91c-.02 1.83-1.39 2.83-3.13 3.16z",
            }
        }
    }
}

pub fn money_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12.5 6.9c1.42 0 2.13.54 2.39 1.4.13.43.56.7 1.01.7h.06c.7 0 1.22-.71.97-1.36-.44-1.15-1.41-2.08-2.93-2.45V4.5c0-.83-.67-1.5-1.5-1.5S11 3.67 11 4.5v.66c-.39.08-.75.21-1.1.36l1.51 1.51c.32-.08.69-.13 1.09-.13zM4.77 4.62c-.39.39-.39 1.02 0 1.41L7.5 8.77c0 2.08 1.56 3.22 3.91 3.91l3.51 3.51c-.34.49-1.05.91-2.42.91-1.65 0-2.5-.59-2.83-1.43-.15-.39-.49-.67-.9-.67H8.6c-.72 0-1.24.74-.95 1.39.59 1.33 1.89 2.12 3.36 2.44v.67c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5v-.65c.96-.18 1.83-.55 2.46-1.12l1.51 1.51c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L6.18 4.62c-.39-.39-1.02-.39-1.41 0z",
            }
        }
    }
}

pub fn money_off_csred_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M12.5 6.9c1.42 0 2.13.54 2.39 1.4.13.43.56.7 1.01.7h.06c.7 0 1.22-.71.97-1.36-.44-1.15-1.41-2.08-2.93-2.45V4.5c0-.83-.67-1.5-1.5-1.5S11 3.67 11 4.5v.66c-.39.08-.75.21-1.1.36l1.51 1.51c.32-.08.69-.13 1.09-.13zM4.77 4.62c-.39.39-.39 1.02 0 1.41L7.5 8.77c0 2.08 1.56 3.22 3.91 3.91l3.51 3.51c-.34.49-1.05.91-2.42.91-1.65 0-2.5-.59-2.83-1.43-.15-.39-.49-.67-.9-.67H8.6c-.72 0-1.24.74-.95 1.39.59 1.33 1.89 2.12 3.36 2.44v.67c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5v-.65c.96-.18 1.83-.55 2.46-1.12l1.51 1.51c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L6.18 4.62c-.39-.39-1.02-.39-1.41 0z",
            }
        }
    }
}

pub fn move_down_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M11,15v-3c0-0.55,0.45-1,1-1h5c0.55,0,1,0.45,1,1v3c0,0.55-0.45,1-1,1h-5C11.45,16,11,15.55,11,15z",
                    }
                    path {
                        d: "M11,5v3c0,0.55,0.45,1,1,1h5c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h-5C11.45,4,11,4.45,11,5z M12.5,5.5h4v2h-4V5.5z",
                    }
                    path {
                        d: "M6.1,11.41L6.1,11.41c0.29-0.29,0.77-0.29,1.06,0l1.99,1.99c0.2,0.2,0.2,0.51,0,0.71l-2,2c-0.29,0.29-0.77,0.29-1.06,0h0 c-0.29-0.29-0.29-0.77,0-1.06l0.53-0.53V14.5l-0.14,0c-2.74,0-5.19-2-5.45-4.73C0.73,6.63,3.18,4,6.25,4h2.5 C9.16,4,9.5,4.34,9.5,4.75v0c0,0.41-0.34,0.75-0.75,0.75l-2.29,0c-2.03,0-3.85,1.52-3.96,3.55C2.39,11.21,4.11,13,6.25,13h0.37 v-0.01L6.1,12.47C5.81,12.17,5.81,11.7,6.1,11.41z",
                    }
                }
            }
        }
    }
}

pub fn move_down_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M3.01,10.72c-0.14,2.57,1.66,4.73,4.07,5.18l-0.79-0.79c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0 l2.59,2.59c0.39,0.39,0.39,1.02,0,1.41L7.71,20.3c-0.39,0.39-1.02,0.39-1.41,0h0c-0.39-0.39-0.39-1.02,0-1.41l0.88-0.88l0-0.06 c-3.64-0.43-6.43-3.65-6.15-7.47C1.29,6.78,4.55,4,8.26,4L10,4c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1L8.22,6 C5.52,6,3.15,8.04,3.01,10.72z",
                    }
                    path {
                        d: "M15,11h5c1.1,0,2-0.9,2-2V6c0-1.1-0.9-2-2-2h-5c-1.1,0-2,0.9-2,2v3C13,10.1,13.9,11,15,11z M20,9h-5V6h5V9z",
                    }
                    path {
                        d: "M20,20h-5c-1.1,0-2-0.9-2-2v-3c0-1.1,0.9-2,2-2h5c1.1,0,2,0.9,2,2v3C22,19.1,21.1,20,20,20z",
                    }
                }
            }
        }
    }
}

pub fn move_up_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M11,5v3c0,0.55,0.45,1,1,1h5c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h-5C11.45,4,11,4.45,11,5z",
                    }
                    path {
                        d: "M12,16h5c0.55,0,1-0.45,1-1v-3c0-0.55-0.45-1-1-1h-5c-0.55,0-1,0.45-1,1v3C11,15.55,11.45,16,12,16z M12.5,12.5h4v2h-4 V12.5z",
                    }
                    path {
                        d: "M6.1,8.59L6.1,8.59c0.29,0.29,0.77,0.29,1.06,0l1.99-1.99c0.2-0.2,0.2-0.51,0-0.71l-2-2c-0.29-0.29-0.77-0.29-1.06,0l0,0 C5.8,4.2,5.8,4.67,6.09,4.96l0.53,0.53V5.5l-0.14,0c-2.74,0-5.19,2-5.45,4.73C0.73,13.37,3.18,16,6.25,16h2.5 c0.41,0,0.75-0.34,0.75-0.75v0c0-0.41-0.34-0.75-0.75-0.75l-2.29,0c-2.03,0-3.85-1.52-3.96-3.55C2.39,8.79,4.11,7,6.25,7h0.37 v0.01L6.1,7.53C5.81,7.83,5.81,8.3,6.1,8.59z",
                    }
                }
            }
        }
    }
}

pub fn move_up_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M3.01,13.28c-0.14-2.57,1.66-4.73,4.07-5.18L6.29,8.88c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0 l2.59-2.59c0.39-0.39,0.39-1.02,0-1.41L7.71,3.7c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l0.88,0.88l0,0.06 c-3.64,0.43-6.43,3.65-6.15,7.47C1.29,17.22,4.55,20,8.26,20H10c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H8.22 C5.52,18,3.15,15.96,3.01,13.28z",
                    }
                    path {
                        d: "M13,15v3c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2v-3c0-1.1-0.9-2-2-2h-5C13.9,13,13,13.9,13,15z M20,18h-5v-3h5V18z",
                    }
                    path {
                        d: "M20,4h-5c-1.1,0-2,0.9-2,2v3c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z",
                    }
                }
            }
        }
    }
}

pub fn multiline_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M21.36 6.28l-.06-.06c-.39-.39-1.03-.37-1.39.04l-2.18 2.45C15.68 6.4 12.83 5 9.61 5c-2.5 0-4.83.87-6.75 2.3-.47.35-.52 1.04-.11 1.45l.06.06c.33.33.86.39 1.23.11C5.63 7.72 7.54 7 9.61 7c2.74 0 5.09 1.26 6.77 3.24l-2.88 3.24-3.29-3.29c-.39-.39-1.02-.39-1.41 0l-6.12 6.13c-.37.37-.37.98 0 1.35l.15.15c.37.37.98.37 1.35 0l5.32-5.33 3.25 3.25c.41.41 1.07.39 1.45-.04l3.35-3.76c.62 1.12 1.08 2.39 1.32 3.73.08.47.47.82.95.82h.09c.6 0 1.05-.55.94-1.14-.32-1.85-.98-3.54-1.89-5L21.4 7.6c.34-.38.32-.96-.04-1.32z",
            }
        }
    }
}

pub fn notes_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M20 11H4c-.55 0-1 .45-1 1s.45 1 1 1h16c.55 0 1-.45 1-1s-.45-1-1-1zM4 18h10c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zM20 6H4c-.55 0-1 .45-1 1v.01c0 .55.45 1 1 1h16c.55 0 1-.45 1-1V7c0-.55-.45-1-1-1z",
            }
        }
    }
}

pub fn numbers_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M16.76,7.95l0.01-0.04C16.89,7.44,16.54,7,16.07,7h-2.32l0.52-2.1C14.39,4.44,14.04,4,13.57,4h0 c-0.33,0-0.62,0.23-0.71,0.55L12.25,7h-3l0.52-2.1C9.89,4.44,9.54,4,9.07,4h0C8.73,4,8.44,4.23,8.36,4.55L7.75,7H4.94 C4.61,7,4.32,7.23,4.24,7.55L4.23,7.6C4.11,8.06,4.46,8.5,4.93,8.5h2.44L6.5,12H3.94c-0.33,0-0.62,0.23-0.71,0.55L3.23,12.6 c-0.11,0.46,0.23,0.9,0.71,0.9h2.19l-0.4,1.6C5.61,15.56,5.96,16,6.43,16h0c0.33,0,0.62-0.23,0.71-0.55l0.49-1.95h3l-0.4,1.6 c-0.11,0.46,0.23,0.9,0.71,0.9h0c0.33,0,0.62-0.23,0.71-0.55l0.49-1.95h2.93c0.33,0,0.62-0.23,0.71-0.55l0.01-0.04 c0.11-0.46-0.23-0.9-0.71-0.9H12.5l0.88-3.5h2.68C16.39,8.5,16.68,8.27,16.76,7.95z M11,12H8l0.88-3.5h3L11,12z",
                }
            }
        }
    }
}

pub fn numbers_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M20.68,9.27l0.01-0.06C20.85,8.59,20.39,8,19.76,8H17l0.7-2.79C17.85,4.59,17.39,4,16.76,4h0c-0.45,0-0.83,0.3-0.94,0.73 L15,8h-4l0.7-2.79C11.85,4.59,11.39,4,10.76,4h0c-0.45,0-0.83,0.3-0.94,0.73L9,8H5.76C5.31,8,4.92,8.3,4.82,8.73L4.8,8.79 C4.65,9.41,5.11,10,5.74,10H8.5l-1,4H4.26c-0.45,0-0.83,0.3-0.94,0.73L3.3,14.79C3.15,15.41,3.61,16,4.24,16H7l-0.7,2.79 C6.15,19.41,6.61,20,7.24,20h0c0.45,0,0.83-0.3,0.94-0.73L9,16h4l-0.7,2.79C12.15,19.41,12.61,20,13.24,20h0 c0.45,0,0.83-0.3,0.94-0.73L15,16h3.24c0.45,0,0.83-0.3,0.94-0.73l0.01-0.06c0.15-0.61-0.31-1.21-0.94-1.21H15.5l1-4h3.24 C20.19,10,20.58,9.7,20.68,9.27z M13.5,14h-4l1-4h4L13.5,14z",
                }
            }
        }
    }
}

pub fn padding_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5z M9,8c0,0.55-0.45,1-1,1S7,8.55,7,8 s0.45-1,1-1S9,7.45,9,8z M13,8c0,0.55-0.45,1-1,1s-1-0.45-1-1s0.45-1,1-1S13,7.45,13,8z M17,8c0,0.55-0.45,1-1,1 c-0.55,0-1-0.45-1-1s0.45-1,1-1C16.55,7,17,7.45,17,8z",
                }
            }
        }
    }
}

pub fn pentagon_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M2.94,6.85C2.39,7.23,2.15,7.94,2.38,8.58l2.57,7.41C5.16,16.6,5.73,17,6.37,17h7.27c0.64,0,1.21-0.4,1.42-1.01l2.57-7.41 c0.22-0.64-0.01-1.35-0.57-1.73l-6.21-4.27c-0.51-0.35-1.19-0.35-1.7,0L2.94,6.85z",
                }
            }
        }
    }
}

pub fn pentagon_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M2.47,10.42l3.07,9.22C5.82,20.45,6.58,21,7.44,21h9.12c0.86,0,1.63-0.55,1.9-1.37l3.07-9.22 c0.28-0.84-0.03-1.76-0.75-2.27L13.15,2.8c-0.69-0.48-1.61-0.48-2.29,0L3.22,8.14C2.5,8.65,2.19,9.58,2.47,10.42z",
                }
            }
        }
    }
}

pub fn pie_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M11 3.18v17.64c0 .64-.59 1.12-1.21.98C5.32 20.8 2 16.79 2 12s3.32-8.8 7.79-9.8c.62-.14 1.21.34 1.21.98zm2.03 0v6.81c0 .55.45 1 1 1h6.79c.64 0 1.12-.59.98-1.22-.85-3.76-3.8-6.72-7.55-7.57-.63-.14-1.22.34-1.22.98zm0 10.83v6.81c0 .64.59 1.12 1.22.98 3.76-.85 6.71-3.82 7.56-7.58.14-.62-.35-1.22-.98-1.22h-6.79c-.56.01-1.01.46-1.01 1.01z",
            }
        }
    }
}

pub fn pie_chart_outline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zm1 2.07c3.61.45 6.48 3.33 6.93 6.93H14c-.55 0-1-.45-1-1V4.07zM4 12c0-4.06 3.07-7.44 7-7.93v15.87c-3.93-.5-7-3.88-7-7.94zm9 7.93V14c0-.55.45-1 1-1h5.93c-.45 3.61-3.32 6.48-6.93 6.93z",
            }
        }
    }
}

pub fn polyline_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M8.5,3v2.5c0,0.08,0.01,0.15,0.02,0.22L6.16,8.5H4c-0.55,0-1,0.45-1,1V12c0,0.55,0.45,1,1,1h2.5c0.03,0,0.06,0,0.09,0 l5.91,3.2V17c0,0.55,0.45,1,1,1H16c0.55,0,1-0.45,1-1v-2.5c0-0.55-0.45-1-1-1h-2.5c-0.55,0-1,0.44-1,0.99l-5-2.71V9.5 c0-0.08-0.01-0.16-0.03-0.23L9.83,6.5H12c0.55,0,1-0.45,1-1V3c0-0.55-0.45-1-1-1H9.5C8.95,2,8.5,2.45,8.5,3z",
                }
            }
        }
    }
}

pub fn polyline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M10.04,6.85L7.3,10H4.5C3.67,10,3,10.67,3,11.5v3C3,15.33,3.67,16,4.5,16h3c0.14,0,0.27-0.02,0.39-0.05L15,19.5v1 c0,0.83,0.67,1.5,1.5,1.5h3c0.83,0,1.5-0.67,1.5-1.5v-3c0-0.83-0.67-1.5-1.5-1.5h-3c-0.75,0-1.37,0.55-1.48,1.27L9,14.26V11.5 c0-0.12-0.01-0.24-0.04-0.36L11.7,8h2.8C15.33,8,16,7.33,16,6.5v-3C16,2.67,15.33,2,14.5,2h-3C10.67,2,10,2.67,10,3.5v3 C10,6.62,10.01,6.74,10.04,6.85z",
                }
            }
        }
    }
}

pub fn post_add_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                }
                g {
                    path {
                        d: "M7,8.5C7,8.78,7.22,9,7.5,9h4C11.78,9,12,8.78,12,8.5C12,8.22,11.78,8,11.5,8h-4C7.22,8,7,8.22,7,8.5z",
                    }
                    path {
                        d: "M11.5,10h-4C7.22,10,7,10.22,7,10.5C7,10.78,7.22,11,7.5,11h4c0.28,0,0.5-0.22,0.5-0.5C12,10.22,11.78,10,11.5,10z",
                    }
                    path {
                        d: "M11.5,12h-4C7.22,12,7,12.22,7,12.5C7,12.78,7.22,13,7.5,13h4c0.28,0,0.5-0.22,0.5-0.5C12,12.22,11.78,12,11.5,12z",
                    }
                    path {
                        d: "M16.5,5H15V3.5C15,3.22,14.78,3,14.5,3S14,3.22,14,3.5V5h-1.5C12.22,5,12,5.22,12,5.5C12,5.78,12.22,6,12.5,6H14v1.5 C14,7.78,14.22,8,14.5,8S15,7.78,15,7.5V6h1.5C16.78,6,17,5.78,17,5.5C17,5.22,16.78,5,16.5,5z",
                    }
                    path {
                        d: "M14.5,9C14.22,9,14,9.22,14,9.5v5c0,0.28-0.22,0.5-0.5,0.5h-8C5.22,15,5,14.78,5,14.5v-8C5,6.22,5.22,6,5.5,6h5 C10.78,6,11,5.78,11,5.5C11,5.22,10.78,5,10.5,5H5C4.45,5,4,5.45,4,6V15c0,0.55,0.45,1,1,1H14c0.55,0,1-0.45,1-1V9.5 C15,9.22,14.78,9,14.5,9z",
                    }
                }
            }
        }
    }
}

pub fn post_add_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                }
                g {
                    path {
                        d: "M18,12c-0.55,0-1,0.45-1,1v5.22c0,0.55-0.45,1-1,1H6c-0.55,0-1-0.45-1-1V8c0-0.55,0.45-1,1-1h5c0.55,0,1-0.45,1-1 c0-0.55-0.45-1-1-1H5C3.9,5,3,5.9,3,7v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-6C19,12.45,18.55,12,18,12z",
                    }
                    path {
                        d: "M21.02,5H19V2.98C19,2.44,18.56,2,18.02,2h-0.03C17.44,2,17,2.44,17,2.98V5h-2.01C14.45,5,14.01,5.44,14,5.98 c0,0.01,0,0.02,0,0.03C14,6.56,14.44,7,14.99,7H17v2.01c0,0.54,0.44,0.99,0.99,0.98c0.01,0,0.02,0,0.03,0 c0.54,0,0.98-0.44,0.98-0.98V7h2.02C21.56,7,22,6.56,22,6.02V5.98C22,5.44,21.56,5,21.02,5z",
                    }
                    path {
                        d: "M14,9H8c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1C15,9.45,14.55,9,14,9z",
                    }
                    path {
                        d: "M14,12H8c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1C15,12.45,14.55,12,14,12z",
                    }
                    path {
                        d: "M14,15H8c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1C15,15.45,14.55,15,14,15z",
                    }
                }
            }
        }
    }
}

pub fn publish_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M5 5c0 .55.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1H6c-.55 0-1 .45-1 1zm2.41 9H9v5c0 .55.45 1 1 1h4c.55 0 1-.45 1-1v-5h1.59c.89 0 1.34-1.08.71-1.71L12.71 7.7c-.39-.39-1.02-.39-1.41 0l-4.59 4.59c-.63.63-.19 1.71.7 1.71z",
            }
        }
    }
}

pub fn query_stats_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                height: "20",
                fill: "none",
                width: "20",
            }
            path {
                d: "M16.44,15.38c0.51-0.79,0.72-1.79,0.42-2.86c-0.35-1.24-1.41-2.22-2.68-2.46c-2.47-0.47-4.59,1.65-4.12,4.12 c0.24,1.27,1.21,2.33,2.46,2.68c1.07,0.3,2.07,0.09,2.86-0.42l2.03,2.03c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06 L16.44,15.38z M13.5,15.5c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2S14.6,15.5,13.5,15.5z M18.41,2.45L18.41,2.45 c0.33,0.23,0.41,0.69,0.19,1.02l-3.49,5.29h0C14.61,8.59,14.07,8.5,13.5,8.5l3.85-5.85C17.59,2.3,18.07,2.21,18.41,2.45z M13.5,8.5 c-0.58,0-1.13,0.1-1.65,0.28l0,0l-0.78-1.1l-2.67,4.2c-0.36,0.57-1.18,0.62-1.61,0.1l-1.6-1.92l-2.54,4.13 c-0.23,0.37-0.72,0.47-1.07,0.22l0,0c-0.32-0.23-0.41-0.67-0.2-1l2.9-4.72C4.63,8.1,5.45,8.04,5.89,8.57L7.5,10.5l2.7-4.25 c0.38-0.6,1.25-0.62,1.66-0.04L13.5,8.5z",
            }
        }
    }
}

pub fn query_stats_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                width: "24",
                fill: "none",
                height: "24",
            }
            g {
                path {
                    d: "M19.88,18.47c0.48-0.77,0.75-1.67,0.69-2.66c-0.13-2.15-1.84-3.97-3.97-4.2c-2.72-0.3-5.02,1.81-5.02,4.47 c0,2.49,2.01,4.5,4.49,4.5c0.88,0,1.7-0.26,2.39-0.7l2.41,2.41c0.39,0.39,1.03,0.39,1.42,0v0c0.39-0.39,0.39-1.03,0-1.42 L19.88,18.47z M16.08,18.58c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5 C18.58,17.46,17.46,18.58,16.08,18.58z M15.72,10.08c-0.74,0.02-1.45,0.18-2.1,0.45l-0.55-0.83l-3.08,5.01 c-0.36,0.58-1.17,0.64-1.61,0.13l-2.12-2.47l-3.06,4.9c-0.31,0.49-0.97,0.62-1.44,0.28l0,0c-0.42-0.31-0.54-0.89-0.26-1.34 l3.78-6.05c0.36-0.57,1.17-0.63,1.61-0.12L9,12.5l3.18-5.17c0.38-0.62,1.28-0.64,1.68-0.03L15.72,10.08z M18.31,10.58 c-0.64-0.28-1.33-0.45-2.05-0.49L20.8,2.9c0.31-0.49,0.97-0.61,1.43-0.27l0,0c0.43,0.31,0.54,0.9,0.26,1.34L18.31,10.58z",
                }
            }
        }
    }
}

pub fn rectangle_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M16.5,16h-13C2.67,16,2,15.33,2,14.5v-9C2,4.67,2.67,4,3.5,4h13C17.33,4,18,4.67,18,5.5v9C18,15.33,17.33,16,16.5,16z",
                }
            }
        }
    }
}

pub fn rectangle_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6c0-1.1-0.9-2-2-2H4C2.9,4,2,4.9,2,6z",
                    }
                }
            }
        }
    }
}

pub fn scatter_plot_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            circle {
                cx: "7",
                cy: "14",
                r: "3",
            }
            circle {
                cy: "6",
                cx: "11",
                r: "3",
            }
            circle {
                cy: "17.6",
                cx: "16.6",
                r: "3",
            }
        }
    }
}

pub fn schema_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                width: "20",
                height: "20",
                fill: "none",
            }
            path {
                d: "M11.5,8.75v0.5h-2v-0.5c0-0.55-0.45-1-1-1h-1V6.5h1c0.55,0,1-0.45,1-1V3c0-0.55-0.45-1-1-1H5C4.45,2,4,2.45,4,3v2.5 c0,0.55,0.45,1,1,1h1v1.25H5c-0.55,0-1,0.45-1,1v2.5c0,0.55,0.45,1,1,1h1V14H5c-0.55,0-1,0.45-1,1v2.5c0,0.55,0.45,1,1,1h3.5 c0.55,0,1-0.45,1-1V15c0-0.55-0.45-1-1-1h-1v-1.75h1c0.55,0,1-0.45,1-1v-0.5h2v0.5c0,0.55,0.45,1,1,1H16c0.55,0,1-0.45,1-1v-2.5 c0-0.55-0.45-1-1-1h-3.5C11.95,7.75,11.5,8.2,11.5,8.75z",
            }
        }
    }
}

pub fn schema_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                fill: "none",
                height: "24",
                width: "24",
            }
            path {
                d: "M14,10.5V11h-3v-0.5C11,9.67,10.33,9,9.5,9h-1V7h1C10.33,7,11,6.33,11,5.5v-3C11,1.67,10.33,1,9.5,1h-4C4.67,1,4,1.67,4,2.5 v3C4,6.33,4.67,7,5.5,7h1v2h-1C4.67,9,4,9.67,4,10.5v3C4,14.33,4.67,15,5.5,15h1v2h-1C4.67,17,4,17.67,4,18.5v3 C4,22.33,4.67,23,5.5,23h4c0.83,0,1.5-0.67,1.5-1.5v-3c0-0.83-0.67-1.5-1.5-1.5h-1v-2h1c0.83,0,1.5-0.67,1.5-1.5V13h3v0.5 c0,0.83,0.67,1.5,1.5,1.5h4c0.83,0,1.5-0.67,1.5-1.5v-3C21,9.67,20.33,9,19.5,9h-4C14.67,9,14,9.67,14,10.5z",
            }
        }
    }
}

pub fn score_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 2.75c0-.41.34-.75.75-.75s.75.34.75.75V8l1.79-2.69c.13-.19.35-.31.59-.31.56 0 .9.63.59 1.1L15.2 8l1.27 1.9c.31.47-.02 1.1-.59 1.1-.24 0-.46-.12-.59-.31L13.5 8v2.25c0 .41-.34.75-.75.75s-.75-.34-.75-.75v-4.5zm-5 2.5c0-.55.45-1 1-1h1.5V6.5H7.75c-.41 0-.75-.34-.75-.75S7.34 5 7.75 5H10c.55 0 1 .45 1 1v1.75c0 .55-.45 1-1 1H8.5v.75h1.75c.41 0 .75.34.75.75s-.34.75-.75.75H8c-.55 0-1-.45-1-1V8.25zm11.74 5.01l-5.03 5.03c-.39.39-1.02.39-1.41 0L9 15l-2.49 2.49c-.56.56-1.51.16-1.51-.62 0-.23.09-.46.26-.62l3.03-3.03c.39-.39 1.02-.39 1.41 0L13 16.5l4.49-4.49c.56-.56 1.51-.16 1.51.62 0 .24-.09.46-.26.63z",
            }
        }
    }
}

pub fn shape_line_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M5,9c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4C2.79,1,1,2.79,1,5C1,7.21,2.79,9,5,9z",
                    }
                    path {
                        d: "M17.5,12h-4c-0.83,0-1.5,0.67-1.5,1.5v4c0,0.83,0.67,1.5,1.5,1.5h4c0.83,0,1.5-0.67,1.5-1.5v-4C19,12.67,18.33,12,17.5,12 z",
                    }
                    path {
                        d: "M14.35,6.71C14.7,6.89,15.08,7,15.5,7C16.88,7,18,5.88,18,4.5S16.88,2,15.5,2S13,3.12,13,4.5c0,0.42,0.11,0.8,0.29,1.15 l-7.64,7.64C5.3,13.11,4.92,13,4.5,13C3.12,13,2,14.12,2,15.5S3.12,18,4.5,18S7,16.88,7,15.5c0-0.42-0.11-0.8-0.29-1.15 L14.35,6.71z",
                    }
                }
            }
        }
    }
}

pub fn shape_line_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M6,11c2.76,0,5-2.24,5-5S8.76,1,6,1S1,3.24,1,6S3.24,11,6,11z",
                    }
                    path {
                        d: "M21,14h-5c-1.1,0-2,0.9-2,2v5c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2v-5C23,14.9,22.1,14,21,14z",
                    }
                    path {
                        d: "M17.71,7.7C18.11,7.89,18.54,8,19,8c1.65,0,3-1.35,3-3s-1.35-3-3-3s-3,1.35-3,3c0,0.46,0.11,0.89,0.3,1.29L6.29,16.3 C5.89,16.11,5.46,16,5,16c-1.65,0-3,1.35-3,3s1.35,3,3,3s3-1.35,3-3c0-0.46-0.11-0.89-0.3-1.29L17.71,7.7z",
                    }
                }
            }
        }
    }
}

pub fn short_text_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M5 9h14c.55 0 1 .45 1 1s-.45 1-1 1H5c-.55 0-1-.45-1-1s.45-1 1-1zm0 4h8c.55 0 1 .45 1 1s-.45 1-1 1H5c-.55 0-1-.45-1-1s.45-1 1-1z",
            }
        }
    }
}

pub fn show_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M4.2 17.78l5.3-5.3 3.25 3.25c.41.41 1.07.39 1.45-.04l7.17-8.07c.35-.39.33-.99-.04-1.37-.4-.4-1.07-.39-1.45.04l-6.39 7.18-3.29-3.29c-.39-.39-1.02-.39-1.41 0l-6.09 6.1c-.39.39-.39 1.02 0 1.41l.09.09c.39.39 1.03.39 1.41 0z",
            }
        }
    }
}

pub fn space_bar_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M18 10v3H6v-3c0-.55-.45-1-1-1s-1 .45-1 1v4c0 .55.45 1 1 1h14c.55 0 1-.45 1-1v-4c0-.55-.45-1-1-1s-1 .45-1 1z",
            }
        }
    }
}

pub fn square_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M15.5,17h-11C3.67,17,3,16.33,3,15.5v-11C3,3.67,3.67,3,4.5,3h11C16.33,3,17,3.67,17,4.5v11C17,16.33,16.33,17,15.5,17z",
                }
            }
        }
    }
}

pub fn square_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5z",
                    }
                }
            }
        }
    }
}

pub fn stacked_line_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M2.79,14.78L2.7,14.69c-0.39-0.39-0.39-1.02,0-1.41l6.09-6.1c0.39-0.39,1.02-0.39,1.41,0l3.29,3.29l6.39-7.18 c0.38-0.43,1.05-0.44,1.45-0.04l0,0c0.37,0.38,0.39,0.98,0.04,1.37l-7.17,8.07c-0.38,0.43-1.04,0.45-1.45,0.04L9.5,9.48l-5.3,5.3 C3.82,15.17,3.18,15.17,2.79,14.78z M4.2,20.78l5.3-5.3l3.25,3.25c0.41,0.41,1.07,0.39,1.45-0.04l7.17-8.07 c0.35-0.39,0.33-0.99-0.04-1.37l0,0c-0.4-0.4-1.07-0.39-1.45,0.04l-6.39,7.18l-3.29-3.29c-0.39-0.39-1.02-0.39-1.41,0l-6.09,6.1 c-0.39,0.39-0.39,1.02,0,1.41l0.09,0.09C3.18,21.17,3.82,21.17,4.2,20.78z",
            }
        }
    }
}

pub fn strikethrough_s_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M14.59 7.52c0-.31-.05-.59-.15-.85-.09-.27-.24-.49-.44-.68-.2-.19-.45-.33-.75-.44-.3-.1-.66-.16-1.06-.16-.39 0-.74.04-1.03.13s-.53.21-.72.36c-.19.16-.34.34-.44.55-.1.21-.15.43-.15.66 0 .48.25.88.74 1.21.38.25.77.48 1.41.7H7.39c-.05-.08-.11-.17-.15-.25-.26-.48-.39-1.03-.39-1.67 0-.61.13-1.16.4-1.67.26-.5.63-.93 1.11-1.29.48-.35 1.05-.63 1.7-.83.66-.19 1.39-.29 2.18-.29.81 0 1.54.11 2.21.34.66.22 1.23.54 1.69.94.47.4.83.88 1.08 1.43s.38 1.15.38 1.81h-3.01M20 10H4c-.55 0-1 .45-1 1s.45 1 1 1h8.62c.18.07.4.14.55.2.37.17.66.34.87.51s.35.36.43.57c.07.2.11.43.11.69 0 .23-.05.45-.14.66-.09.2-.23.38-.42.53-.19.15-.42.26-.71.35-.29.08-.63.13-1.01.13-.43 0-.83-.04-1.18-.13s-.66-.23-.91-.42c-.25-.19-.45-.44-.59-.75s-.25-.76-.25-1.21H6.4c0 .55.08 1.13.24 1.58s.37.85.65 1.21c.28.35.6.66.98.92.37.26.78.48 1.22.65.44.17.9.3 1.38.39.48.08.96.13 1.44.13.8 0 1.53-.09 2.18-.28s1.21-.45 1.67-.79c.46-.34.82-.77 1.07-1.27s.38-1.07.38-1.71c0-.6-.1-1.14-.31-1.61-.05-.11-.11-.23-.17-.33H20c.55 0 1-.45 1-1V11c0-.55-.45-1-1-1z",
            }
        }
    }
}

pub fn subscript_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
                path {
                    d: "M13.71,13.2c0,0.44-0.36,0.8-0.8,0.8c-0.29,0-0.54-0.15-0.68-0.38c0,0,0,0,0,0l-2.19-3.49H9.96l-2.2,3.5c0,0,0,0,0,0 C7.62,13.85,7.37,14,7.09,14c-0.44,0-0.8-0.36-0.8-0.8c0-0.16,0.05-0.31,0.13-0.43l0-0.01L8.94,8.8L6.63,5.2 C6.55,5.08,6.51,4.94,6.51,4.8c0-0.44,0.36-0.8,0.8-0.8c0.28,0,0.53,0.15,0.67,0.37c0,0,0,0,0,0l1.98,3.19h0.08L12,4.38 c0,0,0,0,0,0C12.14,4.15,12.4,4,12.68,4c0.44,0,0.8,0.36,0.8,0.8c0,0.16-0.05,0.3-0.12,0.43c0,0,0,0,0,0L11.05,8.8l2.54,3.97 C13.67,12.9,13.71,13.05,13.71,13.2z M18,16.5c0-0.28-0.22-0.5-0.5-0.5H16v-1h1c0.55,0,1-0.45,1-1v-1c0-0.55-0.45-1-1-1h-1.5 c-0.28,0-0.5,0.22-0.5,0.5v0c0,0.28,0.22,0.5,0.5,0.5H17v1l-1,0c-0.55,0-1,0.45-1,1v1c0,0.55,0.45,1,1,1h1.5 C17.78,17,18,16.78,18,16.5z",
                }
            }
        }
    }
}

pub fn subscript_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
                path {
                    d: "M10.52,10.73L7.3,5.72C6.82,4.97,7.35,4,8.23,4h0c0.39,0,0.74,0.2,0.95,0.53l2.76,4.46h0.12l2.74-4.45 C15.01,4.2,15.37,4,15.76,4h0c0.88,0,1.42,0.98,0.94,1.72l-3.23,5l3.55,5.55C17.5,17.02,16.96,18,16.08,18h0 c-0.38,0-0.74-0.2-0.95-0.52l-3.07-4.89h-0.12l-3.07,4.89C8.67,17.8,8.31,18,7.92,18h0c-0.88,0-1.42-0.97-0.94-1.72L10.52,10.73z M23,19.5L23,19.5c0-0.28-0.22-0.5-0.5-0.5c0,0,0,0,0,0H20v-1h2c0.55,0,1-0.45,1-1v-1c0-0.55-0.45-1-1-1h-2.5 c-0.28,0-0.5,0.22-0.5,0.5v0c0,0.28,0.22,0.5,0.5,0.5H22v1h-2c-0.55,0-1,0.45-1,1v1c0,0.55,0.45,1,1,1h2.5 C22.78,20,23,19.78,23,19.5z",
                }
            }
        }
    }
}

pub fn superscript_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
                path {
                    d: "M13.71,15.2c0,0.44-0.36,0.8-0.8,0.8c-0.29,0-0.54-0.15-0.68-0.38c0,0,0,0,0,0l-2.19-3.49H9.96l-2.2,3.5c0,0,0,0,0,0 C7.62,15.85,7.37,16,7.09,16c-0.44,0-0.8-0.36-0.8-0.8c0-0.16,0.05-0.31,0.13-0.43l0-0.01l2.52-3.96L6.63,7.2 C6.55,7.08,6.51,6.94,6.51,6.8c0-0.44,0.36-0.8,0.8-0.8c0.28,0,0.53,0.15,0.67,0.37c0,0,0,0,0,0l1.98,3.19h0.08L12,6.38 c0,0,0,0,0,0C12.14,6.15,12.4,6,12.68,6c0.44,0,0.8,0.36,0.8,0.8c0,0.16-0.05,0.3-0.12,0.43c0,0,0,0,0,0l-2.31,3.58l2.54,3.97 C13.67,14.9,13.71,15.05,13.71,15.2z M18,7.5C18,7.22,17.78,7,17.5,7H16V6h1c0.55,0,1-0.45,1-1V4c0-0.55-0.45-1-1-1h-1.5 C15.22,3,15,3.22,15,3.5v0C15,3.78,15.22,4,15.5,4H17v1l-1,0c-0.55,0-1,0.45-1,1v1c0,0.55,0.45,1,1,1h1.5C17.78,8,18,7.78,18,7.5z",
                }
            }
        }
    }
}

pub fn superscript_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    x: "0",
                    y: "0",
                    height: "24",
                    fill: "none",
                    width: "24",
                }
                path {
                    d: "M10.51,12.73L7.3,7.72C6.82,6.97,7.35,6,8.23,6h0c0.39,0,0.74,0.2,0.95,0.53l2.76,4.46h0.12l2.74-4.45 C15,6.2,15.36,6,15.75,6h0c0.88,0,1.42,0.98,0.94,1.72l-3.23,5l3.55,5.55C17.5,19.02,16.96,20,16.08,20h0 c-0.38,0-0.74-0.2-0.95-0.52l-3.07-4.89h-0.12l-3.07,4.89C8.66,19.8,8.31,20,7.92,20h0c-0.88,0-1.42-0.97-0.94-1.72L10.51,12.73z M23,8.5L23,8.5C23,8.22,22.78,8,22.5,8c0,0,0,0,0,0H20V7h2c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h-2.5C19.22,4,19,4.22,19,4.5v0 C19,4.78,19.22,5,19.5,5H22v1h-2c-0.55,0-1,0.45-1,1v1c0,0.55,0.45,1,1,1h2.5C22.78,9,23,8.78,23,8.5z",
                }
            }
        }
    }
}

pub fn table_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M10 10.02h5V21h-5V10.02zM17 21h3c1.1 0 2-.9 2-2v-9h-5v11zm3-18H5c-1.1 0-2 .9-2 2v3h19V5c0-1.1-.9-2-2-2zM3 19c0 1.1.9 2 2 2h3V10H3v9z",
            }
        }
    }
}

pub fn table_rows_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                fill: "none",
                x: "0",
                y: "0",
                width: "20",
                height: "20",
            }
            path {
                d: "M15.5,7h-11C3.67,7,3,6.33,3,5.5v0C3,4.67,3.67,4,4.5,4h11C16.33,4,17,4.67,17,5.5v0C17,6.33,16.33,7,15.5,7z M15.5,8.5h-11 C3.67,8.5,3,9.17,3,10v0c0,0.83,0.67,1.5,1.5,1.5h11c0.83,0,1.5-0.67,1.5-1.5v0C17,9.17,16.33,8.5,15.5,8.5z M15.5,13h-11 C3.67,13,3,13.67,3,14.5v0C3,15.33,3.67,16,4.5,16h11c0.83,0,1.5-0.67,1.5-1.5v0C17,13.67,16.33,13,15.5,13z",
            }
        }
    }
}

pub fn table_rows_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M19,8H5C3.9,8,3,7.1,3,6v0c0-1.1,0.9-2,2-2h14c1.1,0,2,0.9,2,2v0C21,7.1,20.1,8,19,8z M19,10H5c-1.1,0-2,0.9-2,2v0 c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v0C21,10.9,20.1,10,19,10z M19,16H5c-1.1,0-2,0.9-2,2v0c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v0 C21,16.9,20.1,16,19,16z",
            }
        }
    }
}

pub fn text_decrease_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                height: "20",
                fill: "none",
                width: "20",
            }
            path {
                d: "M12,10c0-0.41,0.34-0.75,0.75-0.75h4.5C17.66,9.25,18,9.59,18,10s-0.34,0.75-0.75,0.75h-4.5C12.34,10.75,12,10.41,12,10z M3,15c0.34,0,0.65-0.21,0.76-0.53l0.72-2.02h4.04l0.71,2.02C9.35,14.79,9.66,15,10,15c0.56,0,0.95-0.55,0.75-1.08L7.72,5.84 C7.51,5.33,7.03,5,6.49,5S5.47,5.33,5.28,5.84l-3.03,8.08C2.05,14.44,2.44,15,3,15z M6.06,8.02l0.39-1.16h0.08l0.39,1.16L7.98,11H5 L6.06,8.02z",
            }
        }
    }
}

pub fn text_decrease_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M2.61,19L2.61,19c0.48,0,0.91-0.3,1.06-0.75l1.01-2.83h5.65l0.99,2.82C11.48,18.7,11.91,19,12.39,19 c0.79,0,1.33-0.79,1.05-1.52L9.19,6.17C8.93,5.47,8.25,5,7.5,5S6.07,5.47,5.81,6.17L1.56,17.48C1.28,18.21,1.83,19,2.61,19z M7.44,7.6h0.12l2.03,5.79H5.41L7.44,7.6z M15,12c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1s-0.45,1-1,1h-6C15.45,13,15,12.55,15,12z",
            }
        }
    }
}

pub fn text_fields_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M2.5 5.5C2.5 6.33 3.17 7 4 7h3.5v10.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5V7H14c.83 0 1.5-.67 1.5-1.5S14.83 4 14 4H4c-.83 0-1.5.67-1.5 1.5zM20 9h-6c-.83 0-1.5.67-1.5 1.5S13.17 12 14 12h1.5v5.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5V12H20c.83 0 1.5-.67 1.5-1.5S20.83 9 20 9z",
            }
        }
    }
}

pub fn text_increase_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            rect {
                width: "20",
                fill: "none",
                height: "20",
            }
            path {
                d: "M12,10c0-0.41,0.34-0.75,0.75-0.75h1.5v-1.5C14.25,7.34,14.59,7,15,7s0.75,0.34,0.75,0.75v1.5h1.5C17.66,9.25,18,9.59,18,10 s-0.34,0.75-0.75,0.75h-1.5v1.5c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75v-1.5h-1.5C12.34,10.75,12,10.41,12,10z M3,15 c0.34,0,0.65-0.21,0.76-0.53l0.72-2.02h4.04l0.71,2.02C9.35,14.79,9.66,15,10,15c0.56,0,0.95-0.55,0.75-1.08L7.72,5.84 C7.51,5.33,7.03,5,6.49,5S5.47,5.33,5.28,5.84l-3.03,8.08C2.05,14.44,2.44,15,3,15z M6.06,8.02l0.39-1.16h0.08l0.39,1.16L7.98,11H5 L6.06,8.02z",
            }
        }
    }
}

pub fn text_increase_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M2.61,19L2.61,19c0.48,0,0.91-0.3,1.06-0.75l1.01-2.83h5.65l0.99,2.82C11.48,18.7,11.91,19,12.39,19 c0.79,0,1.33-0.79,1.05-1.52L9.19,6.17C8.93,5.47,8.25,5,7.5,5S6.07,5.47,5.81,6.17L1.56,17.48C1.28,18.21,1.83,19,2.61,19z M7.44,7.6h0.12l2.03,5.79H5.41L7.44,7.6z M15,12c0-0.55,0.45-1,1-1h2V9c0-0.55,0.45-1,1-1s1,0.45,1,1v2h2c0.55,0,1,0.45,1,1 s-0.45,1-1,1h-2v2c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2h-2C15.45,13,15,12.55,15,12z",
            }
        }
    }
}

pub fn title_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M5 5.5C5 6.33 5.67 7 6.5 7h4v10.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5V7h4c.83 0 1.5-.67 1.5-1.5S18.33 4 17.5 4h-11C5.67 4 5 4.67 5 5.5z",
            }
        }
    }
}

pub fn type_specimen_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                rect {
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M16.5,2h-10C5.67,2,5,2.67,5,3.5v10C5,14.33,5.67,15,6.5,15h10c0.83,0,1.5-0.67,1.5-1.5v-10C18,2.67,17.33,2,16.5,2z M13.41,11.62l-0.49-1.41h-2.83l-0.5,1.41C9.51,11.85,9.3,12,9.06,12h0c-0.39,0-0.67-0.39-0.53-0.76l2.12-5.65 C10.79,5.23,11.12,5,11.5,5h0c0.38,0,0.71,0.23,0.85,0.59l2.12,5.65c0.14,0.37-0.13,0.76-0.53,0.76h0 C13.7,12,13.49,11.85,13.41,11.62z",
                    }
                    path {
                        d: "M2.75,5L2.75,5C2.34,5,2,5.34,2,5.75V16.5C2,17.33,2.67,18,3.5,18h10.75c0.41,0,0.75-0.34,0.75-0.75l0,0 c0-0.41-0.34-0.75-0.75-0.75H3.5V5.75C3.5,5.34,3.16,5,2.75,5z",
                    }
                    polygon {
                        points: "11.47,6.3 10.45,9.19 12.55,9.19 11.53,6.3",
                    }
                }
            }
        }
    }
}

pub fn type_specimen_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M3,6L3,6C2.45,6,2,6.45,2,7v13c0,1.1,0.9,2,2,2h13c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4V7C4,6.45,3.55,6,3,6z",
                    }
                    path {
                        d: "M20,2H8C6.9,2,6,2.9,6,4v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M16.46,14.01l-0.63-1.82h-3.63 l-0.65,1.82c-0.1,0.29-0.38,0.48-0.68,0.48h0c-0.51,0-0.86-0.51-0.68-0.98l2.73-7.27C13.08,5.8,13.52,5.5,14,5.5h0 c0.48,0,0.92,0.3,1.09,0.75l2.73,7.27c0.18,0.47-0.17,0.98-0.68,0.98h0C16.83,14.5,16.56,14.31,16.46,14.01z",
                    }
                    polygon {
                        points: "13.96,7.17 12.65,10.89 15.34,10.89 14.04,7.17",
                    }
                }
            }
        }
    }
}

pub fn vertical_align_bottom_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M14.79 13H13V4c0-.55-.45-1-1-1s-1 .45-1 1v9H9.21c-.45 0-.67.54-.35.85l2.79 2.79c.2.2.51.2.71 0l2.79-2.79c.31-.31.09-.85-.36-.85zM4 20c0 .55.45 1 1 1h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn vertical_align_center_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M9.21 19H11v3c0 .55.45 1 1 1s1-.45 1-1v-3h1.79c.45 0 .67-.54.35-.85l-2.79-2.79c-.2-.2-.51-.2-.71 0l-2.79 2.79c-.31.31-.09.85.36.85zm5.58-14H13V2c0-.55-.45-1-1-1s-1 .45-1 1v3H9.21c-.45 0-.67.54-.36.85l2.79 2.79c.2.2.51.2.71 0l2.79-2.79c.32-.31.1-.85-.35-.85zM4 12c0 .55.45 1 1 1h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn vertical_align_top_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M9.21 11H11v9c0 .55.45 1 1 1s1-.45 1-1v-9h1.79c.45 0 .67-.54.35-.85l-2.79-2.79c-.2-.2-.51-.2-.71 0l-2.79 2.79c-.31.31-.09.85.36.85zM4 4c0 .55.45 1 1 1h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn vertical_distribute_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            rect {
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M22,3L22,3c0,0.55-0.45,1-1,1H3C2.45,4,2,3.55,2,3v0c0-0.55,0.45-1,1-1h18C21.55,2,22,2.45,22,3z M7,12L7,12 c0,0.83,0.67,1.5,1.5,1.5h7c0.83,0,1.5-0.67,1.5-1.5v0c0-0.83-0.67-1.5-1.5-1.5h-7C7.67,10.5,7,11.17,7,12z M2,21L2,21 c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H3C2.45,20,2,20.45,2,21z",
            }
        }
    }
}

pub fn wrap_text_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M5 7h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1 .45-1 1s.45 1 1 1zm11.83 4H5c-.55 0-1 .45-1 1s.45 1 1 1h12.13c1 0 1.93.67 2.09 1.66.21 1.25-.76 2.34-1.97 2.34H15v-.79c0-.45-.54-.67-.85-.35l-1.79 1.79c-.2.2-.2.51 0 .71l1.79 1.79c.32.32.85.09.85-.35V19h2c2.34 0 4.21-2.01 3.98-4.39-.2-2.08-2.06-3.61-4.15-3.61zM9 17H5c-.55 0-1 .45-1 1s.45 1 1 1h4c.55 0 1-.45 1-1s-.45-1-1-1z",
            }
        }
    }
}

