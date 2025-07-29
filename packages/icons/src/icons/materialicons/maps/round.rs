use dioxus::prelude::*;
use crate::IconProps;
pub fn _360_icons_24px(props: IconProps) -> Element {
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
                d: "M12 7C6.48 7 2 9.24 2 12c0 2.24 2.94 4.13 7 4.77v2.02c0 .45.54.67.85.35l2.79-2.79c.2-.2.2-.51 0-.71l-2.79-2.79c-.31-.31-.85-.09-.85.36v1.52c-3.15-.56-5-1.9-5-2.73 0-1.06 3.04-3 8-3s8 1.94 8 3c0 .66-1.2 1.68-3.32 2.34-.41.13-.68.51-.68.94 0 .67.65 1.16 1.28.96C20.11 15.36 22 13.79 22 12c0-2.76-4.48-5-10-5z",
            }
        }
    }
}

pub fn add_business_icons_20px(props: IconProps) -> Element {
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
                        d: "M4.5,5h9C13.78,5,14,4.78,14,4.5C14,4.22,13.78,4,13.5,4h-9C4.22,4,4,4.22,4,4.5C4,4.78,4.22,5,4.5,5z",
                    }
                    path {
                        d: "M13,14h1v-3h0.61c0.33,0,0.56-0.31,0.49-0.62l-1-4C14.04,6.16,13.84,6,13.61,6H4.39C4.16,6,3.96,6.16,3.91,6.38l-1,4 C2.83,10.69,3.07,11,3.39,11H4v4.5C4,15.78,4.22,16,4.5,16h5c0.28,0,0.5-0.22,0.5-0.5V11h3V14z M9,15H5v-4h4V15z",
                    }
                    path {
                        d: "M16,15v-1.5c0-0.28-0.22-0.5-0.5-0.5h0c-0.28,0-0.5,0.22-0.5,0.5V15h-1.5c-0.28,0-0.5,0.22-0.5,0.5v0 c0,0.28,0.22,0.5,0.5,0.5H15v1.5c0,0.28,0.22,0.5,0.5,0.5h0c0.28,0,0.5-0.22,0.5-0.5V16h1.5c0.28,0,0.5-0.22,0.5-0.5v0 c0-0.28-0.22-0.5-0.5-0.5H16z",
                    }
                }
            }
        }
    }
}

pub fn add_business_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M3,6h13c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H3C2.45,4,2,4.45,2,5C2,5.55,2.45,6,3,6z",
                    }
                    path {
                        d: "M15,17h2v-3h0.18c0.63,0,1.1-0.58,0.98-1.2l-1-5C17.07,7.34,16.66,7,16.18,7H2.82C2.34,7,1.93,7.34,1.84,7.8l-1,5 C0.72,13.42,1.19,14,1.82,14H2v5c0,0.55,0.45,1,1,1h7c0.55,0,1-0.45,1-1v-5h4V17z M9,18H4v-4h5V18z",
                    }
                    path {
                        d: "M22,18h-2v-2c0-0.55-0.45-1-1-1s-1,0.45-1,1v2h-2c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1 v-2h2c0.55,0,1-0.45,1-1C23,18.45,22.55,18,22,18z",
                    }
                }
            }
        }
    }
}

pub fn add_location_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M13,7c0-0.55-0.44-1-1-1c-0.55,0-1,0.44-1,1v2H9c-0.55,0-1,0.44-1,1c0,0.55,0.44,1,1,1h2v2 c0,0.55,0.44,1,1,1c0.55,0,1-0.44,1-1v-2h2c0.55,0,1-0.44,1-1c0-0.55-0.44-1-1-1h-2V7z M12,2c4.2,0,8,3.22,8,8.2 c0,3.18-2.45,6.92-7.34,11.23c-0.38,0.33-0.95,0.33-1.33,0C6.45,17.12,4,13.38,4,10.2C4,5.22,7.8,2,12,2z",
                        }
                    }
                }
            }
        }
    }
}

pub fn add_location_alt_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M19,0c0.55,0,1,0.45,1,1v2h2c0.55,0,1,0.45,1,1s-0.45,1-1,1h-2v2c0,0.55-0.45,1-1,1s-1-0.45-1-1V5h-2c-0.55,0-1-0.45-1-1 s0.45-1,1-1h2V1C18,0.45,18.45,0,19,0z M12,12c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S10.9,12,12,12z M14.72,2.47 C14.28,2.83,14,3.38,14,4c0,1.1,0.9,2,2,2h1v1c0,1.1,0.9,2,2,2c0.32,0,0.62-0.08,0.89-0.21C19.96,9.24,20,9.71,20,10.2 c0,3.18-2.45,6.92-7.34,11.23c-0.38,0.33-0.95,0.33-1.33,0C6.45,17.12,4,13.38,4,10.2C4,5.22,7.8,2,12,2 C12.94,2,13.86,2.16,14.72,2.47z",
                }
            }
        }
    }
}

pub fn add_road_icons_20px(props: IconProps) -> Element {
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
                        d: "M4.5,16L4.5,16C4.78,16,5,15.78,5,15.5v-11C5,4.22,4.78,4,4.5,4h0C4.22,4,4,4.22,4,4.5v11C4,15.78,4.22,16,4.5,16z",
                    }
                    path {
                        d: "M15.5,4L15.5,4C15.22,4,15,4.22,15,4.5v6c0,0.28,0.22,0.5,0.5,0.5h0c0.28,0,0.5-0.22,0.5-0.5v-6C16,4.22,15.78,4,15.5,4z",
                    }
                    path {
                        d: "M10,6L10,6c0.28,0,0.5-0.22,0.5-0.5v-1C10.5,4.22,10.28,4,10,4h0C9.72,4,9.5,4.22,9.5,4.5v1C9.5,5.78,9.72,6,10,6z",
                    }
                    path {
                        d: "M10,16L10,16c0.28,0,0.5-0.22,0.5-0.5v-1c0-0.28-0.22-0.5-0.5-0.5h0c-0.28,0-0.5,0.22-0.5,0.5v1C9.5,15.78,9.72,16,10,16z",
                    }
                    path {
                        d: "M17.5,15H16v-1.5c0-0.28-0.22-0.5-0.5-0.5h0c-0.28,0-0.5,0.22-0.5,0.5V15h-1.5c-0.28,0-0.5,0.22-0.5,0.5v0 c0,0.28,0.22,0.5,0.5,0.5H15v1.5c0,0.28,0.22,0.5,0.5,0.5h0c0.28,0,0.5-0.22,0.5-0.5V16h1.5c0.28,0,0.5-0.22,0.5-0.5v0 C18,15.22,17.78,15,17.5,15z",
                    }
                    path {
                        d: "M10,11L10,11c0.28,0,0.5-0.22,0.5-0.5v-1C10.5,9.22,10.28,9,10,9h0C9.72,9,9.5,9.22,9.5,9.5v1C9.5,10.78,9.72,11,10,11z",
                    }
                }
            }
        }
    }
}

pub fn add_road_icons_24px(props: IconProps) -> Element {
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
                    path {
                        d: "M20,18v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-2h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H20z",
                    }
                    path {
                        d: "M19,4L19,4c-0.55,0-1,0.45-1,1v7c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V5C20,4.45,19.55,4,19,4z",
                    }
                    path {
                        d: "M5,20L5,20c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h0C4.45,4,4,4.45,4,5v14C4,19.55,4.45,20,5,20z",
                    }
                    path {
                        d: "M12,8L12,8c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C11,7.55,11.45,8,12,8z",
                    }
                    path {
                        d: "M12,14L12,14c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C11,13.55,11.45,14,12,14z",
                    }
                    path {
                        d: "M12,20L12,20c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C11,19.55,11.45,20,12,20z",
                    }
                }
            }
        }
    }
}

pub fn agriculture_icons_20px(props: IconProps) -> Element {
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
                        d: "M15.5,10c0.54,0,1.04,0.13,1.5,0.35V7c0-0.55-0.45-1-1-1h-4.47l-0.8-0.8l0.8-0.8c0.15-0.15,0.15-0.38,0-0.53v0 c-0.15-0.15-0.38-0.15-0.53,0L8.88,6c-0.15,0.15-0.15,0.38,0,0.53v0c0.15,0.15,0.38,0.15,0.53,0l0.8-0.8l0.8,0.8V9 c0,0.55-0.45,1-1,1H9.24C9.72,10.72,10,11.57,10,12.5c0,0.17-0.03,0.33-0.05,0.5h2.1C12.29,11.31,13.74,10,15.5,10z",
                    }
                    path {
                        d: "M3.5,8H8c0-0.55-0.45-1-1-1H3.5C3.22,7,3,7.22,3,7.5C3,7.78,3.22,8,3.5,8z",
                    }
                    path {
                        d: "M15.5,11c-1.38,0-2.5,1.12-2.5,2.5c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5C18,12.12,16.88,11,15.5,11z M15.5,15 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,15,15.5,15z",
                    }
                    path {
                        d: "M8.34,11.53L8.2,11.18l0.45-0.19C8.3,10.28,7.72,9.7,7.01,9.35L6.82,9.8L6.47,9.66l0.18-0.44C6.29,9.08,5.91,9,5.5,9 S4.71,9.08,4.34,9.21l0.18,0.44L4.18,9.8L3.99,9.35c-0.72,0.34-1.3,0.92-1.64,1.64l0.45,0.19l-0.14,0.35l-0.44-0.18 C2.08,11.71,2,12.09,2,12.5s0.08,0.79,0.21,1.16l0.44-0.18l0.14,0.35l-0.45,0.19c0.34,0.72,0.92,1.3,1.64,1.64l0.19-0.45 l0.35,0.14l-0.18,0.44C4.71,15.92,5.09,16,5.5,16s0.79-0.08,1.16-0.21l-0.18-0.44l0.35-0.14l0.19,0.45 c0.72-0.34,1.3-0.92,1.64-1.64L8.2,13.82l0.14-0.35l0.44,0.18C8.92,13.29,9,12.91,9,12.5s-0.08-0.79-0.21-1.16L8.34,11.53z M5.5,15C4.12,15,3,13.88,3,12.5C3,11.12,4.12,10,5.5,10S8,11.12,8,12.5C8,13.88,6.88,15,5.5,15z",
                    }
                }
            }
        }
    }
}

pub fn agriculture_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M19.5,11.97c0.93,0,1.78,0.28,2.5,0.76V7.97c0-1.1-0.9-2-2-2h-6.29l-1.06-1.06l1.06-1.06c0.2-0.2,0.2-0.51,0-0.71 s-0.51-0.2-0.71,0l-2.83,2.83c-0.2,0.2-0.2,0.51,0,0.71l0,0c0.2,0.2,0.51,0.2,0.71,0l1.06-1.06L13,6.68v2.29c0,1.1-0.9,2-2,2 h-0.54c0.95,1.06,1.54,2.46,1.54,4c0,0.34-0.04,0.67-0.09,1h3.14C15.3,13.73,17.19,11.97,19.5,11.97z",
                    }
                    path {
                        d: "M19.5,12.97c-1.93,0-3.5,1.57-3.5,3.5s1.57,3.5,3.5,3.5s3.5-1.57,3.5-3.5S21.43,12.97,19.5,12.97z M19.5,17.97 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S20.33,17.97,19.5,17.97z",
                    }
                    path {
                        d: "M4,8.97h5c0-1.1-0.9-2-2-2H4c-0.55,0-1,0.45-1,1C3,8.53,3.45,8.97,4,8.97z",
                    }
                    path {
                        d: "M9.83,13.79l-0.18-0.47l0.93-0.35c-0.46-1.06-1.28-1.91-2.31-2.43l-0.4,0.89l-0.46-0.21l0.4-0.9 C7.26,10.11,6.64,9.97,6,9.97c-0.53,0-1.04,0.11-1.52,0.26l0.34,0.91l-0.47,0.18L4,10.4c-1.06,0.46-1.91,1.28-2.43,2.31l0.89,0.4 l-0.21,0.46l-0.9-0.4C1.13,13.72,1,14.33,1,14.97c0,0.53,0.11,1.04,0.26,1.52l0.91-0.34l0.18,0.47l-0.93,0.35 c0.46,1.06,1.28,1.91,2.31,2.43l0.4-0.89l0.46,0.21l-0.4,0.9c0.57,0.22,1.18,0.35,1.82,0.35c0.53,0,1.04-0.11,1.52-0.26L7.18,18.8 l0.47-0.18L8,19.55c1.06-0.46,1.91-1.28,2.43-2.31l-0.89-0.4l0.21-0.46l0.9,0.4c0.22-0.57,0.35-1.18,0.35-1.82 c0-0.53-0.11-1.04-0.26-1.52L9.83,13.79z M7.15,17.75c-1.53,0.63-3.29-0.09-3.92-1.62c-0.63-1.53,0.09-3.29,1.62-3.92 c1.53-0.63,3.29,0.09,3.92,1.62C9.41,15.36,8.68,17.11,7.15,17.75z",
                    }
                }
            }
        }
    }
}

pub fn airlines_icons_20px(props: IconProps) -> Element {
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
                d: "M15.69,4h-3.63C11.24,4,10.47,4.41,10,5.08L2.5,16h12.75l1.92-10.22C17.34,4.85,16.63,4,15.69,4z M11.88,11.75 c-1.04,0-1.88-0.84-1.88-1.88S10.84,8,11.88,8s1.88,0.84,1.88,1.88S12.91,11.75,11.88,11.75z",
            }
        }
    }
}

pub fn airlines_icons_24px(props: IconProps) -> Element {
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
                y: "0",
                fill: "none",
                height: "24",
            }
            path {
                d: "M19.59,4h-5.01c-0.99,0-1.91,0.49-2.47,1.3L2,20h17l2.56-13.63C21.79,5.14,20.84,4,19.59,4z M14.5,14 c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5C17,12.88,15.88,14,14.5,14z",
            }
        }
    }
}

pub fn airline_stops_icons_20px(props: IconProps) -> Element {
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
                width: "20",
                fill: "none",
            }
            path {
                d: "M12.25,14.75c0,0.41-0.33,0.75-0.75,0.75h-3c-0.41,0-0.75-0.34-0.75-0.75C7.75,14.33,8.08,14,8.5,14h0.75 c0-3.39-2.9-6.17-6.59-6.47C2.28,7.5,2,7.16,2,6.78C2,6.33,2.39,6,2.84,6.04c3.2,0.28,5.91,2.14,7.16,4.72 c0.72-1.49,1.96-2.87,3.68-4.08l-1.32-1.32c-0.31-0.31-0.09-0.85,0.35-0.85h3.79C16.78,4.5,17,4.72,17,5v3.79 c0,0.45-0.54,0.67-0.85,0.35l-1.39-1.39c-1.78,1.21-4.01,3.28-4.01,6.24h0.75C11.92,14,12.25,14.33,12.25,14.75z",
            }
        }
    }
}

pub fn airline_stops_icons_24px(props: IconProps) -> Element {
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
                d: "M15,18c0,0.55-0.45,1-1,1h-4c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h1c-0.47-4.21-3.89-7.55-8.12-7.96 C2.37,8.99,2,8.56,2,8.05c0-0.59,0.52-1.06,1.11-1C7.03,7.44,10.37,9.87,12,13.3c1.13-2.43,2.99-4.25,4.78-5.52l-1.92-1.92 C14.54,5.54,14.76,5,15.21,5h5.29C20.78,5,21,5.22,21,5.5v5.29c0,0.45-0.54,0.67-0.85,0.35l-1.94-1.94C15.93,10.78,13.45,13.3,13,17 h1C14.55,17,15,17.45,15,18z",
            }
        }
    }
}

pub fn alt_route_icons_20px(props: IconProps) -> Element {
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
                    d: "M8.38,9.41l-0.71,0.71c-0.47-0.5-0.93-1.1-1.25-1.96L7.4,7.91C7.66,8.54,8,9.01,8.38,9.41z M9.5,6l-3-3l-3,3h2.53 c0.02,0.43,0.07,0.83,0.14,1.19l0.97-0.24C7.08,6.66,7.04,6.34,7.02,6H9.5z M16.5,6l-3-3l-3,3h2.48c-0.12,2.22-1.02,3.11-1.9,3.96 c-0.39,0.37-0.78,0.75-1.08,1.23c-0.26-0.41-0.58-0.75-0.91-1.07l-0.71,0.71C9.03,11.47,9.5,11.99,9.5,13v3.5 c0,0.28,0.22,0.5,0.5,0.5h0c0.28,0,0.5-0.22,0.5-0.5V13c0,0,0,0,0,0h0c0-1.1,0.53-1.61,1.27-2.32c0.94-0.9,2.07-2.03,2.21-4.68 H16.5z",
                }
            }
        }
    }
}

pub fn alt_route_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M9.78,11.16l-1.42,1.42c-0.68-0.69-1.34-1.58-1.79-2.94l1.94-0.49C8.83,10.04,9.28,10.65,9.78,11.16z M10.15,5.15 L7.35,2.35c-0.2-0.2-0.51-0.2-0.71,0L3.85,5.15C3.54,5.46,3.76,6,4.21,6h1.81C6.04,6.81,6.1,7.54,6.21,8.17l1.94-0.49 C8.08,7.2,8.03,6.63,8.02,6h1.78C10.24,6,10.46,5.46,10.15,5.15z M20.15,5.15l-2.79-2.79c-0.2-0.2-0.51-0.2-0.71,0l-2.79,2.79 C13.54,5.46,13.76,6,14.21,6h1.78c-0.1,3.68-1.28,4.75-2.54,5.88c-0.5,0.44-1.01,0.92-1.45,1.55c-0.34-0.49-0.73-0.88-1.13-1.24 L9.46,13.6C10.39,14.45,11,15.14,11,17c0,0,0,0,0,0h0v4c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-4c0,0,0,0,0,0 c0-2.02,0.71-2.66,1.79-3.63c1.38-1.24,3.08-2.78,3.2-7.37h1.8C20.24,6,20.46,5.46,20.15,5.15z",
                }
            }
        }
    }
}

pub fn atm_icons_24px(props: IconProps) -> Element {
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
                d: "M8 9.75c0 .41.34.75.75.75h1.5v3.75c0 .41.34.75.75.75s.75-.34.75-.75V10.5h1.5c.41 0 .75-.34.75-.75S13.66 9 13.25 9h-4.5c-.41 0-.75.34-.75.75zM6 9H3c-.55 0-1 .45-1 1v4.25c0 .41.34.75.75.75s.75-.34.75-.75v-.75h2v.75c0 .41.34.75.75.75s.75-.34.75-.75V10c0-.55-.45-1-1-1zm-.5 3h-2v-1.5h2V12zM21 9h-4.5c-.55 0-1 .45-1 1v4.25c0 .41.34.75.75.75s.75-.34.75-.75V10.5h1v2.75c0 .41.34.75.75.75s.75-.34.75-.75v-2.76h1v3.76c0 .41.34.75.75.75s.75-.34.75-.75V10c0-.55-.45-1-1-1z",
            }
        }
    }
}

pub fn attractions_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M17,10c0-0.68-0.1-1.34-0.29-1.97c0.49-0.55,0.6-1.36,0.21-2.03c-0.39-0.67-1.15-0.98-1.87-0.83 c-0.91-0.95-2.08-1.64-3.4-1.96C11.42,2.51,10.77,2,10,2S8.58,2.51,8.35,3.21c-1.32,0.32-2.49,1.01-3.4,1.96 C4.22,5.02,3.46,5.33,3.07,6C2.68,6.67,2.8,7.48,3.29,8.03C3.1,8.66,3,9.32,3,10s0.1,1.34,0.29,1.97C2.8,12.52,2.68,13.33,3.07,14 c0.39,0.67,1.15,0.98,1.87,0.83c0.33,0.35,0.69,0.66,1.09,0.93l-0.55,1.17C5.26,17.43,5.62,18,6.17,18h0 c0.29,0,0.56-0.17,0.68-0.43l0.51-1.09c0.32,0.13,0.65,0.23,0.99,0.32C8.58,17.49,9.23,18,10,18s1.42-0.51,1.65-1.21 c0.34-0.08,0.67-0.19,0.99-0.32l0.51,1.09c0.12,0.26,0.39,0.43,0.68,0.43h0c0.55,0,0.91-0.57,0.68-1.07l-0.55-1.17 c0.4-0.27,0.76-0.59,1.09-0.93c0.72,0.15,1.48-0.16,1.87-0.83c0.39-0.67,0.27-1.48-0.21-2.03C16.9,11.34,17,10.68,17,10z M11.46,15.3c-0.31-0.48-0.85-0.8-1.46-0.8s-1.15,0.32-1.46,0.8c-0.19-0.05-0.37-0.11-0.55-0.18l1.06-2.28 C9.35,12.93,9.67,13,10,13s0.65-0.07,0.95-0.17l1.06,2.28C11.83,15.18,11.65,15.24,11.46,15.3z M15.32,11.38 c-0.57,0.03-1.11,0.34-1.42,0.87c-0.31,0.53-0.3,1.16-0.04,1.67c-0.17,0.16-0.35,0.32-0.53,0.46l-1.1-2.37 C12.7,11.47,13,10.78,13,10c0-1.66-1.34-3-3-3s-3,1.34-3,3c0,0.78,0.3,1.47,0.78,2.01l-1.1,2.37c-0.19-0.14-0.37-0.29-0.53-0.46 c0.26-0.51,0.26-1.14-0.04-1.67c-0.31-0.53-0.85-0.84-1.42-0.87C4.57,10.94,4.5,10.48,4.5,10s0.07-0.94,0.18-1.38 C5.25,8.59,5.8,8.28,6.1,7.75c0.31-0.53,0.3-1.16,0.04-1.67C6.8,5.44,7.62,4.96,8.54,4.7C8.85,5.18,9.39,5.5,10,5.5 s1.15-0.32,1.46-0.8c0.91,0.25,1.73,0.73,2.39,1.38c-0.26,0.51-0.26,1.14,0.04,1.67c0.31,0.53,0.85,0.84,1.42,0.87 c0.11,0.44,0.18,0.9,0.18,1.38S15.43,10.94,15.32,11.38z",
                }
            }
        }
    }
}

pub fn attractions_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M10.44,18.75c0.37-0.46,0.94-0.75,1.57-0.75s1.19,0.29,1.56,0.75c0.39-0.09,0.76-0.21,1.12-0.36l-1.42-3.18 c-0.39,0.15-0.82,0.23-1.26,0.23c-0.46,0-0.9-0.09-1.3-0.25l-1.43,3.19C9.65,18.54,10.03,18.67,10.44,18.75z M5.16,10 C5,10.59,4.91,11.21,4.91,11.85c0,0.75,0.12,1.47,0.33,2.15c0.63,0.05,1.22,0.4,1.56,0.99c0.33,0.57,0.35,1.23,0.11,1.79 c0.27,0.27,0.56,0.53,0.87,0.76l1.52-3.39l0,0c-0.47-0.58-0.75-1.32-0.75-2.13c0-1.89,1.55-3.41,3.46-3.41s3.46,1.53,3.46,3.41 c0,0.82-0.29,1.57-0.78,2.16l1.5,3.35c0.32-0.24,0.62-0.5,0.9-0.79c-0.22-0.55-0.2-1.2,0.12-1.75c0.33-0.57,0.9-0.92,1.52-0.99 c0.22-0.68,0.34-1.41,0.34-2.16c0-0.64-0.09-1.27-0.25-1.86c-0.64-0.04-1.26-0.39-1.6-1c-0.36-0.62-0.35-1.36-0.03-1.95 c-0.91-0.98-2.1-1.71-3.44-2.05C13.4,5.6,12.74,6,12.01,6s-1.39-0.41-1.74-1.01C8.93,5.33,7.74,6.04,6.83,7.02 C7.16,7.62,7.18,8.37,6.81,9C6.45,9.62,5.82,9.97,5.16,10z M3.86,9.58C3.08,8.98,2.84,7.88,3.35,7s1.58-1.23,2.49-0.85 c1.11-1.17,2.56-2.03,4.18-2.42C10.15,2.75,10.99,2,12.01,2s1.85,0.75,1.98,1.73c1.63,0.39,3.07,1.24,4.18,2.42 c0.91-0.38,1.99-0.03,2.49,0.85c0.51,0.88,0.27,1.98-0.51,2.58c0.23,0.77,0.35,1.58,0.35,2.42s-0.12,1.65-0.35,2.42 c0.78,0.6,1.02,1.7,0.51,2.58s-1.58,1.23-2.49,0.85c-0.4,0.43-0.85,0.81-1.34,1.15l0.81,1.8c0.25,0.56-0.16,1.2-0.78,1.2h0 c-0.33,0-0.64-0.2-0.78-0.5l-0.75-1.67c-0.43,0.18-0.88,0.33-1.34,0.44C13.86,21.25,13.02,22,12.01,22s-1.85-0.75-1.98-1.73 C9.55,20.15,9.09,20,8.65,19.81L7.89,21.5C7.75,21.8,7.45,22,7.11,22H7.1c-0.62,0-1.03-0.64-0.77-1.2l0.82-1.83 c-0.47-0.33-0.91-0.71-1.3-1.12c-0.92,0.38-1.99,0.03-2.5-0.85s-0.27-1.98,0.51-2.58C3.62,13.65,3.51,12.84,3.51,12 S3.62,10.35,3.86,9.58z",
                    }
                }
            }
        }
    }
}

pub fn badge_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                path {
                    d: "M16.5,6H12V3.5C12,2.67,11.33,2,10.5,2h-1C8.67,2,8,2.67,8,3.5V6H3.5C2.67,6,2,6.67,2,7.5v9C2,17.33,2.67,18,3.5,18h13 c0.83,0,1.5-0.67,1.5-1.5v-9C18,6.67,17.33,6,16.5,6z M7.5,10c0.69,0,1.25,0.56,1.25,1.25S8.19,12.5,7.5,12.5s-1.25-0.56-1.25-1.25 S6.81,10,7.5,10z M10,15H5v-0.48c0-0.5,0.3-0.95,0.76-1.16C6.3,13.13,6.88,13,7.5,13c0.62,0,1.2,0.13,1.74,0.36 C9.7,13.56,10,14.01,10,14.52V15z M10.5,7.5h-1v-4h1V7.5z M14.25,14h-1.5C12.34,14,12,13.66,12,13.25v0c0-0.41,0.34-0.75,0.75-0.75 h1.5c0.41,0,0.75,0.34,0.75,0.75v0C15,13.66,14.66,14,14.25,14z M14.25,11.5h-1.5c-0.41,0-0.75-0.34-0.75-0.75v0 c0-0.41,0.34-0.75,0.75-0.75h1.5c0.41,0,0.75,0.34,0.75,0.75v0C15,11.16,14.66,11.5,14.25,11.5z",
                }
            }
        }
    }
}

pub fn badge_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                path {
                    d: "M20,7h-5V4c0-1.1-0.9-2-2-2h-2C9.9,2,9,2.9,9,4v3H4C2.9,7,2,7.9,2,9v11c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V9 C22,7.9,21.1,7,20,7z M9,12c0.83,0,1.5,0.67,1.5,1.5c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5C7.5,12.67,8.17,12,9,12z M12,18H6 v-0.43c0-0.6,0.36-1.15,0.92-1.39C7.56,15.9,8.26,15.75,9,15.75s1.44,0.15,2.08,0.43c0.55,0.24,0.92,0.78,0.92,1.39V18z M13,9h-2V4 h2V9z M17.25,16.5h-2.5c-0.41,0-0.75-0.34-0.75-0.75v0c0-0.41,0.34-0.75,0.75-0.75h2.5c0.41,0,0.75,0.34,0.75,0.75v0 C18,16.16,17.66,16.5,17.25,16.5z M17.25,13.5h-2.5c-0.41,0-0.75-0.34-0.75-0.75v0c0-0.41,0.34-0.75,0.75-0.75h2.5 c0.41,0,0.75,0.34,0.75,0.75v0C18,13.16,17.66,13.5,17.25,13.5z",
                }
            }
        }
    }
}

pub fn bakery_dining_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M6,6.5l-1.8,0.71c-0.52,0.2-0.77,0.79-0.56,1.31L5.88,14h1.46L6.68,6.92C6.65,6.59,6.31,6.38,6,6.5z",
                        }
                    }
                    g {
                        path {
                            d: "M15.81,7.22L14,6.5c-0.31-0.12-0.65,0.09-0.68,0.42L12.67,14h1.46l2.24-5.48C16.57,8.01,16.32,7.42,15.81,7.22z",
                        }
                    }
                    g {
                        path {
                            d: "M11.38,5H8.62c-0.59,0-1.05,0.51-1,1.09L8.35,14h3.3l0.73-7.91C12.43,5.51,11.97,5,11.38,5z",
                        }
                    }
                    g {
                        path {
                            d: "M18.1,13.04l-1.39-2.71l-1.6,3.91l1.22,0.62c0.88,0.45,1.92-0.2,1.92-1.2C18.25,13.44,18.2,13.23,18.1,13.04z",
                        }
                    }
                    g {
                        path {
                            d: "M1.9,13.04c-0.1,0.19-0.15,0.4-0.15,0.62c0,1,1.04,1.65,1.92,1.2l1.22-0.62l-1.6-3.91L1.9,13.04z",
                        }
                    }
                }
            }
        }
    }
}

pub fn bakery_dining_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M18.77,8.55L17.6,8.08c-0.62-0.25-1.31,0.17-1.37,0.84l-0.74,8.08H17l2.6-6.5C19.91,9.73,19.54,8.85,18.77,8.55z",
                        }
                    }
                    g {
                        path {
                            d: "M6.4,8.08L5.23,8.55C4.46,8.85,4.09,9.73,4.4,10.5l2.6,6.5h1.5L7.76,8.92C7.7,8.25,7.02,7.83,6.4,8.08z",
                        }
                    }
                    g {
                        path {
                            d: "M13.36,6h-2.71C9.76,6,9.07,6.76,9.15,7.64L10,16.99h4l0.85-9.36C14.93,6.76,14.24,6,13.36,6z",
                        }
                    }
                    g {
                        path {
                            d: "M3.18,13.72l-1,1.93c-0.19,0.36-0.23,0.78-0.12,1.19c0.29,1.01,1.43,1.41,2.38,0.94l1.05-0.52l-1.4-3.49 C3.93,13.37,3.38,13.34,3.18,13.72z",
                        }
                    }
                    g {
                        path {
                            d: "M21.82,15.65l-1-1.93c-0.2-0.38-0.75-0.35-0.91,0.04l-1.4,3.49l1.05,0.52c0.94,0.47,2.09,0.07,2.38-0.94 C22.05,16.43,22.01,16.01,21.82,15.65z",
                        }
                    }
                }
            }
        }
    }
}

pub fn beenhere_icons_24px(props: IconProps) -> Element {
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
                d: "M19 1H5c-1.1 0-1.99.9-1.99 2L3 15.93c0 .69.35 1.3.88 1.66l7.57 5.04c.34.22.77.22 1.11 0l7.56-5.04c.53-.36.88-.97.88-1.66V3c0-1.1-.9-2-2-2zm-.7 6.7l-7.59 7.59c-.39.39-1.02.39-1.41 0L5.71 11.7c-.39-.39-.39-1.02 0-1.41s1.02-.39 1.41 0L10 13.17l6.88-6.88c.39-.39 1.02-.39 1.41 0s.4 1.02.01 1.41z",
            }
        }
    }
}

pub fn bike_scooter_icons_20px(props: IconProps) -> Element {
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
                        d: "M9,13c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S10.1,13,9,13z M9,16c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S9.55,16,9,16z",
                    }
                    path {
                        d: "M9.24,12L7.82,5.78C7.72,5.32,7.31,5,6.84,5H4.51c-0.28,0-0.5,0.22-0.5,0.5v0c0,0.28,0.22,0.5,0.5,0.5h2.34l1.17,5.14 c-1.57,0.4-2.75,1.72-2.96,3.36H1v1h5.01v-0.51C6.01,13.34,7.35,12,9,12L9.24,12z",
                    }
                    path {
                        d: "M15.5,8h-0.68l-1.58-4.34C13.1,3.26,12.72,3,12.3,3h-1.8C10.22,3,10,3.22,10,3.5v0C10,3.78,10.22,4,10.5,4h1.8l1.46,4 h-4.4l0.23,1h3.45c-0.53,0.52-0.88,1.22-0.98,2h-2.01l0.23,1h1.79c0.25,1.78,1.77,3.1,3.66,2.99c1.68-0.09,3.1-1.43,3.27-3.1 C19.2,9.75,17.59,8,15.5,8z M15.48,14C14.13,13.99,13,12.85,13,11.5c0-0.94,0.5-1.73,1.24-2.16l0.86,2.36 c0.09,0.26,0.38,0.39,0.64,0.3l0,0c0.26-0.09,0.39-0.38,0.3-0.64l-0.85-2.33C15.3,9.02,15.4,9,15.5,9c1.4,0,2.5,1.1,2.5,2.5 C18,12.91,16.89,14.01,15.48,14z",
                    }
                }
            }
        }
    }
}

pub fn bike_scooter_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M10,14h0.74L8.82,5.56C8.61,4.65,7.8,4,6.87,4H4C3.45,4,3,4.45,3,5v0c0,0.55,0.45,1,1,1h2.87l1.42,6.25c0,0-0.01,0-0.01,0 C6.12,12.9,4.47,14.73,4.09,17H0v2h6v-1C6,15.79,7.79,14,10,14z",
                    }
                    path {
                        d: "M18.75,8l-0.56,0l-1.35-3.69C16.55,3.52,15.8,3,14.96,3H12c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2.96l1.1,3H10.4 l0.46,2H15c-0.43,0.58-0.75,1.25-0.9,2h-2.79l0.46,2h2.33c0.44,2.23,2.31,3.88,4.65,3.99c3.16,0.15,5.88-2.83,5.12-6.1 C23.34,9.57,21.13,8,18.75,8z M18.88,16c-1.54-0.06-2.84-1.37-2.88-2.92c-0.02-0.96,0.39-1.8,1.05-2.36l0.62,1.7 c0.19,0.52,0.76,0.79,1.28,0.6l0,0c0.52-0.19,0.79-0.76,0.6-1.28l-0.63-1.73c0,0,0,0,0.01-0.01C20.64,9.96,22,11.29,22,13 C22,14.72,20.62,16.06,18.88,16z",
                    }
                    path {
                        d: "M10,15c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S11.66,15,10,15z M10,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S10.55,19,10,19z",
                    }
                }
            }
        }
    }
}

pub fn breakfast_dining_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M18,6c0-1.66-1.34-3-3-3H5C3.34,3,2,4.34,2,6c0,1.3,0.84,2.4,2,2.82v6.68C4,16.33,4.67,17,5.5,17h9 c0.83,0,1.5-0.67,1.5-1.5l0-6.68C17.16,8.4,18,7.3,18,6z M12.71,11.71l-2,2c-0.39,0.39-1.02,0.39-1.41,0l-2-2 c-0.39-0.39-0.39-1.02,0-1.41l2-2c0.39-0.39,1.02-0.39,1.41,0l2,2C13.1,10.68,13.1,11.32,12.71,11.71z",
                }
            }
        }
    }
}

pub fn breakfast_dining_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                path {
                    d: "M18,3H6C3.79,3,2,4.79,2,7c0,1.48,0.81,2.75,2,3.45V19c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-8.55c1.19-0.69,2-1.97,2-3.45 C22,4.79,20.21,3,18,3z M15.71,13.7l-3,3c-0.39,0.39-1.02,0.39-1.42,0l-3-3c-0.39-0.39-0.39-1.02,0-1.41l3-3 c0.39-0.39,1.02-0.39,1.41,0l3,3C16.1,12.68,16.1,13.31,15.71,13.7z",
                }
            }
        }
    }
}

pub fn brunch_dining_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M2.5,18h10c0.28,0,0.5-0.22,0.5-0.5v-1H2v1C2,17.78,2.22,18,2.5,18z",
                        }
                    }
                    g {
                        path {
                            d: "M12.5,13.5H9V12H6v1.5H2.5C2.22,13.5,2,13.72,2,14v1h11v-1C13,13.72,12.78,13.5,12.5,13.5z",
                        }
                    }
                    g {
                        path {
                            d: "M18,9.76V3c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v6.76c0,1.27,0.67,2.44,1.75,3.09V17c0,0.55,0.45,1,1,1h1.5 c0.41,0,0.75-0.34,0.75-0.75l0,0c0-0.41-0.34-0.75-0.75-0.75h-1v-3.65C17.33,12.2,18,11.04,18,9.76z M16.5,3.5V7h-2V3.5H16.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn brunch_dining_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M18,8h2V4h-2V8z M15,22H3c-0.55,0-1-0.45-1-1v-1h14v1C16,21.55,15.55,22,15,22z M18,15.89l-0.4-0.42 c-1.03-1.08-1.6-2.51-1.6-4V3c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v8.51c0,1.46-0.54,2.87-1.53,3.94L20,15.97V20h1 c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h-2c-0.55,0-1-0.45-1-1V15.89z M7,16v-1c0-0.55,0.45-1,1-1h2c0.55,0,1,0.45,1,1v1h4 c0.55,0,1,0.45,1,1v1H2v-1c0-0.55,0.45-1,1-1H7z",
                    }
                }
            }
        }
    }
}

pub fn bus_alert_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M4,11V8h7.29c-0.77-2.6,0.21-4.61,0.37-4.97C2.97,2.67,2,5.02,2,7v9.5c0,0.95,0.38,1.81,1,2.44v1.56 C3,21.33,3.67,22,4.5,22h0C5.33,22,6,21.33,6,20.5V20h8v0.5c0,0.82,0.67,1.5,1.5,1.5h0c0.82,0,1.5-0.67,1.5-1.5v-1.56 c0.62-0.63,1-1.49,1-2.44V13c-1.91,0-3.63-0.76-4.89-2H4z M6.5,17C5.67,17,5,16.33,5,15.5S5.67,14,6.5,14S8,14.67,8,15.5 S7.33,17,6.5,17z M15,15.5c0,0.83-0.67,1.5-1.5,1.5S12,16.33,12,15.5s0.67-1.5,1.5-1.5S15,14.67,15,15.5z",
                        }
                    }
                    g {
                        path {
                            d: "M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,6.5C18.5,6.78,18.28,7,18,7s-0.5-0.22-0.5-0.5v-3 C17.5,3.22,17.72,3,18,3s0.5,0.22,0.5,0.5V6.5z M18.5,8.5C18.5,8.78,18.28,9,18,9s-0.5-0.22-0.5-0.5S17.72,8,18,8 S18.5,8.22,18.5,8.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn car_crash_icons_20px(props: IconProps) -> Element {
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
                        d: "M19,5c0,2.21-1.79,4-4,4s-4-1.79-4-4s1.79-4,4-4S19,2.79,19,5z M15.5,6.5C15.5,6.22,15.28,6,15,6s-0.5,0.22-0.5,0.5 S14.72,7,15,7S15.5,6.78,15.5,6.5z M16,16c0.55,0,1-0.45,1-1v-4.88c-0.62,0.24-1.29,0.38-2,0.38c-0.32,0-0.64-0.03-0.94-0.08 c0.27,0.18,0.44,0.48,0.44,0.83c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1c0-0.51,0.39-0.93,0.88-0.99C12.14,9.87,11.09,9.06,10.39,8 H5.02l1-2.5h3.51C9.47,4.99,9.51,4.39,9.59,4H6.02C5.4,4,4.85,4.37,4.62,4.94L3.11,8.73C3.04,8.91,3,9.1,3,9.29L3,15 c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-0.5h10V15C15,15.55,15.45,16,16,16z M6.5,12.25c-0.55,0-1-0.45-1-1s0.45-1,1-1 c0.55,0,1,0.45,1,1S7.05,12.25,6.5,12.25z M15.5,3.5C15.5,3.22,15.28,3,15,3s-0.5,0.22-0.5,0.5v1C14.5,4.78,14.72,5,15,5 s0.5-0.22,0.5-0.5V3.5z",
                    }
                }
            }
        }
    }
}

pub fn car_crash_icons_24px(props: IconProps) -> Element {
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
                        d: "M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18,7c-0.28,0-0.5-0.22-0.5-0.5v-3C17.5,3.22,17.72,3,18,3 s0.5,0.22,0.5,0.5v3C18.5,6.78,18.28,7,18,7z M18.5,8.5C18.5,8.78,18.28,9,18,9s-0.5-0.22-0.5-0.5S17.72,8,18,8 S18.5,8.22,18.5,8.5z M19.5,20c0.82,0,1.5-0.67,1.5-1.5v-6.18c-1.05,0.51-2.16,0.69-3.09,0.68c0.06,0.16,0.09,0.33,0.09,0.5 c0,0.83-0.67,1.5-1.5,1.5S15,14.33,15,13.5c0-0.39,0.15-0.74,0.39-1.01c-1.63-0.66-2.96-1.91-3.71-3.49H5.81l1.04-3H11 c0-0.69,0.1-1.37,0.29-2H6.5C5.84,4,5.29,4.42,5.08,5.01l-1.97,5.67C3.04,10.89,3,11.11,3,11.34v7.16C3,19.33,3.67,20,4.5,20 S6,19.33,6,18.5V18h12v0.5C18,19.33,18.68,20,19.5,20z M7.5,15C6.67,15,6,14.33,6,13.5S6.67,12,7.5,12S9,12.67,9,13.5 S8.33,15,7.5,15z",
                    }
                }
            }
        }
    }
}

pub fn car_rental_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M6.26,8.65l-1.2,3.18C5.02,11.94,5,12.06,5,12.18v4.07C5,16.66,5.34,17,5.75,17h0c0.41,0,0.75-0.34,0.75-0.75V16h7v0.25 c0,0.41,0.34,0.75,0.75,0.75h0c0.41,0,0.75-0.34,0.75-0.75v-4.07c0-0.12-0.02-0.23-0.06-0.34l-1.16-3.18 C13.63,8.26,13.26,8,12.84,8H7.2C6.78,8,6.41,8.26,6.26,8.65z M7.75,14.25C7.34,14.25,7,13.91,7,13.5s0.34-0.75,0.75-0.75 c0.41,0,0.75,0.34,0.75,0.75S8.16,14.25,7.75,14.25z M12.25,14.25c-0.41,0-0.75-0.34-0.75-0.75s0.34-0.75,0.75-0.75 S13,13.09,13,13.5S12.66,14.25,12.25,14.25z M6.98,11l0.56-1.5h4.95l0.55,1.5H6.98z",
                    }
                    path {
                        d: "M9.37,2.75C9,1.6,7.83,0.79,6.51,1.05C5.55,1.23,4.77,2,4.56,2.95C4.21,4.56,5.45,6,7,6c1.11,0,2.05-0.74,2.37-1.75h3.13 v1C12.5,5.66,12.84,6,13.25,6h0C13.66,6,14,5.66,14,5.25v-1h0.25C14.66,4.25,15,3.91,15,3.5v0c0-0.41-0.34-0.75-0.75-0.75H9.37z M7,4.5c-0.54,0-1-0.46-1-1s0.46-1,1-1s1,0.46,1,1S7.54,4.5,7,4.5z",
                    }
                }
            }
        }
    }
}

pub fn car_rental_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                path {
                    d: "M8,7c1.3,0,2.41-0.84,2.83-2H16v1c0,0.55,0.45,1,1,1s1-0.45,1-1V5h0c0.55,0,1-0.45,1-1s-0.45-1-1-1h-7.17 C10.35,1.65,8.95,0.76,7.4,1.06C6.23,1.29,5.28,2.25,5.05,3.42C4.7,5.32,6.15,7,8,7z M8,3c0.55,0,1,0.45,1,1S8.55,5,8,5S7,4.55,7,4 S7.45,3,8,3z M16.39,9H7.61C7.18,9,6.8,9.28,6.66,9.68L5,14.69V21c0,0.55,0.45,1,1,1s1-0.45,1-1v-1h10v1c0,0.55,0.45,1,1,1 s1-0.45,1-1v-6.31l-1.66-5.01C17.2,9.28,16.82,9,16.39,9z M9,17.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S9.55,17.5,9,17.5z M15,17.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S15.55,17.5,15,17.5z M7.67,13l0.66-2h7.34l0.66,2H7.67z",
                }
            }
        }
    }
}

pub fn car_repair_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M5.75,12c0.41,0,0.75-0.34,0.75-0.75V11h7v0.25c0,0.41,0.34,0.75,0.75,0.75c0.41,0,0.75-0.34,0.75-0.75V7.18 c0-0.12-0.02-0.23-0.06-0.34l-1.16-3.18C13.63,3.26,13.26,3,12.84,3H7.2C6.78,3,6.41,3.26,6.26,3.65l-1.2,3.18 C5.02,6.94,5,7.06,5,7.18v4.07C5,11.66,5.34,12,5.75,12z M7.75,9.25C7.34,9.25,7,8.91,7,8.5s0.34-0.75,0.75-0.75S8.5,8.09,8.5,8.5 S8.16,9.25,7.75,9.25z M12.25,9.25c-0.41,0-0.75-0.34-0.75-0.75s0.34-0.75,0.75-0.75S13,8.09,13,8.5S12.66,9.25,12.25,9.25z M7.54,4.5h4.95L13.04,6H6.98L7.54,4.5z",
                    }
                    path {
                        d: "M15.25,13H4.75C4.34,13,4,13.34,4,13.75s0.34,0.75,0.75,0.75h4.5v1.75C9.25,16.66,9.59,17,10,17s0.75-0.34,0.75-0.75V14.5 h4.5c0.41,0,0.75-0.34,0.75-0.75S15.66,13,15.25,13z",
                    }
                }
            }
        }
    }
}

pub fn car_repair_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M7,15v-1h10v1c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V8.69c0,0-1.34-4.03-1.56-4.69c-0.05-0.16-0.12-0.29-0.19-0.4 c-0.02-0.02-0.03-0.04-0.05-0.07C16.82,3.01,16.28,3,16.28,3H7.72c0,0-0.54,0.01-0.92,0.54C6.78,3.56,6.77,3.58,6.75,3.6 C6.68,3.71,6.61,3.84,6.56,4C6.34,4.66,5,8.69,5,8.69V15c0,0.55,0.45,1,1,1h0C6.55,16,7,15.55,7,15z M9,11.5c-0.55,0-1-0.45-1-1 s0.45-1,1-1s1,0.45,1,1S9.55,11.5,9,11.5z M15,11.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S15.55,11.5,15,11.5z M8.33,5h7.34 l0.23,0.69L16.33,7H7.67L8.33,5z",
                    }
                    path {
                        d: "M4,18.01L4,18.01C4,18.55,4.45,19,4.99,19H11v2.01c0,0.55,0.45,0.99,0.99,0.99h0.01c0.55,0,0.99-0.45,0.99-0.99V19h6.01 c0.55,0,0.99-0.45,0.99-0.99v0c0-0.55-0.45-0.99-0.99-0.99H4.99C4.45,17.01,4,17.46,4,18.01z",
                    }
                }
            }
        }
    }
}

pub fn castle_icons_20px(props: IconProps) -> Element {
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
                        d: "M18.25,7c-0.41,0-0.75,0.34-0.75,0.75V9H16V2.75C16,2.34,15.66,2,15.25,2S14.5,2.34,14.5,2.75V4h-2V2.75 C12.5,2.34,12.16,2,11.75,2S11,2.34,11,2.75V4H9V2.75C9,2.34,8.66,2,8.25,2S7.5,2.34,7.5,2.75V4h-2V2.75C5.5,2.34,5.16,2,4.75,2 S4,2.34,4,2.75V9H2.5V7.75C2.5,7.34,2.16,7,1.75,7S1,7.34,1,7.75v7.75C1,16.33,1.67,17,2.5,17H8v-2c0-1.1,0.9-2,2-2s2,0.9,2,2v2 h5.5c0.83,0,1.5-0.67,1.5-1.5V7.75C19,7.34,18.66,7,18.25,7z M9,10H7.5V7H9V10z M12.5,10H11V7h1.5V10z",
                    }
                }
            }
        }
    }
}

pub fn castle_icons_24px(props: IconProps) -> Element {
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
                        d: "M22,9c-0.55,0-1,0.45-1,1v1h-2V4c0-0.55-0.45-1-1-1s-1,0.45-1,1v1h-2V4c0-0.55-0.45-1-1-1s-1,0.45-1,1v1h-2V4 c0-0.55-0.45-1-1-1S9,3.45,9,4v1H7V4c0-0.55-0.45-1-1-1S5,3.45,5,4v7H3v-1c0-0.55-0.45-1-1-1s-1,0.45-1,1v9c0,1.1,0.9,2,2,2h7v-3 c0-1.1,0.9-2,2-2s2,0.9,2,2v3h7c1.1,0,2-0.9,2-2v-9C23,9.45,22.55,9,22,9z M11,12H9V9h2V12z M15,12h-2V9h2V12z",
                    }
                }
            }
        }
    }
}

pub fn category_icons_24px(props: IconProps) -> Element {
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
                d: "M11.15 3.4L7.43 9.48c-.41.66.07 1.52.85 1.52h7.43c.78 0 1.26-.86.85-1.52L12.85 3.4c-.39-.64-1.31-.64-1.7 0z",
            }
            circle {
                r: "4.5",
                cy: "17.5",
                cx: "17.5",
            }
            path {
                d: "M4 21.5h6c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1z",
            }
        }
    }
}

pub fn celebration_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M3.98,17.3L12,14.46c0.69-0.25,0.89-1.13,0.37-1.65L7.19,7.63C6.67,7.11,5.78,7.31,5.54,8L2.7,16.02 C2.42,16.82,3.18,17.58,3.98,17.3z",
                    }
                    path {
                        d: "M11.19,2.44L11.19,2.44c-0.2,0.2-0.2,0.51,0,0.71l1.06,1.06c0.39,0.39,0.39,1.02,0,1.41l-2.12,2.12 c-0.2,0.2-0.2,0.51,0,0.71l0,0c0.2,0.2,0.51,0.2,0.71,0l2.12-2.12c0.78-0.78,0.78-2.05,0-2.83L11.9,2.44 C11.7,2.25,11.39,2.25,11.19,2.44z",
                    }
                    path {
                        d: "M8.72,3.5L8.72,3.5c-0.2,0.2-0.2,0.51,0,0.71l0.35,0.35c0.39,0.39,0.39,1.02,0,1.41L8.72,6.33c-0.2,0.2-0.2,0.51,0,0.71 l0,0c0.2,0.2,0.51,0.2,0.71,0l0.35-0.35c0.78-0.78,0.78-2.05,0-2.83L9.42,3.5C9.23,3.31,8.91,3.31,8.72,3.5z",
                    }
                    path {
                        d: "M17.2,9.16c-0.78-0.78-2.05-0.78-2.83,0l-1.41,1.41c-0.2,0.2-0.2,0.51,0,0.71l0,0c0.2,0.2,0.51,0.2,0.71,0l1.41-1.41 c0.39-0.39,1.02-0.39,1.41,0l1.06,1.06c0.2,0.2,0.51,0.2,0.71,0v0c0.2-0.2,0.2-0.51,0-0.71L17.2,9.16z",
                    }
                    path {
                        d: "M18.97,5.27l-0.35-0.35c-0.78-0.78-2.05-0.78-2.83,0l-4.24,4.24c-0.2,0.2-0.2,0.51,0,0.71v0c0.2,0.2,0.51,0.2,0.71,0 l4.24-4.24c0.39-0.39,1.02-0.39,1.41,0l0.35,0.35c0.2,0.2,0.51,0.2,0.71,0v0C19.17,5.78,19.17,5.47,18.97,5.27z",
                    }
                }
            }
        }
    }
}

pub fn celebration_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M3.99,21.29l9.04-3.23c1.38-0.49,1.78-2.26,0.74-3.3l-4.53-4.53c-1.04-1.04-2.8-0.64-3.3,0.74l-3.23,9.04 C2.43,20.81,3.19,21.57,3.99,21.29z",
                        }
                        path {
                            d: "M15.06,12l5.06-5.06c0.49-0.49,1.28-0.49,1.77,0L21.95,7c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06 l-0.06-0.06c-1.07-1.07-2.82-1.07-3.89,0L14,10.94c-0.29,0.29-0.29,0.77,0,1.06l0,0C14.29,12.29,14.77,12.29,15.06,12z",
                        }
                        path {
                            d: "M10.06,6.88L10,6.94C9.71,7.23,9.71,7.71,10,8l0,0c0.29,0.29,0.77,0.29,1.06,0l0.06-0.06c1.07-1.07,1.07-2.82,0-3.89 L11.07,4C10.77,3.7,10.29,3.7,10,4l0,0c-0.29,0.29-0.29,0.77,0,1.06l0.06,0.06C10.54,5.6,10.54,6.4,10.06,6.88z",
                        }
                        path {
                            d: "M17.06,11.88L16,12.94c-0.29,0.29-0.29,0.77,0,1.06l0,0c0.29,0.29,0.77,0.29,1.06,0l1.06-1.06 c0.49-0.49,1.28-0.49,1.77,0l1.08,1.08c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06l-1.08-1.08 C19.87,10.81,18.13,10.81,17.06,11.88z",
                        }
                        path {
                            d: "M15.06,5.88L12,8.94c-0.29,0.29-0.29,0.77,0,1.06l0,0c0.29,0.29,0.77,0.29,1.06,0l3.06-3.06c1.07-1.07,1.07-2.82,0-3.89 l-1.06-1.06c-0.29-0.29-0.77-0.29-1.06,0l0,0c-0.29,0.29-0.29,0.77,0,1.06l1.06,1.06C15.54,4.6,15.54,5.4,15.06,5.88z",
                        }
                    }
                }
            }
        }
    }
}

pub fn church_icons_20px(props: IconProps) -> Element {
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
                        d: "M15,11V8.85c0-0.53-0.28-1.02-0.73-1.29l-3.52-2.11v-1.2h1c0.41,0,0.75-0.34,0.75-0.75v0c0-0.41-0.34-0.75-0.75-0.75h-1 v-1C10.75,1.34,10.41,1,10,1h0C9.59,1,9.25,1.34,9.25,1.75v1h-1C7.84,2.75,7.5,3.09,7.5,3.5v0c0,0.41,0.34,0.75,0.75,0.75h1v1.2 L5.73,7.56C5.28,7.83,5,8.32,5,8.85V11l-1.97,0.66C2.41,11.86,2,12.44,2,13.08v3.42C2,17.33,2.67,18,3.5,18H8l0-1.89 c0-1,0.68-1.92,1.66-2.08C10.92,13.82,12,14.79,12,16v2h4.5c0.83,0,1.5-0.67,1.5-1.5v-3.42c0-0.65-0.41-1.22-1.03-1.42L15,11z M10,11.25c-0.69,0-1.25-0.56-1.25-1.25S9.31,8.75,10,8.75s1.25,0.56,1.25,1.25S10.69,11.25,10,11.25z",
                    }
                }
            }
        }
    }
}

pub fn church_icons_24px(props: IconProps) -> Element {
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
                        d: "M18,12.22v-1.99c0-0.76-0.43-1.45-1.11-1.79L13,6.5V5h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1V2c0-0.55-0.45-1-1-1h0 c-0.55,0-1,0.45-1,1v1h-1C9.45,3,9,3.45,9,4v0c0,0.55,0.45,1,1,1h1v1.5L7.11,8.45C6.43,8.79,6,9.48,6,10.24v1.99l-2.81,1.25 C2.47,13.79,2,14.51,2,15.3V20c0,1.1,0.9,2,2,2h6l0-2.89c0-1,0.68-1.92,1.66-2.08C12.92,16.82,14,17.79,14,19v3h6c1.1,0,2-0.9,2-2 v-4.7c0-0.79-0.47-1.51-1.19-1.83L18,12.22z M12,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 S12.83,13.5,12,13.5z",
                    }
                }
            }
        }
    }
}

pub fn cleaning_services_icons_20px(props: IconProps) -> Element {
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
                    d: "M13,9h-1V5c0-1.1-0.9-2-2-2h0C8.9,3,8,3.9,8,5v4H7c-1.66,0-3,1.34-3,3v4c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1v-4 C16,10.34,14.66,9,13,9z M15,16h-2v-1.5c0-0.28-0.22-0.5-0.5-0.5S12,14.22,12,14.5V16h-1.5v-1.5c0-0.28-0.22-0.5-0.5-0.5 s-0.5,0.22-0.5,0.5V16H8v-1.5C8,14.22,7.78,14,7.5,14S7,14.22,7,14.5V16H5v-4c0-1.1,0.9-2,2-2h6c1.1,0,2,0.9,2,2V16z",
                }
            }
        }
    }
}

pub fn cleaning_services_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M16,11h-1V4c0-1.66-1.34-3-3-3h0c-1.66,0-3,1.34-3,3v7H8c-2.76,0-5,2.24-5,5v5c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v-5 C21,13.24,18.76,11,16,11z M19,21h-2v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v3h-2v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v3H9v-3 c0-0.55-0.45-1-1-1s-1,0.45-1,1v3H5v-5c0-1.65,1.35-3,3-3h8c1.65,0,3,1.35,3,3V21z",
                }
            }
        }
    }
}

pub fn compass_calibration_icons_24px(props: IconProps) -> Element {
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
            circle {
                r: "4",
                cx: "12",
                cy: "17",
            }
            path {
                d: "M12 3C8.49 3 5.28 4.29 2.8 6.41c-.44.38-.48 1.06-.06 1.48l3.6 3.6c.36.36.92.39 1.32.08 1.2-.94 2.71-1.5 4.34-1.5 1.64 0 3.14.56 4.34 1.49.4.31.96.28 1.31-.08l3.6-3.6c.42-.42.38-1.1-.07-1.48C18.72 4.28 15.51 3 12 3z",
            }
        }
    }
}

pub fn connecting_airports_icons_20px(props: IconProps) -> Element {
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
                d: "M7.24,2.5c0.16,0,0.32,0.09,0.4,0.23L9.6,6l2.12,0c0.41,0,0.77,0.32,0.78,0.73c0.01,0.42-0.33,0.77-0.75,0.77H9.6 l-1.96,3.27C7.55,10.91,7.4,11,7.24,11c-0.32,0-0.54-0.31-0.45-0.61L7.7,7.5H5.45c0,0-0.43,0.58-0.64,0.85 C4.74,8.45,4.62,8.5,4.51,8.5c-0.26,0-0.44-0.24-0.37-0.49C4.29,7.49,4.5,6.75,4.5,6.75L4.14,5.49C4.07,5.24,4.25,5,4.51,5 c0.12,0,0.23,0.06,0.31,0.15L5.45,6H7.7L6.79,3.11C6.7,2.81,6.92,2.5,7.24,2.5z M12.76,17.5c0.32,0,0.54-0.31,0.45-0.61L12.3,14 h2.25l0.63,0.85c0.07,0.1,0.19,0.15,0.31,0.15c0.25,0,0.44-0.24,0.37-0.49l-0.36-1.26l0.36-1.26c0.07-0.24-0.11-0.49-0.37-0.49 c-0.12,0-0.23,0.06-0.31,0.15l-0.63,0.85H12.3l0.91-2.89C13.3,9.31,13.08,9,12.76,9c-0.16,0-0.32,0.09-0.4,0.23L10.4,12.5H8.28 c-0.41,0-0.77,0.32-0.78,0.73C7.49,13.65,7.83,14,8.25,14h2.15l1.96,3.27C12.45,17.41,12.6,17.5,12.76,17.5z",
            }
        }
    }
}

pub fn connecting_airports_icons_24px(props: IconProps) -> Element {
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
                d: "M15.93,10.6c0.39,0,0.66,0.37,0.55,0.74L15.4,15h2.85l0.59-0.78c0.1-0.14,0.26-0.22,0.43-0.22c0.36,0,0.62,0.35,0.52,0.7 L19.4,16l0.39,1.3c0.1,0.35-0.16,0.7-0.52,0.7c-0.17,0-0.33-0.08-0.43-0.22L18.25,17H15.4l1.08,3.66c0.11,0.37-0.17,0.74-0.55,0.74 c-0.2,0-0.39-0.11-0.5-0.28L13,17h-2.97c-0.53,0-1-0.4-1.03-0.93C8.96,15.48,9.43,15,10,15h3l2.43-4.12 C15.54,10.71,15.73,10.6,15.93,10.6z M8.07,2.6c-0.39,0-0.66,0.37-0.55,0.74L8.6,7H5.75L5.16,6.22C5.06,6.08,4.9,6,4.73,6 C4.37,6,4.11,6.35,4.21,6.7L4.6,8L4.21,9.3C4.11,9.65,4.37,10,4.73,10c0.17,0,0.33-0.08,0.43-0.22L5.75,9H8.6l-1.08,3.66 c-0.11,0.37,0.17,0.74,0.55,0.74c0.2,0,0.39-0.11,0.5-0.28L11,9h2.97c0.53,0,1-0.4,1.03-0.93C15.04,7.48,14.57,7,14,7h-3L8.57,2.88 C8.46,2.71,8.27,2.6,8.07,2.6z",
            }
        }
    }
}

pub fn crisis_alert_icons_20px(props: IconProps) -> Element {
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
                    d: "M12,2c0,1.2-1.2,5-1.2,5H9.2C9.2,7,8,3.2,8,2c0-1.1,0.9-2,2-2S12,0.9,12,2z M10,8.25c-0.97,0-1.75,0.78-1.75,1.75 s0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75S10.97,8.25,10,8.25z M13.12,4.3c2.01,1.1,3.38,3.24,3.38,5.7c0,3.59-2.91,6.5-6.5,6.5 S3.5,13.59,3.5,10c0-2.46,1.37-4.6,3.38-5.7C6.75,3.76,6.64,3.23,6.57,2.77C3.87,4.06,2,6.81,2,10c0,4.42,3.58,8,8,8s8-3.58,8-8 c0-3.19-1.87-5.94-4.57-7.23C13.36,3.23,13.25,3.76,13.12,4.3z M13.5,10c0,1.93-1.57,3.5-3.5,3.5S6.5,11.93,6.5,10 c0-1.07,0.48-2.02,1.23-2.67c-0.11-0.35-0.28-0.9-0.46-1.53C5.91,6.7,5,8.24,5,10c0,2.76,2.24,5,5,5s5-2.24,5-5 c0-1.76-0.91-3.3-2.28-4.19c-0.18,0.63-0.35,1.18-0.46,1.53C13.02,7.98,13.5,8.93,13.5,10z",
                }
            }
        }
    }
}

pub fn crisis_alert_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M14.5,2.5c0,1.5-1.5,6-1.5,6h-2c0,0-1.5-4.5-1.5-6C9.5,1.12,10.62,0,12,0S14.5,1.12,14.5,2.5z M12,10c-1.1,0-2,0.9-2,2 s0.9,2,2,2s2-0.9,2-2S13.1,10,12,10z M16.08,5.11c0.18-0.75,0.33-1.47,0.39-2.06C19.75,4.69,22,8.08,22,12c0,5.52-4.48,10-10,10 S2,17.52,2,12c0-3.92,2.25-7.31,5.53-8.95C7.6,3.64,7.74,4.37,7.92,5.11C5.58,6.51,4,9.07,4,12c0,4.42,3.58,8,8,8s8-3.58,8-8 C20,9.07,18.42,6.51,16.08,5.11z M18,12c0,3.31-2.69,6-6,6s-6-2.69-6-6c0-2,0.98-3.77,2.48-4.86c0.23,0.81,0.65,2.07,0.65,2.07 C8.43,9.93,8,10.92,8,12c0,2.21,1.79,4,4,4s4-1.79,4-4c0-1.08-0.43-2.07-1.13-2.79c0,0,0.41-1.22,0.65-2.07C17.02,8.23,18,10,18,12 z",
                }
            }
        }
    }
}

pub fn delivery_dining_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M7.25,5.5h-2.5C4.34,5.5,4,5.84,4,6.25v0C4,6.66,4.34,7,4.75,7h2.5C7.66,7,8,6.66,8,6.25v0C8,5.84,7.66,5.5,7.25,5.5z",
                    }
                    path {
                        d: "M15.5,7.91V6.5C15.5,5.67,14.83,5,14,5h-1.75c-0.41,0-0.75,0.34-0.75,0.75v0c0,0.41,0.34,0.75,0.75,0.75H14v1.41l-3,3.59 H8V9c0-0.55-0.45-1-1-1H5c-1.66,0-3,1.34-3,3v1c0,0.55,0.45,1,1,1h0.5c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5H11 c0.44,0,0.87-0.2,1.15-0.54l3-3.59C15.38,8.6,15.5,8.26,15.5,7.91z M6,14c-0.55,0-1-0.45-1-1h2C7,13.55,6.55,14,6,14z",
                    }
                    path {
                        d: "M15.5,10.5c-1.38,0-2.5,1.12-2.5,2.5s1.12,2.5,2.5,2.5S18,14.38,18,13S16.88,10.5,15.5,10.5z M15.5,14c-0.55,0-1-0.45-1-1 s0.45-1,1-1s1,0.45,1,1S16.05,14,15.5,14z",
                    }
                }
            }
        }
    }
}

pub fn delivery_dining_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M19,7c0-1.1-0.9-2-2-2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2.65L13.52,14H10v-4c0-0.55-0.45-1-1-1H6 c-2.21,0-4,1.79-4,4v2c0,0.55,0.45,1,1,1h1c0,1.66,1.34,3,3,3s3-1.34,3-3h3.52c0.61,0,1.18-0.28,1.56-0.75l3.48-4.35 C18.85,10.54,19,10.1,19,9.65V7z M7,17c-0.55,0-1-0.45-1-1h2C8,16.55,7.55,17,7,17z",
                        }
                        path {
                            d: "M6,6h3c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H6C5.45,8,5,7.55,5,7v0C5,6.45,5.45,6,6,6z",
                        }
                        path {
                            d: "M19,13c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,13,19,13z M19,17c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 C20,16.55,19.55,17,19,17z",
                        }
                    }
                }
            }
        }
    }
}

pub fn departure_board_icons_24px(props: IconProps) -> Element {
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
                d: "M17.34 1.13c-2.94-.55-5.63.75-7.12 2.92.01-.01.01-.02.02-.03C9.84 4 9.42 4 9 4c-4.42 0-8 .5-8 4v10c0 .88.39 1.67 1 2.22v1.28c0 .83.67 1.5 1.5 1.5S5 22.33 5 21.5V21h8v.5c0 .82.67 1.5 1.5 1.5.82 0 1.5-.67 1.5-1.5v-1.28c.61-.55 1-1.34 1-2.22v-3.08c3.72-.54 6.5-3.98 5.92-7.97-.42-2.9-2.7-5.29-5.58-5.82zM4.5 19c-.83 0-1.5-.67-1.5-1.5S3.67 16 4.5 16s1.5.67 1.5 1.5S5.33 19 4.5 19zM3 13V8h6c0 1.96.81 3.73 2.11 5H3zm10.5 6c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm2.5-6c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm-.25-9c-.41 0-.75.34-.75.75v3.68c0 .35.19.68.49.86l2.52 1.51c.34.2.78.09.98-.24.21-.34.1-.79-.25-.99L16.5 8.25v-3.5c0-.41-.34-.75-.75-.75z",
            }
        }
    }
}

pub fn design_services_icons_20px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M15.71,13.21l-3.46-3.46l1.33-1.33l-2-2l-1.33,1.33L6.79,4.29C6.4,3.9,5.76,3.9,5.37,4.29L4.29,5.37 C3.9,5.76,3.9,6.4,4.29,6.79l3.46,3.46L4,14v1.5C4,15.78,4.22,16,4.5,16H6l3.75-3.75l3.46,3.46c0.39,0.39,1.02,0.39,1.41,0 l1.08-1.08C16.1,14.24,16.1,13.6,15.71,13.21z M8.46,9.54L5,6.08L6.08,5c0,0,0,0,0,0l0.69,0.69L6.56,5.9 c-0.18,0.18-0.18,0.48,0,0.67l0,0c0.18,0.18,0.48,0.18,0.67,0l0.21-0.21l1.06,1.06L8.28,7.62c-0.18,0.18-0.18,0.48,0,0.67l0,0 c0.18,0.18,0.48,0.18,0.67,0l0.21-0.21l0.38,0.38L8.46,9.54z M13.92,15l-3.46-3.46l1.08-1.08l0.4,0.4l-0.21,0.21 c-0.18,0.18-0.18,0.48,0,0.67l0,0c0.18,0.18,0.48,0.18,0.67,0l0.21-0.21l1.06,1.06l-0.21,0.21c-0.18,0.18-0.18,0.48,0,0.67l0,0 c0.18,0.18,0.48,0.18,0.67,0l0.21-0.21L15,13.92L13.92,15z",
                    }
                    path {
                        d: "M15.62,6.38c0.2-0.2,0.2-0.51,0-0.71l-1.29-1.29c-0.2-0.2-0.51-0.2-0.71,0l-1.34,1.34l2,2L15.62,6.38z",
                    }
                }
            }
        }
    }
}

pub fn design_services_icons_24px(props: IconProps) -> Element {
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
                        d: "M16.24,11.51l1.57-1.57l-3.75-3.75l-1.57,1.57L8.35,3.63c-0.78-0.78-2.05-0.78-2.83,0l-1.9,1.9 c-0.78,0.78-0.78,2.05,0,2.83l4.13,4.13L3.15,17.1C3.05,17.2,3,17.32,3,17.46v3.04C3,20.78,3.22,21,3.5,21h3.04 c0.13,0,0.26-0.05,0.35-0.15l4.62-4.62l4.13,4.13c1.32,1.32,2.76,0.07,2.83,0l1.9-1.9c0.78-0.78,0.78-2.05,0-2.83L16.24,11.51z M9.18,11.07L5.04,6.94l1.89-1.9c0,0,0,0,0,0l1.27,1.27L7.73,6.8c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0 l0.48-0.48l1.45,1.45L9.18,11.07z M17.06,18.96l-4.13-4.13l1.9-1.9l1.45,1.45l-0.48,0.48c-0.39,0.39-0.39,1.02,0,1.41 c0.39,0.39,1.02,0.39,1.41,0l0.48-0.48l1.27,1.27L17.06,18.96z",
                    }
                    path {
                        d: "M20.71,7.04c0.39-0.39,0.39-1.02,0-1.41l-2.34-2.34c-0.47-0.47-1.12-0.29-1.41,0l-1.83,1.83l3.75,3.75L20.71,7.04z",
                    }
                }
            }
        }
    }
}

pub fn diamond_icons_20px(props: IconProps) -> Element {
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
                    g {
                        polygon {
                            points: "9.5,16.44 9.5,8.5 2.44,8.5",
                        }
                    }
                    g {
                        polygon {
                            points: "9.9,3.31 7.81,7.5 12.16,7.5",
                        }
                    }
                    g {
                        polygon {
                            points: "10.5,8.5 10.5,16.44 17.56,8.5",
                        }
                    }
                    g {
                        path {
                            d: "M6.69,7.5L8.94,3H5.02C4.4,3,3.85,3.37,3.62,3.94L2.2,7.5H6.69z",
                        }
                    }
                    g {
                        path {
                            d: "M13.3,7.5h4.5l-1.42-3.56C16.15,3.37,15.6,3,14.98,3h-4.12L13.3,7.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn diamond_icons_24px(props: IconProps) -> Element {
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
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        polygon {
                            points: "12.16,3 11.84,3 9.21,8.25 14.79,8.25",
                        }
                    }
                    g {
                        path {
                            d: "M16.46,8.25h5.16l-2.07-4.14C19.21,3.43,18.52,3,17.76,3h-3.93L16.46,8.25z",
                        }
                    }
                    g {
                        polygon {
                            points: "21.38,9.75 12.75,9.75 12.75,20.1",
                        }
                    }
                    g {
                        polygon {
                            points: "11.25,20.1 11.25,9.75 2.62,9.75",
                        }
                    }
                    g {
                        path {
                            d: "M7.54,8.25L10.16,3H6.24C5.48,3,4.79,3.43,4.45,4.11L2.38,8.25H7.54z",
                        }
                    }
                }
            }
        }
    }
}

pub fn dinner_dining_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M3,14.5h5h1.3H17c0-2.49-2.01-4.5-4.5-4.5c-2.03,0-3.72,1.35-4.28,3.19C7.86,12.92,7.45,12.73,7,12.61V8h1 c1.1,0,2-0.9,2-2v0h6.5C16.78,6,17,5.78,17,5.5S16.78,5,16.5,5H10v0c0-1.1-0.9-2-2-2H3.5C3.22,3,3,3.22,3,3.5S3.22,4,3.5,4H4v1 H3.5C3.22,5,3,5.22,3,5.5S3.22,6,3.5,6H4v1H3.5C3.22,7,3,7.22,3,7.5S3.22,8,3.5,8H4v5.26C3.58,13.59,3.23,14.01,3,14.5z M8.5,7H7 V6h1.5V7z M8.5,4v1H7V4H8.5z M5,4h1v1H5V4z M5,6h1v1H5V6z M5,8h1v4.51c-0.35,0.02-0.68,0.08-1,0.19V8z",
                    }
                    path {
                        d: "M2.85,16.35l0.5,0.5C3.45,16.95,3.57,17,3.71,17h12.59c0.13,0,0.26-0.05,0.35-0.15l0.5-0.5c0.31-0.31,0.09-0.85-0.35-0.85 H3.21C2.76,15.5,2.54,16.04,2.85,16.35z",
                    }
                }
            }
        }
    }
}

pub fn dinner_dining_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M2.85,19.85l1,1C3.95,20.95,4.07,21,4.21,21h15.59c0.13,0,0.26-0.05,0.35-0.15l1-1c0.31-0.31,0.09-0.85-0.35-0.85H3.21 C2.76,19,2.54,19.54,2.85,19.85z",
                    }
                    path {
                        d: "M3,18l16.97,0c0,0,0,0,0,0c0.29-3.26-2.28-6-5.48-6c-2.35,0-4.35,1.48-5.14,3.55C8.94,15.32,8.48,15.17,8,15.08V9h1.75 C10.99,9,12,7.99,12,6.75v0h8.25C20.66,6.75,21,6.41,21,6s-0.34-0.75-0.75-0.75H12v0C12,4.01,10.99,3,9.75,3h-6 C3.34,3,3,3.34,3,3.75S3.34,4.5,3.75,4.5H4v0.75H3.75C3.34,5.25,3,5.59,3,6s0.34,0.75,0.75,0.75H4V7.5H3.75 C3.34,7.5,3,7.84,3,8.25S3.34,9,3.75,9H4v7.39C3.56,16.85,3.22,17.39,3,18z M8,4.5h2v0.75H8V4.5z M8,6.75h2V7.5H8V6.75z M5.5,4.5 h1v0.75h-1V4.5z M5.5,6.75h1V7.5h-1V6.75z M5.5,9h1v6.06c-0.35,0.06-0.68,0.17-1,0.3V9z",
                    }
                }
            }
        }
    }
}

pub fn directions_icons_24px(props: IconProps) -> Element {
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
                d: "m21.71 11.29-9-9a.996.996 0 00-1.41 0l-9 9a.996.996 0 000 1.41l9 9c.39.39 1.02.39 1.41 0l9-9a.996.996 0 000-1.41zM14 14.5V12h-4v2c0 .55-.45 1-1 1s-1-.45-1-1v-3c0-.55.45-1 1-1h5V7.5l3.15 3.15c.2.2.2.51 0 .71L14 14.5z",
            }
        }
    }
}

pub fn directions_bike_icons_24px(props: IconProps) -> Element {
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
                d: "M15.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM5 12c-2.8 0-5 2.2-5 5s2.2 5 5 5 5-2.2 5-5-2.2-5-5-5zm0 8.5c-1.9 0-3.5-1.6-3.5-3.5s1.6-3.5 3.5-3.5 3.5 1.6 3.5 3.5-1.6 3.5-3.5 3.5zm5.8-10l2.4-2.4.8.8c1.06 1.06 2.38 1.78 3.96 2.02.6.09 1.14-.39 1.14-1 0-.49-.37-.91-.85-.99-1.11-.18-2.02-.71-2.75-1.43l-1.9-1.9c-.5-.4-1-.6-1.6-.6s-1.1.2-1.4.6L7.8 8.4c-.4.4-.6.9-.6 1.4 0 .6.2 1.1.6 1.4L11 14v4c0 .55.45 1 1 1s1-.45 1-1v-4.4c0-.52-.2-1.01-.55-1.38L10.8 10.5zM19 12c-2.8 0-5 2.2-5 5s2.2 5 5 5 5-2.2 5-5-2.2-5-5-5zm0 8.5c-1.9 0-3.5-1.6-3.5-3.5s1.6-3.5 3.5-3.5 3.5 1.6 3.5 3.5-1.6 3.5-3.5 3.5z",
            }
        }
    }
}

pub fn directions_boat_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M20,21c-1.19,0-2.38-0.35-3.47-0.98c-0.33-0.19-0.73-0.19-1.07,0c-2.17,1.26-4.76,1.26-6.93,0 c-0.33-0.19-0.73-0.19-1.07,0C6.38,20.65,5.19,21,4,21H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1c1.38,0,2.74-0.35,4-0.99 c2.52,1.29,5.48,1.29,8,0c1.26,0.65,2.62,0.99,4,0.99h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H20z M3.95,19H4 c1.27,0,2.42-0.55,3.33-1.33c0.39-0.34,0.95-0.34,1.34,0C9.58,18.45,10.73,19,12,19s2.42-0.55,3.33-1.33 c0.39-0.34,0.95-0.34,1.34,0C17.58,18.45,18.73,19,20,19h0.05l1.9-6.68c0.11-0.37,0.04-1.06-0.66-1.28L20,10.62V6c0-1.1-0.9-2-2-2 h-3V2c0-0.55-0.45-1-1-1h-4C9.45,1,9,1.45,9,2v2H6C4.9,4,4,4.9,4,6v4.62l-1.29,0.42c-0.63,0.19-0.81,0.84-0.66,1.28L3.95,19z M6,6 h12v3.97L12.62,8.2c-0.41-0.13-0.84-0.13-1.25,0L6,9.97V6z",
                    }
                }
            }
        }
    }
}

pub fn directions_boat_filled_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M20,21c-1.19,0-2.38-0.35-3.47-0.98c-0.33-0.19-0.73-0.19-1.07,0c-2.17,1.26-4.76,1.26-6.93,0 c-0.33-0.19-0.73-0.19-1.07,0C6.38,20.65,5.19,21,4,21H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1c1.38,0,2.74-0.35,4-0.99 c2.52,1.29,5.48,1.29,8,0c1.26,0.65,2.62,0.99,4,0.99h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H20z M3.95,19H4 c1.27,0,2.42-0.55,3.33-1.33c0.39-0.34,0.95-0.34,1.34,0C9.58,18.45,10.73,19,12,19s2.42-0.55,3.33-1.33 c0.39-0.34,0.95-0.34,1.34,0C17.58,18.45,18.73,19,20,19h0.05l1.9-6.68c0.11-0.37,0.04-1.06-0.66-1.28L20,10.62V6c0-1.1-0.9-2-2-2 h-3V2c0-0.55-0.45-1-1-1h-4C9.45,1,9,1.45,9,2v2H6C4.9,4,4,4.9,4,6v4.62l-1.29,0.42c-0.63,0.19-0.81,0.84-0.66,1.28L3.95,19z M6,6 h12v3.97L12.62,8.2c-0.41-0.13-0.84-0.13-1.25,0L6,9.97V6z",
                    }
                }
            }
        }
    }
}

pub fn directions_bus_icons_24px(props: IconProps) -> Element {
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
                d: "M4 16c0 .88.39 1.67 1 2.22v1.28c0 .83.67 1.5 1.5 1.5S8 20.33 8 19.5V19h8v.5c0 .82.67 1.5 1.5 1.5.82 0 1.5-.67 1.5-1.5v-1.28c.61-.55 1-1.34 1-2.22V6c0-3.5-3.58-4-8-4s-8 .5-8 4v10zm3.5 1c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm9 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-6H6V6h12v5z",
            }
        }
    }
}

pub fn directions_bus_filled_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M12,2C8,2,4,2.5,4,6v9.5c0,0.95,0.38,1.81,1,2.44v1.56C5,20.33,5.67,21,6.5,21h0 C7.33,21,8,20.33,8,19.5V19h8v0.5c0,0.82,0.67,1.5,1.5,1.5h0c0.82,0,1.5-0.67,1.5-1.5v-1.56c0.62-0.63,1-1.49,1-2.44V6 C20,2.5,16.42,2,12,2z M8.5,16C7.67,16,7,15.33,7,14.5S7.67,13,8.5,13s1.5,0.67,1.5,1.5S9.33,16,8.5,16z M15.5,16 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,16,15.5,16z M18,10H6V7h12V10z",
                    enable_background: "new",
                }
            }
        }
    }
}

pub fn directions_car_icons_24px(props: IconProps) -> Element {
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
                d: "M18.92 6.01C18.72 5.42 18.16 5 17.5 5h-11c-.66 0-1.21.42-1.42 1.01l-1.97 5.67c-.07.21-.11.43-.11.66v7.16c0 .83.67 1.5 1.5 1.5S6 20.33 6 19.5V19h12v.5c0 .82.67 1.5 1.5 1.5.82 0 1.5-.67 1.5-1.5v-7.16c0-.22-.04-.45-.11-.66l-1.97-5.67zM6.5 16c-.83 0-1.5-.67-1.5-1.5S5.67 13 6.5 13s1.5.67 1.5 1.5S7.33 16 6.5 16zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 11l1.27-3.82c.14-.4.52-.68.95-.68h9.56c.43 0 .81.28.95.68L19 11H5z",
            }
        }
    }
}

pub fn directions_car_filled_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18.92,6.01C18.72,5.42,18.16,5,17.5,5h-11C5.84,5,5.29,5.42,5.08,6.01L3,12v7.5C3,20.33,3.67,21,4.5,21h0 C5.33,21,6,20.33,6,19.5V19h12v0.5c0,0.82,0.67,1.5,1.5,1.5h0c0.82,0,1.5-0.67,1.5-1.5V12L18.92,6.01z M7.5,16 C6.67,16,6,15.33,6,14.5S6.67,13,7.5,13S9,13.67,9,14.5S8.33,16,7.5,16z M16.5,16c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5S17.33,16,16.5,16z M5.81,10l1.04-3h10.29l1.04,3H5.81z",
                }
            }
        }
    }
}

pub fn directions_railway_icons_24px(props: IconProps) -> Element {
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
                d: "M4 15.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V5c0-3.5-3.58-4-8-4s-8 .5-8 4v10.5zm8 1.5c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm6-7H6V5h12v5zM4 15.5C4 17.43 5.57 19 7.5 19l-1.14 1.15c-.32.31-.1.85.35.85h10.58c.45 0 .67-.54.35-.85L16.5 19c1.93 0 3.5-1.57 3.5-3.5V5c0-3.5-3.58-4-8-4s-8 .5-8 4v10.5zm8 1.5c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm6-7H6V5h12v5z",
            }
        }
    }
}

pub fn directions_railway_filled_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                path {
                    d: "M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19l-1.21,0.81C6.11,19.93,6,20.13,6,20.35v0 C6,20.71,6.29,21,6.65,21h10.7c0.36,0,0.65-0.29,0.65-0.65v0c0-0.22-0.11-0.42-0.29-0.54L16.5,19c1.93,0,3.5-1.57,3.5-3.5V6 C20,2.5,16.42,2,12,2z M12,16c-0.83,0-1.5-0.67-1.5-1.5S11.17,13,12,13s1.5,0.67,1.5,1.5S12.83,16,12,16z M18,10H6V7h12V10z",
                    enable_background: "new",
                }
            }
        }
    }
}

pub fn directions_run_icons_24px(props: IconProps) -> Element {
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
                d: "M13.49 5.48c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-3.17 12l.57-2.5 2.1 2v5c0 .55.45 1 1 1s1-.45 1-1v-5.64c0-.55-.22-1.07-.62-1.45l-1.48-1.41.6-3c1.07 1.24 2.62 2.13 4.36 2.41.6.09 1.14-.39 1.14-1 0-.49-.36-.9-.85-.98-1.52-.25-2.78-1.15-3.45-2.33l-1-1.6c-.4-.6-1-1-1.7-1-.3 0-.5.1-.8.1L7.21 7.76c-.74.32-1.22 1.04-1.22 1.85v2.37c0 .55.45 1 1 1s1-.45 1-1v-2.4l1.8-.7-1.6 8.1-3.92-.8c-.54-.11-1.07.24-1.18.78V17c-.11.54.24 1.07.78 1.18l4.11.82c1.06.21 2.1-.46 2.34-1.52z",
            }
        }
    }
}

pub fn directions_subway_icons_24px(props: IconProps) -> Element {
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
                d: "M12 2c-4.42 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19l-1.15 1.15c-.31.31-.09.85.36.85H17.3c.45 0 .67-.54.35-.85L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-3.58-4-8-4zM7.5 17c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm3.5-6H6V6h5v5zm5.5 6c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-6h-5V6h5v5z",
            }
        }
    }
}

pub fn directions_subway_filled_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19l-1.21,0.81C6.11,19.93,6,20.13,6,20.35v0C6,20.71,6.29,21,6.65,21h10.7 c0.36,0,0.65-0.29,0.65-0.65v0c0-0.22-0.11-0.42-0.29-0.54L16.5,19c1.93,0,3.5-1.57,3.5-3.5V6C20,2.5,16.42,2,12,2z M8.5,16 C7.67,16,7,15.33,7,14.5S7.67,13,8.5,13s1.5,0.67,1.5,1.5S9.33,16,8.5,16z M11,10H6V7h5V10z M15.5,16c-0.83,0-1.5-0.67-1.5-1.5 s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,16,15.5,16z M18,10h-5V7h5V10z",
                }
            }
        }
    }
}

pub fn directions_transit_icons_24px(props: IconProps) -> Element {
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
                d: "M12 2c-4.42 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19l-1.15 1.15c-.31.31-.09.85.36.85H17.3c.45 0 .67-.54.35-.85L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-3.58-4-8-4zM7.5 17c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm3.5-6H6V6h5v5zm5.5 6c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-6h-5V6h5v5z",
            }
        }
    }
}

pub fn directions_transit_filled_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19l-1.21,0.81C6.11,19.93,6,20.13,6,20.35v0C6,20.71,6.29,21,6.65,21h10.7 c0.36,0,0.65-0.29,0.65-0.65v0c0-0.22-0.11-0.42-0.29-0.54L16.5,19c1.93,0,3.5-1.57,3.5-3.5V6C20,2.5,16.42,2,12,2z M8.5,16 C7.67,16,7,15.33,7,14.5S7.67,13,8.5,13s1.5,0.67,1.5,1.5S9.33,16,8.5,16z M11,10H6V7h5V10z M15.5,16c-0.83,0-1.5-0.67-1.5-1.5 s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,16,15.5,16z M18,10h-5V7h5V10z",
                }
            }
        }
    }
}

pub fn directions_walk_icons_24px(props: IconProps) -> Element {
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
                d: "M13.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM9.8 8.9L7.24 21.81c-.13.61.35 1.19.98 1.19h.08c.47 0 .87-.32.98-.78L10.9 15l2.1 2v5c0 .55.45 1 1 1s1-.45 1-1v-5.64c0-.55-.22-1.07-.62-1.45L12.9 13.5l.6-3c1.07 1.24 2.62 2.13 4.36 2.41.6.09 1.14-.39 1.14-1 0-.49-.36-.9-.85-.98-1.52-.25-2.78-1.15-3.45-2.33l-1-1.6c-.56-.89-1.68-1.25-2.65-.84L7.22 7.78C6.48 8.1 6 8.82 6 9.63V12c0 .55.45 1 1 1s1-.45 1-1V9.6l1.8-.7",
            }
        }
    }
}

pub fn dry_cleaning_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                path {
                    d: "M16.97,10.87c-0.11-0.62-0.55-1.12-1.13-1.37l-5.09-2.18V6.25c0-0.41-0.34-0.75-0.75-0.75c-0.62,0-1-0.56-1-1 c0-0.55,0.45-1,1-1c0.39,0,0.72,0.22,0.89,0.54c0.14,0.27,0.39,0.46,0.69,0.46c0.54,0,0.92-0.56,0.69-1.04 c-0.46-1.01-1.59-1.66-2.82-1.4C8.5,2.26,7.73,3.05,7.55,4.01c-0.25,1.33,0.55,2.5,1.7,2.86v0.45L4.16,9.5 c-0.57,0.25-1.02,0.75-1.13,1.37C2.83,12.01,3.7,13,4.81,13H6v4c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-4h1.19 C16.3,13,17.17,12.01,16.97,10.87z M15.19,11.5h-1.35C13.67,11.21,13.37,11,13,11H7c-0.37,0-0.67,0.21-0.85,0.5H4.81 c-0.17,0-0.31-0.14-0.31-0.31c0-0.12,0.07-0.23,0.19-0.28L10,8.63l5.31,2.28c0.11,0.05,0.19,0.16,0.19,0.28 C15.5,11.36,15.36,11.5,15.19,11.5z",
                }
            }
        }
    }
}

pub fn dry_cleaning_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                path {
                    d: "M19.56,11.36L13,8.44V7c0-0.55-0.45-1-1-1s-1-0.45-1-1s0.45-1,1-1c0.38,0,0.72,0.22,0.88,0.53C13.04,4.84,13.39,5,13.73,5 c0.74,0,1.26-0.79,0.91-1.44c-0.6-1.1-1.86-1.78-3.24-1.51c-1.17,0.23-2.12,1.2-2.34,2.37C8.77,5.98,9.67,7.35,11,7.82v0.63 l-6.56,2.92C3.56,11.75,3,12.62,3,13.57v0.01C3,14.92,4.08,16,5.42,16H7v4c0,1.1,0.9,2,2,2h6c1.1,0,2-0.9,2-2v-4h1.58 c1.34,0,2.42-1.08,2.42-2.42v-0.01C21,12.62,20.44,11.75,19.56,11.36z M18.58,14h-1.86c-0.35-0.6-0.98-1-1.72-1H9 c-0.74,0-1.38,0.4-1.72,1H5.42c-0.46,0-0.58-0.65-0.17-0.81l6.75-3l6.75,3C19.17,13.38,19.03,14,18.58,14z",
                }
            }
        }
    }
}

pub fn edit_attributes_icons_24px(props: IconProps) -> Element {
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
                d: "M17.63 7H6.37C3.96 7 2 9.24 2 12s1.96 5 4.37 5h11.26c2.41 0 4.37-2.24 4.37-5s-1.96-5-4.37-5zm-6.52 3.6L7.6 14.11c-.1.1-.23.15-.35.15s-.26-.05-.35-.15l-1.86-1.86c-.2-.2-.2-.51 0-.71s.51-.2.71 0l1.51 1.51 3.16-3.16c.2-.2.51-.2.71 0s.17.51-.02.71z",
            }
        }
    }
}

pub fn edit_location_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M12,2c-4.2,0-8,3.22-8,8.2c0,3.18,2.45,6.92,7.34,11.23c0.38,0.33,0.95,0.33,1.33,0C17.55,17.12,20,13.38,20,10.2 C20,5.22,16.2,2,12,2z M9.73,13.5H8.5v-1.44l3.93-3.92l1.43,1.43l-3.77,3.78C9.99,13.45,9.87,13.5,9.73,13.5z M15.28,8.16l-0.7,0.7 l-1.44-1.44l0.7-0.7c0.15-0.15,0.39-0.15,0.54,0l0.9,0.9C15.43,7.77,15.43,8.01,15.28,8.16z",
                }
            }
        }
    }
}

pub fn edit_location_alt_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                g {
                    g {
                        polygon {
                            points: "9,10 10.41,10 15.44,4.97 14.03,3.56 9,8.59",
                        }
                    }
                    g {
                        path {
                            d: "M10.62,11.5H8.5c-0.55,0-1-0.45-1-1V8.38c0-0.27,0.11-0.52,0.29-0.71l5.02-5.02C11.96,2.24,11.01,2,10,2 C6.41,2,3.5,4.91,3.5,8.5C3.5,12.84,10,18,10,18s6.5-5.16,6.5-9.5c0-0.74-0.13-1.45-0.36-2.11l-4.82,4.82 C11.14,11.39,10.89,11.5,10.62,11.5z",
                        }
                    }
                    g {
                        path {
                            d: "M16.85,2.85l-0.71-0.71c-0.2-0.2-0.51-0.2-0.71,0l-0.71,0.71l1.41,1.41l0.71-0.71C17.05,3.37,17.05,3.05,16.85,2.85z",
                        }
                    }
                }
            }
        }
    }
}

pub fn edit_location_alt_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M13.54,13H10c-0.55,0-1-0.45-1-1V8.46C9,8.2,9.11,7.94,9.29,7.76l5.32-5.32C13.78,2.16,12.9,2,12,2c-4.2,0-8,3.22-8,8.2 c0,3.18,2.44,6.92,7.33,11.22c0.38,0.33,0.96,0.33,1.34,0C17.56,17.12,20,13.37,20,10.2c0-1.01-0.16-1.94-0.45-2.8l-5.31,5.31 C14.06,12.89,13.8,13,13.54,13z",
                        }
                    }
                    polygon {
                        points: "11,11 13.12,11 19.28,4.84 17.16,2.72 11,8.88",
                    }
                    path {
                        d: "M20.71,2L20,1.29c-0.39-0.39-1.02-0.39-1.41,0l-0.72,0.72l2.12,2.12l0.72-0.72C21.1,3.02,21.1,2.39,20.71,2z",
                    }
                }
            }
        }
    }
}

pub fn edit_road_icons_20px(props: IconProps) -> Element {
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
                        d: "M4.5,4C4.22,4,4,4.22,4,4.5v11C4,15.78,4.22,16,4.5,16S5,15.78,5,15.5v-11C5,4.22,4.78,4,4.5,4z",
                    }
                    path {
                        d: "M14,4.5C14,4.22,13.78,4,13.5,4S13,4.22,13,4.5v5.96l1-1V4.5z",
                    }
                    path {
                        d: "M9,4C8.72,4,8.5,4.22,8.5,4.5v1C8.5,5.78,8.72,6,9,6s0.5-0.22,0.5-0.5v-1C9.5,4.22,9.28,4,9,4z",
                    }
                    path {
                        d: "M9,14c-0.28,0-0.5,0.22-0.5,0.5v1C8.5,15.78,8.72,16,9,16s0.5-0.22,0.5-0.5v-1C9.5,14.22,9.28,14,9,14z",
                    }
                    path {
                        d: "M9,9C8.72,9,8.5,9.22,8.5,9.5v1C8.5,10.78,8.72,11,9,11s0.5-0.22,0.5-0.5v-1C9.5,9.22,9.28,9,9,9z",
                    }
                    path {
                        d: "M18.07,9.64l-0.71-0.71c-0.39-0.39-1.02-0.39-1.41,0L11,13.88V16h2.12l4.95-4.95C18.46,10.66,18.46,10.03,18.07,9.64z M12.71,15H12v-0.71l3.24-3.24l0.71,0.71L12.71,15z",
                    }
                }
            }
        }
    }
}

pub fn edit_road_icons_24px(props: IconProps) -> Element {
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
                        d: "M17,4L17,4c-0.55,0-1,0.45-1,1v6.9l2-2V5C18,4.45,17.55,4,17,4z",
                    }
                    path {
                        d: "M5,20L5,20c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h0C4.45,4,4,4.45,4,5v14C4,19.55,4.45,20,5,20z",
                    }
                    path {
                        d: "M11,8L11,8c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C10,7.55,10.45,8,11,8z",
                    }
                    path {
                        d: "M11,14L11,14c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C10,13.55,10.45,14,11,14z",
                    }
                    path {
                        d: "M11,20L11,20c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C10,19.55,10.45,20,11,20z",
                    }
                    path {
                        d: "M22.56,12.59l-1.15-1.15c-0.59-0.59-1.54-0.59-2.12,0L14,16.73v2.77c0,0.28,0.22,0.5,0.5,0.5h2.77l5.29-5.29 C23.15,14.12,23.15,13.17,22.56,12.59z M16.58,18.45h-1.03v-1.03L19,13.97L20.03,15L16.58,18.45z",
                    }
                }
            }
        }
    }
}

pub fn egg_icons_20px(props: IconProps) -> Element {
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
                        d: "M10,3c-2.75,0-5.5,4.93-5.5,8.56C4.5,14.56,6.96,17,10,17s5.5-2.44,5.5-5.44C15.5,7.93,12.75,3,10,3z M10.25,14.5 C8.37,14.5,7,12.92,7,10.75C7,10.34,7.34,10,7.75,10s0.75,0.34,0.75,0.75c0,1.12,0.54,2.25,1.75,2.25c0.41,0,0.75,0.34,0.75,0.75 S10.66,14.5,10.25,14.5z",
                    }
                }
            }
        }
    }
}

pub fn egg_icons_24px(props: IconProps) -> Element {
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
                    path {
                        d: "M12,3C8.5,3,5,9.33,5,14c0,3.87,3.13,7,7,7s7-3.13,7-7C19,9.33,15.5,3,12,3z M13,18c-3,0-5-1.99-5-5c0-0.55,0.45-1,1-1 s1,0.45,1,1c0,2.92,2.42,3,3,3c0.55,0,1,0.45,1,1S13.55,18,13,18z",
                    }
                }
            }
        }
    }
}

pub fn egg_alt_icons_20px(props: IconProps) -> Element {
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
                        d: "M15.6,7.6C14,6,13.19,2,8.38,2C4.36,2,1.95,5.2,2,9.62S5.97,16,8.38,16c1.61,0,1.61,2,4.82,2C15.6,18,18,15.6,18,12.42 C18,10,17.21,9.2,15.6,7.6z M10,12.5c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5 C12.5,11.38,11.38,12.5,10,12.5z",
                    }
                }
            }
        }
    }
}

pub fn egg_alt_icons_24px(props: IconProps) -> Element {
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
                        d: "M19,9C17,7,15.99,2,9.97,2C4.95,2,1.94,6,2,11.52C2.06,17.04,6.96,19,9.97,19c2.01,0,2.01,3,6.02,3C19,22,22,19,22,15.02 C22,12,21.01,11,19,9z M12,15.5c-1.93,0-3.5-1.57-3.5-3.5s1.57-3.5,3.5-3.5s3.5,1.57,3.5,3.5S13.93,15.5,12,15.5z",
                    }
                }
            }
        }
    }
}

pub fn electrical_services_icons_20px(props: IconProps) -> Element {
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
                        d: "M16,11.5c0-0.28-0.22-0.5-0.5-0.5H14v1h1.5C15.78,12,16,11.78,16,11.5z",
                    }
                    path {
                        d: "M15.5,14H14v1h1.5c0.28,0,0.5-0.22,0.5-0.5C16,14.22,15.78,14,15.5,14z",
                    }
                    path {
                        d: "M12.5,10H10c-0.55,0-1,0.45-1,1v1H8.5C8.22,12,8,12.22,8,12.5v1C8,13.77,8.23,14,8.5,14H9l0,1c0,0.55,0.45,1,1,1h2.5 c0.28,0,0.5-0.22,0.5-0.5v-5C13,10.22,12.78,10,12.5,10z",
                    }
                    path {
                        d: "M6.75,9H7.5C8.88,9,10,7.88,10,6.5C10,5.12,8.88,4,7.5,4h-2C5.22,4,5,4.22,5,4.5C5,4.78,5.22,5,5.5,5h2 C8.33,5,9,5.67,9,6.5S8.33,8,7.5,8H6.75C5.23,8,4,9.23,4,10.75s1.23,2.75,2.75,2.75H7v-1H6.75C5.78,12.5,5,11.72,5,10.75 S5.78,9,6.75,9z",
                    }
                }
            }
        }
    }
}

pub fn electrical_services_icons_24px(props: IconProps) -> Element {
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
                        d: "M21,14c0-0.55-0.45-1-1-1h-2v2h2C20.55,15,21,14.55,21,14z",
                    }
                    path {
                        d: "M20,17h-2v2h2c0.55,0,1-0.45,1-1C21,17.45,20.55,17,20,17z",
                    }
                    path {
                        d: "M16,12h-2c-1.1,0-2,0.9-2,2h-1c-0.55,0-1,0.45-1,1v2c0,0.55,0.45,1,1,1h1c0,1.1,0.9,2,2,2h2c0.55,0,1-0.45,1-1v-6 C17,12.45,16.55,12,16,12z",
                    }
                    path {
                        d: "M5,13c0-1.1,0.9-2,2-2h1.5c1.93,0,3.5-1.57,3.5-3.5S10.43,4,8.5,4H5C4.45,4,4,4.45,4,5c0,0.55,0.45,1,1,1h3.5 C9.33,6,10,6.67,10,7.5S9.33,9,8.5,9H7c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4h2v-2H7C5.9,15,5,14.1,5,13z",
                    }
                }
            }
        }
    }
}

pub fn electric_bike_icons_20px(props: IconProps) -> Element {
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
                        d: "M15.5,7h-0.68l-1.58-4.34C13.1,2.26,12.72,2,12.3,2h-1.8C10.22,2,10,2.22,10,2.5C10,2.78,10.22,3,10.5,3h1.8l1.46,4H8.75 L8.38,6H9.5C9.78,6,10,5.78,10,5.5C10,5.22,9.78,5,9.5,5h-3C6.22,5,6,5.22,6,5.5C6,5.78,6.22,6,6.5,6h0.82l1.46,4H7.95 C7.7,8.22,6.18,6.9,4.29,7.01c-1.68,0.09-3.1,1.43-3.27,3.1C0.8,12.25,2.41,14,4.5,14c1.79,0,3.21-1.29,3.45-3h4.1 c0.25,1.78,1.77,3.1,3.66,2.99c1.68-0.09,3.1-1.43,3.27-3.1C19.2,8.75,17.59,7,15.5,7z M4.5,11h2.45c-0.23,1.15-1.22,2-2.45,2 C3.1,13,2,11.9,2,10.5C2,9.1,3.22,7.94,4.62,8c1.17,0.05,2.11,0.88,2.33,2H4.5C4.22,10,4,10.22,4,10.5C4,10.78,4.22,11,4.5,11z M9.84,10L9.11,8h3.92c-0.53,0.52-0.88,1.22-0.98,2H9.84z M15.48,13C14.13,12.99,13,11.85,13,10.5c0-0.94,0.5-1.73,1.24-2.16 l0.86,2.36c0.09,0.26,0.38,0.39,0.64,0.3c0.26-0.09,0.39-0.38,0.3-0.64l-0.85-2.33C15.3,8.02,15.4,8,15.5,8c1.4,0,2.5,1.1,2.5,2.5 C18,11.91,16.89,13.01,15.48,13z",
                    }
                    polygon {
                        points: "9.5,16 7,16 10.5,18 10.5,17 13,17 9.5,15",
                    }
                }
            }
        }
    }
}

pub fn electric_bike_icons_24px(props: IconProps) -> Element {
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
                        d: "M19,7h-0.82l-1.7-4.68C16.19,1.53,15.44,1,14.6,1H13c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h1.6l1.46,4h-4.81l-0.36-1h0.09 c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H8C7.45,4,7,4.45,7,5c0,0.55,0.45,1,1,1h0.75l1.82,5H9.9C9.46,8.77,7.59,7.12,5.25,7.01 C2.45,6.87,0,9.2,0,12c0,2.8,2.2,5,5,5c2.46,0,4.45-1.69,4.9-4h4.2c0.44,2.23,2.31,3.88,4.65,3.99c2.8,0.13,5.25-2.19,5.25-5 C24,9.2,21.8,7,19,7z M6,13h1.82c-0.42,1.23-1.6,2.08-3.02,1.99C3.31,14.9,2.07,13.64,2,12.14C1.93,10.39,3.27,9,5,9 c1.33,0,2.42,0.83,2.82,2H6c-0.55,0-1,0.45-1,1C5,12.55,5.45,13,6,13z M14.1,11h-1.4l-0.73-2H15C14.56,9.58,14.24,10.25,14.1,11z M18.88,15c-1.54-0.06-2.84-1.37-2.88-2.92c-0.02-0.96,0.39-1.8,1.05-2.36l0.62,1.7c0.19,0.52,0.76,0.79,1.28,0.6 c0.52-0.19,0.79-0.76,0.6-1.28l-0.63-1.73c0,0,0,0,0.01-0.01C20.64,8.96,22,10.29,22,12C22,13.72,20.62,15.06,18.88,15z",
                    }
                    polygon {
                        points: "11,20 7,20 13,23 13,21 17,21 11,18",
                    }
                }
            }
        }
    }
}

pub fn electric_car_icons_20px(props: IconProps) -> Element {
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
                    d: "M14.28,3.55C14.11,3.21,13.76,3,13.38,3H6.62C6.24,3,5.89,3.21,5.72,3.55L4,7v5c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v0h8 v0c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V7L14.28,3.55z M6.93,4h6.15c0.19,0,0.36,0.11,0.45,0.28L14.88,7H5.12l1.36-2.72 C6.56,4.11,6.74,4,6.93,4z M6,10c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C7,9.55,6.55,10,6,10z M14,10c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C15,9.55,14.55,10,14,10z",
                }
                polygon {
                    points: "7,15 9.5,15 9.5,14 13,16 10.5,16 10.5,17",
                }
            }
        }
    }
}

pub fn electric_car_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18.92,2.01C18.72,1.42,18.16,1,17.5,1h-11C5.84,1,5.29,1.42,5.08,2.01L3.11,7.68C3.04,7.89,3,8.11,3,8.34v7.16 C3,16.33,3.67,17,4.5,17h0C5.33,17,6,16.33,6,15.5V15h12v0.5c0,0.82,0.67,1.5,1.5,1.5h0c0.82,0,1.5-0.67,1.5-1.5V8.34 c0-0.22-0.04-0.45-0.11-0.66L18.92,2.01z M6.5,12C5.67,12,5,11.33,5,10.5S5.67,9,6.5,9S8,9.67,8,10.5S7.33,12,6.5,12z M17.5,12 c-0.83,0-1.5-0.67-1.5-1.5S16.67,9,17.5,9S19,9.67,19,10.5S18.33,12,17.5,12z M5,7l1.27-3.82C6.41,2.78,6.79,2.5,7.22,2.5h9.56 c0.43,0,0.81,0.28,0.95,0.68L19,7H5z",
                }
                polygon {
                    points: "7,20 11,20 11,18 17,21 13,21 13,23",
                }
            }
        }
    }
}

pub fn electric_moped_icons_20px(props: IconProps) -> Element {
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
                        d: "M15,5.5C15,4.67,14.33,4,13.5,4h-1C12.22,4,12,4.22,12,4.5v0C12,4.78,12.22,5,12.5,5h1C13.78,5,14,5.22,14,5.5v1.29 L10.79,10H9V6.5C9,6.22,8.78,6,8.5,6H6C4.34,6,3,7.34,3,9v2h2c0,1.1,0.9,2,2,2s2-0.9,2-2h2.21L15,7.21V5.5z M7,12 c-0.55,0-1-0.45-1-1h2C8,11.55,7.55,12,7,12z",
                    }
                    path {
                        d: "M8.5,4h-3C5.22,4,5,4.22,5,4.5v0C5,4.78,5.22,5,5.5,5h3C8.78,5,9,4.78,9,4.5v0C9,4.22,8.78,4,8.5,4z",
                    }
                    path {
                        d: "M15,9c-1.1,0-2,0.9-2,2c0,1.1,0.9,2,2,2s2-0.9,2-2C17,9.9,16.1,9,15,9z M15,12c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 s1,0.45,1,1C16,11.55,15.55,12,15,12z",
                    }
                }
                polygon {
                    points: "7,15 9.5,15 9.5,14 13,16 10.5,16 10.5,17",
                }
            }
        }
    }
}

pub fn electric_moped_icons_24px(props: IconProps) -> Element {
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
                    path {
                        d: "M19,5c0-1.1-0.9-2-2-2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2.65L13.52,12H10V8c0-0.55-0.45-1-1-1H6 c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,8.35V5z M7,15c-0.55,0-1-0.45-1-1h2C8,14.55,7.55,15,7,15z",
                    }
                    path {
                        d: "M9,4H6C5.45,4,5,4.45,5,5v0c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v0C10,4.45,9.55,4,9,4z",
                    }
                    path {
                        d: "M19,11c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,11,19,11z M19,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S19.55,15,19,15z",
                    }
                }
                polygon {
                    points: "7,20 11,20 11,18 17,21 13,21 13,23",
                }
            }
        }
    }
}

pub fn electric_rickshaw_icons_20px(props: IconProps) -> Element {
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
                polygon {
                    points: "7,16 9.5,16 9.5,15 13,17 10.5,17 10.5,18",
                }
                path {
                    d: "M17,9.05v-1.7c0-0.23-0.08-0.45-0.22-0.62L13.3,2.38C13.11,2.14,12.82,2,12.52,2H2C1.45,2,1,2.45,1,3v8c0,0.55,0.45,1,1,1 h1.05c0.23,1.14,1.24,2,2.45,2s2.22-0.86,2.45-2h6.1c0.28,1.38,1.69,2.34,3.22,1.89c0.77-0.23,1.39-0.85,1.62-1.62 C19.34,10.74,18.38,9.33,17,9.05z M5.5,13C4.67,13,4,12.33,4,11.5S4.67,10,5.5,10S7,10.67,7,11.5S6.33,13,5.5,13z M7,7H2V3.5 C2,3.22,2.22,3,2.5,3H7V7z M12,11H8V8h1.5C9.78,8,10,7.78,10,7.5v0C10,7.22,9.78,7,9.5,7H8V3h3.5C11.78,3,12,3.22,12,3.5V11z M13,7 V3.6L15.72,7H13z M16.5,13c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S17.33,13,16.5,13z",
                }
            }
        }
    }
}

pub fn electric_rickshaw_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M21,11.18V9.72c0-0.47-0.16-0.92-0.46-1.28L16.6,3.72C16.22,3.26,15.66,3,15.06,3H3C1.9,3,1,3.9,1,5v8c0,1.1,0.9,2,2,2 h0.18C3.6,16.16,4.7,17,6,17s2.4-0.84,2.82-2h8.37c0.41,1.16,1.51,2,2.82,2c1.66,0,3-1.34,3-3C23,12.7,22.16,11.6,21,11.18z M18.4,9H16V6.12L18.4,9z M4,5h3v4H3V6C3,5.45,3.45,5,4,5z M6,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.55,15,6,15z M9,13v-2 h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H9V5h4c0.55,0,1,0.45,1,1v7H9z M20,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S20.55,15,20,15z",
                }
                polygon {
                    points: "7,20 11,20 11,18 17,21 13,21 13,23",
                }
            }
        }
    }
}

pub fn electric_scooter_icons_20px(props: IconProps) -> Element {
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
                        d: "M15,10c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S16.1,10,15,10z M15,13c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S15.55,13,15,13z",
                    }
                    path {
                        d: "M12,11.99C12,10.34,13.34,9,14.99,9l0.24,0l-1.2-5.22C13.93,3.32,13.52,3,13.05,3H10.5C10.22,3,10,3.22,10,3.5 C10,3.78,10.22,4,10.5,4h2.55l0.95,4.14c-1.57,0.4-2.75,1.72-2.96,3.36H6.93c-0.26-1-1.27-1.71-2.4-1.45 c-0.72,0.17-1.3,0.75-1.47,1.47C2.75,12.83,3.74,14,5,14c0.93,0,1.71-0.64,1.93-1.5H12V11.99z M5,13c-0.55,0-1-0.45-1-1 s0.45-1,1-1s1,0.45,1,1S5.55,13,5,13z",
                    }
                    polygon {
                        points: "9.5,15 7,15 10.5,17 10.5,16 13,16 9.5,14",
                    }
                }
            }
        }
    }
}

pub fn electric_scooter_icons_24px(props: IconProps) -> Element {
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
                    path {
                        d: "M7.82,16H15v-1c0-2.21,1.79-4,4-4h0.74l-1.9-8.44C17.63,1.65,16.82,1,15.89,1H13c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1 h2.89l1.4,6.25c0,0-0.01,0-0.01,0c-2.16,0.65-3.81,2.48-4.19,4.75H7.82c-0.48-1.34-1.86-2.24-3.42-1.94 c-1.18,0.23-2.13,1.2-2.35,2.38C1.7,16.34,3.16,18,5,18C6.3,18,7.4,17.16,7.82,16z M5,16c-0.55,0-1-0.45-1-1s0.45-1,1-1 s1,0.45,1,1S5.55,16,5,16z",
                    }
                    path {
                        d: "M19,12c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,12,19,12z M19,16c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S19.55,16,19,16z",
                    }
                    polygon {
                        points: "11,20 7,20 13,23 13,21 17,21 11,18",
                    }
                }
            }
        }
    }
}

pub fn emergency_icons_20px(props: IconProps) -> Element {
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
                d: "M16.31,6.93l-0.5-0.87c-0.28-0.48-0.89-0.64-1.37-0.37L11.5,7.4V4c0-0.55-0.45-1-1-1h-1c-0.55,0-1,0.45-1,1v3.4L5.55,5.7 C5.08,5.42,4.46,5.59,4.19,6.07l-0.5,0.87C3.41,7.41,3.58,8.02,4.05,8.3L7,10l-2.95,1.7c-0.48,0.28-0.64,0.89-0.37,1.37l0.5,0.87 c0.28,0.48,0.89,0.64,1.37,0.37l2.95-1.7V16c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-3.4l2.95,1.7c0.48,0.28,1.09,0.11,1.37-0.37 l0.5-0.87c0.28-0.48,0.11-1.09-0.37-1.37L13,10l2.95-1.7C16.42,8.02,16.59,7.41,16.31,6.93z",
            }
        }
    }
}

pub fn emergency_icons_24px(props: IconProps) -> Element {
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
                d: "M20.29,8.37l-1-1.73c-0.28-0.48-0.89-0.64-1.37-0.37L14,8.54V4c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v4.54L6.07,6.27 C5.59,5.99,4.98,6.16,4.71,6.63l-1,1.73C3.43,8.84,3.59,9.46,4.07,9.73L8,12l-3.93,2.27c-0.48,0.28-0.64,0.89-0.37,1.37l1,1.73 c0.28,0.48,0.89,0.64,1.37,0.37L10,15.46V20c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-4.54l3.93,2.27c0.48,0.28,1.09,0.11,1.37-0.37 l1-1.73c0.28-0.48,0.11-1.09-0.37-1.37L16,12l3.93-2.27C20.41,9.46,20.57,8.84,20.29,8.37z",
            }
        }
    }
}

pub fn emergency_recording_icons_20px(props: IconProps) -> Element {
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
                    d: "M17.15,6.85L15,9V5.5C15,4.67,14.33,4,13.5,4h-9C3.67,4,3,4.67,3,5.5v9C3,15.33,3.67,16,4.5,16h9c0.83,0,1.5-0.67,1.5-1.5 V11l2.15,2.15c0.31,0.31,0.85,0.09,0.85-0.35V7.21C18,6.76,17.46,6.54,17.15,6.85z M12.38,11.95c-0.21,0.36-0.67,0.48-1.02,0.27 l-1.6-0.92V13c0,0.41-0.34,0.75-0.75,0.75S8.25,13.41,8.25,13v-1.7l-1.6,0.92c-0.36,0.21-0.82,0.08-1.02-0.27 c-0.21-0.36-0.08-0.82,0.27-1.02L7.5,10L5.9,9.08C5.54,8.87,5.42,8.41,5.63,8.05c0.21-0.36,0.67-0.48,1.02-0.27l1.6,0.92V7 c0-0.41,0.34-0.75,0.75-0.75S9.75,6.59,9.75,7v1.7l1.6-0.92c0.36-0.21,0.82-0.08,1.02,0.27c0.21,0.36,0.08,0.82-0.27,1.02L10.5,10 l1.6,0.92C12.46,11.13,12.58,11.59,12.38,11.95z",
                }
            }
        }
    }
}

pub fn emergency_recording_icons_24px(props: IconProps) -> Element {
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
                    d: "M18,10.48V6c0-1.1-0.9-2-2-2H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-4.48l3.15,3.13 C21.46,16.97,22,16.74,22,16.3V7.7c0-0.44-0.54-0.67-0.85-0.35L18,10.48z M14.5,14.6c-0.28,0.48-0.89,0.64-1.37,0.37L11,13.73V16 c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2.27l-2.13,1.23c-0.48,0.28-1.09,0.11-1.37-0.37c-0.28-0.48-0.11-1.09,0.37-1.37L8,12l-2.13-1.23 C5.39,10.49,5.22,9.88,5.5,9.4c0.28-0.48,0.89-0.64,1.37-0.37L9,10.27V8c0-0.55,0.45-1,1-1s1,0.45,1,1v2.27l2.13-1.23 c0.48-0.28,1.09-0.11,1.37,0.37c0.28,0.48,0.11,1.09-0.37,1.37L12,12l2.13,1.23C14.61,13.51,14.78,14.12,14.5,14.6z",
                }
            }
        }
    }
}

pub fn emergency_share_icons_20px(props: IconProps) -> Element {
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
                    d: "M15.78,3.22c-0.27,0.27-0.7,0.31-0.99,0.06c-1.31-1.11-3-1.78-4.84-1.78c-1.82,0-3.49,0.65-4.79,1.74 c-0.29,0.24-0.72,0.2-0.99-0.06C3.86,2.86,3.89,2.34,4.24,2.05C5.79,0.77,7.78,0,9.95,0c2.2,0,4.21,0.79,5.77,2.1 C16.07,2.38,16.1,2.9,15.78,3.22z M13.29,5.71c0.32-0.32,0.29-0.86-0.07-1.14C12.3,3.9,11.18,3.5,9.95,3.5 c-1.2,0-2.3,0.38-3.2,1.03C6.38,4.8,6.35,5.35,6.67,5.67c0.26,0.26,0.67,0.29,0.96,0.08C8.29,5.28,9.09,5,9.95,5 c0.89,0,1.7,0.29,2.37,0.78C12.62,6,13.03,5.97,13.29,5.71z M15.28,12.28c0,2.75-3.2,5.89-4.62,7.15c-0.38,0.34-0.95,0.34-1.33,0 c-1.42-1.26-4.62-4.41-4.62-7.15C4.72,9.36,7.08,7,10,7S15.28,9.36,15.28,12.28z M10,11c-0.69,0-1.25,0.56-1.25,1.25 c0,0.69,0.56,1.25,1.25,1.25c0.69,0,1.25-0.56,1.25-1.25C11.25,11.56,10.69,11,10,11z",
                }
            }
        }
    }
}

pub fn emergency_share_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M12,9c-3.15,0-6,2.41-6,6.15c0,2.35,1.78,5.11,5.34,8.27c0.37,0.33,0.95,0.33,1.33,0C16.22,20.25,18,17.5,18,15.15 C18,11.41,15.15,9,12,9z M12,16.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S12.83,16.5,12,16.5z M16.18,6.82 c-0.35,0.35-0.89,0.38-1.3,0.09C14.07,6.34,13.07,6,12,6S9.93,6.34,9.12,6.91c-0.41,0.28-0.95,0.26-1.3-0.09 c-0.43-0.43-0.39-1.15,0.09-1.5C9.06,4.49,10.48,4,12,4s2.94,0.49,4.09,1.32C16.58,5.67,16.61,6.39,16.18,6.82z M4.97,3.97 C4.55,3.54,4.59,2.85,5.05,2.47C6.95,0.93,9.37,0,12.01,0c2.64,0,5.06,0.93,6.95,2.48c0.46,0.38,0.5,1.07,0.08,1.49 c-0.36,0.36-0.93,0.39-1.32,0.07C16.16,2.77,14.17,2,12.01,2C9.83,2,7.84,2.77,6.29,4.04C5.9,4.36,5.33,4.32,4.97,3.97z",
                }
            }
        }
    }
}

pub fn ev_station_icons_24px(props: IconProps) -> Element {
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
                d: "M19.77 7.23l.01-.01-3.19-3.19c-.29-.29-.77-.29-1.06 0-.29.29-.29.77 0 1.06l1.58 1.58c-1.05.4-1.76 1.47-1.58 2.71.16 1.1 1.1 1.99 2.2 2.11.47.05.88-.03 1.27-.2v7.21c0 .55-.45 1-1 1s-1-.45-1-1V14c0-1.1-.9-2-2-2h-1V5c0-1.1-.9-2-2-2H6c-1.1 0-2 .9-2 2v15c0 .55.45 1 1 1h8c.55 0 1-.45 1-1v-6.5h1.5v4.86c0 1.31.94 2.5 2.24 2.63 1.5.15 2.76-1.02 2.76-2.49V9c0-.69-.28-1.32-.73-1.77zM18 10c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zM8 16.12V13.5H6.83c-.38 0-.62-.4-.44-.74l2.67-5c.24-.45.94-.28.94.24v3h1.14c.38 0 .62.41.43.75l-2.64 4.62c-.25.44-.93.26-.93-.25z",
            }
        }
    }
}

pub fn factory_icons_20px(props: IconProps) -> Element {
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
                        d: "M17.85,7.5l-0.78-5.08C17.03,2.18,16.82,2,16.57,2h-1.14c-0.25,0-0.46,0.18-0.49,0.42L14.15,7.5h2.35H17.85z",
                    }
                    path {
                        d: "M10.8,6.31L7.5,7.75V6.78c0-0.36-0.38-0.61-0.71-0.46L2.88,8.1C2.34,8.34,2,8.88,2,9.47v7.03C2,17.33,2.67,18,3.5,18h13 c0.83,0,1.5-0.67,1.5-1.5v-8h-6.5V6.76C11.5,6.4,11.13,6.16,10.8,6.31z M12.5,12.25c0-0.41,0.34-0.75,0.75-0.75S14,11.84,14,12.25 v2c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75V12.25z M7.5,14.25C7.5,14.66,7.16,15,6.75,15S6,14.66,6,14.25v-2 c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75V14.25z M10.75,14.25c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75v-2 c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75V14.25z",
                    }
                }
            }
        }
    }
}

pub fn factory_icons_24px(props: IconProps) -> Element {
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
                        d: "M14,10V8.48c0-0.71-0.71-1.19-1.37-0.93L9,9V8.52C9,7.8,8.27,7.31,7.61,7.6L3.21,9.48C2.48,9.8,2,10.52,2,11.32V20 c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V10H14z M9,17c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2c0-0.55,0.45-1,1-1s1,0.45,1,1V17z M13,17 c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2c0-0.55,0.45-1,1-1s1,0.45,1,1V17z M17,17c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2c0-0.55,0.45-1,1-1 s1,0.45,1,1V17z",
                    }
                    path {
                        d: "M20.12,2h-1.23c-0.51,0-0.93,0.38-0.99,0.88L17.2,8.5h4.6l-0.69-5.62C21.05,2.38,20.62,2,20.12,2z",
                    }
                }
            }
        }
    }
}

pub fn fastfood_icons_24px(props: IconProps) -> Element {
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
                d: "M21.9 5H18V2c0-.55-.45-1-1-1s-1 .45-1 1v3h-3.9c-.59 0-1.05.51-1 1.1l.12 1.21C14.9 8.16 18 10.77 18 15l.02 8h1.7c.84 0 1.53-.65 1.63-1.47L22.89 6.1c.06-.59-.4-1.1-.99-1.1zM15 21H2c-.55 0-1 .45-1 1s.45 1 1 1h13c.55 0 1-.45 1-1s-.45-1-1-1zM2.1 15h12.8c.62 0 1.11-.56.99-1.16-.65-3.23-4.02-4.85-7.39-4.85s-6.73 1.62-7.39 4.85c-.12.6.38 1.16.99 1.16zM15 17H2c-.55 0-1 .45-1 1s.45 1 1 1h13c.55 0 1-.45 1-1s-.45-1-1-1z",
            }
        }
    }
}

pub fn festival_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                path {
                    d: "M9.01,2.92C7.56,4.2,4.74,6.47,2.11,7.26C1.46,7.45,1,8.03,1,8.7V9c0,1.23,0.79,2.26,1.89,2.64 c-0.08,1.58-0.2,3.3-0.42,4.62C2.32,17.18,3.02,18,3.95,18h12.1c0.93,0,1.64-0.83,1.48-1.75c-0.17-0.96-0.34-2.42-0.44-4.61 C18.2,11.27,19,10.24,19,9V8.7c0-0.68-0.46-1.25-1.11-1.44c-2.64-0.79-5.46-3.06-6.91-4.34C10.42,2.41,9.58,2.41,9.01,2.92z M9.25,9c0,0.72-0.59,1.31-1.31,1.31S6.63,9.72,6.63,9H9.25z M8.23,11.78c0.7-0.08,1.32-0.39,1.77-0.88 c0.45,0.49,1.07,0.81,1.77,0.88c0.03,0.95,0.13,2.67,0.4,4.72H7.83C8.11,14.45,8.2,12.73,8.23,11.78z M12.06,10.31 c-0.72,0-1.31-0.59-1.31-1.31h2.62C13.37,9.72,12.79,10.31,12.06,10.31z M10,4.05c1.07,0.95,2.79,2.37,4.72,3.45H5.28 C7.21,6.42,8.93,5.01,10,4.05z M2.5,9h2.62c0,0.72-0.59,1.31-1.31,1.31S2.5,9.72,2.5,9z M4.39,11.75c0.58-0.12,1.09-0.43,1.48-0.85 c0.24,0.27,0.54,0.48,0.87,0.64c-0.03,0.88-0.11,2.76-0.43,4.97H3.95C4.18,15.17,4.31,13.37,4.39,11.75z M13.69,16.5 c-0.31-2.21-0.4-4.09-0.43-4.97c0.33-0.16,0.62-0.37,0.87-0.64c0.39,0.42,0.9,0.73,1.48,0.85c0.08,1.62,0.21,3.42,0.44,4.75H13.69z M16.19,10.31c-0.72,0-1.31-0.59-1.31-1.31h2.62C17.5,9.72,16.91,10.31,16.19,10.31z",
                }
            }
        }
    }
}

pub fn festival_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M23,11v-0.61c0-0.8-0.48-1.54-1.23-1.84c-3.65-1.48-6.81-3.93-8.48-5.37c-0.74-0.64-1.84-0.64-2.58,0 C9.03,4.62,5.88,7.06,2.23,8.55C1.48,8.85,1,9.58,1,10.39V11c0,1.49,0.93,2.75,2.24,3.26c-0.03,1.68-0.16,3.55-0.52,5.29 C2.46,20.81,3.38,22,4.67,22h14.67c1.29,0,2.21-1.19,1.95-2.45c-0.36-1.75-0.5-3.62-0.52-5.29C22.07,13.75,23,12.49,23,11z M12,4.71c1.33,1.14,3.49,2.84,6.11,4.29H5.89C8.51,7.55,10.67,5.85,12,4.71z M13,11h3c0,0.83-0.67,1.5-1.5,1.5S13,11.83,13,11z M9.5,12.5C8.67,12.5,8,11.83,8,11h3C11,11.83,10.33,12.5,9.5,12.5z M6,11c0,0.83-0.67,1.5-1.5,1.5S3,11.83,3,11H6z M4.66,20 c0.39-1.86,0.54-3.82,0.57-5.58c0.68-0.15,1.29-0.49,1.76-0.98c0.25,0.25,0.54,0.45,0.85,0.62c-0.1,1.87-0.26,4-0.52,5.93H4.66z M9.35,20c0.24-1.83,0.39-3.78,0.48-5.53c0.84-0.08,1.61-0.45,2.17-1.02c0.56,0.57,1.32,0.94,2.17,1.02 c0.1,1.75,0.24,3.7,0.48,5.53H9.35z M16.67,20c-0.27-1.94-0.43-4.07-0.52-5.93c0.31-0.17,0.61-0.37,0.85-0.62 c0.47,0.48,1.08,0.83,1.76,0.98c0.03,1.76,0.18,3.72,0.57,5.58H16.67z M19.5,12.5c-0.83,0-1.5-0.67-1.5-1.5h3 C21,11.83,20.33,12.5,19.5,12.5z",
                }
            }
        }
    }
}

pub fn fire_hydrant_alt_icons_20px(props: IconProps) -> Element {
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
                    circle {
                        r: "1.25",
                        cx: "10",
                        cy: "11.5",
                    }
                    path {
                        d: "M15.5,9H15V7h0.25C15.66,7,16,6.66,16,6.25v0c0-0.41-0.34-0.75-0.75-0.75h-0.5C14.11,3.48,12.24,2,10,2 S5.89,3.48,5.25,5.5h-0.5C4.34,5.5,4,5.84,4,6.25v0C4,6.66,4.34,7,4.75,7H5v2H4.5C3.67,9,3,9.67,3,10.5v2C3,13.33,3.67,14,4.5,14 H5v2.5H4.75C4.34,16.5,4,16.84,4,17.25l0,0C4,17.66,4.34,18,4.75,18h10.5c0.41,0,0.75-0.34,0.75-0.75l0,0 c0-0.41-0.34-0.75-0.75-0.75H15V14h0.5c0.83,0,1.5-0.67,1.5-1.5v-2C17,9.67,16.33,9,15.5,9z M10,14.25 c-1.52,0-2.75-1.23-2.75-2.75S8.48,8.75,10,8.75s2.75,1.23,2.75,2.75S11.52,14.25,10,14.25z",
                    }
                }
            }
        }
    }
}

pub fn fire_hydrant_alt_icons_24px(props: IconProps) -> Element {
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
                        d: "M19,11h-1V8h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1.35C16.83,3.67,14.61,2,12,2S7.17,3.67,6.35,6H5 C4.45,6,4,6.45,4,7v0c0,0.55,0.45,1,1,1h1v3H5c-1.1,0-2,0.9-2,2v2c0,1.1,0.9,2,2,2h1v3H5c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1 h14c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1v-3h1c1.1,0,2-0.9,2-2v-2C21,11.9,20.1,11,19,11z M12,17.5c-1.93,0-3.5-1.57-3.5-3.5 s1.57-3.5,3.5-3.5s3.5,1.57,3.5,3.5S13.93,17.5,12,17.5z",
                    }
                    circle {
                        cx: "12",
                        r: "1.5",
                        cy: "14",
                    }
                }
            }
        }
    }
}

pub fn fire_truck_icons_20px(props: IconProps) -> Element {
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
                        d: "M18.84,8.68l-1.43-2.85C17.16,5.32,16.64,5,16.07,5H15.5V3.5C15.5,3.22,15.28,3,15,3h-1.5C13.22,3,13,3.22,13,3.5V5h-1.5 C10.67,5,10,5.67,10,6.5v3H2.5C1.67,9.5,1,10.17,1,11v2.5C1,14.33,1.67,15,2.5,15h0.55c0.23,1.14,1.24,2,2.45,2 c1.21,0,2.22-0.86,2.45-2h4.1c0.23,1.14,1.24,2,2.45,2c1.21,0,2.22-0.86,2.45-2h0.55c0.83,0,1.5-0.67,1.5-1.5V9.35 C19,9.12,18.95,8.89,18.84,8.68z M5.5,15.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.05,15.5,5.5,15.5z M14.5,15.5 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S15.05,15.5,14.5,15.5z M17.5,9.5h-6v-3h4.57l1.43,2.85V9.5z",
                    }
                    path {
                        d: "M8.5,7.5H8V6h0.5C8.78,6,9,5.78,9,5.5v0C9,5.22,8.78,5,8.5,5h-7C1.22,5,1,5.22,1,5.5v0C1,5.78,1.22,6,1.5,6H2v1.5H1.5 C1.22,7.5,1,7.72,1,8v0c0,0.28,0.22,0.5,0.5,0.5h7C8.78,8.5,9,8.28,9,8v0C9,7.72,8.78,7.5,8.5,7.5z M4.5,7.5H3V6h1.5V7.5z M7,7.5 H5.5V6H7V7.5z",
                    }
                }
            }
        }
    }
}

pub fn fire_truck_icons_24px(props: IconProps) -> Element {
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
                        d: "M22.9,10.69l-1.44-4.32C21.18,5.55,20.42,5,19.56,5H19V4c0-0.55-0.45-1-1-1h-1c-0.55,0-1,0.45-1,1v1h-2c-1.1,0-2,0.9-2,2 v4H3c-1.1,0-2,0.9-2,2v3c0,1.1,0.9,2,2,2h1c0,1.66,1.34,3,3,3s3-1.34,3-3h4c0,1.66,1.34,3,3,3s3-1.34,3-3h1c1.1,0,2-0.9,2-2v-4.68 C23,11.11,22.97,10.9,22.9,10.69z M7,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S7.55,19,7,19z M17,19c-0.55,0-1-0.45-1-1 s0.45-1,1-1s1,0.45,1,1S17.55,19,17,19z M14,11V7h5.56l1.33,4H14z",
                    }
                    path {
                        d: "M10.25,8.5H10v-2h0.25C10.66,6.5,11,6.16,11,5.75v0C11,5.34,10.66,5,10.25,5h-8.5C1.34,5,1,5.34,1,5.75v0 C1,6.16,1.34,6.5,1.75,6.5H2v2H1.75C1.34,8.5,1,8.84,1,9.25v0C1,9.66,1.34,10,1.75,10h8.5C10.66,10,11,9.66,11,9.25v0 C11,8.84,10.66,8.5,10.25,8.5z M5.25,8.5H3.5v-2h1.75V8.5z M8.5,8.5H6.75v-2H8.5V8.5z",
                    }
                }
            }
        }
    }
}

pub fn flight_icons_24px(props: IconProps) -> Element {
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
                d: "M21 14.58c0-.36-.19-.69-.49-.89L13 9V3.5c0-.83-.67-1.5-1.5-1.5S10 2.67 10 3.5V9l-7.51 4.69c-.3.19-.49.53-.49.89 0 .7.68 1.21 1.36 1L10 13.5V19l-1.8 1.35c-.13.09-.2.24-.2.4v.59c0 .33.32.57.64.48L11.5 21l2.86.82c.32.09.64-.15.64-.48v-.59c0-.16-.07-.31-.2-.4L13 19v-5.5l6.64 2.08c.68.21 1.36-.3 1.36-1z",
            }
        }
    }
}

pub fn flight_class_icons_20px(props: IconProps) -> Element {
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
                d: "M13,4h-1.5C10.67,4,10,4.67,10,5.5v3.75c0,0.83,0.67,1.5,1.5,1.5H13c0.83,0,1.5-0.67,1.5-1.5V5.5C14.5,4.67,13.83,4,13,4z M5.75,4C6.16,4,6.5,4.34,6.5,4.75V7l1.9,6h5.85c0.41,0,0.75,0.34,0.75,0.75s-0.34,0.75-0.75,0.75H8.4c-0.66,0-1.24-0.43-1.43-1.06 l-1.9-6.22C5.02,7.07,5,6.92,5,6.78V4.75C5,4.34,5.34,4,5.75,4z M7.5,16.25c0-0.41,0.34-0.75,0.75-0.75h6 c0.41,0,0.75,0.34,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75h-6C7.84,17,7.5,16.66,7.5,16.25z",
            }
        }
    }
}

pub fn flight_class_icons_24px(props: IconProps) -> Element {
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
                height: "24",
                fill: "none",
                y: "0",
            }
            path {
                d: "M16,4h-2c-1.1,0-2,0.9-2,2v5c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2V6C18,4.9,17.1,4,16,4z M6,4c0.55,0,1,0.45,1,1v3l2.5,8H17 c0.55,0,1,0.45,1,1s-0.45,1-1,1H9.49c-0.88,0-1.66-0.58-1.92-1.43L5.08,8.28C5.03,8.09,5,7.9,5,7.71V5C5,4.45,5.45,4,6,4z M18,20 c0,0.55-0.45,1-1,1H9c-0.55,0-1-0.45-1-1s0.45-1,1-1h8C17.55,19,18,19.45,18,20z",
            }
        }
    }
}

pub fn forest_icons_20px(props: IconProps) -> Element {
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
                        d: "M11.82,10h0.28c0.39,0,0.63-0.43,0.42-0.77l-4.1-6.56c-0.2-0.31-0.65-0.31-0.85,0l-4.1,6.56C3.27,9.57,3.51,10,3.9,10 h0.28l-2.69,4.23C1.28,14.56,1.52,15,1.91,15H6.5v1.5C6.5,17.33,7.17,18,8,18s1.5-0.67,1.5-1.5V15h4.59 c0.39,0,0.63-0.44,0.42-0.77L11.82,10z",
                    }
                    path {
                        d: "M18.51,14.23L15.82,10h0.28c0.39,0,0.63-0.43,0.42-0.77l-4.1-6.56c-0.2-0.31-0.65-0.31-0.85,0l-0.99,1.58l2.78,4.45 c0.29,0.46,0.3,1.05,0.04,1.52c-0.04,0.07-0.08,0.14-0.13,0.2l2.08,3.27c0.25,0.39,0.29,0.87,0.13,1.31h2.6 C18.48,15,18.72,14.56,18.51,14.23z",
                    }
                    path {
                        d: "M10.5,16.5c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5V16h-3V16.5z",
                    }
                }
            }
        }
    }
}

pub fn forest_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M14.14,12h-0.06c0.81,0,1.28-0.91,0.82-1.57L9.82,3.17c-0.4-0.57-1.24-0.57-1.64,0L3.1,10.43C2.64,11.09,3.11,12,3.92,12 H3.86l-2.87,4.46C0.56,17.12,1.04,18,1.83,18H7v2c0,1.1,0.9,2,2,2s2-0.9,2-2v-2h5.17c0.79,0,1.27-0.88,0.84-1.54L14.14,12z",
                    }
                    path {
                        d: "M23.01,16.46L20.14,12h-0.06c0.81,0,1.28-0.91,0.82-1.57l-5.08-7.26c-0.4-0.57-1.24-0.57-1.64,0l-1.57,2.24l3.11,4.44 c0.43,0.61,0.48,1.41,0.14,2.07c-0.08,0.16-0.18,0.3-0.3,0.43l2.29,3.57c0.4,0.62,0.42,1.4,0.07,2.04 c-0.01,0.02-0.02,0.03-0.03,0.04h4.28C22.96,18,23.44,17.12,23.01,16.46z",
                    }
                    path {
                        d: "M13,20c0,1.1,0.9,2,2,2s2-0.9,2-2v-1h-4V20z",
                    }
                }
            }
        }
    }
}

pub fn fork_left_icons_20px(props: IconProps) -> Element {
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
                    d: "M11,5.87l0,5.48c-1.2-1.05-2.74-1.4-4.13-1.11l0.66-0.66c0.29-0.29,0.29-0.77,0-1.06c-0.29-0.29-0.77-0.29-1.06,0 l-1.76,1.76c-0.39,0.39-0.39,1.02,0,1.41l1.76,1.76c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06l-0.66-0.66 C8.74,11.32,10.44,12.28,11,14v2.25c0,0.41,0.34,0.75,0.75,0.75s0.75-0.34,0.75-0.75l0-10.38l0.66,0.66c0.29,0.29,0.77,0.29,1.06,0 c0.29-0.29,0.29-0.77,0-1.06l-1.76-1.76c-0.39-0.39-1.02-0.39-1.41,0L9.28,5.47c-0.29,0.29-0.29,0.77,0,1.06 c0.29,0.29,0.77,0.29,1.06,0L11,5.87z",
                }
            }
        }
    }
}

pub fn fork_left_icons_24px(props: IconProps) -> Element {
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
                    d: "M15,20c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1v-3c-0.73-2.58-3.07-3.47-5.17-3l0.88,0.88c0.39,0.39,0.39,1.02,0,1.41 c-0.39,0.39-1.02,0.39-1.41,0l-2.59-2.59c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59c0.39-0.39,1.02-0.39,1.41,0 c0.39,0.39,0.39,1.02,0,1.41L7.83,12c1.51-0.33,3.73,0.08,5.17,1.36l0-6.53l-0.88,0.88c-0.39,0.39-1.02,0.39-1.41,0 c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59c0.39-0.39,1.02-0.39,1.41,0l2.59,2.59c0.39,0.39,0.39,1.02,0,1.41 c-0.39,0.39-1.02,0.39-1.41,0L15,6.83V20z",
                }
            }
        }
    }
}

pub fn fork_right_icons_20px(props: IconProps) -> Element {
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
                    d: "M9,5.87l0,5.48c1.2-1.05,2.74-1.4,4.13-1.11l-0.66-0.66c-0.29-0.29-0.29-0.77,0-1.06c0.29-0.29,0.77-0.29,1.06,0l1.76,1.76 c0.39,0.39,0.39,1.02,0,1.41l-1.76,1.76c-0.29,0.29-0.77,0.29-1.06,0l0,0c-0.29-0.29-0.29-0.77,0-1.06l0.66-0.66 C11.26,11.32,9.56,12.28,9,14v2.25C9,16.66,8.66,17,8.25,17S7.5,16.66,7.5,16.25l0-10.38L6.84,6.53c-0.29,0.29-0.77,0.29-1.06,0 c-0.29-0.29-0.29-0.77,0-1.06l1.76-1.76c0.39-0.39,1.02-0.39,1.41,0l1.76,1.76c0.29,0.29,0.29,0.77,0,1.06 c-0.29,0.29-0.77,0.29-1.06,0L9,5.87z",
                }
            }
        }
    }
}

pub fn fork_right_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M9,20c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-3c0.73-2.58,3.07-3.47,5.17-3l-0.88,0.88c-0.39,0.39-0.39,1.02,0,1.41 c0.39,0.39,1.02,0.39,1.41,0l2.59-2.59c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59c-0.39-0.39-1.02-0.39-1.41,0 c-0.39,0.39-0.39,1.02,0,1.41L16.17,12c-1.51-0.33-3.73,0.08-5.17,1.36l0-6.53l0.88,0.88c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59c-0.39-0.39-1.02-0.39-1.41,0L6.71,6.29c-0.39,0.39-0.39,1.02,0,1.41 c0.39,0.39,1.02,0.39,1.41,0L9,6.83V20z",
                }
            }
        }
    }
}

pub fn fort_icons_20px(props: IconProps) -> Element {
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
                        d: "M17.5,3.75V4.5h-1.75V3.75C15.75,3.34,15.41,3,15,3h0c-0.41,0-0.75,0.34-0.75,0.75V4.5H12.5V3.75 C12.5,3.34,12.16,3,11.75,3h0C11.34,3,11,3.34,11,3.75v1.84c0,0.27,0.11,0.52,0.29,0.71L13,8v1H7V8l1.71-1.71 C8.89,6.11,9,5.85,9,5.59V3.75C9,3.34,8.66,3,8.25,3h0C7.84,3,7.5,3.34,7.5,3.75V4.5H5.75V3.75C5.75,3.34,5.41,3,5,3h0 C4.59,3,4.25,3.34,4.25,3.75V4.5H2.5V3.75C2.5,3.34,2.16,3,1.75,3h0C1.34,3,1,3.34,1,3.75v1.84c0,0.27,0.11,0.52,0.29,0.71L3,8v4 l-1.71,1.71C1.11,13.89,1,14.15,1,14.41V16c0,0.55,0.45,1,1,1h6l0-1.89c0-1,0.68-1.92,1.66-2.08C10.92,12.82,12,13.79,12,15v2h6 c0.55,0,1-0.45,1-1v-1.59c0-0.27-0.11-0.52-0.29-0.71L17,12V8l1.71-1.71C18.89,6.11,19,5.85,19,5.59V3.75C19,3.34,18.66,3,18.25,3 l0,0C17.84,3,17.5,3.34,17.5,3.75z",
                    }
                }
            }
        }
    }
}

pub fn fort_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M21,4v1h-2V4c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v1h-2V4c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2.17 c0,0.53,0.21,1.04,0.59,1.41L15,9v1H9V9l1.41-1.41C10.79,7.21,11,6.7,11,6.17V4c0-0.55-0.45-1-1-1h0C9.45,3,9,3.45,9,4v1H7V4 c0-0.55-0.45-1-1-1h0C5.45,3,5,3.45,5,4v1H3V4c0-0.55-0.45-1-1-1h0C1.45,3,1,3.45,1,4v2.17C1,6.7,1.21,7.21,1.59,7.59L3,9v6 l-1.41,1.41C1.21,16.79,1,17.3,1,17.83V19c0,1.1,0.9,2,2,2h7l0-2.89c0-1,0.68-1.92,1.66-2.08C12.92,15.82,14,16.79,14,18v3h7 c1.1,0,2-0.9,2-2v-1.17c0-0.53-0.21-1.04-0.59-1.41L21,15V9l1.41-1.41C22.79,7.21,23,6.7,23,6.17V4c0-0.55-0.45-1-1-1h0 C21.45,3,21,3.45,21,4z",
                    }
                }
            }
        }
    }
}

pub fn hail_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M12,6c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2S13.1,6,12,6z M17.95,2L17.95,2c0.59,0,1.06,0.51,1,1.09 C18.93,3.24,18.74,7.15,15,8.4V21c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-5h-2v5c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V10.1 c-0.3,0.1-0.5,0.2-0.6,0.3c-0.46,0.36-1.17,0.87-1.36,2.67C6.99,13.59,6.57,14,6.04,14h0c-0.58,0-1.05-0.49-1-1.07 c0.13-1.6,0.62-2.98,2.07-4.22C8.21,7.81,10,7,12,7s2.68-0.46,3.48-1.06c0.43-0.34,1.28-0.99,1.48-3.02C17.01,2.4,17.43,2,17.95,2z M5,16h1c0.55,0,1,0.45,1,1v4c0,0.55-0.45,1-1,1H5c-0.55,0-1-0.45-1-1v-4C4,16.45,4.45,16,5,16z",
                }
            }
        }
    }
}

pub fn handyman_icons_20px(props: IconProps) -> Element {
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
                        d: "M16.71,14.59l-3.24-3.24c-0.19-0.19-0.44-0.29-0.71-0.29h-1l-0.34-0.34l-0.71,0.71l0.34,0.34v1 c0,0.27,0.11,0.52,0.29,0.71l3.24,3.24c0.39,0.39,1.02,0.39,1.41,0L16.71,16C17.1,15.61,17.1,14.98,16.71,14.59z",
                    }
                    path {
                        d: "M12.57,8.14L12.57,8.14l0.53,0.53c0.2,0.2,0.51,0.2,0.71,0l0.71-0.71l1.41,1.41c0.78-0.78,0.78-2.05,0-2.83l-2.12-2.12 c-0.2-0.2-0.51-0.2-0.71,0l-0.39,0.39l0-1.27c0-0.45-0.54-0.67-0.85-0.35L10.1,4.95C9.79,5.26,10.01,5.8,10.45,5.8l1.27,0 l-0.39,0.39c-0.2,0.2-0.2,0.51,0,0.71l0.53,0.53L10,9.29L7.51,6.81l0.12-1.02c0.02-0.15-0.03-0.3-0.14-0.41L5.71,3.59 c-0.2-0.2-0.51-0.2-0.71,0L3.59,5c-0.2,0.2-0.2,0.51,0,0.71l1.78,1.78C5.48,7.6,5.63,7.65,5.78,7.64l1.02-0.12L9.29,10l-1.05,1.05 h-1c-0.27,0-0.52,0.11-0.71,0.29l-3.24,3.24c-0.39,0.39-0.39,1.02,0,1.41L4,16.71c0.39,0.39,1.02,0.39,1.41,0l3.24-3.24 c0.19-0.19,0.29-0.44,0.29-0.71v-1L12.57,8.14z",
                    }
                }
            }
        }
    }
}

pub fn handyman_icons_24px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M21.67,18.17l-4.72-4.72c-0.48-0.48-0.99-0.59-1.58-0.59l-2.54,2.54c0,0.59,0.11,1.11,0.59,1.58l4.72,4.72 c0.39,0.39,1.02,0.39,1.41,0l2.12-2.12C22.06,19.2,22.06,18.56,21.67,18.17z",
                        }
                    }
                    g {
                        path {
                            d: "M16.63,9.49c0.39,0.39,1.02,0.39,1.41,0l0.71-0.71l2.12,2.12c1.17-1.17,1.17-3.07,0-4.24l-2.83-2.83 c-0.39-0.39-1.02-0.39-1.41,0l-0.71,0.71V2c0-0.62-0.76-0.95-1.21-0.5l-2.54,2.54c-0.45,0.45-0.12,1.21,0.5,1.21h2.54l-0.71,0.71 c-0.39,0.39-0.39,1.02,0,1.41l0.35,0.35l-2.89,2.89L7.85,6.48v-1c0-0.27-0.11-0.52-0.29-0.71L5.54,2.74 c-0.39-0.39-1.02-0.39-1.41,0L2.71,4.16c-0.39,0.39-0.39,1.02,0,1.41L4.73,7.6c0.19,0.19,0.44,0.29,0.71,0.29h1l4.13,4.13 l-0.85,0.85H8.42c-0.53,0-1.04,0.21-1.41,0.59l-4.72,4.72c-0.39,0.39-0.39,1.02,0,1.41l2.12,2.12c0.39,0.39,1.02,0.39,1.41,0 l4.72-4.72c0.38-0.38,0.59-0.88,0.59-1.41v-1.29l5.15-5.15L16.63,9.49z",
                        }
                    }
                }
            }
        }
    }
}

pub fn hardware_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M8.5,17H11c0.55,0,1-0.45,1-1v-5.25H7.5V16C7.5,16.55,7.95,17,8.5,17z",
                    }
                    path {
                        d: "M14.94,3c-0.28,0-0.55,0.11-0.75,0.31L12,5.5v-1C12,3.67,11.33,3,10.5,3H8C5.79,3,4,4.79,4,7h3.5v2.25H12V6.5l2.19,2.19 C14.39,8.89,14.66,9,14.94,9h0C15.53,9,16,8.53,16,7.94V4.06C16,3.47,15.53,3,14.94,3L14.94,3z",
                    }
                }
            }
        }
    }
}

pub fn hardware_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                g {
                    g {
                        g {
                            path {
                                d: "M17.59,3.41L15,6V5c0-1.1-0.9-2-2-2H9C6.24,3,4,5.24,4,8h5v3h6V8l2.59,2.59c0.26,0.26,0.62,0.41,1,0.41h0.01 C19.37,11,20,10.37,20,9.59V4.41C20,3.63,19.37,3,18.59,3h-0.01C18.21,3,17.85,3.15,17.59,3.41z",
                            }
                        }
                        g {
                            path {
                                d: "M9,13v7c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-7H9z",
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn home_repair_service_icons_20px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M14,12.5c0,0.28-0.22,0.5-0.5,0.5S13,12.78,13,12.5V12H7v0.5C7,12.78,6.78,13,6.5,13S6,12.78,6,12.5V12H3v3 c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1v-3h-3V12.5z",
                        }
                    }
                    path {
                        d: "M16,7h-3V5c0-0.55-0.45-1-1-1H8C7.45,4,7,4.45,7,5v2H4C3.45,7,3,7.45,3,8v3h3v-0.5C6,10.22,6.22,10,6.5,10S7,10.22,7,10.5 V11h6v-0.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5V11h3V8C17,7.45,16.55,7,16,7z M12,7H8V5h4V7z",
                    }
                }
            }
        }
    }
}

pub fn home_repair_service_icons_24px(props: IconProps) -> Element {
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
                g {
                    g {
                        path {
                            d: "M17,16c-0.55,0-1-0.45-1-1H8c0,0.55-0.45,1-1,1s-1-0.45-1-1H2v3c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-3h-4 C18,15.55,17.55,16,17,16z",
                        }
                    }
                    path {
                        d: "M20,8h-3V6c0-1.1-0.9-2-2-2H9C7.9,4,7,4.9,7,6v2H4c-1.1,0-2,0.9-2,2v4h4v-1c0-0.55,0.45-1,1-1s1,0.45,1,1v1h8v-1 c0-0.55,0.45-1,1-1s1,0.45,1,1v1h4v-4C22,8.9,21.1,8,20,8z M15,8H9V6h6V8z",
                    }
                }
            }
        }
    }
}

pub fn hotel_icons_24px(props: IconProps) -> Element {
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
                d: "M7 13c1.66 0 3-1.34 3-3S8.66 7 7 7s-3 1.34-3 3 1.34 3 3 3zm12-6h-6c-1.1 0-2 .9-2 2v5H3V6c0-.55-.45-1-1-1s-1 .45-1 1v13c0 .55.45 1 1 1s1-.45 1-1v-2h18v2c0 .55.45 1 1 1s1-.45 1-1v-8c0-2.21-1.79-4-4-4z",
            }
        }
    }
}

pub fn hvac_icons_20px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M7.42,11.5h5.17c0.22-0.38,0.36-0.8,0.39-1.26H7.02C7.06,10.7,7.2,11.12,7.42,11.5z",
                        }
                    }
                    g {
                        path {
                            d: "M10,13c0.88,0,1.67-0.39,2.22-1H7.78C8.33,12.61,9.12,13,10,13z",
                        }
                    }
                    g {
                        path {
                            d: "M10,7C9.12,7,8.34,7.39,7.79,7.99h4.42C11.66,7.39,10.88,7,10,7z",
                        }
                    }
                    g {
                        path {
                            d: "M7.42,8.49c-0.22,0.38-0.36,0.8-0.4,1.26h5.95c-0.04-0.46-0.18-0.88-0.4-1.26H7.42z",
                        }
                    }
                    g {
                        path {
                            d: "M15,4H5C4.45,4,4,4.45,4,5v10c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V5C16,4.45,15.55,4,15,4z M10,14 c-2.21,0-4-1.79-4-4c0-2.21,1.79-4,4-4c2.21,0,4,1.79,4,4C14,12.21,12.21,14,10,14z",
                        }
                    }
                }
            }
        }
    }
}

pub fn hvac_icons_24px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M12,16c1.01,0,1.91-0.39,2.62-1H9.38C10.09,15.61,10.99,16,12,16z",
                        }
                    }
                    g {
                        path {
                            d: "M8.56,14h6.89c0.26-0.45,0.44-0.96,0.51-1.5h-7.9C8.12,13.04,8.29,13.55,8.56,14z",
                        }
                    }
                    g {
                        path {
                            d: "M12,8c-1.01,0-1.91,0.39-2.62,1h5.24C13.91,8.39,13.01,8,12,8z",
                        }
                    }
                    g {
                        path {
                            d: "M8.56,10c-0.26,0.45-0.44,0.96-0.51,1.5h7.9c-0.07-0.54-0.24-1.05-0.51-1.5H8.56z",
                        }
                    }
                    g {
                        path {
                            d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12,18c-3.31,0-6-2.69-6-6 s2.69-6,6-6s6,2.69,6,6S15.31,18,12,18z",
                        }
                    }
                }
            }
        }
    }
}

pub fn icecream_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M14.92,5.16C14.51,2.8,12.47,1,10,1S5.49,2.8,5.08,5.16C3.88,5.55,3,6.66,3,8c0,1.66,1.34,3,3,3c0.02,0,0.03,0,0.04,0 l3.06,6.19c0.37,0.74,1.43,0.74,1.79,0L13.96,11c0.01,0,0.03,0,0.04,0c1.66,0,3-1.34,3-3C17,6.66,16.12,5.55,14.92,5.16z M10,15.61l-2.49-5.04c0.16-0.1,0.07-0.08,0.37-0.08C8.53,10.81,9.24,11,10,11s1.47-0.19,2.12-0.5c0.3,0,0.21-0.02,0.37,0.08 L10,15.61z",
                    }
                }
            }
        }
    }
}

pub fn icecream_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18.38,6.24C17.79,3.24,15.14,1,12,1S6.21,3.24,5.62,6.24C4.08,6.81,3,8.29,3,10c0,2.21,1.79,4,4,4 c0.12,0,0.23-0.02,0.34-0.02l3.83,7.31c0.38,0.72,1.41,0.71,1.78-0.01l3.73-7.31C16.79,13.98,16.89,14,17,14c2.21,0,4-1.79,4-4 C21,8.29,19.92,6.81,18.38,6.24z M12.05,18.63l-2.73-5.21C10.15,13.79,11.06,14,12,14c0.95,0,1.88-0.21,2.72-0.6L12.05,18.63z",
                    }
                }
            }
        }
    }
}

pub fn kebab_dining_icons_20px(props: IconProps) -> Element {
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
                d: "M6.5,12H7c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2H6.5v2.5C6.5,18.78,6.28,19,6,19s-0.5-0.22-0.5-0.5V16H5c-1.1,0-2-0.9-2-2 c0-1.1,0.9-2,2-2h0.5v-1h-2C3.22,11,3,10.78,3,10.5v-3C3,7.22,3.22,7,3.5,7h2V6H5C3.9,6,3,5.1,3,4c0-1.1,0.9-2,2-2h0.5V1.5 C5.5,1.22,5.72,1,6,1s0.5,0.22,0.5,0.5V2H7c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2H6.5v1h2C8.78,7,9,7.22,9,7.5v3C9,10.78,8.78,11,8.5,11 h-2V12z M14.5,11h2c0.28,0,0.5-0.22,0.5-0.5v-3C17,7.22,16.78,7,16.5,7h-2V6H15c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2h-0.5V1.5 C14.5,1.22,14.28,1,14,1s-0.5,0.22-0.5,0.5V2H13c-1.1,0-2,0.9-2,2c0,1.1,0.9,2,2,2h0.5v1h-2C11.22,7,11,7.22,11,7.5v3 c0,0.28,0.22,0.5,0.5,0.5h2v1H13c-1.1,0-2,0.9-2,2c0,1.1,0.9,2,2,2h0.5v2.5c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5V16H15 c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2h-0.5V11z",
            }
        }
    }
}

pub fn kebab_dining_icons_24px(props: IconProps) -> Element {
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
                d: "M7.75,13v1H8.5c1.38,0,2.5,1.12,2.5,2.5c0,1.38-1.12,2.5-2.5,2.5H7.75v3.25C7.75,22.66,7.41,23,7,23s-0.75-0.34-0.75-0.75 V19H5.5C4.12,19,3,17.88,3,16.5C3,15.12,4.12,14,5.5,14h0.75v-1H4c-0.55,0-1-0.45-1-1V9c0-0.55,0.45-1,1-1h2.25V7H5.5 C4.12,7,3,5.88,3,4.5C3,3.12,4.12,2,5.5,2h0.75V1.75C6.25,1.34,6.59,1,7,1s0.75,0.34,0.75,0.75V2H8.5C9.88,2,11,3.12,11,4.5 C11,5.88,9.88,7,8.5,7H7.75v1H10c0.55,0,1,0.45,1,1v3c0,0.55-0.45,1-1,1H7.75z M17.75,13v1h0.75c1.38,0,2.5,1.12,2.5,2.5 c0,1.38-1.12,2.5-2.5,2.5h-0.75v3.25c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75V19H15.5c-1.38,0-2.5-1.12-2.5-2.5 c0-1.38,1.12-2.5,2.5-2.5h0.75v-1H14c-0.55,0-1-0.45-1-1V9c0-0.55,0.45-1,1-1h2.25V7H15.5C14.12,7,13,5.88,13,4.5 C13,3.12,14.12,2,15.5,2h0.75V1.75C16.25,1.34,16.59,1,17,1s0.75,0.34,0.75,0.75V2h0.75C19.88,2,21,3.12,21,4.5 C21,5.88,19.88,7,18.5,7h-0.75v1H20c0.55,0,1,0.45,1,1v3c0,0.55-0.45,1-1,1H17.75z",
            }
        }
    }
}

pub fn layers_icons_24px(props: IconProps) -> Element {
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
                d: "M12.6 18.06c-.36.28-.87.28-1.23 0l-6.15-4.78c-.36-.28-.86-.28-1.22 0-.51.4-.51 1.17 0 1.57l6.76 5.26c.72.56 1.73.56 2.46 0l6.76-5.26c.51-.4.51-1.17 0-1.57l-.01-.01c-.36-.28-.86-.28-1.22 0l-6.15 4.79zm.63-3.02l6.76-5.26c.51-.4.51-1.18 0-1.58l-6.76-5.26c-.72-.56-1.73-.56-2.46 0L4.01 8.21c-.51.4-.51 1.18 0 1.58l6.76 5.26c.72.56 1.74.56 2.46-.01z",
            }
        }
    }
}

pub fn layers_clear_icons_24px(props: IconProps) -> Element {
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
                d: "M19.99 9.79c.51-.4.51-1.18 0-1.58l-6.76-5.26c-.72-.56-1.73-.56-2.46 0L9.41 4.02l7.88 7.88 2.7-2.11zm0 3.49l-.01-.01c-.36-.28-.86-.28-1.22 0l-.05.04 1.4 1.4c.37-.41.34-1.07-.12-1.43zm1.45 5.6L4.12 1.56c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l3.52 3.52-2.22 1.72c-.51.4-.51 1.18 0 1.58l6.76 5.26c.72.56 1.73.56 2.46 0l.87-.68 1.42 1.42-2.92 2.27c-.36.28-.87.28-1.23 0l-6.15-4.78c-.36-.28-.86-.28-1.22 0-.51.4-.51 1.17 0 1.57l6.76 5.26c.72.56 1.73.56 2.46 0l3.72-2.89 3.07 3.07c.39.39 1.02.39 1.41 0 .41-.39.41-1.02.02-1.41z",
            }
        }
    }
}

pub fn liquor_icons_20px(props: IconProps) -> Element {
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
                    y: "0",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M3,11.5c0,1.12,0.74,2.05,1.75,2.37v2.63h-1C3.34,16.5,3,16.84,3,17.25l0,0C3,17.66,3.34,18,3.75,18h3.5 C7.66,18,8,17.66,8,17.25l0,0c0-0.41-0.34-0.75-0.75-0.75h-1v-2.63C7.26,13.55,8,12.62,8,11.5V6c0-0.55-0.45-1-1-1H4 C3.45,5,3,5.45,3,6V11.5z M6.5,6.5V9h-2V6.5H6.5z",
                        }
                    }
                    g {
                        path {
                            d: "M17.06,7.62l-0.61-0.25C15.87,7.15,15.5,6.6,15.5,5.98V3c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v2.98 c0,0.61-0.37,1.16-0.94,1.39L9.94,7.62C9.37,7.85,9,8.4,9,9.02v7.48c0,0.83,0.67,1.5,1.5,1.5h6c0.83,0,1.5-0.67,1.5-1.5V9.02 C18,8.4,17.63,7.85,17.06,7.62z M14,3.5v1h-1v-1H14z M11.11,8.77C12.25,8.31,12.99,7.23,13,6h1c0.01,1.23,0.74,2.31,1.88,2.77 l0.61,0.25V10h-6V9.02L11.11,8.77z M10.5,16.5V15h6v1.5H10.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn liquor_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        g {
                            path {
                                d: "M3,14c0,1.3,0.84,2.4,2,2.82V20H4c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H7 v-3.18C8.16,16.4,9,15.3,9,14V7c0-0.55-0.45-1-1-1H4C3.45,6,3,6.45,3,7V14z M5,8h2v3H5V8z",
                            }
                        }
                        g {
                            path {
                                d: "M20.64,8.54l-0.96-0.32C19.27,8.08,19,7.7,19,7.27V3c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v4.28 c0,0.43-0.27,0.81-0.68,0.95l-0.96,0.32C11.55,8.83,11,9.59,11,10.45V20c0,1.1,0.9,2,2,2h7c1.1,0,2-0.9,2-2v-9.56 C22,9.58,21.45,8.82,20.64,8.54z M16,4h1v1h-1V4z M13,10.44l0.95-0.32C15.18,9.72,16,8.57,16,7.28V7h1v0.28 c0,1.29,0.82,2.44,2.05,2.85L20,10.44V12h-7V10.44z M20,20h-7v-2h7V20z",
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn local_activity_icons_24px(props: IconProps) -> Element {
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
                d: "M20 12c0-.76.43-1.42 1.06-1.76.6-.33.94-1.01.94-1.7V6c0-1.1-.9-2-2-2H4c-1.1 0-1.99.89-1.99 1.99v2.55c0 .69.33 1.37.94 1.69C3.58 10.58 4 11.24 4 12s-.43 1.43-1.06 1.76c-.6.33-.94 1.01-.94 1.7V18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-2.54c0-.69-.34-1.37-.94-1.7-.63-.34-1.06-1-1.06-1.76zm-5.5 4.1L12 14.5l-2.5 1.61c-.38.24-.87-.11-.75-.55l.75-2.88-2.3-1.88c-.35-.29-.17-.86.29-.89l2.96-.17 1.08-2.75c.17-.42.77-.42.93 0l1.08 2.76 2.96.17c.45.03.64.6.29.89l-2.3 1.88.76 2.86c.12.45-.37.8-.75.55z",
            }
        }
    }
}

pub fn local_airport_icons_20px(props: IconProps) -> Element {
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
                    x: "0",
                    fill: "none",
                    height: "20",
                    width: "20",
                }
                path {
                    d: "M16.77,11.85l-1.4-0.93L11,8V4c0-0.55-0.45-1-1-1S9,3.45,9,4v4l-4.22,2.81h0l-1.55,1.04C3.09,11.94,3,12.1,3,12.28 c0,0.36,0.35,0.61,0.68,0.49L9,11v4l-1.25,0.83C7.6,15.94,7.5,16.11,7.5,16.3c0,0.36,0.33,0.63,0.68,0.56L10,16.5l0,0l0,0 l1.82,0.36c0.35,0.07,0.68-0.2,0.68-0.56c0-0.19-0.1-0.37-0.25-0.47L11,15v-4l5.32,1.77c0.34,0.11,0.68-0.14,0.68-0.49 C17,12.1,16.91,11.94,16.77,11.85z",
                }
            }
        }
    }
}

pub fn local_airport_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M21.48,13.7L13.5,9V3.5C13.5,2.67,12.83,2,12,2c-0.83,0-1.5,0.67-1.5,1.5V9l-7.98,4.7C2.2,13.88,2,14.23,2,14.6 c0,0.7,0.67,1.2,1.34,1.01l7.16-2.1V19l-2.26,1.35C8.09,20.44,8,20.61,8,20.78l0,0.5h0v0.08c0,0.33,0.31,0.57,0.62,0.49l2.92-0.73 L12,21l0.38,0.09c0,0,0,0,0,0l0.42,0.11l1.9,0.48l0,0l0.67,0.17c0.32,0.08,0.62-0.16,0.62-0.49v-0.37c0,0,0,0,0,0v-0.21 c0-0.18-0.09-0.34-0.24-0.43L13.5,19v-5.5l7.16,2.1C21.33,15.8,22,15.3,22,14.6C22,14.23,21.8,13.88,21.48,13.7z",
                }
                path {
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
        }
    }
}

pub fn local_atm_icons_24px(props: IconProps) -> Element {
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
                d: "M20 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zm0 13c0 .55-.45 1-1 1H5c-.55 0-1-.45-1-1V7c0-.55.45-1 1-1h14c.55 0 1 .45 1 1v10zm-6-7c.55 0 1-.45 1-1s-.45-1-1-1h-1v-.01c0-.55-.45-1-1-1s-1 .45-1 1V8h-1c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1h3v1h-3c-.55 0-1 .45-1 1s.45 1 1 1h1c0 .55.45 1 1 1s1-.45 1-1h1c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1h-3v-1h3z",
            }
        }
    }
}

pub fn local_bar_icons_24px(props: IconProps) -> Element {
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
                d: "M21 4.45c0-.8-.65-1.45-1.45-1.45H4.45C3.65 3 3 3.65 3 4.45c0 .35.13.7.37.96L11 14v5H7c-.55 0-1 .45-1 1s.45 1 1 1h10c.55 0 1-.45 1-1s-.45-1-1-1h-4v-5l7.63-8.59c.24-.26.37-.61.37-.96zM7.43 7L5.66 5h12.69l-1.78 2H7.43z",
            }
        }
    }
}

pub fn local_cafe_icons_24px(props: IconProps) -> Element {
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
                d: "M20 3H6c-1.1 0-2 .9-2 2v8c0 2.21 1.79 4 4 4h6c2.21 0 4-1.79 4-4v-3h2c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 5h-2V5h2v3zM3 21h16c.55 0 1-.45 1-1s-.45-1-1-1H3c-.55 0-1 .45-1 1s.45 1 1 1z",
            }
        }
    }
}

pub fn local_car_wash_icons_24px(props: IconProps) -> Element {
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
                d: "M17 5c.83 0 1.5-.67 1.5-1.5 0-.66-.66-1.64-1.11-2.22-.2-.26-.59-.26-.79 0-.44.58-1.1 1.56-1.1 2.22 0 .83.67 1.5 1.5 1.5zm-5 0c.83 0 1.5-.67 1.5-1.5 0-.66-.66-1.64-1.11-2.22-.2-.26-.59-.26-.79 0-.44.58-1.1 1.56-1.1 2.22 0 .83.67 1.5 1.5 1.5zM7 5c.83 0 1.5-.67 1.5-1.5 0-.66-.66-1.64-1.11-2.22-.2-.26-.59-.26-.79 0-.44.58-1.1 1.56-1.1 2.22C5.5 4.33 6.17 5 7 5zm11.92 3.01C18.72 7.42 18.16 7 17.5 7h-11c-.66 0-1.21.42-1.42 1.01l-1.97 5.67c-.07.21-.11.43-.11.66v7.16c0 .83.67 1.5 1.5 1.5S6 22.33 6 21.5V21h12v.5c0 .82.67 1.5 1.5 1.5.82 0 1.5-.67 1.5-1.5v-7.16c0-.22-.04-.45-.11-.66l-1.97-5.67zM6.5 18c-.83 0-1.5-.67-1.5-1.5S5.67 15 6.5 15s1.5.67 1.5 1.5S7.33 18 6.5 18zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 13l1.27-3.82c.14-.4.52-.68.95-.68h9.56c.43 0 .81.28.95.68L19 13H5z",
            }
        }
    }
}

pub fn local_convenience_store_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                }
            }
            g {
                path {
                    d: "M21.9,7.89l-1.05-3.37c-0.22-0.9-1-1.52-1.91-1.52H5.05c-0.9,0-1.69,0.63-1.9,1.52L2.1,7.89C1.64,9.86,2.95,11,3,11.06V19 c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v-7.94C22.12,9.94,22.09,8.65,21.9,7.89z M13,5h1.96l0.54,3.52C15.59,9.23,15.11,10,14.22,10 C13.55,10,13,9.41,13,8.69V5z M6.44,8.86C6.36,9.51,5.84,10,5.23,10C4.3,10,3.88,9.03,4.04,8.36L5.05,5h1.97L6.44,8.86z M10.5,16H9 v1h1.5c0.28,0,0.5,0.22,0.5,0.5l0,0c0,0.28-0.22,0.5-0.5,0.5h-2C8.22,18,8,17.78,8,17.5v-2C8,15.22,8.22,15,8.5,15H10v-1H8.5 C8.22,14,8,13.78,8,13.5v0C8,13.22,8.22,13,8.5,13h2c0.28,0,0.5,0.22,0.5,0.5v2C11,15.78,10.78,16,10.5,16z M11,8.69 C11,9.41,10.45,10,9.71,10c-0.75,0-1.3-0.7-1.22-1.48L9.04,5H11V8.69z M15.5,18L15.5,18c-0.28,0-0.5-0.22-0.5-0.5V16h-1.5 c-0.28,0-0.5-0.22-0.5-0.5v-2c0-0.28,0.22-0.5,0.5-0.5h0c0.28,0,0.5,0.22,0.5,0.5V15h1v-1.5c0-0.28,0.22-0.5,0.5-0.5h0 c0.28,0,0.5,0.22,0.5,0.5v4C16,17.78,15.78,18,15.5,18z M18.77,10c-0.61,0-1.14-0.49-1.21-1.14L16.98,5l1.93-0.01l1.05,3.37 C20.12,9.03,19.71,10,18.77,10z",
                }
            }
        }
    }
}

pub fn local_dining_icons_24px(props: IconProps) -> Element {
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
                d: "M8.1 13.34l2.83-2.83-6.19-6.18c-.48-.48-1.31-.35-1.61.27-.71 1.49-.45 3.32.78 4.56l4.19 4.18zm6.78-1.81c1.53.71 3.68.21 5.27-1.38 1.91-1.91 2.28-4.65.81-6.12-1.46-1.46-4.2-1.1-6.12.81-1.59 1.59-2.09 3.74-1.38 5.27l-9.05 9.05c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 14.41l6.18 6.18c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 13l1.47-1.47z",
            }
        }
    }
}

pub fn local_drink_icons_24px(props: IconProps) -> Element {
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
                d: "M5.23 2C4.04 2 3.11 3.04 3.24 4.22l1.77 16.01C5.13 21.23 5.97 22 7 22h10c1.03 0 1.87-.77 1.99-1.77l1.77-16.01c.13-1.18-.8-2.22-1.99-2.22H5.23zM12 19c-1.66 0-3-1.34-3-3 0-1.55 1.81-3.95 2.62-4.94.2-.25.57-.25.77 0 .81 1 2.62 3.39 2.62 4.94-.01 1.66-1.35 3-3.01 3zm6.33-11H5.67l-.32-2.89c-.06-.59.4-1.11 1-1.11h11.3c.59 0 1.06.52.99 1.11L18.33 8z",
            }
        }
    }
}

pub fn local_fire_department_icons_20px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M11.67,12.5L10,10.63L8.33,12.5c-0.37,0.42-0.58,0.95-0.58,1.5c0,1.24,1.01,2.25,2.25,2.25s2.25-1.01,2.25-2.25 C12.25,13.45,12.04,12.92,11.67,12.5L11.67,12.5z",
                        }
                    }
                    g {
                        path {
                            d: "M12.67,5.41L12.67,5.41C11.79,6.52,10,5.89,10,4.47V3.11c0-0.47-0.54-0.74-0.91-0.46C7.41,3.94,4,7.04,4,11 c0,2.2,1.18,4.12,2.95,5.16c-0.44-0.61-0.7-1.36-0.7-2.16c0-0.92,0.34-1.81,0.96-2.5l2.36-2.65c0.23-0.25,0.62-0.25,0.85,0 l2.29,2.56c0.46,0.52,0.82,1.14,0.96,1.83c0.21,1.08-0.04,2.09-0.59,2.89c1.43-0.86,2.5-2.3,2.81-3.99 c0.46-2.47-0.62-5.46-2.46-6.85C13.2,5.11,12.86,5.18,12.67,5.41z",
                        }
                    }
                }
            }
        }
    }
}

pub fn local_fire_department_icons_24px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M12,12.9l-2.03,2c-0.46,0.46-0.82,1.03-0.93,1.67C8.74,18.41,10.18,20,12,20s3.26-1.59,2.96-3.42 c-0.11-0.64-0.46-1.22-0.93-1.67L12,12.9z",
                        }
                    }
                    g {
                        path {
                            d: "M15.56,6.55L15.56,6.55C14.38,8.02,12,7.19,12,5.3V3.77c0-0.8-0.89-1.28-1.55-0.84C8.12,4.49,4,7.97,4,13 c0,2.92,1.56,5.47,3.89,6.86c-0.71-1.02-1.06-2.31-0.81-3.68c0.19-1.04,0.75-1.98,1.51-2.72l2.71-2.67c0.39-0.38,1.01-0.38,1.4,0 l2.73,2.69c0.74,0.73,1.3,1.65,1.48,2.68c0.25,1.36-0.07,2.64-0.77,3.66c1.89-1.15,3.29-3.06,3.71-5.3 c0.61-3.27-0.81-6.37-3.22-8.1C16.3,6.17,15.83,6.22,15.56,6.55z",
                        }
                    }
                }
            }
        }
    }
}

pub fn local_florist_icons_24px(props: IconProps) -> Element {
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
                d: "M12 22c4.56 0 8.33-3.4 8.92-7.8.09-.64-.48-1.21-1.12-1.12-4.4.59-7.8 4.36-7.8 8.92zM5.6 10.25c0 1.38 1.12 2.5 2.5 2.5.53 0 1.01-.16 1.42-.44l-.02.19c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5l-.02-.19c.4.28.89.44 1.42.44 1.38 0 2.5-1.12 2.5-2.5 0-1-.59-1.85-1.43-2.25.84-.4 1.43-1.25 1.43-2.25 0-1.38-1.12-2.5-2.5-2.5-.53 0-1.01.16-1.42.44l.02-.19C14.5 2.12 13.38 1 12 1S9.5 2.12 9.5 3.5l.02.19c-.4-.28-.89-.44-1.42-.44-1.38 0-2.5 1.12-2.5 2.5 0 1 .59 1.85 1.43 2.25-.84.4-1.43 1.25-1.43 2.25zM12 5.5c1.38 0 2.5 1.12 2.5 2.5s-1.12 2.5-2.5 2.5S9.5 9.38 9.5 8s1.12-2.5 2.5-2.5zm-8.92 8.7C3.67 18.6 7.44 22 12 22c0-4.56-3.4-8.33-7.8-8.92-.64-.09-1.21.48-1.12 1.12z",
            }
        }
    }
}

pub fn local_gas_station_icons_24px(props: IconProps) -> Element {
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
                d: "m19.77 7.23.01-.01-3.19-3.19c-.29-.29-.77-.29-1.06 0-.29.29-.29.77 0 1.06l1.58 1.58c-1.05.4-1.76 1.47-1.58 2.71.16 1.1 1.1 1.99 2.2 2.11.47.05.88-.03 1.27-.2v7.21c0 .55-.45 1-1 1s-1-.45-1-1V14c0-1.1-.9-2-2-2h-1V5c0-1.1-.9-2-2-2H6c-1.1 0-2 .9-2 2v15c0 .55.45 1 1 1h8c.55 0 1-.45 1-1v-6.5h1.5v4.86c0 1.31.94 2.5 2.24 2.63 1.5.15 2.76-1.02 2.76-2.49V9c0-.69-.28-1.32-.73-1.77zM12 10H6V6c0-.55.45-1 1-1h4c.55 0 1 .45 1 1v4zm6 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

pub fn local_grocery_store_icons_24px(props: IconProps) -> Element {
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
                d: "M7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zM2 4h1l3.6 7.59-1.35 2.44C4.52 15.37 5.48 17 7 17h11c.55 0 1-.45 1-1s-.45-1-1-1H7l1.1-2h7.45c.75 0 1.41-.41 1.75-1.03l3.58-6.49c.37-.66-.11-1.48-.87-1.48H5.21l-.67-1.43c-.16-.35-.52-.57-.9-.57H2c-.55 0-1 .45-1 1s.45 1 1 1zm15 14c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

pub fn local_hospital_icons_24px(props: IconProps) -> Element {
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
                d: "M19 3H5c-1.1 0-1.99.9-1.99 2L3 19c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-2 11h-3v3c0 .55-.45 1-1 1h-2c-.55 0-1-.45-1-1v-3H7c-.55 0-1-.45-1-1v-2c0-.55.45-1 1-1h3V7c0-.55.45-1 1-1h2c.55 0 1 .45 1 1v3h3c.55 0 1 .45 1 1v2c0 .55-.45 1-1 1z",
            }
        }
    }
}

pub fn local_hotel_icons_24px(props: IconProps) -> Element {
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
                d: "M7 13c1.66 0 3-1.34 3-3S8.66 7 7 7s-3 1.34-3 3 1.34 3 3 3zm12-6h-6c-1.1 0-2 .9-2 2v5H3V6c0-.55-.45-1-1-1s-1 .45-1 1v13c0 .55.45 1 1 1s1-.45 1-1v-2h18v2c0 .55.45 1 1 1s1-.45 1-1v-8c0-2.21-1.79-4-4-4z",
            }
        }
    }
}

pub fn local_laundry_service_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M9.64,16.36c1.3,1.3,3.42,1.3,4.72,0c1.3-1.3,1.3-3.42,0-4.72L9.64,16.36z M18,2.01L6,2C4.89,2,4,2.89,4,4v16 c0,1.11,0.89,2,2,2h12c1.11,0,2-0.89,2-2V4C20,2.89,19.11,2.01,18,2.01z M11,5c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1 S10.45,5,11,5z M8,5c0.55,0,1,0.45,1,1S8.55,7,8,7S7,6.55,7,6S7.45,5,8,5z M12,19c-2.76,0-5-2.24-5-5c0-2.76,2.24-5,5-5s5,2.24,5,5 C17,16.76,14.76,19,12,19z",
                }
            }
        }
    }
}

pub fn local_library_icons_24px(props: IconProps) -> Element {
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
                d: "M12 11.55c-1.82-1.7-4.12-2.89-6.68-3.35C4.11 7.99 3 8.95 3 10.18v6.24c0 1.68.72 2.56 1.71 2.69 2.5.32 4.77 1.35 6.63 2.87.35.29.92.32 1.27.04 1.87-1.53 4.16-2.58 6.68-2.9.94-.13 1.71-1.06 1.71-2.02v-6.92c0-1.23-1.11-2.19-2.32-1.98-2.56.46-4.86 1.65-6.68 3.35zM12 8c1.66 0 3-1.34 3-3s-1.34-3-3-3-3 1.34-3 3 1.34 3 3 3z",
            }
        }
    }
}

pub fn local_mall_icons_24px(props: IconProps) -> Element {
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
                d: "M19 6h-2c0-2.76-2.24-5-5-5S7 3.24 7 6H5c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-7-3c1.66 0 3 1.34 3 3H9c0-1.66 1.34-3 3-3zm0 10c-2.33 0-4.29-1.59-4.84-3.75-.17-.63.32-1.25.97-1.25.47 0 .85.34.98.8.35 1.27 1.51 2.2 2.89 2.2s2.54-.93 2.89-2.2c.13-.46.51-.8.98-.8.65 0 1.13.62.97 1.25C16.29 11.41 14.33 13 12 13z",
            }
        }
    }
}

pub fn local_movies_icons_24px(props: IconProps) -> Element {
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
                d: "M18 4v1h-2V4c0-.55-.45-1-1-1H9c-.55 0-1 .45-1 1v1H6V4c0-.55-.45-1-1-1s-1 .45-1 1v16c0 .55.45 1 1 1s1-.45 1-1v-1h2v1c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-1h2v1c0 .55.45 1 1 1s1-.45 1-1V4c0-.55-.45-1-1-1s-1 .45-1 1zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z",
            }
        }
    }
}

pub fn local_offer_icons_24px(props: IconProps) -> Element {
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
                d: "m21.41 11.58-9-9C12.05 2.22 11.55 2 11 2H4c-1.1 0-2 .9-2 2v7c0 .55.22 1.05.59 1.42l9 9c.36.36.86.58 1.41.58s1.05-.22 1.41-.59l7-7c.37-.36.59-.86.59-1.41s-.23-1.06-.59-1.42zM5.5 7C4.67 7 4 6.33 4 5.5S4.67 4 5.5 4 7 4.67 7 5.5 6.33 7 5.5 7z",
            }
        }
    }
}

pub fn local_parking_icons_24px(props: IconProps) -> Element {
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
                d: "M12.79 3H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2s2-.9 2-2v-4h3c3.57 0 6.42-3.13 5.95-6.79C18.56 5.19 15.84 3 12.79 3zm.41 8H10V7h3.2c1.1 0 2 .9 2 2s-.9 2-2 2z",
            }
        }
    }
}

pub fn local_pharmacy_icons_24px(props: IconProps) -> Element {
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
                d: "M18.89 5h-.53l.71-1.97c.24-.65-.1-1.37-.75-1.6-.65-.24-1.37.1-1.61.75L15.69 5H5.1C3.73 5 2.77 6.34 3.2 7.63L5 13l-1.79 5.37C2.77 19.66 3.74 21 5.1 21h13.78c1.36 0 2.33-1.34 1.9-2.63L19 13l1.78-5.37C21.21 6.34 20.25 5 18.89 5zM15 14h-2v2c0 .55-.45 1-1 1s-1-.45-1-1v-2H9c-.55 0-1-.45-1-1s.45-1 1-1h2v-2c0-.55.45-1 1-1s1 .45 1 1v2h2c.55 0 1 .45 1 1s-.45 1-1 1z",
            }
        }
    }
}

pub fn local_phone_icons_24px(props: IconProps) -> Element {
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
                d: "M19.23 15.26l-2.54-.29c-.61-.07-1.21.14-1.64.57l-1.84 1.84c-2.83-1.44-5.15-3.75-6.59-6.59l1.85-1.85c.43-.43.64-1.03.57-1.64l-.29-2.52c-.12-1.01-.97-1.77-1.99-1.77H5.03c-1.13 0-2.07.94-2 2.07.53 8.54 7.36 15.36 15.89 15.89 1.13.07 2.07-.87 2.07-2v-1.73c.01-1.01-.75-1.86-1.76-1.98z",
            }
        }
    }
}

pub fn local_pizza_icons_24px(props: IconProps) -> Element {
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
                d: "M12 2C9.01 2 6.28 3.08 4.17 4.88c-.71.61-.86 1.65-.4 2.46l7.36 13.11c.38.68 1.36.68 1.74 0l7.36-13.11c.46-.81.31-1.86-.4-2.46C17.73 3.09 14.99 2 12 2zM7 7c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2-2-.9-2-2zm5 8c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

pub fn local_play_icons_24px(props: IconProps) -> Element {
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
                d: "M20 12c0-.76.43-1.42 1.06-1.76.6-.33.94-1.01.94-1.7V6c0-1.1-.9-2-2-2H4c-1.1 0-1.99.89-1.99 1.99v2.55c0 .69.33 1.37.94 1.69C3.58 10.58 4 11.24 4 12s-.43 1.43-1.06 1.76c-.6.33-.94 1.01-.94 1.7v2.25C2 19.1 2.9 20 4 20h16c1.1 0 2-.9 2-2v-2.54c0-.69-.34-1.37-.94-1.7-.63-.34-1.06-1-1.06-1.76zm-5.5 4.1L12 14.5l-2.5 1.61c-.38.24-.87-.11-.75-.55l.75-2.88-2.3-1.88c-.35-.29-.17-.86.29-.89l2.96-.17 1.08-2.75c.17-.42.77-.42.93 0l1.08 2.76 2.96.17c.45.03.64.6.29.89l-2.3 1.88.76 2.86c.12.45-.37.8-.75.55z",
            }
        }
    }
}

pub fn local_police_icons_24px(props: IconProps) -> Element {
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
                d: "M14.5,12.59l0.63,2.73c0.1,0.43-0.37,0.77-0.75,0.54L12,14.42l-2.39,1.44c-0.38,0.23-0.85-0.11-0.75-0.54L9.5,12.6 l-2.1-1.81C7.06,10.5,7.24,9.95,7.68,9.91l2.78-0.24l1.08-2.56c0.17-0.41,0.75-0.41,0.92,0l1.08,2.55l2.78,0.24 c0.44,0.04,0.62,0.59,0.28,0.88L14.5,12.59z M4.19,4.47C3.47,4.79,3,5.51,3,6.3V11c0,5.55,3.84,10.74,9,12c5.16-1.26,9-6.45,9-12 V6.3c0-0.79-0.47-1.51-1.19-1.83l-7-3.11c-0.52-0.23-1.11-0.23-1.62,0L4.19,4.47z",
            }
        }
    }
}

pub fn local_post_office_icons_24px(props: IconProps) -> Element {
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
                d: "M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-.4 4.25l-6.54 4.09c-.65.41-1.47.41-2.12 0L4.4 8.25c-.25-.16-.4-.43-.4-.72 0-.67.73-1.07 1.3-.72L12 11l6.7-4.19c.57-.35 1.3.05 1.3.72 0 .29-.15.56-.4.72z",
            }
        }
    }
}

pub fn local_printshop_icons_24px(props: IconProps) -> Element {
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
                        d: "M18,7V4c0-0.55-0.45-1-1-1H7C6.45,3,6,3.45,6,4v3H18z",
                    }
                    path {
                        d: "M19,8H5c-1.66,0-3,1.34-3,3v5c0,0.55,0.45,1,1,1h3v2c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-2h3c0.55,0,1-0.45,1-1v-5 C22,9.34,20.66,8,19,8z M16,19H8v-4h8V19z M18,12.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S18.55,12.5,18,12.5z",
                    }
                }
            }
        }
    }
}

pub fn local_see_icons_24px(props: IconProps) -> Element {
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
                g {
                    g {
                        path {
                            d: "M9.5,14c0,1.38,1.12,2.5,2.5,2.5c1.23,0,2.25-0.9,2.46-2.07c-1-1.01-1.83-1.98-2.48-2.93C10.61,11.52,9.5,12.63,9.5,14z",
                        }
                    }
                    g {
                        path {
                            d: "M18.65,17.08c-0.37,0.32-0.92,0.32-1.3,0c-1.26-1.08-0.7-0.61-1.3-1.14c-0.83,1.74-2.73,2.87-4.85,2.5 c-1.83-0.32-3.31-1.8-3.63-3.63c-0.42-2.44,1.13-4.58,3.31-5.14C10.3,8.45,10,7.28,10,6.15C10,5.4,10.1,4.68,10.28,4h-0.4 c-0.56,0-1.1,0.24-1.48,0.65L7.17,6H4C2.9,6,2,6.9,2,8v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-6.03 C20.59,15.46,19.35,16.48,18.65,17.08z",
                        }
                    }
                    g {
                        path {
                            d: "M17.34,14.42c0.37,0.33,0.95,0.33,1.33,0C22.22,11.25,24,8.5,24,6.15C24,2.42,21.15,0,18,0s-6,2.42-6,6.15 C12,8.5,13.78,11.25,17.34,14.42z M17.27,5.25L18,3l0.73,2.25H21l-1.85,1.47L19.85,9L18,7.59L16.15,9l0.7-2.28L15,5.25H17.27z",
                        }
                    }
                }
            }
        }
    }
}

pub fn local_shipping_icons_24px(props: IconProps) -> Element {
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
                d: "M19.5 8H17V6c0-1.1-.9-2-2-2H3c-1.1 0-2 .9-2 2v9c0 1.1.9 2 2 2 0 1.66 1.34 3 3 3s3-1.34 3-3h6c0 1.66 1.34 3 3 3s3-1.34 3-3h1c.55 0 1-.45 1-1v-3.33c0-.43-.14-.85-.4-1.2L20.3 8.4c-.19-.25-.49-.4-.8-.4zM6 18c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm13.5-8.5l1.96 2.5H17V9.5h2.5zM18 18c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

pub fn local_taxi_icons_24px(props: IconProps) -> Element {
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
                d: "M18.92 6.01C18.72 5.42 18.16 5 17.5 5H15V4c0-.55-.45-1-1-1h-4c-.55 0-1 .45-1 1v1H6.5c-.66 0-1.21.42-1.42 1.01l-1.97 5.67c-.07.21-.11.43-.11.66v7.16c0 .83.67 1.5 1.5 1.5S6 20.33 6 19.5V19h12v.5c0 .82.67 1.5 1.5 1.5.82 0 1.5-.67 1.5-1.5v-7.16c0-.22-.04-.45-.11-.66l-1.97-5.67zM6.5 16c-.83 0-1.5-.67-1.5-1.5S5.67 13 6.5 13s1.5.67 1.5 1.5S7.33 16 6.5 16zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 11l1.5-4.5h11L19 11H5z",
            }
        }
    }
}

pub fn lunch_dining_icons_20px(props: IconProps) -> Element {
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
                        d: "M16.85,8c0.67,0,1.19-0.66,0.96-1.29C17.02,4.59,13.82,3,10,3C6.19,3,2.99,4.59,2.19,6.71C1.95,7.34,2.48,8,3.15,8H16.85z",
                    }
                    path {
                        d: "M2,15.5C2,16.33,2.67,17,3.5,17h13c0.83,0,1.5-0.67,1.5-1.5v-1c0-0.83-0.67-1.5-1.5-1.5h-13C2.67,13,2,13.67,2,14.5V15.5z",
                    }
                    path {
                        d: "M15.34,9.25c-1.58,0-1.72,1-2.67,1c-0.95,0-1.08-1-2.67-1c-1.58,0-1.72,1-2.67,1c-0.95,0-1.09-1-2.67-1 c-1.28,0-1.61,0.65-2.19,0.9C2.19,10.28,2,10.54,2,10.85v0c0,0.53,0.54,0.91,1.03,0.71c0.7-0.29,0.96-0.82,1.65-0.82 c0.95,0,1.09,1,2.67,1c1.58,0,1.72-1,2.67-1c0.95,0,1.08,1,2.67,1c1.58,0,1.72-1,2.67-1c0.69,0,0.94,0.53,1.64,0.81 c0.49,0.2,1.03-0.18,1.03-0.71v0c0-0.31-0.19-0.58-0.48-0.7C16.95,9.9,16.61,9.25,15.34,9.25z",
                    }
                }
            }
        }
    }
}

pub fn lunch_dining_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M3.37,14.28c0.79-0.29,1.17-0.78,1.99-0.78c1.19,0,1.42,1,3.33,1c1.95,0,2.09-1,3.33-1c1.19,0,1.42,1,3.33,1 c1.95,0,2.09-1,3.33-1c0.81,0,1.17,0.46,1.93,0.76c0.67,0.26,1.39-0.25,1.39-0.96c0-0.43-0.28-0.81-0.69-0.96 c-0.97-0.35-1.22-0.83-2.65-0.83c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1 c-1.55,0-1.96,0.63-2.68,0.89c-0.39,0.14-0.65,0.52-0.65,0.94C2.01,14.03,2.71,14.52,3.37,14.28z",
                    }
                    path {
                        d: "M2,19c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-1c0-1.1-0.9-2-2-2H4c-1.1,0-2,0.9-2,2V19z",
                    }
                    path {
                        d: "M22,9c0.02-4-4.28-6-10-6C6.29,3,2,5,2,9v0c0,0.55,0.45,1,1,1h18C21.55,10,22,9.55,22,9L22,9L22,9z",
                    }
                }
            }
        }
    }
}

pub fn map_icons_24px(props: IconProps) -> Element {
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
                d: "M14.65 4.98l-5-1.75c-.42-.15-.88-.15-1.3-.01L4.36 4.56C3.55 4.84 3 5.6 3 6.46v11.85c0 1.41 1.41 2.37 2.72 1.86l2.93-1.14c.22-.09.47-.09.69-.01l5 1.75c.42.15.88.15 1.3.01l3.99-1.34c.81-.27 1.36-1.04 1.36-1.9V5.69c0-1.41-1.41-2.37-2.72-1.86l-2.93 1.14c-.22.08-.46.09-.69.01zM15 18.89l-6-2.11V5.11l6 2.11v11.67z",
            }
        }
    }
}

pub fn maps_ugc_icons_24px(props: IconProps) -> Element {
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
                fill_rule: "evenodd",
            }
            g {
                path {
                    d: "M12,4c4.97,0,8.9,4.56,7.82,9.72c-0.68,3.23-3.4,5.74-6.67,6.2c-1.59,0.22-3.14-0.01-4.58-0.7 c-0.27-0.13-0.56-0.19-0.86-0.19c-0.19,0-0.38,0.03-0.56,0.08l-2.31,0.68c-0.38,0.11-0.74-0.24-0.63-0.63l0.7-2.39 c0.13-0.45,0.07-0.92-0.14-1.35C4.26,14.34,4,13.18,4,12C4,7.59,7.59,4,12,4 M12,2C6.48,2,2,6.48,2,12c0,1.54,0.36,2.98,0.97,4.29 l-1.46,4.96C1.29,22,2,22.71,2.76,22.48l4.96-1.46c1.66,0.79,3.56,1.15,5.58,0.89c4.56-0.59,8.21-4.35,8.66-8.92 C22.53,7.03,17.85,2,12,2L12,2z",
                }
            }
            path {
                fill_rule: "evenodd",
                d: "M12,8L12,8c-0.55,0-1,0.45-1,1v2H9c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-2V9C13,8.45,12.55,8,12,8z",
            }
        }
    }
}

pub fn medical_information_icons_20px(props: IconProps) -> Element {
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
                    d: "M16.5,6h-4.25V3.5c0-0.83-0.67-1.5-1.5-1.5h-1.5c-0.83,0-1.5,0.67-1.5,1.5V6H3.5C2.67,6,2,6.67,2,7.5v9 C2,17.33,2.67,18,3.5,18h13c0.83,0,1.5-0.67,1.5-1.5v-9C18,6.67,17.33,6,16.5,6z M9.25,3.5h1.5v4h-1.5V3.5z M11.25,12 c-0.28,0-0.5-0.22-0.5-0.5s0.22-0.5,0.5-0.5h4c0.28,0,0.5,0.22,0.5,0.5s-0.22,0.5-0.5,0.5H11.25z M11.25,14.5 c-0.28,0-0.5-0.22-0.5-0.5s0.22-0.5,0.5-0.5h2.25c0.28,0,0.5,0.22,0.5,0.5s-0.22,0.5-0.5,0.5H11.25z M7.5,14.25 C7.5,14.66,7.16,15,6.75,15S6,14.66,6,14.25v-1H5c-0.41,0-0.75-0.34-0.75-0.75S4.59,11.75,5,11.75h1v-1C6,10.34,6.34,10,6.75,10 s0.75,0.34,0.75,0.75v1h1c0.41,0,0.75,0.34,0.75,0.75s-0.34,0.75-0.75,0.75h-1V14.25z",
                }
            }
        }
    }
}

pub fn medical_information_icons_24px(props: IconProps) -> Element {
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
                    d: "M20,7h-5V4c0-1.1-0.9-2-2-2h-2C9.9,2,9,2.9,9,4v3H4C2.9,7,2,7.9,2,9v11c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V9 C22,7.9,21.1,7,20,7z M11,4h2v5h-2V4z M7,16H6c-0.55,0-1-0.45-1-1s0.45-1,1-1h1v-1c0-0.55,0.45-1,1-1s1,0.45,1,1v1h1 c0.55,0,1,0.45,1,1s-0.45,1-1,1H9v1c0,0.55-0.45,1-1,1s-1-0.45-1-1V16z M13.75,14.5c-0.41,0-0.75-0.34-0.75-0.75S13.34,13,13.75,13 h4.5c0.41,0,0.75,0.34,0.75,0.75s-0.34,0.75-0.75,0.75H13.75z M13.75,17.5c-0.41,0-0.75-0.34-0.75-0.75S13.34,16,13.75,16h2.5 c0.41,0,0.75,0.34,0.75,0.75s-0.34,0.75-0.75,0.75H13.75z",
                }
            }
        }
    }
}

pub fn medical_services_icons_20px(props: IconProps) -> Element {
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
                        d: "M16,6h-3V4c0-0.55-0.45-1-1-1H8C7.45,3,7,3.45,7,4v2H4C3.45,6,3,6.45,3,7v9c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1V7 C17,6.45,16.55,6,16,6z M8,4h4v2H8V4z M12,12h-1.5v1.5c0,0.28-0.22,0.5-0.5,0.5s-0.5-0.22-0.5-0.5V12H8c-0.28,0-0.5-0.22-0.5-0.5 C7.5,11.22,7.72,11,8,11h1.5V9.5C9.5,9.22,9.72,9,10,9s0.5,0.22,0.5,0.5V11H12c0.28,0,0.5,0.22,0.5,0.5C12.5,11.78,12.28,12,12,12 z",
                    }
                }
            }
        }
    }
}

pub fn medical_services_icons_24px(props: IconProps) -> Element {
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
                    path {
                        d: "M20,6h-4V4c0-1.1-0.9-2-2-2h-4C8.9,2,8,2.9,8,4v2H4C2.9,6,2,6.9,2,8v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M10,4h4v2h-4V4z M15,15h-2v2c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2H9c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h2v-2 c0-0.55,0.45-1,1-1s1,0.45,1,1v2h2c0.55,0,1,0.45,1,1C16,14.55,15.55,15,15,15z",
                    }
                }
            }
        }
    }
}

pub fn menu_book_icons_24px(props: IconProps) -> Element {
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
                    d: "M17.5,4.5c-1.95,0-4.05,0.4-5.5,1.5c-1.45-1.1-3.55-1.5-5.5-1.5c-1.45,0-2.99,0.22-4.28,0.79C1.49,5.62,1,6.33,1,7.14 l0,11.28c0,1.3,1.22,2.26,2.48,1.94C4.46,20.11,5.5,20,6.5,20c1.56,0,3.22,0.26,4.56,0.92c0.6,0.3,1.28,0.3,1.87,0 c1.34-0.67,3-0.92,4.56-0.92c1,0,2.04,0.11,3.02,0.36c1.26,0.33,2.48-0.63,2.48-1.94l0-11.28c0-0.81-0.49-1.52-1.22-1.85 C20.49,4.72,18.95,4.5,17.5,4.5z M21,17.23c0,0.63-0.58,1.09-1.2,0.98c-0.75-0.14-1.53-0.2-2.3-0.2c-1.7,0-4.15,0.65-5.5,1.5V8 c1.35-0.85,3.8-1.5,5.5-1.5c0.92,0,1.83,0.09,2.7,0.28c0.46,0.1,0.8,0.51,0.8,0.98V17.23z",
                }
                g {
                }
                path {
                    d: "M13.98,11.01c-0.32,0-0.61-0.2-0.71-0.52c-0.13-0.39,0.09-0.82,0.48-0.94c1.54-0.5,3.53-0.66,5.36-0.45 c0.41,0.05,0.71,0.42,0.66,0.83c-0.05,0.41-0.42,0.71-0.83,0.66c-1.62-0.19-3.39-0.04-4.73,0.39 C14.13,10.99,14.05,11.01,13.98,11.01z",
                }
                path {
                    d: "M13.98,13.67c-0.32,0-0.61-0.2-0.71-0.52c-0.13-0.39,0.09-0.82,0.48-0.94c1.53-0.5,3.53-0.66,5.36-0.45 c0.41,0.05,0.71,0.42,0.66,0.83c-0.05,0.41-0.42,0.71-0.83,0.66c-1.62-0.19-3.39-0.04-4.73,0.39 C14.13,13.66,14.05,13.67,13.98,13.67z",
                }
                path {
                    d: "M13.98,16.33c-0.32,0-0.61-0.2-0.71-0.52c-0.13-0.39,0.09-0.82,0.48-0.94c1.53-0.5,3.53-0.66,5.36-0.45 c0.41,0.05,0.71,0.42,0.66,0.83c-0.05,0.41-0.42,0.7-0.83,0.66c-1.62-0.19-3.39-0.04-4.73,0.39 C14.13,16.32,14.05,16.33,13.98,16.33z",
                }
            }
        }
    }
}

pub fn merge_icons_20px(props: IconProps) -> Element {
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
                    d: "M14.47,16.47c0.29-0.29,0.29-0.77,0-1.06l-2.84-2.84c-0.56-0.56-0.88-1.33-0.88-2.12l0-4.58l0.66,0.66 c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06l-1.76-1.76c-0.39-0.39-1.02-0.39-1.41,0L7.53,5.47 c-0.29,0.29-0.29,0.77,0,1.06c0.29,0.29,0.77,0.29,1.06,0l0.66-0.66l0,4.58c0,0.8-0.32,1.56-0.88,2.12l-2.84,2.84 c-0.29,0.29-0.29,0.77,0,1.06c0.29,0.29,0.77,0.29,1.06,0L10,13.06l3.41,3.41C13.7,16.76,14.18,16.76,14.47,16.47z",
                }
            }
        }
    }
}

pub fn merge_icons_24px(props: IconProps) -> Element {
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
                    fill: "none",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M8.71,7.71c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59c0.39-0.39,1.02-0.39,1.41,0l2.59,2.59c0.39,0.39,0.39,1.02,0,1.41 c-0.39,0.39-1.02,0.39-1.41,0L13,6.83v5.1c0,1.06,0.42,2.08,1.17,2.83l4.12,4.12c0.39,0.39,0.39,1.02,0,1.41s-1.02,0.39-1.41,0 L12,15.41l-4.88,4.88c-0.39,0.39-1.02,0.39-1.41,0c-0.39-0.39-0.39-1.02,0-1.41l4.12-4.12c0.75-0.75,1.17-1.77,1.17-2.83v-5.1 l-0.88,0.88C9.73,8.1,9.1,8.1,8.71,7.71z",
                }
            }
        }
    }
}

pub fn minor_crash_icons_20px(props: IconProps) -> Element {
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
                    d: "M6.17,2.87l1.3,1.3c0.29,0.29,0.29,0.76,0,1.05c-0.29,0.29-0.76,0.29-1.05,0l-1.3-1.3c-0.29-0.29-0.29-0.76,0-1.05 C5.41,2.59,5.89,2.59,6.17,2.87z M13.83,2.87l-1.3,1.3c-0.29,0.29-0.29,0.76,0,1.05c0.29,0.29,0.76,0.29,1.05,0l1.3-1.3 c0.29-0.29,0.29-0.76,0-1.05C14.59,2.59,14.11,2.59,13.83,2.87z M10.75,3.95v-2.2C10.75,1.34,10.41,1,10,1S9.25,1.34,9.25,1.75v2.2 C9.25,4.36,9.59,4.7,10,4.7S10.75,4.36,10.75,3.95z M17,18v-5.71c0-0.19-0.04-0.38-0.11-0.56l-1.52-3.79C15.15,7.37,14.6,7,13.98,7 H6.02C5.4,7,4.85,7.37,4.62,7.94l-1.52,3.79C3.04,11.91,3,12.1,3,12.29L3,18c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-0.5h10V18 c0,0.55,0.45,1,1,1C16.55,19,17,18.55,17,18z M13.98,8.5l1,2.5H5.02l1-2.5H13.98z M7.5,14.25c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1 s0.45-1,1-1C7.05,13.25,7.5,13.7,7.5,14.25z M14.5,14.25c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1s0.45-1,1-1 C14.05,13.25,14.5,13.7,14.5,14.25z",
                }
            }
        }
    }
}

pub fn minor_crash_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M19.5,24c0.82,0,1.5-0.67,1.5-1.5v-7.16c0-0.22-0.04-0.45-0.11-0.66l-1.97-5.67C18.72,8.42,18.16,8,17.5,8h-11 C5.84,8,5.29,8.42,5.08,9.01l-1.97,5.67C3.04,14.89,3,15.11,3,15.34v7.16C3,23.33,3.68,24,4.5,24S6,23.33,6,22.5V22h12v0.5 C18,23.33,18.67,24,19.5,24z M6.85,10h10.29l1.04,3H5.81L6.85,10z M6,17.5C6,16.67,6.67,16,7.5,16S9,16.67,9,17.5S8.33,19,7.5,19 S6,18.33,6,17.5z M15,17.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S17.33,19,16.5,19S15,18.33,15,17.5z M8.71,5.71 c-0.39,0.39-1.02,0.39-1.41,0L5.71,4.12c-0.39-0.39-0.39-1.02,0-1.41s1.02-0.39,1.41,0l1.59,1.59C9.1,4.68,9.1,5.32,8.71,5.71z M18.29,2.71c0.39,0.39,0.39,1.02,0,1.41l-1.59,1.59c-0.39,0.39-1.02,0.39-1.41,0s-0.39-1.02,0-1.41l1.59-1.59 C17.27,2.32,17.9,2.32,18.29,2.71z M12,5c-0.55,0-1-0.45-1-1V1c0-0.55,0.45-1,1-1s1,0.45,1,1v3C13,4.55,12.55,5,12,5z",
                }
            }
        }
    }
}

pub fn miscellaneous_services_icons_20px(props: IconProps) -> Element {
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
                        d: "M11.69,11.36l1-1.73c0.06-0.11,0.04-0.24-0.06-0.32l-1.06-0.83C11.6,8.33,11.61,8.16,11.61,8c0-0.16-0.01-0.33-0.04-0.49 l1.06-0.83c0.09-0.08,0.12-0.21,0.06-0.32l-1-1.73c-0.06-0.11-0.19-0.15-0.31-0.11l-1.24,0.5C9.88,4.82,9.6,4.66,9.3,4.53 L9.11,3.21C9.09,3.09,8.98,3,8.86,3h-2C6.74,3,6.63,3.09,6.61,3.21L6.42,4.54c-0.3,0.13-0.59,0.29-0.84,0.49l-1.24-0.5 c-0.11-0.04-0.24,0-0.31,0.11l-1,1.73C2.97,6.47,3,6.61,3.09,6.69l1.06,0.83C4.12,7.67,4.11,7.84,4.11,8 c0,0.16,0.01,0.33,0.04,0.49L3.09,9.32C3,9.4,2.97,9.54,3.03,9.64l1,1.73c0.06,0.11,0.19,0.15,0.31,0.11l1.24-0.5 c0.26,0.2,0.54,0.36,0.84,0.49l0.19,1.32C6.63,12.91,6.74,13,6.86,13h2c0.12,0,0.23-0.09,0.25-0.21l0.19-1.32 c0.3-0.13,0.59-0.29,0.84-0.49l1.24,0.5C11.5,11.51,11.62,11.47,11.69,11.36z M7.86,9.5c-0.83,0-1.5-0.68-1.5-1.5 c0-0.83,0.67-1.5,1.5-1.5c0.82,0,1.5,0.67,1.5,1.5C9.36,8.82,8.68,9.5,7.86,9.5z",
                    }
                    path {
                        d: "M16.81,14.79l-0.64-0.5c0.01-0.1,0.02-0.19,0.02-0.29c0-0.1-0.01-0.2-0.02-0.29l0.64-0.5c0.05-0.05,0.07-0.13,0.04-0.19 l-0.6-1.04c-0.04-0.07-0.11-0.09-0.19-0.07l-0.74,0.3c-0.16-0.12-0.32-0.22-0.51-0.29l-0.11-0.79C14.68,11.05,14.61,11,14.54,11 h-1.2c-0.07,0-0.14,0.05-0.15,0.13l-0.11,0.79c-0.18,0.08-0.35,0.17-0.51,0.29l-0.74-0.3c-0.07-0.02-0.14,0-0.19,0.07l-0.6,1.04 c-0.04,0.07-0.02,0.14,0.04,0.19l0.64,0.5c-0.01,0.1-0.02,0.19-0.02,0.29c0,0.1,0.01,0.2,0.02,0.29l-0.64,0.5 c-0.05,0.05-0.07,0.13-0.04,0.19l0.6,1.04c0.04,0.07,0.11,0.09,0.19,0.07l0.74-0.3c0.16,0.12,0.32,0.22,0.51,0.29l0.11,0.79 C13.2,16.95,13.26,17,13.34,17h1.2c0.07,0,0.14-0.05,0.15-0.13l0.11-0.79c0.18-0.08,0.35-0.17,0.5-0.29l0.75,0.3 c0.07,0.02,0.14,0,0.19-0.07l0.6-1.04C16.88,14.92,16.86,14.84,16.81,14.79z M14,15c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 c0.55,0,1,0.45,1,1C15,14.55,14.55,15,14,15z",
                    }
                }
            }
        }
    }
}

pub fn miscellaneous_services_icons_24px(props: IconProps) -> Element {
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
                        d: "M14.17,13.71l1.4-2.42c0.09-0.15,0.05-0.34-0.08-0.45l-1.48-1.16c0.03-0.22,0.05-0.45,0.05-0.68s-0.02-0.46-0.05-0.69 l1.48-1.16c0.13-0.11,0.17-0.3,0.08-0.45l-1.4-2.42c-0.09-0.15-0.27-0.21-0.43-0.15L12,4.83c-0.36-0.28-0.75-0.51-1.18-0.69 l-0.26-1.85C10.53,2.13,10.38,2,10.21,2h-2.8C7.24,2,7.09,2.13,7.06,2.3L6.8,4.15C6.38,4.33,5.98,4.56,5.62,4.84l-1.74-0.7 c-0.16-0.06-0.34,0-0.43,0.15l-1.4,2.42C1.96,6.86,2,7.05,2.13,7.16l1.48,1.16C3.58,8.54,3.56,8.77,3.56,9s0.02,0.46,0.05,0.69 l-1.48,1.16C2,10.96,1.96,11.15,2.05,11.3l1.4,2.42c0.09,0.15,0.27,0.21,0.43,0.15l1.74-0.7c0.36,0.28,0.75,0.51,1.18,0.69 l0.26,1.85C7.09,15.87,7.24,16,7.41,16h2.8c0.17,0,0.32-0.13,0.35-0.3l0.26-1.85c0.42-0.18,0.82-0.41,1.18-0.69l1.74,0.7 C13.9,13.92,14.08,13.86,14.17,13.71z M8.81,11c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2C10.81,10.1,9.91,11,8.81,11z",
                    }
                    path {
                        d: "M21.92,18.67l-0.96-0.74c0.02-0.14,0.04-0.29,0.04-0.44c0-0.15-0.01-0.3-0.04-0.44l0.95-0.74 c0.08-0.07,0.11-0.19,0.05-0.29l-0.9-1.55c-0.05-0.1-0.17-0.13-0.28-0.1l-1.11,0.45c-0.23-0.18-0.48-0.33-0.76-0.44l-0.17-1.18 C18.73,13.08,18.63,13,18.53,13h-1.79c-0.11,0-0.21,0.08-0.22,0.19l-0.17,1.18c-0.27,0.12-0.53,0.26-0.76,0.44l-1.11-0.45 c-0.1-0.04-0.22,0-0.28,0.1l-0.9,1.55c-0.05,0.1-0.04,0.22,0.05,0.29l0.95,0.74c-0.02,0.14-0.03,0.29-0.03,0.44 c0,0.15,0.01,0.3,0.03,0.44l-0.95,0.74c-0.08,0.07-0.11,0.19-0.05,0.29l0.9,1.55c0.05,0.1,0.17,0.13,0.28,0.1l1.11-0.45 c0.23,0.18,0.48,0.33,0.76,0.44l0.17,1.18c0.02,0.11,0.11,0.19,0.22,0.19h1.79c0.11,0,0.21-0.08,0.22-0.19l0.17-1.18 c0.27-0.12,0.53-0.26,0.75-0.44l1.12,0.45c0.1,0.04,0.22,0,0.28-0.1l0.9-1.55C22.03,18.86,22,18.74,21.92,18.67z M17.63,18.83 c-0.74,0-1.35-0.6-1.35-1.35s0.6-1.35,1.35-1.35s1.35,0.6,1.35,1.35S18.37,18.83,17.63,18.83z",
                    }
                }
            }
        }
    }
}

pub fn mode_of_travel_icons_20px(props: IconProps) -> Element {
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
                width: "20",
                height: "20",
            }
            path {
                d: "M14.98,8h-1.52c-0.45,0-0.67,0.54-0.35,0.85l2.29,2.29c0.2,0.2,0.51,0.2,0.71,0l2.29-2.29C18.71,8.54,18.49,8,18.04,8 l-1.56,0C16.23,4.64,13.42,2,10,2C6.41,2,3.5,4.91,3.5,8.5C3.5,12.84,10,18,10,18s1.28-1.01,2.7-2.52c0.1,0.01,0.2,0.02,0.3,0.02 c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2c0,0.44,0.14,0.85,0.38,1.18c-0.49,0.51-0.97,0.97-1.38,1.35C8.1,14.3,5,10.97,5,8.5 c0-2.76,2.24-5,5-5C12.59,3.5,14.72,5.48,14.98,8z",
            }
        }
    }
}

pub fn mode_of_travel_icons_24px(props: IconProps) -> Element {
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
                d: "M4,10.2C4,5.22,7.8,2,12,2c4,0,7.64,2.92,7.97,7.5l2.32,0c0.45,0,0.67,0.54,0.35,0.85l-3.29,3.29c-0.2,0.2-0.51,0.2-0.71,0 l-3.29-3.29c-0.31-0.31-0.09-0.85,0.35-0.85l2.26,0C17.65,6.24,15.13,4,12,4c-3.35,0-6,2.57-6,6.2c0,2.34,1.95,5.44,6,9.14 c0.64-0.59,1.23-1.16,1.77-1.71c-0.17-0.34-0.27-0.72-0.27-1.12c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5S17.38,19,16,19 c-0.24,0-0.47-0.03-0.69-0.1c-0.78,0.82-1.67,1.66-2.65,2.52c-0.38,0.33-0.95,0.33-1.33,0C6.45,17.12,4,13.38,4,10.2z",
            }
        }
    }
}

pub fn money_icons_24px(props: IconProps) -> Element {
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
                d: "M15 16h3c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1h-3c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1zm1-6h1v4h-1v-4zm-7 6h3c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1H9c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1zm1-6h1v4h-1v-4zM6 8c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1s1-.45 1-1V9c0-.55-.45-1-1-1zM2 6v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2zm17 12H5c-.55 0-1-.45-1-1V7c0-.55.45-1 1-1h14c.55 0 1 .45 1 1v10c0 .55-.45 1-1 1z",
            }
        }
    }
}

pub fn moped_icons_20px(props: IconProps) -> Element {
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
                        d: "M15,7.5C15,6.67,14.33,6,13.5,6h-1C12.22,6,12,6.22,12,6.5v0C12,6.78,12.22,7,12.5,7h1C13.78,7,14,7.22,14,7.5v1.29 L10.79,12H9V8.5C9,8.22,8.78,8,8.5,8H6c-1.66,0-3,1.34-3,3v2h2c0,1.1,0.9,2,2,2s2-0.9,2-2h2.21L15,9.21V7.5z M7,14 c-0.55,0-1-0.45-1-1h2C8,13.55,7.55,14,7,14z",
                    }
                    path {
                        d: "M8.5,6h-3C5.22,6,5,6.22,5,6.5v0C5,6.78,5.22,7,5.5,7h3C8.78,7,9,6.78,9,6.5v0C9,6.22,8.78,6,8.5,6z",
                    }
                    path {
                        d: "M15,11c-1.1,0-2,0.9-2,2c0,1.1,0.9,2,2,2s2-0.9,2-2C17,11.9,16.1,11,15,11z M15,14c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 s1,0.45,1,1C16,13.55,15.55,14,15,14z",
                    }
                }
            }
        }
    }
}

pub fn moped_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M19,7c0-1.1-0.9-2-2-2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2.65L13.52,14H10v-4c0-0.55-0.45-1-1-1H6 c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,10.35V7z M7,17c-0.55,0-1-0.45-1-1h2C8,16.55,7.55,17,7,17z",
                    }
                    path {
                        d: "M9,6H6C5.45,6,5,6.45,5,7v0c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v0C10,6.45,9.55,6,9,6z",
                    }
                    path {
                        d: "M19,13c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,13,19,13z M19,17c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S19.55,17,19,17z",
                    }
                }
            }
        }
    }
}

pub fn mosque_icons_20px(props: IconProps) -> Element {
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
                        d: "M5.67,7h8.67c0.17,0,0.33,0.02,0.49,0.05C14.93,6.71,15,6.36,15,5.99c0-1.23-0.65-2.38-1.71-3.01L10,1L6.71,2.98 C5.65,3.61,5,4.76,5,5.99c0,0.37,0.07,0.72,0.18,1.06C5.34,7.02,5.5,7,5.67,7z",
                    }
                    path {
                        d: "M19.75,5.5c0-0.83-1.5-2.5-1.5-2.5s-1.5,1.67-1.5,2.5c0,0.55,0.3,1.03,0.75,1.29V11H16V9.67C16,8.75,15.25,8,14.33,8H5.67 C4.75,8,4,8.75,4,9.67V11H2.5V6.79C2.95,6.53,3.25,6.05,3.25,5.5c0-0.83-1.5-2.5-1.5-2.5s-1.5,1.67-1.5,2.5 c0,0.55,0.3,1.03,0.75,1.29V17h7v-3c0-1.1,0.9-2,2-2s2,0.9,2,2v3h7V6.79C19.45,6.53,19.75,6.05,19.75,5.5z",
                    }
                }
            }
        }
    }
}

pub fn mosque_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M7,8h10c0.29,0,0.57,0.06,0.84,0.13C17.93,7.8,18,7.46,18,7.09c0-1.31-0.65-2.53-1.74-3.25L12,1L7.74,3.84 C6.65,4.56,6,5.78,6,7.09C6,7.46,6.07,7.8,6.16,8.13C6.43,8.06,6.71,8,7,8z",
                    }
                    path {
                        d: "M24,7c0-1.1-2-3-2-3s-2,1.9-2,3c0,0.74,0.4,1.38,1,1.72V13h-2v-2c0-1.1-0.9-2-2-2H7c-1.1,0-2,0.9-2,2v2H3V8.72 C3.6,8.38,4,7.74,4,7c0-1.1-2-3-2-3S0,5.9,0,7c0,0.74,0.4,1.38,1,1.72V21h9v-4c0-1.1,0.9-2,2-2s2,0.9,2,2v4h9V8.72 C23.6,8.38,24,7.74,24,7z",
                    }
                }
            }
        }
    }
}

pub fn moving_icons_20px(props: IconProps) -> Element {
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
                width: "20",
                height: "20",
            }
            path {
                d: "M2.53,14.12c-0.29-0.29-0.29-0.77,0-1.06L6.08,9.5c0.98-0.98,2.56-0.98,3.54,0l0.71,0.71c0.39,0.39,1.02,0.39,1.41,0 l3.23-3.23l-1.12-1.12C13.54,5.54,13.76,5,14.21,5h3.29C17.78,5,18,5.22,18,5.5v3.29c0,0.45-0.54,0.67-0.85,0.35l-1.12-1.12 l-3.23,3.23c-0.98,0.98-2.56,0.98-3.54,0l-0.7-0.7c-0.39-0.39-1.02-0.39-1.41,0l-3.56,3.56C3.3,14.41,2.82,14.41,2.53,14.12z",
            }
        }
    }
}

pub fn moving_icons_24px(props: IconProps) -> Element {
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
                d: "M2.7,17.29c0.39,0.39,1.02,0.39,1.41,0l4.59-4.59c0.39-0.39,1.02-0.39,1.41,0l1.17,1.17c1.17,1.17,3.07,1.17,4.24,0 l4.18-4.17l1.44,1.44c0.31,0.31,0.85,0.09,0.85-0.35V6.5C22,6.22,21.78,6,21.5,6h-4.29c-0.45,0-0.67,0.54-0.35,0.85l1.44,1.44 l-4.17,4.17c-0.39,0.39-1.02,0.39-1.41,0l-1.17-1.17c-1.17-1.17-3.07-1.17-4.24,0L2.7,15.88C2.32,16.27,2.32,16.91,2.7,17.29z",
            }
        }
    }
}

pub fn multiple_stop_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M17,5.21c0-0.45,0.54-0.67,0.85-0.35l2.79,2.79c0.2,0.2,0.2,0.51,0,0.71l-2.79,2.79C17.54,11.46,17,11.24,17,10.79V9h-3 c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h3V5.21z M10,7C9.45,7,9,7.45,9,8s0.45,1,1,1s1-0.45,1-1S10.55,7,10,7z M6,7 C5.45,7,5,7.45,5,8s0.45,1,1,1s1-0.45,1-1S6.55,7,6,7z M7,17h3c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H7v-1.79 c0-0.45-0.54-0.67-0.85-0.35l-2.79,2.79c-0.2,0.2-0.2,0.51,0,0.71l2.79,2.79C6.46,19.46,7,19.24,7,18.79V17z M14,17 c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1s-1,0.45-1,1C13,16.55,13.45,17,14,17z M18,17c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1 s-1,0.45-1,1C17,16.55,17.45,17,18,17z",
                }
            }
        }
    }
}

pub fn museum_icons_20px(props: IconProps) -> Element {
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
                    d: "M16.49,9.06c0.27,0,0.5-0.22,0.5-0.49l0-0.24C17,8.16,16.92,8,16.79,7.9l-6.5-4.61c-0.17-0.12-0.41-0.12-0.58,0l-6.5,4.62 C3.08,8.01,3,8.16,3,8.33l0,0.24c0,0.27,0.23,0.49,0.5,0.49H5v7H3.52c-0.28,0-0.5,0.22-0.5,0.5c0,0.28,0.22,0.5,0.5,0.5h12.97 c0.28,0,0.5-0.22,0.5-0.5c0-0.28-0.22-0.5-0.5-0.5H15v-7H16.49z M13,14.56c0,0.28-0.22,0.5-0.5,0.5c-0.28,0-0.5-0.22-0.5-0.5v-3.17 l-1.6,2.13c-0.2,0.27-0.6,0.27-0.8,0L8,11.39v3.17c0,0.28-0.22,0.5-0.5,0.5c-0.28,0-0.5-0.22-0.5-0.5v-5c0-0.28,0.22-0.5,0.5-0.5 h0.23c0.17,0,0.32,0.08,0.42,0.22L10,12.06l1.85-2.78c0.09-0.14,0.25-0.22,0.42-0.22h0.23c0.28,0,0.5,0.22,0.5,0.5V14.56z",
                }
            }
        }
    }
}

pub fn museum_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M21.5,11c0.28,0,0.5-0.22,0.5-0.5V9.26c0-0.16-0.08-0.32-0.21-0.41L12.57,2.4c-0.34-0.24-0.8-0.24-1.15,0L2.21,8.85 C2.08,8.94,2,9.1,2,9.26v1.24C2,10.78,2.22,11,2.5,11H4v9H3c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1 c0-0.55-0.45-1-1-1h-1v-9H21.5z M16,17c0,0.55-0.45,1-1,1s-1-0.45-1-1v-3l-1.17,1.75c-0.4,0.59-1.27,0.59-1.66,0L10,14v3 c0,0.55-0.45,1-1,1s-1-0.45-1-1v-4.7C8,11.58,8.58,11,9.3,11h0c0.43,0,0.84,0.22,1.08,0.58L12,14l1.61-2.42 C13.86,11.22,14.26,11,14.7,11h0c0.72,0,1.3,0.58,1.3,1.3V17z",
                }
            }
        }
    }
}

pub fn my_location_icons_24px(props: IconProps) -> Element {
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
                d: "M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm8.94 3c-.46-4.17-3.77-7.48-7.94-7.94V2c0-.55-.45-1-1-1s-1 .45-1 1v1.06C6.83 3.52 3.52 6.83 3.06 11H2c-.55 0-1 .45-1 1s.45 1 1 1h1.06c.46 4.17 3.77 7.48 7.94 7.94V22c0 .55.45 1 1 1s1-.45 1-1v-1.06c4.17-.46 7.48-3.77 7.94-7.94H22c.55 0 1-.45 1-1s-.45-1-1-1h-1.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

pub fn navigation_icons_24px(props: IconProps) -> Element {
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
                d: "M12.93 4.26l6.15 14.99c.34.83-.51 1.66-1.33 1.29l-5.34-2.36c-.26-.11-.55-.11-.81 0l-5.34 2.36c-.82.36-1.67-.46-1.33-1.29l6.15-14.99c.33-.83 1.51-.83 1.85 0z",
            }
        }
    }
}

pub fn near_me_icons_24px(props: IconProps) -> Element {
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
                d: "M18.75 3.94L4.07 10.08c-.83.35-.81 1.53.02 1.85L9.43 14c.26.1.47.31.57.57l2.06 5.33c.32.84 1.51.86 1.86.03l6.15-14.67c.33-.83-.5-1.66-1.32-1.32z",
            }
        }
    }
}

pub fn near_me_disabled_icons_24px(props: IconProps) -> Element {
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
                d: "M12,6.34l6.95-2.58c0.8-0.3,1.58,0.48,1.29,1.29L17.66,12L12,6.34z M21.9,19.07L4.93,2.1c-0.39-0.39-1.02-0.39-1.41,0 c-0.39,0.39-0.39,1.02,0,1.41l4.36,4.36l-4.2,1.56C3.27,9.59,3,9.97,3,10.4c0,0.42,0.26,0.8,0.65,0.96l6.42,2.57l2.57,6.42 C12.8,20.74,13.18,21,13.6,21c0.43,0,0.82-0.27,0.97-0.67l1.56-4.2l4.36,4.36c0.39,0.39,1.02,0.39,1.41,0 C22.29,20.09,22.29,19.46,21.9,19.07z",
            }
        }
    }
}

pub fn nightlife_icons_20px(props: IconProps) -> Element {
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
                        d: "M8.25,14.5H7.74v-3.54l3.13-4.38c0.47-0.66,0-1.58-0.81-1.58H3.94C3.13,5,2.66,5.92,3.13,6.58l3.12,4.36v3.56H5.75 C5.34,14.5,5,14.84,5,15.25C5,15.66,5.34,16,5.75,16h2.5C8.66,16,9,15.66,9,15.25C9,14.84,8.66,14.5,8.25,14.5z M5.99,8L4.91,6.5 h4.17L8.01,8H5.99z",
                    }
                    path {
                        d: "M16,5h-1c-0.83,0-1.5,0.67-1.5,1.5v4.71c-0.48-0.21-1.03-0.28-1.61-0.14c-0.95,0.23-1.7,1.04-1.86,2.01 c-0.26,1.6,1,2.98,2.56,2.92c1.36-0.05,2.4-1.25,2.4-2.61V7h1c0.55,0,1-0.45,1-1C17,5.45,16.55,5,16,5z",
                    }
                }
            }
        }
    }
}

pub fn nightlife_icons_24px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M2.87,5h10.26c0.8,0,1.28,0.89,0.83,1.55L9,14v4h1c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H6c-0.55,0-1-0.45-1-1v0 c0-0.55,0.45-1,1-1h1v-4L2.04,6.55C1.59,5.89,2.07,5,2.87,5z M10.1,9l1.4-2H4.49l1.4,2H10.1z M19,5h1.5C21.33,5,22,5.67,22,6.5v0 C22,7.33,21.33,8,20.5,8H19v9l0,0c0,1.84-1.64,3.28-3.54,2.95c-1.21-0.21-2.2-1.2-2.41-2.41C12.72,15.64,14.16,14,16,14 c0.35,0,0.69,0.06,1,0.17V7C17,5.9,17.9,5,19,5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn not_listed_location_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M12 2c-4.2 0-8 3.22-8 8.2 0 3.18 2.45 6.92 7.34 11.22.38.33.95.33 1.33 0C17.55 17.12 20 13.38 20 10.2 20 5.22 16.2 2 12 2zm.01 14c-.59 0-1.05-.47-1.05-1.05 0-.59.47-1.04 1.05-1.04.59 0 1.04.45 1.04 1.04 0 .58-.44 1.05-1.04 1.05zm2.51-6.17c-.63.93-1.23 1.21-1.56 1.81-.08.14-.13.26-.16.49-.05.39-.36.68-.75.68h-.03c-.44 0-.79-.38-.75-.82.03-.27.09-.57.25-.84.41-.73 1.18-1.16 1.63-1.8.48-.68.21-1.94-1.14-1.94-.61 0-1.01.32-1.26.7-.19.29-.57.39-.89.25-.42-.18-.6-.7-.34-1.07C10.03 6.55 10.88 6 12 6c1.23 0 2.08.56 2.51 1.26.36.61.58 1.73.01 2.57z",
                }
            }
        }
    }
}

pub fn no_crash_icons_20px(props: IconProps) -> Element {
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
                    d: "M12.41,1.53c0.29,0.29,0.29,0.77,0,1.06L9.93,5.07c-0.29,0.29-0.77,0.29-1.06,0L7.81,4.01c-0.29-0.29-0.29-0.77,0-1.06 c0.29-0.29,0.77-0.29,1.06,0L9.4,3.47l1.94-1.94C11.64,1.24,12.11,1.24,12.41,1.53z M16,19c-0.55,0-1-0.45-1-1v-0.5H5V18 c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1l0-5.71c0-0.19,0.04-0.38,0.11-0.56l1.52-3.79C4.85,7.37,5.4,7,6.02,7h7.97 c0.61,0,1.16,0.37,1.39,0.94l1.52,3.79C16.96,11.91,17,12.1,17,12.29V18C17,18.55,16.55,19,16,19z M6.02,8.5l-1,2.5h9.97l-1-2.5 H6.02z M6.5,13.25c-0.55,0-1,0.45-1,1s0.45,1,1,1c0.55,0,1-0.45,1-1S7.05,13.25,6.5,13.25z M13.5,13.25c-0.55,0-1,0.45-1,1 s0.45,1,1,1c0.55,0,1-0.45,1-1S14.05,13.25,13.5,13.25z",
                }
            }
        }
    }
}

pub fn no_crash_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M19.5,24c0.82,0,1.5-0.67,1.5-1.5v-7.16c0-0.22-0.04-0.45-0.11-0.66l-1.97-5.67C18.72,8.42,18.16,8,17.5,8h-11 C5.84,8,5.29,8.42,5.08,9.01l-1.97,5.67C3.04,14.89,3,15.11,3,15.34v7.16C3,23.33,3.68,24,4.5,24S6,23.33,6,22.5V22h12v0.5 C18,23.33,18.67,24,19.5,24z M6.85,10h10.29l1.04,3H5.81L6.85,10z M6,17.5C6,16.67,6.67,16,7.5,16S9,16.67,9,17.5S8.33,19,7.5,19 S6,18.33,6,17.5z M15,17.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S17.33,19,16.5,19S15,18.33,15,17.5z M16.24,0.71 c0.39,0.39,0.39,1.02,0,1.41l-3.54,3.54c-0.39,0.39-1.02,0.39-1.41,0L9.88,4.24c-0.39-0.39-0.39-1.02,0-1.41l0,0 c0.39-0.39,1.02-0.39,1.41,0L12,3.54l2.83-2.83C15.22,0.32,15.85,0.32,16.24,0.71z",
                }
            }
        }
    }
}

pub fn no_meals_icons_24px(props: IconProps) -> Element {
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
                d: "M21,18.17l-2-2V14h-1c-1.1,0-2-0.9-2-2V6c0-1.49,1.6-3.32,3.76-3.85C20.39,2,21,2.48,21,3.13V18.17z M21.19,22.61 c-0.39,0.39-1.02,0.39-1.41,0l-9.76-9.76C9.69,12.94,9.36,13,9,13v8c0,0.55-0.45,1-1,1s-1-0.45-1-1v-8c-2.21,0-4-1.79-4-4V5.83 L1.39,4.22C1,3.83,1,3.2,1.39,2.81c0.39-0.39,1.02-0.39,1.41,0l18.38,18.38C21.58,21.58,21.58,22.22,21.19,22.61z M6.17,9L5,7.83V9 H6.17z M13,9V3c0-0.55-0.45-1-1-1s-1,0.45-1,1v5.17l1.85,1.85C12.94,9.69,13,9.36,13,9z M9,3c0-0.55-0.45-1-1-1S7,2.45,7,3v1.17l2,2 V3z",
            }
        }
    }
}

pub fn no_transfer_icons_24px(props: IconProps) -> Element {
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
                height: "24",
                fill: "none",
            }
            path {
                d: "M5.78,2.95C7.24,2.16,9.48,2,12,2c4.42,0,8,0.5,8,4v10c0,0.35-0.08,0.67-0.19,0.98L13.83,11H18V6H8.83L5.78,2.95z M20.49,21.9c-0.39,0.39-1.02,0.39-1.41,0l-1.01-1.01C17.89,20.96,17.7,21,17.5,21c-0.83,0-1.5-0.68-1.5-1.5V19H8v0.5 C8,20.33,7.33,21,6.5,21S5,20.33,5,19.5v-1.28C4.39,17.67,4,16.88,4,16V6.83l-1.9-1.9c-0.39-0.39-0.39-1.02,0-1.41 c0.39-0.39,1.02-0.39,1.41,0l16.97,16.97C20.88,20.88,20.88,21.51,20.49,21.9z M9,15.5C9,14.67,8.33,14,7.5,14S6,14.67,6,15.5 C6,16.33,6.67,17,7.5,17S9,16.33,9,15.5z M8.17,11L6,8.83V11H8.17z",
            }
        }
    }
}

pub fn park_icons_20px(props: IconProps) -> Element {
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
                    d: "M13.82,10L13.82,10c0.51,0,0.83-0.57,0.56-1l-3.82-6.11c-0.26-0.41-0.85-0.41-1.11,0L5.63,9c-0.27,0.44,0.04,1,0.56,1h0 l-2.54,3.99C3.36,14.43,3.68,15,4.19,15H8.5v1.5c0,0.83,0.67,1.5,1.5,1.5h0c0.83,0,1.5-0.67,1.5-1.5V15h4.31 c0.52,0,0.83-0.57,0.55-1.01L13.82,10z",
                }
            }
        }
    }
}

pub fn park_icons_24px(props: IconProps) -> Element {
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
            }
            g {
                g {
                    g {
                        path {
                            d: "M16.96,12h0.08c0.81,0,1.28-0.91,0.82-1.57l-5.08-7.25c-0.4-0.57-1.24-0.57-1.64,0L6.1,10.43C5.64,11.09,6.12,12,6.93,12 h0.04l-2.9,4.46C3.63,17.12,4.11,18,4.91,18h5.08v2.02c0,1.09,0.89,1.98,1.98,1.98h0c1.09,0,1.98-0.89,1.98-1.98V18h5.15 c0.8,0,1.28-0.89,0.83-1.55L16.96,12z",
                        }
                    }
                }
            }
        }
    }
}

pub fn pedal_bike_icons_20px(props: IconProps) -> Element {
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
                    d: "M14.82,9l-1.58-4.34C13.1,4.26,12.72,4,12.3,4h-1.8C10.22,4,10,4.22,10,4.5v0C10,4.78,10.22,5,10.5,5h1.8l1.46,4H8.75 L8.38,8H9.5C9.78,8,10,7.78,10,7.5v0C10,7.22,9.78,7,9.5,7h-3C6.22,7,6,7.22,6,7.5v0C6,7.78,6.22,8,6.5,8h0.82l1.46,4H7.95 C7.7,10.22,6.18,8.9,4.29,9.01c-1.68,0.09-3.1,1.43-3.27,3.1C0.8,14.25,2.41,16,4.5,16c1.79,0,3.21-1.29,3.45-3h4.1 c0.25,1.78,1.77,3.1,3.66,2.99c1.68-0.09,3.1-1.43,3.27-3.1C19.2,10.75,17.59,9,15.5,9H14.82z M9.11,10h3.92 c-0.53,0.52-0.88,1.22-0.98,2H9.84L9.11,10z M4.5,15C3.1,15,2,13.9,2,12.5c0-1.4,1.22-2.56,2.62-2.5c1.17,0.05,2.11,0.88,2.33,2 H4.5C4.22,12,4,12.22,4,12.5v0C4,12.78,4.22,13,4.5,13h2.45C6.73,14.15,5.73,15,4.5,15z M15.48,15C14.13,14.99,13,13.85,13,12.5 c0-0.94,0.5-1.73,1.24-2.16l0.86,2.36c0.09,0.26,0.38,0.39,0.64,0.3h0c0.26-0.09,0.39-0.38,0.3-0.64l-0.85-2.33 C15.3,10.02,15.4,10,15.5,10c1.4,0,2.5,1.1,2.5,2.5C18,13.91,16.89,15.01,15.48,15z",
                }
            }
        }
    }
}

pub fn pedal_bike_icons_24px(props: IconProps) -> Element {
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
                    d: "M18.18,10l-1.7-4.68C16.19,4.53,15.44,4,14.6,4H13c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1.6l1.46,4h-4.81l-0.36-1h0.09 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H8C7.45,7,7,7.45,7,8v0c0,0.55,0.45,1,1,1h0.75l1.82,5H9.9c-0.44-2.23-2.31-3.88-4.65-3.99 C2.45,9.87,0,12.2,0,15c0,2.8,2.2,5,5,5c2.46,0,4.45-1.69,4.9-4h4.2c0.44,2.23,2.31,3.88,4.65,3.99c2.8,0.13,5.25-2.19,5.25-5 c0-2.8-2.2-5-5-5H18.18z M7.82,16c-0.42,1.23-1.6,2.08-3.02,1.99C3.31,17.9,2.07,16.64,2,15.14C1.93,13.39,3.27,12,5,12 c1.33,0,2.42,0.83,2.82,2H6c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1H7.82z M14.1,14h-1.4l-0.73-2H15 C14.56,12.58,14.24,13.25,14.1,14z M18.88,18c-1.54-0.06-2.84-1.37-2.88-2.92c-0.02-0.96,0.39-1.8,1.05-2.36l0.62,1.7 c0.19,0.52,0.76,0.79,1.28,0.6l0,0c0.52-0.19,0.79-0.76,0.6-1.28l-0.63-1.73c0,0,0,0,0.01-0.01c1.72-0.04,3.08,1.29,3.08,3 C22,16.72,20.62,18.06,18.88,18z",
                }
            }
        }
    }
}

pub fn person_pin_icons_24px(props: IconProps) -> Element {
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
                d: "M19 2H5c-1.11 0-2 .9-2 2v14c0 1.1.9 2 2 2h4l2.29 2.29c.39.39 1.02.39 1.41 0L15 20h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 3.3c1.49 0 2.7 1.21 2.7 2.7s-1.21 2.7-2.7 2.7S9.3 9.49 9.3 8s1.21-2.7 2.7-2.7zM18 16H6v-.9c0-2 4-3.1 6-3.1s6 1.1 6 3.1v.9z",
            }
        }
    }
}

pub fn person_pin_circle_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M12 2c-4.2 0-8 3.22-8 8.2 0 3.18 2.45 6.92 7.34 11.22.36.32.97.32 1.33 0C17.55 17.12 20 13.38 20 10.2 20 5.22 16.2 2 12 2zM7.69 12.49C8.88 11.56 10.37 11 12 11s3.12.56 4.31 1.49C15.45 13.98 13.85 15 12 15s-3.45-1.02-4.31-2.51zM12 6c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2z",
                }
            }
        }
    }
}

pub fn pest_control_icons_20px(props: IconProps) -> Element {
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
                        d: "M16,11.5L16,11.5c0-0.28-0.22-0.5-0.5-0.5h-1.54c-0.04-0.38-0.11-0.74-0.22-1.08l1.28-0.74c0.24-0.14,0.32-0.44,0.18-0.68 l0,0c-0.14-0.24-0.44-0.32-0.68-0.18L13.33,9c-0.24-0.4-0.54-0.74-0.87-1.03c0.07-0.39,0.13-1.19-0.48-1.99l0.89-0.89 c0.2-0.2,0.2-0.51,0-0.71l0,0c-0.2-0.2-0.51-0.2-0.71,0l-0.94,0.94c-0.41-0.23-1.35-0.61-2.43,0L7.85,4.39 c-0.2-0.2-0.51-0.2-0.71,0l0,0c-0.2,0.2-0.2,0.51,0,0.71l0.89,0.89C7.41,6.78,7.47,7.58,7.55,7.97C7.21,8.26,6.91,8.6,6.67,9 L5.49,8.32C5.25,8.18,4.94,8.26,4.8,8.5l0,0C4.67,8.74,4.75,9.04,4.99,9.18l1.28,0.74c-0.11,0.34-0.18,0.7-0.22,1.08H4.5 C4.22,11,4,11.22,4,11.5v0C4,11.78,4.22,12,4.5,12h1.54c0.04,0.38,0.11,0.74,0.22,1.08l-1.28,0.74c-0.24,0.14-0.32,0.44-0.18,0.68 l0,0c0.14,0.24,0.44,0.32,0.68,0.18L6.67,14c0.72,1.21,1.94,2,3.33,2s2.61-0.8,3.33-2l1.19,0.69c0.24,0.14,0.54,0.06,0.68-0.18 l0,0c0.14-0.24,0.06-0.54-0.18-0.68l-1.28-0.74c0.11-0.34,0.18-0.7,0.22-1.08h1.54C15.78,12,16,11.78,16,11.5z M10,13.5L10,13.5 c-0.28,0-0.5-0.22-0.5-0.5v-3c0-0.28,0.22-0.5,0.5-0.5h0c0.28,0,0.5,0.22,0.5,0.5v3C10.5,13.28,10.28,13.5,10,13.5z",
                    }
                }
            }
        }
    }
}

pub fn pest_control_icons_24px(props: IconProps) -> Element {
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
                    path {
                        d: "M21,14L21,14c0-0.55-0.45-1-1-1h-2.07c-0.05-0.39-0.12-0.77-0.22-1.14l1.72-0.99c0.48-0.28,0.64-0.89,0.37-1.37l0,0 c-0.28-0.48-0.89-0.64-1.37-0.37L16.92,10c-0.28-0.48-0.62-0.91-0.99-1.29C15.97,8.48,16,8.25,16,8c0-0.8-0.24-1.55-0.65-2.18 l0.94-0.94c0.39-0.39,0.39-1.02,0-1.41l0,0c-0.39-0.39-1.02-0.39-1.41,0l-1.02,1.02c-1.68-0.89-3.1-0.33-3.73,0L9.12,3.46 c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l0.94,0.94C8.24,6.45,8,7.2,8,8c0,0.25,0.03,0.48,0.07,0.72 C7.7,9.1,7.36,9.53,7.08,10L5.57,9.13C5.09,8.86,4.48,9.02,4.21,9.5l0,0c-0.28,0.48-0.11,1.09,0.37,1.37l1.72,0.99 c-0.1,0.37-0.17,0.75-0.22,1.14H4c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2.07c0.05,0.39,0.12,0.77,0.22,1.14l-1.72,0.99 c-0.48,0.28-0.64,0.89-0.37,1.37l0,0c0.28,0.48,0.89,0.64,1.37,0.37L7.08,18c1.08,1.81,2.88,3,4.92,3s3.84-1.19,4.92-3l1.51,0.87 c0.48,0.28,1.09,0.11,1.37-0.37l0,0c0.28-0.48,0.11-1.09-0.37-1.37l-1.72-0.99c0.1-0.37,0.17-0.75,0.22-1.14H20 C20.55,15,21,14.55,21,14z M12,17L12,17c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v4C13,16.55,12.55,17,12,17z",
                    }
                }
            }
        }
    }
}

pub fn pest_control_rodent_icons_20px(props: IconProps) -> Element {
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
                    d: "M16.57,13.59l-1.63-1.55C15.31,10.33,13.96,9,12.5,9C11.32,9,10,9.96,10,11.5c0,0.54,0.17,1.03,0.45,1.44 c0.14,0.2,0.13,0.48-0.05,0.65l0,0c-0.21,0.21-0.57,0.2-0.75-0.05C9.24,12.97,9,12.26,9,11.5c0-1.36,0.78-2.52,1.91-3.1 C10.43,8.15,9.91,8,9.36,8C6.44,8,6,10.79,6,11.47c-1.22-0.14-2.15-1.23-1.98-2.53C4.17,7.82,5.19,7,6.33,7l2.08,0 C9.23,7,9.96,6.39,10,5.58C10.04,4.72,9.36,4,8.5,4h-1C7.22,4,7,4.22,7,4.5C7,4.78,7.22,5,7.5,5h1C8.78,5,9,5.22,9,5.5 C9,5.78,8.78,6,8.5,6L6.38,6C4.63,6,3.1,7.33,3,9.07c-0.1,1.79,1.26,3.27,3,3.4C6,13.07,6.34,16,9.36,16h6.24 C16.86,16,17.47,14.45,16.57,13.59z M14.25,15c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75S15,13.84,15,14.25 C15,14.66,14.66,15,14.25,15z",
                }
            }
        }
    }
}

pub fn pest_control_rodent_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M21.31,17.38l-2.39-2.13C19.44,12.89,17.56,11,15.5,11c-1.16,0-3.5,0.9-3.5,3.5c0,0.81,0.27,1.55,0.74,2.15 c0.15,0.2,0.14,0.48-0.04,0.66l0,0c-0.21,0.21-0.56,0.19-0.75-0.04C11.35,16.5,11,15.54,11,14.5c0-1.7,0.96-3.17,2.35-3.93 c-0.7-0.36-1.48-0.57-2.28-0.57c-2.38,0-4.37,1.65-4.91,3.87c-1.33-0.39-2.28-1.66-2.15-3.14C4.15,9.16,5.54,8,7.11,8l2,0 c1.58,0,2.75-0.95,2.87-2.25C12.13,4.25,10.96,3,9.5,3H8.05c-0.5,0-0.96,0.34-1.04,0.83C6.91,4.46,7.39,5,8,5h1.5 C9.78,5,10,5.22,10,5.5C10,5.78,9.78,6,9.5,6L7.16,6c-2.67,0-4.99,2.03-5.15,4.7C1.86,13.25,3.62,15.42,6,15.9v0.03 C6,18.73,8.27,21,11.07,21h8.86C21.8,21,22.74,18.66,21.31,17.38z M18,19c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C19,18.55,18.55,19,18,19z",
                }
            }
        }
    }
}

pub fn pin_drop_icons_24px(props: IconProps) -> Element {
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
                        d: "M6 20h12c.55 0 1 .45 1 1s-.45 1-1 1H6c-.55 0-1-.45-1-1s.45-1 1-1zm6-13c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-5c3.27 0 7 2.46 7 7.15 0 2.98-2.13 6.12-6.39 9.39-.36.28-.86.28-1.22 0C7.13 15.26 5 12.13 5 9.15 5 4.46 8.73 2 12 2z",
                    }
                }
            }
        }
    }
}

pub fn place_icons_24px(props: IconProps) -> Element {
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
                    enable_background: "new",
                    d: "M12 2c-4.2 0-8 3.22-8 8.2 0 3.18 2.45 6.92 7.34 11.23.38.33.95.33 1.33 0C17.55 17.12 20 13.38 20 10.2 20 5.22 16.2 2 12 2zm0 10c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
                }
            }
        }
    }
}

pub fn plumbing_icons_20px(props: IconProps) -> Element {
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
                        d: "M13.29,4.45l2.83,2.83l0,0c0.78-0.78,0.78-2.05,0-2.83l-2.12-2.12c-0.39-0.39-1.02-0.39-1.41,0L9.75,5.15l1.41,1.41 L13.29,4.45z",
                    }
                    path {
                        d: "M4.8,11.52L4.8,11.52c0.39,0.39,1.02,0.39,1.41,0L8.34,9.4L6.92,7.98L4.8,10.1C4.41,10.49,4.41,11.13,4.8,11.52z",
                    }
                    path {
                        d: "M12.22,6.92l-1.06,1.06L8.34,5.15c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41L9.75,9.4L9.04,10.1 l-5.3,5.3c-0.59,0.59-0.59,1.54,0,2.12l0,0c0.59,0.59,1.54,0.59,2.12,0l7.42-7.42l0,0c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41l-1.77-1.77C12.74,6.73,12.42,6.73,12.22,6.92z",
                    }
                }
            }
        }
    }
}

pub fn plumbing_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M19.28,4.93l-2.12-2.12c-0.78-0.78-2.05-0.78-2.83,0L11.5,5.64l2.12,2.12l2.12-2.12l3.54,3.54 C20.45,8,20.45,6.1,19.28,4.93z",
                    }
                    path {
                        d: "M5.49,13.77c0.59,0.59,1.54,0.59,2.12,0l2.47-2.47L7.96,9.17l-2.47,2.47C4.9,12.23,4.9,13.18,5.49,13.77L5.49,13.77z",
                    }
                    path {
                        d: "M14.33,8.46l-0.71,0.71l-3.18-3.18C9.85,5.4,8.9,5.4,8.32,5.99c-0.59,0.59-0.59,1.54,0,2.12l3.18,3.18l-7,7 c-0.7,0.7-0.88,1.84-0.29,2.65c0.74,1.03,2.19,1.12,3.05,0.26L16.45,12c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41 l-2.12-2.12C15.35,8.07,14.72,8.07,14.33,8.46z",
                    }
                }
            }
        }
    }
}

pub fn railway_alert_icons_24px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M4,11V8h7.29c-0.77-2.6,0.21-4.61,0.37-4.97C2.97,2.67,2,5.02,2,7v9.5C2,18.43,3.57,20,5.5,20l-1.21,0.81 C4.11,20.93,4,21.13,4,21.35v0C4,21.71,4.29,22,4.65,22h10.7c0.36,0,0.65-0.29,0.65-0.65v0c0-0.22-0.11-0.42-0.29-0.54L14.5,20 c1.93,0,3.5-1.57,3.5-3.5V13c-1.91,0-3.63-0.76-4.89-2H4z M10,17c-0.83,0-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5C11.5,16.33,10.83,17,10,17z",
                        }
                    }
                    g {
                        path {
                            d: "M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18,9c-0.28,0-0.5-0.22-0.5-0.5C17.5,8.22,17.72,8,18,8 s0.5,0.22,0.5,0.5C18.5,8.78,18.28,9,18,9z M18.5,6.5C18.5,6.78,18.28,7,18,7s-0.5-0.22-0.5-0.5v-3C17.5,3.22,17.72,3,18,3 s0.5,0.22,0.5,0.5V6.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn ramen_dining_icons_20px(props: IconProps) -> Element {
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
                    d: "M18,2.56L18,2.56c0-0.3-0.26-0.53-0.55-0.5L4.89,3.44C4.38,3.5,4,3.93,4,4.44V9L3.07,9c-0.61,0-1.08,0.55-0.99,1.15 C2.5,13,4.41,15.36,7,16.41V17c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-0.59c2.59-1.05,4.5-3.41,4.92-6.26 C18.01,9.55,17.54,9,16.93,9L9,9V7h8.5C17.78,7,18,6.78,18,6.5v0C18,6.22,17.78,6,17.5,6H9V3.99l8.55-0.94 C17.81,3.03,18,2.81,18,2.56z M5,4.43l1-0.11V6H5V4.43z M5,7h1v2H5V7z M8,9H7V7h1V9z M8,6H7V4.21L8,4.1V6z",
                }
            }
        }
    }
}

pub fn ramen_dining_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M22,2.84L22,2.84c0-0.45-0.39-0.79-0.83-0.75L4.89,3.9C4.38,3.95,4,4.38,4,4.89V12H3.08c-0.6,0-1.08,0.53-1,1.13 C2.52,16.33,4.83,19,8,20.25V21c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-0.75c3.17-1.25,5.48-3.92,5.92-7.12 c0.08-0.6-0.4-1.13-1-1.13H10.5V8h10.75C21.66,8,22,7.66,22,7.25v0c0-0.41-0.34-0.75-0.75-0.75H10.5V4.78l10.83-1.19 C21.71,3.54,22,3.22,22,2.84z M6.5,5.22V6.5h-1V5.34L6.5,5.22z M5.5,8h1v4h-1V8z M9,12H8V8h1V12z M9,6.5H8V5.06l1-0.11V6.5z",
                }
            }
        }
    }
}

pub fn ramp_left_icons_20px(props: IconProps) -> Element {
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
                    d: "M10.75,5.87l0.66,0.66c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06l-1.76-1.76c-0.39-0.39-1.02-0.39-1.41,0 L7.53,5.47c-0.29,0.29-0.29,0.77,0,1.06c0.29,0.29,0.77,0.29,1.06,0l0.66-0.66l0,10.38C9.25,16.66,9.59,17,10,17 s0.75-0.34,0.75-0.75V12c0.85,1.16,1.93,2.08,2.87,2.74c0.3,0.21,0.7,0.17,0.96-0.08c0.33-0.33,0.28-0.88-0.1-1.14 c-1.61-1.12-3.72-3.14-3.72-5.81L10.75,5.87z",
                }
            }
        }
    }
}

pub fn ramp_left_icons_24px(props: IconProps) -> Element {
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
                    d: "M12,21c-0.55,0-1-0.45-1-1V6.83l-0.88,0.88C9.73,8.1,9.1,8.1,8.71,7.71c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59 c0.39-0.39,1.02-0.39,1.41,0l2.59,2.59c0.39,0.39,0.39,1.02,0,1.41c-0.39,0.39-1.02,0.39-1.41,0L13,6.83v0V9 c0,3.62,2.89,6.22,4.97,7.62c0.52,0.35,0.59,1.09,0.14,1.53c-0.33,0.33-0.87,0.4-1.26,0.13c-1.59-1.06-2.89-2.28-3.85-3.59l0,5.3 C13,20.55,12.55,21,12,21z",
                }
            }
        }
    }
}

pub fn ramp_right_icons_20px(props: IconProps) -> Element {
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
                    d: "M9.25,5.87L8.59,6.53c-0.29,0.29-0.77,0.29-1.06,0c-0.29-0.29-0.29-0.77,0-1.06l1.76-1.76c0.39-0.39,1.02-0.39,1.41,0 l1.76,1.76c0.29,0.29,0.29,0.77,0,1.06c-0.29,0.29-0.77,0.29-1.06,0l-0.66-0.66l0,10.38c0,0.41-0.34,0.75-0.75,0.75 s-0.75-0.34-0.75-0.75V12c-0.85,1.16-1.93,2.08-2.87,2.74c-0.3,0.21-0.7,0.17-0.96-0.08c-0.33-0.33-0.28-0.88,0.1-1.14 c1.61-1.12,3.72-3.14,3.72-5.81L9.25,5.87z",
                }
            }
        }
    }
}

pub fn ramp_right_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M12,21c0.55,0,1-0.45,1-1V6.83l0.88,0.88c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59 c-0.39-0.39-1.02-0.39-1.41,0L8.71,6.29c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0L11,6.83v0V9 c0,3.62-2.89,6.22-4.97,7.62c-0.52,0.35-0.59,1.09-0.14,1.53c0.33,0.33,0.87,0.4,1.26,0.13c1.59-1.06,2.89-2.28,3.85-3.59l0,5.3 C11,20.55,11.45,21,12,21z",
                }
            }
        }
    }
}

pub fn rate_review_icons_24px(props: IconProps) -> Element {
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
                d: "M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 14v-2.47l6.88-6.88c.2-.2.51-.2.71 0l1.77 1.77c.2.2.2.51 0 .71L8.47 14H6zm11 0h-6.5l2-2H17c.55 0 1 .45 1 1s-.45 1-1 1z",
            }
        }
    }
}

pub fn remove_road_icons_20px(props: IconProps) -> Element {
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
                        d: "M4.75,16L4.75,16c0.41,0,0.75-0.34,0.75-0.75V4.75C5.5,4.34,5.16,4,4.75,4h0C4.34,4,4,4.34,4,4.75v10.5 C4,15.66,4.34,16,4.75,16z",
                    }
                    path {
                        d: "M15.25,4L15.25,4c-0.41,0-0.75,0.34-0.75,0.75v5c0,0.41,0.34,0.75,0.75,0.75h0c0.41,0,0.75-0.34,0.75-0.75v-5 C16,4.34,15.66,4,15.25,4z",
                    }
                    path {
                        d: "M10,11.5L10,11.5c0.41,0,0.75-0.34,0.75-0.75v-1.5c0-0.41-0.34-0.75-0.75-0.75h0c-0.41,0-0.75,0.34-0.75,0.75v1.5 C9.25,11.16,9.59,11.5,10,11.5z",
                    }
                    path {
                        d: "M10,7L10,7c0.41,0,0.75-0.34,0.75-0.75v-1.5C10.75,4.34,10.41,4,10,4h0C9.59,4,9.25,4.34,9.25,4.75v1.5 C9.25,6.66,9.59,7,10,7z",
                    }
                    path {
                        d: "M10,16L10,16c0.41,0,0.75-0.34,0.75-0.75v-1.5c0-0.41-0.34-0.75-0.75-0.75h0c-0.41,0-0.75,0.34-0.75,0.75v1.5 C9.25,15.66,9.59,16,10,16z",
                    }
                    path {
                        d: "M17.47,13.03L17.47,13.03c-0.29-0.29-0.77-0.29-1.06,0l-1.16,1.16l-1.16-1.16c-0.29-0.29-0.77-0.29-1.06,0l0,0 c-0.29,0.29-0.29,0.77,0,1.06l1.16,1.16l-1.16,1.16c-0.29,0.29-0.29,0.77,0,1.06h0c0.29,0.29,0.77,0.29,1.06,0l1.16-1.16 l1.16,1.16c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06l-1.16-1.16l1.16-1.16C17.76,13.8,17.76,13.32,17.47,13.03z",
                    }
                }
            }
        }
    }
}

pub fn remove_road_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M19,4L19,4c-0.55,0-1,0.45-1,1v7c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V5C20,4.45,19.55,4,19,4z",
                    }
                    path {
                        d: "M5,20L5,20c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h0C4.45,4,4,4.45,4,5v14C4,19.55,4.45,20,5,20z",
                    }
                    path {
                        d: "M12,8L12,8c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C11,7.55,11.45,8,12,8z",
                    }
                    path {
                        d: "M12,14L12,14c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C11,13.55,11.45,14,12,14z",
                    }
                    path {
                        d: "M12,20L12,20c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2C11,19.55,11.45,20,12,20z",
                    }
                    path {
                        d: "M21.79,15.71L21.79,15.71c-0.39-0.39-1.02-0.39-1.41,0L19,17.09l-1.38-1.38c-0.39-0.39-1.02-0.39-1.41,0v0 c-0.39,0.39-0.39,1.02,0,1.41l1.38,1.38l-1.38,1.38c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0L19,19.91 l1.38,1.38c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41l-1.38-1.38l1.38-1.38C22.18,16.73,22.18,16.1,21.79,15.71z",
                    }
                }
            }
        }
    }
}

pub fn restaurant_icons_24px(props: IconProps) -> Element {
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
                d: "M16 6v6c0 1.1.9 2 2 2h1v7c0 .55.45 1 1 1s1-.45 1-1V3.13c0-.65-.61-1.13-1.24-.98C17.6 2.68 16 4.51 16 6zm-5 3H9V3c0-.55-.45-1-1-1s-1 .45-1 1v6H5V3c0-.55-.45-1-1-1s-1 .45-1 1v6c0 2.21 1.79 4 4 4v8c0 .55.45 1 1 1s1-.45 1-1v-8c2.21 0 4-1.79 4-4V3c0-.55-.45-1-1-1s-1 .45-1 1v6z",
            }
        }
    }
}

pub fn restaurant_menu_icons_24px(props: IconProps) -> Element {
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
                d: "M8.1 13.34l2.83-2.83-6.19-6.18c-.48-.48-1.31-.35-1.61.27-.71 1.49-.45 3.32.78 4.56l4.19 4.18zm6.78-1.81c1.53.71 3.68.21 5.27-1.38 1.91-1.91 2.28-4.65.81-6.12-1.46-1.46-4.2-1.1-6.12.81-1.59 1.59-2.09 3.74-1.38 5.27L4.4 19.17c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 14.41l6.18 6.18c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 13l1.47-1.47z",
            }
        }
    }
}

pub fn roundabout_left_icons_20px(props: IconProps) -> Element {
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
                    d: "M13.25,3c-2.37,0-4.33,1.73-4.69,4L4.87,7l0.66-0.66c0.29-0.29,0.29-0.77,0-1.06c-0.29-0.29-0.77-0.29-1.06,0L2.71,7.04 c-0.39,0.39-0.39,1.02,0,1.41l1.76,1.76c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06L4.87,8.5l3.69,0 c0.74,0,1.36-0.54,1.48-1.25c0.24-1.56,1.59-2.75,3.21-2.75l0,0c1.79,0,3.25,1.46,3.25,3.25c0,1.62-1.18,2.96-2.73,3.21 c-0.72,0.11-1.27,0.67-1.27,1.4l0,3.89c0,0.41,0.34,0.75,0.75,0.75c0.41,0,0.75-0.34,0.75-0.75v-3.81l0,0c2.27-0.36,4-2.32,4-4.69 C18,5.13,15.87,3,13.25,3",
                }
            }
        }
    }
}

pub fn roundabout_left_icons_24px(props: IconProps) -> Element {
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
                    d: "M16,21c-0.55,0-1-0.45-1-1l0-5.09c0-0.98,0.71-1.8,1.67-1.97C18.56,12.63,20,10.98,20,9c0-2.21-1.79-4-4-4 c-1.98,0-3.63,1.44-3.94,3.33C11.89,9.29,11.07,10,10.09,10l-4.26,0l0.88,0.88c0.39,0.39,0.39,1.02,0,1.41 c-0.39,0.39-1.02,0.39-1.41,0L2.71,9.71c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59c0.39-0.39,1.02-0.39,1.41,0 c0.39,0.39,0.39,1.02,0,1.41L5.83,8l4.25,0c0.48-2.84,2.94-5,5.92-5c3.31,0,6,2.69,6,6c0,2.97-2.16,5.44-5,5.92L17,20 C17,20.55,16.55,21,16,21z",
                }
            }
        }
    }
}

pub fn roundabout_right_icons_20px(props: IconProps) -> Element {
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
                    d: "M6.75,3c2.37,0,4.33,1.73,4.69,4l3.69,0l-0.66-0.66c-0.29-0.29-0.29-0.77,0-1.06c0.29-0.29,0.77-0.29,1.06,0l1.76,1.76 c0.39,0.39,0.39,1.02,0,1.41l-1.76,1.76c-0.29,0.29-0.77,0.29-1.06,0c-0.29-0.29-0.29-0.77,0-1.06l0.66-0.66l-3.69,0 c-0.74,0-1.36-0.54-1.48-1.25C9.72,5.69,8.37,4.5,6.75,4.5l0,0C4.96,4.5,3.5,5.96,3.5,7.75c0,1.62,1.18,2.96,2.73,3.21 c0.72,0.11,1.27,0.67,1.27,1.4l0,3.89C7.5,16.66,7.16,17,6.75,17C6.34,17,6,16.66,6,16.25v-3.81l0,0c-2.27-0.36-4-2.32-4-4.69 C2,5.13,4.13,3,6.75,3",
                }
            }
        }
    }
}

pub fn roundabout_right_icons_24px(props: IconProps) -> Element {
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
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M8,21c0.55,0,1-0.45,1-1l0-5.09c0-0.98-0.71-1.8-1.67-1.97C5.44,12.63,4,10.98,4,9c0-2.21,1.79-4,4-4 c1.98,0,3.63,1.44,3.94,3.33C12.11,9.29,12.93,10,13.91,10l4.26,0l-0.88,0.88c-0.39,0.39-0.39,1.02,0,1.41 c0.39,0.39,1.02,0.39,1.41,0l2.59-2.59c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59c-0.39-0.39-1.02-0.39-1.41,0 c-0.39,0.39-0.39,1.02,0,1.41L18.17,8l-4.25,0C13.44,5.16,10.97,3,8,3C4.69,3,2,5.69,2,9c0,2.97,2.16,5.44,5,5.92L7,20 C7,20.55,7.45,21,8,21z",
                }
            }
        }
    }
}

pub fn route_icons_20px(props: IconProps) -> Element {
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
                        d: "M15.25,12.13V6c0-1.65-1.35-3-3-3s-3,1.35-3,3v8c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5V7.87C7.26,7.55,8,6.62,8,5.5 C8,4.12,6.88,3,5.5,3S3,4.12,3,5.5c0,1.12,0.74,2.05,1.75,2.37V14c0,1.65,1.35,3,3,3s3-1.35,3-3V6c0-0.83,0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5v6.13C12.74,12.45,12,13.38,12,14.5c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5C17,13.38,16.26,12.45,15.25,12.13 z",
                    }
                }
            }
        }
    }
}

pub fn route_icons_24px(props: IconProps) -> Element {
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
                        d: "M19,15.18V7c0-2.21-1.79-4-4-4s-4,1.79-4,4v10c0,1.1-0.9,2-2,2s-2-0.9-2-2V8.82C8.16,8.4,9,7.3,9,6c0-1.66-1.34-3-3-3 S3,4.34,3,6c0,1.3,0.84,2.4,2,2.82V17c0,2.21,1.79,4,4,4s4-1.79,4-4V7c0-1.1,0.9-2,2-2s2,0.9,2,2v8.18c-1.16,0.41-2,1.51-2,2.82 c0,1.66,1.34,3,3,3s3-1.34,3-3C21,16.7,20.16,15.6,19,15.18z",
                    }
                }
            }
        }
    }
}

pub fn run_circle_icons_20px(props: IconProps) -> Element {
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
                        d: "M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C17,6.13,13.87,3,10,3z M11,5c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1 s-1-0.45-1-1C10,5.45,10.45,5,11,5z M12.88,10.53c-0.64-0.16-1.47-0.57-2.05-1.31l-0.21,1.19l1.24,1.24 C11.95,11.74,12,11.87,12,12v2.5c0,0.28-0.22,0.5-0.5,0.5h0c-0.28,0-0.5-0.22-0.5-0.5v-2.29l-1-0.99l-0.33,1.64 c-0.05,0.27-0.32,0.45-0.59,0.39L6.9,12.79c-0.27-0.06-0.45-0.32-0.39-0.59l0,0c0.06-0.27,0.32-0.44,0.59-0.39l1.7,0.35l0.62-3.12 L8.5,9.37v0.76c0,0.27-0.22,0.49-0.49,0.49H7.99c-0.27,0-0.49-0.22-0.49-0.49V9.02c0-0.21,0.13-0.4,0.33-0.47l2.34-0.84 c0.46-0.17,1.01,0.06,1.25,0.49c0.52,0.94,1.23,1.27,1.7,1.37c0.22,0.05,0.38,0.25,0.38,0.48v0 C13.5,10.37,13.19,10.61,12.88,10.53z",
                    }
                }
            }
        }
    }
}

pub fn run_circle_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M13.5,6c0.55,0,1,0.45,1,1 c0,0.55-0.45,1-1,1s-1-0.45-1-1C12.5,6.45,12.95,6,13.5,6z M15.41,11.91c-0.71-0.2-1.63-0.74-2.32-1.66l-0.41,2.35l1.19,1.3 C13.95,13.98,14,14.1,14,14.22v3.28c0,0.28-0.22,0.5-0.5,0.5h0c-0.28,0-0.5-0.22-0.5-0.5v-3.08l-1.11-1.21l-0.43,2.15 c-0.05,0.27-0.32,0.45-0.59,0.39l-2.78-0.57c-0.27-0.06-0.45-0.32-0.39-0.59v0c0.06-0.27,0.32-0.44,0.59-0.39l2.29,0.47l0.96-4.89 L10,10.35v1.15c0,0.28-0.22,0.5-0.5,0.5h0C9.22,12,9,11.78,9,11.5V10c0-0.21,0.13-0.4,0.33-0.47l2.95-1.09 c0.49-0.18,1.02,0.04,1.25,0.51c0.65,1.35,1.55,1.85,2.1,2C15.85,11,16,11.18,16,11.4v0.04C16,11.75,15.71,11.99,15.41,11.91z",
                }
            }
        }
    }
}

pub fn safety_check_icons_20px(props: IconProps) -> Element {
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
                    d: "M9.46,2.21l-5,1.92C3.88,4.35,3.5,4.91,3.5,5.53v3.74c0,3.88,2.56,7.52,6.07,8.61c0.28,0.09,0.58,0.09,0.86,0 c3.51-1.09,6.07-4.73,6.07-8.61V5.53c0-0.62-0.38-1.18-0.96-1.4l-5-1.92C10.19,2.07,9.81,2.07,9.46,2.21z M10,14 c-2.21,0-4-1.79-4-4c0-2.21,1.79-4,4-4s4,1.79,4,4C14,12.21,12.21,14,10,14z M11.5,11.5c-0.2,0.2-0.51,0.2-0.71,0l-1.15-1.15 C9.55,10.26,9.5,10.13,9.5,10V8.5C9.5,8.22,9.72,8,10,8c0.28,0,0.5,0.22,0.5,0.5v1.29l1,1C11.7,10.99,11.7,11.3,11.5,11.5z",
                }
            }
        }
    }
}

pub fn safety_check_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M11.3,2.26l-6,2.25C4.52,4.81,4,5.55,4,6.39v4.7c0,4.83,3.13,9.37,7.43,10.75c0.37,0.12,0.77,0.12,1.14,0 c4.3-1.38,7.43-5.91,7.43-10.75v-4.7c0-0.83-0.52-1.58-1.3-1.87l-6-2.25C12.25,2.09,11.75,2.09,11.3,2.26z M12,17 c-2.76,0-5-2.24-5-5s2.24-5,5-5s5,2.24,5,5S14.76,17,12,17z M14,14c-0.2,0.2-0.51,0.2-0.71,0l-1.65-1.65 c-0.09-0.09-0.15-0.22-0.15-0.35V9.5C11.5,9.22,11.72,9,12,9c0.28,0,0.5,0.22,0.5,0.5v2.29l1.5,1.5C14.2,13.49,14.2,13.8,14,14z",
                }
            }
        }
    }
}

pub fn sailing_icons_20px(props: IconProps) -> Element {
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
                d: "M3.54,10.72c-0.23,0.33,0.01,0.78,0.41,0.78H9c0.28,0,0.5-0.22,0.5-0.5l0-7.38c0-0.49-0.64-0.69-0.91-0.28L3.54,10.72z M11.47,1.34c-0.38-0.22-0.84,0.12-0.74,0.55c0.24,0.99,0.56,2.7,0.56,4.61c0,1.99-0.35,3.54-0.59,4.36 c-0.09,0.32,0.15,0.64,0.48,0.64h5.3c0.28,0,0.51-0.24,0.5-0.52C16.76,6.2,13.43,2.45,11.47,1.34z M16.08,15.55 c-0.26-0.17-0.5-0.37-0.71-0.6c-0.2-0.21-0.54-0.21-0.73,0C14.06,15.56,13.32,16,12.5,16c-0.82,0-1.56-0.44-2.13-1.06 c-0.2-0.21-0.53-0.21-0.73,0C9.06,15.56,8.32,16,7.5,16c-0.82,0-1.56-0.44-2.13-1.06c-0.2-0.21-0.54-0.21-0.73,0 c-0.22,0.23-0.46,0.44-0.71,0.6c-0.8-0.59-1.42-1.42-1.75-2.39c-0.11-0.32,0.13-0.66,0.48-0.66h14.71c0.34,0,0.58,0.33,0.48,0.66 C17.5,14.13,16.88,14.96,16.08,15.55z M5,18.4c-0.7,0.36-1.46,0.55-2.22,0.59C2.39,19.01,2,18.7,2,18.24c0-0.39,0.3-0.72,0.71-0.75 c0.68-0.04,1.36-0.29,1.98-0.75c0.18-0.14,0.43-0.14,0.61,0c1.37,1.01,3.02,1.01,4.39,0c0.19-0.14,0.43-0.14,0.61,0 c1.37,1.01,3.02,1,4.39,0c0.18-0.14,0.43-0.14,0.61,0c0.62,0.46,1.3,0.7,1.98,0.75C17.7,17.52,18,17.85,18,18.24 c0,0.46-0.39,0.77-0.78,0.75c-0.76-0.04-1.52-0.24-2.22-0.59c-1.57,0.8-3.43,0.8-5,0C8.43,19.2,6.57,19.2,5,18.4z",
            }
        }
    }
}

pub fn sailing_icons_24px(props: IconProps) -> Element {
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
                d: "M11,13V3.59c0-0.49-0.63-0.69-0.91-0.29l-6.54,9.41c-0.23,0.33,0.01,0.79,0.41,0.79h6.54C10.78,13.5,11,13.28,11,13z M20.99,12.98C20.72,7.07,15.9,2.32,13.4,1.23c-0.37-0.16-0.77,0.2-0.67,0.59c0.3,1.13,0.76,3.28,0.76,5.68 c0,2.44-0.49,4.39-0.78,5.35c-0.1,0.32,0.14,0.65,0.48,0.65h7.28C20.76,13.5,21,13.26,20.99,12.98z M20.62,15H3.38 c-0.73,0-1.22,0.76-0.92,1.42c0.43,0.92,1.07,1.71,1.86,2.31c0.38-0.16,0.74-0.38,1.06-0.63c0.35-0.29,0.87-0.29,1.23,0 C7.28,18.63,8.1,19,9,19c0.9,0,1.72-0.37,2.39-0.91c0.35-0.28,0.87-0.28,1.22,0C13.28,18.63,14.1,19,15,19 c0.9,0,1.72-0.37,2.39-0.91c0.35-0.29,0.87-0.28,1.23,0c0.32,0.26,0.67,0.48,1.06,0.63c0.79-0.6,1.43-1.39,1.86-2.31 C21.84,15.76,21.35,15,20.62,15z M22,22c0-0.55-0.45-1-1-1h0c-0.87,0-1.73-0.24-2.53-0.7c-0.29-0.16-0.65-0.17-0.94,0 c-1.59,0.9-3.47,0.9-5.06,0c-0.29-0.16-0.65-0.16-0.94,0c-1.59,0.9-3.47,0.9-5.06,0c-0.29-0.16-0.65-0.16-0.94,0 C4.73,20.76,3.87,21,3,21h0c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h0c1.15,0,2.3-0.31,3.33-0.94c1.66,1.11,3.78,1.01,5.58,0.14 c1.91,1.05,4.17,1.07,6.09,0.05h0c0.95,0.5,1.97,0.75,3,0.75h0C21.55,23,22,22.55,22,22z",
            }
        }
    }
}

pub fn satellite_icons_24px(props: IconProps) -> Element {
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM5 5h3c0 1.66-1.34 3-3 3V5zm0 5.91c0-.49.36-.9.85-.98 2.08-.36 3.72-2 4.08-4.08.08-.49.49-.85.98-.85.61 0 1.09.53 1 1.13-.48 2.96-2.81 5.3-5.77 5.78-.6.1-1.14-.39-1.14-1zm.63 6.28l2.49-3.2c.2-.25.58-.26.78-.01l2.1 2.53 3.1-3.99c.2-.26.6-.26.8.01l3.51 4.68c.25.33.01.8-.4.8H6.02c-.41-.01-.65-.49-.39-.82z",
            }
        }
    }
}

pub fn screen_rotation_alt_icons_20px(props: IconProps) -> Element {
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
                    d: "M2.75,4.5c0.41,0,0.75,0.34,0.75,0.75v1.19l4-4c0.59-0.59,1.54-0.59,2.12,0l5.28,5.28C15.37,8.19,15.04,9,14.37,9 c-0.2,0-0.39-0.08-0.53-0.22L8.56,3.5l-4,4h1.19c0.41,0,0.75,0.34,0.75,0.75S6.16,9,5.75,9H3C2.45,9,2,8.55,2,8V5.25 C2,4.84,2.34,4.5,2.75,4.5z M17.25,15.5c-0.41,0-0.75-0.34-0.75-0.75v-1.19l-4,4c-0.59,0.59-1.54,0.59-2.12,0L5.1,12.28 C4.63,11.81,4.96,11,5.63,11c0.2,0,0.39,0.08,0.53,0.22l5.28,5.28l4-4h-1.19c-0.41,0-0.75-0.34-0.75-0.75 c0-0.41,0.34-0.75,0.75-0.75H17c0.55,0,1,0.45,1,1v2.75C18,15.16,17.66,15.5,17.25,15.5z",
                }
            }
        }
    }
}

pub fn screen_rotation_alt_icons_24px(props: IconProps) -> Element {
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
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18.53,9.29C19.16,9.92,18.71,11,17.82,11c-0.27,0-0.52-0.11-0.71-0.29L10.4,4L5.41,9H7c0.55,0,1,0.45,1,1s-0.45,1-1,1H3 c-0.55,0-1-0.45-1-1V6c0-0.55,0.45-1,1-1s1,0.45,1,1v1.59l5-5c0.78-0.78,2.05-0.78,2.83,0L18.53,9.29z",
                }
                path {
                    d: "M5.47,14.71C4.84,14.08,5.29,13,6.18,13c0.27,0,0.52,0.11,0.71,0.29L13.6,20l4.99-5H17c-0.55,0-1-0.45-1-1s0.45-1,1-1h4 c0.55,0,1,0.45,1,1v4c0,0.55-0.45,1-1,1s-1-0.45-1-1v-1.59l-5,5c-0.78,0.78-2.05,0.78-2.83,0L5.47,14.71z",
                }
            }
        }
    }
}

pub fn set_meal_icons_24px(props: IconProps) -> Element {
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
                d: "M20.3,17.6L3.83,18.46c-0.41,0.02-0.77-0.3-0.79-0.71l0,0c-0.02-0.41,0.3-0.77,0.71-0.79l16.48-0.86 c0.41-0.02,0.77,0.3,0.79,0.71v0C21.04,17.22,20.72,17.58,20.3,17.6z M20.25,19.48H3.75C3.34,19.48,3,19.82,3,20.23l0,0 c0,0.41,0.34,0.75,0.75,0.75h16.5c0.41,0,0.75-0.34,0.75-0.75l0,0C21,19.82,20.66,19.48,20.25,19.48z M22,5v7c0,1.1-0.9,2-2,2H4 c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2h16C21.1,3,22,3.9,22,5z M19.12,6.09c-1.25,0.27-2.19,1.11-2.33,2.14 C16.15,7.5,14.06,5.5,10.25,5.5c-3.44,0-5.48,1.63-6.31,2.49c-0.28,0.29-0.28,0.74,0,1.03c0.83,0.86,2.87,2.49,6.31,2.49 c3.81,0,5.9-2,6.54-2.73c0.14,1.02,1.08,1.86,2.33,2.14c0.46,0.1,0.88-0.28,0.88-0.74V6.84C20,6.37,19.57,5.99,19.12,6.09z",
            }
        }
    }
}

pub fn signpost_icons_20px(props: IconProps) -> Element {
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
                    d: "M10.75,8H15l2-2.25L15,3.5h-4.25V2h-1.5v1.5H4V8h5.25v1.5H5l-2,2.25L5,14h4.25v4h1.5v-4H16V9.5h-5.25V8z",
                }
            }
        }
    }
}

pub fn signpost_icons_24px(props: IconProps) -> Element {
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
                    polygon {
                        points: "13,10 18,10 21,7 18,4 13,4 13,2 11,2 11,4 4,4 4,10 11,10 11,12 6,12 3,15 6,18 11,18 11,22 13,22 13,18 20,18 20,12 13,12",
                    }
                }
            }
        }
    }
}

pub fn snowmobile_icons_20px(props: IconProps) -> Element {
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
                d: "M19.08,14.5c-0.29,0-0.55,0.17-0.68,0.43c-0.16,0.34-0.5,0.57-0.9,0.57l-2.22-2.19C16.87,12.96,18,12.17,18,11 c0-0.44-4.96-5.08-6.22-6.24C11.6,4.59,11.36,4.5,11.11,4.5H9.75C9.34,4.5,9,4.84,9,5.25C9,5.66,9.34,6,9.75,6h1.16l1.22,1.14L9.1,9 L1.51,8.52C0.69,8.46,0,9.11,0,9.94c0,0.63,0.42,1.19,1.02,1.36l3.3,0.97l-3.53,1.91C-0.58,14.92-0.06,17,1.5,17h5 c1.93,0,3.5-1.57,3.5-3.5h3.33l2.05,2h-2.13c-0.41,0-0.75,0.34-0.75,0.75c0,0.41,0.34,0.75,0.75,0.75h4.25c1,0,1.86-0.59,2.26-1.44 C20,15.07,19.62,14.5,19.08,14.5z M6.5,15.5h-5l4.87-2.63L8.5,13.5C8.5,14.6,7.6,15.5,6.5,15.5z",
            }
        }
    }
}

pub fn snowmobile_icons_24px(props: IconProps) -> Element {
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
                d: "M11,6c0,0.55,0.45,1,1,1h1.25l1.45,1.3L11,11l-9.12-0.96C0.88,9.93,0,10.72,0,11.73c0,0.75,0.49,1.41,1.21,1.63l3.33,1 l-3.49,1.88C-0.77,17.22-0.07,20,2,20h6c2.21,0,4-1.79,4-4h4l2,2h-2c-0.55,0-1,0.45-1,1s0.45,1,1,1h5c1.13,0,2.11-0.62,2.63-1.55 c0.36-0.65-0.15-1.45-0.9-1.45c-0.34,0-0.68,0.16-0.84,0.47C21.72,17.78,21.38,18,21,18h-0.17l-2.2-2.2C20.58,15.37,22,14.4,22,13 c0-0.89-7.72-7.75-7.72-7.75C14.1,5.09,13.87,5,13.62,5H12C11.45,5,11,5.45,11,6z M8,18H2l5.25-2.83L10,16C10,17.1,9.11,18,8,18z",
            }
        }
    }
}

pub fn sos_icons_20px(props: IconProps) -> Element {
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
                    d: "M7.38,12c0,0.82,0.68,1.5,1.5,1.5h2.25c0.82,0,1.5-0.68,1.5-1.5V8c0-0.82-0.68-1.5-1.5-1.5H8.88c-0.82,0-1.5,0.68-1.5,1.5 V12z M11.12,12H8.88V8h2.25V12z M5.25,6.5C5.66,6.5,6,6.84,6,7.25S5.66,8,5.25,8H3v1.25h1.5c0.83,0,1.5,0.67,1.5,1.5V12 c0,0.83-0.67,1.5-1.5,1.5H2.25c-0.41,0-0.75-0.34-0.75-0.75S1.84,12,2.25,12H4.5v-1.25H3c-0.83,0-1.5-0.67-1.5-1.5V8 c0-0.83,0.67-1.5,1.5-1.5H5.25z M17.75,6.5c0.41,0,0.75,0.34,0.75,0.75S18.16,8,17.75,8H15.5v1.25H17c0.83,0,1.5,0.67,1.5,1.5V12 c0,0.83-0.67,1.5-1.5,1.5h-2.25c-0.41,0-0.75-0.34-0.75-0.75S14.34,12,14.75,12H17v-1.25h-1.5c-0.83,0-1.5-0.67-1.5-1.5V8 c0-0.83,0.67-1.5,1.5-1.5H17.75z",
                }
            }
        }
    }
}

pub fn sos_icons_24px(props: IconProps) -> Element {
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
                    d: "M13.5,7h-3c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h3c1.1,0,2-0.9,2-2V9C15.5,7.9,14.6,7,13.5,7z M13.5,15h-3V9h3V15z M3,9v2h2 c1.1,0,2,0.9,2,2v2c0,1.1-0.9,2-2,2H2c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h3v-2H3c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h3 c0.55,0,1,0.45,1,1S6.55,9,6,9H3z M19,9v2h2c1.1,0,2,0.9,2,2v2c0,1.1-0.9,2-2,2h-3c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h3v-2h-2 c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h3c0.55,0,1,0.45,1,1s-0.45,1-1,1H19z",
                }
            }
        }
    }
}

pub fn soup_kitchen_icons_20px(props: IconProps) -> Element {
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
                width: "20",
                fill: "none",
            }
            g {
                path {
                    d: "M5.08,6c0.3,0,0.52,0.28,0.45,0.58C5.49,6.75,5.48,6.9,5.48,7.06c0,0.59,0.77,2.02,0.77,2.79c0,0.37-0.08,0.66-0.16,0.86 C6.02,10.88,5.85,11,5.66,11c-0.3,0-0.53-0.28-0.46-0.58c0.04-0.16,0.07-0.35,0.07-0.58c0-0.77-0.77-2.19-0.77-2.79 c0-0.24,0.03-0.49,0.16-0.79C4.74,6.11,4.9,6,5.08,6z M8.03,11c0.19,0,0.37-0.12,0.44-0.3c0.08-0.2,0.16-0.49,0.16-0.86 c0-0.77-0.77-2.19-0.77-2.79c0-0.16,0.01-0.31,0.05-0.48C7.98,6.28,7.76,6,7.46,6C7.28,6,7.11,6.11,7.04,6.27 C6.9,6.57,6.88,6.81,6.88,7.06c0,0.59,0.77,2.02,0.77,2.79c0,0.22-0.03,0.42-0.07,0.58C7.51,10.72,7.73,11,8.03,11z M10.41,11 c0.19,0,0.37-0.12,0.44-0.3c0.08-0.2,0.16-0.49,0.16-0.86c0-0.77-0.77-2.19-0.77-2.79c0-0.16,0.01-0.31,0.05-0.48 C10.36,6.28,10.13,6,9.83,6C9.65,6,9.49,6.11,9.41,6.27c-0.13,0.3-0.16,0.54-0.16,0.79c0,0.59,0.77,2.02,0.77,2.79 c0,0.22-0.03,0.42-0.07,0.58C9.88,10.72,10.11,11,10.41,11z M16.89,5.39c0.44,0.06,0.83-0.28,0.85-0.72c0-0.12,0.01-0.26,0.01-0.4 c0-1.25-1.02-2.28-2.26-2.27c-1.15,0.01-2.1,0.88-2.22,2l-0.94,8.4l-8.82,0c-0.29,0-0.53,0.25-0.5,0.55C3.26,15.78,5.59,18,8.4,18 c2.78,0,5.07-2.18,5.36-4.99l0.99-8.84c0.04-0.38,0.36-0.67,0.75-0.67c0.41,0,0.75,0.34,0.75,0.75c0,0.11,0,0.24-0.01,0.36 C16.22,5,16.51,5.34,16.89,5.39z",
                }
            }
        }
    }
}

pub fn soup_kitchen_icons_24px(props: IconProps) -> Element {
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
                d: "M6.15,13.5c-0.46,0-0.8-0.42-0.71-0.87C5.48,12.45,5.5,12.24,5.5,12c0-1-1-2.85-1-3.62c0-0.29,0.03-0.59,0.17-0.93 C4.78,7.18,5.04,7,5.34,7c0.45,0,0.8,0.42,0.71,0.86C6.01,8.04,6,8.21,6,8.38C6,9.15,7,11,7,12c0,0.42-0.08,0.76-0.17,1.01 C6.73,13.31,6.46,13.5,6.15,13.5z M12.65,13.5c0.31,0,0.58-0.19,0.68-0.49c0.09-0.25,0.17-0.59,0.17-1.01c0-1-1-2.85-1-3.62 c0-0.17,0.01-0.34,0.04-0.51C12.63,7.42,12.29,7,11.84,7c-0.29,0-0.56,0.18-0.67,0.45C11.03,7.79,11,8.08,11,8.38 C11,9.15,12,11,12,12c0,0.24-0.02,0.45-0.06,0.63C11.85,13.08,12.19,13.5,12.65,13.5z M9.4,13.5c0.31,0,0.58-0.19,0.68-0.49 c0.09-0.25,0.17-0.59,0.17-1.01c0-1-1-2.85-1-3.62c0-0.17,0.01-0.34,0.04-0.51C9.38,7.42,9.04,7,8.59,7C8.29,7,8.03,7.18,7.92,7.45 C7.78,7.79,7.75,8.08,7.75,8.38c0,0.77,1,2.63,1,3.62c0,0.24-0.02,0.45-0.06,0.63C8.6,13.08,8.94,13.5,9.4,13.5z M20.46,6.37 c0.57,0.07,1.08-0.34,1.12-0.91C21.59,5.28,21.6,5.12,21.6,5c0-1.65-1.35-3-3-3c-1.54,0-2.81,1.16-2.98,2.65L14.53,15H3.99 c-0.6,0-1.07,0.54-0.98,1.14C3.54,19.46,6.39,22,9.75,22c3.48,0,6.34-2.73,6.71-6.23l1.15-10.87C17.66,4.39,18.08,4,18.6,4 c0.55,0,1,0.45,1,1c0,0.07-0.01,0.18-0.01,0.31C19.55,5.84,19.93,6.3,20.46,6.37L20.46,6.37z",
            }
        }
    }
}

pub fn stadium_icons_20px(props: IconProps) -> Element {
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
                    d: "M6,4.5L3,6V3L6,4.5z M15,3v3l3-1.5L15,3z M9,2v3l3-1.5L9,2z M18,8.25c0,0,0,5.87,0,7.25c0,1.11-2.31,2.06-5.5,2.39l0-2.89 c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1l0,2.89C4.31,17.56,2,16.61,2,15.5c0-0.99,0-7.25,0-7.25C2,7.01,5.58,6,10,6 S18,7.01,18,8.25z M4.4,8.3C5.51,8.65,7.42,9,10,9s4.49-0.35,5.6-0.7c0-0.21-2.38-0.8-5.6-0.8S4.4,8.09,4.4,8.3z",
                }
            }
        }
    }
}

pub fn stadium_icons_24px(props: IconProps) -> Element {
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
                    d: "M6.11,5.45L3.72,6.64C3.39,6.8,3,6.56,3,6.19V3.81C3,3.44,3.39,3.2,3.72,3.36l2.38,1.19C6.47,4.74,6.47,5.26,6.11,5.45z M18,3.81v2.38c0,0.37,0.39,0.61,0.72,0.45l2.38-1.19c0.37-0.18,0.37-0.71,0-0.89l-2.38-1.19C18.39,3.2,18,3.44,18,3.81z M11,2.81 v2.38c0,0.37,0.39,0.61,0.72,0.45l2.38-1.19c0.37-0.18,0.37-0.71,0-0.89l-2.38-1.19C11.39,2.2,11,2.44,11,2.81z M5,10.04 C6.38,10.53,8.77,11,12,11s5.62-0.47,7-0.96C19,9.86,16.22,9,12,9S5,9.86,5,10.04z M14,17h-4c-0.55,0-1,0.45-1,1l0,3.88 C4.94,21.49,2,20.34,2,19v-9c0-1.66,4.48-3,10-3s10,1.34,10,3v9c0,1.34-2.94,2.48-7,2.87L15,18C15,17.45,14.55,17,14,17z",
                }
            }
        }
    }
}

pub fn store_mall_directory_icons_24px(props: IconProps) -> Element {
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
                d: "M20.16 7.8c-.09-.46-.5-.8-.98-.8H4.82c-.48 0-.89.34-.98.8L3 12v1c0 .55.45 1 1 1v5c0 .55.45 1 1 1h8c.55 0 1-.45 1-1v-5h4v5c0 .55.45 1 1 1s1-.45 1-1v-5c.55 0 1-.45 1-1v-1l-.84-4.2zM12 18H6v-4h6v4zM5 6h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1 .45-1 1s.45 1 1 1z",
            }
        }
    }
}

pub fn straight_icons_20px(props: IconProps) -> Element {
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
                    d: "M9.25,16.25C9.25,16.66,9.59,17,10,17s0.75-0.34,0.75-0.75l0-10.38l0.66,0.66c0.29,0.29,0.77,0.29,1.06,0 c0.29-0.29,0.29-0.77,0-1.06l-1.76-1.76c-0.39-0.39-1.02-0.39-1.41,0L7.53,5.47c-0.29,0.29-0.29,0.77,0,1.06 c0.29,0.29,0.77,0.29,1.06,0l0.66-0.66L9.25,16.25z",
                }
            }
        }
    }
}

pub fn straight_icons_24px(props: IconProps) -> Element {
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
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M13,6.83l0.88,0.88c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59c-0.39-0.39-1.02-0.39-1.41,0 L8.71,6.29c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0L11,6.83V20c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V6.83 L13,6.83z",
                }
            }
        }
    }
}

pub fn streetview_icons_24px(props: IconProps) -> Element {
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
                d: "M12.56 14.33c-.34.27-.56.7-.56 1.17V21h7c1.1 0 2-.9 2-2v-5.98c-.94-.33-1.95-.52-3-.52-2.03 0-3.93.7-5.44 1.83z",
            }
            circle {
                cx: "18",
                cy: "6",
                r: "5",
            }
            path {
                d: "M11.5 6c0-1.08.27-2.1.74-3H5c-1.1 0-2 .9-2 2v14c0 .55.23 1.05.59 1.41l9.82-9.82C12.23 9.42 11.5 7.8 11.5 6z",
            }
        }
    }
}

pub fn subway_icons_24px(props: IconProps) -> Element {
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
                r: "1",
                cy: "16",
                cx: "8.5",
            }
            circle {
                cx: "15.5",
                cy: "16",
                r: "1",
            }
            path {
                d: "M7.01 9h10v5h-10zM17.8 2.8C16 2.09 13.86 2 12 2s-4 .09-5.8.8C3.53 3.84 2 6.05 2 8.86V22h20V8.86c0-2.81-1.53-5.02-4.2-6.06zm.2 12.7c0 1.54-1.16 2.79-2.65 2.96l1.15 1.16V20h-1.67l-1.5-1.5h-2.66L9.17 20H7.5v-.38l1.15-1.16C7.16 18.29 6 17.04 6 15.5V9c0-2.63 3-3 6-3s6 .37 6 3v6.5z",
            }
        }
    }
}

pub fn synagogue_icons_20px(props: IconProps) -> Element {
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
                        d: "M5,7.72V17h3v-2.89c0-1,0.68-1.92,1.66-2.08C10.92,11.82,12,12.79,12,14v3h3V7.72c0-0.46-0.21-0.89-0.56-1.17l-3.5-2.8 c-0.55-0.44-1.33-0.44-1.87,0l-3.5,2.8C5.21,6.83,5,7.27,5,7.72z M11.25,8.25c0,0.69-0.56,1.25-1.25,1.25S8.75,8.94,8.75,8.25 S9.31,7,10,7S11.25,7.56,11.25,8.25z",
                    }
                    path {
                        d: "M17.5,4C16.67,4,16,4.67,16,5.5v1.17h3V5.5C19,4.67,18.33,4,17.5,4z",
                    }
                    path {
                        d: "M16,17h1.5c0.83,0,1.5-0.67,1.5-1.5v-8h-3V17z",
                    }
                    path {
                        d: "M2.5,4C1.67,4,1,4.67,1,5.5v1.17h3V5.5C4,4.67,3.33,4,2.5,4z",
                    }
                    path {
                        d: "M2.5,17H4V7.5H1v8C1,16.33,1.67,17,2.5,17z",
                    }
                }
            }
        }
    }
}

pub fn synagogue_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M6,8.94V21h4l0-4.89c0-1,0.68-1.92,1.66-2.08C12.92,13.82,14,14.79,14,16v5h4V8.94c0-0.59-0.26-1.16-0.72-1.54l-4-3.33 c-0.74-0.62-1.82-0.62-2.56,0l-4,3.33C6.26,7.78,6,8.34,6,8.94z M13.5,10c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5 s0.67-1.5,1.5-1.5S13.5,9.17,13.5,10z",
                    }
                    path {
                        d: "M3,5C1.9,5,1,5.9,1,7v1h4V7C5,5.9,4.1,5,3,5z",
                    }
                    path {
                        d: "M3,21h2V9H1v10C1,20.1,1.9,21,3,21z",
                    }
                    path {
                        d: "M21,5c-1.1,0-2,0.9-2,2v1h4V7C23,5.9,22.1,5,21,5z",
                    }
                    path {
                        d: "M19,21h2c1.1,0,2-0.9,2-2V9h-4V21z",
                    }
                }
            }
        }
    }
}

pub fn takeout_dining_icons_20px(props: IconProps) -> Element {
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
                        d: "M6.93,16h6.15c0.8,0,1.46-0.63,1.5-1.43L14.8,10H5.2l0.23,4.57C5.47,15.37,6.13,16,6.93,16z",
                    }
                    path {
                        d: "M16.97,6.03c-0.29-0.29-0.77-0.29-1.06,0l-0.96,0.96L15,6l-2.56-2.56C12.16,3.16,11.78,3,11.38,3H8.62 c-0.4,0-0.78,0.16-1.06,0.44L5,6l0.05,0.99L4.09,6.03c-0.29-0.29-0.77-0.29-1.06,0l0,0c-0.29,0.29-0.29,0.77,0,1.06L4.94,9h10.12 l1.91-1.91C17.26,6.8,17.26,6.32,16.97,6.03L16.97,6.03z",
                    }
                }
            }
        }
    }
}

pub fn takeout_dining_icons_24px(props: IconProps) -> Element {
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
                g {
                    path {
                        d: "M21.29,6.75c-0.39-0.39-1.01-0.39-1.4,0L19,7.63l0.03-0.56l-3.46-3.48C15.19,3.21,14.68,3,14.15,3h-4.3 C9.32,3,8.81,3.21,8.43,3.59L4.97,7.07L5,7.57L4.11,6.7C3.72,6.32,3.1,6.32,2.72,6.71L2.7,6.73C2.32,7.12,2.32,7.75,2.72,8.13 L4.66,10h14.69l1.92-1.84C21.67,7.78,21.68,7.14,21.29,6.75z",
                    }
                    path {
                        d: "M5.79,18.15C5.87,19.19,6.74,20,7.79,20h8.43c1.05,0,1.92-0.81,1.99-1.85l0.49-6.6H5.3L5.79,18.15z",
                    }
                }
            }
        }
    }
}

pub fn taxi_alert_icons_24px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M18,13c-1.91,0-3.63-0.76-4.89-2H4.81l1.04-3h5.44C11.1,7.37,11,6.7,11,6s0.1-1.37,0.29-2H9C8.45,4,8,4.45,8,5v1H5.5 C4.84,6,4.29,6.42,4.08,7.01L2,13v7.5C2,21.32,2.67,22,3.5,22S5,21.32,5,20.5V20h12v0.5c0,0.82,0.67,1.5,1.5,1.5 s1.5-0.68,1.5-1.5V13l-0.09-0.27C19.3,12.9,18.66,13,18,13z M6.5,17C5.67,17,5,16.33,5,15.5S5.67,14,6.5,14S8,14.67,8,15.5 S7.33,17,6.5,17z M15.5,17c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,17,15.5,17z",
                        }
                    }
                    g {
                        path {
                            d: "M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18,9c-0.28,0-0.5-0.22-0.5-0.5S17.72,8,18,8 s0.5,0.22,0.5,0.5S18.28,9,18,9z M18.5,6.5C18.5,6.78,18.28,7,18,7s-0.5-0.22-0.5-0.5v-3C17.5,3.22,17.72,3,18,3 s0.5,0.22,0.5,0.5V6.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn temple_buddhist_icons_20px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M18.14,7.01c-0.35,0-0.63,0.25-0.73,0.58C17.17,8.4,16.41,9,15.52,9H4.48c-0.9,0-1.66-0.6-1.9-1.41 C2.48,7.26,2.2,7.01,1.86,7.01C1.35,7.01,1,7.51,1.14,7.99c0.38,1.29,1.5,2.27,2.86,2.46v6.05C4,17.33,4.67,18,5.5,18H8l0-1.89 c0-1,0.68-1.92,1.66-2.08C10.92,13.82,12,14.79,12,16v2h2.5c0.83,0,1.5-0.67,1.5-1.5v-6.05c0.49-0.07,2.43-0.61,2.9-2.55 C19.01,7.44,18.62,7,18.14,7.01z",
                        }
                    }
                    g {
                        path {
                            d: "M9.6,1.52L6.92,5h6.15L10.4,1.52C10.2,1.26,9.8,1.26,9.6,1.52z",
                        }
                    }
                    g {
                        path {
                            d: "M4.89,7.16V8H15V7.16c1.18-0.56,1.66-1.49,1.86-2.2c0.13-0.48-0.23-0.95-0.73-0.94c-0.34,0-0.62,0.24-0.71,0.57 c-0.24,0.81-0.99,1.4-1.87,1.42h-7.2C5.46,5.99,4.71,5.39,4.47,4.58C4.38,4.25,4.1,4.01,3.76,4.01c-0.5,0-0.86,0.46-0.73,0.94 C3.23,5.67,3.71,6.6,4.89,7.16z",
                        }
                    }
                }
            }
        }
    }
}

pub fn temple_buddhist_icons_24px(props: IconProps) -> Element {
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
                g {
                    g {
                        path {
                            d: "M21.85,9.01c-0.41,0-0.82,0.24-0.95,0.63C20.64,10.43,19.89,11,19.02,11H4.98c-0.87,0-1.62-0.57-1.88-1.36 C2.97,9.25,2.57,9.02,2.16,9.02h0C1.5,9.02,1,9.66,1.21,10.28c0.43,1.27,1.48,2.24,2.79,2.58V20c0,1.1,0.9,2,2,2h4l0-2.89 c0-1,0.68-1.92,1.66-2.08C12.92,16.82,14,17.79,14,19v3h4c1.1,0,2-0.9,2-2v-7.14c0.46-0.12,2.22-0.76,2.81-2.58 C23.01,9.65,22.51,9,21.85,9.01L21.85,9.01z",
                        }
                    }
                    g {
                        path {
                            d: "M6,8.86V10h12V8.86c0.46-0.12,2.22-0.76,2.81-2.58c0.2-0.63-0.3-1.27-0.96-1.27l0,0c-0.41,0-0.82,0.24-0.95,0.63 C18.64,6.43,17.89,7,17.02,7H6.98C6.11,7,5.36,6.43,5.1,5.64C4.97,5.25,4.57,5.02,4.16,5.02h0C3.5,5.02,3,5.66,3.21,6.28 C3.64,7.55,4.69,8.53,6,8.86z",
                        }
                    }
                    g {
                        path {
                            d: "M11.2,2.07L8.25,6h7.5L12.8,2.07C12.4,1.53,11.6,1.53,11.2,2.07z",
                        }
                    }
                }
            }
        }
    }
}

pub fn temple_hindu_icons_20px(props: IconProps) -> Element {
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
                    polygon {
                        points: "13.5,7 6.5,7 5.56,9.5 14.44,9.5",
                    }
                    path {
                        d: "M12 3V1.75c0-.41-.34-.75-.75-.75s-.75.34-.75.75V3h-1V1.75c0-.41-.34-.75-.75-.75S8 1.34 8 1.75V3l-.94 2.5h5.88L12 3zm5.25 6c-.41 0-.75.34-.75.75V11h-13V9.75c0-.41-.34-.75-.75-.75S2 9.34 2 9.75V18h6v-2c0-1.1.9-2 2-2s2 .9 2 2v2h6V9.75c0-.41-.34-.75-.75-.75z",
                    }
                }
            }
        }
    }
}

pub fn temple_hindu_icons_24px(props: IconProps) -> Element {
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
                    polygon {
                        points: "6.6,11 17.4,11 16.5,8 7.5,8",
                    }
                    path {
                        d: "M20 12v1H4v-1c0-.55-.45-1-1-1s-1 .45-1 1v8c0 1.1.9 2 2 2h6v-3c0-1.1.9-2 2-2s2 .9 2 2v3h6c1.1 0 2-.9 2-2v-8c0-.55-.45-1-1-1s-1 .45-1 1zm-4.1-6L15 3V2c0-.55-.45-1-1-1s-1 .45-1 1v1h-2.03V2c0-.55-.45-1-1-1s-1 .45-1 1v1.12L8.1 6h7.8z",
                    }
                }
            }
        }
    }
}

pub fn terrain_icons_24px(props: IconProps) -> Element {
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
                d: "M13.2 7.07L10.25 11l2.25 3c.33.44.24 1.07-.2 1.4-.44.33-1.07.25-1.4-.2-1.05-1.4-2.31-3.07-3.1-4.14-.4-.53-1.2-.53-1.6 0l-4 5.33c-.49.67-.02 1.61.8 1.61h18c.82 0 1.29-.94.8-1.6l-7-9.33c-.4-.54-1.2-.54-1.6 0z",
            }
        }
    }
}

pub fn theater_comedy_icons_20px(props: IconProps) -> Element {
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
                    g {
                        path {
                            d: "M9.5,7h-7C1.67,7,1,7.67,1,8.5V13c0,2.76,2.24,5,5,5s5-2.24,5-5V8.5C11,7.67,10.33,7,9.5,7z M3.75,10.5 c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75S3.75,10.91,3.75,10.5z M7.7,13.8 c-0.35,0.43-0.98,0.71-1.7,0.71s-1.34-0.28-1.7-0.71C4.04,13.48,4.29,13,4.7,13H7.3C7.71,13,7.96,13.48,7.7,13.8z M7.5,11.25 c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75C8.25,10.91,7.91,11.25,7.5,11.25z",
                        }
                    }
                    g {
                        path {
                            d: "M17.5,2h-7C9.67,2,9,2.67,9,3.5V6h1.5C11.33,6,12,6.67,12,7.5v5.08c0.61,0.27,1.29,0.42,2,0.42c2.76,0,5-2.24,5-5V3.5 C19,2.67,18.33,2,17.5,2z M11.75,5.5c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75 S11.75,5.91,11.75,5.5z M15.3,9.51H12.7c-0.41,0-0.66-0.48-0.4-0.8C12.66,8.28,13.28,8,14,8s1.34,0.28,1.7,0.71 C15.96,9.03,15.71,9.51,15.3,9.51z M15.5,6.25c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75 C16.25,5.91,15.91,6.25,15.5,6.25z",
                        }
                    }
                }
            }
        }
    }
}

pub fn theater_comedy_icons_24px(props: IconProps) -> Element {
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
                        d: "M21,2h-8c-1.1,0-2,0.9-2,2v3.5h1.5c1.1,0,2,0.9,2,2v4.95c1.04,0.48,2.24,0.68,3.5,0.47c2.93-0.49,5-3.17,5-6.14V4 C23,2.9,22.1,2,21,2z M14,6.5c0-0.55,0.45-1,1-1s1,0.45,1,1s-0.45,1-1,1S14,7.05,14,6.5z M18.85,10.88h-3.72 c-0.38,0-0.63-0.41-0.44-0.75C15.08,9.47,15.96,9,16.99,9s1.91,0.47,2.3,1.14C19.48,10.47,19.23,10.88,18.85,10.88z M19,7.5 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S19.55,7.5,19,7.5z",
                    }
                    path {
                        d: "M11,9H3c-1.1,0-2,0.9-2,2v4.79c0,3.05,2.19,5.77,5.21,6.16C9.87,22.42,13,19.57,13,16v-5C13,9.9,12.1,9,11,9z M4,13.5 c0-0.55,0.45-1,1-1s1,0.45,1,1s-0.45,1-1,1S4,14.05,4,13.5z M9.3,16.75c-0.38,0.67-1.27,1.14-2.3,1.14s-1.91-0.47-2.3-1.14 C4.51,16.41,4.76,16,5.14,16h3.72C9.24,16,9.49,16.41,9.3,16.75z M9,14.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S9.55,14.5,9,14.5z",
                    }
                }
            }
        }
    }
}

pub fn tire_repair_icons_20px(props: IconProps) -> Element {
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
                        d: "M14,5.75c0,0.41,0.34,0.75,0.75,0.75c0.21,0,0.39-0.08,0.53-0.22C15.58,5.98,16,4.5,16,4.5s-1.48,0.42-1.78,0.72 C14.08,5.36,14,5.54,14,5.75z",
                    }
                    path {
                        d: "M14.75,2C12.68,2,11,3.68,11,5.75c0,1.44,0.81,2.69,2,3.32V9.5c0,0.28,0.22,0.5,0.5,0.5H14v4.75 c0,0.41-0.34,0.75-0.75,0.75c-0.41,0-0.75-0.34-0.75-0.75v-1.5c0-1.24-1.01-2.25-2.25-2.25c-0.26,0-0.52,0.05-0.75,0.13V4.5 C9.5,3.67,8.83,3,8,3H3.5C2.67,3,2,3.67,2,4.5v11C2,16.33,2.67,17,3.5,17H8c0.83,0,1.5-0.67,1.5-1.5v-2.25 c0-0.41,0.34-0.75,0.75-0.75S11,12.84,11,13.25v1.5c0,1.24,1.01,2.25,2.25,2.25c1.24,0,2.25-1.01,2.25-2.25V10H16 c0.28,0,0.5-0.22,0.5-0.5V9.07c1.19-0.63,2-1.88,2-3.32C18.5,3.68,16.82,2,14.75,2z M5,15.5L3.5,14v-2.12l1.5,1.5V15.5z M5,11.75 l-1.5-1.5V8.13L5,9.62V11.75z M5,8L3.5,6.5V4.38L5,5.88V8z M8,14l-1.5,1.5v-2.12l1.5-1.5V14z M8,10.25l-1.5,1.5V9.62L8,8.13V10.25 z M8,6.5L6.5,8V5.88L8,4.38V6.5z M14.75,8c-1.24,0-2.25-1.01-2.25-2.25c0-1.24,1.01-2.25,2.25-2.25S17,4.51,17,5.75 C17,6.99,15.99,8,14.75,8z",
                    }
                }
            }
        }
    }
}

pub fn tire_repair_icons_24px(props: IconProps) -> Element {
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
                        d: "M18,7c0,0.55,0.45,1,1,1c0.28,0,0.53-0.11,0.71-0.29c0.4-0.4,1.04-2.46,1.04-2.46s-2.06,0.64-2.46,1.04 C18.11,6.47,18,6.72,18,7z",
                    }
                    path {
                        d: "M19,2c-2.76,0-5,2.24-5,5c0,2.05,1.23,3.81,3,4.58V12c0,0.55,0.45,1,1,1v5c0,0.55-0.45,1-1,1s-1-0.45-1-1v-3 c0-1.65-1.35-3-3-3c-0.35,0-0.69,0.06-1,0.17V5c0-1.1-0.9-2-2-2H4C2.9,3,2,3.9,2,5v14c0,1.1,0.9,2,2,2h6c1.1,0,2-0.9,2-2v-4 c0-0.55,0.45-1,1-1s1,0.45,1,1v3c0,1.65,1.35,3,3,3s3-1.35,3-3v-5c0.55,0,1-0.45,1-1v-0.42c1.77-0.77,3-2.53,3-4.58 C24,4.24,21.76,2,19,2z M6,19.5l-2-2v-2.83l2,2V19.5z M6,14.5l-2-2V9.67l2,2V14.5z M6,9.5l-2-2V4.67l2,2V9.5z M10,17.5l-2,2v-2.83 l2-2V17.5z M10,12.5l-2,2v-2.83l2-2V12.5z M10,7.5l-2,2V6.67l2-2V7.5z M19,10c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3 S20.66,10,19,10z",
                    }
                }
            }
        }
    }
}

pub fn traffic_icons_24px(props: IconProps) -> Element {
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
                d: "M19.96 10.59c.04-.31-.19-.59-.5-.59H17V8.86c1.54-.4 2.72-1.68 2.96-3.27.04-.31-.19-.59-.5-.59H17V4c0-.55-.45-1-1-1H8c-.55 0-1 .45-1 1v1H4.54c-.31 0-.54.28-.5.59C4.28 7.18 5.46 8.46 7 8.86V10H4.54c-.31 0-.54.28-.5.59.24 1.59 1.42 2.87 2.96 3.27V15H4.54c-.31 0-.54.28-.5.59.24 1.59 1.42 2.87 2.96 3.27V20c0 .55.45 1 1 1h8c.55 0 1-.45 1-1v-1.14c1.54-.4 2.72-1.68 2.96-3.27.04-.31-.19-.59-.5-.59H17v-1.14c1.54-.4 2.72-1.68 2.96-3.27zM12 19c-1.11 0-2-.9-2-2s.89-2 2-2c1.1 0 2 .9 2 2s-.89 2-2 2zm0-5c-1.11 0-2-.9-2-2s.89-2 2-2c1.1 0 2 .9 2 2s-.89 2-2 2zm0-5c-1.11 0-2-.9-2-2 0-1.11.89-2 2-2 1.1 0 2 .89 2 2 0 1.1-.89 2-2 2z",
            }
        }
    }
}

pub fn train_icons_24px(props: IconProps) -> Element {
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
                d: "M12 2c-4 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19l-1.15 1.15c-.31.31-.09.85.36.85H7.8c.13 0 .26-.05.35-.15L10 19h4l1.85 1.85c.09.09.22.15.35.15h1.09c.45 0 .67-.54.35-.85L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-4-4-8-4zM7.5 17c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm3.5-7H6V6h5v4zm5.5 7c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-7h-5V6h5v4z",
            }
        }
    }
}

pub fn tram_icons_24px(props: IconProps) -> Element {
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
                d: "M13 5l.75-1.5H17V2H7v1.5h4.75L11 5c-3.13.09-6 .73-6 3.5V17c0 1.5 1.11 2.73 2.55 2.95l-1.19 1.19c-.32.32-.1.86.35.86H7.8c.13 0 .26-.05.35-.15L10 20h4l1.85 1.85c.09.09.22.15.35.15h1.09c.45 0 .67-.54.35-.85l-1.19-1.19C17.89 19.73 19 18.5 19 17V8.5c0-2.77-2.87-3.41-6-3.5zm-1 13.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm5-4.5H7V9h10v5z",
            }
        }
    }
}

pub fn transfer_within_a_station_icons_24px(props: IconProps) -> Element {
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
                d: "M22 15.5h-5.52v-.77c0-.36-.44-.54-.69-.29l-1.51 1.52c-.16.16-.16.41 0 .57l1.51 1.52c.26.26.69.08.69-.29V17H22v-1.5zm-.28 4.71l-1.51-1.52c-.26-.26-.69-.08-.69.29v.77H14v1.5h5.52v.77c0 .36.44.54.69.29l1.51-1.52c.16-.16.16-.42 0-.58zM9.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM5.75 8.9L3.23 21.81c-.12.62.35 1.19.98 1.19h.09c.47 0 .88-.33.98-.79L6.85 15 9 17v5c0 .55.45 1 1 1s1-.45 1-1v-5.72c0-.53-.21-1.04-.59-1.41L8.95 13.4l.6-3c1.07 1.32 2.58 2.23 4.31 2.51.6.1 1.14-.39 1.14-1 0-.49-.36-.9-.84-.98-1.49-.25-2.75-1.15-3.51-2.38l-.95-1.6C9.35 6.35 8.7 6 8 6c-.25 0-.5.05-.75.15L3.24 7.79C2.49 8.1 2 8.83 2 9.64V12c0 .55.45 1 1 1s1-.45 1-1V9.65l1.75-.75",
            }
        }
    }
}

pub fn transit_enterexit_icons_24px(props: IconProps) -> Element {
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
                d: "M14.5 18H8c-1.1 0-2-.9-2-2V9.5C6 8.67 6.67 8 7.5 8S9 8.67 9 9.5v3.27L14.95 7c.57-.55 1.48-.54 2.04.02s.56 1.47.01 2.04L11.15 15h3.35c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5z",
            }
        }
    }
}

pub fn trip_origin_icons_24px(props: IconProps) -> Element {
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
                d: "M2 12C2 6.48 6.48 2 12 2s10 4.48 10 10-4.48 10-10 10S2 17.52 2 12zm10 6c3.31 0 6-2.69 6-6s-2.69-6-6-6-6 2.69-6 6 2.69 6 6 6z",
            }
        }
    }
}

pub fn turn_left_icons_20px(props: IconProps) -> Element {
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
                    d: "M6.53,10.72c0.29-0.29,0.29-0.77,0-1.06L5.87,9L12,9v6.25c0,0.41,0.34,0.75,0.75,0.75s0.75-0.34,0.75-0.75V9 c0-0.83-0.67-1.5-1.5-1.5l-6.13,0l0.66-0.66c0.29-0.29,0.29-0.77,0-1.06c-0.29-0.29-0.77-0.29-1.06,0L3.71,7.54 c-0.39,0.39-0.39,1.02,0,1.41l1.76,1.76C5.76,11.01,6.24,11.01,6.53,10.72z",
                }
            }
        }
    }
}

pub fn turn_left_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M7.71,13.29c-0.39,0.39-1.02,0.39-1.41,0l-2.59-2.59c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59c0.39-0.39,1.02-0.39,1.41,0 c0.39,0.39,0.39,1.02,0,1.41L6.83,9L15,9c1.1,0,2,0.9,2,2v8c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1v-8l-8.17,0l0.88,0.88 C8.1,12.27,8.1,12.9,7.71,13.29z",
                }
            }
        }
    }
}

pub fn turn_right_icons_20px(props: IconProps) -> Element {
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
                    d: "M13.47,10.72c-0.29-0.29-0.29-0.77,0-1.06L14.13,9L8,9v6.25C8,15.66,7.66,16,7.25,16S6.5,15.66,6.5,15.25L6.5,9 c0-0.83,0.67-1.5,1.5-1.5l6.13,0l-0.66-0.66c-0.29-0.29-0.29-0.77,0-1.06c0.29-0.29,0.77-0.29,1.06,0l1.76,1.76 c0.39,0.39,0.39,1.02,0,1.41l-1.76,1.76C14.24,11.01,13.76,11.01,13.47,10.72z",
                }
            }
        }
    }
}

pub fn turn_right_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M16.29,13.29c0.39,0.39,1.02,0.39,1.41,0l2.59-2.59c0.39-0.39,0.39-1.02,0-1.41l-2.59-2.59c-0.39-0.39-1.02-0.39-1.41,0 c-0.39,0.39-0.39,1.02,0,1.41L17.17,9L9,9c-1.1,0-2,0.9-2,2v8c0,0.55,0.45,1,1,1s1-0.45,1-1v-8l8.17,0l-0.88,0.88 C15.9,12.27,15.9,12.9,16.29,13.29z",
                }
            }
        }
    }
}

pub fn turn_sharp_left_icons_20px(props: IconProps) -> Element {
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
                    d: "M13,16.25c0,0.41,0.34,0.75,0.75,0.75s0.75-0.34,0.75-0.75V12c0-0.83-0.67-1.5-1.5-1.5l-6,0l0-4.63l0.66,0.66 c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06L6.96,3.71c-0.39-0.39-1.02-0.39-1.41,0L3.78,5.47 c-0.29,0.29-0.29,0.77,0,1.06c0.29,0.29,0.77,0.29,1.06,0L5.5,5.87l0,4.63C5.5,11.33,6.17,12,7,12l6,0V16.25z",
                }
            }
        }
    }
}

pub fn turn_sharp_left_icons_24px(props: IconProps) -> Element {
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M8,6.83l0.88,0.88c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41L7.71,3.71c-0.39-0.39-1.02-0.39-1.41,0 L3.71,6.29c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0L6,6.83V13c0,1.1,0.9,2,2,2h8v5c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-5c0-1.1-0.9-2-2-2H8V6.83L8,6.83z",
                }
            }
        }
    }
}

pub fn turn_sharp_right_icons_20px(props: IconProps) -> Element {
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
                    d: "M7,16.25C7,16.66,6.66,17,6.25,17S5.5,16.66,5.5,16.25V12c0-0.83,0.67-1.5,1.5-1.5l6,0l0-4.63l-0.66,0.66 c-0.29,0.29-0.77,0.29-1.06,0c-0.29-0.29-0.29-0.77,0-1.06l1.76-1.76c0.39-0.39,1.02-0.39,1.41,0l1.76,1.76 c0.29,0.29,0.29,0.77,0,1.06c-0.29,0.29-0.77,0.29-1.06,0L14.5,5.87l0,4.63c0,0.83-0.67,1.5-1.5,1.5l-6,0V16.25z",
                }
            }
        }
    }
}

pub fn turn_sharp_right_icons_24px(props: IconProps) -> Element {
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
                    d: "M16,6.83l-0.88,0.88c-0.39,0.39-1.02,0.39-1.41,0c-0.39-0.39-0.39-1.02,0-1.41l2.59-2.59c0.39-0.39,1.02-0.39,1.41,0 l2.59,2.59c0.39,0.39,0.39,1.02,0,1.41c-0.39,0.39-1.02,0.39-1.41,0L18,6.83V13c0,1.1-0.9,2-2,2H8v5c0,0.55-0.45,1-1,1h0 c-0.55,0-1-0.45-1-1v-5c0-1.1,0.9-2,2-2h8V6.83L16,6.83z",
                }
            }
        }
    }
}

pub fn turn_slight_left_icons_20px(props: IconProps) -> Element {
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
                    d: "M11.75,16C11.34,16,11,15.66,11,15.25v-4.69l-4-4L7,7.5c0,0.41-0.34,0.75-0.75,0.75C5.84,8.25,5.5,7.91,5.5,7.5l0-2.5 c0-0.55,0.45-1,1-1L9,4c0.41,0,0.75,0.34,0.75,0.75C9.75,5.16,9.41,5.5,9,5.5l-0.94,0l4,4c0.28,0.28,0.44,0.66,0.44,1.06v4.69 C12.5,15.66,12.16,16,11.75,16z",
                }
            }
        }
    }
}

pub fn turn_slight_left_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M11.66,5L11.66,5c0-0.55-0.45-1-1-1H7C6.45,4,6,4.45,6,5v3.66c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V7.41l5,5V19 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-6.58c0-0.53-0.21-1.04-0.59-1.41l-5-5h1.24C11.21,6,11.66,5.55,11.66,5z",
                }
            }
        }
    }
}

pub fn turn_slight_right_icons_20px(props: IconProps) -> Element {
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
                    d: "M8.25,16C8.66,16,9,15.66,9,15.25v-4.69l4-4l0,0.94c0,0.41,0.34,0.75,0.75,0.75c0.41,0,0.75-0.34,0.75-0.75l0-2.5 c0-0.55-0.45-1-1-1L11,4c-0.41,0-0.75,0.34-0.75,0.75c0,0.41,0.34,0.75,0.75,0.75l0.94,0l-4,4C7.66,9.78,7.5,10.16,7.5,10.56v4.69 C7.5,15.66,7.84,16,8.25,16z",
                }
            }
        }
    }
}

pub fn turn_slight_right_icons_24px(props: IconProps) -> Element {
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
                    d: "M12.34,5L12.34,5c0-0.55,0.45-1,1-1H17c0.55,0,1,0.45,1,1v3.66c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V7.41l-5,5V19 c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-6.58c0-0.53,0.21-1.04,0.59-1.41l5-5h-1.24C12.79,6,12.34,5.55,12.34,5z",
                }
            }
        }
    }
}

pub fn two_wheeler_icons_20px(props: IconProps) -> Element {
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
                    x: "0",
                    y: "0",
                }
                path {
                    d: "M14.5,9c-0.16,0-0.31,0.02-0.45,0.05L13,8h1c0.28,0,0.5-0.22,0.5-0.5V7.31c0-0.37-0.39-0.61-0.72-0.45L12.5,7.5l-1.35-1.35 C11.05,6.05,10.93,6,10.79,6H9.5C9.22,6,9,6.22,9,6.5C9,6.78,9.22,7,9.5,7h0.88c0.13,0,0.26,0.05,0.35,0.15L11.59,8H9.6 C9.53,8.01,9.47,8.02,9.41,8.04l-2.1,0.84c-0.19,0.07-0.4,0.03-0.54-0.11L6.15,8.15C6.05,8.05,5.93,8,5.79,8H3.5 C3.22,8,3,8.22,3,8.5C3,8.78,3.22,9,3.5,9h2c-1.61,0-2.86,1.52-2.41,3.2c0.22,0.8,0.86,1.46,1.65,1.69 c1.54,0.45,2.95-0.54,3.21-1.94l0.75,0.75C8.9,12.89,9.15,13,9.41,13h0.47c0.38,0,0.73-0.21,0.89-0.55l1.75-3.5l0.52,0.52 C12.42,9.92,12,10.66,12,11.5c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5C17,10.12,15.88,9,14.5,9z M5.5,13C4.67,13,4,12.33,4,11.5 S4.67,10,5.5,10S7,10.67,7,11.5S6.33,13,5.5,13z M14.5,13c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 S15.33,13,14.5,13z",
                }
            }
        }
    }
}

pub fn two_wheeler_icons_24px(props: IconProps) -> Element {
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
                    y: "0",
                    fill: "none",
                    x: "0",
                    fill_rule: "evenodd",
                }
                path {
                    d: "M20,11c-0.18,0-0.36,0.03-0.53,0.05L17.41,9H19c0.55,0,1-0.45,1-1V7.62c0-0.74-0.78-1.23-1.45-0.89l-2.28,1.14l-2.57-2.57 C13.52,5.11,13.26,5,13,5h-3C9.45,5,9,5.45,9,6v0c0,0.55,0.45,1,1,1h2.17c0.27,0,0.52,0.11,0.71,0.29L14.59,9h-3.35 c-0.16,0-0.31,0.04-0.45,0.11l-3.14,1.57c-0.38,0.19-0.85,0.12-1.15-0.19l-1.2-1.2C5.11,9.11,4.85,9,4.59,9H1c-0.55,0-1,0.45-1,1v0 c0,0.55,0.45,1,1,1h3c-2.52,0-4.49,2.32-3.89,4.94c0.33,1.45,1.5,2.62,2.95,2.95C5.68,19.49,8,17.52,8,15l1.41,1.41 C9.79,16.79,10.3,17,10.83,17h1.01c0.72,0,1.38-0.38,1.74-1.01l2.91-5.09l1.01,1.01c-1.13,0.91-1.76,2.41-1.38,4.05 c0.34,1.44,1.51,2.61,2.95,2.94C21.68,19.49,24,17.51,24,15C24,12.79,22.21,11,20,11z M4,17c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2 c1.1,0,2,0.9,2,2C6,16.1,5.1,17,4,17z M20,17c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2C22,16.1,21.1,17,20,17z",
                }
            }
        }
    }
}

pub fn u_turn_left_icons_20px(props: IconProps) -> Element {
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
                    d: "M3.78,9.47c0.29-0.29,0.77-0.29,1.06,0l0.66,0.66l0-2.44c0-2.35,1.72-4.45,4.06-4.67c2.68-0.26,4.94,1.85,4.94,4.48v7.75 c0,0.41-0.34,0.75-0.75,0.75S13,15.66,13,15.25l0-7.58c0-1.62-1.22-3.08-2.84-3.17C8.43,4.41,7,5.79,7,7.5l0,2.63l0.66-0.66 c0.29-0.29,0.77-0.29,1.06,0c0.29,0.29,0.29,0.77,0,1.06l-1.76,1.76c-0.39,0.39-1.02,0.39-1.41,0l-1.76-1.76 C3.49,10.24,3.49,9.76,3.78,9.47z",
                }
            }
        }
    }
}

pub fn u_turn_left_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M3.71,12.29c0.39-0.39,1.02-0.39,1.41,0L6,13.17V9c0-3.31,2.69-6,6-6s6,2.69,6,6v11c0,0.55-0.45,1-1,1s-1-0.45-1-1V9 c0-2.21-1.79-4-4-4S8,6.79,8,9v4.17l0.88-0.88c0.39-0.39,1.02-0.39,1.41,0c0.39,0.39,0.39,1.02,0,1.41l-2.59,2.59 c-0.39,0.39-1.02,0.39-1.41,0l-2.59-2.59C3.32,13.32,3.32,12.68,3.71,12.29z",
                }
            }
        }
    }
}

pub fn u_turn_right_icons_20px(props: IconProps) -> Element {
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
                    d: "M16.22,9.47c-0.29-0.29-0.77-0.29-1.06,0l-0.66,0.66l0-2.44c0-2.35-1.72-4.45-4.06-4.67C7.76,2.77,5.5,4.87,5.5,7.5v7.75 C5.5,15.66,5.84,16,6.25,16C6.66,16,7,15.66,7,15.25l0-7.58C7,6.05,8.22,4.59,9.84,4.5C11.57,4.41,13,5.79,13,7.5l0,2.63 l-0.66-0.66c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.77,0,1.06l1.76,1.76c0.39,0.39,1.02,0.39,1.41,0l1.76-1.76 C16.51,10.24,16.51,9.76,16.22,9.47z",
                }
            }
        }
    }
}

pub fn u_turn_right_icons_24px(props: IconProps) -> Element {
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
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M20.29,12.29c-0.39-0.39-1.02-0.39-1.41,0L18,13.17V9c0-3.31-2.69-6-6-6S6,5.69,6,9v11c0,0.55,0.45,1,1,1s1-0.45,1-1V9 c0-2.21,1.79-4,4-4s4,1.79,4,4v4.17l-0.88-0.88c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l2.59,2.59 c0.39,0.39,1.02,0.39,1.41,0l2.59-2.59C20.68,13.32,20.68,12.68,20.29,12.29z",
                }
            }
        }
    }
}

pub fn volunteer_activism_icons_20px(props: IconProps) -> Element {
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
                        d: "M2.5,18L2.5,18C3.33,18,4,17.33,4,16.5v-6C4,9.67,3.33,9,2.5,9h0C1.67,9,1,9.67,1,10.5v6C1,17.33,1.67,18,2.5,18z",
                    }
                    path {
                        d: "M12.58,9.9c0.38,0.34,0.96,0.34,1.34,0c0.98-0.88,1.9-1.71,2.62-2.5C17.42,6.43,18,5.51,18,4.61 c0-0.73-0.29-1.39-0.76-1.86c-1.03-1.03-2.36-0.8-3.04-0.49c-0.37,0.17-0.7,0.42-0.95,0.72c-0.26-0.3-0.59-0.55-0.95-0.72 c-0.68-0.32-2.01-0.54-3.04,0.49C8.79,3.23,8.5,3.88,8.5,4.61c0,0.89,0.58,1.81,1.46,2.78C10.68,8.18,11.6,9.01,12.58,9.9z",
                    }
                    path {
                        d: "M10.5,14l-1.03-0.34C9.2,13.57,9.05,13.27,9.15,13v0c0.1-0.28,0.42-0.42,0.69-0.3L10.5,13h2.62 c0.49,0,0.88-0.39,0.88-0.88v0c0-0.37-0.23-0.7-0.58-0.83l-6.17-2.2C7.08,9.03,6.91,9,6.74,9H5.5v7.36l5.53,1.51 c0.31,0.08,0.64,0.07,0.93-0.05L18,15.5v0c0-0.83-0.67-1.5-1.5-1.5H10.5z",
                    }
                }
            }
        }
    }
}

pub fn volunteer_activism_icons_24px(props: IconProps) -> Element {
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
                        d: "M3,11L3,11c-1.1,0-2,0.9-2,2v7c0,1.1,0.9,2,2,2h0c1.1,0,2-0.9,2-2v-7C5,11.9,4.1,11,3,11z",
                    }
                    path {
                        d: "M10,5.3C10,3.45,11.45,2,13.3,2c1.04,0,2.05,0.49,2.7,1.25C16.65,2.49,17.66,2,18.7,2C20.55,2,22,3.45,22,5.3 c0,2.1-2.5,4.51-5.33,7.09c-0.38,0.35-0.97,0.35-1.35,0C12.5,9.81,10,7.4,10,5.3",
                    }
                    path {
                        d: "M19.99,17h-6.83c-0.11,0-0.22-0.02-0.33-0.06l-1.47-0.51c-0.26-0.09-0.39-0.37-0.3-0.63l0,0c0.09-0.26,0.38-0.4,0.64-0.3 l1.12,0.43c0.11,0.04,0.24,0.07,0.36,0.07h2.63c0.65,0,1.18-0.53,1.18-1.18v0c0-0.49-0.31-0.93-0.77-1.11L9.3,11.13 C9.08,11.04,8.84,11,8.6,11H7v9.02l6.37,1.81c0.41,0.12,0.85,0.1,1.25-0.05L22,19l0,0C22,17.89,21.1,17,19.99,17z",
                    }
                }
            }
        }
    }
}

pub fn warehouse_icons_20px(props: IconProps) -> Element {
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
                    d: "M14,17h2.5c0.83,0,1.5-0.67,1.5-1.5V7.04c0-0.63-0.39-1.18-0.97-1.4l-6.5-2.44c-0.34-0.13-0.71-0.13-1.05,0l-6.5,2.44 C2.39,5.85,2,6.41,2,7.04v8.46C2,16.33,2.67,17,3.5,17H6v-7h8V17z M9.25,17h-1.5v-1.5h1.5V17z M10.75,14.5h-1.5V13h1.5V14.5z M12.25,17h-1.5v-1.5h1.5V17z",
                }
            }
        }
    }
}

pub fn warehouse_icons_24px(props: IconProps) -> Element {
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
                path {
                    d: "M22,19V8.35c0-0.82-0.5-1.55-1.26-1.86l-8-3.2c-0.48-0.19-1.01-0.19-1.49,0l-8,3.2C2.5,6.8,2,7.54,2,8.35V19 c0,1.1,0.9,2,2,2h3v-9h10v9h3C21.1,21,22,20.1,22,19z M11,19H9v2h2V19z M13,16h-2v2h2V16z M15,19h-2v2h2V19z",
                }
            }
        }
    }
}

pub fn wine_bar_icons_24px(props: IconProps) -> Element {
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
                height: "24",
                fill: "none",
            }
            path {
                d: "M7,3C6.45,3,6,3.45,6,4l0,5c0,2.97,2.16,5.43,5,5.91V19H9c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1h-2v-4.09c2.84-0.48,5-2.94,5-5.91l0-5c0-0.55-0.45-1-1-1H7z M16,8H8l0-3h8C16,5,16,8,16,8z",
            }
        }
    }
}

pub fn wrong_location_icons_20px(props: IconProps) -> Element {
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
                        d: "M16.81,4.75l1.16-1.16c0.29-0.29,0.29-0.77,0-1.06s-0.77-0.29-1.06,0l-1.16,1.16l-1.16-1.16 c-0.29-0.29-0.77-0.29-1.06,0s-0.29,0.77,0,1.06l1.16,1.16l-1.16,1.16c-0.29,0.29-0.29,0.77,0,1.06s0.77,0.29,1.06,0l1.16-1.16 l1.16,1.16c0.29,0.29,0.77,0.29,1.06,0s0.29-0.77,0-1.06L16.81,4.75z",
                        enable_background: "new",
                    }
                    path {
                        enable_background: "new",
                        d: "M16.2,7.68l-0.45-0.45L15.3,7.68c-0.68,0.68-1.79,0.68-2.48,0c-0.68-0.68-0.68-1.79,0-2.47 l0.45-0.45L12.82,4.3c-0.24-0.24-0.39-0.53-0.47-0.85C11.63,3.16,10.83,3,10,3C6.41,3,3.5,5.91,3.5,9.5c0,3.49,4.2,7.5,5.85,8.95 c0.38,0.33,0.93,0.33,1.31,0C12.3,17,16.5,12.99,16.5,9.5c0-0.62-0.09-1.22-0.25-1.79C16.23,7.7,16.22,7.69,16.2,7.68z M10,11 c-0.83,0-1.5-0.67-1.5-1.5C8.5,8.67,9.17,8,10,8s1.5,0.67,1.5,1.5C11.5,10.33,10.83,11,10,11z",
                    }
                }
            }
        }
    }
}

pub fn wrong_location_icons_24px(props: IconProps) -> Element {
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
                        enable_background: "new",
                        d: "M20.42,4.5l1.38-1.38c0.39-0.39,0.39-1.02,0-1.41l0,0c-0.39-0.39-1.02-0.39-1.41,0L19,3.08 l-1.38-1.38c-0.39-0.39-1.02-0.39-1.41,0s-0.39,1.02,0,1.41l1.38,1.38l-1.38,1.38c-0.39,0.39-0.39,1.02,0,1.41l0,0 c0.39,0.39,1.02,0.39,1.41,0L19,5.92l1.38,1.38c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41L20.42,4.5z",
                    }
                    path {
                        d: "M19.67,8L19,7.33l-0.59,0.59c-0.7,0.7-1.84,0.88-2.65,0.3c-1.03-0.74-1.12-2.19-0.26-3.05 l0.67-0.67L15.5,3.83c-0.36-0.36-0.54-0.81-0.57-1.28C14.01,2.19,13.02,2,12,2c-4.2,0-8,3.22-8,8.2c0,3.18,2.45,6.92,7.34,11.23 c0.38,0.33,0.95,0.33,1.33,0C17.55,17.12,20,13.38,20,10.2c0-0.76-0.1-1.47-0.26-2.14C19.72,8.04,19.69,8.02,19.67,8z M12,12 c-1.1,0-2-0.9-2-2s0.9-2,2-2c1.1,0,2,0.9,2,2S13.1,12,12,12z",
                        enable_background: "new",
                    }
                }
            }
        }
    }
}

pub fn zoom_in_map_icons_20px(props: IconProps) -> Element {
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
                    d: "M15.25,8l-2.5,0C12.34,8,12,7.66,12,7.25l0-2.5C12,4.34,12.34,4,12.75,4c0.41,0,0.75,0.34,0.75,0.75v0.69l1.91-1.91 c0.29-0.29,0.77-0.29,1.06,0c0.29,0.29,0.29,0.77,0,1.06L14.56,6.5l0.69,0C15.66,6.5,16,6.84,16,7.25C16,7.66,15.66,8,15.25,8z M4,7.25C4,6.84,4.34,6.5,4.75,6.5h0.69L3.53,4.59c-0.29-0.29-0.29-0.77,0-1.06c0.29-0.29,0.77-0.29,1.06,0L6.5,5.44V4.75 C6.5,4.34,6.84,4,7.25,4S8,4.34,8,4.75v2.5C8,7.66,7.66,8,7.25,8h-2.5C4.34,8,4,7.66,4,7.25z M16,12.75c0,0.41-0.34,0.75-0.75,0.75 l-0.69,0l1.91,1.91c0.29,0.29,0.29,0.77,0,1.06c-0.29,0.29-0.77,0.29-1.06,0l-1.91-1.91v0.69c0,0.41-0.34,0.75-0.75,0.75 S12,15.66,12,15.25l0-2.5c0-0.41,0.34-0.75,0.75-0.75l2.5,0C15.66,12,16,12.34,16,12.75z M4.75,12h2.5C7.66,12,8,12.34,8,12.75v2.5 C8,15.66,7.66,16,7.25,16S6.5,15.66,6.5,15.25v-0.69l-1.91,1.91c-0.29,0.29-0.77,0.29-1.06,0c-0.29-0.29-0.29-0.77,0-1.06 l1.91-1.91H4.75C4.34,13.5,4,13.16,4,12.75C4,12.34,4.34,12,4.75,12z",
                }
            }
        }
    }
}

pub fn zoom_in_map_icons_24px(props: IconProps) -> Element {
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
                    d: "M3,8c0,0.55,0.45,1,1,1l4,0c0.55,0,1-0.45,1-1l0-4c0-0.55-0.45-1-1-1S7,3.45,7,4l0,1.59L4.62,3.21 c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L5.59,7L4,7C3.45,7,3,7.45,3,8z M20,7h-1.59l2.38-2.38 c0.39-0.39,0.39-1.02,0-1.41c-0.39-0.39-1.02-0.39-1.41,0L17,5.59V4c0-0.55-0.45-1-1-1c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h4 c0.55,0,1-0.45,1-1S20.55,7,20,7z M4,17h1.59l-2.38,2.38c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0L7,18.41L7,20 c0,0.55,0.45,1,1,1s1-0.45,1-1l0-4c0-0.55-0.45-1-1-1l-4,0c-0.55,0-1,0.45-1,1C3,16.55,3.45,17,4,17z M21,16c0-0.55-0.45-1-1-1h-4 c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-1.59l2.38,2.38c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41L18.41,17H20C20.55,17,21,16.55,21,16z",
                }
            }
        }
    }
}

pub fn zoom_out_map_icons_24px(props: IconProps) -> Element {
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
            }
            g {
                g {
                    g {
                        g {
                            path {
                                d: "M15.85,3.85L17.3,5.3l-2.18,2.16c-0.39,0.39-0.39,1.03,0,1.42l0,0c0.39,0.39,1.03,0.39,1.42,0L18.7,6.7l1.45,1.45 C20.46,8.46,21,8.24,21,7.79V3.5C21,3.22,20.78,3,20.5,3h-4.29C15.76,3,15.54,3.54,15.85,3.85z M3.85,8.15L5.3,6.7l2.16,2.18 c0.39,0.39,1.03,0.39,1.42,0l0,0c0.39-0.39,0.39-1.03,0-1.42L6.7,5.3l1.45-1.45C8.46,3.54,8.24,3,7.79,3H3.5 C3.22,3,3,3.22,3,3.5v4.29C3,8.24,3.54,8.46,3.85,8.15z M8.15,20.15L6.7,18.7l2.18-2.16c0.39-0.39,0.39-1.03,0-1.42l0,0 c-0.39-0.39-1.03-0.39-1.42,0L5.3,17.3l-1.45-1.45C3.54,15.54,3,15.76,3,16.21v4.29C3,20.78,3.22,21,3.5,21h4.29 C8.24,21,8.46,20.46,8.15,20.15z M20.15,15.85L18.7,17.3l-2.16-2.18c-0.39-0.39-1.03-0.39-1.42,0l0,0 c-0.39,0.39-0.39,1.03,0,1.42l2.18,2.16l-1.45,1.45C15.54,20.46,15.76,21,16.21,21h4.29c0.28,0,0.5-0.22,0.5-0.5v-4.29 C21,15.76,20.46,15.54,20.15,15.85z",
                            }
                        }
                    }
                }
            }
        }
    }
}

