use dioxus::prelude::*;
use crate::IconProps;
pub fn auto_mode_icons_20px(props: IconProps) -> Element {
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
                        d: "M15.13,2.61c-1.04-0.72-2.24-1.23-3.53-1.46c-0.44-0.08-0.85,0.25-0.85,0.7v0.08c0,0.35,0.26,0.64,0.61,0.7 c1.06,0.19,2.04,0.61,2.89,1.2c0.29,0.2,0.68,0.18,0.93-0.07l0.06-0.06C15.55,3.39,15.5,2.87,15.13,2.61z",
                    }
                    path {
                        d: "M10,17.5c-2.66,0-4.98-1.41-6.31-3.5H5c0.41,0,0.75-0.34,0.75-0.75v0c0-0.41-0.34-0.75-0.75-0.75H1.5 C1.22,12.5,1,12.72,1,13v3.5c0,0.41,0.34,0.75,0.75,0.75h0c0.41,0,0.75-0.34,0.75-0.75v-1.54C4.11,17.39,6.86,19,10,19 c3.69,0,6.86-2.22,8.25-5.4c0.19-0.44-0.06-0.94-0.52-1.05h0c-0.36-0.08-0.72,0.12-0.87,0.46C15.7,15.65,13.06,17.5,10,17.5z",
                    }
                    path {
                        d: "M9.25,1.92V1.85c0-0.45-0.41-0.78-0.85-0.7C7.1,1.38,5.91,1.89,4.87,2.62c-0.37,0.26-0.42,0.78-0.1,1.09l0.06,0.06 c0.25,0.25,0.64,0.27,0.93,0.06c0.85-0.59,1.83-1.01,2.89-1.21C8.99,2.56,9.25,2.28,9.25,1.92z",
                    }
                    path {
                        d: "M3.77,4.83L3.71,4.77c-0.32-0.32-0.84-0.26-1.09,0.1C1.89,5.91,1.38,7.11,1.15,8.4C1.07,8.84,1.4,9.25,1.85,9.25h0.08 c0.35,0,0.64-0.26,0.7-0.61c0.2-1.06,0.62-2.04,1.21-2.89C4.03,5.46,4.01,5.08,3.77,4.83z",
                    }
                    path {
                        d: "M7,10.46l1.75,0.79L9.54,13c0.18,0.39,0.73,0.39,0.91,0l0.79-1.75L13,10.46c0.39-0.18,0.39-0.73,0-0.91l-1.75-0.79 L10.46,7c-0.18-0.39-0.73-0.39-0.91,0L8.75,8.75L7,9.54C6.61,9.72,6.61,10.28,7,10.46z",
                    }
                    path {
                        d: "M18.07,9.25h0.08c0.45,0,0.78-0.41,0.7-0.85c-0.23-1.29-0.74-2.49-1.46-3.53c-0.26-0.37-0.78-0.42-1.09-0.1l-0.06,0.06 c-0.25,0.25-0.27,0.64-0.07,0.93c0.59,0.85,1.01,1.84,1.2,2.89C17.44,8.99,17.72,9.25,18.07,9.25z",
                    }
                }
            }
        }
    }
}

pub fn auto_mode_icons_24px(props: IconProps) -> Element {
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18.06,2.83c-1.15-0.77-2.46-1.32-3.86-1.61C13.58,1.1,13,1.57,13,2.21v0c0,0.46,0.31,0.88,0.76,0.97 c1.17,0.23,2.26,0.7,3.21,1.34c0.39,0.26,0.9,0.19,1.23-0.14l0,0C18.66,3.93,18.59,3.18,18.06,2.83z",
                    }
                    path {
                        d: "M11,2.21L11,2.21c0-0.64-0.58-1.11-1.2-0.99c-1.4,0.29-2.71,0.84-3.86,1.61c-0.52,0.35-0.59,1.1-0.15,1.54l0,0 c0.33,0.33,0.84,0.4,1.23,0.14c0.96-0.64,2.04-1.1,3.21-1.34C10.69,3.09,11,2.67,11,2.21z",
                    }
                    path {
                        d: "M4.38,5.79L4.38,5.79C3.93,5.34,3.18,5.42,2.84,5.94C2.07,7.09,1.51,8.39,1.23,9.8C1.1,10.42,1.58,11,2.21,11h0 c0.46,0,0.88-0.31,0.97-0.76c0.23-1.17,0.7-2.26,1.34-3.22C4.77,6.64,4.7,6.12,4.38,5.79z",
                    }
                    path {
                        d: "M21.79,11L21.79,11c0.63,0,1.11-0.58,0.98-1.2c-0.29-1.4-0.84-2.7-1.61-3.86c-0.35-0.52-1.1-0.6-1.54-0.15l0,0 c-0.33,0.33-0.4,0.84-0.14,1.23c0.64,0.96,1.1,2.05,1.34,3.22C20.91,10.69,21.33,11,21.79,11z",
                    }
                    path {
                        d: "M8,12.46l2.44,1.11L11.54,16c0.18,0.39,0.73,0.39,0.91,0l1.11-2.44L16,12.46c0.39-0.18,0.39-0.73,0-0.91l-2.44-1.11 L12.46,8c-0.18-0.39-0.73-0.39-0.91,0l-1.11,2.44L8,11.54C7.61,11.72,7.61,12.28,8,12.46z",
                    }
                    path {
                        d: "M12,21c-3.11,0-5.85-1.59-7.46-4H6c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H2c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-1.7c1.99,2.84,5.27,4.7,9,4.7c4.45,0,8.27-2.64,10-6.43c0.26-0.57-0.08-1.25-0.69-1.39l0,0 c-0.45-0.1-0.93,0.11-1.12,0.54C18.77,18.83,15.64,21,12,21z",
                    }
                }
            }
        }
    }
}

pub fn blinds_icons_20px(props: IconProps) -> Element {
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
                path {
                    d: "M16,15.5v-11C16,3.67,15.33,3,14.5,3h-9C4.67,3,4,3.67,4,4.5v11H2.75C2.34,15.5,2,15.84,2,16.25l0,0 C2,16.66,2.34,17,2.75,17h14.5c0.41,0,0.75-0.34,0.75-0.75l0,0c0-0.41-0.34-0.75-0.75-0.75H16z M12.5,7.75h2v1.5h-2V7.75z M11,9.25 H5.5v-1.5H11V9.25z M14.5,6.25h-2V4.5h2V6.25z M11,4.5v1.75H5.5V4.5H11z M5.5,15.5v-4.75H11v1.53c-0.3,0.23-0.5,0.57-0.5,0.97 c0,0.69,0.56,1.25,1.25,1.25S13,13.94,13,13.25c0-0.4-0.2-0.75-0.5-0.97v-1.53h2v4.75H5.5z",
                }
            }
        }
    }
}

pub fn blinds_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H20z M16,9h2v2h-2V9z M14,11H6V9h8V11z M18,7h-2V5h2V7z M14,5v2H6V5H14z M6,19v-6h8v1.82 c-0.45,0.32-0.75,0.84-0.75,1.43c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75c0-0.59-0.3-1.12-0.75-1.43V13h2v6H6z",
                }
            }
        }
    }
}

pub fn blinds_closed_icons_20px(props: IconProps) -> Element {
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
                    d: "M16,15.5v-11C16,3.67,15.33,3,14.5,3h-9C4.67,3,4,3.67,4,4.5v11H2.75C2.34,15.5,2,15.84,2,16.25l0,0 C2,16.66,2.34,17,2.75,17h7.75c0,0.69,0.56,1.25,1.25,1.25S13,17.69,13,17h4.25c0.41,0,0.75-0.34,0.75-0.75l0,0 c0-0.41-0.34-0.75-0.75-0.75H16z M14.5,9.25h-2v-1.5h2V9.25z M11,9.25H5.5v-1.5H11V9.25z M11,10.75v1.5H5.5v-1.5H11z M12.5,10.75h2 v1.5h-2V10.75z M14.5,6.25h-2V4.5h2V6.25z M11,4.5v1.75H5.5V4.5H11z M5.5,15.5v-1.75H11v1.75H5.5z M12.5,15.5v-1.75h2v1.75H12.5z",
                }
            }
        }
    }
}

pub fn blinds_closed_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h10.25 c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H21c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H20z M18,11h-2V9h2V11z M14,11H6V9h8V11 z M14,13v2H6v-2H14z M16,13h2v2h-2V13z M18,7h-2V5h2V7z M14,5v2H6V5H14z M6,19v-2h8v2H6z M16,19v-2h2v2H16z",
                }
            }
        }
    }
}

pub fn broadcast_on_home_icons_20px(props: IconProps) -> Element {
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
                        d: "M18,5.5C18,4.67,17.33,4,16.5,4H4.75C4.34,4,4,4.34,4,4.75v0C4,5.16,4.34,5.5,4.75,5.5H16.5v2.05 c0.55,0.25,1.05,0.59,1.5,0.99V5.5z",
                    }
                    path {
                        d: "M6.5,7.5H3c-0.55,0-1,0.45-1,1V15c0,0.55,0.45,1,1,1h3.5c0.55,0,1-0.45,1-1V8.5C7.5,7.95,7.05,7.5,6.5,7.5z M6,14.5H3.5V9 H6V14.5z",
                    }
                    path {
                        d: "M14.5,13.85c0.36-0.21,0.58-0.62,0.47-1.09c-0.09-0.36-0.39-0.66-0.76-0.74C13.57,11.89,13,12.38,13,13 c0,0.37,0.21,0.67,0.5,0.85v3.65c0,0.28,0.22,0.5,0.5,0.5h0c0.28,0,0.5-0.22,0.5-0.5V13.85z",
                    }
                    path {
                        d: "M13.43,8.03c-2.32,0.26-4.2,2.17-4.41,4.5c-0.12,1.36,0.3,2.62,1.07,3.59c0.19,0.23,0.53,0.25,0.74,0.04l0,0 c0.18-0.18,0.19-0.46,0.04-0.66c-0.7-0.87-1.04-2.04-0.8-3.29c0.3-1.56,1.56-2.83,3.12-3.13C15.75,8.58,18,10.53,18,13 c0,1.02-0.39,1.94-1.02,2.65c-0.17,0.19-0.17,0.47,0,0.66v0c0.2,0.22,0.55,0.22,0.75,0C18.52,15.42,19,14.27,19,13 C19,10.05,16.45,7.7,13.43,8.03z",
                    }
                    path {
                        d: "M14.47,11.05c0.71,0.16,1.29,0.74,1.47,1.44c0.16,0.63,0.01,1.22-0.33,1.68c-0.14,0.19-0.14,0.46,0.02,0.64l0,0 c0.21,0.24,0.59,0.21,0.78-0.05c0.46-0.62,0.69-1.42,0.54-2.29c-0.22-1.29-1.3-2.31-2.6-2.46C12.54,9.81,11,11.23,11,13 c0,0.62,0.19,1.2,0.51,1.68c0.17,0.25,0.54,0.27,0.75,0.05l0,0c0.17-0.17,0.19-0.44,0.05-0.64C12.12,13.77,12,13.4,12,13 C12,11.74,13.16,10.75,14.47,11.05z",
                    }
                }
            }
        }
    }
}

pub fn broadcast_on_home_icons_24px(props: IconProps) -> Element {
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
                        d: "M22,6c0-1.1-0.9-2-2-2H5C4.45,4,4,4.45,4,5v0c0,0.55,0.45,1,1,1h15v2.59c0.73,0.29,1.4,0.69,2,1.17V6z",
                    }
                    path {
                        d: "M8,9H3c-0.5,0-1,0.5-1,1v9c0,0.5,0.5,1,1,1h5c0.5,0,1-0.5,1-1v-9C9,9.5,8.5,9,8,9z M7,18H4v-7h3V18z",
                    }
                    path {
                        d: "M17.75,16.97c0.3-0.23,0.5-0.57,0.5-0.97c0-0.69-0.56-1.25-1.25-1.25s-1.25,0.56-1.25,1.25c0,0.4,0.2,0.75,0.5,0.97v4.28 c0,0.41,0.34,0.75,0.75,0.75l0,0c0.41,0,0.75-0.34,0.75-0.75V16.97z",
                    }
                    path {
                        d: "M17.54,13.56c0.98,0.21,1.76,1.03,1.93,2.02c0.11,0.64-0.03,1.25-0.34,1.74c-0.18,0.29-0.13,0.67,0.12,0.91l0,0 c0.34,0.33,0.9,0.29,1.16-0.12c0.51-0.82,0.73-1.83,0.53-2.9c-0.3-1.56-1.56-2.83-3.12-3.13C15.24,11.58,13,13.53,13,16 c0,0.78,0.22,1.5,0.6,2.11c0.25,0.41,0.83,0.46,1.16,0.12l0,0c0.24-0.24,0.29-0.63,0.11-0.92c-0.24-0.38-0.37-0.83-0.37-1.31 C14.5,14.45,15.93,13.22,17.54,13.56z",
                    }
                    path {
                        d: "M16.25,9.54c-2.94,0.33-5.32,2.68-5.69,5.61c-0.23,1.82,0.29,3.51,1.3,4.82c0.27,0.35,0.8,0.37,1.12,0.06l0,0 c0.27-0.27,0.28-0.7,0.05-1c-0.8-1.05-1.2-2.43-0.95-3.89c0.34-2.03,1.95-3.67,3.98-4.05C19.22,10.5,22,12.93,22,16 c0,1.13-0.38,2.18-1.02,3.02c-0.23,0.3-0.21,0.73,0.06,1l0,0c0.31,0.31,0.84,0.3,1.11-0.06C23,18.87,23.5,17.49,23.5,16 C23.5,12.16,20.17,9.1,16.25,9.54z",
                    }
                }
            }
        }
    }
}

pub fn broadcast_on_personal_icons_20px(props: IconProps) -> Element {
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
                        d: "M13.38,7.03L8,3L2,7.5V17h7.54C8.58,15.94,8,14.54,8,13C8,9.9,10.36,7.34,13.38,7.03z",
                    }
                    path {
                        d: "M14,12c-0.55,0-1,0.45-1,1c0,0.37,0.21,0.67,0.5,0.85V18h1v-4.15C14.79,13.67,15,13.37,15,13C15,12.45,14.55,12,14,12z",
                    }
                    path {
                        d: "M14,8c-2.76,0-5,2.24-5,5c0,1.38,0.56,2.63,1.46,3.54l0.71-0.71C10.45,15.1,10,14.1,10,13c0-2.21,1.79-4,4-4s4,1.79,4,4 c0,1.18-0.52,2.23-1.33,2.96l0.67,0.74C18.35,15.78,19,14.47,19,13C19,10.24,16.76,8,14,8z",
                    }
                    path {
                        d: "M14,10c-1.66,0-3,1.34-3,3c0,0.83,0.34,1.58,0.88,2.12l0.71-0.71C12.22,14.05,12,13.55,12,13c0-1.1,0.9-2,2-2s2,0.9,2,2 c0,0.59-0.26,1.11-0.67,1.48L16,15.22c0.61-0.55,1-1.34,1-2.22C17,11.34,15.66,10,14,10z",
                    }
                }
            }
        }
    }
}

pub fn broadcast_on_personal_icons_24px(props: IconProps) -> Element {
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M17,8c0.7,0,1.38,0.1,2.02,0.27L12,3L4,9v12h6.76C9.66,19.63,9,17.89,9,16C9,11.58,12.58,8,17,8z",
                    }
                    path {
                        d: "M17,14.75c-0.69,0-1.25,0.56-1.25,1.25c0,0.4,0.2,0.75,0.5,0.97V22h1.5v-5.03c0.3-0.23,0.5-0.57,0.5-0.97 C18.25,15.31,17.69,14.75,17,14.75z",
                    }
                    path {
                        d: "M17,12c-2.21,0-4,1.79-4,4c0,1.1,0.45,2.1,1.17,2.83l1.06-1.06c-0.45-0.45-0.73-1.08-0.73-1.77c0-1.38,1.12-2.5,2.5-2.5 s2.5,1.12,2.5,2.5c0,0.69-0.28,1.31-0.73,1.76l1.06,1.06C20.55,18.1,21,17.1,21,16C21,13.79,19.21,12,17,12z",
                    }
                    path {
                        d: "M17,9.5c-3.59,0-6.5,2.91-6.5,6.5c0,1.79,0.73,3.42,1.9,4.6l1.06-1.06C12.56,18.63,12,17.38,12,16c0-2.76,2.24-5,5-5 s5,2.24,5,5c0,1.37-0.56,2.62-1.46,3.52l1.07,1.06c1.17-1.18,1.89-2.8,1.89-4.58C23.5,12.41,20.59,9.5,17,9.5z",
                    }
                }
            }
        }
    }
}

pub fn curtains_icons_20px(props: IconProps) -> Element {
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
                    d: "M16,15.5v-11C16,3.67,15.33,3,14.5,3h-9C4.67,3,4,3.67,4,4.5v11H2.75C2.34,15.5,2,15.84,2,16.25l0,0 C2,16.66,2.34,17,2.75,17h14.5c0.41,0,0.75-0.34,0.75-0.75l0,0c0-0.41-0.34-0.75-0.75-0.75H16z M7.35,10 C8.6,9.16,9.57,7.68,10,5.88c0.43,1.8,1.4,3.28,2.65,4.12c-1.25,0.84-2.22,2.32-2.65,4.12C9.57,12.32,8.6,10.84,7.35,10z",
                }
            }
        }
    }
}

pub fn curtains_icons_24px(props: IconProps) -> Element {
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
                    d: "M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H20z M8.19,12c2.04-1.35,3.5-3.94,3.76-7h0.09c0.26,3.06,1.72,5.65,3.76,7c-2.04,1.35-3.5,3.94-3.76,7h-0.09 C11.69,15.94,10.23,13.35,8.19,12z",
                }
            }
        }
    }
}

pub fn curtains_closed_icons_20px(props: IconProps) -> Element {
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
                    d: "M16,15.5v-11C16,3.67,15.33,3,14.5,3h-9C4.67,3,4,3.67,4,4.5v11H2.75C2.34,15.5,2,15.84,2,16.25l0,0 C2,16.66,2.34,17,2.75,17h14.5c0.41,0,0.75-0.34,0.75-0.75l0,0c0-0.41-0.34-0.75-0.75-0.75H16z M9,4.5h2v11H9V4.5z",
                }
            }
        }
    }
}

pub fn curtains_closed_icons_24px(props: IconProps) -> Element {
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H20z M11,5h2v14h-2V5z",
                }
            }
        }
    }
}

pub fn electric_bolt_icons_20px(props: IconProps) -> Element {
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
                path {
                    d: "M11.72,2.21l-8.56,7.93c-0.32,0.3-0.12,0.83,0.31,0.86l7.33,0.5l-3.64,5.27c-0.24,0.3-0.21,0.73,0.06,1.01h0 c0.29,0.29,0.76,0.3,1.06,0.01l8.56-7.93c0.32-0.3,0.12-0.83-0.31-0.86L9.2,8.5l3.64-5.27c0.24-0.3,0.21-0.73-0.06-1.01l0,0 C12.49,1.93,12.01,1.93,11.72,2.21z",
                }
            }
        }
    }
}

pub fn electric_bolt_icons_24px(props: IconProps) -> Element {
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M14.69,2.21L4.33,11.49c-0.64,0.58-0.28,1.65,0.58,1.73L13,14l-4.85,6.76c-0.22,0.31-0.19,0.74,0.08,1.01h0 c0.3,0.3,0.77,0.31,1.08,0.02l10.36-9.28c0.64-0.58,0.28-1.65-0.58-1.73L11,10l4.85-6.76c0.22-0.31,0.19-0.74-0.08-1.01l0,0 C15.47,1.93,15,1.92,14.69,2.21z",
                    }
                }
            }
        }
    }
}

pub fn electric_meter_icons_20px(props: IconProps) -> Element {
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
                    d: "M9.9,2C6.17,2.05,3.03,5.21,3,8.94c-0.02,3.01,1.85,5.58,4.5,6.59v1.72C7.5,17.66,7.84,18,8.25,18h0 C8.66,18,9,17.66,9,17.25v-1.33C9.33,15.97,9.66,16,10,16s0.67-0.03,1-0.08v1.33c0,0.41,0.34,0.75,0.75,0.75h0 c0.41,0,0.75-0.34,0.75-0.75v-1.72C15.13,14.52,17,11.98,17,9C17,5.1,13.81,1.94,9.9,2z M11.4,11.65l-1.57,1.57 c-0.29,0.29-0.76,0.29-1.05,0l0,0c-0.29-0.29-0.29-0.76,0-1.05l0.48-0.48L8.6,11.05c-0.2-0.2-0.2-0.51,0-0.71l1.57-1.57 c0.29-0.29,0.76-0.29,1.05,0l0,0c0.29,0.29,0.29,0.76,0,1.05l-0.48,0.48l0.65,0.65C11.59,11.14,11.59,11.46,11.4,11.65z M12.25,7.5 h-4.5C7.34,7.5,7,7.16,7,6.75v0C7,6.34,7.34,6,7.75,6h4.5C12.66,6,13,6.34,13,6.75v0C13,7.16,12.66,7.5,12.25,7.5z",
                }
            }
        }
    }
}

pub fn electric_meter_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M11.73,2C7.05,2.14,3.15,6.03,3,10.71c-0.13,4.04,2.42,7.5,6,8.77V21c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-1.06 c0.33,0.04,0.66,0.06,1,0.06s0.67-0.02,1-0.06V21c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-1.53c3.49-1.24,6-4.57,6-8.47 C21,5.95,16.82,1.85,11.73,2z M13.54,14.71L12,16.25c-0.41,0.41-1.09,0.41-1.5,0h0c-0.41-0.41-0.41-1.09,0-1.5l0.5-0.5l-0.54-0.54 c-0.39-0.39-0.39-1.02,0-1.41L12,10.75c0.41-0.41,1.09-0.41,1.5,0l0,0c0.41,0.41,0.41,1.09,0,1.5l-0.5,0.5l0.54,0.54 C13.93,13.68,13.93,14.32,13.54,14.71z M15,9H9C8.45,9,8,8.55,8,8v0c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1v0C16,8.55,15.55,9,15,9 z",
                }
            }
        }
    }
}

pub fn energy_savings_leaf_icons_20px(props: IconProps) -> Element {
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
                        d: "M10,3c-3.73,0-7,3-7,7c0,1.66,0.58,3.19,1.55,4.39l-1.3,1.3c-0.29,0.29-0.29,0.77,0,1.06c0.29,0.29,0.77,0.29,1.06,0 l1.3-1.3C6.81,16.42,8.34,17,10,17c1.79,0,3.58-0.68,4.95-2.05C16.32,13.58,17,11.79,17,10V4.5C17,3.67,16.33,3,15.5,3H10z M12.84,10.3l-3.86,3.59c-0.16,0.15-0.4,0.14-0.55-0.01c-0.14-0.14-0.15-0.37-0.03-0.52l2.02-2.58L7.49,10.6 c-0.45-0.03-0.65-0.58-0.32-0.9l3.86-3.59c0.16-0.15,0.4-0.14,0.55,0.01c0.14,0.14,0.15,0.37,0.03,0.52L9.58,9.22l2.93,0.18 C12.97,9.43,13.17,9.99,12.84,10.3z",
                    }
                }
            }
        }
    }
}

pub fn energy_savings_leaf_icons_24px(props: IconProps) -> Element {
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,3c-4.8,0-9,3.86-9,9c0,2.12,0.74,4.07,1.97,5.61l-1.68,1.68c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0 l1.68-1.68C7.93,20.26,9.88,21,12,21c2.3,0,4.61-0.88,6.36-2.64C20.12,16.61,21,14.3,21,12V5c0-1.1-0.9-2-2-2H12z M15.83,12.26 l-5.16,4.63c-0.16,0.15-0.41,0.14-0.56-0.01c-0.14-0.14-0.16-0.36-0.04-0.52l2.44-3.33l-4.05-0.4c-0.44-0.04-0.63-0.59-0.3-0.89 l5.16-4.63c0.16-0.15,0.41-0.14,0.56,0.01c0.14,0.14,0.16,0.36,0.04,0.52l-2.44,3.33l4.05,0.4 C15.98,11.41,16.16,11.96,15.83,12.26z",
                    }
                }
            }
        }
    }
}

pub fn gas_meter_icons_20px(props: IconProps) -> Element {
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
                    d: "M13,3.5h-0.5V2.75C12.5,2.34,12.16,2,11.75,2h0C11.34,2,11,2.34,11,2.75V3.5H9V2.75C9,2.34,8.66,2,8.25,2h0 C7.84,2,7.5,2.34,7.5,2.75V3.5H7c-1.66,0-3,1.34-3,3V15c0,1.66,1.34,3,3,3h6c1.66,0,3-1.34,3-3V6.5C16,4.84,14.66,3.5,13,3.5z M10,14.5c-1.1,0-2-0.88-2-1.97c0-0.8,0.29-1.07,1.62-2.6c0.2-0.23,0.56-0.23,0.76,0c1.32,1.52,1.62,1.79,1.62,2.6 C12,13.62,11.1,14.5,10,14.5z M12.25,8h-4.5C7.34,8,7,7.66,7,7.25v0C7,6.84,7.34,6.5,7.75,6.5h4.5C12.66,6.5,13,6.84,13,7.25v0 C13,7.66,12.66,8,12.25,8z",
                }
            }
        }
    }
}

pub fn gas_meter_icons_24px(props: IconProps) -> Element {
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M16,4h-1V3c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v1h-2V3c0-0.55-0.45-1-1-1h0C9.45,2,9,2.45,9,3v1H8C5.79,4,4,5.79,4,8 v10c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4V8C20,5.79,18.21,4,16,4z M12,18c-1.38,0-2.5-1.1-2.5-2.46c0-1.02,0.38-1.35,2.12-3.35 c0.2-0.23,0.56-0.23,0.75,0c1.73,1.99,2.12,2.34,2.12,3.35C14.5,16.9,13.38,18,12,18z M15,10H9c-0.55,0-1-0.45-1-1v0 c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1v0C16,9.55,15.55,10,15,10z",
                }
            }
        }
    }
}

pub fn heat_pump_icons_20px(props: IconProps) -> Element {
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
                path {
                    d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M10.5,6.3c0.66,0.09,1.26,0.34,1.76,0.73L10.5,8.79V6.3z M9.5,6.3v2.49L7.74,7.03C8.24,6.64,8.84,6.39,9.5,6.3z M7.03,7.74 L8.79,9.5H6.3C6.39,8.84,6.64,8.24,7.03,7.74z M6.3,10.5h2.49l-1.76,1.76C6.64,11.76,6.39,11.16,6.3,10.5z M9.5,13.7 c-0.66-0.09-1.26-0.34-1.76-0.73l1.76-1.76V13.7z M9.25,10c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75 c0,0.41-0.34,0.75-0.75,0.75S9.25,10.41,9.25,10z M10.5,13.7v-2.49l1.76,1.76C11.76,13.36,11.16,13.61,10.5,13.7z M12.97,12.26 l-1.76-1.76h2.49C13.61,11.16,13.36,11.76,12.97,12.26z M11.21,9.5l1.76-1.76c0.38,0.5,0.64,1.1,0.73,1.76H11.21z",
                }
            }
        }
    }
}

pub fn heat_pump_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12.75,7.08 c0.82,0.12,1.57,0.44,2.2,0.91l-2.2,2.2V7.08z M11.25,7.08v3.11l-2.2-2.2C9.68,7.52,10.43,7.2,11.25,7.08z M7.99,9.05l2.2,2.2H7.08 C7.2,10.43,7.52,9.68,7.99,9.05z M7.08,12.75h3.11l-2.2,2.2C7.52,14.32,7.2,13.57,7.08,12.75z M11.25,16.92 c-0.82-0.12-1.57-0.44-2.2-0.91l2.2-2.2V16.92z M12,13c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C13,12.55,12.55,13,12,13z M12.75,16.92v-3.11l2.2,2.2C14.32,16.48,13.57,16.8,12.75,16.92z M16.01,14.95l-2.2-2.2h3.11C16.8,13.57,16.48,14.32,16.01,14.95z M13.81,11.25l2.2-2.2c0.47,0.64,0.79,1.39,0.91,2.2H13.81z",
                }
            }
        }
    }
}

pub fn mode_fan_off_icons_20px(props: IconProps) -> Element {
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
                        d: "M15.46,13.34c0.17,0.04,0.35,0.06,0.54,0.06c0.98,0,2-0.76,2-2.08C18,8.52,16.49,7,14.8,7c-0.83,0-1.29,0.22-3.01,0.91 c-0.14-0.12-0.28-0.22-0.44-0.31c0.22-0.99,0.64-1.47,1.1-1.78C13.09,5.39,13.4,4.75,13.4,4c0-0.98-0.76-2-2.08-2 C8.69,2,7.2,3.34,7.03,4.91L15.46,13.34z",
                    }
                    path {
                        d: "M2.4,3.46L2.4,3.46c-0.29,0.29-0.29,0.77,0,1.06l2.14,2.14C4.37,6.62,4.19,6.6,4,6.6c-0.98,0-2,0.76-2,2.08 C2,11.48,3.51,13,5.2,13c0.83,0,1.29-0.22,3.01-0.91c0.14,0.12,0.28,0.22,0.44,0.31c-0.22,0.99-0.64,1.47-1.1,1.78 C6.91,14.61,6.6,15.25,6.6,16c0,0.98,0.76,2,2.08,2c2.63,0,4.12-1.34,4.29-2.91l2.51,2.51c0.29,0.29,0.77,0.29,1.06,0l0,0 c0.29-0.29,0.29-0.77,0-1.06L3.46,3.46C3.17,3.17,2.69,3.17,2.4,3.46z",
                    }
                }
            }
        }
    }
}

pub fn mode_fan_off_icons_24px(props: IconProps) -> Element {
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M16.34,8.36l-2.29,0.82c-0.18-0.13-0.38-0.25-0.58-0.34c0.17-0.83,0.63-1.58,1.36-2.06C16.85,5.44,16.18,2,13.39,2 c-3.08,0-4.9,1.47-5.3,3.26L18.73,15.9c1.5,0.39,3.27-0.51,3.27-2.51C22,9,18.99,7.16,16.34,8.36z",
                        }
                    }
                    g {
                        path {
                            d: "M2.1,3.51L2.1,3.51c-0.39,0.39-0.39,1.02,0,1.41L5.27,8.1C3.77,7.7,2,8.61,2,10.61c0,4.4,3.01,6.24,5.66,5.03l2.29-0.82 c0.18,0.13,0.38,0.25,0.58,0.34c-0.17,0.83-0.63,1.58-1.36,2.06C7.15,18.56,7.82,22,10.61,22c3.08,0,4.9-1.47,5.3-3.26l3.16,3.16 c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41L3.51,3.51C3.12,3.12,2.49,3.12,2.1,3.51z",
                        }
                    }
                }
            }
        }
    }
}

pub fn nest_cam_wired_stand_icons_20px(props: IconProps) -> Element {
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
                        d: "M13.31,1.01L9.72,1.47C7.02,1.79,5,4.06,5,6.75c0,2.69,2.02,4.96,4.72,5.28l1.37,0.17l-0.57,0.85 C10.35,13.03,10.18,13,10,13c-2.21,0-4,1.79-4,4v1.5C6,18.78,6.22,19,6.5,19h7c0.28,0,0.5-0.22,0.5-0.5V17 c0-1.48-0.81-2.76-2.01-3.45l0.76-1.13l0.56,0.07c0.9,0.11,1.69-0.58,1.69-1.48V2.49C15,1.59,14.21,0.9,13.31,1.01z",
                    }
                }
            }
        }
    }
}

pub fn nest_cam_wired_stand_icons_24px(props: IconProps) -> Element {
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M15.83,1.01l-4.11,0.42C8.47,1.75,6,4.48,6,7.75s2.47,6,5.72,6.33l1.9,0.19l-0.56,0.85C12.71,15.04,12.36,15,12,15 c-2.76,0-5,2.24-5,5v2c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v-2c0-1.67-0.83-3.15-2.09-4.06l0.97-1.45 C17.02,14.56,18,13.66,18,12.5V3C18,1.83,17,0.91,15.83,1.01z",
                }
            }
        }
    }
}

pub fn oil_barrel_icons_20px(props: IconProps) -> Element {
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
                    d: "M16.25,10.75c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75H15.5V4.5h0.75C16.66,4.5,17,4.16,17,3.75S16.66,3,16.25,3 H3.75C3.34,3,3,3.34,3,3.75S3.34,4.5,3.75,4.5H4.5v4.75H3.75C3.34,9.25,3,9.59,3,10s0.34,0.75,0.75,0.75H4.5v4.75H3.75 C3.34,15.5,3,15.84,3,16.25S3.34,17,3.75,17h12.5c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75H15.5v-4.75H16.25z M10,13 c-1.24,0-2.25-0.99-2.25-2.21c0-0.91,0.34-1.21,1.87-2.98c0.2-0.23,0.55-0.23,0.75,0c1.53,1.76,1.87,2.07,1.87,2.98 C12.25,12.01,11.24,13,10,13z",
                }
            }
        }
    }
}

pub fn oil_barrel_icons_24px(props: IconProps) -> Element {
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M20,13c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1V5h1c0.55,0,1-0.45,1-1s-0.45-1-1-1H4C3.45,3,3,3.45,3,4s0.45,1,1,1h1v6H4 c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v6H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h16c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1v-6H20z M12,16 c-1.66,0-3-1.32-3-2.95c0-1.16,0.41-1.58,2.24-3.68c0.4-0.46,1.12-0.46,1.51,0c1.82,2.09,2.24,2.52,2.24,3.68 C15,14.68,13.66,16,12,16z",
                }
            }
        }
    }
}

pub fn propane_icons_20px(props: IconProps) -> Element {
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
                    d: "M13.79,5L13,5V4c0-0.83-0.67-1.5-1.5-1.5h-3C7.67,2.5,7,3.17,7,4v1L6.21,5C3.6,5,1.27,6.91,1.02,9.51 C0.74,12.49,3.08,15,6,15v1.25C6,16.66,6.34,17,6.75,17h0c0.41,0,0.75-0.34,0.75-0.75V15h5v1.25c0,0.41,0.34,0.75,0.75,0.75h0 c0.41,0,0.75-0.34,0.75-0.75V15c2.92,0,5.26-2.51,4.98-5.49C18.73,6.91,16.4,5,13.79,5z M8.5,4h3v1h-3V4z",
                }
            }
        }
    }
}

pub fn propane_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M16.75,6L16,6V5c0-1.1-0.9-2-2-2h-4C8.9,3,8,3.9,8,5v1L7.25,6C3.97,6,1.1,8.53,1,11.82C0.9,15.21,3.62,18,7,18v2 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h6v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2c3.38,0,6.1-2.79,6-6.18 C22.9,8.53,20.03,6,16.75,6z M10,5h4v1h-4V5z",
                }
            }
        }
    }
}

pub fn propane_tank_icons_20px(props: IconProps) -> Element {
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
                        d: "M4,12.25V15c0,1.66,1.34,3,3,3h6c1.66,0,3-1.34,3-3v-2.75H4z",
                    }
                    path {
                        d: "M16,10.75V8c0-1.3-0.84-2.4-2-2.82V3.5C14,2.67,13.33,2,12.5,2h-5C6.67,2,6,2.67,6,3.5v1.68C4.84,5.6,4,6.7,4,8v2.75H16z M7.5,3.5h5V5h-1.75c0-0.41-0.34-0.75-0.75-0.75S9.25,4.59,9.25,5H7.5V3.5z",
                    }
                }
            }
        }
    }
}

pub fn propane_tank_icons_24px(props: IconProps) -> Element {
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M4,15v3c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4v-3H4z",
                    }
                    path {
                        d: "M20,13v-3c0-1.86-1.28-3.41-3-3.86V4c0-1.1-0.9-2-2-2H9C7.9,2,7,2.9,7,4v2.14c-1.72,0.45-3,2-3,3.86v3H20z M9,4h6v2h-2 c0-0.55-0.45-1-1-1s-1,0.45-1,1H9V4z",
                    }
                }
            }
        }
    }
}

pub fn roller_shades_icons_20px(props: IconProps) -> Element {
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
                    d: "M16,15.5v-11C16,3.67,15.33,3,14.5,3h-9C4.67,3,4,3.67,4,4.5v11H2.75C2.34,15.5,2,15.84,2,16.25l0,0 C2,16.66,2.34,17,2.75,17h14.5c0.41,0,0.75-0.34,0.75-0.75l0,0c0-0.41-0.34-0.75-0.75-0.75H16z M5.5,15.5v-4.75h3.75v1.53 c-0.3,0.23-0.5,0.57-0.5,0.97c0,0.69,0.56,1.25,1.25,1.25s1.25-0.56,1.25-1.25c0-0.4-0.2-0.75-0.5-0.97v-1.53h3.75v4.75H5.5z",
                }
            }
        }
    }
}

pub fn roller_shades_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H20z M6,19v-6h5v1.8c-0.4,0.3-0.8,0.8-0.8,1.4c0,1,0.8,1.8,1.8,1.8s1.8-0.8,1.8-1.8c0-0.6-0.3-1.1-0.8-1.4V13h5 v6H6z",
                }
            }
        }
    }
}

pub fn roller_shades_closed_icons_20px(props: IconProps) -> Element {
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
                    d: "M16,15.5v-11C16,3.67,15.33,3,14.5,3h-9C4.67,3,4,3.67,4,4.5v11H2.75C2.34,15.5,2,15.84,2,16.25l0,0 C2,16.66,2.34,17,2.75,17h6c0,0.69,0.56,1.25,1.25,1.25s1.25-0.56,1.25-1.25h6c0.41,0,0.75-0.34,0.75-0.75l0,0 c0-0.41-0.34-0.75-0.75-0.75H16z M9.25,15.5H5.5V14h3.75V15.5z M14.5,15.5h-3.75V14h3.75V15.5z",
                }
            }
        }
    }
}

pub fn roller_shades_closed_icons_24px(props: IconProps) -> Element {
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
                        d: "M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h7.25 c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H21c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H20z M6,19v-2h5v2H6z M13,19v-2h5v2H13 z",
                    }
                }
            }
        }
    }
}

pub fn sensor_door_icons_20px(props: IconProps) -> Element {
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
                path {
                    d: "M15,3H5C4.45,3,4,3.45,4,4v12c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V4C16,3.45,15.55,3,15,3z M13,11c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C14,10.55,13.55,11,13,11z",
                }
            }
        }
    }
}

pub fn sensor_door_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M18,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2z M15.5,13.5 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5S17,11.17,17,12S16.33,13.5,15.5,13.5z",
                }
            }
        }
    }
}

pub fn sensor_occupied_icons_20px(props: IconProps) -> Element {
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
                        d: "M18.51,6.45L18.51,6.45c0.39-0.16,0.58-0.63,0.39-1c-0.96-1.87-2.48-3.4-4.35-4.35c-0.38-0.19-0.84,0-1,0.39l-0.02,0.05 c-0.14,0.34,0.02,0.73,0.35,0.9c1.58,0.81,2.87,2.11,3.69,3.69C17.74,6.47,18.16,6.6,18.51,6.45z",
                    }
                    path {
                        d: "M13.55,18.51L13.55,18.51c0.16,0.39,0.63,0.58,1,0.39c1.87-0.96,3.4-2.48,4.35-4.35c0.19-0.38,0-0.84-0.39-1l0,0 c-0.35-0.15-0.78-0.01-0.95,0.33c-0.81,1.58-2.11,2.87-3.69,3.69C13.53,17.74,13.4,18.16,13.55,18.51z",
                    }
                    path {
                        d: "M1.49,13.55L1.49,13.55c-0.39,0.16-0.58,0.63-0.39,1c0.96,1.87,2.48,3.4,4.35,4.35c0.38,0.19,0.84,0,1-0.39l0,0 c0.15-0.35,0.01-0.78-0.33-0.95c-1.58-0.81-2.87-2.11-3.69-3.69C2.26,13.53,1.84,13.4,1.49,13.55z",
                    }
                    path {
                        d: "M6.45,1.49L6.45,1.49c-0.16-0.39-0.63-0.58-1-0.39C3.58,2.05,2.05,3.58,1.1,5.45c-0.19,0.38,0,0.84,0.39,1l0,0 C1.84,6.6,2.26,6.47,2.44,6.13c0.81-1.58,2.11-2.87,3.69-3.69C6.47,2.26,6.6,1.84,6.45,1.49z",
                    }
                    path {
                        d: "M10,9c1.38,0,2.5-1.12,2.5-2.5C12.5,5.12,11.38,4,10,4S7.5,5.12,7.5,6.5C7.5,7.88,8.62,9,10,9z",
                    }
                    path {
                        d: "M10,10c-1.53,0-2.95,0.41-4.18,1.13C5.31,11.43,5,11.98,5,12.57l0,0.93C5,13.78,5.22,14,5.5,14h9c0.28,0,0.5-0.22,0.5-0.5 l0-0.93c0-0.59-0.31-1.14-0.82-1.44C12.95,10.41,11.53,10,10,10z",
                    }
                }
            }
        }
    }
}

pub fn sensor_occupied_icons_24px(props: IconProps) -> Element {
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
                        d: "M12,11c1.66,0,3-1.34,3-3s-1.34-3-3-3S9,6.34,9,8S10.34,11,12,11z",
                    }
                    path {
                        d: "M12,12c-1.84,0-3.56,0.5-5.03,1.37C6.36,13.73,6,14.39,6,15.09L6,16c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1l0-0.91 c0-0.7-0.36-1.36-0.97-1.72C15.56,12.5,13.84,12,12,12z",
                    }
                    path {
                        d: "M22.11,7.79L22.11,7.79c0.55-0.23,0.78-0.88,0.5-1.41c-1.13-2.12-2.87-3.86-4.99-4.99c-0.52-0.28-1.17-0.04-1.4,0.5v0 c-0.19,0.47-0.01,1.02,0.43,1.25c1.79,0.94,3.26,2.42,4.21,4.21C21.09,7.8,21.64,7.98,22.11,7.79z",
                    }
                    path {
                        d: "M7.79,1.89L7.79,1.89c-0.23-0.55-0.88-0.78-1.4-0.5C4.27,2.52,2.52,4.26,1.4,6.38c-0.28,0.52-0.05,1.18,0.5,1.41l0,0 c0.47,0.2,1.02,0.01,1.25-0.43c0.94-1.79,2.42-3.26,4.21-4.21C7.8,2.91,7.98,2.36,7.79,1.89z",
                    }
                    path {
                        d: "M1.89,16.21L1.89,16.21c-0.55,0.23-0.78,0.88-0.5,1.4c1.13,2.12,2.87,3.87,5,5c0.52,0.28,1.17,0.04,1.4-0.5l0,0 c0.19-0.47,0.01-1.02-0.43-1.25c-1.79-0.94-3.26-2.42-4.21-4.21C2.91,16.2,2.36,16.02,1.89,16.21z",
                    }
                    path {
                        d: "M16.21,22.11L16.21,22.11c0.23,0.55,0.88,0.78,1.4,0.5c2.12-1.13,3.87-2.87,5-5c0.28-0.52,0.04-1.17-0.5-1.4h0 c-0.47-0.19-1.02-0.01-1.25,0.43c-0.94,1.79-2.42,3.26-4.21,4.21C16.2,21.09,16.02,21.64,16.21,22.11z",
                    }
                }
            }
        }
    }
}

pub fn sensor_window_icons_20px(props: IconProps) -> Element {
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
                    d: "M15,4v12H5V4H15 M15,3H5C4.45,3,4,3.45,4,4v12c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V4C16,3.45,15.55,3,15,3L15,3z M6,10 v5h8v-5H6z M8.5,9V8h3v1H14V5H6v4H8.5z",
                }
            }
        }
    }
}

pub fn sensor_window_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M18,4v16H6V4H18 M18,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2L18,2z M7,19h10v-6H7 V19z M10,10h4v1h3V5H7v6h3V10z",
                }
            }
        }
    }
}

pub fn shield_moon_icons_20px(props: IconProps) -> Element {
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
                path {
                    d: "M9.46,2.21l-5,1.92C3.88,4.35,3.5,4.91,3.5,5.53v3.74c0,3.88,2.56,7.52,6.07,8.61c0.28,0.09,0.58,0.09,0.86,0 c3.51-1.09,6.07-4.73,6.07-8.61V5.53c0-0.62-0.38-1.18-0.96-1.4l-5-1.92C10.19,2.07,9.81,2.07,9.46,2.21z M13.06,11.76 c-1.43,1.69-4.05,1.63-5.41-0.06c-1.71-2.12-0.51-5.23,2.09-5.7c0.27-0.05,0.49,0.21,0.39,0.47c-0.36,0.95-0.3,2.05,0.25,3 s1.47,1.55,2.48,1.71C13.12,11.23,13.24,11.55,13.06,11.76z",
                }
            }
        }
    }
}

pub fn shield_moon_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M11.3,2.26l-6,2.25C4.52,4.81,4,5.55,4,6.39v4.7c0,4.83,3.13,9.37,7.43,10.75c0.37,0.12,0.77,0.12,1.14,0 c4.3-1.38,7.43-5.91,7.43-10.75v-4.7c0-0.83-0.52-1.58-1.3-1.87l-6-2.25C12.25,2.09,11.75,2.09,11.3,2.26z M15.97,14.41 c-1.84,2.17-5.21,2.1-6.96-0.07c-2.19-2.72-0.65-6.72,2.69-7.33c0.34-0.06,0.63,0.27,0.51,0.6c-0.46,1.23-0.39,2.64,0.32,3.86 c0.71,1.22,1.89,1.99,3.18,2.2C16.05,13.72,16.2,14.14,15.97,14.41z",
                }
            }
        }
    }
}

pub fn solar_power_icons_20px(props: IconProps) -> Element {
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
                        d: "M10.75,10v3.25h6.36l-0.38-2.03C16.6,10.51,15.98,10,15.26,10H10.75z",
                    }
                    path {
                        d: "M2.89,13.25h6.36V10H4.74c-0.72,0-1.34,0.51-1.47,1.22L2.89,13.25z",
                    }
                    path {
                        d: "M3.81,18h5.44v-3.25H2.61l-0.28,1.47C2.16,17.15,2.87,18,3.81,18z",
                    }
                    path {
                        d: "M17.39,14.75h-6.64V18h5.44c0.94,0,1.65-0.85,1.47-1.78L17.39,14.75z",
                    }
                    path {
                        d: "M10,7L10,7C9.59,7,9.25,7.34,9.25,7.75v0.5C9.25,8.66,9.59,9,10,9h0c0.41,0,0.75-0.34,0.75-0.75v-0.5 C10.75,7.34,10.41,7,10,7z",
                    }
                    path {
                        d: "M15.16,7.16L15.16,7.16c0.29-0.29,0.29-0.77,0-1.06l-0.35-0.35c-0.29-0.29-0.77-0.29-1.06,0l0,0 c-0.29,0.29-0.29,0.77,0,1.06l0.35,0.35C14.4,7.46,14.87,7.46,15.16,7.16z",
                    }
                    path {
                        d: "M5.9,7.16l0.35-0.35c0.29-0.29,0.29-0.77,0-1.06l0,0c-0.29-0.29-0.77-0.29-1.06,0L4.84,6.1c-0.29,0.29-0.29,0.77,0,1.06 l0,0C5.13,7.46,5.6,7.46,5.9,7.16z",
                    }
                    path {
                        d: "M4.25,2.05h-0.5C3.34,2.05,3,2.39,3,2.8v0c0,0.41,0.34,0.75,0.75,0.75h0.5C4.66,3.55,5,3.21,5,2.8v0 C5,2.39,4.66,2.05,4.25,2.05z",
                    }
                    path {
                        d: "M16.25,2.05h-0.5C15.34,2.05,15,2.39,15,2.8v0c0,0.41,0.34,0.75,0.75,0.75h0.5C16.66,3.55,17,3.21,17,2.8v0 C17,2.39,16.66,2.05,16.25,2.05z",
                    }
                    path {
                        d: "M10,6c2.21,0,4-1.79,4-4H6C6,4.21,7.79,6,10,6z",
                    }
                }
            }
        }
    }
}

pub fn solar_power_icons_24px(props: IconProps) -> Element {
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M3.33,16H11v-3H5.6c-0.94,0-1.75,0.65-1.95,1.57L3.33,16z",
                    }
                    path {
                        d: "M13,16h7.67l-0.32-1.43C20.14,13.65,19.33,13,18.4,13H13V16z",
                    }
                    path {
                        d: "M21.11,18H13v4h6.51c1.28,0,2.23-1.18,1.95-2.43L21.11,18z",
                    }
                    path {
                        d: "M4.49,22H11v-4H2.89l-0.35,1.57C2.26,20.82,3.21,22,4.49,22z",
                    }
                    path {
                        d: "M12,8L12,8c-0.55,0-1,0.45-1,1v1c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V9C13,8.45,12.55,8,12,8z",
                    }
                    path {
                        d: "M18.59,8.62L18.59,8.62c0.39-0.39,0.39-1.02,0-1.41L17.88,6.5c-0.39-0.39-1.02-0.39-1.41,0v0c-0.39,0.39-0.39,1.02,0,1.41 l0.71,0.71C17.57,9.01,18.2,9.01,18.59,8.62z",
                    }
                    path {
                        d: "M6.82,8.62l0.71-0.71c0.39-0.39,0.39-1.02,0-1.41l0,0c-0.39-0.39-1.02-0.39-1.41,0L5.41,7.2c-0.39,0.39-0.39,1.02,0,1.41 l0,0C5.8,9.01,6.43,9.01,6.82,8.62z",
                    }
                    path {
                        d: "M5,2H4C3.45,2,3,2.45,3,3v0c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v0C6,2.45,5.55,2,5,2z",
                    }
                    path {
                        d: "M20,2h-1c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v0C21,2.45,20.55,2,20,2z",
                    }
                    path {
                        d: "M12,7c2.76,0,5-2.24,5-5H7C7,4.76,9.24,7,12,7z",
                    }
                }
            }
        }
    }
}

pub fn vertical_shades_icons_20px(props: IconProps) -> Element {
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
                path {
                    d: "M16,15.5v-11C16,3.67,15.33,3,14.5,3h-9C4.67,3,4,3.67,4,4.5v11H2.75C2.34,15.5,2,15.84,2,16.25l0,0 C2,16.66,2.34,17,2.75,17h14.5c0.41,0,0.75-0.34,0.75-0.75l0,0c0-0.41-0.34-0.75-0.75-0.75H16z M8.5,4.5h3v11h-3V4.5z",
                }
            }
        }
    }
}

pub fn vertical_shades_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H20z M10,19V5h4v14H10z",
                }
            }
        }
    }
}

pub fn vertical_shades_closed_icons_20px(props: IconProps) -> Element {
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
                    d: "M16,15.5v-11C16,3.67,15.33,3,14.5,3h-9C4.67,3,4,3.67,4,4.5v11H2.75C2.34,15.5,2,15.84,2,16.25l0,0 C2,16.66,2.34,17,2.75,17h14.5c0.41,0,0.75-0.34,0.75-0.75l0,0c0-0.41-0.34-0.75-0.75-0.75H16z M10.75,4.5h1.12v11h-1.12V4.5z M9.25,15.5H8.12v-11h1.12V15.5z M5.5,4.5h1.12v11H5.5V4.5z M13.38,15.5v-11h1.12v11H13.38z",
                }
            }
        }
    }
}

pub fn vertical_shades_closed_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H20z M13,5h1.5v14H13V5z M11,19H9.5V5H11V19z M6,5h1.5v14H6V5z M16.5,19V5H18v14H16.5z",
                }
            }
        }
    }
}

pub fn wind_power_icons_20px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M7.25,3h-3.5C3.34,3,3,3.34,3,3.75v0C3,4.16,3.34,4.5,3.75,4.5h3.5C7.66,4.5,8,4.16,8,3.75v0C8,3.34,7.66,3,7.25,3z",
                        }
                    }
                    g {
                        path {
                            d: "M4.25,6.5h-2.5C1.34,6.5,1,6.84,1,7.25v0C1,7.66,1.34,8,1.75,8h2.5C4.66,8,5,7.66,5,7.25v0C5,6.84,4.66,6.5,4.25,6.5z",
                        }
                    }
                    g {
                        path {
                            d: "M3.75,17h2.5C6.66,17,7,16.66,7,16.25l0,0c0-0.41-0.34-0.75-0.75-0.75h-2.5C3.34,15.5,3,15.84,3,16.25l0,0 C3,16.66,3.34,17,3.75,17z",
                        }
                    }
                    g {
                        path {
                            d: "M10.06,12.43c0.79,0.24,1.63-0.2,1.87-1c0.24-0.79-0.2-1.63-1-1.87c-0.79-0.24-1.63,0.2-1.87,1 C8.82,11.35,9.27,12.19,10.06,12.43z",
                        }
                    }
                    g {
                        path {
                            d: "M8.11,10.27C8.27,9.75,8.59,9.31,9,9H2.8C1.81,9,1,9.81,1,10.8c0,0.8,0.53,1.51,1.31,1.73l3.6,1.03 c0.4,0.11,0.83,0.06,1.18-0.16l1.44-0.87C8.05,11.92,7.86,11.08,8.11,10.27z",
                        }
                    }
                    g {
                        path {
                            d: "M17.77,15.32l-1.82-3.27c-0.2-0.36-0.54-0.63-0.95-0.73l-2.02-0.5c0.02,0.3,0,0.61-0.1,0.92 c-0.5,1.62-2.03,1.77-2.39,1.77c-0.25,0-0.49-0.04-0.73-0.11c-0.01,0-0.01-0.01-0.02-0.01v4.04H9.58C8.71,17.42,8,18.13,8,19h5 c0-0.87-0.71-1.58-1.58-1.58h-0.17V13.8l3.67,3.67c0.7,0.7,1.85,0.7,2.55,0C18.04,16.9,18.16,16.03,17.77,15.32z",
                        }
                    }
                    g {
                        path {
                            d: "M11.23,8.61c0.63,0.19,1.13,0.62,1.43,1.16l3.32-5.54c0.51-0.85,0.24-1.96-0.62-2.47c-0.69-0.41-1.57-0.32-2.16,0.23 l-2.74,2.56C10.17,4.83,10,5.23,10,5.64v2.91C10.18,8.52,10.64,8.43,11.23,8.61z",
                        }
                    }
                }
            }
        }
    }
}

pub fn wind_power_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M9,3H5C4.45,3,4,3.45,4,4v0c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v0C10,3.45,9.55,3,9,3z",
                    }
                    path {
                        d: "M5,7H2C1.45,7,1,7.45,1,8v0c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v0C6,7.45,5.55,7,5,7z",
                    }
                    path {
                        d: "M4,21h3c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4c-0.55,0-1,0.45-1,1v0C3,20.55,3.45,21,4,21z",
                    }
                    path {
                        d: "M13.73,10.61c0.75,0.23,1.3,0.78,1.57,1.46l4.27-7.11c0.65-1.08,0.3-2.48-0.78-3.13c-0.87-0.52-1.99-0.41-2.73,0.29 l-3.43,3.21C12.23,5.7,12,6.23,12,6.78v3.93C12.36,10.56,12.98,10.38,13.73,10.61z",
                    }
                    path {
                        d: "M10.61,12.27c0.16-0.52,0.48-0.96,0.89-1.27H3.28C2.02,11,1,12.02,1,13.28c0,1.02,0.67,1.91,1.65,2.19l4.51,1.29 c0.53,0.15,1.1,0.08,1.58-0.21l2.69-1.61C10.66,14.32,10.3,13.27,10.61,12.27z",
                    }
                    path {
                        d: "M22.21,18.61l-2.28-4.1c-0.27-0.48-0.73-0.83-1.26-0.97l-3.18-0.8c0.03,0.32,0,0.66-0.1,0.99 c-0.32,1.06-1.28,1.77-2.39,1.77c-0.61,0-0.99-0.22-1-0.22V21c-1.1,0-2,0.9-2,2h6c0-1.1-0.9-2-2-2v-4.28l4.61,4.61 c0.89,0.89,2.33,0.89,3.22,0C22.55,20.61,22.71,19.5,22.21,18.61z",
                    }
                    path {
                        d: "M12.56,14.43c0.79,0.24,1.63-0.2,1.87-1c0.24-0.79-0.2-1.63-1-1.87c-0.79-0.24-1.63,0.2-1.87,1 C11.32,13.35,11.77,14.19,12.56,14.43z",
                    }
                }
            }
        }
    }
}

