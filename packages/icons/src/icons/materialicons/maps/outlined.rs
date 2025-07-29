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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M12 7C6.48 7 2 9.24 2 12c0 2.24 2.94 4.13 7 4.77V20l4-4-4-4v2.73c-3.15-.56-5-1.9-5-2.73 0-1.06 3.04-3 8-3s8 1.94 8 3c0 .73-1.46 1.89-4 2.53v2.05c3.53-.77 6-2.53 6-4.58 0-2.76-4.48-5-10-5z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                    x: "0",
                }
            }
            g {
                g {
                    rect {
                        x: "4",
                        y: "4",
                        width: "10",
                        height: "1",
                    }
                    path {
                        d: "M13,14h1v-3h1v-1l-1-4H4l-1,4v1h1v5h6v-5h3V14z M9,15H5v-4h4V15z M4.03,10l0.75-3h8.44l0.75,3H4.03z",
                    }
                    polygon {
                        points: "18,15 16,15 16,13 15,13 15,15 13,15 13,16 15,16 15,18 16,18 16,16 18,16",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    rect {
                        width: "15",
                        height: "2",
                        y: "4",
                        x: "2",
                    }
                    path {
                        d: "M15,17h2v-3h1v-2l-1-5H2l-1,5v2h1v6h9v-6h4V17z M9,18H4v-4h5V18z M3.04,12l0.6-3h11.72l0.6,3H3.04z",
                    }
                    polygon {
                        points: "23,18 20,18 20,15 18,15 18,18 15,18 15,20 18,20 18,23 20,23 20,20 23,20",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        g {
                            path {
                                d: "M13,6v3h3v2h-3v3h-2v-3H8V9h3V6H13z M18,10.2C18,6.57,15.35,4,12,4s-6,2.57-6,6.2 c0,2.34,1.95,5.44,6,9.14C16.05,15.64,18,12.54,18,10.2z M12,2c4.2,0,8,3.22,8,8.2c0,3.32-2.67,7.25-8,11.8 c-5.33-4.55-8-8.48-8-11.8C4,5.22,7.8,2,12,2z",
                            }
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
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M20 1v3h3v2h-3v3h-2V6h-3V4h3V1h2zm-8 12c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm1-9.94v2.02A6.53 6.53 0 0 0 12 5c-3.35 0-6 2.57-6 6.2 0 2.34 1.95 5.44 6 9.14 4.05-3.7 6-6.79 6-9.14V11h2v.2c0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8C4 6.22 7.8 3 12 3c.34 0 .67.02 1 .06z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                    x: "0",
                }
            }
            g {
                g {
                    rect {
                        width: "1",
                        y: "4",
                        height: "12",
                        x: "4",
                    }
                    rect {
                        y: "4",
                        x: "15",
                        width: "1",
                        height: "7",
                    }
                    rect {
                        x: "9.5",
                        y: "4",
                        width: "1",
                        height: "2",
                    }
                    rect {
                        width: "1",
                        height: "2",
                        x: "9.5",
                        y: "14",
                    }
                    polygon {
                        points: "18,15 16,15 16,13 15,13 15,15 13,15 13,16 15,16 15,18 16,18 16,16 18,16",
                    }
                    rect {
                        x: "9.5",
                        y: "9",
                        width: "1",
                        height: "2",
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
            }
            g {
                g {
                    polygon {
                        points: "20,18 20,15 18,15 18,18 15,18 15,20 18,20 18,23 20,23 20,20 23,20 23,18",
                    }
                    rect {
                        x: "18",
                        height: "9",
                        y: "4",
                        width: "2",
                    }
                    rect {
                        width: "2",
                        x: "4",
                        height: "16",
                        y: "4",
                    }
                    rect {
                        x: "11",
                        width: "2",
                        height: "4",
                        y: "4",
                    }
                    rect {
                        y: "10",
                        height: "4",
                        width: "2",
                        x: "11",
                    }
                    rect {
                        width: "2",
                        y: "16",
                        height: "4",
                        x: "11",
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
                    x: "0",
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M3.5,8H8c0-0.55-0.45-1-1-1H3.5C3.22,7,3,7.22,3,7.5C3,7.78,3.22,8,3.5,8z",
                    }
                    path {
                        d: "M17,11.51V7c0-0.55-0.45-1-1-1h-4.47l-0.8-0.8l1.06-1.06l-0.53-0.53L8.61,6.27L9.14,6.8l1.06-1.06l0.8,0.8V9 c0,0.55-0.45,1-1,1H7.93C7.66,9.74,7.36,9.52,7.02,9.36L6.83,9.8L6.48,9.66l0.19-0.44C6.3,9.08,5.91,9,5.5,9 C5.1,9,4.71,9.08,4.35,9.21l0.18,0.44L4.19,9.8L4,9.35c-0.72,0.34-1.3,0.92-1.65,1.64l0.45,0.19l-0.14,0.35l-0.44-0.19 C2.08,11.7,2,12.09,2,12.5c0,0.4,0.08,0.79,0.21,1.15l0.44-0.18l0.14,0.35L2.35,14c0.34,0.72,0.92,1.3,1.64,1.65l0.19-0.45 l0.35,0.14l-0.19,0.44C4.7,15.92,5.09,16,5.5,16c0.4,0,0.79-0.08,1.15-0.21l-0.18-0.44l0.35-0.14L7,15.65 c0.72-0.34,1.3-0.92,1.65-1.64L8.2,13.83l0.14-0.35l0.44,0.19c0.08-0.22,0.13-0.44,0.17-0.67h4.1C13.02,13.16,13,13.33,13,13.5 c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5C18,12.68,17.6,11.97,17,11.51z M5.5,15C4.12,15,3,13.88,3,12.5C3,11.12,4.12,10,5.5,10 S8,11.12,8,12.5C8,13.88,6.88,15,5.5,15z M13.51,12H8.95c-0.04-0.22-0.09-0.44-0.16-0.65l-0.44,0.18L8.2,11.19 c0,0,0.45-0.19,0.45-0.19H10c1.1,0,2-0.9,2-2V7h4v4.05C15.84,11.02,15.67,11,15.5,11C14.68,11,13.97,11.4,13.51,12z M15.5,15 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,15,15.5,15z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M4,9h5c0-1.1-0.9-2-2-2H4C3.45,7,3,7.45,3,8C3,8.55,3.45,9,4,9z",
                    }
                    path {
                        d: "M22,14.06V8c0-1.1-0.9-2-2-2h-6.29l-1.06-1.06l1.41-1.41l-0.71-0.71L9.82,6.35l0.71,0.71l1.41-1.41L13,6.71V9 c0,1.1-0.9,2-2,2H8.96c-0.22-0.16-0.45-0.3-0.69-0.43l-0.4,0.89l-0.46-0.21l0.4-0.9C7.26,10.13,6.64,10,6,10 c-0.53,0-1.04,0.11-1.52,0.26l0.34,0.91l-0.47,0.18L4,10.42c-1.06,0.46-1.91,1.28-2.43,2.31l0.89,0.4l-0.21,0.46l-0.9-0.4 C1.13,13.74,1,14.36,1,15c0,0.53,0.11,1.04,0.26,1.52l0.91-0.34l0.18,0.47L1.42,17c0.46,1.06,1.28,1.91,2.31,2.43l0.4-0.89 l0.46,0.21l-0.4,0.9C4.74,19.87,5.36,20,6,20c0.53,0,1.04-0.11,1.52-0.26l-0.34-0.91l0.47-0.18L8,19.58 c1.06-0.46,1.91-1.28,2.43-2.31l-0.89-0.4l0.21-0.46l0.9,0.4c0.1-0.26,0.18-0.54,0.24-0.82h5.16C16.03,16.16,16,16.33,16,16.5 c0,1.93,1.57,3.5,3.5,3.5s3.5-1.57,3.5-3.5C23,15.55,22.62,14.69,22,14.06z M6,18c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3 S7.66,18,6,18z M10.87,14c-0.04-0.18-0.08-0.35-0.13-0.52l-0.91,0.34l-0.18-0.47L10.58,13c0,0,0.42,0,0.42,0c2.21,0,4-1.79,4-4V8 h5v5.05C19.84,13.03,19.67,13,19.5,13c-0.95,0-1.81,0.38-2.44,1H10.87z M19.5,18c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5S20.33,18,19.5,18z",
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
                height: "20",
                width: "20",
                fill: "none",
            }
            path {
                d: "M14.01,14.5H5.35l6.19-9h4.15L14.01,14.5z M10.75,4L2.5,16h12.75L17.5,4H10.75z M11.88,8C10.84,8,10,8.84,10,9.88 s0.84,1.88,1.88,1.88s1.88-0.84,1.88-1.88S12.91,8,11.88,8z",
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
                height: "24",
                y: "0",
                fill: "none",
            }
            path {
                d: "M17.34,18H5.8l8.25-12h5.54L17.34,18z M13,4L2,20h17l3-16H13z M14.5,9c-1.38,0-2.5,1.12-2.5,2.5s1.12,2.5,2.5,2.5 s2.5-1.12,2.5-2.5S15.88,9,14.5,9z",
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
                width: "20",
                height: "20",
                fill: "none",
            }
            path {
                d: "M9.25,14C9.25,10.41,6,7.5,2,7.5V6c3.56,0,6.64,1.96,8,4.76c0.86-1.78,2.45-3.39,4.72-4.76L11.5,6V4.5H17V10h-1.5l0-2.72 c-1.82,1.09-4.75,3.32-4.75,6.72h1.5v1.5h-4.5V14H9.25z",
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
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M19,8.7c-2.46,1.5-5.5,4.17-6,8.3h2v2H9v-2h2c-0.5-4.5-4.37-8-9-8V7c4.39,0,8.22,2.55,10,6.3c1.38-2.97,3.86-5.03,5.96-6.31 L14,7V5h7v7h-2V8.7z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                }
                path {
                    d: "M8.38,9.41l-0.71,0.71c-0.47-0.5-0.93-1.1-1.25-1.96L7.4,7.91C7.66,8.54,8,9.01,8.38,9.41z M9.5,6l-3-3l-3,3h2.53 c0.02,0.43,0.07,0.83,0.14,1.19l0.97-0.24C7.08,6.66,7.04,6.34,7.02,6H9.5z M16.5,6l-3-3l-3,3h2.48c-0.12,2.22-1.02,3.11-1.9,3.96 c-0.39,0.37-0.78,0.75-1.08,1.23c-0.26-0.41-0.58-0.75-0.91-1.07l-0.71,0.71C9.03,11.47,9.5,11.99,9.5,13v4h1v-4c0,0,0,0,0,0h0 c0-1.1,0.53-1.61,1.27-2.32c0.94-0.9,2.07-2.03,2.21-4.68H16.5z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
                path {
                    d: "M9.78,11.16l-1.42,1.42c-0.68-0.69-1.34-1.58-1.79-2.94l1.94-0.49C8.83,10.04,9.28,10.65,9.78,11.16z M11,6L7,2L3,6h3.02 C6.04,6.81,6.1,7.54,6.21,8.17l1.94-0.49C8.08,7.2,8.03,6.63,8.02,6H11z M21,6l-4-4l-4,4h2.99c-0.1,3.68-1.28,4.75-2.54,5.88 c-0.5,0.44-1.01,0.92-1.45,1.55c-0.34-0.49-0.73-0.88-1.13-1.24L9.46,13.6C10.39,14.45,11,15.14,11,17c0,0,0,0,0,0h0v5h2v-5 c0,0,0,0,0,0c0-2.02,0.71-2.66,1.79-3.63c1.38-1.24,3.08-2.78,3.2-7.37H21z",
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
                d: "M8 9v1.5h2.25V15h1.5v-4.5H14V9H8zM6 9H3c-.55 0-1 .45-1 1v5h1.5v-1.5h2V15H7v-5c0-.55-.45-1-1-1zm-.5 3h-2v-1.5h2V12zM21 9h-4.5c-.55 0-1 .45-1 1v5H17v-4.5h1V14h1.5v-3.51h1V15H22v-5c0-.55-.45-1-1-1z",
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
                    fill: "none",
                    y: "0",
                    width: "20",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M17,10c0-0.68-0.1-1.34-0.29-1.97c0.49-0.55,0.6-1.36,0.21-2.03c-0.39-0.67-1.15-0.98-1.87-0.83 c-0.91-0.95-2.08-1.64-3.4-1.96C11.42,2.51,10.77,2,10,2S8.58,2.51,8.35,3.21c-1.32,0.32-2.49,1.01-3.4,1.96 C4.22,5.02,3.46,5.33,3.07,6C2.68,6.67,2.8,7.48,3.29,8.03C3.1,8.66,3,9.32,3,10s0.1,1.34,0.29,1.97C2.8,12.52,2.68,13.33,3.07,14 c0.39,0.67,1.15,0.98,1.87,0.83c0.33,0.35,0.69,0.66,1.09,0.93L4.99,18h1.66l0.71-1.52c0.32,0.13,0.65,0.23,0.99,0.32 C8.58,17.49,9.23,18,10,18s1.42-0.51,1.65-1.21c0.34-0.08,0.67-0.19,0.99-0.32L13.36,18h1.66l-1.04-2.24 c0.4-0.27,0.76-0.59,1.09-0.93c0.72,0.15,1.48-0.16,1.87-0.83c0.39-0.67,0.27-1.48-0.21-2.03C16.9,11.34,17,10.68,17,10z M11.46,15.3c-0.31-0.48-0.85-0.8-1.46-0.8s-1.15,0.32-1.46,0.8c-0.19-0.05-0.37-0.11-0.55-0.18l1.06-2.28 C9.35,12.93,9.67,13,10,13s0.65-0.07,0.95-0.17l1.06,2.28C11.83,15.18,11.65,15.24,11.46,15.3z M8.5,10c0-0.83,0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5s-0.67,1.5-1.5,1.5S8.5,10.83,8.5,10z M15.32,11.38c-0.57,0.03-1.11,0.34-1.42,0.87 c-0.31,0.53-0.3,1.16-0.04,1.67c-0.17,0.16-0.35,0.32-0.53,0.46l-1.1-2.37C12.7,11.47,13,10.78,13,10c0-1.66-1.34-3-3-3 s-3,1.34-3,3c0,0.78,0.3,1.47,0.78,2.01l-1.1,2.37c-0.19-0.14-0.37-0.29-0.53-0.46c0.26-0.51,0.26-1.14-0.04-1.67 c-0.31-0.53-0.85-0.84-1.42-0.87C4.57,10.94,4.5,10.48,4.5,10s0.07-0.94,0.18-1.38C5.25,8.59,5.8,8.28,6.1,7.75 c0.31-0.53,0.3-1.16,0.04-1.67C6.8,5.44,7.62,4.96,8.54,4.7C8.85,5.18,9.39,5.5,10,5.5s1.15-0.32,1.46-0.8 c0.91,0.25,1.73,0.73,2.39,1.38c-0.26,0.51-0.26,1.14,0.04,1.67c0.31,0.53,0.85,0.84,1.42,0.87c0.11,0.44,0.18,0.9,0.18,1.38 S15.43,10.94,15.32,11.38z",
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
                    g {
                        path {
                            d: "M20.15,14.42c0.23-0.77,0.35-1.58,0.35-2.42s-0.12-1.65-0.35-2.42c0.78-0.6,1.02-1.7,0.51-2.58s-1.58-1.23-2.49-0.85 c-1.11-1.17-2.56-2.03-4.18-2.42C13.85,2.75,13.01,2,12,2s-1.85,0.75-1.98,1.73C8.39,4.12,6.95,4.98,5.83,6.15 C4.92,5.77,3.85,6.12,3.34,7S3.07,8.98,3.85,9.58C3.62,10.35,3.5,11.16,3.5,12s0.12,1.65,0.35,2.42c-0.78,0.6-1.02,1.7-0.51,2.58 s1.58,1.23,2.49,0.85c0.4,0.42,0.83,0.79,1.3,1.12L5.78,22h1.88l0.98-2.19c0.44,0.19,0.9,0.34,1.38,0.46 C10.15,21.25,10.99,22,12,22s1.85-0.75,1.98-1.73c0.46-0.11,0.91-0.26,1.34-0.44L16.3,22h1.88l-1.34-3 c0.48-0.34,0.93-0.72,1.34-1.15c0.91,0.38,1.99,0.03,2.49-0.85C21.17,16.12,20.93,15.02,20.15,14.42z M13.56,18.75 C13.19,18.29,12.63,18,12,18s-1.2,0.29-1.57,0.75c-0.4-0.09-0.79-0.21-1.16-0.37l1.43-3.19c0.4,0.16,0.84,0.25,1.3,0.25 c0.44,0,0.87-0.08,1.26-0.23l1.42,3.18C14.32,18.54,13.95,18.66,13.56,18.75z M10.48,12.02c0-0.83,0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5s-0.67,1.5-1.5,1.5S10.48,12.85,10.48,12.02z M18.71,14.01c-0.61,0.07-1.18,0.41-1.52,0.99 c-0.32,0.56-0.34,1.2-0.12,1.75c-0.28,0.29-0.58,0.55-0.9,0.79l-1.5-3.35c0.49-0.59,0.78-1.34,0.78-2.16 c0-1.89-1.55-3.41-3.46-3.41s-3.46,1.53-3.46,3.41c0,0.8,0.28,1.54,0.75,2.13l0,0l-1.52,3.39c-0.31-0.23-0.6-0.48-0.87-0.76 C7.15,16.23,7.13,15.57,6.8,15c-0.34-0.59-0.93-0.94-1.56-0.99c-0.22-0.68-0.33-1.4-0.33-2.15c0-0.64,0.09-1.26,0.25-1.85 c0.66-0.03,1.3-0.38,1.65-1c0.37-0.63,0.35-1.38,0.01-1.98C7.74,6.05,8.93,5.34,10.27,5c0.34,0.59,0.99,1,1.73,1s1.39-0.4,1.73-1 c1.34,0.34,2.53,1.07,3.44,2.05C16.85,7.64,16.84,8.38,17.2,9c0.35,0.6,0.96,0.95,1.6,1c0.16,0.59,0.25,1.21,0.25,1.86 C19.05,12.61,18.93,13.33,18.71,14.01z",
                        }
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
                    y: "0",
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M16.5,6H12V3.5C12,2.67,11.33,2,10.5,2h-1C8.67,2,8,2.67,8,3.5V6H3.5C2.67,6,2,6.67,2,7.5v9C2,17.33,2.67,18,3.5,18h13 c0.83,0,1.5-0.67,1.5-1.5v-9C18,6.67,17.33,6,16.5,6z M9.5,3.5h1v4h-1V3.5z M16.5,16.5h-13v-9H8C8,8.33,8.67,9,9.5,9h1 C11.33,9,12,8.33,12,7.5h4.5V16.5z",
                        }
                    }
                    g {
                        rect {
                            height: "1.5",
                            x: "12",
                            y: "10",
                            width: "3",
                        }
                    }
                    g {
                        rect {
                            height: "1.5",
                            width: "3",
                            x: "12",
                            y: "12.5",
                        }
                    }
                    g {
                        circle {
                            r: "1.25",
                            cx: "7.5",
                            cy: "11.25",
                        }
                    }
                    g {
                        path {
                            d: "M9.24,13.36C8.7,13.13,8.12,13,7.5,13c-0.62,0-1.2,0.13-1.74,0.36C5.3,13.56,5,14.01,5,14.52V15h5v-0.48 C10,14.01,9.7,13.56,9.24,13.36z",
                        }
                    }
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
                    fill: "none",
                    width: "24",
                    height: "24",
                    y: "0",
                }
            }
            g {
                g {
                    rect {
                        width: "4",
                        x: "14",
                        height: "1.5",
                        y: "12",
                    }
                    rect {
                        x: "14",
                        y: "15",
                        height: "1.5",
                        width: "4",
                    }
                    path {
                        d: "M20,7h-5V4c0-1.1-0.9-2-2-2h-2C9.9,2,9,2.9,9,4v3H4C2.9,7,2,7.9,2,9v11c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V9 C22,7.9,21.1,7,20,7z M11,7V4h2v3v2h-2V7z M20,20H4V9h5c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2h5V20z",
                    }
                    circle {
                        cy: "13.5",
                        cx: "9",
                        r: "1.5",
                    }
                    path {
                        d: "M11.08,16.18C10.44,15.9,9.74,15.75,9,15.75s-1.44,0.15-2.08,0.43C6.36,16.42,6,16.96,6,17.57V18h6v-0.43 C12,16.96,11.64,16.42,11.08,16.18z",
                    }
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
                    height: "20",
                    y: "0",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M16.93,9.12c0.12-0.3,0.09-0.23,0.13-0.35c0.24-0.96-0.27-1.97-1.19-2.34l-1.66-0.67c-0.39-0.16-0.8-0.18-1.2-0.08 c-0.09-0.29-0.23-0.56-0.44-0.78c-0.38-0.41-0.91-0.64-1.46-0.64H8.88c-0.55,0-1.09,0.23-1.46,0.64c-0.21,0.23-0.35,0.5-0.44,0.79 C6.59,5.58,6.17,5.6,5.78,5.76L4.12,6.43C3.21,6.8,2.7,7.81,2.94,8.77l0.14,0.35C1.06,13.03,1,13,1,13.66 c0,1.58,1.65,2.56,3.01,1.87l1.22-0.61h9.54l1.22,0.61c1.38,0.7,3.01-0.31,3.01-1.87C19,12.99,18.93,13.01,16.93,9.12z M3.34,14.19c-0.49,0.25-0.84-0.22-0.84-0.53c0-0.2-0.05-0.05,1.31-2.69l0.98,2.48L3.34,14.19z M6.38,13.41l-2-5.04 c-0.04-0.23,0.09-0.46,0.3-0.55l1.66-0.67c0.31-0.12,0.64,0.09,0.66,0.42l0.52,5.84H6.38z M10.96,13.41H9.04L8.39,6.29 C8.37,5.98,8.61,5.75,8.88,5.75h2.24c0.27,0,0.51,0.23,0.49,0.54L10.96,13.41z M13.62,13.41h-1.15l0.52-5.84 c0.03-0.33,0.36-0.54,0.66-0.42l1.66,0.67c0.22,0.09,0.34,0.32,0.3,0.55L13.62,13.41z M16.66,14.19l-1.45-0.73l0.98-2.48l1.24,2.4 C17.7,13.89,17.17,14.44,16.66,14.19z",
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
                    path {
                        d: "M20.5,10.94c0.13-0.32,0.1-0.23,0.15-0.39c0.3-1.21-0.34-2.47-1.5-2.93l-2.01-0.8c-0.46-0.18-0.95-0.21-1.41-0.12 c-0.11-0.33-0.29-0.63-0.52-0.89C14.73,5.29,14.06,5,13.36,5h-2.71C9.94,5,9.27,5.29,8.8,5.81C8.56,6.07,8.38,6.37,8.27,6.69 C7.81,6.6,7.32,6.63,6.86,6.81l-2.01,0.8c-1.16,0.46-1.8,1.72-1.5,2.93l0.15,0.38C1.1,15.55,1,15.55,1,16.38 c0,0.91,0.46,1.74,1.24,2.22c1.42,0.88,2.49,0.14,4-0.61h11.53c1.52,0.76,1.86,1.01,2.63,1.01c1,0,2.61-0.77,2.61-2.61 C23,15.54,22.88,15.51,20.5,10.94z M20.12,16.93l-1.68-0.84l1.08-2.7l1.41,2.71C21.21,16.63,20.64,17.19,20.12,16.93z M15.09,15.99l0.62-6.9c0.03-0.33,0.37-0.54,0.68-0.42l2.01,0.8c0.22,0.09,0.34,0.31,0.31,0.54l-2.4,5.98H15.09z M7.68,15.99 l-2.4-5.98C5.25,9.78,5.37,9.56,5.59,9.47l2.01-0.8c0.31-0.12,0.65,0.08,0.68,0.42l0.62,6.9H7.68z M3.07,16.1l1.41-2.72l1.08,2.71 l-1.68,0.84C3.36,17.19,2.79,16.63,3.07,16.1z M10.15,7.54c-0.03-0.31,0.23-0.54,0.5-0.54h2.71c0.27,0,0.53,0.23,0.5,0.54 l-0.77,8.45h-2.17L10.15,7.54z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 1H5c-1.1 0-1.99.9-1.99 2L3 15.93c0 .69.35 1.3.88 1.66L12 23l8.11-5.41c.53-.36.88-.97.88-1.66L21 3c0-1.1-.9-2-2-2zm-7 19.6l-7-4.66V3h14v12.93l-7 4.67zm-2.01-7.42l-2.58-2.59L6 12l4 4 8-8-1.42-1.42z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M9,13c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S10.1,13,9,13z M9,16c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S9.55,16,9,16z",
                    }
                    path {
                        d: "M9.24,12L7.82,5.78C7.72,5.32,7.31,5,6.84,5H4.01v1h2.84l1.17,5.14c-1.57,0.4-2.75,1.72-2.96,3.36H1v1h5.01v-0.51 C6.01,13.34,7.35,12,9,12L9.24,12z",
                    }
                    path {
                        d: "M15.5,8h-0.68l-1.58-4.34C13.1,3.26,12.72,3,12.3,3H10v1h2.3l1.46,4h-4.4l0.23,1h3.45c-0.53,0.52-0.88,1.22-0.98,2h-2.01 l0.23,1h1.79c0.25,1.81,1.83,3.14,3.75,2.99c1.64-0.13,3.01-1.46,3.18-3.1C19.2,9.75,17.59,8,15.5,8z M15.5,14 c-1.4,0-2.5-1.1-2.5-2.5c0-0.94,0.5-1.73,1.24-2.16l1.03,2.83l0.94-0.34l-1.02-2.8C15.3,9.02,15.4,9,15.5,9c1.4,0,2.5,1.1,2.5,2.5 S16.9,14,15.5,14z",
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
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M10,14h0.74L8.82,5.56C8.61,4.65,7.8,4,6.87,4H3v2h3.87l1.42,6.25c0,0-0.01,0-0.01,0C6.12,12.9,4.47,14.73,4.09,17H0v2h6 v-1C6,15.79,7.79,14,10,14z",
                    }
                    path {
                        d: "M19,8h-0.82l-1.35-3.69C16.55,3.52,15.8,3,14.96,3H11v2h3.96l1.1,3H10.4l0.46,2H15c-0.43,0.58-0.75,1.25-0.9,2h-2.79 l0.46,2h2.33c0.44,2.23,2.31,3.88,4.65,3.99c2.8,0.13,5.25-2.19,5.25-5C24,10.2,21.8,8,19,8z M19,16c-1.68,0-3-1.32-3-3 c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64l1.88-0.68l-0.97-2.67c0.03,0,0.06-0.01,0.09-0.01c1.68,0,3,1.32,3,3S20.68,16,19,16z",
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
                    height: "20",
                    fill: "none",
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M18,6c0-1.66-1.34-3-3-3H5C3.34,3,2,4.34,2,6c0,1.3,0.84,2.4,2,2.82v6.68C4,16.33,4.67,17,5.5,17h9 c0.83,0,1.5-0.67,1.5-1.5l0-6.68C17.16,8.4,18,7.3,18,6z M15.5,7.4l-1,0.35l0,7.74h-9V7.76l-1-0.35c-0.6-0.21-1-0.78-1-1.4 c0-0.83,0.67-1.5,1.5-1.5h10c0.83,0,1.5,0.67,1.5,1.5C16.5,6.63,16.1,7.19,15.5,7.4z",
                    }
                    path {
                        d: "M10.71,8.29C10.51,8.1,10.26,8,10,8S9.49,8.1,9.29,8.29l-2,2c-0.39,0.39-0.39,1.02,0,1.41l2,2C9.49,13.9,9.74,14,10,14 c0.26,0,0.51-0.1,0.71-0.29l2-2c0.39-0.39,0.39-1.02,0-1.41L10.71,8.29z M10,12.29L8.71,11L10,9.71L11.29,11L10,12.29z",
                    }
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
                    height: "24",
                    width: "24",
                    fill: "none",
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M18,3H6C3.79,3,2,4.79,2,7c0,1.48,0.81,2.75,2,3.45V19c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-8.55c1.19-0.69,2-1.97,2-3.45 C22,4.79,20.21,3,18,3z M19,8.72L18,9.3V19H6V9.31L5.01,8.73C4.38,8.35,4,7.71,4,7c0-1.1,0.9-2,2-2h12c1.1,0,2,0.9,2,2 C20,7.71,19.62,8.36,19,8.72z",
                    }
                    path {
                        d: "M12.71,9.29C12.51,9.1,12.26,9,12,9s-0.51,0.1-0.71,0.29l-3,3c-0.39,0.39-0.39,1.02,0,1.41l3,3C11.49,16.9,11.74,17,12,17 s0.51-0.1,0.71-0.29l3-3c0.39-0.39,0.39-1.02,0-1.41L12.71,9.29z M12,14.58L10.41,13L12,11.41L13.59,13L12,14.58z",
                    }
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
                    height: "20",
                    y: "0",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    rect {
                        width: "11",
                        x: "2",
                        height: "1.5",
                        y: "16.5",
                    }
                    path {
                        d: "M12.5,13.5H9V12H6v1.5H2.5C2.22,13.5,2,13.72,2,14v1h11v-1C13,13.72,12.78,13.5,12.5,13.5z",
                    }
                    path {
                        d: "M18,9.76V2h-5v7.76c0,1.27,0.67,2.44,1.75,3.09V18H18v-1.5h-1.75v-3.65C17.33,12.2,18,11.04,18,9.76z M14.5,3.5h2V7h-2 V3.5z M15.5,11.55c-0.63-0.39-1-1.06-1-1.79V8.5h2v1.26C16.5,10.49,16.13,11.16,15.5,11.55z",
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
                    fill: "none",
                    height: "24",
                    y: "0",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M2,21.5C2,21.78,2.22,22,2.49,22h13.02c0.27,0,0.49-0.22,0.49-0.5V20H2V21.5z",
                    }
                    path {
                        d: "M15.5,16H11v-2H7v2H2.5C2.22,16,2,16.22,2,16.5V18h14v-1.5C16,16.22,15.78,16,15.5,16z",
                    }
                    path {
                        d: "M20.47,15.45c0.99-1.07,1.53-2.48,1.53-3.94V2h-6v9.47c0,1.48,0.58,2.92,1.6,4l0.4,0.42V22h4v-2h-2v-4.03L20.47,15.45z M18,4h2v4h-2V4z M19.03,14.07c-0.65-0.71-1.03-1.65-1.03-2.6V10h2v1.51C20,12.46,19.66,13.36,19.03,14.07z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                    y: "0",
                }
            }
            g {
                g {
                    g {
                        circle {
                            cx: "6.5",
                            cy: "15.5",
                            r: "1.5",
                        }
                    }
                    g {
                        circle {
                            cx: "13.5",
                            r: "1.5",
                            cy: "15.5",
                        }
                    }
                    g {
                        path {
                            d: "M4,11V8h7.29C11.1,7.37,11,6.7,11,6H4.43c0.83-0.71,2.98-1.09,6.65-0.98c0.1-0.7,0.3-1.37,0.59-1.99 C2.97,2.67,2,5.02,2,7v9.5c0,0.95,0.38,1.81,1,2.44V21c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1h8v1c0,0.55,0.45,1,1,1h1 c0.55,0,1-0.45,1-1v-2.06c0.62-0.63,1-1.49,1-2.44V13c-1.91,0-3.63-0.76-4.89-2H4z M16,16c0,1.1-0.9,2-2,2H6c-1.1,0-2-0.9-2-2v-3 h12V16z",
                        }
                    }
                    g {
                        path {
                            d: "M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,9h-1V8h1V9z M18.5,7h-1V3h1V7z",
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
                    x: "0",
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M19,5c0,2.21-1.79,4-4,4s-4-1.79-4-4s1.79-4,4-4S19,2.79,19,5z M15.5,6.5C15.5,6.22,15.28,6,15,6s-0.5,0.22-0.5,0.5 S14.72,7,15,7S15.5,6.78,15.5,6.5z M15.5,3h-1v2h1V3z M6.5,12.25c-0.55,0-1-0.45-1-1s0.45-1,1-1c0.55,0,1,0.45,1,1 S7.05,12.25,6.5,12.25z M15.5,10.48l0,2.52h-11V9.5l7.34,0c-0.57-0.4-1.07-0.91-1.45-1.5H5.02l1-2.5h3.51 C9.47,4.99,9.51,4.39,9.59,4H6.02C5.4,4,4.85,4.37,4.62,4.94L3,9l0,6.5C3,15.78,3.22,16,3.5,16h1C4.78,16,5,15.78,5,15.5v-1h10v1 c0,0.28,0.22,0.5,0.5,0.5h1c0.28,0,0.5-0.22,0.5-0.5v-5.38C16.53,10.31,16.02,10.43,15.5,10.48z M13.5,10.25c-0.55,0-1,0.45-1,1 s0.45,1,1,1c0.55,0,1-0.45,1-1S14.05,10.25,13.5,10.25z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,7h-1V3h1V7z M18.5,8v1h-1V8H18.5z M6,13.5 C6,12.67,6.67,12,7.5,12S9,12.67,9,13.5S8.33,15,7.5,15S6,14.33,6,13.5z M19,12.93c0.65-0.09,1.34-0.28,2-0.6h0V19 c0,0.55-0.45,1-1,1h-1c-0.55,0-1-0.45-1-1v-1H6v1c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1v-8l2.08-5.99C5.29,4.42,5.84,4,6.5,4 l4.79,0C11.1,4.63,11,5.31,11,6H6.85L5.81,9h5.86v0c0.36,0.75,0.84,1.43,1.43,2L5,11v5h14L19,12.93z M17.91,13 c-0.89-0.01-1.74-0.19-2.53-0.51C15.15,12.76,15,13.11,15,13.5c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5 C18,13.32,17.97,13.16,17.91,13z",
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
                    height: "20",
                    width: "20",
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M12.84,8H7.19C6.78,8,6.4,8.26,6.26,8.65L5,12v4.5C5,16.78,5.22,17,5.5,17H6c0.28,0,0.5-0.22,0.5-0.5V16h7v0.5 c0,0.28,0.22,0.5,0.5,0.5h0.5c0.28,0,0.5-0.22,0.5-0.5V12l-1.22-3.34C13.64,8.26,13.26,8,12.84,8z M7.54,9.5h4.95l0.55,1.5H6.98 L7.54,9.5z M13.5,14.5h-7v-2h7V14.5z",
                    }
                    circle {
                        cy: "13.5",
                        cx: "7.75",
                        r: ".75",
                    }
                    circle {
                        cx: "12.25",
                        cy: "13.5",
                        r: ".75",
                    }
                    path {
                        d: "M9.37,2.75C9.05,1.74,8.11,1,7,1C5.62,1,4.5,2.12,4.5,3.5S5.62,6,7,6c1.11,0,2.05-0.74,2.37-1.75h3.13V6H14V4.25h1v-1.5 H9.37z M7,4.5c-0.54,0-1-0.46-1-1s0.46-1,1-1s1,0.46,1,1S7.54,4.5,7,4.5z",
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
                g {
                    g {
                        circle {
                            cx: "9",
                            cy: "16.5",
                            r: "1",
                        }
                        circle {
                            r: "1",
                            cy: "16.5",
                            cx: "15",
                        }
                        path {
                            d: "M17.25,9.6c-0.02-0.02-0.03-0.04-0.05-0.07C16.82,9.01,16.28,9,16.28,9H7.72c0,0-0.54,0.01-0.92,0.54 C6.78,9.56,6.77,9.58,6.75,9.6C6.68,9.71,6.61,9.84,6.56,10C6.34,10.66,5.82,12.22,5,14.69v6.5C5,21.64,5.35,22,5.78,22h0.44 C6.65,22,7,21.64,7,21.19V20h10v1.19c0,0.45,0.34,0.81,0.78,0.81h0.44c0.43,0,0.78-0.36,0.78-0.81v-6.5 c-0.82-2.46-1.34-4.03-1.56-4.69C17.39,9.84,17.32,9.71,17.25,9.6z M8.33,11h7.34l0.23,0.69L16.33,13H7.67L8.33,11z M17,18H7 v-2.99V15h10v0.01V18z",
                        }
                        path {
                            d: "M10.83,3C10.41,1.83,9.3,1,8,1C6.34,1,5,2.34,5,4c0,1.65,1.34,3,3,3c1.3,0,2.41-0.84,2.83-2H16v2h2V5h1V3H10.83z M8,5 C7.45,5,7,4.55,7,4s0.45-1,1-1s1,0.45,1,1S8.55,5,8,5z",
                        }
                    }
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
                    height: "20",
                    fill: "none",
                    width: "20",
                    y: "0",
                }
            }
            g {
                g {
                    polygon {
                        points: "4,13 4,14.5 9.25,14.5 9.25,17 10.75,17 10.75,14.5 16,14.5 16,13",
                    }
                    path {
                        d: "M5.5,12H6c0.28,0,0.5-0.22,0.5-0.5V11h7v0.5c0,0.28,0.22,0.5,0.5,0.5h0.5c0.28,0,0.5-0.22,0.5-0.5V7l-1.22-3.34 C13.64,3.26,13.26,3,12.84,3H7.19C6.78,3,6.4,3.26,6.26,3.65L5,7v4.5C5,11.78,5.22,12,5.5,12z M7.54,4.5h4.95L13.04,6H6.98 L7.54,4.5z M6.5,7.5h7v2h-7V7.5z",
                    }
                    circle {
                        r: ".75",
                        cy: "8.5",
                        cx: "7.75",
                    }
                    circle {
                        cy: "8.5",
                        r: ".75",
                        cx: "12.25",
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
                    height: "24",
                    fill: "none",
                    y: "0",
                }
            }
            g {
                g {
                    g {
                        circle {
                            cx: "9",
                            r: "1",
                            cy: "10.5",
                        }
                        circle {
                            cx: "15",
                            r: "1",
                            cy: "10.5",
                        }
                        path {
                            d: "M5.78,16h0.44C6.65,16,7,15.64,7,15.19V14h10v1.19c0,0.45,0.34,0.81,0.78,0.81h0.44c0.43,0,0.78-0.36,0.78-0.81v-6.5 c0,0-1.34-4.03-1.56-4.69c-0.05-0.16-0.12-0.29-0.19-0.4c-0.02-0.02-0.03-0.04-0.05-0.07C16.82,3.01,16.28,3,16.28,3H7.72 c0,0-0.54,0.01-0.92,0.54C6.78,3.56,6.77,3.58,6.75,3.6C6.68,3.71,6.61,3.84,6.56,4C6.34,4.66,5,8.69,5,8.69v6.5 C5,15.64,5.35,16,5.78,16z M8.33,5h7.34l0.23,0.69L16.33,7H7.67L8.33,5z M7,9.01V9h10v0.01V12H7V9.01z",
                        }
                        polygon {
                            points: "4,17.01 4,19 11,19 11,22 13,22 13,19 20,19 20,17.01",
                        }
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
                    width: "20",
                    x: "0",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    rect {
                        width: "1.5",
                        x: "7.5",
                        height: "3",
                        y: "7",
                    }
                    rect {
                        width: "1.5",
                        height: "3",
                        y: "7",
                        x: "11",
                    }
                    path {
                        d: "M17.5,7v2H16V2h-1.5v2h-2V2H11v2H9V2H7.5v2h-2V2H4v7H2.5V7H1v10h7v-2c0-1.1,0.9-2,2-2s2,0.9,2,2v2h7V7H17.5z M17.5,15.5 h-4V15c0-1.93-1.57-3.5-3.5-3.5S6.5,13.07,6.5,15v0.5h-4v-5h3v-5h9v5h3V15.5z",
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
            }
            g {
                g {
                    g {
                        path {
                            d: "M21,9v2h-2V3h-2v2h-2V3h-2v2h-2V3H9v2H7V3H5v8H3V9H1v12h9v-3c0-1.1,0.9-2,2-2s2,0.9,2,2v3h9V9H21z M21,19h-5v-1 c0-2.21-1.79-4-4-4s-4,1.79-4,4v1H3v-6h4V7h10v6h4V19z",
                        }
                    }
                    g {
                        rect {
                            width: "2",
                            height: "3",
                            x: "9",
                            y: "9",
                        }
                    }
                    g {
                        rect {
                            height: "3",
                            width: "2",
                            y: "9",
                            x: "13",
                        }
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
                d: "M12 2l-5.5 9h11L12 2zm0 3.84L13.93 9h-3.87L12 5.84zM17.5 13c-2.49 0-4.5 2.01-4.5 4.5s2.01 4.5 4.5 4.5 4.5-2.01 4.5-4.5-2.01-4.5-4.5-4.5zm0 7c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5zM3 21.5h8v-8H3v8zm2-6h4v4H5v-4z",
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
                        d: "M2,18l11.49-4.07L6.07,6.51L2,18z M10.75,13.31l-6.29,2.22l2.22-6.29L10.75,13.31z",
                    }
                    path {
                        d: "M12.25,5.63L9.78,8.1l0.71,0.71l2.47-2.47c0.78-0.78,0.78-2.05,0-2.83l-1.41-1.41L10.84,2.8l1.41,1.41 C12.64,4.6,12.64,5.24,12.25,5.63z",
                    }
                    path {
                        d: "M8.36,6.69l0.71,0.71l0.71-0.71c0.78-0.78,0.78-2.05,0-2.83L9.07,3.15L8.36,3.86l0.71,0.71c0.39,0.39,0.39,1.02,0,1.41 L8.36,6.69z",
                    }
                    path {
                        d: "M15.79,8.58c-0.51,0-1.02,0.2-1.41,0.59l-1.77,1.77l0.71,0.71l1.77-1.77c0.2-0.2,0.45-0.29,0.71-0.29s0.51,0.1,0.71,0.29 l1.41,1.41l0.71-0.71L17.2,9.16C16.81,8.77,16.3,8.58,15.79,8.58z",
                    }
                    path {
                        d: "M18.62,4.92c-0.39-0.39-0.9-0.59-1.41-0.59s-1.02,0.2-1.41,0.59l-4.6,4.6l0.71,0.71l4.6-4.6c0.2-0.2,0.45-0.29,0.71-0.29 s0.51,0.1,0.71,0.29l0.71,0.71l0.71-0.71L18.62,4.92z",
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
                    fill: "none",
                    y: "0",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M2,22l14-5L7,8L2,22z M12.35,16.18L5.3,18.7l2.52-7.05L12.35,16.18z",
                        }
                        path {
                            d: "M14.53,12.53l5.59-5.59c0.49-0.49,1.28-0.49,1.77,0l0.59,0.59l1.06-1.06l-0.59-0.59c-1.07-1.07-2.82-1.07-3.89,0 l-5.59,5.59L14.53,12.53z",
                        }
                        path {
                            d: "M10.06,6.88L9.47,7.47l1.06,1.06l0.59-0.59c1.07-1.07,1.07-2.82,0-3.89l-0.59-0.59L9.47,4.53l0.59,0.59 C10.54,5.6,10.54,6.4,10.06,6.88z",
                        }
                        path {
                            d: "M17.06,11.88l-1.59,1.59l1.06,1.06l1.59-1.59c0.49-0.49,1.28-0.49,1.77,0l1.61,1.61l1.06-1.06l-1.61-1.61 C19.87,10.81,18.13,10.81,17.06,11.88z",
                        }
                        path {
                            d: "M15.06,5.88l-3.59,3.59l1.06,1.06l3.59-3.59c1.07-1.07,1.07-2.82,0-3.89l-1.59-1.59l-1.06,1.06l1.59,1.59 C15.54,4.6,15.54,5.4,15.06,5.88z",
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
                    fill: "none",
                    x: "0",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M15,11V8l-4.25-2.55v-1.2h1.75v-1.5h-1.75V1h-1.5v1.75H7.5v1.5h1.75v1.2L5,8v3l-3,1v6h7v-3c0-0.55,0.45-1,1-1s1,0.45,1,1 v3h7v-6L15,11z M16.5,16.5h-4V15c0-1.38-1.12-2.5-2.5-2.5S7.5,13.62,7.5,15v1.5h-4v-3.42l3-1V8.85l3.5-2.1l3.5,2.1v3.23l3,1V16.5z",
                    }
                    circle {
                        cx: "10",
                        cy: "10",
                        r: "1.25",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M18,12.22V9l-5-2.5V5h2V3h-2V1h-2v2H9v2h2v1.5L6,9v3.22L2,14v8h9v-4c0-0.55,0.45-1,1-1s1,0.45,1,1v4h9v-8L18,12.22z M20,20h-5v-2.04c0-1.69-1.35-3.06-3-3.06c-1.65,0-3,1.37-3,3.06V20H4v-4.79l4-1.81v-3.35L12,8l4,2.04v3.35l4,1.81V20z",
                    }
                    circle {
                        cx: "12",
                        cy: "12",
                        r: "1.5",
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
                    fill: "none",
                    x: "0",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M13,9h-1V4c0-0.55-0.45-1-1-1H9C8.45,3,8,3.45,8,4v5H7c-1.66,0-3,1.34-3,3v5c6.51,0,12,0,12,0v-5C16,10.34,14.66,9,13,9z M9,4h2v5H9V4z M15,16h-2v-1.5c0-0.28-0.22-0.5-0.5-0.5S12,14.22,12,14.5V16h-1.5v-1.5c0-0.28-0.22-0.5-0.5-0.5s-0.5,0.22-0.5,0.5 V16H8v-1.5C8,14.22,7.78,14,7.5,14S7,14.22,7,14.5V16H5v-4c0-1.1,0.9-2,2-2h6c1.1,0,2,0.9,2,2V16z",
                    }
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M16,11h-1V3c0-1.1-0.9-2-2-2h-2C9.9,1,9,1.9,9,3v8H8c-2.76,0-5,2.24-5,5v7h18v-7C21,13.24,18.76,11,16,11z M11,3h2v8h-2V3 z M19,21h-2v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v3h-2v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v3H9v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v3H5 v-5c0-1.65,1.35-3,3-3h8c1.65,0,3,1.35,3,3V21z",
                    }
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
            path {
                d: "M12 12c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zm0 8c-1.65 0-3-1.35-3-3s1.35-3 3-3 3 1.35 3 3-1.35 3-3 3zm0-17C8.1 3 4.56 4.59 2 7.15l5 5c1.28-1.28 3.05-2.08 5-2.08s3.72.79 5 2.07l5-5C19.44 4.59 15.9 3 12 3zm4.84 6.47c-1.44-.91-3.1-1.4-4.84-1.4-1.74 0-3.41.49-4.85 1.41L4.94 7.26C6.99 5.79 9.44 5 12 5c2.56 0 5 .79 7.05 2.26l-2.21 2.21z",
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
                fill: "none",
                height: "20",
            }
            path {
                d: "M4,8.5l0.5-1.75L4,5h0.7l0.75,1H7.7L6.6,2.5h0.9L9.6,6h2.15c0.41,0,0.75,0.34,0.75,0.75S12.16,7.5,11.75,7.5H9.6L7.5,11H6.6 l1.1-3.5H5.45l-0.75,1H4z M15.5,13.25L16,11.5h-0.7l-0.75,1H12.3L13.4,9h-0.9l-2.1,3.5H8.25c-0.41,0-0.75,0.34-0.75,0.75 S7.84,14,8.25,14h2.15l2.1,3.5h0.9L12.3,14h2.25l0.75,1H16L15.5,13.25z",
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
                width: "24",
                height: "24",
                fill: "none",
            }
            path {
                d: "M15.4,17l1.3,4.4h-1.1L13,17h-3c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h3l2.6-4.4h1.1L15.4,15h2.85L19,14h1l-0.6,2l0.6,2h-1 l-0.75-1H15.4z M5.75,7L5,6H4l0.6,2L4,10h1l0.75-1H8.6l-1.3,4.4h1.1L11,9h3c0.55,0,1-0.45,1-1s-0.45-1-1-1h-3L8.4,2.6H7.3L8.6,7 H5.75z",
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
                    x: "0",
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
                    fill: "none",
                    height: "24",
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
                    y: "0",
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        width: "4",
                        y: "5.5",
                        x: "4",
                        height: "1.5",
                    }
                    path {
                        d: "M15.5,6.5C15.5,5.67,14.83,5,14,5h-2.5v1.5H14v1.41l-3,3.59H8V8H5c-1.66,0-3,1.34-3,3v2h1.5c0,1.38,1.12,2.5,2.5,2.5 s2.5-1.12,2.5-2.5h3.2l3.8-4.54V6.5z M3.5,11.5V11c0-0.83,0.67-1.5,1.5-1.5h1.5v2H3.5z M6,14c-0.55,0-1-0.45-1-1h2 C7,13.55,6.55,14,6,14z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                    y: "0",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M19,7c0-1.1-0.9-2-2-2h-3v2h3v2.65L13.52,14H10V9H6c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,10.35 V7z M4,14v-1c0-1.1,0.9-2,2-2h2v3H4z M7,17c-0.55,0-1-0.45-1-1h2C8,16.55,7.55,17,7,17z",
                        }
                        rect {
                            height: "2",
                            x: "5",
                            width: "5",
                            y: "6",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            circle {
                cy: "16.5",
                r: "1.5",
                cx: "5.5",
            }
            circle {
                cy: "16.5",
                r: "1.5",
                cx: "12.5",
            }
            path {
                d: "M16 1c-2.39 0-4.49 1.2-5.75 3.02C9.84 4.01 9.43 4 9 4c-4.42 0-8 .5-8 4v10c0 .88.39 1.67 1 2.22V22c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h8v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1.78c.61-.55 1-1.34 1-2.22v-3.08c3.39-.49 6-3.39 6-6.92 0-3.87-3.13-7-7-7zM9 6h.29c-.09.32-.16.66-.21.99H3.34C3.89 6.46 5.31 6 9 6zM3 8.99h6.08c.16 1.11.57 2.13 1.18 3.01H3V8.99zM15 18c0 .37-.21.62-.34.73l-.29.27H3.63l-.29-.27C3.21 18.62 3 18.37 3 18v-4h9.41c.78.47 1.65.79 2.59.92V18zm1-5c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm.5-9H15v5l3.62 2.16.75-1.23-2.87-1.68z",
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
                    x: "0",
                }
            }
            g {
                g {
                }
                g {
                    path {
                        d: "M15.71,13.21l-3.46-3.46l1.33-1.33l-2-2l-1.33,1.33L6.79,4.29C6.4,3.9,5.76,3.9,5.37,4.29L4.29,5.37 C3.9,5.76,3.9,6.4,4.29,6.79l3.46,3.46L4,14v2h2l3.75-3.75l3.46,3.46c0.39,0.39,1.02,0.39,1.41,0l1.08-1.08 C16.1,14.24,16.1,13.6,15.71,13.21z M5,6.08L6.08,5c0,0,0,0,0,0l0.69,0.69L6.23,6.23L6.89,6.9l0.54-0.54l1.06,1.06L7.95,7.96 l0.67,0.67l0.54-0.54l0.38,0.38L8.46,9.54L5,6.08z M5.59,15H5v-0.59l6.58-6.58l0.59,0.59L5.59,15z M13.92,15l-3.46-3.46l1.08-1.08 l0.4,0.4L11.4,11.4l0.67,0.67l0.54-0.54l1.06,1.06l-0.54,0.54l0.67,0.67l0.54-0.54L15,13.92L13.92,15z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                }
                g {
                    path {
                        d: "M20.97,7.27c0.39-0.39,0.39-1.02,0-1.41l-2.83-2.83c-0.39-0.39-1.02-0.39-1.41,0l-4.49,4.49L8.35,3.63 c-0.78-0.78-2.05-0.78-2.83,0l-1.9,1.9c-0.78,0.78-0.78,2.05,0,2.83l3.89,3.89L3,16.76V21h4.24l4.52-4.52l3.89,3.89 c0.95,0.95,2.23,0.6,2.83,0l1.9-1.9c0.78-0.78,0.78-2.05,0-2.83l-3.89-3.89L20.97,7.27z M5.04,6.94l1.89-1.9c0,0,0,0,0,0 l1.27,1.27L7.02,7.5l1.41,1.41l1.19-1.19l1.2,1.2l-1.9,1.9L5.04,6.94z M16.27,14.38l-1.19,1.19l1.41,1.41l1.19-1.19l1.27,1.27 l-1.9,1.9l-3.89-3.89l1.9-1.9L16.27,14.38z M6.41,19H5v-1.41l9.61-9.61l1.3,1.3l0.11,0.11L6.41,19z M16.02,6.56l1.41-1.41 l1.41,1.41l-1.41,1.41L16.02,6.56z",
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
                    fill: "none",
                    width: "20",
                    x: "0",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M16,3H4L2,8l8,9l8-9L16,3z M8.21,7.25L9.59,4.5h0.82l1.38,2.75H8.21z M9.25,8.75v5.15L4.67,8.75H9.25z M10.75,8.75h4.58 l-4.58,5.15V8.75z M16.08,7.25h-2.62L12.09,4.5h2.9L16.08,7.25z M5.02,4.5h2.9L6.54,7.25H3.92L5.02,4.5z",
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
            }
            g {
                path {
                    d: "M19,3H5L2,9l10,12L22,9L19,3z M9.62,8l1.5-3h1.76l1.5,3H9.62z M11,10v6.68L5.44,10H11z M13,10h5.56L13,16.68V10z M19.26,8 h-2.65l-1.5-3h2.65L19.26,8z M6.24,5h2.65l-1.5,3H4.74L6.24,5z",
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
                    height: "20",
                    y: "0",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M3,14.5h14c0-2.49-2.01-4.5-4.5-4.5c-2.03,0-3.72,1.35-4.28,3.19C7.86,12.92,7.45,12.73,7,12.61V8h1c1.1,0,2-0.9,2-2h7V5 h-7c0-1.1-0.9-2-2-2H3v1h1v1H3v1h1v1H3v1h1v5.26C3.58,13.59,3.23,14.01,3,14.5z M12.5,11.5c1.11,0,2.08,0.6,2.6,1.5H9.92 C10.45,12.08,11.42,11.5,12.5,11.5z M7,4h1.5v1H7V4z M7,6h1.5v1H7V6z M5,4h1v1H5V4z M5,6h1v1H5V6z M5,8h1v4.51 c-0.35,0.02-0.68,0.08-1,0.19V8z",
                    }
                    polygon {
                        points: "2,15.5 3.5,17 16.5,17 18,15.5",
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
                    fill: "none",
                    height: "24",
                    y: "0",
                }
            }
            g {
                g {
                    polygon {
                        points: "2,19 4,21 20,21 22,19",
                    }
                    path {
                        d: "M3,18h16.97c0,0,0,0,0,0c0.29-3.26-2.28-6-5.48-6c-2.35,0-4.35,1.48-5.14,3.55C8.94,15.32,8.48,15.17,8,15.08V9h1.75 C10.99,9,12,7.99,12,6.75h9v-1.5h-9C12,4.01,10.99,3,9.75,3H3v1.5h1v0.75H3v1.5h1V7.5H3V9h1v7.39C3.56,16.85,3.22,17.39,3,18z M14.5,14c0.99,0,1.91,0.4,2.58,1.14c0.24,0.26,0.44,0.55,0.58,0.86h-6.32C11.92,14.79,13.15,14,14.5,14z M8,4.5h2v0.75H8V4.5z M8,6.75h2V7.5H8V6.75z M5.5,4.5h1v0.75h-1V4.5z M5.5,6.75h1V7.5h-1V6.75z M5.5,9h1v6.06c-0.35,0.06-0.68,0.17-1,0.3V9z",
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
                d: "m22.43 10.59-9.01-9.01c-.75-.75-2.07-.76-2.83 0l-9 9c-.78.78-.78 2.04 0 2.82l9 9c.39.39.9.58 1.41.58.51 0 1.02-.19 1.41-.58l8.99-8.99c.79-.76.8-2.02.03-2.82zm-10.42 10.4-9-9 9-9 9 9-9 9zM8 11v4h2v-3h4v2.5l3.5-3.5L14 7.5V10H9c-.55 0-1 .45-1 1z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M15.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM5 12c-2.8 0-5 2.2-5 5s2.2 5 5 5 5-2.2 5-5-2.2-5-5-5zm0 8.5c-1.9 0-3.5-1.6-3.5-3.5s1.6-3.5 3.5-3.5 3.5 1.6 3.5 3.5-1.6 3.5-3.5 3.5zm5.8-10l2.4-2.4.8.8c1.3 1.3 3 2.1 5.1 2.1V9c-1.5 0-2.7-.6-3.6-1.5l-1.9-1.9c-.5-.4-1-.6-1.6-.6s-1.1.2-1.4.6L7.8 8.4c-.4.4-.6.9-.6 1.4 0 .6.2 1.1.6 1.4L11 14v5h2v-6.2l-2.2-2.3zM19 12c-2.8 0-5 2.2-5 5s2.2 5 5 5 5-2.2 5-5-2.2-5-5-5zm0 8.5c-1.9 0-3.5-1.6-3.5-3.5s1.6-3.5 3.5-3.5 3.5 1.6 3.5 3.5-1.6 3.5-3.5 3.5z",
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
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M13 3v1h-2V3h2m-1 7.11l5.38 1.77 2.39.78-1.12 3.97c-.54-.3-.94-.71-1.14-.94L16 13.96l-1.51 1.72c-.34.4-1.28 1.32-2.49 1.32s-2.15-.92-2.49-1.32L8 13.96l-1.51 1.72c-.2.23-.6.63-1.14.93l-1.13-3.96 2.4-.79L12 10.11M15 1H9v3H6c-1.1 0-2 .9-2 2v4.62l-1.29.42c-.26.08-.48.26-.6.5s-.15.52-.06.78L3.95 19H4c1.6 0 3.02-.88 4-2 .98 1.12 2.4 2 4 2s3.02-.88 4-2c.98 1.12 2.4 2 4 2h.05l1.89-6.68c.08-.26.06-.54-.06-.78s-.34-.42-.6-.5L20 10.62V6c0-1.1-.9-2-2-2h-3V1zM6 9.97V6h12v3.97L12 8 6 9.97zm10 9.71c-1.22.85-2.61 1.28-4 1.28s-2.78-.43-4-1.28C6.78 20.53 5.39 21 4 21H2v2h2c1.38 0 2.74-.35 4-.99 1.26.64 2.63.97 4 .97s2.74-.32 4-.97c1.26.65 2.62.99 4 .99h2v-2h-2c-1.39 0-2.78-.47-4-1.32z",
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
            rect {
                fill: "none",
                height: "24",
                width: "24",
            }
            path {
                d: "M19.77,12.66l-1.12,3.97c-0.78-0.43-1.07-0.86-2.65-2.67C14.4,15.78,13.57,17,12,17c-1.53,0-2.34-1.15-4-3.04 c-1.6,1.82-1.87,2.21-2.65,2.65l-1.13-3.96L12,10.11L19.77,12.66z M15,1H9v3H6C4.9,4,4,4.9,4,6v4.62l-1.29,0.42 c-0.63,0.19-0.81,0.84-0.66,1.28L3.95,19H4c1.6,0,3.02-0.88,4-2c0.98,1.12,2.4,2,4,2s3.02-0.88,4-2c0.98,1.12,2.4,2,4,2h0.05 l1.91-6.68c0.11-0.37,0.04-1.06-0.66-1.28L20,10.62V6c0-1.1-0.9-2-2-2h-3V1L15,1z M6,9.97V6h12v3.97L12,8L6,9.97L6,9.97z M16,19.68 c-1.22,0.85-2.61,1.28-4,1.28s-2.78-0.43-4-1.28C6.78,20.53,5.39,21,4,21H2v2h2c1.38,0,2.74-0.35,4-0.99c1.26,0.64,2.63,0.97,4,0.97 s2.74-0.32,4-0.97c1.26,0.65,2.62,0.99,4,0.99h2v-2h-2C18.61,21,17.22,20.53,16,19.68L16,19.68z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 2c-4.42 0-8 .5-8 4v10c0 .88.39 1.67 1 2.22V20c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h8v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1.78c.61-.55 1-1.34 1-2.22V6c0-3.5-3.58-4-8-4zm5.66 2.99H6.34C6.89 4.46 8.31 4 12 4s5.11.46 5.66.99zm.34 2V10H6V6.99h12zm-.34 9.74l-.29.27H6.63l-.29-.27C6.21 16.62 6 16.37 6 16v-4h12v4c0 .37-.21.62-.34.73z",
            }
            circle {
                cx: "8.5",
                cy: "14.5",
                r: "1.5",
            }
            circle {
                cy: "14.5",
                cx: "15.5",
                r: "1.5",
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
                    y: "0",
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    enable_background: "new",
                    path {
                        d: "M12,2C8,2,4,2.5,4,6v9.5c0,0.95,0.38,1.81,1,2.44V20c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1h8v1c0,0.55,0.45,1,1,1h1 c0.55,0,1-0.45,1-1v-2.06c0.62-0.63,1-1.49,1-2.44V6C20,2.5,16.42,2,12,2z M12,4c3.71,0,5.13,0.46,5.67,1H6.43 C7.03,4.48,8.48,4,12,4z M18,15c0,1.1-0.9,2-2,2H8c-1.1,0-2-0.9-2-2v-3h12V15z M18,10H6V7h12V10z",
                    }
                    circle {
                        cy: "14.5",
                        r: "1.5",
                        cx: "8.5",
                    }
                    circle {
                        r: "1.5",
                        cy: "14.5",
                        cx: "15.5",
                    }
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M18.92 6.01C18.72 5.42 18.16 5 17.5 5h-11c-.66 0-1.21.42-1.42 1.01L3 12v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.85 7h10.29l1.08 3.11H5.77L6.85 7zM19 17H5v-5h14v5z",
            }
            circle {
                cx: "7.5",
                cy: "14.5",
                r: "1.5",
            }
            circle {
                r: "1.5",
                cx: "16.5",
                cy: "14.5",
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
                    fill: "none",
                    height: "24",
                    y: "0",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18.92,6.01C18.72,5.42,18.16,5,17.5,5h-11C5.84,5,5.29,5.42,5.08,6.01L3,12v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1 h12v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-8L18.92,6.01z M6.85,7h10.29l1.04,3H5.81L6.85,7z M19,17H5v-5h14V17z",
                    }
                    circle {
                        r: "1.5",
                        cx: "7.5",
                        cy: "14.5",
                    }
                    circle {
                        cy: "14.5",
                        r: "1.5",
                        cx: "16.5",
                    }
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 1c-4.42 0-8 .5-8 4v10.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V5c0-3.5-3.58-4-8-4zm0 2c6 0 6 1.2 6 2H6c0-.8 0-2 6-2zm6 4v3H6V7h12zm-1.5 10h-9c-.83 0-1.5-.67-1.5-1.5V12h12v3.5c0 .83-.67 1.5-1.5 1.5zM12 12.5c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
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
                    y: "0",
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    enable_background: "new",
                    path {
                        d: "M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19L6,20v1h12v-1l-1.5-1c1.93,0,3.5-1.57,3.5-3.5V6C20,2.5,16.42,2,12,2z M12,4c3.71,0,5.13,0.46,5.67,1H6.43C7.03,4.48,8.48,4,12,4z M18,15.5c0,0.83-0.67,1.5-1.5,1.5h-9C6.67,17,6,16.33,6,15.5V12h12 V15.5z M18,10H6V7h12V10z",
                    }
                    circle {
                        r: "1.5",
                        cy: "14.5",
                        cx: "12",
                    }
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
                d: "M13.49 5.48c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-3.6 13.9l1-4.4 2.1 2v6h2v-7.5l-2.1-2 .6-3c1.3 1.5 3.3 2.5 5.5 2.5v-2c-1.9 0-3.5-1-4.3-2.4l-1-1.6c-.4-.6-1-1-1.7-1-.3 0-.5.1-.8.1l-5.2 2.2v4.7h2v-3.4l1.8-.7-1.6 8.1-4.9-1-.4 2 7 1.4z",
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
                d: "M12 2c-4 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-3.58-4-8-4zm5.66 3H6.43c.61-.52 2.06-1 5.57-1 3.71 0 5.12.46 5.66 1zM11 7v3H6V7h5zm2 0h5v3h-5V7zm3.5 10h-9c-.83 0-1.5-.67-1.5-1.5V12h12v3.5c0 .83-.67 1.5-1.5 1.5z",
            }
            circle {
                cx: "8.5",
                cy: "14.5",
                r: "1.5",
            }
            circle {
                cx: "15.5",
                cy: "14.5",
                r: "1.5",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19L6,20v1h12v-1l-1.5-1c1.93,0,3.5-1.57,3.5-3.5V6C20,2.5,16.42,2,12,2z M12,4c3.71,0,5.13,0.46,5.67,1H13h-2H6.43C7.03,4.48,8.48,4,12,4z M6,7h5v3H6V7z M18,15.5c0,0.83-0.67,1.5-1.5,1.5h-9 C6.67,17,6,16.33,6,15.5V12h12V15.5z M18,10h-5V7h5V10z",
                    }
                    circle {
                        r: "1.5",
                        cx: "8.5",
                        cy: "14.5",
                    }
                    circle {
                        r: "1.5",
                        cx: "15.5",
                        cy: "14.5",
                    }
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
                d: "M12 2c-4 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-3.58-4-8-4zm5.66 3H6.43c.61-.52 2.06-1 5.57-1 3.71 0 5.12.46 5.66 1zM11 7v3H6V7h5zm2 0h5v3h-5V7zm3.5 10h-9c-.83 0-1.5-.67-1.5-1.5V12h12v3.5c0 .83-.67 1.5-1.5 1.5z",
            }
            circle {
                r: "1.5",
                cx: "8.5",
                cy: "14.5",
            }
            circle {
                r: "1.5",
                cx: "15.5",
                cy: "14.5",
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
                    width: "24",
                    y: "0",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19L6,20v1h12v-1l-1.5-1c1.93,0,3.5-1.57,3.5-3.5V6C20,2.5,16.42,2,12,2z M12,4c3.71,0,5.13,0.46,5.67,1H13h-2H6.43C7.03,4.48,8.48,4,12,4z M6,7h5v3H6V7z M18,15.5c0,0.83-0.67,1.5-1.5,1.5h-9 C6.67,17,6,16.33,6,15.5V12h12V15.5z M18,10h-5V7h5V10z",
                    }
                    circle {
                        r: "1.5",
                        cx: "8.5",
                        cy: "14.5",
                    }
                    circle {
                        cy: "14.5",
                        cx: "15.5",
                        r: "1.5",
                    }
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M13.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM9.8 8.9L7 23h2.1l1.8-8 2.1 2v6h2v-7.5l-2.1-2 .6-3C14.8 12 16.8 13 19 13v-2c-1.9 0-3.5-1-4.3-2.4l-1-1.6c-.56-.89-1.68-1.25-2.65-.84L6 8.3V13h2V9.6l1.8-.7",
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
                    height: "20",
                    fill: "none",
                    y: "0",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M16.97,10.87c-0.11-0.62-0.55-1.12-1.13-1.37l-5.09-2.18V6.25c0-0.41-0.34-0.75-0.75-0.75c-0.62,0-1-0.56-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1h1.5C12.5,3.12,11.38,2,10,2C9.82,2,9.64,2.02,9.45,2.06C8.5,2.26,7.73,3.05,7.55,4.01 c-0.25,1.33,0.55,2.5,1.7,2.86v0.45L4.16,9.5c-0.57,0.25-1.02,0.75-1.13,1.37C2.83,12.01,3.7,13,4.81,13H6v5h8v-5h1.19 C16.3,13,17.17,12.01,16.97,10.87z M12.5,16.5h-5v-4h5V16.5z M15.19,11.5H14V11H6v0.5H4.81c-0.17,0-0.31-0.14-0.31-0.31 c0-0.12,0.07-0.23,0.19-0.28L10,8.63l5.31,2.28c0.11,0.05,0.19,0.16,0.19,0.28C15.5,11.36,15.36,11.5,15.19,11.5z",
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
                    fill: "none",
                    width: "24",
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M19.56,11.36L13,8.44V7c0-0.55-0.45-1-1-1l0,0c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1h2c0-1.84-1.66-3.3-3.56-2.95 C10.26,2.27,9.29,3.22,9.06,4.4C8.76,5.96,9.66,7.34,11,7.82v0.63l-6.56,2.92C3.56,11.75,3,12.62,3,13.57v0.01 C3,14.92,4.08,16,5.42,16H7v6h10v-6h1.58c1.34,0,2.42-1.08,2.42-2.42v-0.01C21,12.62,20.44,11.75,19.56,11.36z M15,20H9v-5h6V20z M18.58,14H17v-1H7v1H5.42c-0.46,0-0.58-0.65-0.17-0.81l6.75-3l6.75,3C19.17,13.38,19.03,14,18.58,14z",
                    }
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
                d: "M17.63 7H6.37C3.96 7 2 9.24 2 12s1.96 5 4.37 5h11.26c2.41 0 4.37-2.24 4.37-5s-1.96-5-4.37-5zm0 8H6.37C5.09 15 4 13.63 4 12s1.09-3 2.37-3h11.26C18.91 9 20 10.37 20 12s-1.09 3-2.37 3zM7.24 13.06l-1.87-1.87-.7.7 2.57 2.57 4.22-4.22-.7-.7z",
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
            path {
                fill: "none",
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M18.17 4.91L17.1 3.84l-5.55 5.55v1.08h1.08l5.54-5.56zM16 2.74l1.29-1.29a1.49 1.49 0 0 1 2.12 0l1.15 1.15c.59.59.59 1.54 0 2.12l-.68.68-.02.02-.58.58-6 6H10V8.74l6-6zm-2.28-.55l-.55.55-1.27 1.27c-3.3.05-5.9 2.6-5.9 6.2 0 2.34 1.95 5.44 6 9.14 4.05-3.7 6-6.79 6-9.14v-.1l1.8-1.8c.13.6.2 1.24.2 1.9 0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8 0-4.98 3.8-8.2 8-8.2.58 0 1.16.06 1.72.18z",
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
                    y: "0",
                    width: "20",
                    height: "20",
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
                            d: "M14.91,7.62C14.96,7.91,15,8.2,15,8.5c0,2.47-3.1,5.8-5,7.53C8.1,14.3,5,10.97,5,8.5c0-2.76,2.24-5,5-5 c0.58,0,1.14,0.12,1.66,0.3l1.16-1.16C11.96,2.24,11.01,2,10,2C6.41,2,3.5,4.91,3.5,8.5C3.5,12.84,10,18,10,18s6.5-5.16,6.5-9.5 c0-0.74-0.13-1.45-0.36-2.11L14.91,7.62z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                    y: "0",
                }
            }
            g {
                path {
                    d: "M11,11h2.12l6.16-6.16l-2.12-2.12L11,8.88V11z M20.71,2L20,1.29c-0.39-0.39-1.02-0.39-1.41,0l-0.72,0.72l2.12,2.12 l0.72-0.72C21.1,3.02,21.1,2.39,20.71,2z M17.9,9.05C17.96,9.41,18,9.79,18,10.2c0,1.71-1.08,4.64-6,9.14c-4.92-4.49-6-7.43-6-9.14 C6,6.17,9.09,4,12,4c0.32,0,0.65,0.03,0.97,0.08l1.65-1.65C13.78,2.16,12.9,2,12,2c-4.2,0-8,3.22-8,8.2c0,3.32,2.67,7.25,8,11.8 c5.33-4.55,8-8.48,8-11.8c0-1.01-0.16-1.94-0.45-2.8L17.9,9.05z",
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
                    x: "0",
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    rect {
                        width: "1",
                        x: "4",
                        y: "4",
                        height: "12",
                    }
                    rect {
                        width: "1",
                        x: "8.5",
                        height: "2",
                        y: "4",
                    }
                    rect {
                        width: "1",
                        x: "8.5",
                        height: "2",
                        y: "14",
                    }
                    rect {
                        width: "1",
                        height: "2",
                        x: "8.5",
                        y: "9",
                    }
                    path {
                        d: "M18.07,9.64l-0.71-0.71c-0.39-0.39-1.02-0.39-1.41,0L11,13.88V16h2.12l4.95-4.95C18.46,10.66,18.46,10.03,18.07,9.64z M12.71,15H12v-0.71l3.24-3.24l0.71,0.71L12.71,15z",
                    }
                    polygon {
                        points: "14,4 13,4 13,10.46 14,9.46",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    polygon {
                        points: "18,4 16,4 16,11.9 18,9.9",
                    }
                    rect {
                        height: "16",
                        y: "4",
                        width: "2",
                        x: "4",
                    }
                    rect {
                        x: "10",
                        height: "4",
                        y: "4",
                        width: "2",
                    }
                    rect {
                        width: "2",
                        height: "4",
                        x: "10",
                        y: "10",
                    }
                    rect {
                        y: "16",
                        x: "10",
                        height: "4",
                        width: "2",
                    }
                    path {
                        d: "M22.56,12.59l-1.15-1.15c-0.59-0.59-1.54-0.59-2.12,0L14,16.73V20h3.27l5.29-5.29C23.15,14.12,23.15,13.17,22.56,12.59z M16.58,18.45h-1.03v-1.03L19,13.97L20.03,15L16.58,18.45z",
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
                    x: "0",
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M10,3c-2.75,0-5.5,4.93-5.5,8.56C4.5,14.56,6.96,17,10,17c3.04,0,5.5-2.44,5.5-5.44C15.5,7.93,12.75,3,10,3z M10,15.5 c-2.21,0-4-1.77-4-3.94c0-3.2,2.46-7.06,4-7.06c1.54,0,4,3.85,4,7.06C14,13.73,12.21,15.5,10,15.5z",
                    }
                    path {
                        d: "M10.25,13c-1.21,0-1.75-1.13-1.75-2.25C8.5,10.34,8.16,10,7.75,10S7,10.34,7,10.75c0,2.17,1.37,3.75,3.25,3.75 c0.41,0,0.75-0.34,0.75-0.75S10.66,13,10.25,13z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,3C8.5,3,5,9.33,5,14c0,3.87,3.13,7,7,7c3.87,0,7-3.13,7-7C19,9.33,15.5,3,12,3z M12,19c-2.76,0-5-2.24-5-5 c0-4.09,3.07-9,5-9s5,4.91,5,9C17,16.76,14.76,19,12,19z",
                    }
                    path {
                        d: "M13,16c-0.58,0-3-0.08-3-3c0-0.55-0.45-1-1-1s-1,0.45-1,1c0,3,1.99,5,5,5c0.55,0,1-0.45,1-1S13.55,16,13,16z",
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
                    width: "20",
                    fill: "none",
                    x: "0",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M15.6,7.6C14,6,13.19,2,8.38,2C4.36,2,1.95,5.2,2,9.62S5.97,16,8.38,16c1.61,0,1.61,2,4.82,2C15.6,18,18,15.6,18,12.42 C18,10,17.21,9.2,15.6,7.6z M13.19,16c-1.13,0-1.45-0.28-1.98-0.76C10.65,14.75,9.8,14,8.38,14C7.22,14,4.04,12.96,4,9.6 c-0.02-1.9,0.51-3.49,1.48-4.47C6.22,4.38,7.19,4,8.38,4c2.33,0,3.17,1.15,4.35,3.03c0.43,0.69,0.88,1.41,1.46,1.99 c1.5,1.49,1.81,1.8,1.81,3.4C16,14.47,14.52,16,13.19,16z",
                        }
                    }
                    g {
                        circle {
                            cx: "10",
                            cy: "10",
                            r: "2.5",
                        }
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M19,9C17,7,15.99,2,9.97,2C4.95,2,1.94,6,2,11.52C2.06,17.04,6.96,19,9.97,19c2.01,0,2.01,3,6.02,3C19,22,22,19,22,15.02 C22,12,21.01,11,19,9z M15.99,20c-1.49,0-1.96-0.5-2.68-1.26C12.66,18.05,11.66,17,9.97,17C8.33,17,4.05,16.01,4,11.5 C3.97,8.99,4.68,6.88,5.99,5.55C7.01,4.52,8.35,4,9.97,4c3.34,0,4.51,1.86,5.86,4.02c0.55,0.88,1.07,1.71,1.76,2.39 c1.9,1.89,2.41,2.4,2.41,4.61C20,17.87,17.88,20,15.99,20z",
                        }
                    }
                    g {
                        circle {
                            r: "3.5",
                            cx: "12",
                            cy: "12",
                        }
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
                    fill: "none",
                    height: "20",
                    x: "0",
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
                        d: "M9,11v1H8v2h1l0,1c0,0.55,0.45,1,1,1h3v-6h-3C9.45,10,9,10.45,9,11z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
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
                        d: "M12,14h-2v4h2c0,1.1,0.9,2,2,2h3v-8h-3C12.9,12,12,12.9,12,14z",
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
                    width: "20",
                    height: "20",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M15.5,7h-0.68l-1.58-4.34C13.1,2.26,12.72,2,12.3,2H10v1h2.3l1.46,4H8.75L8.38,6H10V5H6v1h1.32l1.46,4H7.95 C7.7,8.19,6.13,6.86,4.2,7.01c-1.64,0.13-3.01,1.46-3.18,3.1C0.8,12.25,2.41,14,4.5,14c1.79,0,3.21-1.29,3.45-3h4.1 c0.25,1.81,1.83,3.14,3.75,2.99c1.64-0.13,3.01-1.46,3.18-3.1C19.2,8.75,17.59,7,15.5,7z M6.95,11c-0.23,1.15-1.22,2-2.45,2 C3.1,13,2,11.9,2,10.5S3.1,8,4.5,8c1.23,0,2.23,0.85,2.45,2H4v1H6.95z M12.05,10H9.84L9.11,8h3.92C12.5,8.52,12.16,9.22,12.05,10z M15.5,13c-1.4,0-2.5-1.1-2.5-2.5c0-0.94,0.5-1.73,1.24-2.16l1.03,2.83l0.94-0.34l-1.02-2.8C15.3,8.02,15.4,8,15.5,8 c1.4,0,2.5,1.1,2.5,2.5S16.9,13,15.5,13z",
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
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M19,7h-0.82l-1.7-4.68C16.19,1.53,15.44,1,14.6,1H12v2h2.6l1.46,4h-4.81l-0.36-1H12V4H7v2h1.75l1.82,5H9.9 C9.46,8.77,7.59,7.12,5.25,7.01C2.45,6.87,0,9.2,0,12c0,2.8,2.2,5,5,5c2.46,0,4.45-1.69,4.9-4h4.2c0.44,2.23,2.31,3.88,4.65,3.99 c2.8,0.13,5.25-2.19,5.25-5C24,9.2,21.8,7,19,7z M7.82,13c-0.4,1.17-1.49,2-2.82,2c-1.68,0-3-1.32-3-3s1.32-3,3-3 c1.33,0,2.42,0.83,2.82,2H5v2H7.82z M14.1,11h-1.4l-0.73-2H15C14.56,9.58,14.24,10.25,14.1,11z M19,15c-1.68,0-3-1.32-3-3 c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64l1.88-0.68l-0.97-2.67C18.94,9.01,18.97,9,19,9c1.68,0,3,1.32,3,3S20.68,15,19,15z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                    x: "0",
                }
            }
            g {
                g {
                    circle {
                        cx: "13",
                        cy: "9.5",
                        r: "1",
                    }
                    circle {
                        r: "1",
                        cy: "9.5",
                        cx: "7",
                    }
                    path {
                        d: "M14.28,3.55C14.11,3.21,13.76,3,13.38,3H6.62C6.24,3,5.89,3.21,5.72,3.55L4,7v5.5C4,12.78,4.22,13,4.5,13h1 C5.78,13,6,12.78,6,12.5V12h8v0.5c0,0.28,0.22,0.5,0.5,0.5h1c0.28,0,0.5-0.22,0.5-0.5V7L14.28,3.55z M6.62,4h6.76l1.5,3H5.12 L6.62,4z M15,11H5V8h10V11z",
                    }
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18.92,2.01C18.72,1.42,18.16,1,17.5,1h-11C5.84,1,5.29,1.42,5.08,2.01L3,8v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1 h12v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1V8L18.92,2.01z M6.85,3h10.29l1.08,3.11H5.77L6.85,3z M19,13H5V8h14V13z",
                    }
                    circle {
                        cx: "7.5",
                        cy: "10.5",
                        r: "1.5",
                    }
                    circle {
                        cy: "10.5",
                        cx: "16.5",
                        r: "1.5",
                    }
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
                    x: "0",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M15,5.5C15,4.67,14.33,4,13.5,4H12v1h1.5C13.78,5,14,5.22,14,5.5v1.29L10.79,10H9V6H6C4.34,6,3,7.34,3,9v2h2 c0,1.1,0.9,2,2,2s2-0.9,2-2h2.21L15,7.21V5.5z M4,10V9c0-1.1,0.9-2,2-2h2v3H4z M7,12c-0.55,0-1-0.45-1-1h2C8,11.55,7.55,12,7,12z",
                    }
                    rect {
                        width: "4",
                        x: "5",
                        y: "4",
                        height: "1",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M19,5c0-1.1-0.9-2-2-2h-3v2h3v2.65L13.52,12H10V7H6c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,8.35V5z M4,12v-1c0-1.1,0.9-2,2-2h2v3H4z M7,15c-0.55,0-1-0.45-1-1h2C8,14.55,7.55,15,7,15z",
                    }
                    rect {
                        width: "5",
                        height: "2",
                        y: "4",
                        x: "5",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    polygon {
                        points: "9,17 6,17 11,19.5 11,17.5 14,17.5 9,15",
                    }
                    path {
                        d: "M17,10.05V8.53c0-0.34-0.12-0.67-0.33-0.94l-3.22-4.03C13.17,3.21,12.73,3,12.28,3H2.5C1.67,3,1,3.67,1,4.5V11 c0,0.83,0.67,1.5,1.5,1.5H3C3,13.88,4.12,15,5.5,15S8,13.88,8,12.5h6c0,1.38,1.12,2.5,2.5,2.5c1.5,0,2.2-1.12,2.39-1.73 C19.34,11.74,18.38,10.33,17,10.05z M2.5,4.5h3.75V8H2.5V4.5z M3.5,11h-1V9.5h3.75v0.61C6.01,10.04,5.76,10,5.5,10 C4.68,10,3.96,10.39,3.5,11z M5.5,13.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.05,13.5,5.5,13.5z M11.5,11H7.75V9.5H10V8 H7.75V4.5h3.75V11z M13,5.4L15.08,8H13V5.4z M14.5,11H13V9.5h2.5v0.71C15.1,10.38,14.76,10.66,14.5,11z M16.5,13.5 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S17.05,13.5,16.5,13.5z",
                    }
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M21,11.18V9.72c0-0.47-0.16-0.92-0.46-1.28L16.6,3.72C16.22,3.26,15.66,3,15.06,3H3C1.9,3,1,3.9,1,5v8c0,1.1,0.9,2,2,2 h0.18C3.6,16.16,4.7,17,6,17s2.4-0.84,2.82-2h8.37c0.41,1.16,1.51,2,2.82,2c1.66,0,3-1.34,3-3C23,12.7,22.16,11.6,21,11.18z M6,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.55,15,6,15z M7,11.17C6.69,11.06,6.35,11,6,11c-1.3,0-2.42,0.84-2.83,2H3v-3 h4V11.17z M7,8H3V5h4V8z M14,13H9v-3h3V8H9V5h5V13z M16,6.12L18.4,9H16V6.12z M17.17,13H16v-2h3v0.17 C18.15,11.47,17.47,12.15,17.17,13z M20,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S20.55,15,20,15z",
                        }
                    }
                    polygon {
                        points: "7,20 11,20 11,18 17,21 13,21 13,23",
                    }
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
                    x: "0",
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M15,10c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S16.1,10,15,10z M15,13c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S15.55,13,15,13z",
                    }
                    path {
                        d: "M12,11.99C12,10.34,13.34,9,14.99,9l0.24,0l-1.2-5.22C13.93,3.32,13.52,3,13.05,3H10v1h3.05l0.95,4.14 c-1.57,0.4-2.75,1.72-2.96,3.36H6.93c-0.26-1.01-1.29-1.72-2.44-1.44c-0.71,0.18-1.29,0.78-1.44,1.5C2.77,12.86,3.75,14,5,14 c0.93,0,1.71-0.64,1.93-1.5H12V11.99z M5,13c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S5.55,13,5,13z",
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
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M7.82,16H15v-1c0-2.21,1.79-4,4-4h0.74l-1.9-8.44C17.63,1.65,16.82,1,15.89,1H12v2h3.89l1.4,6.25c0,0-0.01,0-0.01,0 c-2.16,0.65-3.81,2.48-4.19,4.75H7.82c-0.48-1.34-1.86-2.24-3.42-1.94c-1.18,0.23-2.13,1.2-2.35,2.38C1.7,16.34,3.16,18,5,18 C6.3,18,7.4,17.16,7.82,16z M5,16c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S5.55,16,5,16z",
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
                d: "M11.25,17.75h-2.5c-0.55,0-1-0.45-1-1V13.9l-2.47,1.43c-0.48,0.28-1.09,0.11-1.37-0.37L2.66,12.8 c-0.28-0.48-0.11-1.09,0.37-1.37L5.5,10L3.03,8.57C2.55,8.3,2.39,7.69,2.66,7.21l1.25-2.16C4.19,4.56,4.8,4.4,5.28,4.68L7.75,6.1 V3.25c0-0.55,0.45-1,1-1h2.5c0.55,0,1,0.45,1,1V6.1l2.47-1.43c0.48-0.28,1.09-0.11,1.37,0.37l1.25,2.16 c0.28,0.48,0.11,1.09-0.37,1.37L14.5,10l2.47,1.43c0.48,0.28,0.64,0.89,0.37,1.37l-1.25,2.16c-0.28,0.48-0.89,0.64-1.37,0.37 l-2.47-1.43v2.85C12.25,17.3,11.8,17.75,11.25,17.75z M9.25,16.25h1.5v-4.08c0-0.38,0.42-0.63,0.75-0.43l3.54,2.04l0.75-1.3 l-3.54-2.04c-0.33-0.19-0.33-0.67,0-0.87l3.54-2.04l-0.75-1.3L11.5,8.27c-0.33,0.19-0.75-0.05-0.75-0.43V3.75h-1.5v4.08 c0,0.38-0.42,0.63-0.75,0.43L4.96,6.23l-0.75,1.3l3.54,2.04c0.33,0.19,0.33,0.67,0,0.87l-3.54,2.04l0.75,1.3l3.54-2.04 c0.33-0.19,0.75,0.05,0.75,0.43V16.25z",
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
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M21.29,13.9L18,12l3.29-1.9c0.48-0.28,0.64-0.89,0.37-1.37l-2-3.46c-0.28-0.48-0.89-0.64-1.37-0.37L15,6.8V3 c0-0.55-0.45-1-1-1h-4C9.45,2,9,2.45,9,3v3.8L5.71,4.9C5.23,4.63,4.62,4.79,4.34,5.27l-2,3.46C2.06,9.21,2.23,9.82,2.71,10.1L6,12 l-3.29,1.9c-0.48,0.28-0.64,0.89-0.37,1.37l2,3.46c0.28,0.48,0.89,0.64,1.37,0.37L9,17.2V21c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1 v-3.8l3.29,1.9c0.48,0.28,1.09,0.11,1.37-0.37l2-3.46C21.94,14.79,21.77,14.18,21.29,13.9z M18.43,16.87l-4.68-2.7 C13.42,13.97,13,14.21,13,14.6V20h-2v-5.4c0-0.38-0.42-0.63-0.75-0.43l-4.68,2.7l-1-1.73l4.68-2.7c0.33-0.19,0.33-0.67,0-0.87 l-4.68-2.7l1-1.73l4.68,2.7C10.58,10.03,11,9.79,11,9.4V4h2v5.4c0,0.38,0.42,0.63,0.75,0.43l4.68-2.7l1,1.73l-4.68,2.7 c-0.33,0.19-0.33,0.67,0,0.87l4.68,2.7L18.43,16.87z",
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
                    x: "0",
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18,6l-3,3V5.5C15,4.67,14.33,4,13.5,4h-9C3.67,4,3,4.67,3,5.5v9C3,15.33,3.67,16,4.5,16h9c0.83,0,1.5-0.67,1.5-1.5V11l3,3 V6z M13.5,14.5h-9v-9h9V14.5z M10.5,10l2.25,1.3L12,12.6l-2.25-1.3v2.45h-1.5V11.3L6,12.6l-0.75-1.3L7.5,10L5.25,8.7L6,7.4 l2.25,1.3V6.25h1.5V8.7L12,7.4l0.75,1.3L10.5,10z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M18,10.48V6c0-1.1-0.9-2-2-2H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-4.48l4,3.98v-11L18,10.48z M16,18 H4V6h12V18z M12,12l3,1.73l-1,1.73l-3-1.73V17H9v-3.27l-3,1.73l-1-1.73L8,12l-3-1.73l1-1.73l3,1.73V7h2v3.27l3-1.73l1,1.73L12,12z",
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
                    height: "20",
                    x: "0",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M15.28,12.28C15.28,15.81,10,20,10,20s-5.28-4.19-5.28-7.72C4.72,9.36,7.08,7,10,7S15.28,9.36,15.28,12.28z M16.34,2.66 l-1.06,1.06C13.92,2.35,12.03,1.5,9.95,1.5c-2.06,0-3.92,0.83-5.28,2.18L3.61,2.61C5.24,1,7.48,0,9.95,0 C12.45,0,14.71,1.02,16.34,2.66z M6.09,5.09l1.06,1.06C7.87,5.44,8.86,5,9.95,5c1.11,0,2.12,0.46,2.85,1.2l1.06-1.06 c-1-1.01-2.38-1.63-3.91-1.63C8.45,3.5,7.08,4.11,6.09,5.09z M6.22,12.28c0,1.71,2.05,4.15,3.78,5.74 c1.73-1.59,3.78-4.03,3.78-5.74c0-2.08-1.7-3.78-3.78-3.78S6.22,10.2,6.22,12.28z M10,11c-0.69,0-1.25,0.56-1.25,1.25 c0,0.69,0.56,1.25,1.25,1.25c0.69,0,1.25-0.56,1.25-1.25C11.25,11.56,10.69,11,10,11z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M12,4c1.93,0,3.68,0.78,4.95,2.05l-1.41,1.41C14.63,6.56,13.38,6,12,6S9.37,6.56,8.46,7.46L7.05,6.05 C8.32,4.78,10.07,4,12,4z M19.78,3.23l-1.41,1.41C16.74,3.01,14.49,2,12.01,2S7.27,3.01,5.64,4.63L4.22,3.22 C6.22,1.23,8.97,0,12.01,0S17.79,1.23,19.78,3.23z M12,11c1.94,0,4,1.45,4,4.15c0,0.94-0.55,2.93-4,6.17c-3.45-3.24-4-5.23-4-6.17 C8,12.45,10.06,11,12,11z M12,9c-3.15,0-6,2.41-6,6.15c0,2.49,2,5.44,6,8.85c4-3.41,6-6.36,6-8.85C18,11.41,15.15,9,12,9z M13.5,15 c0-0.83-0.67-1.5-1.5-1.5c-0.83,0-1.5,0.67-1.5,1.5c0,0.83,0.67,1.5,1.5,1.5C12.83,16.5,13.5,15.83,13.5,15z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M19.77 7.23l.01-.01-3.72-3.72L15 4.56l2.11 2.11c-.94.36-1.61 1.26-1.61 2.33 0 1.38 1.12 2.5 2.5 2.5.36 0 .69-.08 1-.21v7.21c0 .55-.45 1-1 1s-1-.45-1-1V14c0-1.1-.9-2-2-2h-1V5c0-1.1-.9-2-2-2H6c-1.1 0-2 .9-2 2v16h10v-7.5h1.5v5c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5V9c0-.69-.28-1.32-.73-1.77zM12 11v8H6V5h6v6zm6-1c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm-8-4l-4 7.5h2V18l4-7h-2z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                    x: "0",
                }
            }
            g {
                path {
                    d: "M16.5,10v6.5h-13V9.47L6,8.33v1.71l4-1.75V10H16.5z M15,2l-1,6.5h-2.5V6l-4,1.75V6L2,8.5V18h16V8.5L17,2H15z M14,11.5h-1.5 V15H14V11.5z M7.5,11.5H6V15h1.5V11.5z M10.75,11.5h-1.5V15h1.5V11.5z",
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
            }
            g {
                path {
                    d: "M22,22H2V10l7-3v2l5-2l0,3h3l1-8h3l1,8V22z M12,9.95l-5,2V10l-3,1.32V20h16v-8h-8L12,9.95z M11,18h2v-4h-2V18z M7,18h2v-4 H7V18z M17,14h-2v4h2V14z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M1 21.98c0 .56.45 1.01 1.01 1.01H15c.56 0 1.01-.45 1.01-1.01V21H1v.98zM8.5 8.99C4.75 8.99 1 11 1 15h15c0-4-3.75-6.01-7.5-6.01zM3.62 13c1.11-1.55 3.47-2.01 4.88-2.01s3.77.46 4.88 2.01H3.62zM1 17h15v2H1zM18 5V1h-2v4h-5l.23 2h9.56l-1.4 14H18v2h1.72c.84 0 1.53-.65 1.63-1.47L23 5h-5z",
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
                    y: "0",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M10,2c0,0-5,5-9,5.5V9c0,1.23,0.79,2.26,1.89,2.64C2.77,14.11,2.53,16.95,2,18h16c-0.06-0.12-0.68-1.27-0.91-6.35 C18.2,11.27,19,10.24,19,9V7.5C15,7,10,2,10,2z M9.25,9c0,0.72-0.59,1.31-1.31,1.31S6.63,9.72,6.63,9H9.25z M8.23,11.78 c0.7-0.08,1.32-0.39,1.77-0.88c0.45,0.49,1.07,0.81,1.77,0.88c0.03,0.95,0.13,2.67,0.4,4.72H7.83C8.11,14.45,8.2,12.73,8.23,11.78z M12.06,10.31c-0.72,0-1.31-0.59-1.31-1.31h2.62C13.37,9.72,12.79,10.31,12.06,10.31z M10,4.05c1.07,0.95,2.79,2.37,4.72,3.45H5.28 C7.21,6.42,8.93,5.01,10,4.05z M2.5,9h2.62c0,0.72-0.59,1.31-1.31,1.31S2.5,9.72,2.5,9z M4.39,11.75c0.58-0.12,1.09-0.43,1.48-0.85 c0.24,0.27,0.54,0.48,0.87,0.64c-0.03,0.88-0.11,2.76-0.43,4.97H3.95C4.18,15.17,4.31,13.37,4.39,11.75z M13.69,16.5 c-0.31-2.21-0.4-4.09-0.43-4.97c0.33-0.16,0.62-0.37,0.87-0.64c0.39,0.42,0.9,0.73,1.48,0.85c0.08,1.62,0.21,3.42,0.44,4.75H13.69z M16.19,10.31c-0.72,0-1.31-0.59-1.31-1.31h2.62C17.5,9.72,16.91,10.31,16.19,10.31z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M23,11V9c-6-2-11-7-11-7S7,7,1,9v2c0,1.49,0.93,2.75,2.24,3.26C3.2,16.76,2.92,19.69,2,22h20 c-0.92-2.31-1.2-5.24-1.24-7.74C22.07,13.75,23,12.49,23,11z M12,4.71c1.33,1.14,3.49,2.84,6.11,4.29H5.89 C8.51,7.55,10.67,5.85,12,4.71z M13,11h3c0,0.83-0.67,1.5-1.5,1.5S13,11.83,13,11z M9.5,12.5C8.67,12.5,8,11.83,8,11h3 C11,11.83,10.33,12.5,9.5,12.5z M6,11c0,0.83-0.67,1.5-1.5,1.5S3,11.83,3,11H6z M4.66,20c0.39-1.86,0.54-3.82,0.57-5.58 c0.68-0.15,1.29-0.49,1.76-0.98c0.25,0.25,0.54,0.45,0.85,0.62c-0.1,1.87-0.26,4-0.52,5.93H4.66z M9.35,20 c0.24-1.83,0.39-3.78,0.48-5.53c0.84-0.08,1.61-0.45,2.17-1.02c0.56,0.57,1.32,0.94,2.17,1.02c0.1,1.75,0.24,3.7,0.48,5.53H9.35z M16.67,20c-0.27-1.94-0.43-4.07-0.52-5.93c0.31-0.17,0.61-0.37,0.85-0.62c0.47,0.48,1.08,0.83,1.76,0.98 c0.03,1.76,0.18,3.72,0.57,5.58H16.67z M19.5,12.5c-0.83,0-1.5-0.67-1.5-1.5h3C21,11.83,20.33,12.5,19.5,12.5z",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M10,8.75c-1.52,0-2.75,1.23-2.75,2.75s1.23,2.75,2.75,2.75s2.75-1.23,2.75-2.75S11.52,8.75,10,8.75z M10,12.75 c-0.69,0-1.25-0.56-1.25-1.25s0.56-1.25,1.25-1.25s1.25,0.56,1.25,1.25S10.69,12.75,10,12.75z",
                    }
                    path {
                        d: "M15.5,9H15V7h1V5.5h-1.25C14.11,3.48,12.24,2,10,2S5.89,3.48,5.25,5.5H4V7h1v2H4.5C3.67,9,3,9.67,3,10.5v2 C3,13.33,3.67,14,4.5,14H5v2.5H4V18h12v-1.5h-1V14h0.5c0.83,0,1.5-0.67,1.5-1.5v-2C17,9.67,16.33,9,15.5,9z M10,3.5 c1.39,0,2.59,0.82,3.15,2h-6.3C7.41,4.32,8.61,3.5,10,3.5z M15.5,12.5h-2v4h-7v-4h-2v-2h2V7h7v3.5h2V12.5z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,10.5c-1.93,0-3.5,1.57-3.5,3.5s1.57,3.5,3.5,3.5s3.5-1.57,3.5-3.5S13.93,10.5,12,10.5z M12,15.5 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S12.83,15.5,12,15.5z",
                    }
                    path {
                        d: "M19,11h-1V8h2V6h-2.35C16.83,3.67,14.61,2,12,2S7.17,3.67,6.35,6H4v2h2v3H5c-1.1,0-2,0.9-2,2v2c0,1.1,0.9,2,2,2h1v3H4v2 h16v-2h-2v-3h1c1.1,0,2-0.9,2-2v-2C21,11.9,20.1,11,19,11z M12,4c1.47,0,2.75,0.81,3.44,2H8.56C9.25,4.81,10.53,4,12,4z M19,15h-3 v5H8v-5H5v-2h3V8h8v5h3V15z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M19,9.35c0-0.23-0.05-0.46-0.16-0.67l-1.43-2.85C17.16,5.32,16.64,5,16.07,5H15.5V3.5C15.5,3.22,15.28,3,15,3h-1.5 C13.22,3,13,3.22,13,3.5V5h-1.5C10.67,5,10,5.67,10,6.5v3H1v4C1,14.33,1.67,15,2.5,15h0.55c0.23,1.14,1.24,2,2.45,2 c1.21,0,2.22-0.86,2.45-2h4.1c0.23,1.14,1.24,2,2.45,2c1.21,0,2.22-0.86,2.45-2H19V9.35z M11.5,6.5h4.57h0l1.43,2.85V9.5h-6V6.5z M6.5,14.5c0,0.55-0.45,1-1,1s-1-0.45-1-1s0.45-1,1-1S6.5,13.95,6.5,14.5z M10,13.5H7.79C7.4,12.62,6.52,12,5.5,12 c-1.02,0-1.9,0.62-2.29,1.5H2.5V11H10V13.5z M15.5,14.5c0,0.55-0.45,1-1,1s-1-0.45-1-1s0.45-1,1-1S15.5,13.95,15.5,14.5z M16.79,13.5C16.4,12.62,15.52,12,14.5,12c-1.02,0-1.9,0.62-2.29,1.5H11.5V11h6v2.5H16.79z",
                    }
                    path {
                        d: "M9,7.5H8V6h1V5H1v1h1v1.5H1v1h8V7.5z M7,7.5H5.5V6H7V7.5z M3,6h1.5v1.5H3V6z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M22.9,10.69l-1.44-4.32C21.18,5.55,20.42,5,19.56,5H19V4c0-0.55-0.45-1-1-1h-1c-0.55,0-1,0.45-1,1v1h-2c-1.1,0-2,0.9-2,2 v4H1v5c0,1.1,0.9,2,2,2h1c0,1.66,1.34,3,3,3s3-1.34,3-3h4c0,1.66,1.34,3,3,3s3-1.34,3-3h3v-6.68C23,11.11,22.97,10.9,22.9,10.69z M14,7h5.56l1.33,4H14V7z M7,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S7.55,19,7,19z M12,16H9.22C8.67,15.39,7.89,15,7,15 s-1.67,0.39-2.22,1H3v-3h9V16z M17,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S17.55,19,17,19z M19.22,16 c-0.55-0.61-1.34-1-2.22-1s-1.67,0.39-2.22,1H14v-3h7v3H19.22z",
                    }
                    path {
                        d: "M11,8.5h-1v-2h1V5H1v1.5h1v2H1V10h10V8.5z M8.5,8.5H6.75v-2H8.5V8.5z M3.5,6.5h1.75v2H3.5V6.5z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M21 16v-2l-8-5V3.5c0-.83-.67-1.5-1.5-1.5S10 2.67 10 3.5V9l-8 5v2l8-2.5V19l-2 1.5V22l3.5-1 3.5 1v-1.5L13 19v-5.5l8 2.5z",
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
                width: "20",
                fill: "none",
                height: "20",
            }
            path {
                d: "M13,4h-1.5C10.67,4,10,4.67,10,5.5v3.75c0,0.83,0.67,1.5,1.5,1.5H13c0.83,0,1.5-0.67,1.5-1.5V5.5C14.5,4.67,13.83,4,13,4z M13,9.25h-1.5V5.5H13V9.25z M8.4,13H15v1.5H8.4c-0.66,0-1.24-0.43-1.43-1.06L5,7V4h1.5v3L8.4,13z M7.5,15.5H15V17H7.5V15.5z",
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
                height: "24",
                fill: "none",
                y: "0",
                width: "24",
            }
            path {
                d: "M16,4h-2c-1.1,0-2,0.9-2,2v5c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2V6C18,4.9,17.1,4,16,4z M16,11h-2V6h2V11z M9.5,16H18v2H9.49 c-0.88,0-1.66-0.58-1.92-1.43L5,8V4h2v4L9.5,16z M8,19h10v2H8V19z",
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
                    x: "0",
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M15.82,10H17l-5-8l-2,3.2L8,2l-5,8h1.18L1,15h5.5v3h3v-3h1v3h3v-3H19L15.82,10z M3.73,13.5l3.18-5H5.71L8,4.83l2.29,3.67 H9.09l3.18,5H3.73z M14.05,13.5L11.82,10H13l-2.12-3.38L12,4.83l2.29,3.67h-1.21l3.18,5H14.05z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M24,18l-3.86-6H22L15,2l-3,4.29L9,2L2,12h1.86L0,18h7v4h4v-4h2v4h4v-4H24z M15,5.49L18.16,10h-1.68l3.86,6h-3.62l-2.57-4 H16l-2.78-3.97L15,5.49z M3.66,16l3.86-6H5.84L9,5.49L12.16,10h-1.68l3.86,6H3.66z",
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
                    width: "20",
                    x: "0",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M11,11.36c-1.2-1.05-2.74-1.4-4.13-1.11l1.19-1.19L7,8l-3,3l3,3l1.06-1.06l-1.19-1.19C8.74,11.32,10.44,12.28,11,14v3h1.5 l0-11.13l1.19,1.19L14.75,6l-3-3l-3,3l1.06,1.06L11,5.87L11,11.36z",
                    }
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
            }
            g {
                path {
                    d: "M9.41,15.59L8,17l-4-4l4-4l1.41,1.41L7.83,12c1.51-0.33,3.73,0.08,5.17,1.36l0-6.53l-1.59,1.59L10,7l4-4l4,4l-1.41,1.41 L15,6.83V21l-2,0v-4c-0.73-2.58-3.07-3.47-5.17-3L9.41,15.59z",
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
                    x: "0",
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M9,11.36c1.2-1.05,2.74-1.4,4.13-1.11l-1.19-1.19L13,8l3,3l-3,3l-1.06-1.06l1.19-1.19C11.26,11.32,9.56,12.28,9,14v3H7.5 l0-11.13L6.31,7.06L5.25,6l3-3l3,3l-1.06,1.06L9,5.87L9,11.36z",
                    }
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
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M14.59,15.59L16,17l4-4l-4-4l-1.41,1.41L16.17,12c-1.51-0.33-3.73,0.08-5.17,1.36l0-6.53l1.59,1.59L14,7l-4-4L6,7 l1.41,1.41L9,6.83V21l2,0v-4c0.73-2.58,3.07-3.47,5.17-3L14.59,15.59z",
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
                    x: "0",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M17.5,3v1.5h-1.75V3h-1.5v1.5H12.5V3H11v3l2,2v1H7V8l2-2V3H7.5v1.5H5.75V3h-1.5v1.5H2.5V3H1v3l2,2v4l-2,2v3h7v-2 c0-1.1,0.9-2,2-2s2,0.9,2,2v2h7v-3l-2-2V8l2-2V3H17.5z M17.5,14.62v0.88h-4V15c0-1.93-1.57-3.5-3.5-3.5S6.5,13.07,6.5,15v0.5h-4 v-0.88l2-2V7.38L3.13,6h3.75L5.5,7.38v3.12h9V7.38L13.13,6h3.75L15.5,7.38v5.24L17.5,14.62z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M21,3v2h-2V3h-2v2h-2V3h-2v4l2,2v1H9V9l2-2V3H9v2H7V3H5v2H3V3H1v4l2,2v6l-2,2v4h9v-3c0-1.1,0.9-2,2-2s2,0.9,2,2v3h9v-4 l-2-2V9l2-2V3H21z M21,19h-5v-1c0-2.21-1.79-4-4-4s-4,1.79-4,4v1H3v-1.17l2-2V8.17L3.83,7h4.34L7,8.17V12h10V8.17L15.83,7h4.34 L19,8.17v7.66l2,2V19z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                    y: "0",
                }
            }
            g {
                path {
                    d: "M12,6c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2S13.1,6,12,6z M17,2h2c0,2.7-0.93,4.41-2.3,5.5c-0.5,0.4-1.1,0.7-1.7,0.9V22h-2 v-6h-2v6H9V10.1c-0.3,0.1-0.5,0.2-0.6,0.3C7.87,10.81,7,11.43,7,14H5c0-2.06,0.35-3.78,2.11-5.29C8.21,7.81,10,7,12,7 s2.68-0.46,3.48-1.06C15.96,5.55,17,4.76,17,2z M4,16h3v6H4V16z",
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
                    height: "20",
                    x: "0",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M16.71,14.59l-3.54-3.54h-1.41l-0.34-0.34l-0.71,0.71l0.34,0.34v1.41l3.54,3.54c0.39,0.39,1.02,0.39,1.41,0L16.71,16 C17.1,15.61,17.1,14.98,16.71,14.59z M15.29,16l-3.18-3.18l0.71-0.71L16,15.29L15.29,16z",
                    }
                    path {
                        d: "M12.57,8.14L12.57,8.14l0.88,0.88l1.06-1.06l1.41,1.41c0.78-0.78,0.78-2.05,0-2.83l-2.47-2.47l-0.74,0.74l0-1.49 l-0.49-0.49L9.74,5.31l0.49,0.49l1.49,0l-0.74,0.74l0.88,0.88L10,9.29L7.51,6.81l0.15-1.26L5.36,3.23L3.23,5.36l2.31,2.31 l1.26-0.15L9.29,10l-1.05,1.05H6.83l-3.54,3.54c-0.39,0.39-0.39,1.02,0,1.41L4,16.71c0.39,0.39,1.02,0.39,1.41,0l3.54-3.54v-1.41 L12.57,8.14z M4.71,16L4,15.29l3.18-3.18l0.71,0.71L4.71,16z",
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
            }
            g {
                g {
                    g {
                        path {
                            d: "M21.67,18.17l-5.3-5.3h-0.99l-2.54,2.54v0.99l5.3,5.3c0.39,0.39,1.02,0.39,1.41,0l2.12-2.12 C22.06,19.2,22.06,18.56,21.67,18.17z M18.84,19.59l-4.24-4.24l0.71-0.71l4.24,4.24L18.84,19.59z",
                        }
                    }
                    g {
                        path {
                            d: "M17.34,10.19l1.41-1.41l2.12,2.12c1.17-1.17,1.17-3.07,0-4.24l-3.54-3.54l-1.41,1.41V1.71L15.22,1l-3.54,3.54l0.71,0.71 h2.83l-1.41,1.41l1.06,1.06l-2.89,2.89L7.85,6.48V5.06L4.83,2.04L2,4.87l3.03,3.03h1.41l4.13,4.13l-0.85,0.85H7.6l-5.3,5.3 c-0.39,0.39-0.39,1.02,0,1.41l2.12,2.12c0.39,0.39,1.02,0.39,1.41,0l5.3-5.3v-2.12l5.15-5.15L17.34,10.19z M9.36,15.34 l-4.24,4.24l-0.71-0.71l4.24-4.24l0,0L9.36,15.34L9.36,15.34z",
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
                    fill: "none",
                    y: "0",
                    width: "20",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M14.5,3L12,5.5V3H8C5.79,3,4,4.79,4,7h3.5v9c0,0.55,0.45,1,1,1H11c0.55,0,1-0.45,1-1V6.5L14.5,9H16V3H14.5z M10.5,15.5H9 v-4.75h1.5V15.5z M10.5,9.25H9V5.5H6c0.46-0.61,1.18-1,2-1h2.5V9.25z",
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
                    fill: "none",
                    y: "0",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18,3l-3,3V3H9C6.24,3,4,5.24,4,8h5v12c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1V8l3,3h2V3H18z M13,19h-2v-6h2V19z M11,11V6 H6.77C7.32,5.39,8.11,5,9,5h4v6H11z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                    x: "0",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M16,7h-3V5c0-0.55-0.45-1-1-1H8C7.45,4,7,4.45,7,5v2H4C3.45,7,3,7.45,3,8v8h14V8C17,7.45,16.55,7,16,7z M8,5h4v2H8V5z M16,15H4v-3h2v1h1v-1h6v1h1v-1h2V15z M14,11v-1h-1v1H7v-1H6v1H4V8h3h6h3v3H14z",
                        }
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M20,8h-3V6c0-1.1-0.9-2-2-2H9C7.9,4,7,4.9,7,6v2H4c-1.1,0-2,0.9-2,2v10h20V10C22,8.9,21.1,8,20,8z M9,6h6v2H9V6z M20,18H4 v-3h2v1h2v-1h8v1h2v-1h2V18z M18,13v-1h-2v1H8v-1H6v1H4v-3h3h10h3v3H18z",
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
                d: "M7 14c1.66 0 3-1.34 3-3S8.66 8 7 8s-3 1.34-3 3 1.34 3 3 3zm0-4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm12-3h-8v8H3V5H1v15h2v-3h18v3h2v-9c0-2.21-1.79-4-4-4zm2 8h-8V9h6c1.1 0 2 .9 2 2v4z",
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
                    width: "20",
                    height: "20",
                    x: "0",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M15,4H5C4.45,4,4,4.45,4,5v10c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V5C16,4.45,15.55,4,15,4z M15,15H5V5h10V15z",
                    }
                    path {
                        d: "M10,14c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4s-4,1.79-4,4C6,12.21,7.79,14,10,14z M12.97,9.74H7.03 c0.04-0.46,0.18-0.88,0.4-1.26h5.15C12.8,8.86,12.93,9.29,12.97,9.74z M12.98,10.24c-0.04,0.46-0.17,0.88-0.39,1.26H7.42 c-0.22-0.38-0.36-0.8-0.39-1.26H12.98z M10,13c-0.88,0-1.67-0.39-2.22-1h4.44C11.67,12.61,10.88,13,10,13z M10,7 c0.88,0,1.66,0.39,2.21,0.99H7.79C8.34,7.39,9.12,7,10,7z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z",
                    }
                    path {
                        d: "M12,18c3.31,0,6-2.69,6-6s-2.69-6-6-6s-6,2.69-6,6S8.69,18,12,18z M15.44,10c0.26,0.45,0.44,0.96,0.51,1.5h-7.9 c0.07-0.54,0.24-1.05,0.51-1.5H15.44z M15.95,12.5c-0.07,0.54-0.24,1.05-0.51,1.5H8.56c-0.26-0.45-0.44-0.96-0.51-1.5H15.95z M9.38,15h5.24c-0.7,0.61-1.61,1-2.62,1S10.09,15.61,9.38,15z M14.62,9H9.38c0.7-0.61,1.61-1,2.62-1S13.91,8.39,14.62,9z",
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
                    fill: "none",
                    y: "0",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M14.92,5.16C14.51,2.8,12.47,1,10,1S5.49,2.8,5.08,5.16C3.88,5.55,3,6.66,3,8c0,1.66,1.34,3,3,3c0.02,0,0.03,0,0.04,0 L10,19l3.96-8c0.01,0,0.03,0,0.04,0c1.66,0,3-1.34,3-3C17,6.66,16.12,5.55,14.92,5.16z M10,15.61l-2.49-5.04 c0.16-0.1,0.07-0.08,0.37-0.08C8.53,10.81,9.24,11,10,11s1.47-0.19,2.12-0.5c0.3,0,0.21-0.02,0.37,0.08L10,15.61z M14,9.5 c-0.39,0-0.69-0.15-0.87-0.28l-0.74-0.53C11.45,9.17,10.92,9.5,10,9.5c-0.92,0-1.49-0.35-2.39-0.81L6.87,9.22 C6.69,9.35,6.39,9.5,6,9.5C5.17,9.5,4.5,8.83,4.5,8c0-0.64,0.42-1.21,1.05-1.42l0.87-0.28l0.15-0.9C6.85,3.72,8.3,2.5,10,2.5 s3.15,1.22,3.44,2.91l0.15,0.9l0.87,0.28C15.08,6.79,15.5,7.36,15.5,8C15.5,8.83,14.83,9.5,14,9.5z",
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
                    width: "24",
                    y: "0",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18.38,6.24C17.79,3.24,15.14,1,12,1S6.21,3.24,5.62,6.24C4.08,6.81,3,8.29,3,10c0,2.21,1.79,4,4,4 c0.12,0,0.23-0.02,0.34-0.02L12.07,23l4.61-9.03C16.79,13.98,16.89,14,17,14c2.21,0,4-1.79,4-4C21,8.29,19.92,6.81,18.38,6.24z M12.05,18.63l-2.73-5.21C10.15,13.79,11.06,14,12,14c0.95,0,1.88-0.21,2.72-0.6L12.05,18.63z M17,12c-0.52,0-1.01-0.2-1.39-0.56 l-0.56-0.54l-0.66,0.42C13.68,11.76,12.86,12,12,12s-1.68-0.24-2.39-0.69L8.95,10.9l-0.56,0.54C8.01,11.79,7.52,12,7,12 c-1.1,0-2-0.89-2-2c0-0.98,0.72-1.82,1.68-1.97L7.45,7.9l0.06-0.78C7.71,4.8,9.66,3,12,3s4.29,1.8,4.48,4.12l0.06,0.78l0.77,0.12 C18.28,8.18,19,9.01,19,10C19,11.1,18.1,12,17,12z",
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
                width: "20",
                height: "20",
                fill: "none",
            }
            path {
                d: "M6.5,6H7c1.1,0,2-0.9,2-2S8.1,2,7,2H6.5V1h-1v1H5C3.9,2,3,2.9,3,4s0.9,2,2,2h0.5v1H3v4h2.5v1H5c-1.1,0-2,0.9-2,2s0.9,2,2,2 h0.5v3h1v-3H7c1.1,0,2-0.9,2-2s-0.9-2-2-2H6.5v-1H9V7H6.5V6z M5,4.5C4.72,4.5,4.5,4.28,4.5,4S4.72,3.5,5,3.5h2 c0.28,0,0.5,0.22,0.5,0.5S7.28,4.5,7,4.5H5z M7,13.5c0.28,0,0.5,0.22,0.5,0.5S7.28,14.5,7,14.5H5c-0.28,0-0.5-0.22-0.5-0.5 s0.22-0.5,0.5-0.5H7z M7.5,8.5v1h-3v-1H7.5z M14.5,6H15c1.1,0,2-0.9,2-2s-0.9-2-2-2h-0.5V1h-1v1H13c-1.1,0-2,0.9-2,2s0.9,2,2,2h0.5 v1H11v4h2.5v1H13c-1.1,0-2,0.9-2,2s0.9,2,2,2h0.5v3h1v-3H15c1.1,0,2-0.9,2-2s-0.9-2-2-2h-0.5v-1H17V7h-2.5V6z M13,4.5 c-0.28,0-0.5-0.22-0.5-0.5s0.22-0.5,0.5-0.5h2c0.28,0,0.5,0.22,0.5,0.5S15.28,4.5,15,4.5H13z M15,13.5c0.28,0,0.5,0.22,0.5,0.5 s-0.22,0.5-0.5,0.5h-2c-0.28,0-0.5-0.22-0.5-0.5s0.22-0.5,0.5-0.5H15z M15.5,8.5v1h-3v-1H15.5z",
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
                d: "M17.75,7h0.75C19.88,7,21,5.88,21,4.5C21,3.12,19.88,2,18.5,2h-0.75V1h-1.5v1H15.5C14.12,2,13,3.12,13,4.5 C13,5.88,14.12,7,15.5,7h0.75v1H13v5h3.25v1H15.5c-1.38,0-2.5,1.12-2.5,2.5c0,1.38,1.12,2.5,2.5,2.5h0.75v4h1.5v-4h0.75 c1.38,0,2.5-1.12,2.5-2.5c0-1.38-1.12-2.5-2.5-2.5h-0.75v-1H21V8h-3.25V7z M15.5,5C15.22,5,15,4.78,15,4.5S15.22,4,15.5,4h3 C18.78,4,19,4.22,19,4.5S18.78,5,18.5,5H15.5z M18.5,16c0.28,0,0.5,0.22,0.5,0.5S18.78,17,18.5,17h-3c-0.28,0-0.5-0.22-0.5-0.5 s0.22-0.5,0.5-0.5H18.5z M19,10v1h-4v-1H19z M7.75,7H8.5C9.88,7,11,5.88,11,4.5C11,3.12,9.88,2,8.5,2H7.75V1h-1.5v1H5.5 C4.12,2,3,3.12,3,4.5C3,5.88,4.12,7,5.5,7h0.75v1H3v5h3.25v1H5.5C4.12,14,3,15.12,3,16.5C3,17.88,4.12,19,5.5,19h0.75v4h1.5v-4H8.5 c1.38,0,2.5-1.12,2.5-2.5c0-1.38-1.12-2.5-2.5-2.5H7.75v-1H11V8H7.75V7z M5.5,5C5.22,5,5,4.78,5,4.5S5.22,4,5.5,4h3 C8.78,4,9,4.22,9,4.5S8.78,5,8.5,5H5.5z M8.5,16C8.78,16,9,16.22,9,16.5S8.78,17,8.5,17h-3C5.22,17,5,16.78,5,16.5S5.22,16,5.5,16 H8.5z M9,10v1H5v-1H9z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M11.99 18.54l-7.37-5.73L3 14.07l9 7 9-7-1.63-1.27zM12 16l7.36-5.73L21 9l-9-7-9 7 1.63 1.27L12 16zm0-11.47L17.74 9 12 13.47 6.26 9 12 4.53z",
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
                d: "M12 4.53L17.74 9l-1.89 1.47 1.43 1.42L21 9l-9-7-2.59 2.02 1.42 1.42zm9 9.54l-1.63-1.27-.67.52 1.43 1.43zM3.41.86L2 2.27l4.22 4.22L3 9l9 7 2.1-1.63 1.42 1.42-3.53 2.75-7.37-5.73L3 14.07l9 7 4.95-3.85L20.73 21l1.41-1.41L3.41.86zM12 13.47L6.26 9l1.39-1.08 5.02 5.02-.67.53z",
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
                    height: "20",
                    y: "0",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M6.5,10.5v1c0,0.43-0.28,0.81-0.7,0.94l-0.3,0.09l-0.3-0.09c-0.42-0.13-0.7-0.51-0.7-0.94v-1H6.5 M8,5H3v6.5 c0,1.12,0.74,2.05,1.75,2.37v2.63H3V18h5v-1.5H6.25v-2.63C7.26,13.55,8,12.62,8,11.5V5L8,5z M4.5,9V6.5h2V9H4.5L4.5,9z",
                    }
                    path {
                        d: "M16.5,11.5v2h-6l0-2H16.5 M14.5,2h-2c-0.55,0-1,0.45-1,1v2.98c0,0.61-0.37,1.17-0.94,1.39L9.94,7.62 C9.37,7.85,9,8.4,9,9.02v7.48c0,0.83,0.67,1.5,1.5,1.5h6c0.83,0,1.5-0.67,1.5-1.5V9.02c0-0.61-0.37-1.17-0.94-1.39l-0.61-0.25 C15.87,7.15,15.5,6.6,15.5,5.98V3C15.5,2.45,15.05,2,14.5,2L14.5,2z M13,4.5v-1h1v1H13L13,4.5z M10.5,10V9.02l0.61-0.25 C12.25,8.31,12.99,7.23,13,6h1c0.01,1.23,0.74,2.31,1.88,2.77l0.61,0.25V10H10.5L10.5,10z M10.5,16.5V15h6v1.5H10.5L10.5,16.5z",
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
                    height: "24",
                    y: "0",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M3,14c0,1.3,0.84,2.4,2,2.82V20H3v2h6v-2H7v-3.18C8.16,16.4,9,15.3,9,14V6H3V14z M5,8h2v3H5V8z M5,13h2v1 c0,0.55-0.45,1-1,1s-1-0.45-1-1V13z",
                    }
                    path {
                        d: "M20.64,8.54l-0.96-0.32C19.27,8.08,19,7.7,19,7.27V3c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v4.28 c0,0.43-0.27,0.81-0.68,0.95l-0.96,0.32C11.55,8.83,11,9.59,11,10.45V20c0,1.1,0.9,2,2,2h7c1.1,0,2-0.9,2-2v-9.56 C22,9.58,21.45,8.82,20.64,8.54z M16,4h1v1h-1V4z M20,20h-7v-2h7V20z M20,16h-7v-2h7V16z M20,12h-7v-1.56l0.95-0.32 C15.18,9.72,16,8.57,16,7.28V7h1v0.28c0,1.29,0.82,2.44,2.05,2.85L20,10.44V12z",
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
                d: "M22 10V6c0-1.1-.9-2-2-2H4c-1.1 0-1.99.9-1.99 2v4c1.1 0 1.99.9 1.99 2s-.89 2-2 2v4c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-4c-1.1 0-2-.9-2-2s.9-2 2-2zm-2-1.46c-1.19.69-2 1.99-2 3.46s.81 2.77 2 3.46V18H4v-2.54c1.19-.69 2-1.99 2-3.46 0-1.48-.8-2.77-1.99-3.46L4 6h16v2.54zM9.07 16L12 14.12 14.93 16l-.89-3.36 2.69-2.2-3.47-.21L12 7l-1.27 3.22-3.47.21 2.69 2.2z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                    x: "0",
                }
                path {
                    d: "M17,13v-1l-6-4V4c0-0.55-0.45-1-1-1S9,3.45,9,4v4l-6,4v1l6-2v4l-1.5,1v1l2.5-0.5l2.5,0.5v-1L11,15v-4L17,13z",
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
                    d: "M22,16v-2l-8.5-5V3.5C13.5,2.67,12.83,2,12,2s-1.5,0.67-1.5,1.5V9L2,14v2l8.5-2.5V19L8,20.5L8,22l4-1l4,1l0-1.5L13.5,19 v-5.5L22,16z",
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
                d: "M11 17h2v-1h1c.55 0 1-.45 1-1v-3c0-.55-.45-1-1-1h-3v-1h4V8h-2V7h-2v1h-1c-.55 0-1 .45-1 1v3c0 .55.45 1 1 1h3v1H9v2h2v1zm9-13H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zm0 14H4V6h16v12z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M14.77 9L12 12.11 9.23 9h5.54M21 3H3v2l8 9v5H6v2h12v-2h-5v-5l8-9V3zM7.43 7L5.66 5h12.69l-1.78 2H7.43z",
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
                d: "M16 5v8c0 1.1-.9 2-2 2H8c-1.1 0-2-.9-2-2V5h10m4-2H4v10c0 2.21 1.79 4 4 4h6c2.21 0 4-1.79 4-4v-3h2c1.11 0 2-.89 2-2V5c0-1.11-.89-2-2-2zm-2 5V5h2v3h-2zm2 11H2v2h18v-2z",
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
                d: "M17 5c.83 0 1.5-.67 1.5-1.5 0-1-1.5-2.7-1.5-2.7s-1.5 1.7-1.5 2.7c0 .83.67 1.5 1.5 1.5zm-5 0c.83 0 1.5-.67 1.5-1.5 0-1-1.5-2.7-1.5-2.7s-1.5 1.7-1.5 2.7c0 .83.67 1.5 1.5 1.5zM7 5c.83 0 1.5-.67 1.5-1.5C8.5 2.5 7 .8 7 .8S5.5 2.5 5.5 3.5C5.5 4.33 6.17 5 7 5zm11.92 3.01C18.72 7.42 18.16 7 17.5 7h-11c-.66 0-1.21.42-1.42 1.01L3 14v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.85 9h10.29l1.04 3H5.81l1.04-3zM19 19H5v-4.66l.12-.34h13.77l.11.34V19z",
            }
            circle {
                cy: "16.5",
                r: "1.5",
                cx: "7.5",
            }
            circle {
                r: "1.5",
                cy: "16.5",
                cx: "16.5",
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
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M19 7V4H5v3H2v13h8v-4h4v4h8V7h-3zm1 11h-4v-4H8v4H4V9h3V6h10v3h3v9zM8 8h2v1H8v3h3v-1H9v-1h2V7H8zm7 1h-1V7h-1v3h2v2h1V7h-1z",
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
                d: "M8.1 13.34l2.83-2.83L3.91 3.5c-1.56 1.56-1.56 4.09 0 5.66l4.19 4.18zm6.78-1.81c1.53.71 3.68.21 5.27-1.38 1.91-1.91 2.28-4.65.81-6.12-1.46-1.46-4.2-1.1-6.12.81-1.59 1.59-2.09 3.74-1.38 5.27L3.7 19.87l1.41 1.41L12 14.41l6.88 6.88 1.41-1.41L13.41 13l1.47-1.47z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M3 2l2.01 18.23C5.13 21.23 5.97 22 7 22h10c1.03 0 1.87-.77 1.99-1.77L21 2H3zm14 18l-10 .01L5.89 10H18.1L17 20zm1.33-12H5.67l-.44-4h13.53l-.43 4zM12 19c1.66 0 3-1.34 3-3 0-2-3-5.4-3-5.4S9 14 9 16c0 1.66 1.34 3 3 3zm0-5.09c.59.91 1 1.73 1 2.09 0 .55-.45 1-1 1s-1-.45-1-1c0-.37.41-1.19 1-2.09z",
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
                    fill: "none",
                    height: "20",
                    x: "0",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M16,11c0.01-2.39-1.2-4.96-3-6l-0.33,0.41c-0.31,0.39-0.74,0.57-1.16,0.57c-0.77,0-1.51-0.59-1.51-1.5V2c0,0-6,3.75-6,9 c0,3.31,2.69,6,6,6S16,14.31,16,11L16,11z M10,15.5c-0.73,0-1.5-0.58-1.5-1.5c0-0.47,0.21-0.8,0.39-1L10,11.75L11.11,13 c0.18,0.2,0.39,0.53,0.39,1C11.5,14.83,10.83,15.5,10,15.5z M12.96,14.37L12.96,14.37c0.03-0.25,0.19-1.35-0.73-2.37L10,9.5 L7.77,12c-0.92,1.02-0.76,2.12-0.73,2.36C6.1,13.53,5.5,12.34,5.5,11c0-2.41,1.62-4.52,3.06-5.92c0.28,1.37,1.5,2.39,2.95,2.39 c0.57,0,1.11-0.16,1.58-0.46c0.85,0.99,1.41,2.55,1.41,3.98C14.5,12.34,13.9,13.55,12.96,14.37z",
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M16,6l-0.44,0.55c-0.42,0.52-0.98,0.75-1.54,0.75C13,7.3,12,6.52,12,5.3V2c0,0-8,4-8,11c0,4.42,3.58,8,8,8s8-3.58,8-8 C20,10.04,18.39,7.38,16,6z M12,19c-1.1,0-2-0.87-2-1.94c0-0.51,0.2-0.99,0.58-1.36l1.42-1.4l1.43,1.4 C13.8,16.07,14,16.55,14,17.06C14,18.13,13.1,19,12,19z M15.96,17.5L15.96,17.5c0.04-0.36,0.22-1.89-1.13-3.22l0,0L12,11.5 l-2.83,2.78l0,0c-1.36,1.34-1.17,2.88-1.13,3.22C6.79,16.4,6,14.79,6,13c0-3.16,2.13-5.65,4.03-7.25c0.23,1.99,1.93,3.55,3.99,3.55 c0.78,0,1.54-0.23,2.18-0.66C17.34,9.78,18,11.35,18,13C18,14.79,17.21,16.4,15.96,17.5z",
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
                d: "M8.66 13.07c.15 0 .29-.01.43-.03C9.56 14.19 10.69 15 12 15s2.44-.81 2.91-1.96c.14.02.29.03.43.03 1.73 0 3.14-1.41 3.14-3.14 0-.71-.25-1.39-.67-1.93.43-.54.67-1.22.67-1.93 0-1.73-1.41-3.14-3.14-3.14-.15 0-.29.01-.43.03C14.44 1.81 13.31 1 12 1s-2.44.81-2.91 1.96c-.14-.02-.29-.03-.43-.03-1.73 0-3.14 1.41-3.14 3.14 0 .71.25 1.39.67 1.93-.43.54-.68 1.22-.68 1.93 0 1.73 1.41 3.14 3.15 3.14zM12 13c-.62 0-1.12-.49-1.14-1.1l.12-1.09c.32.12.66.19 1.02.19s.71-.07 1.03-.19l.11 1.09c-.02.61-.52 1.1-1.14 1.1zm3.34-1.93c-.24 0-.46-.07-.64-.2l-.81-.57c.55-.45.94-1.09 1.06-1.83l.88.42c.4.19.66.59.66 1.03 0 .64-.52 1.15-1.15 1.15zm-.65-5.94c.2-.13.42-.2.65-.2.63 0 1.14.51 1.14 1.14 0 .44-.25.83-.66 1.03l-.88.42c-.12-.74-.51-1.38-1.07-1.83l.82-.56zM12 3c.62 0 1.12.49 1.14 1.1l-.11 1.09C12.71 5.07 12.36 5 12 5s-.7.07-1.02.19l-.12-1.09c.02-.61.52-1.1 1.14-1.1zM8.66 4.93c.24 0 .46.07.64.2l.81.56c-.55.45-.94 1.09-1.06 1.83l-.88-.42c-.4-.2-.66-.59-.66-1.03 0-.63.52-1.14 1.15-1.14zM8.17 8.9l.88-.42c.12.74.51 1.38 1.07 1.83l-.81.55c-.2.13-.42.2-.65.2-.63 0-1.14-.51-1.14-1.14-.01-.43.25-.82.65-1.02zM12 22c4.97 0 9-4.03 9-9-4.97 0-9 4.03-9 9zm2.44-2.44c.71-1.9 2.22-3.42 4.12-4.12-.71 1.9-2.22 3.41-4.12 4.12zM3 13c0 4.97 4.03 9 9 9 0-4.97-4.03-9-9-9zm2.44 2.44c1.9.71 3.42 2.22 4.12 4.12-1.9-.71-3.41-2.22-4.12-4.12z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "m19.77 7.23.01-.01-3.72-3.72L15 4.56l2.11 2.11c-.94.36-1.61 1.26-1.61 2.33 0 1.38 1.12 2.5 2.5 2.5.36 0 .69-.08 1-.21v7.21c0 .55-.45 1-1 1s-1-.45-1-1V14c0-1.1-.9-2-2-2h-1V5c0-1.1-.9-2-2-2H6c-1.1 0-2 .9-2 2v16h10v-7.5h1.5v5c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5V9c0-.69-.28-1.32-.73-1.77zM12 13.5V19H6v-7h6v1.5zm0-3.5H6V5h6v5zm6 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zm10 0c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2zm-1.45-5c.75 0 1.41-.41 1.75-1.03l3.58-6.49c.37-.66-.11-1.48-.87-1.48H5.21l-.94-2H1v2h2l3.6 7.59-1.35 2.44C4.52 15.37 5.48 17 7 17h12v-2H7l1.1-2h7.45zM6.16 6h12.15l-2.76 5H8.53L6.16 6z",
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
                d: "M19 3H5c-1.1 0-1.99.9-1.99 2L3 19c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zm-8.5-2h3v-3.5H17v-3h-3.5V7h-3v3.5H7v3h3.5z",
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
                d: "M7 14c1.66 0 3-1.34 3-3S8.66 8 7 8s-3 1.34-3 3 1.34 3 3 3zm0-4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm12-3h-8v8H3V5H1v15h2v-3h18v3h2v-9c0-2.21-1.79-4-4-4zm2 8h-8V9h6c1.1 0 2 .9 2 2v4z",
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
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M18 2.01L6 2c-1.11 0-2 .89-2 2v16c0 1.11.89 2 2 2h12c1.11 0 2-.89 2-2V4c0-1.11-.89-1.99-2-1.99zM18 20H6L5.99 4H18v16z",
            }
            circle {
                cy: "6",
                r: "1",
                cx: "8",
            }
            circle {
                cx: "11",
                cy: "6",
                r: "1",
            }
            path {
                d: "M12 19c2.76 0 5-2.24 5-5s-2.24-5-5-5-5 2.24-5 5 2.24 5 5 5zm2.36-7.36c1.3 1.3 1.3 3.42 0 4.72-1.3 1.3-3.42 1.3-4.72 0l4.72-4.72z",
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
                d: "M12 9c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0-6c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm0 8.55C9.64 9.35 6.48 8 3 8v11c3.48 0 6.64 1.35 9 3.55 2.36-2.19 5.52-3.55 9-3.55V8c-3.48 0-6.64 1.35-9 3.55zm7 5.58c-2.53.34-4.93 1.3-7 2.82-2.06-1.52-4.47-2.49-7-2.83v-6.95c2.1.38 4.05 1.35 5.64 2.83L12 14.28l1.36-1.27c1.59-1.48 3.54-2.45 5.64-2.83v6.95z",
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
                d: "M19 6h-2c0-2.76-2.24-5-5-5S7 3.24 7 6H5c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-7-3c1.66 0 3 1.34 3 3H9c0-1.66 1.34-3 3-3zm7 17H5V8h14v12zm-7-8c-1.66 0-3-1.34-3-3H7c0 2.76 2.24 5 5 5s5-2.24 5-5h-2c0 1.66-1.34 3-3 3z",
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
                d: "M14 5v14h-4V5h4m6-2h-2v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3zm-4 6V7h2v2h-2zM6 9V7h2v2H6zm10 4v-2h2v2h-2zM6 13v-2h2v2H6zm10 4v-2h2v2h-2zM6 17v-2h2v2H6z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "m21.41 11.58-9-9C12.05 2.22 11.55 2 11 2H4c-1.1 0-2 .9-2 2v7c0 .55.22 1.05.59 1.42l9 9c.36.36.86.58 1.41.58s1.05-.22 1.41-.59l7-7c.37-.36.59-.86.59-1.41s-.23-1.06-.59-1.42zM13 20.01 4 11V4h7v-.01l9 9-7 7.02z",
            }
            circle {
                cy: "6.5",
                cx: "6.5",
                r: "1.5",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M13 3H6v18h4v-6h3c3.31 0 6-2.69 6-6s-2.69-6-6-6zm.2 8H10V7h3.2c1.1 0 2 .9 2 2s-.9 2-2 2z",
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
                d: "M21 5h-2.64l1.14-3.14L17.15 1l-1.46 4H3v2l2 6-2 6v2h18v-2l-2-6 2-6V5zm-3.9 8.63L18.89 19H5.11l1.79-5.37.21-.63-.21-.63L5.11 7h13.78l-1.79 5.37-.21.63.21.63zM13 9h-2v3H8v2h3v3h2v-3h3v-2h-3z",
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
                d: "M6.54 5c.06.89.21 1.76.45 2.59l-1.2 1.2c-.41-1.2-.67-2.47-.76-3.79h1.51m9.86 12.02c.85.24 1.72.39 2.6.45v1.49c-1.32-.09-2.59-.35-3.8-.75l1.2-1.19M7.5 3H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.49c0-.55-.45-1-1-1-1.24 0-2.45-.2-3.57-.57-.1-.04-.21-.05-.31-.05-.26 0-.51.1-.71.29l-2.2 2.2c-2.83-1.45-5.15-3.76-6.59-6.59l2.2-2.2c.28-.28.36-.67.25-1.02C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1z",
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
                d: "M12 2C8.43 2 5.23 3.54 3.01 6L12 22l8.99-16C18.78 3.55 15.57 2 12 2zm0 15.92L5.51 6.36C7.32 4.85 9.62 4 12 4s4.68.85 6.49 2.36L12 17.92zM9 5.5c-.83 0-1.5.67-1.5 1.5S8.17 8.5 9 8.5s1.5-.67 1.5-1.5S9.82 5.5 9 5.5zm1.5 7.5c0 .83.67 1.5 1.5 1.5.82 0 1.5-.67 1.5-1.5s-.68-1.5-1.5-1.5-1.5.67-1.5 1.5z",
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
                d: "M22 10V6c0-1.1-.9-2-2-2H4c-1.1 0-1.99.9-1.99 2v4c1.1 0 1.99.9 1.99 2s-.89 2-2 2v4c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-4c-1.1 0-2-.9-2-2s.9-2 2-2zm-2-1.46c-1.19.69-2 1.99-2 3.46s.81 2.77 2 3.46V18H4v-2.54c1.19-.69 2-1.99 2-3.46 0-1.48-.8-2.77-1.99-3.46L4 6h16v2.54zM9.07 16L12 14.12 14.93 16l-.89-3.36 2.69-2.2-3.47-.21L12 7l-1.27 3.22-3.47.21 2.69 2.2z",
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
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M14.5,12.59l0.9,3.88L12,14.42l-3.4,2.05l0.9-3.87l-3-2.59l3.96-0.34L12,6.02l1.54,3.64L17.5,10L14.5,12.59z M12,3.19 l7,3.11V11c0,4.52-2.98,8.69-7,9.93C7.98,19.69,5,15.52,5,11V6.3L12,3.19 M12,1L3,5v6c0,5.55,3.84,10.74,9,12c5.16-1.26,9-6.45,9-12 V5L12,1L12,1z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M22 6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6zm-2 0l-8 5-8-5h16zm0 12H4V8l8 5 8-5v10z",
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
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 8h-1V3H6v5H5c-1.66 0-3 1.34-3 3v6h4v4h12v-4h4v-6c0-1.66-1.34-3-3-3zM8 5h8v3H8V5zm8 14H8v-4h8v4zm2-4v-2H6v2H4v-4c0-.55.45-1 1-1h14c.55 0 1 .45 1 1v4h-2z",
            }
            circle {
                r: "1",
                cy: "11.5",
                cx: "18",
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
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M20 4h-3.17L15 2H9L7.17 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V6h4.05l.59-.65L9.88 4h4.24l1.24 1.35.59.65H20v12zM12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zm0 8.2c-1.77 0-3.2-1.43-3.2-3.2 0-1.77 1.43-3.2 3.2-3.2s3.2 1.43 3.2 3.2c0 1.77-1.43 3.2-3.2 3.2z",
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
                d: "M20 8h-3V4H3c-1.1 0-2 .9-2 2v11h2c0 1.66 1.34 3 3 3s3-1.34 3-3h6c0 1.66 1.34 3 3 3s3-1.34 3-3h2v-5l-3-4zm-.5 1.5l1.96 2.5H17V9.5h2.5zM6 18c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm2.22-3c-.55-.61-1.33-1-2.22-1s-1.67.39-2.22 1H3V6h12v9H8.22zM18 18c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M18.92 6.01C18.72 5.42 18.16 5 17.5 5H15V3H9v2H6.5c-.66 0-1.21.42-1.42 1.01L3 12v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.85 7h10.29l1.04 3H5.81l1.04-3zM19 17H5v-4.66l.12-.34h13.77l.11.34V17z",
            }
            circle {
                cx: "7.5",
                cy: "14.5",
                r: "1.5",
            }
            circle {
                cy: "14.5",
                r: "1.5",
                cx: "16.5",
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
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M17.99,8c0.26-2.73-3.42-5-7.99-5C5.44,3,1.76,5.27,2.01,8H17.99z M10,4.5c2.5,0,4.83,0.78,5.94,2H4.07 C5.18,5.29,7.51,4.5,10,4.5z",
                    }
                    path {
                        d: "M2,15.5C2,16.33,2.67,17,3.5,17h13c0.83,0,1.5-0.67,1.5-1.5V13H2V15.5z M3.5,14.5h13v1h-13V14.5z",
                    }
                    path {
                        d: "M18,11.75c-1.58,0-1.72-1-2.66-1c-0.95,0-1.08,1-2.67,1c-1.58,0-1.72-1-2.67-1c-0.95,0-1.08,1-2.67,1 c-1.58,0-1.72-1-2.67-1c-0.95,0-1.09,1-2.67,1v-1.5c0.95,0,1.09-1,2.67-1c1.58,0,1.72,1,2.67,1c0.95,0,1.08-1,2.67-1 c1.58,0,1.72,1,2.67,1c0.95,0,1.08-1,2.67-1c1.58,0,1.72,1,2.66,1V11.75z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M2,19c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-3H2V19z M4,18h16v1H4V18z",
                    }
                    path {
                        d: "M18.66,11.5c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1 c-1.95,0-2.09,1-3.33,1v2c1.9,0,2.17-1,3.35-1c1.19,0,1.42,1,3.33,1c1.95,0,2.09-1,3.33-1c1.19,0,1.42,1,3.33,1 c1.95,0,2.09-1,3.33-1c1.19,0,1.4,0.98,3.32,1l-0.01-1.98C20.38,12.19,20.37,11.5,18.66,11.5z",
                    }
                    path {
                        d: "M22,9c0.02-4-4.28-6-10-6C6.29,3,2,5,2,9v1h20L22,9L22,9z M4.18,8C5.01,5.81,8.61,5,12,5c3.31,0,5.93,0.73,7.19,1.99 C19.49,7.3,19.71,7.63,19.84,8H4.18z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M20.5 3l-.16.03L15 5.1 9 3 3.36 4.9c-.21.07-.36.25-.36.48V20.5c0 .28.22.5.5.5l.16-.03L9 18.9l6 2.1 5.64-1.9c.21-.07.36-.25.36-.48V3.5c0-.28-.22-.5-.5-.5zM10 5.47l4 1.4v11.66l-4-1.4V5.47zm-5 .99l3-1.01v11.7l-3 1.16V6.46zm14 11.08l-3 1.01V6.86l3-1.16v11.84z",
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
                fill_rule: "evenodd",
                height: "24",
                fill: "none",
                width: "24",
            }
            g {
                path {
                    d: "M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8c-1.18,0-2.34-0.26-3.43-0.78c-0.27-0.13-0.56-0.19-0.86-0.19 c-0.19,0-0.38,0.03-0.56,0.08l-3.2,0.94l0.94-3.2c0.14-0.47,0.1-0.98-0.11-1.42C4.26,14.34,4,13.18,4,12C4,7.59,7.59,4,12,4 M12,2 C6.48,2,2,6.48,2,12c0,1.54,0.36,2.98,0.97,4.29L1,23l6.71-1.97C9.02,21.64,10.46,22,12,22c5.52,0,10-4.48,10-10 C22,6.48,17.52,2,12,2L12,2z",
                }
            }
            polygon {
                fill_rule: "evenodd",
                points: "13,8 11,8 11,11 8,11 8,13 11,13 11,16 13,16 13,13 16,13 16,11 13,11",
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
                    x: "0",
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M9.25,13.25H7.5V15H6v-1.75H4.25v-1.5H6V10h1.5v1.75h1.75V13.25z M14,14.5v-1h-3.25v1H14z M15.75,12v-1h-5v1H15.75z M18,7.5v9c0,0.83-0.67,1.5-1.5,1.5h-13C2.67,18,2,17.33,2,16.5v-9C2,6.67,2.67,6,3.5,6h4.25V3.5c0-0.83,0.67-1.5,1.5-1.5h1.5 c0.83,0,1.5,0.67,1.5,1.5V6h4.25C17.33,6,18,6.67,18,7.5z M9.25,7.5h1.5v-4h-1.5V7.5z M16.5,7.5h-4.25c0,0.83-0.67,1.5-1.5,1.5 h-1.5c-0.83,0-1.5-0.67-1.5-1.5H3.5v9h13V7.5z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M20,7h-5V4c0-1.1-0.9-2-2-2h-2C9.9,2,9,2.9,9,4v3H4C2.9,7,2,7.9,2,9v11c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V9 C22,7.9,21.1,7,20,7z M11,4h2v5h-2V4z M20,20H4V9h5c0,1.1,0.9,2,2,2h2c1.1,0,2-0.9,2-2h5V20z M11,16H9v2H7v-2H5v-2h2v-2h2v2h2V16z M13,14.5V13h6v1.5H13z M13,17.5V16h4v1.5H13z",
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
                    width: "20",
                    fill: "none",
                    x: "0",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M16,6h-3V4c0-0.55-0.45-1-1-1H8C7.45,3,7,3.45,7,4v2H4C3.45,6,3,6.45,3,7v9c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1V7 C17,6.45,16.55,6,16,6z M8,4h4v2H8V4z M16,16H4V7h12V16z",
                    }
                    polygon {
                        points: "10.5,9 9.5,9 9.5,11 7.5,11 7.5,12 9.5,12 9.5,14 10.5,14 10.5,12 12.5,12 12.5,11 10.5,11",
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M20,6h-4V4c0-1.1-0.9-2-2-2h-4C8.9,2,8,2.9,8,4v2H4C2.9,6,2,6.9,2,8v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M10,4h4v2h-4V4z M20,20H4V8h16V20z",
                    }
                    polygon {
                        points: "13,10 11,10 11,13 8,13 8,15 11,15 11,18 13,18 13,15 16,15 16,13 13,13",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                }
                g {
                    path {
                        d: "M21,5c-1.11-0.35-2.33-0.5-3.5-0.5c-1.95,0-4.05,0.4-5.5,1.5c-1.45-1.1-3.55-1.5-5.5-1.5S2.45,4.9,1,6v14.65 c0,0.25,0.25,0.5,0.5,0.5c0.1,0,0.15-0.05,0.25-0.05C3.1,20.45,5.05,20,6.5,20c1.95,0,4.05,0.4,5.5,1.5c1.35-0.85,3.8-1.5,5.5-1.5 c1.65,0,3.35,0.3,4.75,1.05c0.1,0.05,0.15,0.05,0.25,0.05c0.25,0,0.5-0.25,0.5-0.5V6C22.4,5.55,21.75,5.25,21,5z M21,18.5 c-1.1-0.35-2.3-0.5-3.5-0.5c-1.7,0-4.15,0.65-5.5,1.5V8c1.35-0.85,3.8-1.5,5.5-1.5c1.2,0,2.4,0.15,3.5,0.5V18.5z",
                    }
                    g {
                        path {
                            d: "M17.5,10.5c0.88,0,1.73,0.09,2.5,0.26V9.24C19.21,9.09,18.36,9,17.5,9c-1.7,0-3.24,0.29-4.5,0.83v1.66 C14.13,10.85,15.7,10.5,17.5,10.5z",
                        }
                        path {
                            d: "M13,12.49v1.66c1.13-0.64,2.7-0.99,4.5-0.99c0.88,0,1.73,0.09,2.5,0.26V11.9c-0.79-0.15-1.64-0.24-2.5-0.24 C15.8,11.66,14.26,11.96,13,12.49z",
                        }
                        path {
                            d: "M17.5,14.33c-1.7,0-3.24,0.29-4.5,0.83v1.66c1.13-0.64,2.7-0.99,4.5-0.99c0.88,0,1.73,0.09,2.5,0.26v-1.52 C19.21,14.41,18.36,14.33,17.5,14.33z",
                        }
                    }
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
                    x: "0",
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M9.25,10.45c0,0.8-0.32,1.56-0.88,2.12L5,15.94L6.06,17L10,13.06L13.94,17L15,15.94l-3.37-3.37 c-0.56-0.56-0.88-1.33-0.88-2.12l0-4.58l1.19,1.19L13,6l-3-3L7,6l1.06,1.06l1.19-1.19L9.25,10.45z",
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
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M6.41,21L5,19.59l4.83-4.83c0.75-0.75,1.17-1.77,1.17-2.83v-5.1L9.41,8.41L8,7l4-4l4,4l-1.41,1.41L13,6.83v5.1 c0,1.06,0.42,2.08,1.17,2.83L19,19.59L17.59,21L12,15.41L6.41,21z",
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
                    width: "20",
                    x: "0",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M17,12v6.5c0,0.28-0.22,0.5-0.5,0.5h-1c-0.28,0-0.5-0.22-0.5-0.5v-1H5v1C5,18.78,4.78,19,4.5,19h-1C3.22,19,3,18.78,3,18.5 L3,12l1.62-4.06C4.85,7.37,5.4,7,6.02,7h7.97c0.61,0,1.16,0.37,1.39,0.94L17,12z M6.02,8.5l-1,2.5h9.97l-1-2.5H6.02z M15.5,16v-3.5 h-11V16H15.5z M6.5,13.25c-0.55,0-1,0.45-1,1s0.45,1,1,1c0.55,0,1-0.45,1-1S7.05,13.25,6.5,13.25z M13.5,13.25c-0.55,0-1,0.45-1,1 s0.45,1,1,1c0.55,0,1-0.45,1-1S14.05,13.25,13.5,13.25z M6.95,5.75L8,4.7L5.65,2.35L4.6,3.4L6.95,5.75z M15.4,3.4l-1.05-1.05 L12,4.7l1.05,1.05L15.4,3.4z M10.75,4.7h-1.5V1h1.5V4.7z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18.92,9.01C18.72,8.42,18.16,8,17.5,8h-11C5.84,8,5.29,8.42,5.08,9.01L3,15v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1 h12v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-8L18.92,9.01z M6.85,10h10.29l1.04,3H5.81L6.85,10z M19,20H5v-5h14V20z M6,17.5 C6,16.67,6.67,16,7.5,16S9,16.67,9,17.5S8.33,19,7.5,19S6,18.33,6,17.5z M15,17.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 S17.33,19,16.5,19S15,18.33,15,17.5z M9.41,5L8,6.41l-3-3L6.41,2L9.41,5z M16,6.41L14.59,5l3-3L19,3.41L16,6.41z M13,5h-2V0h2V5z",
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
                    width: "20",
                    x: "0",
                    fill: "none",
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
                    fill: "none",
                    height: "24",
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
                height: "20",
                fill: "none",
                width: "20",
            }
            path {
                d: "M16.5,8.63c0-0.04,0-0.09,0-0.13C16.5,4.91,13.59,2,10,2S3.5,4.91,3.5,8.5C3.5,12.84,10,18,10,18s1.28-1.01,2.7-2.52 c0.1,0.01,0.2,0.02,0.3,0.02c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2c0,0.44,0.14,0.85,0.38,1.18c-0.49,0.51-0.97,0.97-1.38,1.35 C8.1,14.3,5,10.97,5,8.5c0-2.76,2.24-5,5-5s5,2.24,5,5c0,0.04,0,0.08,0,0.13l-1.69-1.69L12.25,8l3.5,3.5l3.5-3.5l-1.06-1.06 L16.5,8.63z",
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
                width: "24",
                height: "24",
                fill: "none",
            }
            path {
                d: "M15.31,18.9c-0.96,1-2.06,2.03-3.31,3.1c-5.33-4.55-8-8.48-8-11.8C4,5.22,7.8,2,12,2c4.19,0,7.99,3.21,8,8.17l2.09-2.09 L23.5,9.5L19,14l-4.5-4.5l1.41-1.41L18,10.17C17.99,6.55,15.34,4,12,4c-3.35,0-6,2.57-6,6.2c0,2.34,1.95,5.44,6,9.14 c0.64-0.59,1.23-1.16,1.77-1.71c-0.17-0.34-0.27-0.72-0.27-1.12c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5S17.38,19,16,19 C15.76,19,15.53,18.97,15.31,18.9z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M15 16h3c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1h-3c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1zm1-6h1v4h-1v-4zm-7 6h3c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1H9c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1zm1-6h1v4h-1v-4zM5 8h2v8H5zM2 4v16h20V4H2zm18 14H4V6h16v12z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M15,7.5C15,6.67,14.33,6,13.5,6H12v1h1.5C13.78,7,14,7.22,14,7.5v1.29L10.79,12H9V8H6c-1.66,0-3,1.34-3,3v2h2 c0,1.1,0.9,2,2,2s2-0.9,2-2h2.21L15,9.21V7.5z M4,12v-1c0-1.1,0.9-2,2-2h2v3H4z M7,14c-0.55,0-1-0.45-1-1h2C8,13.55,7.55,14,7,14z",
                    }
                    rect {
                        x: "5",
                        width: "4",
                        height: "1",
                        y: "6",
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
            }
            g {
                g {
                    path {
                        d: "M19,7c0-1.1-0.9-2-2-2h-3v2h3v2.65L13.52,14H10V9H6c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,10.35V7 z M4,14v-1c0-1.1,0.9-2,2-2h2v3H4z M7,17c-0.55,0-1-0.45-1-1h2C8,16.55,7.55,17,7,17z",
                    }
                    rect {
                        height: "2",
                        x: "5",
                        y: "6",
                        width: "5",
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
                    x: "0",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M18.25,3c0,0-1.5,1.67-1.5,2.5c0,0.55,0.3,1.03,0.75,1.29V11H16V9.5C16,8.67,15.33,8,14.5,8h-0.14 C14.76,7.43,15,6.74,15,5.99c0-1.23-0.65-2.38-1.71-3.01L10,1L6.71,2.98C5.65,3.61,5,4.76,5,5.99C5,6.74,5.24,7.43,5.64,8H5.5 C4.67,8,4,8.67,4,9.5V11H2.5V6.79C2.95,6.53,3.25,6.05,3.25,5.5c0-0.83-1.5-2.5-1.5-2.5s-1.5,1.67-1.5,2.5 c0,0.55,0.3,1.03,0.75,1.29V17h8v-3.5c0-0.55,0.45-1,1-1s1,0.45,1,1V17h8V6.79c0.45-0.26,0.75-0.74,0.75-1.29 C19.75,4.67,18.25,3,18.25,3z M7.48,4.26L10,2.75l2.52,1.51c0.6,0.36,0.98,1.02,0.98,1.73c0,1.1-0.88,1.99-1.97,2.01H8.48 C7.38,7.98,6.5,7.09,6.5,5.99C6.5,5.29,6.87,4.62,7.48,4.26z M17.5,15.5h-5v-2c0-1.38-1.12-2.5-2.5-2.5s-2.5,1.12-2.5,2.5v2h-5v-3 h3v-3h9v3h3V15.5z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M24,7c0-1.1-2-3-2-3s-2,1.9-2,3c0,0.74,0.4,1.38,1,1.72V13h-2v-2c0-0.95-0.66-1.74-1.55-1.94C17.79,8.48,18,7.81,18,7.09 c0-1.31-0.65-2.53-1.74-3.25L12,1L7.74,3.84C6.65,4.56,6,5.78,6,7.09c0,0.72,0.21,1.39,0.55,1.96C5.66,9.26,5,10.05,5,11v2H3V8.72 C3.6,8.38,4,7.74,4,7c0-1.1-2-3-2-3S0,5.9,0,7c0,0.74,0.4,1.38,1,1.72V21h10v-4c0-0.55,0.45-1,1-1s1,0.45,1,1v4h10V8.72 C23.6,8.38,24,7.74,24,7z M8.85,5.5L12,3.4l3.15,2.1C15.68,5.86,16,6.45,16,7.09C16,8.14,15.14,9,14.09,9H9.91 C8.86,9,8,8.14,8,7.09C8,6.45,8.32,5.86,8.85,5.5z M21,19h-6v-2c0-1.65-1.35-3-3-3c-1.65,0-3,1.35-3,3v2H3v-4h4v-4h10v4h4V19z",
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
                d: "M16.5,7.56l-3.7,3.7c-0.98,0.98-2.56,0.98-3.54,0l-0.7-0.7c-0.39-0.39-1.02-0.39-1.41,0l-4.09,4.09L2,13.59L6.08,9.5 c0.98-0.98,2.56-0.98,3.54,0l0.71,0.71c0.39,0.39,1.02,0.39,1.41,0l3.7-3.7H13V5h5v5h-1.5V7.56z",
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
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M20,9.42V12h2V6h-6v2h2.58l-4.46,4.46c-0.39,0.39-1.02,0.39-1.41,0l-1.17-1.17c-1.17-1.17-3.07-1.17-4.24,0L2,16.59L3.41,18 l5.29-5.29c0.39-0.39,1.02-0.39,1.41,0l1.17,1.17c1.17,1.17,3.07,1.17,4.24,0L20,9.42z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
                path {
                    d: "M17,4l4,4l-4,4V9h-4V7h4V4z M10,7C9.45,7,9,7.45,9,8s0.45,1,1,1s1-0.45,1-1S10.55,7,10,7z M6,7C5.45,7,5,7.45,5,8 s0.45,1,1,1s1-0.45,1-1S6.55,7,6,7z M7,17h4v-2H7v-3l-4,4l4,4V17z M14,17c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1s-1,0.45-1,1 C13,16.55,13.45,17,14,17z M18,17c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1s-1,0.45-1,1C17,16.55,17.45,17,18,17z",
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
                    width: "20",
                    height: "20",
                    x: "0",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M10,3.03L3,8h0.02v1H5v7H3.02v1h13.97v-1H15V9h1.98V8H17L10,3.03z M14,16H6V8h8V16z",
                    }
                    polygon {
                        points: "8,11.33 10,14 12,11.33 12,15 13,15 13,9 12,9 10,12 8,9 7,9 7,15 8,15",
                    }
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
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M22,11V9L12,2L2,9v2h2v9H2v2h20v-2h-2v-9H22z M18,20H6V9h12V20z",
                    }
                    polygon {
                        points: "10,14 12,17 14,14 14,18 16,18 16,11 14,11 12,14 10,11 8,11 8,18 10,18",
                    }
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
                d: "M12 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm8.94 3c-.46-4.17-3.77-7.48-7.94-7.94V1h-2v2.06C6.83 3.52 3.52 6.83 3.06 11H1v2h2.06c.46 4.17 3.77 7.48 7.94 7.94V23h2v-2.06c4.17-.46 7.48-3.77 7.94-7.94H23v-2h-2.06zM12 19c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 7.27l4.28 10.43-3.47-1.53-.81-.36-.81.36-3.47 1.53L12 7.27M12 2L4.5 20.29l.71.71L12 18l6.79 3 .71-.71L12 2z",
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
                d: "M17.27 6.73l-4.24 10.13-1.32-3.42-.32-.83-.82-.32-3.43-1.33 10.13-4.23M21 3L3 10.53v.98l6.84 2.65L12.48 21h.98L21 3z",
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
                fill: "none",
                height: "24",
                width: "24",
            }
            path {
                d: "M12,6.34L21,3l-3.34,9l-1.56-1.56l1.5-4.05l-4.05,1.5L12,6.34z M21.19,21.19l-5.07-5.07L14.31,21H12.9l-2.83-7.07L3,11.1 V9.69l4.88-1.81L2.81,2.81l1.41-1.41l18.38,18.38L21.19,21.19z M14.57,14.57L9.43,9.43l-2.71,1.01l4.89,1.95l1.95,4.89L14.57,14.57z",
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
                        d: "M7.74,10.96L12,5H2l4.24,5.94v3.56H5V16h4v-1.5H7.74V10.96z M5.99,8L4.91,6.5h4.17L8.01,8H5.99z",
                    }
                    path {
                        d: "M13.5,5v6.21c-0.31-0.13-0.64-0.21-1-0.21c-1.38,0-2.5,1.12-2.5,2.5c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5V7h2V5H13.5 z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M1,5h14l-6,9v4h2v2H5v-2h2v-4L1,5z M10.1,9l1.4-2H4.49l1.4,2H10.1z M17,5h5v3h-3v9l0,0c0,1.66-1.34,3-3,3s-3-1.34-3-3 s1.34-3,3-3c0.35,0,0.69,0.06,1,0.17V5z",
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
            path {
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12.01 16c-.27 0-.52-.1-.71-.29-.2-.2-.29-.43-.29-.71-.01-.55.43-.99.98-1h.02c.28 0 .51.1.71.29.18.19.28.43.28.7s-.1.51-.29.71-.43.3-.7.3zm-.88-3.66c0-.45.1-.84.29-1.16.19-.33.53-.7 1-1.12.28-.25.48-.47.61-.66s.19-.4.19-.64c0-.29-.11-.53-.32-.74-.21-.2-.5-.3-.85-.3-.37 0-.74.1-.96.3-.21.2-.4.45-.4.98H9c0-1.01.46-1.73.97-2.21C10.53 6.28 11.25 6 12 6c.59 0 1.11.12 1.57.35s.79.55 1.05.96.38.86.38 1.35-.1.9-.31 1.25-.48.71-.89 1.09c-.32.3-.53.56-.65.77s-.18.49-.18.81V13h-1.85v-.66h.01zM18 10.2C18 6.57 15.35 4 12 4s-6 2.57-6 6.2c0 2.34 1.95 5.44 6 9.14 4.05-3.7 6-6.8 6-9.14zM12 2c4.2 0 8 3.22 8 8.2 0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8C4 5.22 7.8 2 12 2z",
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
                    x: "0",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M11.87,1l1.06,1.06L9.4,5.6L7.28,3.47l1.06-1.06L9.4,3.47L11.87,1z M17,12v6.5c0,0.28-0.22,0.5-0.5,0.5h-1 c-0.28,0-0.5-0.22-0.5-0.5v-1H5v1C5,18.78,4.78,19,4.5,19h-1C3.22,19,3,18.78,3,18.5L3,12l1.62-4.06C4.85,7.37,5.4,7,6.02,7h7.97 c0.61,0,1.16,0.37,1.39,0.94L17,12z M6.02,8.5l-1,2.5h9.97l-1-2.5H6.02z M15.5,16v-3.5h-11V16H15.5z M6.5,13.25c-0.55,0-1,0.45-1,1 s0.45,1,1,1c0.55,0,1-0.45,1-1S7.05,13.25,6.5,13.25z M13.5,13.25c-0.55,0-1,0.45-1,1s0.45,1,1,1c0.55,0,1-0.45,1-1 S14.05,13.25,13.5,13.25z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M18.92,9.01C18.72,8.42,18.16,8,17.5,8h-11C5.84,8,5.29,8.42,5.08,9.01L3,15v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1 h12v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-8L18.92,9.01z M6.85,10h10.29l1.04,3H5.81L6.85,10z M19,20H5v-5h14V20z M6,17.5 C6,16.67,6.67,16,7.5,16S9,16.67,9,17.5S8.33,19,7.5,19S6,18.33,6,17.5z M15,17.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 S17.33,19,16.5,19S15,18.33,15,17.5z M12,6.36L9.17,3.54l1.41-1.41L12,3.54L15.54,0l1.41,1.41L12,6.36z",
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
                width: "24",
                height: "24",
                fill: "none",
            }
            path {
                d: "M16,14V6c0-1.76,2.24-4,5-4v16.17l-2-2V14H16z M20.49,23.31L10.02,12.85C9.69,12.94,9.36,13,9,13v9H7v-9c-2.21,0-4-1.79-4-4 V5.83L0.69,3.51L2.1,2.1l19.8,19.8L20.49,23.31z M6.17,9L5,7.83V9H6.17z M9,2H7v2.17l2,2V2z M13,9V2h-2v6.17l1.85,1.85 C12.94,9.69,13,9.36,13,9z",
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
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M8.5,13c0.83,0,1.5,0.67,1.5,1.5S9.33,16,8.5,16S7,15.33,7,14.5S7.67,13,8.5,13z M19.78,22.61l-1.64-1.64 C18.09,20.98,18.05,21,18,21h-1c-0.55,0-1-0.45-1-1v-1H8v1c0,0.55-0.45,1-1,1H6c-0.55,0-1-0.45-1-1v-1.78C4.39,17.67,4,16.88,4,16 V6.83L1.39,4.22l1.41-1.41l18.38,18.38L19.78,22.61z M6,8.83V10h1.17L6,8.83z M14.17,17l-5-5H6v4c0,0.37,0.21,0.62,0.34,0.73 L6.63,17H14.17z M12,4c3.69,0,5.11,0.46,5.66,0.99H7.82l2,2H18V10h-5.17l2,2H18v3.17l1.81,1.81C19.92,16.67,20,16.35,20,16V6 c0-3.5-3.58-4-8-4C9.48,2,7.24,2.16,5.78,2.95l1.53,1.53C8.17,4.2,9.6,4,12,4z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M13.82,10H15l-5-8l-5,8h1.18L3,15h5.5v3h3v-3H17L13.82,10z M5.73,13.5l3.18-5H7.71L10,4.83l2.29,3.67h-1.21l3.18,5H5.73z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M17,12h2L12,2L5.05,12H7l-3.9,6h6.92v4h3.95v-4H21L17,12z M6.79,16l3.9-6H8.88l3.13-4.5l3.15,4.5h-1.9l4,6H6.79z",
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
                    x: "0",
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M14.82,9l-1.58-4.34C13.1,4.26,12.72,4,12.3,4H10v1h2.3l1.46,4H8.75L8.38,8H10V7H6v1h1.32l1.46,4H7.95 C7.7,10.19,6.13,8.86,4.2,9.01c-1.64,0.13-3.01,1.46-3.18,3.1C0.8,14.25,2.41,16,4.5,16c1.79,0,3.21-1.29,3.45-3h4.1 c0.25,1.81,1.83,3.14,3.75,2.99c1.64-0.13,3.01-1.46,3.18-3.1C19.2,10.75,17.59,9,15.5,9H14.82z M9.11,10h3.92 c-0.53,0.52-0.88,1.22-0.98,2H9.84L9.11,10z M4.5,15C3.1,15,2,13.9,2,12.5S3.1,10,4.5,10c1.23,0,2.23,0.85,2.45,2H4v1h2.95 C6.73,14.15,5.73,15,4.5,15z M15.5,15c-1.4,0-2.5-1.1-2.5-2.5c0-0.94,0.5-1.73,1.24-2.16l1.03,2.83l0.94-0.34l-1.02-2.8 C15.3,10.02,15.4,10,15.5,10c1.4,0,2.5,1.1,2.5,2.5S16.9,15,15.5,15z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M18.18,10l-1.7-4.68C16.19,4.53,15.44,4,14.6,4H12v2h2.6l1.46,4h-4.81l-0.36-1H12V7H7v2h1.75l1.82,5H9.9 c-0.44-2.23-2.31-3.88-4.65-3.99C2.45,9.87,0,12.2,0,15c0,2.8,2.2,5,5,5c2.46,0,4.45-1.69,4.9-4h4.2c0.44,2.23,2.31,3.88,4.65,3.99 c2.8,0.13,5.25-2.19,5.25-5c0-2.8-2.2-5-5-5H18.18z M7.82,16c-0.4,1.17-1.49,2-2.82,2c-1.68,0-3-1.32-3-3s1.32-3,3-3 c1.33,0,2.42,0.83,2.82,2H5v2H7.82z M14.1,14h-1.4l-0.73-2H15C14.56,12.58,14.24,13.25,14.1,14z M19,18c-1.68,0-3-1.32-3-3 c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64l1.88-0.68l-0.97-2.67c0.03,0,0.06-0.01,0.09-0.01c1.68,0,3,1.32,3,3S20.68,18,19,18z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M19 2H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h4l3 3 3-3h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 16h-4.83l-.59.59L12 20.17l-1.59-1.59-.58-.58H5V4h14v14zm-7-7c1.65 0 3-1.35 3-3s-1.35-3-3-3-3 1.35-3 3 1.35 3 3 3zm0-4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm6 8.58c0-2.5-3.97-3.58-6-3.58s-6 1.08-6 3.58V17h12v-1.42zM8.48 15c.74-.51 2.23-1 3.52-1s2.78.49 3.52 1H8.48z",
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
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M12 11c1.33 0 4 .67 4 2v.16c-.97 1.12-2.4 1.84-4 1.84s-3.03-.72-4-1.84V13c0-1.33 2.67-2 4-2zm0-1c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm6 .2C18 6.57 15.35 4 12 4s-6 2.57-6 6.2c0 2.34 1.95 5.44 6 9.14 4.05-3.7 6-6.8 6-9.14zM12 2c4.2 0 8 3.22 8 8.2 0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8C4 5.22 7.8 2 12 2z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M16,12v-1h-2.04c-0.04-0.38-0.11-0.74-0.22-1.08l1.71-0.99l-0.5-0.87L13.33,9c-0.24-0.4-0.54-0.74-0.87-1.03 c0.07-0.39,0.13-1.19-0.48-1.99l1.24-1.24l-0.71-0.71l-1.29,1.29c-0.41-0.23-1.35-0.61-2.43,0L7.49,4.04L6.78,4.74l1.24,1.24 C7.41,6.78,7.47,7.58,7.55,7.97C7.21,8.26,6.91,8.6,6.67,9L5.05,8.07l-0.5,0.87l1.71,0.99c-0.11,0.34-0.18,0.7-0.22,1.08H4v1h2.04 c0.04,0.38,0.11,0.74,0.22,1.08l-1.71,0.99l0.5,0.87L6.67,14c0.72,1.21,1.94,2,3.33,2s2.61-0.8,3.33-2l1.62,0.94l0.5-0.87 l-1.71-0.99c0.11-0.34,0.18-0.7,0.22-1.08H16z M10,6c0.77,0,1.4,0.58,1.48,1.33C11.02,7.12,10.52,7,10,7S8.98,7.12,8.52,7.33 C8.6,6.58,9.23,6,10,6z M10,15c-1.65,0-3-1.57-3-3.5C7,9.57,8.35,8,10,8s3,1.57,3,3.5C13,13.43,11.65,15,10,15z",
                    }
                    g {
                        rect {
                            width: "1",
                            x: "9.5",
                            height: "4",
                            y: "9.5",
                        }
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M21,15v-2h-3.07c-0.05-0.39-0.12-0.77-0.22-1.14l2.58-1.49l-1-1.73L16.92,10c-0.28-0.48-0.62-0.91-0.99-1.29 c0.1-0.56,0.2-1.69-0.58-2.89L17,4.17l-1.41-1.41l-1.72,1.72c-1.68-0.89-3.1-0.33-3.73,0L8.41,2.76L7,4.17l1.65,1.65 c-0.78,1.2-0.68,2.34-0.58,2.89C7.7,9.1,7.36,9.53,7.08,10L4.71,8.63l-1,1.73l2.58,1.49c-0.1,0.37-0.17,0.75-0.22,1.14H3v2h3.07 c0.05,0.39,0.12,0.77,0.22,1.14l-2.58,1.49l1,1.73L7.08,18c1.08,1.81,2.88,3,4.92,3s3.84-1.19,4.92-3l2.37,1.37l1-1.73l-2.58-1.49 c0.1-0.37,0.17-0.75,0.22-1.14H21z M12,6c0.88,0,1.62,0.57,1.88,1.36C13.29,7.13,12.66,7,12,7s-1.29,0.13-1.88,0.36 C10.38,6.57,11.12,6,12,6z M12,19c-2.21,0-4-2.24-4-5s1.79-5,4-5s4,2.24,4,5S14.21,19,12,19z",
                    }
                    rect {
                        y: "11",
                        x: "11",
                        width: "2",
                        height: "6",
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
                    width: "20",
                    x: "0",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M16.32,12.38l-1.34-1.21c0.13-2.27-2.05-3.63-3.93-3C10.7,8.07,10.34,8,9.97,8c-2.02,0-3.67,1.52-3.92,3.48 C4.9,11.38,4,10.42,4,9.25C4,8.01,5.01,7,6.25,7H8.5C9.33,7,10,6.33,10,5.5S9.33,4,8.5,4h-1C7.22,4,7,4.22,7,4.5 C7,4.78,7.22,5,7.5,5h1C8.78,5,9,5.22,9,5.5C9,5.78,8.78,6,8.5,6H6.25C4.46,6,3,7.46,3,9.25c0,1.72,1.35,3.12,3.04,3.23 C6.27,14.46,7.93,16,9.97,16h4.95C16.83,16,17.71,13.65,16.32,12.38z M14.92,15H9.97C8.33,15,7,13.67,7,12.03 c0-2.47,2.24-2.98,2.76-3.01c-1.04,1.18-1.01,2.97,0.12,4.1h0l0.71-0.71C9.31,11.14,10.24,9,12,9l0,0c0,0,0,0,0,0 c0.47,0,2.09,0.2,1.96,2.59l1.69,1.53C16.38,13.79,15.9,15,14.92,15z",
                    }
                    circle {
                        r: ".75",
                        cy: "13.45",
                        cx: "13.55",
                    }
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    circle {
                        cx: "17",
                        cy: "17",
                        r: "1",
                    }
                    path {
                        d: "M20.86,14.97l-0.93-0.84c0.48-3.45-2.87-6.04-6.05-4.82C13.3,9.11,12.66,9,12,9c-4.26,0-5.65,3.58-5.89,4.85 C4.89,13.47,4,12.35,4,11c0-1.66,1.34-3,3-3h2.5C10.88,8,12,6.88,12,5.5C12,4.12,10.88,3,9.5,3H8C7.45,3,7,3.45,7,4 c0,0.55,0.45,1,1,1h1.5C9.78,5,10,5.22,10,5.5C10,5.78,9.78,6,9.5,6H7c-2.76,0-5,2.24-5,5c0,2.44,1.76,4.47,4.07,4.91 C6.51,18.79,8.99,21,12,21h6.53C21.64,21,23.23,17.11,20.86,14.97z M18.53,19H12c-1.21,0-2.34-0.54-3.11-1.48 c-0.78-0.95-1.06-2.16-0.8-3.41c0.31-1.48,1.51-2.69,2.99-3.01c0.22-0.05,0.45-0.06,0.67-0.07C11.28,11.74,11,12.58,11,13.5 c0,1.24,0.5,2.37,1.32,3.18l1.41-1.41C13.28,14.82,13,14.19,13,13.5c0-1.42,1.2-2.5,2.5-2.5c1.38,0,2.5,1.12,2.5,2.5 c0,0.46-0.13,0.88-0.35,1.25l1.87,1.7c0.31,0.28,0.48,0.67,0.48,1.09C20,18.34,19.34,19,18.53,19z",
                    }
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
            path {
                fill: "none",
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M12 4c1.93 0 5 1.4 5 5.15 0 2.16-1.72 4.67-5 7.32-3.28-2.65-5-5.17-5-7.32C7 5.4 10.07 4 12 4m0-2C8.73 2 5 4.46 5 9.15c0 3.12 2.33 6.41 7 9.85 4.67-3.44 7-6.73 7-9.85C19 4.46 15.27 2 12 2z",
            }
            path {
                d: "M12 7c-1.1 0-2 .9-2 2s.9 2 2 2a2 2 0 100-4zM5 20h14v2H5v-2z",
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
            path {
                fill: "none",
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M12 12c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm6-1.8C18 6.57 15.35 4 12 4s-6 2.57-6 6.2c0 2.34 1.95 5.44 6 9.14 4.05-3.7 6-6.8 6-9.14zM12 2c4.2 0 8 3.22 8 8.2 0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8C4 5.22 7.8 2 12 2z",
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
                    height: "20",
                    fill: "none",
                    x: "0",
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
                        d: "M12.58,6.57l-1.41,1.41L8.34,5.15c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41L9.75,9.4L9.04,10.1 l-5.3,5.3c-0.59,0.59-0.59,1.54,0,2.12l0,0c0.59,0.59,1.54,0.59,2.12,0l7.42-7.42l0,0c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L12.58,6.57z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
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
                        d: "M15.04,7.76l-0.71,0.71l-0.71,0.71l-3.18-3.18C9.85,5.4,8.9,5.4,8.32,5.99c-0.59,0.59-0.59,1.54,0,2.12l3.18,3.18 L10.79,12l-6.36,6.36c-0.78,0.78-0.78,2.05,0,2.83c0.78,0.78,2.05,0.78,2.83,0L16.45,12c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41L15.04,7.76z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        circle {
                            r: "1.5",
                            cx: "10",
                            cy: "15.5",
                        }
                    }
                    g {
                        path {
                            d: "M4,11V8h7.29C11.1,7.37,11,6.7,11,6H4.43c0.9-0.77,3.28-1.08,6.65-0.98c0.1-0.7,0.3-1.37,0.59-1.99C2.97,2.67,2,5.02,2,7 v9.5C2,18.43,3.57,20,5.5,20L4,21v1h12v-1l-1.5-1c1.93,0,3.5-1.57,3.5-3.5V13c-1.91,0-3.63-0.76-4.89-2H4z M16,16.5 c0,0.83-0.67,1.5-1.5,1.5h-9C4.67,18,4,17.33,4,16.5V13h12V16.5z",
                        }
                    }
                    g {
                        path {
                            d: "M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,9h-1V8h1V9z M18.5,7h-1V3h1V7z",
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
                    d: "M18,3.01V2L4,3.54V9H2c0,3.36,2.07,6.23,5,7.41V18h6v-1.59c2.93-1.19,5-4.06,5-7.41H9V7h9V6H9V3.99L18,3.01z M7,4.21L8,4.1 V6H7V4.21z M7,7h1v2H7V7z M5,4.43l1-0.11V6H5V4.43z M5,7h1v2H5V7z M16.33,10.5c-0.48,2.02-1.9,3.72-3.89,4.52L11.5,15.4v1.01v0.09 h-3v-0.09V15.4l-0.94-0.38c-1.99-0.81-3.41-2.5-3.89-4.52H16.33z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M19.66,14c-0.66,1.92-2.24,3.54-4.4,4.39L14,18.89V20h-4v-1.11l-1.27-0.5c-2.16-0.85-3.74-2.47-4.4-4.39H19.66 M22,2 L4,3.99V12H2c0,3.69,2.47,6.86,6,8.25V22h8v-1.75c3.53-1.39,6-4.56,6-8.25H10.5V8H22V6.5H10.5V4.78L22,3.51V2L22,2z M8,6.5V5.06 l1-0.11V6.5H8L8,6.5z M5.5,6.5V5.34l1-0.11V6.5H5.5L5.5,6.5z M8,12V8h1v4H8L8,12z M5.5,12V8h1v4H5.5L5.5,12z",
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
                    x: "0",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M10.75,5.87l1.19,1.19L13,6l-3-3L7,6l1.06,1.06l1.19-1.19l0,11.13h1.5l0-4.99l0-0.01c1.02,1.39,2.35,2.43,3.39,3.09 L15.23,14c-1.6-0.96-4.48-3.18-4.48-6.3L10.75,5.87z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M13,21h-2V6.83L9.41,8.41L8,7l4-4l4,4l-1.41,1.41L13,6.83V9c0,4.27,4.03,7.13,6,8.27l-1.46,1.46 c-1.91-1.16-3.44-2.53-4.54-4.02L13,21z",
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
                    fill: "none",
                    x: "0",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M9.25,5.87L8.06,7.06L7,6l3-3l3,3l-1.06,1.06l-1.19-1.19l0,11.13h-1.5l0-4.99l0-0.01c-1.02,1.39-2.35,2.43-3.39,3.09 L4.77,14c1.6-0.96,4.48-3.18,4.48-6.3L9.25,5.87z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M11,21h2V6.83l1.59,1.59L16,7l-4-4L8,7l1.41,1.41L11,6.83V9c0,4.27-4.03,7.13-6,8.27l1.46,1.46 C8.37,17.56,9.9,16.19,11,14.7L11,21z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H5.17l-.59.59-.58.58V4h16v12zm-9.5-2H18v-2h-5.5zm3.86-5.87c.2-.2.2-.51 0-.71l-1.77-1.77c-.2-.2-.51-.2-.71 0L6 11.53V14h2.47l5.89-5.87z",
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
                    fill: "none",
                    height: "20",
                    x: "0",
                    width: "20",
                }
            }
            g {
                g {
                    rect {
                        x: "4",
                        y: "4",
                        height: "12",
                        width: "1.5",
                    }
                    rect {
                        height: "6.5",
                        width: "1.5",
                        y: "4",
                        x: "14.5",
                    }
                    rect {
                        width: "1.5",
                        x: "9.25",
                        height: "3",
                        y: "8.5",
                    }
                    rect {
                        height: "3",
                        width: "1.5",
                        y: "4",
                        x: "9.25",
                    }
                    rect {
                        width: "1.5",
                        height: "3",
                        y: "13",
                        x: "9.25",
                    }
                    polygon {
                        points: "18,13.56 16.94,12.5 15.25,14.19 13.56,12.5 12.5,13.56 14.19,15.25 12.5,16.94 13.56,18 15.25,16.31 16.94,18 18,16.94 16.31,15.25",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    rect {
                        y: "4",
                        x: "18",
                        width: "2",
                        height: "9",
                    }
                    rect {
                        x: "4",
                        width: "2",
                        y: "4",
                        height: "16",
                    }
                    rect {
                        width: "2",
                        x: "11",
                        height: "4",
                        y: "4",
                    }
                    rect {
                        height: "4",
                        width: "2",
                        y: "10",
                        x: "11",
                    }
                    rect {
                        x: "11",
                        y: "16",
                        height: "4",
                        width: "2",
                    }
                    polygon {
                        points: "22.5,16.41 21.09,15 19,17.09 16.91,15 15.5,16.41 17.59,18.5 15.5,20.59 16.91,22 19,19.91 21.09,22 22.5,20.59 20.41,18.5",
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
                d: "M16 6v8h3v8h2V2c-2.76 0-5 2.24-5 4zm-5 3H9V2H7v7H5V2H3v7c0 2.21 1.79 4 4 4v9h2v-9c2.21 0 4-1.79 4-4V2h-2v7z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M8.1 13.34l2.83-2.83L3.91 3.5c-1.56 1.56-1.56 4.09 0 5.66l4.19 4.18zm6.78-1.81c1.53.71 3.68.21 5.27-1.38 1.91-1.91 2.28-4.65.81-6.12-1.46-1.46-4.2-1.1-6.12.81-1.59 1.59-2.09 3.74-1.38 5.27L3.7 19.87l1.41 1.41L12 14.41l6.88 6.88 1.41-1.41L13.41 13l1.47-1.47z",
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
                    x: "0",
                }
            }
            g {
                path {
                    d: "M13.25,3c-2.37,0-4.33,1.73-4.69,4L4.87,7l1.19-1.19L5,4.75l-3,3l3,3l1.06-1.06L4.87,8.5l3.69,0 c0.74,0,1.36-0.54,1.48-1.25c0.24-1.56,1.59-2.75,3.21-2.75l0,0c1.79,0,3.25,1.46,3.25,3.25c0,1.62-1.18,2.96-2.73,3.21 c-0.72,0.11-1.27,0.67-1.27,1.4l0,4.64H14v-4.56l0,0c2.27-0.36,4-2.32,4-4.69C18,5.13,15.87,3,13.25,3",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M10.08,8c0.48-2.84,2.94-5,5.92-5c3.31,0,6,2.69,6,6c0,2.97-2.16,5.44-5,5.92L17,21h-2l0-6.09c0-0.98,0.71-1.8,1.67-1.97 C18.56,12.63,20,10.98,20,9c0-2.21-1.79-4-4-4c-1.98,0-3.63,1.44-3.94,3.33C11.89,9.29,11.07,10,10.09,10l-4.26,0l1.59,1.59L6,13 L2,9l4-4l1.41,1.41L5.83,8L10.08,8z",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                    x: "0",
                }
            }
            g {
                path {
                    d: "M6.75,3c2.37,0,4.33,1.73,4.69,4l3.69,0l-1.19-1.19L15,4.75l3,3l-3,3l-1.06-1.06l1.19-1.19l-3.69,0 c-0.74,0-1.36-0.54-1.48-1.25C9.72,5.69,8.37,4.5,6.75,4.5l0,0C4.96,4.5,3.5,5.96,3.5,7.75c0,1.62,1.18,2.96,2.73,3.21 c0.72,0.11,1.27,0.67,1.27,1.4l0,4.64H6v-4.56l0,0c-2.27-0.36-4-2.32-4-4.69C2,5.13,4.13,3,6.75,3",
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
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M13.92,8C13.44,5.16,10.97,3,8,3C4.69,3,2,5.69,2,9c0,2.97,2.16,5.44,5,5.92L7,21h2l0-6.09c0-0.98-0.71-1.8-1.67-1.97 C5.44,12.63,4,10.98,4,9c0-2.21,1.79-4,4-4c1.98,0,3.63,1.44,3.94,3.33C12.11,9.29,12.93,10,13.91,10l4.26,0l-1.59,1.59L18,13l4-4 l-4-4l-1.41,1.41L18.17,8L13.92,8z",
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
                    fill: "none",
                    height: "20",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M15.25,12.13V6c0-1.65-1.35-3-3-3s-3,1.35-3,3v8c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5V7.87C7.26,7.55,8,6.62,8,5.5 C8,4.12,6.88,3,5.5,3C5.33,3,5.16,3.02,5,3.05C4.67,3.12,4.37,3.25,4.1,3.43C3.44,3.88,3,4.64,3,5.5c0,0.86,0.44,1.62,1.1,2.07 C4.37,7.75,4.67,7.88,5,7.95C4.91,7.93,4.83,7.9,4.75,7.87V14c0,1.65,1.35,3,3,3s3-1.35,3-3V6c0-0.83,0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5v6.13c0.08-0.03,0.16-0.06,0.25-0.08c-0.33,0.07-0.63,0.2-0.89,0.38c-0.67,0.45-1.1,1.21-1.1,2.07 c0,0.86,0.44,1.62,1.1,2.07c0.27,0.18,0.57,0.31,0.89,0.38c0.16,0.03,0.33,0.05,0.5,0.05c1.38,0,2.5-1.12,2.5-2.5 C17,13.38,16.26,12.45,15.25,12.13z M4.5,5.5c0-0.55,0.45-1,1-1s1,0.45,1,1s-0.45,1-1,1S4.5,6.05,4.5,5.5z M14.5,15.5 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S15.05,15.5,14.5,15.5z",
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
            }
            g {
                g {
                    path {
                        d: "M19,15.18V7c0-2.21-1.79-4-4-4s-4,1.79-4,4v10c0,1.1-0.9,2-2,2s-2-0.9-2-2V8.82C8.16,8.4,9,7.3,9,6c0-1.66-1.34-3-3-3 S3,4.34,3,6c0,1.3,0.84,2.4,2,2.82V17c0,2.21,1.79,4,4,4s4-1.79,4-4V7c0-1.1,0.9-2,2-2s2,0.9,2,2v8.18c-1.16,0.41-2,1.51-2,2.82 c0,1.66,1.34,3,3,3s3-1.34,3-3C21,16.7,20.16,15.6,19,15.18z M6,7C5.45,7,5,6.55,5,6s0.45-1,1-1s1,0.45,1,1S6.55,7,6,7z M18,19 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S18.55,19,18,19z",
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
                    x: "0",
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C17,6.13,13.87,3,10,3z M10,16c-3.31,0-6-2.69-6-6s2.69-6,6-6 s6,2.69,6,6S13.31,16,10,16z",
                    }
                    path {
                        d: "M11.39,8.15c-0.23-0.43-0.76-0.6-1.22-0.44L7.5,8.67v1.95h1V9.37l0.92-0.33L8.8,12.16l-2.19-0.45l-0.2,0.98l3.17,0.65 L10,11.21l1,0.99V15h1v-3.21l-1.39-1.38l0.21-1.19c0.81,1.03,2.08,1.4,2.67,1.4v-1C13.16,9.62,12.09,9.46,11.39,8.15z",
                    }
                    circle {
                        r: "1",
                        cx: "11",
                        cy: "6",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M12,20c-4.42,0-8-3.58-8-8 s3.58-8,8-8s8,3.58,8,8S16.42,20,12,20z",
                    }
                    path {
                        d: "M13.54,8.97c-0.23-0.47-0.76-0.71-1.26-0.53L9,9.65V12h1v-1.65l1.54-0.57l-0.96,4.89L7.8,14.1l-0.2,0.98l3.76,0.77 l0.52-2.64L13,14.42V18h1v-3.97l-1.32-1.44l0.41-2.35C13.99,11.46,15.3,12,16,12v-1C15.59,11,14.37,10.67,13.54,8.97z",
                    }
                    circle {
                        r: "1",
                        cy: "7",
                        cx: "13.5",
                    }
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
                    height: "20",
                    x: "0",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M10,2L3.5,4.5v4.77c0,4.04,2.77,7.81,6.5,8.73c3.73-0.92,6.5-4.69,6.5-8.73V4.5L10,2z M15,9.27c0,3.26-2.13,6.27-5,7.17 c-2.87-0.9-5-3.91-5-7.17V5.53l5-1.92l5,1.92V9.27z M10,6c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4s4-1.79,4-4C14,7.79,12.21,6,10,6z M11.15,11.85L9.5,10.21V8h1v1.79l1.35,1.35L11.15,11.85z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M12,2L4,5v6.09c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91V5L12,2z M18,11.09c0,4-2.55,7.7-6,8.83 c-3.45-1.13-6-4.82-6-8.83v-4.7l6-2.25l6,2.25V11.09z M12,7c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S14.76,7,12,7z M13.65,14.35 l-2.15-2.15V9h1v2.79l1.85,1.85L13.65,14.35z",
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
                fill: "none",
                width: "20",
                height: "20",
            }
            path {
                d: "M9.5,11.5V2L3,11.5H9.5z M8,10H5.84L8,6.85V10z M17,11.5C17,5.5,12.03,1,10.5,1c0,0,0.8,2.5,0.8,5.5s-0.8,5-0.8,5H17z M12.68,4.35c1.15,1.34,2.33,3.32,2.7,5.65h-2.92c0.18-0.92,0.34-2.12,0.34-3.5C12.8,5.74,12.75,5.02,12.68,4.35z M17.5,17.5H18 l0,1.5h-0.5c-0.86,0-1.71-0.2-2.5-0.6c-1.58,0.8-3.43,0.8-5,0c-1.58,0.8-3.42,0.8-5,0C4.21,18.8,3.36,19,2.5,19H2v-1.5h0.5 c0.87,0,1.74-0.33,2.5-1c1.53,1.33,3.47,1.33,5,0c1.53,1.33,3.48,1.33,5,0C15.76,17.17,16.63,17.5,17.5,17.5z M16.08,15.55 c-0.41-0.27-0.78-0.64-1.08-1.05c-0.61,0.84-1.5,1.5-2.5,1.5s-1.89-0.66-2.5-1.5C9.39,15.34,8.5,16,7.5,16S5.61,15.34,5,14.5 c-0.3,0.41-0.67,0.78-1.08,1.05C2.94,14.83,2.24,13.76,2,12.5h16C17.76,13.76,17.06,14.83,16.08,15.55z",
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
                height: "24",
                width: "24",
            }
            path {
                d: "M11,13.5V2L3,13.5H11z M9,11.5H6.83L9,8.38V11.5z M21,13.5C21,6.5,14.5,1,12.5,1c0,0,1,3,1,6.5s-1,6-1,6H21z M15.38,5.24 c1.42,1.52,2.88,3.72,3.41,6.26h-3.68c0.21-1.1,0.39-2.46,0.39-4C15.5,6.71,15.45,5.95,15.38,5.24z M22,15H2 c0.31,1.53,1.16,2.84,2.33,3.73C4.98,18.46,5.55,18.01,6,17.5C6.73,18.34,7.8,19,9,19s2.27-0.66,3-1.5c0.73,0.84,1.8,1.5,3,1.5 s2.26-0.66,3-1.5c0.45,0.51,1.02,0.96,1.67,1.23C20.84,17.84,21.69,16.53,22,15z M22,23v-2h-1c-1.04,0-2.08-0.35-3-1 c-1.83,1.3-4.17,1.3-6,0c-1.83,1.3-4.17,1.3-6,0c-0.91,0.65-1.96,1-3,1H2l0,2h1c1.03,0,2.05-0.25,3-0.75c1.89,1,4.11,1,6,0 c1.89,1,4.11,1,6,0h0c0.95,0.5,1.97,0.75,3,0.75H22z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM8.57 6H6v2.58c1.42 0 2.57-1.16 2.57-2.58zM12 6h-1.71c0 2.36-1.92 4.29-4.29 4.29V12c3.32 0 6-2.69 6-6zm2.14 5.86l-3 3.87L9 13.15 6 17h12z",
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
                    width: "20",
                    fill: "none",
                    x: "0",
                }
            }
            g {
                path {
                    d: "M4.56,7.5H6.5V9H2V4.5h1.5v1.94l4-4c0.59-0.59,1.54-0.59,2.12,0L16.18,9h-2.12l-5.5-5.5L4.56,7.5z M15.44,12.5H13.5V11H18 v4.5h-1.5v-1.94l-4,4c-0.59,0.59-1.54,0.59-2.12,0L3.82,11h2.12l5.5,5.5L15.44,12.5z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M4,7.59l5-5c0.78-0.78,2.05-0.78,2.83,0L20.24,11h-2.83L10.4,4L5.41,9H8v2H2V5h2V7.59z M20,19h2v-6h-6v2h2.59l-4.99,5 l-7.01-7H3.76l8.41,8.41c0.78,0.78,2.05,0.78,2.83,0l5-5V19z",
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
                d: "M21.05,17.56L3.08,18.5L3,17l17.98-0.94L21.05,17.56z M21,19.48H3v1.5h18V19.48z M23,13V4c0-1.1-0.9-2-2-2H3 C1.9,2,1,2.9,1,4v9c0,1.1,0.9,2,2,2h18C22.1,15,23,14.1,23,13z M21,13H3V4h18V13z M20,6c-1.68,0-3.04,0.98-3.21,2.23 C16.15,7.5,14.06,5.5,10.25,5.5c-4.67,0-6.75,3-6.75,3s2.08,3,6.75,3c3.81,0,5.9-2,6.54-2.73C16.96,10.02,18.32,11,20,11V6z",
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
                    x: "0",
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M10.75,8H15l2-2.25L15,3.5h-4.25V2h-1.5v1.5H4V8h5.25v1.5H5l-2,2.25L5,14h4.25v4h1.5v-4H16V9.5h-5.25V8z M5.5,6.5V5h8.83 l0.67,0.75L14.33,6.5H5.5z M14.5,11v1.5H5.67l-0.67-0.75L5.67,11H14.5z",
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
                path {
                    d: "M13,10h5l3-3l-3-3h-5V2h-2v2H4v6h7v2H6l-3,3l3,3h5v4h2v-4h7v-6h-7V10z M6,6h11.17l1,1l-1,1H6V6z M18,16H6.83l-1-1l1-1H18 V16z",
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
                fill: "none",
                width: "20",
                height: "20",
            }
            path {
                d: "M18.5,14.5c0,0.55-0.45,1-1,1l-2.22-2.19C16.87,12.96,18,12.17,18,11c0-0.5-6.5-6.5-6.5-6.5H9V6h1.91l0.84,0.78L9,8.5L1.5,8 L0,11l4.32,1.27l-3.53,1.91C-0.58,14.92-0.06,17,1.5,17h5c1.93,0,3.5-1.57,3.5-3.5h3.33l2.05,2H12.5V17h5c1.38,0,2.5-1.12,2.5-2.5 H18.5z M6.5,15.5h-5l4.87-2.63L8.5,13.5C8.5,14.6,7.6,15.5,6.5,15.5z M13,12H8.7l-6.56-1.93L2.4,9.56L9.35,10l3.52-2.16 c0,0,3.38,3.22,3.38,3.32C16.25,11.16,16.07,12,13,12z",
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
                d: "M22,17c0,0.55-0.45,1-1,1h-0.17l-2.2-2.2C20.58,15.37,22,14.4,22,13c0-1-8-8-8-8h-3v2h2.25l0.8,0.72L11,10L2,9l-2,4 l4.54,1.36l-3.49,1.88C-0.77,17.22-0.07,20,2,20h6c2.21,0,4-1.79,4-4h4l2,2h-3v2h6c1.66,0,3-1.34,3-3H22z M8,18H2l5.25-2.83L10,16 C10,17.1,9.11,18,8,18z M17,14h-6.7l-7.45-2.23l0.31-0.62L11.6,12l3.93-2.94c0,0,3.77,3.44,4.27,4.14C19.8,13.2,18.7,14,17,14z",
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
                    height: "20",
                    x: "0",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M7.38,12c0,0.82,0.68,1.5,1.5,1.5h2.25c0.82,0,1.5-0.68,1.5-1.5V8c0-0.82-0.68-1.5-1.5-1.5H8.88c-0.82,0-1.5,0.68-1.5,1.5 V12z M11.12,12H8.88V8h2.25V12z M1.5,12h3v-1.25H3c-0.83,0-1.5-0.67-1.5-1.5V8c0-0.83,0.67-1.5,1.5-1.5h3V8H3v1.25h1.5 c0.83,0,1.5,0.67,1.5,1.5V12c0,0.83-0.67,1.5-1.5,1.5h-3V12z M14,12h3v-1.25h-1.5c-0.83,0-1.5-0.67-1.5-1.5V8 c0-0.83,0.67-1.5,1.5-1.5h3V8h-3v1.25H17c0.83,0,1.5,0.67,1.5,1.5V12c0,0.83-0.67,1.5-1.5,1.5h-3V12z",
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
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M13.5,7h-3c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h3c1.1,0,2-0.9,2-2V9C15.5,7.9,14.6,7,13.5,7z M13.5,15h-3V9h3V15z M1,15h4 v-2H3c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h4v2H3v2h2c1.1,0,2,0.9,2,2v2c0,1.1-0.9,2-2,2H1V15z M17,15h4v-2h-2c-1.1,0-2-0.9-2-2V9 c0-1.1,0.9-2,2-2h4v2h-4v2h2c1.1,0,2,0.9,2,2v2c0,1.1-0.9,2-2,2h-4V15z",
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
                width: "20",
                fill: "none",
                height: "20",
            }
            g {
                path {
                    d: "M4.96,11c0,0,0.31-0.42,0.31-1.15c0-0.77-0.77-2.19-0.77-2.79C4.5,6.74,4.54,6.42,4.81,6h0.98 C5.53,6.42,5.48,6.74,5.48,7.06c0,0.59,0.77,2.02,0.77,2.79c0,0.73-0.31,1.15-0.31,1.15H4.96z M8.32,11c0,0,0.31-0.42,0.31-1.15 c0-0.77-0.77-2.19-0.77-2.79C7.86,6.74,7.9,6.42,8.16,6H7.18C6.92,6.42,6.88,6.74,6.88,7.06c0,0.59,0.77,2.02,0.77,2.79 c0,0.73-0.31,1.15-0.31,1.15H8.32z M10.69,11c0,0,0.31-0.42,0.31-1.15c0-0.77-0.77-2.19-0.77-2.79c0-0.31,0.04-0.64,0.31-1.06H9.56 C9.29,6.42,9.25,6.74,9.25,7.06c0,0.59,0.77,2.02,0.77,2.79c0,0.73-0.31,1.15-0.31,1.15H10.69z M15.5,2c-1.16,0-2.11,0.88-2.24,2 l-0.94,8.4H3.5c-0.28,0-0.53,0.24-0.5,0.52C3.25,15.77,5.59,18,8.4,18c2.78,0,5.07-2.18,5.36-4.99l0.99-8.84 c0.04-0.38,0.36-0.67,0.75-0.67c0.41,0,0.75,0.34,0.75,0.75S16.2,5.3,16.2,5.3l1.48,0.19c0,0,0.07-0.47,0.07-1.24 C17.75,3.01,16.74,2,15.5,2z M8.4,16.5c-1.63,0-3.07-1.08-3.65-2.6h7.28C11.46,15.44,10.04,16.5,8.4,16.5z",
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
                d: "M6.4,7C6.06,7.55,6,7.97,6,8.38C6,9.15,7,11,7,12c0,0.95-0.4,1.5-0.4,1.5H5.1c0,0,0.4-0.55,0.4-1.5c0-1-1-2.85-1-3.62 C4.5,7.97,4.56,7.55,4.9,7H6.4z M11.4,7C11.06,7.55,11,7.97,11,8.38C11,9.15,12,11,12,12c0,0.95-0.4,1.5-0.4,1.5h1.5 c0,0,0.4-0.55,0.4-1.5c0-1-1-2.85-1-3.62c0-0.41,0.06-0.83,0.4-1.38H11.4z M8.15,7c-0.34,0.55-0.4,0.97-0.4,1.38 c0,0.77,1,2.63,1,3.62c0,0.95-0.4,1.5-0.4,1.5h1.5c0,0,0.4-0.55,0.4-1.5c0-1-1-2.85-1-3.62c0-0.41,0.06-0.83,0.4-1.38H8.15z M18.6,2 c-1.54,0-2.81,1.16-2.98,2.65L14.53,15H4.01c-0.6,0-1.09,0.53-1,1.13C3.53,19.46,6.39,22,9.75,22c3.48,0,6.34-2.73,6.71-6.23 l1.15-10.87C17.66,4.39,18.08,4,18.6,4c0.55,0,1,0.45,1,1c0,0.3-0.1,1.25-0.1,1.25l1.97,0.25c0,0,0.13-1.06,0.13-1.5 C21.6,3.35,20.25,2,18.6,2z M9.75,20c-1.94,0-3.67-1.23-4.43-3h8.78h0.01C13.39,18.78,11.69,20,9.75,20z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                    x: "0",
                }
            }
            g {
                path {
                    d: "M6,4.5L3,6V3L6,4.5z M15,3v3l3-1.5L15,3z M9,2v3l3-1.5L9,2z M18,8.25c0,0,0,5.87,0,7.25c0,1.27-3.05,2.35-7,2.5v-3.5H9V18 c-3.95-0.15-7-1.23-7-2.5c0-0.99,0-7.25,0-7.25C2,7.01,5.58,6,10,6S18,7.01,18,8.25z M4.4,8.3C5.51,8.65,7.42,9,10,9 s4.49-0.35,5.6-0.7c0-0.21-2.38-0.8-5.6-0.8S4.4,8.09,4.4,8.3z M16.5,9.56c-1.45,0.57-3.82,0.94-6.5,0.94 c-2.68,0-5.05-0.37-6.5-0.94v5.7c0.43,0.35,1.82,0.88,4,1.13V13h5v3.39c2.18-0.24,3.57-0.78,4-1.13V9.56z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M7,5L3,7V3L7,5z M18,3v4l4-2L18,3z M11,2v4l4-2L11,2z M13,18h-2l0,4c-5.05-0.15-9-1.44-9-3v-9c0-1.66,4.48-3,10-3 s10,1.34,10,3v9c0,1.56-3.95,2.85-9,3L13,18z M5,10.04C6.38,10.53,8.77,11,12,11s5.62-0.47,7-0.96C19,9.86,16.22,9,12,9 S5,9.86,5,10.04z M20,11.8c-1.82,0.73-4.73,1.2-8,1.2s-6.18-0.47-8-1.2v6.78c0.61,0.41,2.36,1.01,5,1.28V16h6v3.86 c2.64-0.27,4.39-0.87,5-1.28V11.8z",
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
                d: "M18.36 9l.6 3H5.04l.6-3h12.72M20 4H4v2h16V4zm0 3H4l-1 5v2h1v6h10v-6h4v6h2v-6h1v-2l-1-5zM6 18v-4h6v4H6z",
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
                    fill: "none",
                    x: "0",
                    width: "20",
                    height: "20",
                }
            }
            g {
                polygon {
                    points: "9.25,17 10.75,17 10.75,5.87 11.94,7.06 13,6 10,3 7,6 8.06,7.06 9.25,5.87",
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                polygon {
                    points: "11,6.83 9.41,8.41 8,7 12,3 16,7 14.59,8.41 13,6.83 13,21 11,21",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M12.56 14.33c-.34.27-.56.7-.56 1.17V21h7c1.1 0 2-.9 2-2v-5.98c-.94-.33-1.95-.52-3-.52-2.03 0-3.93.7-5.44 1.83z",
            }
            circle {
                cy: "6",
                r: "5",
                cx: "18",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M17.8 2.8C16 2.09 13.86 2 12 2s-4 .09-5.8.8C3.53 3.84 2 6.05 2 8.86V22h20V8.86c0-2.81-1.53-5.02-4.2-6.06zM9.17 20l1.5-1.5h2.66l1.5 1.5H9.17zm-2.16-6V9h10v5h-10zm9.49 2c0 .55-.45 1-1 1s-1-.45-1-1 .45-1 1-1 1 .45 1 1zm-8-1c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zM20 20h-3.5v-.38l-1.15-1.16c1.49-.17 2.65-1.42 2.65-2.96V9c0-2.63-3-3-6-3s-6 .37-6 3v6.5c0 1.54 1.16 2.79 2.65 2.96L7.5 19.62V20H4V8.86c0-2 1.01-3.45 2.93-4.2C8.41 4.08 10.32 4 12 4s3.59.08 5.07.66c1.92.75 2.93 2.2 2.93 4.2V20z",
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
                    x: "0",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    circle {
                        cx: "10",
                        cy: "8.25",
                        r: "1.25",
                    }
                    path {
                        d: "M16.5,4c-1.38,0-2.5,1.12-2.5,2.5v0.13L10,3L6.01,6.63V6.5C6.01,5.12,4.89,4,3.5,4C2.12,4,1,5.12,1,6.5V17h8v-3 c0-0.55,0.45-1,1-1s1,0.45,1,1v3h8V6.5C19,5.12,17.88,4,16.5,4z M16.5,5.5c0.55,0,1,0.45,1,1v1h-2.01v-1 C15.49,5.95,15.94,5.5,16.5,5.5z M3.5,5.5c0.55,0,1,0.45,1,1v1H2.5v-1C2.5,5.95,2.95,5.5,3.5,5.5z M2.5,15.5V9h2v6.5H2.5z M12.5,15.5V14c0-1.38-1.12-2.5-2.5-2.5S7.5,12.62,7.5,14v1.5H6.01V8.66L10,5.03l3.99,3.63v6.84H12.5z M15.5,15.5V9h2v6.5H15.5z",
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
            }
            g {
                g {
                    path {
                        d: "M20,4c-1.66,0-3,1.34-3,3v0.29L12,3L7,7.29V7c0-1.66-1.34-3-3-3S1,5.34,1,7v14h10v-5c0-0.55,0.45-1,1-1s1,0.45,1,1v5h10V7 C23,5.34,21.66,4,20,4z M20,6c0.55,0,1,0.45,1,1v1h-2V7C19,6.45,19.45,6,20,6z M4,6c0.55,0,1,0.45,1,1v1H3V7C3,6.45,3.45,6,4,6z M3,19v-9h2v9H3z M17,19h-2v-3c0-1.65-1.35-3-3-3c-1.65,0-3,1.35-3,3v3H7V9.92l5-4.29l5,4.29V19z M19,19v-9h2v9H19z",
                    }
                    circle {
                        r: "1.5",
                        cx: "12",
                        cy: "10",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M17.5,6.56L16.44,5.5l-1.49,1.49L15,6l-3-3H8L5,6l0.05,0.99L3.56,5.5L2.5,6.56l2.66,2.66L5.5,16h9l0.34-6.78L17.5,6.56z M8.62,4.5h2.76l2.09,2.09L13.42,7.5H6.58L6.53,6.59L8.62,4.5z M6.93,14.5L6.65,9h6.7l-0.28,5.5H6.93z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M7.79,18l-0.51-7h9.46l-0.51,7H7.79z M9.83,5h4.33l2.8,2.73L16.87,9H7.12L7.03,7.73L9.83,5z M22,7.46l-1.41-1.41L19,7.63 l0.03-0.56L14.98,3H9.02L4.97,7.07L5,7.57L3.41,6.01L2,7.44l3.23,3.11L5.93,20h12.14l0.7-9.44L22,7.46z",
                        }
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        circle {
                            r: "1.5",
                            cy: "15.5",
                            cx: "6.5",
                        }
                    }
                    g {
                        circle {
                            cy: "15.5",
                            cx: "15.5",
                            r: "1.5",
                        }
                    }
                    g {
                        path {
                            d: "M18,13v5H4v-5H18c-1.91,0-3.63-0.76-4.89-2H4.81l1.04-3h5.44C11.1,7.37,11,6.7,11,6s0.1-1.37,0.29-2H8v2H5.5 C4.84,6,4.29,6.42,4.08,7.01L2,13v8c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1h12v1c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-8 l-0.09-0.27C19.3,12.9,18.66,13,18,13z",
                        }
                    }
                    g {
                        path {
                            d: "M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,9h-1V8h1V9z M18.5,7h-1V3h1V7z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                    x: "0",
                }
            }
            g {
                path {
                    d: "M17.5,7.01C17.5,8.11,16.61,9,15.52,9H15V7.16c1.18-0.56,2-1.75,2-3.14V4l-1.5,0.01c0,0.99-0.73,1.8-1.68,1.95L10,1 L6.18,5.97C5.23,5.82,4.5,5.01,4.5,4.02H3C3,5.41,3.82,6.6,5,7.16V9H4.48C3.39,9,2.5,8.11,2.5,7.01H1c0,1.76,1.31,3.2,3,3.44V18h5 v-3c0-0.55,0.45-1,1-1s1,0.45,1,1v3h5v-7.55c1.69-0.24,3-1.68,3-3.44V7L17.5,7.01z M10,3.46L11.95,6H8.05L10,3.46z M6.5,7.5h7V9h-7 V7.5z M14.5,16.5h-2V15c0-1.38-1.12-2.5-2.5-2.5S7.5,13.62,7.5,15v1.5h-2v-6h9V16.5z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M21,9.02c0,1.09-0.89,1.98-1.98,1.98H18V8.86c1.72-0.44,3-1.99,3-3.84V5l-2,0.02C19,6.11,18.11,7,17.02,7H16.5L12,1L7.5,7 H6.98C5.89,7,5,6.11,5,5.02H3c0,1.86,1.28,3.4,3,3.84V11H4.98C3.89,11,3,10.11,3,9.02H1c0,1.86,1.28,3.4,3,3.84V22h7v-4 c0-0.55,0.45-1,1-1s1,0.45,1,1v4h7v-9.14c1.72-0.44,3-1.99,3-3.84V9L21,9.02z M12,4.33L14,7h-4L12,4.33z M8,9h8v2H8V9z M18,20h-3 v-2c0-1.65-1.35-3-3-3c-1.65,0-3,1.35-3,3v2H6v-7h12V20z",
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
                path {
                    d: "M16.5 9v2H15l-3-8V1h-1.5v2h-1V1H8v2l-3 8H3.5V9H2v9h7v-4h2v4h7V9h-1.5zm-4.41-1.5.75 2H7.16l.75-2h4.18zM8.48 6l.56-1.5h1.92l.56 1.5H8.48zm8.02 10.5h-4v-4h-5v4h-4v-4h2.54L6.6 11h6.8l.56 1.5h2.54v4z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M20 11v2h-2L15 3V1h-2v2h-2.03V1h-2v2.12L6 13H4v-2H2v11h9v-5h2v5h9V11h-2zm-4.69 0H8.69l.6-2h5.42l.6 2zm-1.2-4H9.89l.6-2h3.02l.6 2zM20 20h-5v-5H9v5H4v-5h3.49l.6-2h7.82l.6 2H20v5z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M14 6l-4.22 5.63 1.25 1.67L14 9.33 19 16h-8.46l-4.01-5.37L1 18h22L14 6zM5 16l1.52-2.03L8.04 16H5z",
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
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    circle {
                        cy: "5.5",
                        r: ".75",
                        cx: "12.5",
                    }
                    path {
                        d: "M9,2v4h1.5V3.5h7V8c0,1.93-1.57,3.5-3.5,3.5c-0.74,0-1.43-0.24-2-0.64v1.71c0.61,0.27,1.29,0.42,2,0.42c2.76,0,5-2.24,5-5 V2H9z",
                    }
                    circle {
                        r: ".75",
                        cy: "5.5",
                        cx: "15.5",
                    }
                    path {
                        d: "M1,13c0,0.69,0.14,1.35,0.39,1.95c0.51,1.2,1.46,2.15,2.66,2.66C4.65,17.86,5.31,18,6,18s1.35-0.14,1.95-0.39 c1.2-0.51,2.15-1.46,2.66-2.66C10.86,14.35,11,13.69,11,13V7H1V13z M2.5,8.5h7V13c0,1.93-1.57,3.5-3.5,3.5 c-1.93,0-3.5-1.57-3.5-3.5V8.5z",
                    }
                    circle {
                        r: ".75",
                        cx: "4.5",
                        cy: "10.5",
                    }
                    circle {
                        r: ".75",
                        cx: "7.5",
                        cy: "10.5",
                    }
                    path {
                        d: "M6,14.51c1.11,0,2-0.67,2-1.51H4C4,13.83,4.89,14.51,6,14.51z",
                    }
                    path {
                        d: "M14,8c-1.11,0-2,0.67-2,1.51h4C16,8.67,15.11,8,14,8z",
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
                    circle {
                        cy: "6.5",
                        cx: "19",
                        r: "1",
                    }
                    circle {
                        cx: "15",
                        r: "1",
                        cy: "6.5",
                    }
                    path {
                        d: "M16.99,9c-1.38,0-2.5,0.84-2.5,1.88h5C19.49,9.84,18.37,9,16.99,9z",
                    }
                    path {
                        d: "M1,16c0,3.31,2.69,6,6,6s6-2.69,6-6V9H1V16z M3,11h8v5c0,2.21-1.79,4-4,4s-4-1.79-4-4V11z",
                    }
                    path {
                        d: "M11,2v5.5h2V4h8v5c0,2.21-1.79,4-4,4c-0.95,0-1.81-0.35-2.5-0.9v2.35C15.26,14.8,16.11,15,17,15c3.31,0,6-2.69,6-6V2H11z",
                    }
                    circle {
                        cy: "13.5",
                        r: "1",
                        cx: "5",
                    }
                    circle {
                        cx: "9",
                        cy: "13.5",
                        r: "1",
                    }
                    path {
                        d: "M7,17.88c1.38,0,2.5-0.84,2.5-1.88h-5C4.5,17.04,5.62,17.88,7,17.88z",
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
                    width: "20",
                    fill: "none",
                    x: "0",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M14.75,8c-1.24,0-2.25-1.01-2.25-2.25c0-1.24,1.01-2.25,2.25-2.25S17,4.51,17,5.75C17,6.99,15.99,8,14.75,8z M14.75,6.5 C14.34,6.5,14,6.16,14,5.75c0-0.21,0.08-0.39,0.22-0.53C14.52,4.92,16,4.5,16,4.5s-0.42,1.48-0.72,1.78 C15.14,6.42,14.96,6.5,14.75,6.5z M3.5,15.5V14L5,15.5v-2.12l-1.5-1.5v-1.63l1.5,1.5V9.62l-1.5-1.5V6.5L5,8V5.88L3.62,4.5h4.26 L6.5,5.88V8L8,6.5v1.63l-1.5,1.5v2.12l1.5-1.5v1.63l-1.5,1.5v2.12L8,14v1.5H3.5z M15.5,10h1l0-0.93c1.19-0.63,2-1.88,2-3.32 C18.5,3.68,16.82,2,14.75,2S11,3.68,11,5.75c0,1.44,0.81,2.69,2,3.32L13,10h1v4.75c0,0.41-0.34,0.75-0.75,0.75 c-0.41,0-0.75-0.34-0.75-0.75v-1.5c0-1.24-1.01-2.25-2.25-2.25c-0.26,0-0.52,0.05-0.75,0.13l0-4.1l0-2.52C9.5,3.67,8.83,3,8,3H3.5 C2.67,3,2,3.67,2,4.5v11C2,16.33,2.67,17,3.5,17H8c0.83,0,1.5-0.67,1.5-1.5v-2.25c0-0.41,0.34-0.75,0.75-0.75S11,12.84,11,13.25 v1.5c0,1.24,1.01,2.25,2.25,2.25c1.24,0,2.25-1.01,2.25-2.25V10z",
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M19,8c-0.55,0-1-0.45-1-1c0-0.28,0.11-0.53,0.29-0.71c0.4-0.4,2.46-1.04,2.46-1.04s-0.64,2.06-1.04,2.46 C19.53,7.89,19.28,8,19,8z M20,13v5c0,1.65-1.35,3-3,3s-3-1.35-3-3v-2c0-0.55-0.45-1-1-1s-1,0.45-1,1v3c0,1.1-0.9,2-2,2H4 c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2h6c1.1,0,2,0.9,2,2v8.17c0.31-0.11,0.65-0.17,1-0.17c1.65,0,3,1.35,3,3v2c0,0.55,0.45,1,1,1 s1-0.45,1-1v-5h-1l0-1.42c-1.77-0.77-3-2.53-3-4.58c0-2.76,2.24-5,5-5s5,2.24,5,5c0,2.05-1.23,3.81-3,4.58L21,13H20z M22,7 c0-1.66-1.34-3-3-3s-3,1.34-3,3s1.34,3,3,3S22,8.66,22,7z M10,7L8,9V6.17L9.17,5H4.83L6,6.17V9L4,7v2.17l2,2V14l-2-2v2.17l2,2V19 l-2-2v2h6v-2l-2,2v-2.83l2-2V12l-2,2v-2.83l2-2V7z",
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
                d: "M20 10h-3V8.86c1.72-.45 3-2 3-3.86h-3V4c0-.55-.45-1-1-1H8c-.55 0-1 .45-1 1v1H4c0 1.86 1.28 3.41 3 3.86V10H4c0 1.86 1.28 3.41 3 3.86V15H4c0 1.86 1.28 3.41 3 3.86V20c0 .55.45 1 1 1h8c.55 0 1-.45 1-1v-1.14c1.72-.45 3-2 3-3.86h-3v-1.14c1.72-.45 3-2 3-3.86zm-5 9H9V5h6v14zm-3-1c.83 0 1.5-.67 1.5-1.5S12.83 15 12 15s-1.5.67-1.5 1.5.67 1.5 1.5 1.5zm0-4.5c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5-1.5.67-1.5 1.5.67 1.5 1.5 1.5zM12 9c.83 0 1.5-.67 1.5-1.5S12.83 6 12 6s-1.5.67-1.5 1.5S11.17 9 12 9z",
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
            circle {
                cy: "14.5",
                cx: "8.5",
                r: "1.5",
            }
            circle {
                cx: "15.5",
                r: "1.5",
                cy: "14.5",
            }
            path {
                d: "M12 2c-4 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h2l2-2h4l2 2h2v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-4-4-8-4zm0 2c3.51 0 4.96.48 5.57 1H6.43c.61-.52 2.06-1 5.57-1zM6 7h5v3H6V7zm12 8.5c0 .83-.67 1.5-1.5 1.5h-9c-.83 0-1.5-.67-1.5-1.5V12h12v3.5zm0-5.5h-5V7h5v3z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M13 5l.75-1.5H17V2H7v1.5h4.75L11 5c-3.13.09-6 .73-6 3.5V17c0 1.5 1.11 2.73 2.55 2.95L6 21.5v.5h2l2-2h4l2 2h2v-.5l-1.55-1.55h-.01.01C17.89 19.73 19 18.5 19 17V8.5c0-2.77-2.87-3.41-6-3.5zm-1.97 2h1.94c2.75.08 3.62.58 3.9 1H7.13c.28-.42 1.15-.92 3.9-1zm-.18 10.95H7.74C7.3 17.84 7 17.45 7 17v-1h3.89c-.24.27-.39.61-.39 1 0 .36.13.69.35.95zM17 17c0 .45-.3.84-.74.95h-3.11c.22-.26.35-.59.35-.95 0-.39-.15-.73-.39-1H17v1zm0-3H7v-4h10v4z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M16.49 15.5v-1.75L14 16.25l2.49 2.5V17H22v-1.5h-5.51zm3.02 4.25H14v1.5h5.51V23L22 20.5 19.51 18v1.75zM9.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM5.75 8.9L3 23h2.1l1.75-8L9 17v6h2v-7.55L8.95 13.4l.6-3C10.85 12 12.8 13 15 13v-2c-1.85 0-3.45-1-4.35-2.45l-.95-1.6C9.35 6.35 8.7 6 8 6c-.25 0-.5.05-.75.15L2 8.3V13h2V9.65l1.75-.75",
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
                d: "M16 18H6V8h3v4.77L15.98 6 18 8.03 11.15 15H16v3z",
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
                    fill: "none",
                    x: "0",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M12,16h1.5V9c0-0.83-0.67-1.5-1.5-1.5l-6.13,0l1.19-1.19L6,5.25l-3,3l3,3l1.06-1.06L5.87,9L12,9V16z",
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
            }
            g {
                path {
                    d: "M6.83,11l1.59,1.59L7,14l-4-4l4-4l1.41,1.41L6.83,9L15,9c1.1,0,2,0.9,2,2v9h-2v-9L6.83,11z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                    x: "0",
                }
            }
            g {
                path {
                    d: "M8,16H6.5l0-7c0-0.83,0.67-1.5,1.5-1.5l6.13,0l-1.19-1.19L14,5.25l3,3l-3,3l-1.06-1.06L14.13,9L8,9V16z",
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
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M17.17,11l-1.59,1.59L17,14l4-4l-4-4l-1.41,1.41L17.17,9L9,9c-1.1,0-2,0.9-2,2v9h2v-9L17.17,11z",
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
                    width: "20",
                    fill: "none",
                    x: "0",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M5.5,10.5C5.5,11.33,6.17,12,7,12l6,0v5h1.5v-5c0-0.83-0.67-1.5-1.5-1.5l-6,0l0-4.63l1.19,1.19L9.25,6l-3-3l-3,3l1.06,1.06 L5.5,5.87L5.5,10.5z",
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
            }
            g {
                path {
                    d: "M6,6.83L4.41,8.41L3,7l4-4l4,4L9.59,8.41L8,6.83V13h8c1.1,0,2,0.9,2,2v6h-2v-6H8c-1.1,0-2-0.9-2-2V6.83z",
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
                    height: "20",
                    fill: "none",
                    x: "0",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M14.5,10.5c0,0.83-0.67,1.5-1.5,1.5l-6,0v5H5.5v-5c0-0.83,0.67-1.5,1.5-1.5l6,0l0-4.63l-1.19,1.19L10.75,6l3-3l3,3 l-1.06,1.06L14.5,5.87L14.5,10.5z",
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
            }
            g {
                path {
                    d: "M18,6.83l1.59,1.59L21,7l-4-4l-4,4l1.41,1.41L16,6.83V13H8c-1.1,0-2,0.9-2,2v6h2v-6h8c1.1,0,2-0.9,2-2V6.83z",
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
                    height: "20",
                    width: "20",
                    x: "0",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M12.5,10.56V16H11v-5.44l-4-4l0,1.69H5.5L5.5,4l4.25,0v1.5l-1.69,0l4,4C12.34,9.78,12.5,10.16,12.5,10.56z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M11.66,6V4H6v5.66h2V7.41l5,5V20h2v-7.58c0-0.53-0.21-1.04-0.59-1.41l-5-5H11.66z",
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
                    x: "0",
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M7.5,10.56V16H9v-5.44l4-4l0,1.69h1.5l0-4.25l-4.25,0v1.5l1.69,0l-4,4C7.66,9.78,7.5,10.16,7.5,10.56z",
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
                    fill: "none",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M12.34,6V4H18v5.66h-2V7.41l-5,5V20H9v-7.58c0-0.53,0.21-1.04,0.59-1.41l5-5H12.34z",
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
                    d: "M14.5,9c-0.16,0-0.31,0.02-0.45,0.05L13,8h1.5V6.5l-2,1L11,6H9.01v1h1.58l1,1H9.5L7,9L6,8H3v1h2.5C4.12,9,3,10.12,3,11.5 C3,12.88,4.12,14,5.5,14c1.23,0,2.24-0.88,2.45-2.05L9,13h1.5l2.03-4.06l0.52,0.52C12.42,9.92,12,10.66,12,11.5 c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5C17,10.12,15.88,9,14.5,9z M5.5,13C4.67,13,4,12.33,4,11.5S4.67,10,5.5,10 S7,10.67,7,11.5S6.33,13,5.5,13z M14.5,13c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S15.33,13,14.5,13z",
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
                    y: "0",
                    height: "24",
                    x: "0",
                    fill: "none",
                    fill_rule: "evenodd",
                    width: "24",
                }
                path {
                    d: "M4.17,11L4.17,11C4.12,11,4.06,11,4,11H4.17 M13.41,5H9v2h3.59l2,2H11l-4,2L5,9H0v2h4c-2.21,0-4,1.79-4,4 c0,2.21,1.79,4,4,4c2.21,0,4-1.79,4-4l2,2h3l3.49-6.1l1.01,1.01C16.59,12.64,16,13.75,16,15c0,2.21,1.79,4,4,4c2.21,0,4-1.79,4-4 c0-2.21-1.79-4-4-4c-0.18,0-0.36,0.03-0.53,0.05L17.41,9H20V6l-3.72,1.86L13.41,5L13.41,5z M20,17c-1.1,0-2-0.9-2-2 c0-1.1,0.9-2,2-2c1.1,0,2,0.9,2,2C22,16.1,21.1,17,20,17L20,17z M4,17c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2c1.1,0,2,0.9,2,2 C6,16.1,5.1,17,4,17L4,17z",
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
                    x: "0",
                }
            }
            g {
                path {
                    d: "M5.5,7.5C5.5,5.01,7.51,3,10,3s4.5,2.01,4.5,4.5V16H13V7.5c0-1.66-1.34-3-3-3s-3,1.34-3,3l0,2.63l1.19-1.19L9.25,10l-3,3 l-3-3l1.06-1.06l1.19,1.19L5.5,7.5z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M18,9v12h-2V9c0-2.21-1.79-4-4-4S8,6.79,8,9v4.17l1.59-1.59L11,13l-4,4l-4-4l1.41-1.41L6,13.17V9c0-3.31,2.69-6,6-6 S18,5.69,18,9z",
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
                    x: "0",
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M14.5,7.5C14.5,5.01,12.49,3,10,3S5.5,5.01,5.5,7.5V16H7l0-8.5c0-1.66,1.34-3,3-3s3,1.34,3,3l0,2.63l-1.19-1.19L10.75,10 l3,3l3-3l-1.06-1.06l-1.19,1.19L14.5,7.5z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M6,9v12h2V9c0-2.21,1.79-4,4-4s4,1.79,4,4v4.17l-1.59-1.59L13,13l4,4l4-4l-1.41-1.41L18,13.17V9c0-3.31-2.69-6-6-6 S6,5.69,6,9z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M13.25,10.5C15.7,8.28,18,6.41,18,4.61C18,3.15,16.85,2,15.39,2c-0.82,0-1.62,0.39-2.14,0.99C12.74,2.39,11.94,2,11.11,2 C9.65,2,8.5,3.15,8.5,4.61C8.5,6.4,10.8,8.28,13.25,10.5z M11.11,3.5c0.36,0,0.76,0.18,1,0.46l1.14,1.33l1.14-1.33 c0.24-0.28,0.63-0.46,1-0.46c0.63,0,1.11,0.48,1.11,1.11c0,0.9-1.67,2.45-3.25,3.87C11.67,7.06,10,5.5,10,4.61 C10,3.98,10.48,3.5,11.11,3.5z",
                    }
                    path {
                        d: "M15,13h-1c0-0.9-0.57-1.7-1.41-2L7,9H1v9h5v-1.18l5.5,1.68l6.5-2V16C18,14.34,16.66,13,15,13z M4.5,16.36v0.14h-2v-6h2 V16.36z M11.5,16.93L6,15.25V10.5h0.74l5.34,1.91c0.25,0.09,0.42,0.33,0.42,0.59h-2.36l-1.87-0.7l-0.53,1.4l2,0.8H15 c0.62,0,1.16,0.38,1.39,0.93L11.5,16.93z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        g {
                            path {
                                d: "M16,13c3.09-2.81,6-5.44,6-7.7C22,3.45,20.55,2,18.7,2c-1.04,0-2.05,0.49-2.7,1.25C15.34,2.49,14.34,2,13.3,2 C11.45,2,10,3.45,10,5.3C10,7.56,12.91,10.19,16,13z M13.3,4c0.44,0,0.89,0.21,1.18,0.55L16,6.34l1.52-1.79 C17.81,4.21,18.26,4,18.7,4C19.44,4,20,4.56,20,5.3c0,1.12-2.04,3.17-4,4.99c-1.96-1.82-4-3.88-4-4.99C12,4.56,12.56,4,13.3,4z",
                            }
                            path {
                                d: "M19,16h-2c0-1.2-0.75-2.28-1.87-2.7L8.97,11H1v11h6v-1.44l7,1.94l8-2.5v-1C22,17.34,20.66,16,19,16z M3,20v-7h2v7H3z M13.97,20.41L7,18.48V13h1.61l5.82,2.17C14.77,15.3,15,15.63,15,16c0,0-1.99-0.05-2.3-0.15l-2.38-0.79l-0.63,1.9l2.38,0.79 c0.51,0.17,1.04,0.26,1.58,0.26H19c0.39,0,0.74,0.23,0.9,0.56L13.97,20.41z",
                            }
                        }
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
                    height: "20",
                    fill: "none",
                    width: "20",
                    x: "0",
                }
            }
            g {
                path {
                    d: "M13.5,17H18V6l-8-3L2,6v11h4.5v-6.5h7V17z M16.5,7.04v8.46H15V9H5v6.5H3.5V7.04L10,4.6L16.5,7.04z M9.25,17h-1.5v-1.5h1.5 V17z M10.75,14.5h-1.5V13h1.5V14.5z M12.25,17h-1.5v-1.5h1.5V17z",
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
            }
            g {
                path {
                    d: "M20,8.35V19h-2v-8H6v8H4V8.35l8-3.2L20,8.35z M22,21V7L12,3L2,7v14h6v-8h8v8H22z M11,19H9v2h2V19z M13,16h-2v2h2V16z M15,19h-2v2h2V19z",
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
                fill: "none",
                height: "24",
                width: "24",
            }
            path {
                d: "M6,3l0,6c0,2.97,2.16,5.43,5,5.91V19H8v2h8v-2h-3v-4.09c2.84-0.48,5-2.94,5-5.91V3H6z M12,13c-1.86,0-3.41-1.28-3.86-3h7.72 C15.41,11.72,13.86,13,12,13z M16,8H8l0-3h8L16,8z",
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
                    height: "20",
                    width: "20",
                    x: "0",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M14.53,9c0.03,0.24,0.07,0.48,0.07,0.74c0,1.88-1.55,4.21-4.6,6.94c-3.05-2.73-4.6-5.06-4.6-6.94C5.4,6.66,7.77,5,10,5 c0.33,0,0.67,0.05,1,0.12V4.09C10.67,4.03,10.34,4,10,4C7.06,4,4.4,6.25,4.4,9.74c0,2.32,1.87,5.08,5.6,8.26 c3.73-3.18,5.6-5.94,5.6-8.26c0-0.25-0.02-0.5-0.05-0.74H14.53z",
                    }
                    circle {
                        cx: "10",
                        cy: "9.5",
                        r: "1.5",
                    }
                    polygon {
                        points: "17.47,3.23 16.77,2.53 15,4.29 13.23,2.53 12.53,3.23 14.29,5 12.53,6.77 13.23,7.47 15,5.71 16.77,7.47 17.47,6.77 15.71,5",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18,11c0,0.07,0,0.13,0,0.2c0,2.34-1.95,5.44-6,9.14c-4.05-3.7-6-6.79-6-9.14C6,7.57,8.65,5,12,5c0.34,0,0.68,0.03,1,0.08 V3.06C12.67,3.02,12.34,3,12,3c-4.2,0-8,3.22-8,8.2c0,3.32,2.67,7.25,8,11.8c5.33-4.55,8-8.48,8-11.8c0-0.07,0-0.13,0-0.2H18z",
                    }
                    circle {
                        cy: "11",
                        r: "2",
                        cx: "12",
                    }
                    polygon {
                        points: "22.54,2.88 21.12,1.46 19,3.59 16.88,1.46 15.46,2.88 17.59,5 15.46,7.12 16.88,8.54 19,6.41 21.12,8.54 22.54,7.12 20.41,5",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                    x: "0",
                }
            }
            g {
                path {
                    d: "M5.44,6.5L3,4.06L4.06,3L6.5,5.44V4H8v4H4V6.5H5.44z M16,8V6.5h-1.44L17,4.06L15.94,3L13.5,5.44V4H12v4H16z M4.06,17 l2.44-2.44V16H8v-4H4v1.5h1.44L3,15.94L4.06,17z M13.5,14.56L15.94,17L17,15.94l-2.44-2.44H16V12h-4v4h1.5V14.56z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M9,9l0-6L7,3l0,2.59L3.91,2.5L2.5,3.91L5.59,7L3,7l0,2L9,9z M21,9V7l-2.59,0l3.09-3.09L20.09,2.5L17,5.59V3l-2,0l0,6L21,9z M3,15l0,2h2.59L2.5,20.09l1.41,1.41L7,18.41L7,21h2l0-6L3,15z M15,15l0,6h2v-2.59l3.09,3.09l1.41-1.41L18.41,17H21v-2L15,15z",
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
            path {
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M15 3l2.3 2.3-2.89 2.87 1.42 1.42L18.7 6.7 21 9V3h-6zM3 9l2.3-2.3 2.87 2.89 1.42-1.42L6.7 5.3 9 3H3v6zm6 12l-2.3-2.3 2.89-2.87-1.42-1.42L5.3 17.3 3 15v6h6zm12-6l-2.3 2.3-2.87-2.89-1.42 1.42 2.89 2.87L15 21h6v-6z",
            }
        }
    }
}

