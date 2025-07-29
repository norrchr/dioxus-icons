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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        width: "10",
                        x: "4",
                        height: "1",
                        y: "4",
                    }
                    path {
                        d: "M13,14h1v-3h1v-1l-1-4H4l-1,4v1h1v5h6v-5h3V14z M9,15H5v-4h4V15z",
                    }
                    polygon {
                        points: "16,15 16,13 15,13 15,15 13,15 13,16 15,16 15,18 16,18 16,16 18,16 18,15",
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
            }
            g {
                g {
                    path {
                        d: "M15,17h2v-3h1v-2l-1-5H2l-1,5v2h1v6h9v-6h4V17z M9,18H4v-4h5V18z",
                    }
                    rect {
                        height: "2",
                        width: "15",
                        y: "4",
                        x: "2",
                    }
                    polygon {
                        points: "20,18 20,15 18,15 18,18 15,18 15,20 18,20 18,23 20,23 20,20 23,20 23,18",
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
            g {
                g {
                    path {
                        d: "M13,6h-2v3H8v2h3v3h2v-3h3V9h-3V6z M12,2c4.2,0,8,3.22,8,8.2c0,3.32-2.67,7.25-8,11.8 c-5.33-4.55-8-8.48-8-11.8C4,5.22,7.8,2,12,2z",
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
                d: "M20 1v3h3v2h-3v3h-2V6h-3V4h3V1h2zm-8 12c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm2-9.75V7h3v3h2.92c.05.39.08.79.08 1.2 0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8C4 6.22 7.8 3 12 3c.68 0 1.35.08 2 .25z",
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
                    rect {
                        width: "1",
                        x: "4",
                        height: "12",
                        y: "4",
                    }
                    rect {
                        y: "4",
                        width: "1",
                        x: "15",
                        height: "7",
                    }
                    rect {
                        x: "9.5",
                        height: "2",
                        y: "4",
                        width: "1",
                    }
                    rect {
                        y: "14",
                        x: "9.5",
                        width: "1",
                        height: "2",
                    }
                    polygon {
                        points: "18,15 16,15 16,13 15,13 15,15 13,15 13,16 15,16 15,18 16,18 16,16 18,16",
                    }
                    rect {
                        x: "9.5",
                        height: "2",
                        width: "1",
                        y: "9",
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
                        height: "9",
                        width: "2",
                        y: "4",
                        x: "18",
                    }
                    rect {
                        width: "2",
                        x: "4",
                        y: "4",
                        height: "16",
                    }
                    rect {
                        x: "11",
                        width: "2",
                        y: "4",
                        height: "4",
                    }
                    rect {
                        height: "4",
                        width: "2",
                        x: "11",
                        y: "10",
                    }
                    rect {
                        width: "2",
                        y: "16",
                        x: "11",
                        height: "4",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M15.5,10c0.54,0,1.04,0.13,1.5,0.35V7c0-0.55-0.45-1-1-1h-4.47l-0.8-0.8l1.06-1.06l-0.53-0.53L8.61,6.27L9.14,6.8 l1.06-1.06l0.8,0.8V9c0,0.55-0.45,1-1,1H9.24C9.72,10.72,10,11.57,10,12.5c0,0.17-0.03,0.33-0.05,0.5h2.1 C12.29,11.31,13.74,10,15.5,10z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M19.5,12c0.93,0,1.78,0.28,2.5,0.76V8c0-1.1-0.9-2-2-2h-6.29l-1.06-1.06l1.41-1.41l-0.71-0.71L9.82,6.35l0.71,0.71 l1.41-1.41L13,6.71V9c0,1.1-0.9,2-2,2h-0.54c0.95,1.06,1.54,2.46,1.54,4c0,0.34-0.04,0.67-0.09,1h3.14 C15.3,13.75,17.19,12,19.5,12z",
                    }
                    path {
                        d: "M19.5,13c-1.93,0-3.5,1.57-3.5,3.5s1.57,3.5,3.5,3.5s3.5-1.57,3.5-3.5S21.43,13,19.5,13z M19.5,18 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S20.33,18,19.5,18z",
                    }
                    path {
                        d: "M4,9h5c0-1.1-0.9-2-2-2H4C3.45,7,3,7.45,3,8C3,8.55,3.45,9,4,9z",
                    }
                    path {
                        d: "M9.83,13.82l-0.18-0.47L10.58,13c-0.46-1.06-1.28-1.91-2.31-2.43l-0.4,0.89l-0.46-0.21l0.4-0.9C7.26,10.13,6.64,10,6,10 c-0.53,0-1.04,0.11-1.52,0.26l0.34,0.91l-0.47,0.18L4,10.42c-1.06,0.46-1.91,1.28-2.43,2.31l0.89,0.4l-0.21,0.46l-0.9-0.4 C1.13,13.74,1,14.36,1,15c0,0.53,0.11,1.04,0.26,1.52l0.91-0.34l0.18,0.47L1.42,17c0.46,1.06,1.28,1.91,2.31,2.43l0.4-0.89 l0.46,0.21l-0.4,0.9C4.74,19.87,5.36,20,6,20c0.53,0,1.04-0.11,1.52-0.26l-0.34-0.91l0.47-0.18L8,19.58 c1.06-0.46,1.91-1.28,2.43-2.31l-0.89-0.4l0.21-0.46l0.9,0.4C10.87,16.26,11,15.64,11,15c0-0.53-0.11-1.04-0.26-1.52L9.83,13.82z M7.15,17.77c-1.53,0.63-3.29-0.09-3.92-1.62c-0.63-1.53,0.09-3.29,1.62-3.92c1.53-0.63,3.29,0.09,3.92,1.62 C9.41,15.38,8.68,17.14,7.15,17.77z",
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
                d: "M10.75,4L2.5,16h12.75L17.5,4H10.75z M11.88,11.75c-1.04,0-1.88-0.84-1.88-1.88S10.84,8,11.88,8s1.88,0.84,1.88,1.88 S12.91,11.75,11.88,11.75z",
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
                d: "M13,4L2,20h17l3-16H13z M14.5,14c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5 C17,12.88,15.88,14,14.5,14z",
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
                fill: "none",
                width: "20",
            }
            path {
                d: "M9.25,14C9.25,10.41,6,7.5,2,7.5V6c3.56,0,6.64,1.96,8,4.76c0.72-1.49,1.96-2.87,3.68-4.08L11.5,4.5H17V10l-2.24-2.24 c-1.78,1.21-4.01,3.28-4.01,6.24h1.5v1.5h-4.5V14H9.25z",
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
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M18.21,9.21C15.93,10.78,13.45,13.3,13,17h2v2H9v-2h2c-0.5-4.5-4.37-8-9-8V7c4.39,0,8.22,2.55,10,6.3 c1.13-2.43,2.99-4.25,4.78-5.52L14,5h7v7L18.21,9.21z",
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
                    height: "20",
                    width: "20",
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
                    width: "24",
                    height: "24",
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
                d: "M8 9v1.5h2.25V15h1.5v-4.5H14V9H8zM7 9H2v6h1.5v-1.5h2V15H7V9zm-1.5 3h-2v-1.5h2V12zM22 9h-6.5v6H17v-4.5h1V14h1.5v-3.51h1V15H22V9z",
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
                    y: "0",
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M17,10c0-0.68-0.1-1.34-0.29-1.97c0.49-0.55,0.6-1.36,0.21-2.03c-0.39-0.67-1.15-0.98-1.87-0.83 c-0.91-0.95-2.08-1.64-3.4-1.96C11.42,2.51,10.77,2,10,2S8.58,2.51,8.35,3.21c-1.32,0.32-2.49,1.01-3.4,1.96 C4.22,5.02,3.46,5.33,3.07,6C2.68,6.67,2.8,7.48,3.29,8.03C3.1,8.66,3,9.32,3,10s0.1,1.34,0.29,1.97C2.8,12.52,2.68,13.33,3.07,14 c0.39,0.67,1.15,0.98,1.87,0.83c0.33,0.35,0.69,0.66,1.09,0.93L4.99,18h1.66l0.71-1.52c0.32,0.13,0.65,0.23,0.99,0.32 C8.58,17.49,9.23,18,10,18s1.42-0.51,1.65-1.21c0.34-0.08,0.67-0.19,0.99-0.32L13.36,18h1.66l-1.04-2.24 c0.4-0.27,0.76-0.59,1.09-0.93c0.72,0.15,1.48-0.16,1.87-0.83c0.39-0.67,0.27-1.48-0.21-2.03C16.9,11.34,17,10.68,17,10z M11.46,15.3c-0.31-0.48-0.85-0.8-1.46-0.8s-1.15,0.32-1.46,0.8c-0.19-0.05-0.37-0.11-0.55-0.18l1.06-2.28 C9.35,12.93,9.67,13,10,13s0.65-0.07,0.95-0.17l1.06,2.28C11.83,15.18,11.65,15.24,11.46,15.3z M15.32,11.38 c-0.57,0.03-1.11,0.34-1.42,0.87c-0.31,0.53-0.3,1.16-0.04,1.67c-0.17,0.16-0.35,0.32-0.53,0.46l-1.1-2.37 C12.7,11.47,13,10.78,13,10c0-1.66-1.34-3-3-3s-3,1.34-3,3c0,0.78,0.3,1.47,0.78,2.01l-1.1,2.37c-0.19-0.14-0.37-0.29-0.53-0.46 c0.26-0.51,0.26-1.14-0.04-1.67c-0.31-0.53-0.85-0.84-1.42-0.87C4.57,10.94,4.5,10.48,4.5,10s0.07-0.94,0.18-1.38 C5.25,8.59,5.8,8.28,6.1,7.75c0.31-0.53,0.3-1.16,0.04-1.67C6.8,5.44,7.62,4.96,8.54,4.7C8.85,5.18,9.39,5.5,10,5.5 s1.15-0.32,1.46-0.8c0.91,0.25,1.73,0.73,2.39,1.38c-0.26,0.51-0.26,1.14,0.04,1.67c0.31,0.53,0.85,0.84,1.42,0.87 c0.11,0.44,0.18,0.9,0.18,1.38S15.43,10.94,15.32,11.38z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M10.44,18.75c0.37-0.46,0.94-0.75,1.57-0.75s1.19,0.29,1.56,0.75c0.39-0.09,0.76-0.21,1.12-0.36l-1.42-3.18 c-0.39,0.15-0.82,0.23-1.26,0.23c-0.46,0-0.9-0.09-1.3-0.25l-1.43,3.19C9.65,18.54,10.03,18.67,10.44,18.75z M5.16,10 C5,10.59,4.91,11.21,4.91,11.85c0,0.75,0.12,1.47,0.33,2.15c0.63,0.05,1.22,0.4,1.56,0.99c0.33,0.57,0.35,1.23,0.11,1.79 c0.27,0.27,0.56,0.53,0.87,0.76l1.52-3.39l0,0c-0.47-0.58-0.75-1.32-0.75-2.13c0-1.89,1.55-3.41,3.46-3.41s3.46,1.53,3.46,3.41 c0,0.82-0.29,1.57-0.78,2.16l1.5,3.35c0.32-0.24,0.62-0.5,0.9-0.79c-0.22-0.55-0.2-1.2,0.12-1.75c0.33-0.57,0.9-0.92,1.52-0.99 c0.22-0.68,0.34-1.41,0.34-2.16c0-0.64-0.09-1.27-0.25-1.86c-0.64-0.04-1.26-0.39-1.6-1c-0.36-0.62-0.35-1.36-0.03-1.95 c-0.91-0.98-2.1-1.71-3.44-2.05C13.4,5.6,12.74,6,12.01,6s-1.39-0.41-1.74-1.01C8.93,5.33,7.74,6.04,6.83,7.02 C7.16,7.62,7.18,8.37,6.81,9C6.45,9.62,5.82,9.97,5.16,10z M3.86,9.58C3.08,8.98,2.84,7.88,3.35,7s1.58-1.23,2.49-0.85 c1.11-1.17,2.56-2.03,4.18-2.42C10.15,2.75,10.99,2,12.01,2s1.85,0.75,1.98,1.73c1.63,0.39,3.07,1.24,4.18,2.42 c0.91-0.38,1.99-0.03,2.49,0.85c0.51,0.88,0.27,1.98-0.51,2.58c0.23,0.77,0.35,1.58,0.35,2.42s-0.12,1.65-0.35,2.42 c0.78,0.6,1.02,1.7,0.51,2.58s-1.58,1.23-2.49,0.85c-0.4,0.43-0.85,0.81-1.34,1.15l1.34,3h-1.86l-0.97-2.17 c-0.43,0.18-0.88,0.33-1.34,0.44C13.86,21.25,13.02,22,12.01,22s-1.85-0.75-1.98-1.73C9.55,20.15,9.09,20,8.65,19.81L7.66,22H5.79 l1.36-3.03c-0.47-0.33-0.91-0.71-1.3-1.12c-0.92,0.38-1.99,0.03-2.5-0.85s-0.27-1.98,0.51-2.58C3.62,13.65,3.51,12.84,3.51,12 S3.62,10.35,3.86,9.58z",
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
                    height: "20",
                    y: "0",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M18,6h-6V2H8v4H2v12h16V6z M7.5,10c0.69,0,1.25,0.56,1.25,1.25S8.19,12.5,7.5,12.5s-1.25-0.56-1.25-1.25S6.81,10,7.5,10z M10,15H5v-0.48c0-0.5,0.3-0.95,0.76-1.16C6.3,13.13,6.88,13,7.5,13c0.62,0,1.2,0.13,1.74,0.36C9.7,13.56,10,14.01,10,14.52V15z M10.5,7.5h-1v-4h1V7.5z M15,14h-3v-1.5h3V14z M15,11.5h-3V10h3V11.5z",
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
                    y: "0",
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M22,7h-7V2H9v5H2v15h20V7z M9,12c0.83,0,1.5,0.67,1.5,1.5c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5 C7.5,12.67,8.17,12,9,12z M12,18H6v-0.43c0-0.6,0.36-1.15,0.92-1.39C7.56,15.9,8.26,15.75,9,15.75s1.44,0.15,2.08,0.43 c0.55,0.24,0.92,0.78,0.92,1.39V18z M13,9h-2V4h2V9z M18,16.5h-4V15h4V16.5z M18,13.5h-4V12h4V13.5z",
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
                    width: "20",
                    y: "0",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    g {
                        polygon {
                            points: "3.25,7.59 5.88,14 7.33,14 6.62,6.26",
                        }
                    }
                    g {
                        polygon {
                            points: "13.38,6.26 12.67,14 14.12,14 16.75,7.59",
                        }
                    }
                    g {
                        polygon {
                            points: "8.35,14 11.65,14 12.48,5 7.52,5",
                        }
                    }
                    g {
                        polygon {
                            points: "16.71,10.33 15.11,14.24 17.19,15.29 18.55,13.91",
                        }
                    }
                    g {
                        polygon {
                            points: "1.45,13.91 2.81,15.29 4.89,14.24 3.29,10.33",
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
                    height: "24",
                    fill: "none",
                    y: "0",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        polygon {
                            points: "16.36,7.58 15.5,16.99 17,16.99 20.16,9.1",
                        }
                    }
                    g {
                        polygon {
                            points: "3.84,9.1 7,16.99 8.5,16.99 7.64,7.58",
                        }
                    }
                    g {
                        polygon {
                            points: "10,16.99 14,16.99 15,6 9,6",
                        }
                    }
                    g {
                        polygon {
                            points: "20.32,12.75 18.51,17.25 20.46,18.21 22.52,16.99",
                        }
                    }
                    g {
                        polygon {
                            points: "1.48,16.99 3.54,18.21 5.49,17.25 3.68,12.75",
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
                d: "M3.01 1L3 17l9 6 8.99-6L21 1H3.01zM10 16l-5-5 1.41-1.42L10 13.17l7.59-7.59L19 7l-9 9z",
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
                }
            }
            g {
                g {
                    path {
                        d: "M9,13c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S10.1,13,9,13z M9,16c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S9.55,16,9,16z",
                    }
                    path {
                        d: "M9.24,12L7.64,5H4.01v1h2.84l1.17,5.14c-1.57,0.4-2.75,1.72-2.96,3.36H1v1h5.01v-0.51C6.01,13.34,7.35,12,9,12L9.24,12z",
                    }
                    path {
                        d: "M15.5,8h-0.68L13,3h-3v1h2.3l1.46,4h-4.4l0.23,1h3.45c-0.53,0.52-0.88,1.22-0.98,2h-2.01l0.23,1h1.79 c0.25,1.81,1.83,3.14,3.75,2.99c1.64-0.13,3.01-1.46,3.18-3.1C19.2,9.75,17.59,8,15.5,8z M15.5,14c-1.4,0-2.5-1.1-2.5-2.5 c0-0.94,0.5-1.73,1.24-2.16l1.03,2.83l0.94-0.34l-1.02-2.8C15.3,9.02,15.4,9,15.5,9c1.4,0,2.5,1.1,2.5,2.5S16.9,14,15.5,14z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M10,14h0.74L8.47,4H3v2h3.87l1.42,6.25c0,0-0.01,0-0.01,0C6.12,12.9,4.47,14.73,4.09,17H0v2h6v-1C6,15.79,7.79,14,10,14z",
                    }
                    path {
                        d: "M18.18,8l-1.83-5H11v2h3.96l1.1,3H10.4l0.46,2H15c-0.43,0.58-0.75,1.25-0.9,2h-2.79l0.46,2h2.33 c0.44,2.23,2.31,3.88,4.65,3.99c2.8,0.13,5.25-2.19,5.25-5c0-2.8-2.2-5-5-5H18.18z M19,16c-1.68,0-3-1.32-3-3 c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64l1.88-0.68l-0.97-2.67c0.03,0,0.06-0.01,0.09-0.01c1.68,0,3,1.32,3,3S20.68,16,19,16z",
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
                    height: "20",
                    y: "0",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M17.99,5.74C17.86,4.16,16.43,3,14.85,3H5.15C3.57,3,2.14,4.16,2.01,5.74C1.89,7.16,2.76,8.38,4,8.82V17h12l0-8.18 C17.24,8.38,18.11,7.16,17.99,5.74z M13.41,11L10,14.41L6.59,11c0,0,0,0,0,0L10,7.59c0,0,0,0,0,0L13.41,11 C13.41,11,13.41,11,13.41,11z",
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
                    y: "0",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M17.85,3L6.14,3C4.15,3,2.36,4.39,2.05,6.36C1.78,8.11,2.64,9.65,4,10.45V21h16V10.45c1.36-0.79,2.23-2.36,1.95-4.11 C21.63,4.38,19.83,3,17.85,3z M16.41,13L12,17.42L7.59,13L12,8.59L16.41,13z",
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
                    y: "0",
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        rect {
                            y: "16.5",
                            height: "1.5",
                            width: "11",
                            x: "2",
                        }
                    }
                    g {
                        polygon {
                            points: "13,13.5 9,13.5 9,12 6,12 6,13.5 2,13.5 2,15 13,15",
                        }
                    }
                    g {
                        path {
                            d: "M18,9.76V2h-5v7.76c0,1.27,0.67,2.44,1.75,3.09V18H18v-1.5h-1.75v-3.65C17.33,12.2,18,11.04,18,9.76z M16.5,3.5V7h-2V3.5 H16.5z",
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
                    fill: "none",
                    height: "24",
                    y: "0",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18,8h2V4h-2V8z M16,22H2v-2h14V22z M18,15.89l-0.4-0.42c-1.02-1.08-1.6-2.52-1.6-4V2h6v9.51c0,1.46-0.54,2.87-1.53,3.94 L20,15.97V20h2v2h-4V15.89z M7,16v-2h4v2h5v2H2v-2H7z",
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
                        path {
                            d: "M4,11V8h7.29c-0.77-2.6,0.21-4.61,0.37-4.97C2.97,2.67,2,5.02,2,7v9.5c0,0.95,0.38,1.81,1,2.44V22h3v-2h8v2h3v-3.06 c0.62-0.63,1-1.49,1-2.44V13c-1.91,0-3.63-0.76-4.89-2H4z M6.5,17C5.67,17,5,16.33,5,15.5S5.67,14,6.5,14S8,14.67,8,15.5 S7.33,17,6.5,17z M15,15.5c0,0.83-0.67,1.5-1.5,1.5S12,16.33,12,15.5s0.67-1.5,1.5-1.5S15,14.67,15,15.5z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M19,5c0,2.21-1.79,4-4,4s-4-1.79-4-4s1.79-4,4-4S19,2.79,19,5z M15.5,6h-1v1h1V6z M15.5,3h-1v2h1V3z M15,10.5 c-0.32,0-0.64-0.03-0.94-0.08c0.27,0.18,0.44,0.48,0.44,0.83c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1c0-0.51,0.39-0.93,0.88-0.99 C12.14,9.87,11.09,9.06,10.39,8H5.02l1-2.5h3.51C9.47,4.99,9.51,4.39,9.59,4H5L3,9l0,7h2v-1.5h10V16h2v-5.88 C16.38,10.37,15.71,10.5,15,10.5z M6.5,12.25c-0.55,0-1-0.45-1-1s0.45-1,1-1c0.55,0,1,0.45,1,1S7.05,12.25,6.5,12.25z",
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
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18,1c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,1,18,1z M18.5,7h-1V3h1V7z M18.5,8v1h-1V8H18.5z M17.91,13 c0.06,0.16,0.09,0.33,0.09,0.5c0,0.83-0.67,1.5-1.5,1.5S15,14.33,15,13.5c0-0.39,0.15-0.74,0.39-1.01 c-1.63-0.66-2.96-1.91-3.71-3.49H5.81l1.04-3H11c0-0.69,0.1-1.37,0.29-2H5.41L3,11v9h3v-2h12v2h3v-7.68 C19.95,12.83,18.84,13.01,17.91,13z M7.5,15C6.67,15,6,14.33,6,13.5S6.67,12,7.5,12S9,12.67,9,13.5S8.33,15,7.5,15z",
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
                    width: "20",
                    y: "0",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M6.5,8L5,12v5h1.5v-1h7v1H15v-5l-1.46-4H6.5z M7.75,14.25C7.34,14.25,7,13.91,7,13.5s0.34-0.75,0.75-0.75 S8.5,13.09,8.5,13.5S8.16,14.25,7.75,14.25z M12.25,14.25c-0.41,0-0.75-0.34-0.75-0.75s0.34-0.75,0.75-0.75S13,13.09,13,13.5 S12.66,14.25,12.25,14.25z M6.98,11l0.56-1.5h4.95l0.55,1.5H6.98z",
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
                    fill: "none",
                    y: "0",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M10.83,3C10.41,1.83,9.3,1,8,1C6.34,1,5,2.34,5,4c0,1.65,1.34,3,3,3c1.3,0,2.41-0.84,2.83-2H16v2h2V5h1V3H10.83z M8,5 C7.45,5,7,4.55,7,4s0.45-1,1-1s1,0.45,1,1S8.55,5,8,5z M17.11,9H6.89L5,14.69V22h2v-2h10v2h2v-7.31L17.11,9z M9,17.5 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S9.55,17.5,9,17.5z M15,17.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S15.55,17.5,15,17.5z M7.67,13l0.66-2h7.34l0.66,2H7.67z",
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
                    fill: "none",
                    y: "0",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M6.5,11h7v1H15V7l-1.46-4H6.5L5,7v5h1.5V11z M7.75,9.25C7.34,9.25,7,8.91,7,8.5s0.34-0.75,0.75-0.75S8.5,8.09,8.5,8.5 S8.16,9.25,7.75,9.25z M12.25,9.25c-0.41,0-0.75-0.34-0.75-0.75s0.34-0.75,0.75-0.75S13,8.09,13,8.5S12.66,9.25,12.25,9.25z M7.54,4.5h4.95L13.04,6H6.98L7.54,4.5z",
                    }
                    polygon {
                        points: "4,13 4,14.5 9.25,14.5 9.25,17 10.75,17 10.75,14.5 16,14.5 16,13",
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
                    height: "24",
                    y: "0",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M4,17.01V19h7v3h2v-3h7v-1.99H4z M7,14h10v2h2V8.69L17.11,3H6.89L5,8.69V16h2V14z M9,11.5c-0.55,0-1-0.45-1-1s0.45-1,1-1 s1,0.45,1,1S9.55,11.5,9,11.5z M15,11.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S15.55,11.5,15,11.5z M8.33,5h7.34l0.66,2H7.67 L8.33,5z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M17.5,7v2H16V2h-1.5v2h-2V2H11v2H9V2H7.5v2h-2V2H4v7H2.5V7H1v10h7v-4h4v4h7V7H17.5z M9,10H7.5V7H9V10z M12.5,10H11V7h1.5 V10z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M21,9v2h-2V3h-2v2h-2V3h-2v2h-2V3H9v2H7V3H5v8H3V9H1v12h9v-5h4v5h9V9H21z M11,12H9V9h2V12z M15,12h-2V9h2V12z",
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
                d: "M12 2l-5.5 9h11z",
            }
            circle {
                cx: "17.5",
                cy: "17.5",
                r: "4.5",
            }
            path {
                d: "M3 13.5h8v8H3z",
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
                    height: "20",
                    fill: "none",
                    y: "0",
                    width: "20",
                }
            }
            g {
                g {
                    polygon {
                        points: "2,18 13.49,13.93 6.07,6.51",
                    }
                    polygon {
                        points: "11.55,2.09 10.84,2.8 12.96,4.92 9.78,8.1 10.49,8.81 14.37,4.92",
                    }
                    polygon {
                        points: "9.07,3.15 8.36,3.86 9.78,5.27 8.36,6.69 9.07,7.39 11.19,5.27",
                    }
                    polygon {
                        points: "15.79,7.75 12.61,10.93 13.31,11.64 15.79,9.16 17.91,11.28 18.62,10.58",
                    }
                    polygon {
                        points: "19.32,5.63 17.2,3.5 11.19,9.51 11.9,10.22 17.2,4.92 18.62,6.33",
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
                    width: "24",
                    y: "0",
                }
            }
            g {
                g {
                    g {
                        polygon {
                            points: "2,22 16,17 7,8",
                        }
                        path {
                            d: "M14.53,12.53L21,6.05l1.48,1.48l1.06-1.06L21,3.93l-7.53,7.53L14.53,12.53z",
                        }
                        path {
                            d: "M10.94,6L9.47,7.47l1.06,1.06l2.54-2.54l-2.54-2.53L9.47,4.53L10.94,6z",
                        }
                        path {
                            d: "M18.97,9.97l-3.5,3.5l1.06,1.06L19,12.06l2.5,2.49l1.06-1.06L18.97,9.97z",
                        }
                        path {
                            d: "M15.97,4.97l-4.5,4.5l1.06,1.06L18.07,5l-3.53-3.53l-1.06,1.06L15.97,4.97z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M15,11V8l-4.25-2.55v-1.2h1.75v-1.5h-1.75V1h-1.5v1.75H7.5v1.5h1.75v1.2L5,8v3l-3,1v6h6v-4h4v4h6v-6L15,11z M10,11.25 c-0.69,0-1.25-0.56-1.25-1.25S9.31,8.75,10,8.75s1.25,0.56,1.25,1.25S10.69,11.25,10,11.25z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18,12.22V9l-5-2.5V5h2V3h-2V1h-2v2H9v2h2v1.5L6,9v3.22L2,14v8h8v-5h4v5h8v-8L18,12.22z M12,13.5 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S12.83,13.5,12,13.5z",
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
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M16,9h-4V3H8v6H4v8h12V9z M15,16h-2v-2h-1v2h-1.5v-2h-1v2H8v-2H7v2H5v-6h10V16z",
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
            }
            g {
                path {
                    d: "M15,11V1H9v10H3v12h18V11H15z M19,21h-2v-4h-2v4h-2v-4h-2v4H9v-4H7v4H5v-8h14V21z",
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
                cy: "17",
                cx: "12",
            }
            path {
                d: "M12 3C8.1 3 4.56 4.59 2 7.15l5 5c1.28-1.28 3.05-2.08 5-2.08s3.72.79 5 2.07l5-5C19.44 4.59 15.9 3 12 3z",
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
                fill: "none",
                height: "20",
                width: "20",
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
                height: "24",
                width: "24",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M12,2c0,0.86-0.62,3.05-0.96,4.23C10.9,6.69,10.48,7,10,7h0C9.52,7,9.1,6.69,8.96,6.23C8.62,5.05,8,2.86,8,2 c0-1.1,0.9-2,2-2S12,0.9,12,2z M10,8.25c-0.97,0-1.75,0.78-1.75,1.75s0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75S10.97,8.25,10,8.25z M13.12,4.3c2.01,1.1,3.38,3.24,3.38,5.7c0,3.59-2.91,6.5-6.5,6.5S3.5,13.59,3.5,10c0-2.46,1.37-4.6,3.38-5.7 C6.75,3.76,6.64,3.23,6.57,2.77C3.87,4.06,2,6.81,2,10c0,4.42,3.58,8,8,8s8-3.58,8-8c0-3.19-1.87-5.94-4.57-7.23 C13.36,3.23,13.25,3.76,13.12,4.3z M13.5,10c0,1.93-1.57,3.5-3.5,3.5S6.5,11.93,6.5,10c0-1.07,0.48-2.02,1.23-2.67 c-0.11-0.35-0.28-0.9-0.46-1.53C5.91,6.7,5,8.24,5,10c0,2.76,2.24,5,5,5s5-2.24,5-5c0-1.76-0.91-3.3-2.28-4.19 c-0.18,0.63-0.35,1.18-0.46,1.53C13.02,7.98,13.5,8.93,13.5,10z",
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
                    d: "M14.5,2.5c0,1.06-0.75,3.64-1.19,5.04C13.13,8.11,12.6,8.5,12,8.5h0c-0.6,0-1.13-0.39-1.31-0.96 C10.25,6.14,9.5,3.56,9.5,2.5C9.5,1.12,10.62,0,12,0S14.5,1.12,14.5,2.5z M12,10c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2 S13.1,10,12,10z M16.08,5.11c0.18-0.75,0.33-1.47,0.39-2.06C19.75,4.69,22,8.08,22,12c0,5.52-4.48,10-10,10S2,17.52,2,12 c0-3.92,2.25-7.31,5.53-8.95C7.6,3.64,7.74,4.37,7.92,5.11C5.58,6.51,4,9.07,4,12c0,4.42,3.58,8,8,8s8-3.58,8-8 C20,9.07,18.42,6.51,16.08,5.11z M18,12c0,3.31-2.69,6-6,6s-6-2.69-6-6c0-2,0.98-3.77,2.48-4.86c0.23,0.81,0.65,2.07,0.65,2.07 C8.43,9.93,8,10.92,8,12c0,2.21,1.79,4,4,4s4-1.79,4-4c0-1.08-0.43-2.07-1.13-2.79c0,0,0.41-1.22,0.65-2.07C17.02,8.23,18,10,18,12 z",
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
                    fill: "none",
                    y: "0",
                }
            }
            g {
                g {
                    rect {
                        x: "4",
                        height: "1.5",
                        y: "5.5",
                        width: "4",
                    }
                    path {
                        d: "M15.5,8.46V5h-4v1.5H14v1.41l-3,3.59H8V8H2v5h1.5c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5h3.2L15.5,8.46z M6,14 c-0.55,0-1-0.45-1-1h2C7,13.55,6.55,14,6,14z",
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
                    fill: "none",
                    y: "0",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M19,10.35V5h-5v2h3v2.65L13.52,14H10V9H2v7h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,10.35z M7,17c-0.55,0-1-0.45-1-1h2 C8,16.55,7.55,17,7,17z",
                        }
                        rect {
                            x: "5",
                            width: "5",
                            y: "6",
                            height: "2",
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
                d: "M17.34 1.13c-2.94-.55-5.63.75-7.12 2.92.01-.01.01-.02.02-.03C9.84 4 9.42 4 9 4c-4.42 0-8 .5-8 4v10c0 .88.39 1.67 1 2.22V23h3v-2h8v2h3v-2.78c.61-.55 1-1.34 1-2.22v-3.08c3.72-.54 6.5-3.98 5.92-7.97-.42-2.9-2.7-5.29-5.58-5.82zM4.5 19c-.83 0-1.5-.67-1.5-1.5S3.67 16 4.5 16s1.5.67 1.5 1.5S5.33 19 4.5 19zM3 13V8h6c0 1.96.81 3.73 2.11 5H3zm10.5 6c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm2.5-6c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm.5-9H15v5l3.62 2.16.75-1.23-2.87-1.68V4z",
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
                    path {
                        d: "M16.42,13.92l-4.17-4.17l1.33-1.33l-2-2l-1.33,1.33L6.08,3.58l-2.5,2.5l4.17,4.17L4,14v2h2l3.75-3.75l4.17,4.17 L16.42,13.92z M8.46,9.54L5,6.08L6.08,5c0,0,0,0,0,0l0.69,0.69L6.23,6.23L6.89,6.9l0.54-0.54l1.06,1.06L7.95,7.96l0.67,0.67 l0.54-0.54l0.38,0.38L8.46,9.54z M13.92,15l-3.46-3.46l1.08-1.08l0.4,0.4L11.4,11.4l0.67,0.67l0.54-0.54l1.06,1.06l-0.54,0.54 l0.67,0.67l0.54-0.54L15,13.92L13.92,15z",
                    }
                    rect {
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.0106 11.7111)",
                        width: "2.39",
                        height: "2.83",
                        y: "4.45",
                        x: "12.94",
                    }
                }
                g {
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
                        d: "M21.79,17.06l-5.55-5.55l1.57-1.57l-3.75-3.75l-1.57,1.57L6.94,2.21L2.21,6.94l5.55,5.55L3,17.25V21h3.75l4.76-4.76 l5.55,5.55l0,0v0L21.79,17.06z M9.18,11.07L5.04,6.94l1.9-1.9l1.27,1.27L7.02,7.5l1.41,1.41l1.19-1.19l1.45,1.45L9.18,11.07z M12.93,14.82l1.9-1.9l1.45,1.45l-1.19,1.19l1.41,1.41l1.19-1.19l1.27,1.27l-1.9,1.9L12.93,14.82z",
                    }
                    rect {
                        width: "3.59",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 1.302 14.5981)",
                        height: "5.3",
                        x: "16.48",
                        y: "3.08",
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
                    height: "20",
                    fill: "none",
                    width: "20",
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
                        polygon {
                            points: "6.69,7.5 8.94,3 4,3 2.2,7.5",
                        }
                    }
                    g {
                        polygon {
                            points: "13.3,7.5 17.8,7.5 16,3 10.87,3",
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
                    width: "24",
                    height: "24",
                    fill: "none",
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
                        polygon {
                            points: "16.46,8.25 21.62,8.25 19,3 13.84,3",
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
                        polygon {
                            points: "7.54,8.25 10.16,3 5,3 2.38,8.25",
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
                    height: "20",
                    width: "20",
                    y: "0",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M3,14.5h14c0-2.49-2.01-4.5-4.5-4.5c-2.03,0-3.72,1.35-4.28,3.19C7.86,12.92,7.45,12.73,7,12.61V8h3V6h7V5h-7V3H3v1h1v1H3 v1h1v1H3v1h1v5.26C3.58,13.59,3.23,14.01,3,14.5z M8.5,7H7V6h1.5V7z M8.5,4v1H7V4H8.5z M5,4h1v1H5V4z M5,6h1v1H5V6z M5,8h1v4.51 c-0.35,0.02-0.68,0.08-1,0.19V8z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                    y: "0",
                }
            }
            g {
                g {
                    polygon {
                        points: "2,19 4,21 20,21 22,19",
                    }
                    path {
                        d: "M3,18l16.97,0c0,0,0,0,0,0c0.29-3.26-2.28-6-5.48-6c-2.35,0-4.35,1.48-5.14,3.55C8.94,15.32,8.48,15.17,8,15.08V9h4V6.75 h9v-1.5h-9V3H3v1.5h1v0.75H3v1.5h1V7.5H3V9h1v7.39C3.56,16.85,3.22,17.39,3,18z M8,7.5V6.75h2V7.5H8z M10,5.25H8V4.5h2V5.25z M5.5,4.5h1v0.75h-1V4.5z M5.5,6.75h1V7.5h-1V6.75z M6.5,9v6.06c-0.35,0.06-0.68,0.17-1,0.3V9H6.5z",
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
                d: "M22.41 12 12 1.59 1.59 11.99 12 22.41 22.41 12zM14 14.5V12h-4v3H8v-5h6V7.5l3.5 3.5-3.5 3.5z",
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
                d: "M15.5 5.5c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zM5 12c-2.8 0-5 2.2-5 5s2.2 5 5 5 5-2.2 5-5-2.2-5-5-5zm0 8.5c-1.9 0-3.5-1.6-3.5-3.5s1.6-3.5 3.5-3.5 3.5 1.6 3.5 3.5-1.6 3.5-3.5 3.5zm5.8-10l2.4-2.4.8.8c1.3 1.3 3 2.1 5.1 2.1V9c-1.5 0-2.7-.6-3.6-1.5l-1.9-1.9c-.5-.4-1-.6-1.6-.6s-1.1.2-1.4.6L6.31 9.9 11 14v5h2v-6.2l-2.2-2.3zM19 12c-2.8 0-5 2.2-5 5s2.2 5 5 5 5-2.2 5-5-2.2-5-5-5zm0 8.5c-1.9 0-3.5-1.6-3.5-3.5s1.6-3.5 3.5-3.5 3.5 1.6 3.5 3.5-1.6 3.5-3.5 3.5z",
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
                d: "M20 21c-1.39 0-2.78-.47-4-1.32-2.44 1.71-5.56 1.71-8 0C6.78 20.53 5.39 21 4 21H2v2h2c1.38 0 2.74-.35 4-.99 2.52 1.29 5.48 1.29 8 0 1.26.65 2.62.99 4 .99h2v-2h-2zM3.95 19H4c1.6 0 3.02-.88 4-2 .98 1.12 2.4 2 4 2s3.02-.88 4-2c.98 1.12 2.4 2 4 2h.05l2.18-7.65-2.23-.73V4h-5V1H9v3H4v6.62l-2.23.73L3.95 19zM6 6h12v3.97L12 8 6 9.97V6z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M20,21c-1.39,0-2.78-0.47-4-1.32c-2.44,1.71-5.56,1.71-8,0C6.78,20.53,5.39,21,4,21H2v2h2c1.38,0,2.74-0.35,4-0.99 c2.52,1.29,5.48,1.29,8,0c1.26,0.65,2.62,0.99,4,0.99h2v-2H20z M3.95,19H4c1.6,0,3.02-0.88,4-2c0.98,1.12,2.4,2,4,2s3.02-0.88,4-2 c0.98,1.12,2.4,2,4,2h0.05l2.18-7.65L20,10.62V4h-5V1H9v3H4v6.62l-2.23,0.73L3.95,19z M6,6h12v3.97L12,8L6,9.97V6z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M4 16c0 .88.39 1.67 1 2.22V21h3v-2h8v2h3v-2.78c.61-.55 1-1.34 1-2.22V6c0-3.5-3.58-4-8-4s-8 .5-8 4v10zm3.5 1c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm9 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-6H6V6h12v5z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                    y: "0",
                }
            }
            g {
                path {
                    d: "M12,2C8,2,4,2.5,4,6v9.5c0,0.95,0.38,1.81,1,2.44V21h3v-2h8v2h3v-3.06 c0.62-0.63,1-1.49,1-2.44V6C20,2.5,16.42,2,12,2z M8.5,16C7.67,16,7,15.33,7,14.5S7.67,13,8.5,13s1.5,0.67,1.5,1.5S9.33,16,8.5,16z M15.5,16c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,16,15.5,16z M18,10H6V7h12V10z",
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
                d: "M18.58 5H5.43L3 12v9h3v-2h12v2h3v-9l-2.42-7zM6.5 16c-.83 0-1.5-.67-1.5-1.5S5.67 13 6.5 13s1.5.67 1.5 1.5S7.33 16 6.5 16zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 11l1.5-4.5h11L19 11H5z",
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
                    width: "24",
                    height: "24",
                    y: "0",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18.57,5H5.43L3,12v9h3v-2h12v2h3v-9L18.57,5z M7.5,16C6.67,16,6,15.33,6,14.5S6.67,13,7.5,13S9,13.67,9,14.5 S8.33,16,7.5,16z M16.5,16c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S17.33,16,16.5,16z M5.81,10l1.04-3h10.29 l1.04,3H5.81z",
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
                d: "M4 15.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V5c0-3.5-3.58-4-8-4s-8 .5-8 4v10.5zm8 1.5c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm6-7H6V5h12v5z",
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
                    width: "24",
                    fill: "none",
                    y: "0",
                    height: "24",
                }
            }
            g {
                path {
                    enable_background: "new",
                    d: "M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19L6,20v1h12v-1l-1.5-1c1.93,0,3.5-1.57,3.5-3.5 V6C20,2.5,16.42,2,12,2z M12,16c-0.83,0-1.5-0.67-1.5-1.5S11.17,13,12,13s1.5,0.67,1.5,1.5S12.83,16,12,16z M18,10H6V7h12V10z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 2c-4.42 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-3.58-4-8-4zM7.5 17c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm3.5-6H6V6h5v5zm5.5 6c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-6h-5V6h5v5z",
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
                    width: "24",
                    y: "0",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19L6,20v1h12v-1l-1.5-1c1.93,0,3.5-1.57,3.5-3.5V6C20,2.5,16.42,2,12,2z M8.5,16C7.67,16,7,15.33,7,14.5S7.67,13,8.5,13s1.5,0.67,1.5,1.5S9.33,16,8.5,16z M11,10H6V7h5V10z M15.5,16 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,16,15.5,16z M18,10h-5V7h5V10z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 2c-4.42 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h12v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-3.58-4-8-4zM7.5 17c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm3.5-6H6V6h5v5zm5.5 6c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-6h-5V6h5v5z",
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
                    fill: "none",
                    height: "24",
                    y: "0",
                }
            }
            g {
                path {
                    d: "M12,2C8,2,4,2.5,4,6v9.5C4,17.43,5.57,19,7.5,19L6,20v1h12v-1l-1.5-1c1.93,0,3.5-1.57,3.5-3.5V6C20,2.5,16.42,2,12,2z M8.5,16C7.67,16,7,15.33,7,14.5S7.67,13,8.5,13s1.5,0.67,1.5,1.5S9.33,16,8.5,16z M11,10H6V7h5V10z M15.5,16 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,16,15.5,16z M18,10h-5V7h5V10z",
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
                    width: "20",
                    y: "0",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M17,10l-6.25-2.68V5.5H10c-0.62,0-1-0.56-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1h1.5c0-1.56-1.43-2.78-3.05-2.44 C8.5,2.26,7.73,3.05,7.55,4.01c-0.25,1.33,0.55,2.5,1.7,2.86v0.45L3,10v3h3v5h8v-5h3V10z M15.5,11.5H14V11H6v0.5H4.5v-0.51L10,8.63 l5.5,2.36V11.5z",
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
                g {
                    path {
                        d: "M21,12l-8-3.56V6h-1c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1h2c0-1.84-1.66-3.3-3.56-2.95 C10.26,2.27,9.29,3.22,9.06,4.4C8.76,5.96,9.66,7.34,11,7.82v0.63l-8,3.56L3,16h4v6h10v-6h4V12z M19,14h-2v-1H7v1H5v-0.7l7-3.11 l7,3.11V14z",
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
                d: "M17.63 7H6.37C3.96 7 2 9.24 2 12s1.96 5 4.37 5h11.26c2.41 0 4.37-2.24 4.37-5s-1.96-5-4.37-5zM7.24 14.46l-2.57-2.57.7-.7 1.87 1.87 3.52-3.52.7.7-4.22 4.22z",
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
                d: "M18.11 1.77L19.78.1l2.12 2.12-1.67 1.67-2.12-2.12zm-1 1l2.12 2.12L13.12 11H11V8.89l6.11-6.12zm-1.98-.13L9.5 8.27v4.24h4.24l5.62-5.62c.41.99.64 2.1.64 3.32 0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8 0-4.98 3.8-8.2 8-8.2 1.09 0 2.16.22 3.13.63z",
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
                    height: "20",
                    width: "20",
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
                            d: "M11.04,11.5H7.5V7.96l5.32-5.32C11.96,2.24,11.01,2,10,2C6.41,2,3.5,4.91,3.5,8.5C3.5,12.84,10,18,10,18 s6.5-5.16,6.5-9.5c0-0.74-0.13-1.45-0.36-2.11L11.04,11.5z",
                        }
                    }
                    g {
                        rect {
                            transform: "matrix(0.7071 -0.7071 0.7071 0.7071 2.5347 12.1798)",
                            height: "2",
                            width: "1.5",
                            y: "2.03",
                            x: "15.22",
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
                path {
                    d: "M13.95,13H9V8.05l5.61-5.61C13.78,2.16,12.9,2,12,2c-4.2,0-8,3.22-8,8.2c0,3.32,2.67,7.25,8,11.8c5.33-4.55,8-8.48,8-11.8 c0-1.01-0.16-1.94-0.45-2.8L13.95,13z M11,11h2.12l6.16-6.16l-2.12-2.12L11,8.88V11z M19.29,0.59l-1.42,1.42l2.12,2.12l1.42-1.42 L19.29,0.59z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    rect {
                        y: "4",
                        x: "4",
                        width: "1",
                        height: "12",
                    }
                    rect {
                        y: "4",
                        height: "2",
                        width: "1",
                        x: "8.5",
                    }
                    rect {
                        height: "2",
                        y: "14",
                        x: "8.5",
                        width: "1",
                    }
                    rect {
                        height: "2",
                        width: "1",
                        y: "9",
                        x: "8.5",
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
                        x: "4",
                        width: "2",
                        y: "4",
                    }
                    rect {
                        width: "2",
                        height: "4",
                        y: "4",
                        x: "10",
                    }
                    rect {
                        x: "10",
                        width: "2",
                        y: "10",
                        height: "4",
                    }
                    rect {
                        height: "4",
                        x: "10",
                        y: "16",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M10,3c-2.75,0-5.5,4.93-5.5,8.56C4.5,14.56,6.96,17,10,17s5.5-2.44,5.5-5.44C15.5,7.93,12.75,3,10,3z M10.25,14.5 C8.37,14.5,7,12.92,7,10.75C7,10.34,7,10,7,10h1.5c0,0,0,0.5,0,0.75c0,1.12,0.54,2.25,1.75,2.25c0.41,0,0.75-0.01,0.75,0 c0,0,0,1.5,0,1.5S10.66,14.5,10.25,14.5z",
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
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,3C8.5,3,5,9.33,5,14c0,3.87,3.13,7,7,7s7-3.13,7-7C19,9.33,15.5,3,12,3z M13,18c-3,0-5-1.99-5-5c0-0.55,0-1,0-1h2 c0,0,0,1,0,1c0,2.92,2.42,3,3,3c0.55,0,1,0,1,0l0,2C14,18,13.55,18,13,18z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    polygon {
                        points: "5,9 10,9 10,4 5,4 5,5 9,5 9,8 4,8 4,13.5 7,13.5 7,12.5 5,12.5",
                    }
                    rect {
                        x: "14",
                        height: "1",
                        width: "2",
                        y: "11",
                    }
                    rect {
                        height: "1",
                        width: "2",
                        x: "14",
                        y: "14",
                    }
                    polygon {
                        points: "9,12 8,12 8,14 9,14 9,16 13,16 13,10 9,10",
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
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    rect {
                        x: "18",
                        width: "3",
                        height: "2",
                        y: "13",
                    }
                    polygon {
                        points: "12,12 12,14 10,14 10,18 12,18 12,20 17,20 17,12",
                    }
                    polygon {
                        points: "5,11 12,11 12,4 4,4 4,6 10,6 10,9 3,9 3,17 9,17 9,15 5,15",
                    }
                    rect {
                        y: "17",
                        height: "2",
                        width: "3",
                        x: "18",
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
                        d: "M15.5,7h-0.68L13,2h-3v1h2.3l1.46,4H8.75L8.38,6H10V5H6v1h1.32l1.46,4H7.95C7.7,8.19,6.12,6.86,4.2,7.01 c-1.64,0.13-3.01,1.46-3.18,3.1C0.8,12.25,2.41,14,4.5,14c1.79,0,3.21-1.29,3.45-3h4.1c0.25,1.81,1.83,3.14,3.75,2.99 c1.64-0.13,3.01-1.46,3.18-3.1C19.2,8.75,17.59,7,15.5,7z M6.95,11c-0.23,1.15-1.22,2-2.45,2C3.1,13,2,11.9,2,10.5S3.1,8,4.5,8 c1.23,0,2.23,0.85,2.45,2H4v1H6.95z M12.05,10H9.84L9.11,8h3.92C12.5,8.52,12.16,9.22,12.05,10z M15.5,13c-1.4,0-2.5-1.1-2.5-2.5 c0-0.94,0.5-1.73,1.24-2.16l1.03,2.83l0.94-0.34l-1.02-2.8C15.3,8.02,15.4,8,15.5,8c1.4,0,2.5,1.1,2.5,2.5S16.9,13,15.5,13z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M19,7h-0.82L16,1h-4v2h2.6l1.46,4h-4.81l-0.36-1H12V4H7v2h1.75l1.82,5H9.9C9.46,8.77,7.59,7.12,5.25,7.01 C2.45,6.87,0,9.2,0,12c0,2.8,2.2,5,5,5c2.46,0,4.45-1.69,4.9-4h4.2c0.44,2.23,2.31,3.88,4.65,3.99c2.8,0.13,5.25-2.19,5.25-5 C24,9.2,21.8,7,19,7z M7.82,13c-0.4,1.17-1.49,2-2.82,2c-1.68,0-3-1.32-3-3s1.32-3,3-3c1.33,0,2.42,0.83,2.82,2H5v2H7.82z M14.1,11h-1.4l-0.73-2H15C14.56,9.58,14.24,10.25,14.1,11z M19,15c-1.68,0-3-1.32-3-3c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64 l1.88-0.68l-0.97-2.67C18.94,9.01,18.97,9,19,9c1.68,0,3,1.32,3,3S20.68,15,19,15z",
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
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M16,7l-2-4H6L4,7v6h2v-1h8v1h2V7z M6.62,4h6.76l1.5,3H5.12L6.62,4z M6,10c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C7,9.55,6.55,10,6,10z M14,10c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C15,9.55,14.55,10,14,10z",
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
            }
            g {
                path {
                    d: "M18.58,1H5.43L3,8v9h3v-2h12v2h3V8L18.58,1z M6.5,12C5.67,12,5,11.33,5,10.5S5.67,9,6.5,9S8,9.67,8,10.5S7.33,12,6.5,12z M17.5,12c-0.83,0-1.5-0.67-1.5-1.5S16.67,9,17.5,9S19,9.67,19,10.5S18.33,12,17.5,12z M5,7l1.5-4.5h11L19,7H5z",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M15,4h-3v1h2v1.79L10.79,10H9V6H6C4.34,6,3,7.34,3,9v2h2c0,1.1,0.9,2,2,2s2-0.9,2-2h2.21L15,7.21V4z M7,12 c-0.55,0-1-0.45-1-1h2C8,11.55,7.55,12,7,12z",
                    }
                    rect {
                        width: "4",
                        y: "4",
                        height: "1",
                        x: "5",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M19,8.35V3h-5v2h3v2.65L13.52,12H10V7H6c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,8.35z M7,15 c-0.55,0-1-0.45-1-1h2C8,14.55,7.55,15,7,15z",
                    }
                    rect {
                        x: "5",
                        width: "5",
                        height: "2",
                        y: "4",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        polygon {
                            points: "9,17 6,17 11,19.5 11,17.5 14,17.5 9,15",
                        }
                    }
                    g {
                        path {
                            d: "M17,10.05V8l-4-5H1v9.5h2C3,13.88,4.12,15,5.5,15S8,13.88,8,12.5h6c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5 C19,11.29,18.14,10.28,17,10.05z M2.5,8V4.5h3.75V8H2.5z M5.5,13.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.05,13.5,5.5,13.5 z M11.5,11H7.75V9.5H10V8H7.75V4.5h3.75V11z M13,8V5.4L15.08,8H13z M16.5,13.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S17.05,13.5,16.5,13.5z",
                        }
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
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M21,11.18V9l-5-6H1v12h2.18C3.6,16.16,4.7,17,6,17s2.4-0.84,2.82-2h8.37c0.48,1.34,1.86,2.25,3.42,1.94 c1.16-0.23,2.11-1.17,2.33-2.33C23.25,13.05,22.34,11.66,21,11.18z M18.4,9H16V6.12L18.4,9z M3,5h4v4H3V5z M6,15 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.55,15,6,15z M9,13v-2h3V9H9V5h5v8H9z M20,15c-0.55,0-1-0.45-1-1s0.45-1,1-1 s1,0.45,1,1S20.55,15,20,15z",
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
                        d: "M12,11.99C12,10.34,13.34,9,14.99,9l0.24,0l-1.38-6H10v1h3.05l0.95,4.14c-1.57,0.4-2.75,1.72-2.96,3.36H6.93 c-0.26-1.01-1.29-1.72-2.44-1.44c-0.71,0.18-1.29,0.78-1.44,1.5C2.77,12.86,3.75,14,5,14c0.93,0,1.71-0.64,1.93-1.5H12V11.99z M5,13c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S5.55,13,5,13z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M7.82,16H15v-1c0-2.21,1.79-4,4-4h0.74L17.49,1H12v2h3.89l1.4,6.25c0,0-0.01,0-0.01,0c-2.16,0.65-3.81,2.48-4.19,4.75 H7.82c-0.48-1.34-1.86-2.24-3.42-1.94c-1.18,0.23-2.13,1.2-2.35,2.38C1.7,16.34,3.16,18,5,18C6.3,18,7.4,17.16,7.82,16z M5,16 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S5.55,16,5,16z",
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
                height: "20",
                width: "20",
                fill: "none",
            }
            polygon {
                points: "16.81,7.8 15.31,5.2 11.5,7.4 11.5,3 8.5,3 8.5,7.4 4.69,5.2 3.19,7.8 7,10 3.19,12.2 4.69,14.8 8.5,12.6 8.5,17 11.5,17 11.5,12.6 15.31,14.8 16.81,12.2 13,10",
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
                height: "24",
                fill: "none",
                width: "24",
            }
            polygon {
                points: "20.79,9.23 18.79,5.77 14,8.54 14,3 10,3 10,8.54 5.21,5.77 3.21,9.23 8,12 3.21,14.77 5.21,18.23 10,15.46 10,21 14,21 14,15.46 18.79,18.23 20.79,14.77 16,12",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M18,6l-3,3V4H3v12h12v-5l3,3V6z M10.5,10l2.25,1.3L12,12.6l-2.25-1.3v2.45h-1.5V11.3L6,12.6l-0.75-1.3L7.5,10L5.25,8.7 L6,7.4l2.25,1.3V6.25h1.5V8.7L12,7.4l0.75,1.3L10.5,10z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M18,10.48V4H2v16h16v-6.48l4,3.98v-11L18,10.48z M12,12l3,1.73l-1,1.73l-3-1.73V17H9v-3.27l-3,1.73l-1-1.73L8,12l-3-1.73 l1-1.73l3,1.73V7h2v3.27l3-1.73l1,1.73L12,12z",
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
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M16.34,2.66l-1.06,1.06C13.92,2.35,12.03,1.5,9.95,1.5c-2.06,0-3.92,0.83-5.28,2.18L3.61,2.61C5.24,1,7.48,0,9.95,0 C12.45,0,14.71,1.02,16.34,2.66z M6.09,5.09l1.06,1.06C7.87,5.44,8.86,5,9.95,5c1.11,0,2.12,0.46,2.85,1.2l1.06-1.06 c-1-1.01-2.38-1.63-3.91-1.63C8.45,3.5,7.08,4.11,6.09,5.09z M15.28,12.28C15.28,15.81,10,20,10,20s-5.28-4.19-5.28-7.72 C4.72,9.36,7.08,7,10,7S15.28,9.36,15.28,12.28z M10,11c-0.69,0-1.25,0.56-1.25,1.25c0,0.69,0.56,1.25,1.25,1.25 c0.69,0,1.25-0.56,1.25-1.25C11.25,11.56,10.69,11,10,11z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M12,9c-3.15,0-6,2.41-6,6.15c0,2.49,2,5.44,6,8.85c4-3.41,6-6.36,6-8.85C18,11.41,15.15,9,12,9z M12,16.5 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S12.83,16.5,12,16.5z M12,4c1.93,0,3.68,0.78,4.95,2.05l-1.41,1.41 C14.63,6.56,13.38,6,12,6S9.37,6.56,8.46,7.46L7.05,6.05C8.32,4.78,10.07,4,12,4z M19.78,3.23l-1.41,1.41 C16.74,3.01,14.49,2,12.01,2S7.27,3.01,5.64,4.63L4.22,3.22C6.22,1.23,8.97,0,12.01,0S17.79,1.23,19.78,3.23z",
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
                d: "M19.77 7.23l.01-.01-3.72-3.72L15 4.56l2.11 2.11c-1.05.4-1.76 1.47-1.58 2.71.16 1.1 1.1 1.99 2.2 2.11.47.05.88-.03 1.27-.2v8.21h-2V12h-3V3H4v18h10v-7.5h1.5V21h5V9c0-.69-.28-1.32-.73-1.77zM18 10c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zM8 18v-4.5H6L10 6v5h2l-4 7z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M17.85,7.5L17,2h-2l-0.85,5.5L17.85,7.5z M11.5,8.5V6l-4,1.75V6L2,8.5V18h16V8.5H11.5z M7.5,15H6v-3.5h1.5V15z M10.75,15 h-1.5v-3.5h1.5V15z M14,15h-1.5v-3.5H14V15z",
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
                    d: "M22,10v12H2V10l7-3v2l5-2l0,0l0,3H22z M17.2,8.5L18,2h3l0.8,6.5H17.2z M11,18h2v-4h-2V18z M7,18h2v-4H7V18z M17,14h-2v4h2 V14z",
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
                d: "M18 5V1h-2v4h-5l.23 2.31C14.9 8.16 18 10.77 18 15l.02 8h3.18L23 5h-5zM1 21h15v2H1zM8.5 8.99C4.75 8.99 1 11 1 15h15c0-4-3.75-6.01-7.5-6.01zM1 17h15v2H1z",
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
                    height: "20",
                    width: "20",
                    y: "0",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M23,11V9c-6-2-11-7-11-7S7,7,1,9v2c0,1.49,0.93,2.75,2.24,3.26C3.2,16.76,2.92,19.69,2,22h5h10h5 c-0.92-2.31-1.2-5.24-1.24-7.74C22.07,13.75,23,12.49,23,11z M12,4.71c1.33,1.14,3.49,2.84,6.11,4.29H5.89 C8.51,7.55,10.67,5.85,12,4.71z M13,11h3c0,0.83-0.67,1.5-1.5,1.5S13,11.83,13,11z M9.5,12.5C8.67,12.5,8,11.83,8,11h3 C11,11.83,10.33,12.5,9.5,12.5z M6,11c0,0.83-0.67,1.5-1.5,1.5S3,11.83,3,11H6z M4.66,20c0.39-1.86,0.54-3.82,0.57-5.58 c0.68-0.15,1.29-0.49,1.76-0.98c0.25,0.25,0.54,0.45,0.85,0.62c-0.1,1.87-0.26,4-0.52,5.93H4.66z M9.35,20 c0.24-1.83,0.39-3.78,0.48-5.53c0.84-0.08,1.61-0.45,2.17-1.02c0.56,0.57,1.32,0.94,2.17,1.02c0.1,1.75,0.24,3.7,0.48,5.53H9.35z M16.67,20c-0.27-1.94-0.43-4.07-0.52-5.93c0.31-0.17,0.61-0.37,0.85-0.62c0.47,0.48,1.08,0.83,1.76,0.98 c0.03,1.76,0.18,3.72,0.57,5.58H16.67z M19.5,12.5c-0.83,0-1.5-0.67-1.5-1.5h3C21,11.83,20.33,12.5,19.5,12.5z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    circle {
                        cy: "11.5",
                        r: "1.25",
                        cx: "10",
                    }
                    path {
                        d: "M17,9h-2V7h1V5.5h-1.25C14.11,3.48,12.24,2,10,2S5.89,3.48,5.25,5.5H4V7h1v2H3v5h2v2.5H4V18h12v-1.5h-1V14h2V9z M10.54,14.2c-1.94,0.37-3.6-1.3-3.24-3.24c0.2-1.08,1.08-1.96,2.16-2.16c1.94-0.37,3.6,1.3,3.24,3.24 C12.49,13.11,11.61,13.99,10.54,14.2z",
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
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M21,11h-3V8h2V6h-2.35C16.83,3.67,14.61,2,12,2S7.17,3.67,6.35,6H4v2h2v3H3v6h3v3H4v2h16v-2h-2v-3h3V11z M12,17.5 c-1.93,0-3.5-1.57-3.5-3.5s1.57-3.5,3.5-3.5s3.5,1.57,3.5,3.5S13.93,17.5,12,17.5z",
                    }
                    circle {
                        r: "1.5",
                        cx: "12",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M19,9l-2-4h-1.5V3H13v2h-3v4.5H1V15h2.05c0.23,1.14,1.24,2,2.45,2c1.21,0,2.22-0.86,2.45-2h4.1c0.23,1.14,1.24,2,2.45,2 c1.21,0,2.22-0.86,2.45-2H19V9z M5.5,15.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.05,15.5,5.5,15.5z M14.5,15.5 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S15.05,15.5,14.5,15.5z M17.5,9.5h-6v-3h4.57l1.43,2.85V9.5z",
                    }
                    path {
                        d: "M9,7.5H8V6h1V5H1v1h1v1.5H1v1h8V7.5z M4.5,7.5H3V6h1.5V7.5z M7,7.5H5.5V6H7V7.5z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M23,11l-2-6h-2V3h-3v2h-4v6H1v7h3c0,1.66,1.34,3,3,3s3-1.34,3-3h4c0,1.66,1.34,3,3,3s3-1.34,3-3h3V11z M7,19 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S7.55,19,7,19z M17,19c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S17.55,19,17,19z M14,11V7h5.56l1.33,4H14z",
                    }
                    path {
                        d: "M11,8.5h-1v-2h1V5H1v1.5h1v2H1V10h10V8.5z M5.25,8.5H3.5v-2h1.75V8.5z M8.5,8.5H6.75v-2H8.5V8.5z",
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
                height: "20",
                fill: "none",
            }
            path {
                d: "M14.5,4H10v6.75h4.5V4z M8.4,13H15v1.5H7.29L5,7V4h1.5v3L8.4,13z M7.5,15.5H15V17H7.5V15.5z",
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
                d: "M18,4h-6v9h6V4z M9.5,16H18v2H8L5,8V4h2v4L9.5,16z M8,19h10v2H8V19z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    polygon {
                        points: "13,10 8,2 3,10 4.18,10 1,15 6.5,15 6.5,18 9.5,18 9.5,15 15,15 11.82,10",
                    }
                    polygon {
                        points: "15.82,10 17,10 12,2 10.59,4.26 14.8,11 13.64,11 16.19,15 19,15",
                    }
                    rect {
                        height: "2",
                        width: "3",
                        x: "10.5",
                        y: "16",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    polygon {
                        points: "16,12 9,2 2,12 3.86,12 0,18 7,18 7,22 11,22 11,18 18,18 14.14,12",
                    }
                    polygon {
                        points: "20.14,12 22,12 15,2 12.61,5.41 17.92,13 16,13 15.97,13 19.19,18 24,18",
                    }
                    rect {
                        height: "3",
                        y: "19",
                        x: "13",
                        width: "4",
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
                    height: "24",
                    width: "24",
                    fill: "none",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    polygon {
                        points: "17.5,3 17.5,4.5 15.75,4.5 15.75,3 14.25,3 14.25,4.5 12.5,4.5 12.5,3 11,3 11,6 13,8 13,9 7,9 7,8 9,6 9,3 7.5,3 7.5,4.5 5.75,4.5 5.75,3 4.25,3 4.25,4.5 2.5,4.5 2.5,3 1,3 1,6 3,8 3,12 1,14 1,17 8,17 8,13 12,13 12,17 19,17 19,14 17,12 17,8 19,6 19,3",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    polygon {
                        points: "21,3 21,5 19,5 19,3 17,3 17,5 15,5 15,3 13,3 13,7 15,9 15,10 9,10 9,9 11,7 11,3 9,3 9,5 7,5 7,3 5,3 5,5 3,5 3,3 1,3 1,7 3,9 3,15 1,17 1,21 10,21 10,16 14,16 14,21 23,21 23,17 21,15 21,9 23,7 23,3",
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
                    width: "24",
                    y: "0",
                    height: "24",
                    fill: "none",
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
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M17.41,15.29l-4.24-4.24h-1.41l-0.34-0.34l-0.71,0.71l0.34,0.34v1.41l4.24,4.24L17.41,15.29z",
                    }
                    path {
                        d: "M12.57,8.14L12.57,8.14l0.88,0.88l1.06-1.06l1.41,1.41c0.78-0.78,0.78-2.05,0-2.83l-2.47-2.47l-0.74,0.74l0-1.49 l-0.49-0.49L9.74,5.31l0.49,0.49l1.49,0l-0.74,0.74l0.88,0.88L10,9.29L7.51,6.81l0.15-1.26L5.36,3.23L3.23,5.36l2.31,2.31 l1.26-0.15L9.29,10l-1.05,1.05H6.83l-4.24,4.24l2.12,2.12l4.24-4.24v-1.41L12.57,8.14z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M16.37,12.87h-0.99l-2.54,2.54v0.99l6.01,6.01l3.54-3.54L16.37,12.87z",
                        }
                    }
                    g {
                        path {
                            d: "M17.34,10.19l1.41-1.41l2.12,2.12c1.17-1.17,1.17-3.07,0-4.24l-3.54-3.54l-1.41,1.41V1.71L15.22,1l-3.54,3.54l0.71,0.71 h2.83l-1.41,1.41l1.06,1.06l-2.89,2.89L7.85,6.48V5.06L4.83,2.04L2,4.87l3.03,3.03h1.41l4.13,4.13l-0.85,0.85H7.6l-6.01,6.01 l3.54,3.54l6.01-6.01v-2.12l5.15-5.15L17.34,10.19z",
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
                    y: "0",
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        y: "10.75",
                        x: "7.5",
                        height: "6.25",
                        width: "4.5",
                    }
                    path {
                        d: "M14.5,3L12,5.5V3H8C5.79,3,4,4.79,4,7h3.5v2.25H12V6.5L14.5,9H16V3H14.5z",
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
                    fill: "none",
                    width: "24",
                    y: "0",
                }
            }
            g {
                g {
                    g {
                        g {
                            path {
                                d: "M18,3l-3,3V3H9C6.24,3,4,5.24,4,8h5v3h6V8l3,3h2V3H18z",
                            }
                        }
                        g {
                            path {
                                d: "M9,13v8h6v-8H9z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    g {
                        polygon {
                            points: "14,13 13,13 13,12 7,12 7,13 6,13 6,12 3,12 3,16 17,16 17,12 14,12",
                        }
                    }
                    path {
                        d: "M13,7V4H7v3H3v4h3v-1h1v1h6v-1h1v1h3V7H13z M12,7H8V5h4V7z",
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
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        polygon {
                            points: "18,16 16,16 16,15 8,15 8,16 6,16 6,15 2,15 2,20 22,20 22,15 18,15",
                        }
                    }
                    path {
                        d: "M17,8V4H7v4H2v6h4v-2h2v2h8v-2h2v2h4V8H17z M9,6h6v2H9V6z",
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
                d: "M7 13c1.66 0 3-1.34 3-3S8.66 7 7 7s-3 1.34-3 3 1.34 3 3 3zm16-6H11v7H3V5H1v15h2v-3h18v3h2V7z",
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
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
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
                            d: "M7.42,11.5h5.17c0.22-0.38,0.36-0.8,0.39-1.26H7.02C7.06,10.7,7.2,11.12,7.42,11.5z",
                        }
                    }
                    g {
                        path {
                            d: "M7.42,8.49c-0.22,0.38-0.36,0.8-0.4,1.26h5.95c-0.04-0.46-0.18-0.88-0.4-1.26H7.42z",
                        }
                    }
                    g {
                        path {
                            d: "M4,4v12h12V4H4z M10,14c-2.21,0-4-1.79-4-4c0-2.21,1.79-4,4-4c2.21,0,4,1.79,4,4C14,12.21,12.21,14,10,14z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M8.56,14h6.89c0.26-0.45,0.44-0.96,0.51-1.5h-7.9C8.12,13.04,8.29,13.55,8.56,14z",
                        }
                    }
                    g {
                        path {
                            d: "M12,16c1.01,0,1.91-0.39,2.62-1H9.38C10.09,15.61,10.99,16,12,16z",
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
                            d: "M3,3v18h18V3H3z M12,18c-3.31,0-6-2.69-6-6s2.69-6,6-6s6,2.69,6,6S15.31,18,12,18z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M14.92,5.16C14.51,2.8,12.47,1,10,1S5.49,2.8,5.08,5.16C3.88,5.55,3,6.66,3,8c0,1.66,1.34,3,3,3c0.02,0,0.03,0,0.04,0 L10,19l3.96-8c0.01,0,0.03,0,0.04,0c1.66,0,3-1.34,3-3C17,6.66,16.12,5.55,14.92,5.16z M10,15.61l-2.49-5.04 c0.16-0.1,0.07-0.08,0.37-0.08C8.53,10.81,9.24,11,10,11s1.47-0.19,2.12-0.5c0.3,0,0.21-0.02,0.37,0.08L10,15.61z",
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
                    height: "24",
                    y: "0",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M18.38,6.24C17.79,3.24,15.14,1,12,1S6.21,3.24,5.62,6.24C4.08,6.81,3,8.29,3,10c0,2.21,1.79,4,4,4 c0.12,0,0.23-0.02,0.34-0.02L12.07,23l4.61-9.03C16.79,13.98,16.89,14,17,14c2.21,0,4-1.79,4-4C21,8.29,19.92,6.81,18.38,6.24z M12.05,18.63l-2.73-5.21C10.15,13.79,11.06,14,12,14c0.95,0,1.88-0.21,2.72-0.6L12.05,18.63z",
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
                fill: "none",
                height: "20",
            }
            path {
                d: "M6.5,7H9v4H6.5v1H7c1.1,0,2,0.9,2,2s-0.9,2-2,2H6.5v3h-1v-3H5c-1.1,0-2-0.9-2-2s0.9-2,2-2h0.5v-1H3V7h2.5V6H5 C3.9,6,3,5.1,3,4s0.9-2,2-2h0.5V1h1v1H7c1.1,0,2,0.9,2,2S8.1,6,7,6H6.5V7z M14.5,6H15c1.1,0,2-0.9,2-2s-0.9-2-2-2h-0.5V1h-1v1H13 c-1.1,0-2,0.9-2,2s0.9,2,2,2h0.5v1H11v4h2.5v1H13c-1.1,0-2,0.9-2,2s0.9,2,2,2h0.5v3h1v-3H15c1.1,0,2-0.9,2-2s-0.9-2-2-2h-0.5v-1H17 V7h-2.5V6z",
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
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M7.75,8H11v5H7.75v1H8.5c1.38,0,2.5,1.12,2.5,2.5S9.88,19,8.5,19H7.75v4h-1.5v-4H5.5C4.12,19,3,17.88,3,16.5S4.12,14,5.5,14 h0.75v-1H3V8h3.25V7H5.5C4.12,7,3,5.88,3,4.5S4.12,2,5.5,2h0.75V1h1.5v1H8.5C9.88,2,11,3.12,11,4.5S9.88,7,8.5,7H7.75V8z M17.75,7 h0.75C19.88,7,21,5.88,21,4.5S19.88,2,18.5,2h-0.75V1h-1.5v1H15.5C14.12,2,13,3.12,13,4.5S14.12,7,15.5,7h0.75v1H13v5h3.25v1H15.5 c-1.38,0-2.5,1.12-2.5,2.5s1.12,2.5,2.5,2.5h0.75v4h1.5v-4h0.75c1.38,0,2.5-1.12,2.5-2.5S19.88,14,18.5,14h-0.75v-1H21V8h-3.25V7z",
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
                d: "M11.99 18.54l-7.37-5.73L3 14.07l9 7 9-7-1.63-1.27-7.38 5.74zM12 16l7.36-5.73L21 9l-9-7-9 7 1.63 1.27L12 16z",
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
                d: "M21 9l-9-7-2.59 2.02 7.87 7.87zm0 5.07l-1.63-1.27-.67.52 1.43 1.43zM3.41.86L2 2.27l4.22 4.22L3 9l9 7 2.1-1.63 1.42 1.42-3.53 2.75-7.37-5.73L3 14.07l9 7 4.95-3.85L20.73 21l1.41-1.41z",
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
                    y: "0",
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M3,11.5c0,1.12,0.74,2.05,1.75,2.37v2.63H3V18h5v-1.5H6.25v-2.63C7.26,13.55,8,12.62,8,11.5V5H3V11.5z M6.5,6.5V9h-2V6.5 H6.5z",
                        }
                    }
                    g {
                        path {
                            d: "M18,8l-2.5-1V2h-4v5L9,8v10h9V8z M14,3.5v1h-1v-1H14z M13,8.02L13,6h1l-0.04,2l2.54,1.02V10h-6V9.02L13,8.02z M10.5,16.5 V15h6v1.5H10.5z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                    y: "0",
                }
            }
            g {
                g {
                    g {
                        g {
                            path {
                                d: "M3,14c0,1.3,0.84,2.4,2,2.82V20H3v2h6v-2H7v-3.18C8.16,16.4,9,15.3,9,14V6H3V14z M5,8h2v3H5V8z",
                            }
                        }
                        g {
                            path {
                                d: "M22,9l-3-1.01V2h-5v6l-3,1.01V22h11V9z M16,4h1v1h-1V4z M13,10.44l3-0.98V7h1v2.46l3,0.98V12h-7V10.44z M20,20h-7v-2h7 V20z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M20 12c0-1.1.9-2 2-2V4H2.01v6c1.1 0 1.99.9 1.99 2s-.89 2-2 2v6h20v-6c-1.1 0-2-.9-2-2zm-4.42 4.8L12 14.5l-3.58 2.3 1.08-4.12-3.29-2.69 4.24-.25L12 5.8l1.54 3.95 4.24.25-3.29 2.69 1.09 4.11z",
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
                    x: "0",
                    width: "20",
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
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
                d: "M11 17h2v-1h2v-5h-4v-1h4V8h-2V7h-2v1H9v5h4v1H9v2h2v1zM22 4H2.01L2 20h20V4zm-2 14H4V6h16v12z",
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
                d: "M21 5V3H3v2l8 9v5H6v2h12v-2h-5v-5l8-9zM7.43 7L5.66 5h12.69l-1.78 2H7.43z",
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
                d: "M20 3H4v14h14v-7h2c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 5h-2V5h2v3zM2 21h18v-2H2v2z",
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
                d: "M18.58 7H5.43L3 14v9h3v-2h12v2h3v-9l-2.42-7zM6.5 18c-.83 0-1.5-.67-1.5-1.5S5.67 15 6.5 15s1.5.67 1.5 1.5S7.33 18 6.5 18zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 13l1.5-4.5h11L19 13H5zm12-8c.83 0 1.5-.67 1.5-1.5 0-1-1.5-2.7-1.5-2.7s-1.5 1.7-1.5 2.7c0 .83.67 1.5 1.5 1.5zm-5 0c.83 0 1.5-.67 1.5-1.5 0-1-1.5-2.7-1.5-2.7s-1.5 1.7-1.5 2.7c0 .83.67 1.5 1.5 1.5zM7 5c.83 0 1.5-.67 1.5-1.5C8.5 2.5 7 .8 7 .8S5.5 2.5 5.5 3.5C5.5 4.33 6.17 5 7 5z",
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
                d: "M19 7V4H5v3H2v13h8v-4h4v4h8V7h-3zm-8 3H9v1h2v1H8V9h2V8H8V7h3v3zm5 2h-1v-2h-2V7h1v2h1V7h1v5z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
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
                d: "M3 2l2.21 20H18.8L21 2H3zm9 17c-1.66 0-3-1.34-3-3 0-2 3-5.4 3-5.4s3 3.4 3 5.4c0 1.66-1.34 3-3 3zm6.33-11H5.67l-.44-4h13.53l-.43 4z",
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
                    width: "20",
                    fill: "none",
                    height: "20",
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
                            d: "M13,5l-0.33,0.41C11.79,6.52,10,5.89,10,4.47V2c0,0-6,3.75-6,9c0,2.2,1.18,4.12,2.95,5.16c-0.44-0.61-0.7-1.36-0.7-2.16 c0-0.92,0.34-1.81,0.96-2.5L10,8.37l2.79,3.12c0.62,0.69,0.96,1.58,0.96,2.5c0,0.79-0.25,1.52-0.67,2.13 c1.43-0.86,2.5-2.3,2.81-3.99C16.39,9.48,15.1,6.21,13,5z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M12,12.9l-2.13,2.09C9.31,15.55,9,16.28,9,17.06C9,18.68,10.35,20,12,20s3-1.32,3-2.94c0-0.78-0.31-1.52-0.87-2.07 L12,12.9z",
                        }
                    }
                    g {
                        path {
                            d: "M16,6l-0.44,0.55C14.38,8.02,12,7.19,12,5.3V2c0,0-8,4-8,11c0,2.92,1.56,5.47,3.89,6.86C7.33,19.07,7,18.1,7,17.06 c0-1.32,0.52-2.56,1.47-3.5L12,10.1l3.53,3.47c0.95,0.93,1.47,2.17,1.47,3.5c0,1.02-0.31,1.96-0.85,2.75 c1.89-1.15,3.29-3.06,3.71-5.3C20.52,10.97,18.79,7.62,16,6z",
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
                d: "M12 22c4.97 0 9-4.03 9-9-4.97 0-9 4.03-9 9zM5.6 10.25c0 1.38 1.12 2.5 2.5 2.5.53 0 1.01-.16 1.42-.44l-.02.19c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5l-.02-.19c.4.28.89.44 1.42.44 1.38 0 2.5-1.12 2.5-2.5 0-1-.59-1.85-1.43-2.25.84-.4 1.43-1.25 1.43-2.25 0-1.38-1.12-2.5-2.5-2.5-.53 0-1.01.16-1.42.44l.02-.19C14.5 2.12 13.38 1 12 1S9.5 2.12 9.5 3.5l.02.19c-.4-.28-.89-.44-1.42-.44-1.38 0-2.5 1.12-2.5 2.5 0 1 .59 1.85 1.43 2.25-.84.4-1.43 1.25-1.43 2.25zM12 5.5c1.38 0 2.5 1.12 2.5 2.5s-1.12 2.5-2.5 2.5S9.5 9.38 9.5 8s1.12-2.5 2.5-2.5zM3 13c0 4.97 4.03 9 9 9 0-4.97-4.03-9-9-9z",
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
                d: "m19.77 7.23.01-.01-3.72-3.72L15 4.56l2.11 2.11c-1.05.4-1.76 1.47-1.58 2.71.16 1.1 1.1 1.99 2.2 2.11.47.05.88-.03 1.27-.2v8.21h-2V12h-3V3H4v18h10v-7.5h1.5v7.49h5V9c0-.69-.28-1.32-.73-1.77zM12 10H6V5h6v5zm6 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
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
                d: "M7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zM1 4h2l3.6 7.59L3.62 17H19v-2H7l1.1-2h8.64l4.97-9H5.21l-.94-2H1v2zm16 14c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M21 3H3.01L3 21h18V3zm-3 11h-4v4h-4v-4H6v-4h4V6h4v4h4v4z",
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
                d: "M7 13c1.66 0 3-1.34 3-3S8.66 7 7 7s-3 1.34-3 3 1.34 3 3 3zm16-6H11v7H3V5H1v15h2v-3h18v3h2V7z",
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
                d: "M9.17 16.83c1.56 1.56 4.1 1.56 5.66 0s1.56-4.1 0-5.66l-5.66 5.66zM20 2.01L4 2v20h16V2.01zM10 4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zM7 4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm5 16c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M12 11.55C9.64 9.35 6.48 8 3 8v11c3.48 0 6.64 1.35 9 3.55 2.36-2.19 5.52-3.55 9-3.55V8c-3.48 0-6.64 1.35-9 3.55zM12 8c1.66 0 3-1.34 3-3s-1.34-3-3-3-3 1.34-3 3 1.34 3 3 3z",
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
                d: "M21 6h-4c0-2.76-2.24-5-5-5S7 3.24 7 6H3v16h18V6zm-9-3c1.66 0 3 1.34 3 3H9c0-1.66 1.34-3 3-3zm0 10c-2.76 0-5-2.24-5-5h2c0 1.66 1.34 3 3 3s3-1.34 3-3h2c0 2.76-2.24 5-5 5z",
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
                d: "M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z",
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
                d: "M22.83 12.99 11.83 2H2v9.83l10.99 10.99 9.84-9.83zM5.5 7C4.67 7 4 6.33 4 5.5S4.67 4 5.5 4 7 4.67 7 5.5 6.33 7 5.5 7z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M21 5h-2.64l1.14-3.14L17.15 1l-1.46 4H3v2l2 6-2 6v2h18v-2l-2-6 2-6V5zm-5 9h-3v3h-2v-3H8v-2h3V9h2v3h3v2z",
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
                d: "M21 15.46l-5.27-.61-2.52 2.52c-2.83-1.44-5.15-3.75-6.59-6.59l2.53-2.53L8.54 3H3.03C2.45 13.18 10.82 21.55 21 20.97v-5.51z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M12 2C8.43 2 5.23 3.54 3.01 6L12 22l8.99-16C18.78 3.55 15.57 2 12 2zM7 7c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2-2-.9-2-2zm5 8c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
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
                d: "M20 12c0-1.1.9-2 2-2V4H2.01v6c1.1 0 1.99.9 1.99 2s-.89 2-2 2v6h20v-6c-1.1 0-2-.9-2-2zm-4.42 4.8L12 14.5l-3.58 2.3 1.08-4.12-3.29-2.69 4.24-.25L12 5.8l1.54 3.95 4.24.25-3.29 2.69 1.09 4.11z",
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
                d: "M14.5,12.59l0.9,3.88L12,14.42l-3.4,2.05l0.9-3.87l-3-2.59l3.96-0.34L12,6.02l1.54,3.64L17.5,10L14.5,12.59z M3,5v6 c0,5.55,3.84,10.74,9,12c5.16-1.26,9-6.45,9-12V5l-9-4L3,5z",
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
                d: "M22 4H2.01v16H22V4zm-2 4l-8 5-8-5V6l8 5 8-5v2z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M2 8v9h4v4h12v-4h4V8H2zm14 11H8v-5h8v5zm3-7c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm-1-9H6v4h12V3z",
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
            circle {
                cy: "12",
                cx: "12",
                r: "3.2",
            }
            path {
                d: "M22 4h-5.17L15 2H9L7.17 4H2v16h20V4zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z",
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
                d: "M20 8h-3V4H1v13h2c0 1.66 1.34 3 3 3s3-1.34 3-3h6c0 1.66 1.34 3 3 3s3-1.34 3-3h2v-5l-3-4zM6 18c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm13.5-8.5l1.96 2.5H17V9.5h2.5zM18 18c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
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
                d: "M18.58 5H15V3H9v2H5.43L3 12v9h3v-2h12v2h3v-9l-2.42-7zM6.5 16c-.83 0-1.5-.67-1.5-1.5S5.67 13 6.5 13s1.5.67 1.5 1.5S7.33 16 6.5 16zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 11l1.5-4.5h11L19 11H5z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M17.99,8c0.26-2.73-3.42-5-7.99-5C5.44,3,1.76,5.27,2.01,8H17.99z",
                    }
                    rect {
                        width: "16",
                        height: "4",
                        x: "2",
                        y: "13",
                    }
                    path {
                        d: "M15.34,9.25c-1.58,0-1.72,1-2.67,1c-0.95,0-1.08-1-2.67-1c-1.58,0-1.72,1-2.67,1c-0.95,0-1.09-1-2.67-1 c-1.58,0-1.72,1-2.67,1v1.5c1.58,0,1.72-1,2.67-1c0.95,0,1.09,1,2.67,1c1.58,0,1.72-1,2.67-1c0.95,0,1.08,1,2.67,1 c1.58,0,1.72-1,2.67-1c0.95,0,1.08,1,2.66,1v-1.5C17.05,10.25,16.92,9.25,15.34,9.25z",
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
                    rect {
                        width: "20",
                        y: "16",
                        height: "5",
                        x: "2",
                    }
                    path {
                        d: "M18.66,11.5c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1 c-1.95,0-2.09,1-3.33,1v2c1.9,0,2.17-1,3.35-1c1.19,0,1.42,1,3.33,1c1.95,0,2.09-1,3.33-1c1.19,0,1.42,1,3.33,1 c1.95,0,2.09-1,3.33-1c1.19,0,1.4,0.98,3.32,1l-0.01-1.98C20.38,12.19,20.37,11.5,18.66,11.5z",
                    }
                    path {
                        d: "M22,9c0.02-4-4.28-6-10-6C6.29,3,2,5,2,9v1h20L22,9L22,9z",
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
                d: "M15 5.1L9 3 3 5.02v16.2l6-2.33 6 2.1 6-2.02V2.77L15 5.1zm0 13.79l-6-2.11V5.11l6 2.11v11.67z",
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
                y: "0",
                fill_rule: "evenodd",
                height: "24",
                fill: "none",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M18,6h-5.75V2h-4.5v4H2v12h16V6z M9.25,13.25H7.5V15H6v-1.75H4.25v-1.5H6V10h1.5v1.75h1.75V13.25z M9.25,3.5h1.5v4h-1.5 V3.5z M14,14.5h-3.25v-1H14V14.5z M15.75,12h-5v-1h5V12z",
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
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M22,7h-7V2H9v5H2v15h20V7z M11,4h2v5h-2V4z M11,16H9v2H7v-2H5v-2h2v-2h2v2h2V16z M13,14.5V13h6v1.5H13z M13,17.5V16h4v1.5 H13z",
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
                        d: "M13,6V3H7v3H3v11h14V6H13z M8,4h4v2H8V4z M12.5,12h-2v2h-1v-2h-2v-1h2V9h1v2h2V12z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M16,6V2H8v4H2v16h20V6H16z M10,4h4v2h-4V4z M16,15h-3v3h-2v-3H8v-2h3v-3h2v3h3V15z",
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
            }
            g {
                g {
                }
                path {
                    d: "M21,5c-1.11-0.35-2.33-0.5-3.5-0.5c-1.95,0-4.05,0.4-5.5,1.5c-1.45-1.1-3.55-1.5-5.5-1.5S2.45,4.9,1,6v15.5 C2.45,20.4,4.55,20,6.5,20s4.05,0.4,5.5,1.5c1.45-1.1,3.55-1.5,5.5-1.5c1.17,0,2.39,0.15,3.5,0.5c0.75,0.25,1.4,0.55,2,1V6 C22.4,5.55,21.75,5.25,21,5z M21,18.5c-1.1-0.35-2.3-0.5-3.5-0.5c-1.7,0-4.15,0.65-5.5,1.5V8c1.35-0.85,3.8-1.5,5.5-1.5 c1.2,0,2.4,0.15,3.5,0.5V18.5z",
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
                    fill: "none",
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
                    width: "24",
                    height: "24",
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
                    fill: "none",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M17,12v7h-2v-1.5H5V19H3l0-7l2-5h10L17,12z M6.02,8.5l-1,2.5h9.97l-1-2.5H6.02z M6.5,13.25c-0.55,0-1,0.45-1,1s0.45,1,1,1 c0.55,0,1-0.45,1-1S7.05,13.25,6.5,13.25z M13.5,13.25c-0.55,0-1,0.45-1,1s0.45,1,1,1c0.55,0,1-0.45,1-1S14.05,13.25,13.5,13.25z M6.95,5.75L8,4.7L5.65,2.35L4.6,3.4L6.95,5.75z M15.4,3.4l-1.05-1.05L12,4.7l1.05,1.05L15.4,3.4z M10.75,4.7h-1.5V1h1.5V4.7z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18.57,8H5.43L3,15v9h3v-2h12v2h3v-9L18.57,8z M6.85,10h10.29l1.04,3H5.81L6.85,10z M6,17.5C6,16.67,6.67,16,7.5,16 S9,16.67,9,17.5S8.33,19,7.5,19S6,18.33,6,17.5z M15,17.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S17.33,19,16.5,19 S15,18.33,15,17.5z M9.41,5L8,6.41l-3-3L6.41,2L9.41,5z M16,6.41L14.59,5l3-3L19,3.41L16,6.41z M13,5h-2V0h2V5z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M11.58,11.55l1.21-2.1c0,0-0.06-0.06-0.16-0.13l-1.05-0.83l-0.01,0.01C11.6,8.34,11.61,8.17,11.61,8 c0-0.17-0.01-0.34-0.04-0.49l0.01,0.01l1.22-0.96l-1.21-2.11l-1.44,0.58v0.01c-0.26-0.2-0.55-0.37-0.86-0.5H9.3L9.08,3H6.65 L6.42,4.54h0.01c-0.31,0.13-0.59,0.3-0.86,0.5V5.03L4.14,4.45L2.92,6.56l1.21,0.96l0.01-0.01C4.12,7.66,4.11,7.83,4.11,8 c0,0.17,0.01,0.34,0.04,0.51L4.14,8.49L3.09,9.32C3,9.39,2.92,9.45,2.92,9.45l1.21,2.1l1.44-0.57l-0.01-0.02 c0.26,0.21,0.55,0.38,0.86,0.51H6.42L6.65,13h2.43c0,0,0.01-0.09,0.03-0.21l0.19-1.32H9.29c0.31-0.13,0.6-0.3,0.86-0.51 l-0.01,0.02L11.58,11.55z M7.86,9.5c-0.83,0-1.5-0.68-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5c0.82,0,1.5,0.67,1.5,1.5 C9.36,8.82,8.68,9.5,7.86,9.5z",
                    }
                    path {
                        d: "M16.17,14.3L16.17,14.3c0.01-0.09,0.02-0.19,0.02-0.3c0-0.1-0.01-0.2-0.03-0.3l0,0l0.73-0.57l-0.73-1.26l-0.86,0.35v0 c-0.16-0.12-0.33-0.22-0.51-0.3h0L14.67,11h-1.46l-0.13,0.92h0c-0.18,0.08-0.36,0.18-0.51,0.3v0l-0.86-0.35l-0.73,1.26l0.73,0.57 l0,0c-0.01,0.09-0.02,0.19-0.02,0.3c0,0.1,0.01,0.2,0.02,0.3l0-0.01l-0.63,0.5c-0.06,0.04-0.1,0.08-0.1,0.08l0.73,1.26l0.87-0.34 l-0.01-0.01c0.16,0.12,0.33,0.23,0.52,0.3h0L13.21,17h1.46c0,0,0.01-0.06,0.02-0.13l0.12-0.79h0c0.19-0.08,0.36-0.18,0.52-0.3 l-0.01,0.01l0.87,0.34l0.73-1.26c0,0-0.04-0.03-0.1-0.08L16.17,14.3z M14,15c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 c0.55,0,1,0.45,1,1C15,14.55,14.55,15,14,15z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M14.02,13.97l1.7-2.94c0,0-0.09-0.08-0.23-0.18l-1.47-1.16l-0.01,0.02c0.03-0.24,0.05-0.47,0.05-0.71S14.04,8.53,14,8.31 l0.01,0.01l1.71-1.34l-1.7-2.95l-2.01,0.81v0.01c-0.37-0.28-0.77-0.52-1.2-0.7h0.01L10.52,2H7.11L6.8,4.15h0.01 c-0.43,0.18-0.83,0.42-1.2,0.7V4.84L3.6,4.03L1.9,6.98l1.7,1.34l0.01-0.01C3.58,8.53,3.56,8.76,3.56,9s0.02,0.47,0.05,0.71 L3.6,9.69l-1.47,1.16C2,10.95,1.9,11.03,1.9,11.03l1.7,2.94l2.02-0.8L5.6,13.14c0.37,0.29,0.77,0.53,1.21,0.71H6.8L7.11,16h3.4 c0,0,0.02-0.13,0.04-0.3l0.27-1.85h-0.01c0.44-0.18,0.84-0.42,1.21-0.71L12,13.17L14.02,13.97z M8.81,11c-1.1,0-2-0.9-2-2 s0.9-2,2-2s2,0.9,2,2S9.91,11,8.81,11z",
                    }
                    path {
                        d: "M20.98,17.94l-0.01,0.01C20.99,17.8,21,17.65,21,17.5c0-0.15-0.01-0.3-0.04-0.44l0.01,0.01l1.1-0.86l-1.09-1.9l-1.29,0.52 v0.01c-0.24-0.18-0.49-0.33-0.77-0.45h0.01L18.73,13h-2.19l-0.2,1.38h0.01c-0.28,0.12-0.53,0.27-0.77,0.45v-0.01l-1.29-0.52 l-1.09,1.9l1.09,0.86l0.01-0.01c-0.02,0.14-0.03,0.29-0.03,0.44c0,0.15,0.01,0.3,0.03,0.46l-0.01-0.01l-0.94,0.75 c-0.08,0.06-0.15,0.12-0.15,0.12l1.09,1.89l1.3-0.51l-0.01-0.02c0.24,0.19,0.5,0.34,0.78,0.46h-0.01l0.2,1.38h2.19 c0,0,0.01-0.08,0.03-0.19l0.17-1.19h-0.01c0.28-0.12,0.54-0.27,0.78-0.46l-0.01,0.02l1.3,0.51l1.09-1.89c0,0-0.06-0.05-0.15-0.12 L20.98,17.94z M17.63,18.79c-0.71,0-1.29-0.58-1.29-1.29s0.58-1.29,1.29-1.29s1.29,0.58,1.29,1.29S18.34,18.79,17.63,18.79z",
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
                height: "20",
                width: "20",
            }
            path {
                d: "M16.48,8C16.23,4.64,13.42,2,10,2C6.41,2,3.5,4.91,3.5,8.5C3.5,12.84,10,18,10,18s1.28-1.01,2.7-2.52 c0.1,0.01,0.2,0.02,0.3,0.02c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2c0,0.44,0.14,0.85,0.38,1.18c-0.49,0.51-0.97,0.97-1.38,1.35 C8.1,14.3,5,10.97,5,8.5c0-2.76,2.24-5,5-5c2.59,0,4.72,1.98,4.98,4.5h-2.73l3.5,3.5l3.5-3.5L16.48,8z",
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
                width: "24",
                height: "24",
            }
            path {
                d: "M15.31,18.9c-0.96,1-2.06,2.03-3.31,3.1c-5.33-4.55-8-8.48-8-11.8C4,5.22,7.8,2,12,2c4,0,7.64,2.92,7.97,7.5l3.53,0L19,14 l-4.5-4.5l3.47,0C17.65,6.24,15.13,4,12,4c-3.35,0-6,2.57-6,6.2c0,2.34,1.95,5.44,6,9.14c0.64-0.59,1.23-1.16,1.77-1.71 c-0.17-0.34-0.27-0.72-0.27-1.12c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5S17.38,19,16,19C15.76,19,15.53,18.97,15.31,18.9z",
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
                d: "M14 16h5V8h-5v8zm2-6h1v4h-1v-4zm-8 6h5V8H8v8zm2-6h1v4h-1v-4zM5 8h2v8H5zM2 4v16h20V4H2zm18 14H4V6h16v12z",
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
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M15,6h-3v1h2v1.79L10.79,12H9V8H6c-1.66,0-3,1.34-3,3v2h2c0,1.1,0.9,2,2,2s2-0.9,2-2h2.21L15,9.21V6z M7,14 c-0.55,0-1-0.45-1-1h2C8,13.55,7.55,14,7,14z",
                    }
                    rect {
                        width: "4",
                        x: "5",
                        y: "6",
                        height: "1",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M19,10.35V5h-5v2h3v2.65L13.52,14H10V9H6c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,10.35z M7,17 c-0.55,0-1-0.45-1-1h2C8,16.55,7.55,17,7,17z",
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
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M14.83,7C14.93,6.68,15,6.34,15,5.99c0-1.23-0.65-2.38-1.71-3.01L10,1L6.71,2.98C5.65,3.61,5,4.76,5,5.99 C5,6.34,5.07,6.68,5.17,7",
                    }
                    path {
                        d: "M19.75,5.5c0-0.83-1.5-2.5-1.5-2.5s-1.5,1.67-1.5,2.5c0,0.55,0.3,1.03,0.75,1.29V11H16V8H4c0,0,0,0,0,0v3H2.5V6.79 C2.95,6.53,3.25,6.05,3.25,5.5c0-0.83-1.5-2.5-1.5-2.5s-1.5,1.67-1.5,2.5c0,0.55,0.3,1.03,0.75,1.29V17h7v-5h4v5h7V6.79 C19.45,6.53,19.75,6.05,19.75,5.5z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M6.12,8C6,8,18,8,17.88,8",
                    }
                    path {
                        d: "M17.88,8C17.95,7.71,18,7.41,18,7.09c0-1.31-0.65-2.53-1.74-3.25L12,1L7.74,3.84C6.65,4.56,6,5.78,6,7.09 C6,7.41,6.05,7.71,6.12,8",
                    }
                    path {
                        d: "M24,7c0-1.1-2-3-2-3s-2,1.9-2,3c0,0.74,0.4,1.38,1,1.72V13h-2V9H5v4H3V8.72C3.6,8.38,4,7.74,4,7c0-1.1-2-3-2-3S0,5.9,0,7 c0,0.74,0.4,1.38,1,1.72V21h9v-6h4v6h9V8.72C23.6,8.38,24,7.74,24,7z",
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
                width: "20",
                height: "20",
                fill: "none",
            }
            path {
                d: "M16.03,8.03l-3.23,3.23c-0.98,0.98-2.56,0.98-3.54,0l-0.7-0.7c-0.39-0.39-1.02-0.39-1.41,0l-4.09,4.09L2,13.59L6.08,9.5 c0.98-0.98,2.56-0.98,3.54,0l0.71,0.71c0.39,0.39,1.02,0.39,1.41,0l3.23-3.23L13,5h5v5L16.03,8.03z",
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
                d: "M19.71,9.71L22,12V6h-6l2.29,2.29l-4.17,4.17c-0.39,0.39-1.02,0.39-1.41,0l-1.17-1.17c-1.17-1.17-3.07-1.17-4.24,0L2,16.59 L3.41,18l5.29-5.29c0.39-0.39,1.02-0.39,1.41,0l1.17,1.17c1.17,1.17,3.07,1.17,4.24,0L19.71,9.71z",
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
                    d: "M17,4l4,4l-4,4V9h-4V7h4V4z M7,17h4v-2H7v-3l-4,4l4,4V17z M19,15h-2v2h2V15z M15,15h-2v2h2V15z M11,7H9v2h2V7z M7,7H5v2h2 V7z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M10,3.03L3,8h0.02v1H5v7H3.02v1h13.97v-1H15V9h1.98V8H17L10,3.03z M13,15h-1v-3.67L10,14l-2-2.67V15H7V9h1l2,3l2-3h1V15z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M22,11V9L12,2L2,9v2h2v9H2v2h20v-2h-2v-9H22z M16,18h-2v-4l-2,3l-2-3v4H8v-7h2l2,3l2-3h2V18z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M12 2L4.5 20.29l.71.71L12 18l6.79 3 .71-.71L12 2z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M21 3L3 10.53v.98l6.84 2.65L12.48 21h.98L21 3z",
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
                d: "M12,6.34L21,3l-3.34,9L12,6.34z M22.61,19.78L4.22,1.39L2.81,2.81l5.07,5.07L3,9.69v1.41l7.07,2.83L12.9,21h1.41l1.81-4.88 l5.07,5.07L22.61,19.78z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
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
                    width: "24",
                    height: "24",
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
            path {
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M12.01 16a.99.99 0 001-1 .99.99 0 00-1-1c-.28 0-.51.1-.71.29-.2.19-.3.43-.3.7s.1.51.29.71c.2.2.44.3.72.3zm-.88-3.66V13h1.85v-.42c0-.33.06-.6.18-.81.12-.21.33-.47.65-.77.4-.38.68-.75.89-1.09.19-.35.3-.76.3-1.25s-.13-.94-.39-1.35a2.57 2.57 0 00-1.05-.96C13.11 6.12 12.58 6 12 6c-.78 0-1.51.32-2.03.79C9.46 7.27 9 7.99 9 9h1.68c0-.52.19-.77.4-.98.21-.2.58-.3.96-.3.35 0 .64.1.85.3.21.2.32.45.32.74 0 .24-.06.46-.19.64-.13.19-.33.41-.61.66-.48.42-.81.79-1 1.12-.19.33-.28.71-.28 1.16zM12 2c4.2 0 8 3.22 8 8.2 0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8C4 5.22 7.8 2 12 2z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M11.87,1l1.06,1.06L9.4,5.6L7.28,3.47l1.06-1.06L9.4,3.47L11.87,1z M17,12v7h-2v-1.5H5V19H3l0-7l2-5h10L17,12z M6.02,8.5 l-1,2.5h9.97l-1-2.5H6.02z M6.5,13.25c-0.55,0-1,0.45-1,1s0.45,1,1,1c0.55,0,1-0.45,1-1S7.05,13.25,6.5,13.25z M13.5,13.25 c-0.55,0-1,0.45-1,1s0.45,1,1,1c0.55,0,1-0.45,1-1S14.05,13.25,13.5,13.25z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18.57,8H5.43L3,15v9h3v-2h12v2h3v-9L18.57,8z M6.85,10h10.29l1.04,3H5.81L6.85,10z M6,17.5C6,16.67,6.67,16,7.5,16 S9,16.67,9,17.5S8.33,19,7.5,19S6,18.33,6,17.5z M15,17.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S17.33,19,16.5,19 S15,18.33,15,17.5z M12,6.36L9.17,3.54l1.41-1.41L12,3.54L15.54,0l1.41,1.41L12,6.36z",
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
                fill: "none",
                height: "24",
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
                fill: "none",
                width: "24",
                height: "24",
            }
            path {
                d: "M5.78,2.95C7.24,2.16,9.48,2,12,2c4.42,0,8,0.5,8,4v10c0,0.35-0.08,0.67-0.19,0.98L13.83,11H18V6H8.83L5.78,2.95z M19.78,22.61L18.17,21H16v-2H8v2H5v-2.78C4.39,17.67,4,16.88,4,16V6.83L1.39,4.22l1.41-1.41l18.38,18.38L19.78,22.61z M9,15.5 C9,14.67,8.33,14,7.5,14S6,14.67,6,15.5C6,16.33,6.67,17,7.5,17S9,16.33,9,15.5z M8.17,11L6,8.83V11H8.17z",
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
                polygon {
                    points: "13.82,10 15,10 10,2 5,10 6.18,10 3,15 8.5,15 8.5,18 11.5,18 11.5,15 17,15",
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
                        polygon {
                            points: "17,12 19,12 12,2 5.05,12 7,12 3.1,18 10.02,18 10.02,22 13.98,22 13.98,18 21,18",
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
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M14.82,9L13,4h-3v1h2.3l1.46,4H8.75L8.38,8H10V7H6v1h1.32l1.46,4H7.95C7.7,10.19,6.12,8.86,4.2,9.01 c-1.64,0.13-3.01,1.46-3.18,3.1C0.8,14.25,2.41,16,4.5,16c1.79,0,3.21-1.29,3.45-3h4.1c0.25,1.81,1.83,3.14,3.75,2.99 c1.64-0.13,3.01-1.46,3.18-3.1C19.2,10.75,17.59,9,15.5,9H14.82z M9.11,10h3.92c-0.53,0.52-0.88,1.22-0.98,2H9.84L9.11,10z M4.5,15 C3.1,15,2,13.9,2,12.5S3.1,10,4.5,10c1.23,0,2.23,0.85,2.45,2H4v1h2.95C6.73,14.15,5.73,15,4.5,15z M15.5,15 c-1.4,0-2.5-1.1-2.5-2.5c0-0.94,0.5-1.73,1.24-2.16l1.03,2.83l0.94-0.34l-1.02-2.8C15.3,10.02,15.4,10,15.5,10 c1.4,0,2.5,1.1,2.5,2.5S16.9,15,15.5,15z",
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
            }
            g {
                path {
                    d: "M18.18,10L16,4h-4v2h2.6l1.46,4h-4.81l-0.36-1H12V7H7v2h1.75l1.82,5H9.9c-0.44-2.23-2.31-3.88-4.65-3.99 C2.45,9.87,0,12.2,0,15c0,2.8,2.2,5,5,5c2.46,0,4.45-1.69,4.9-4h4.2c0.44,2.23,2.31,3.88,4.65,3.99c2.8,0.13,5.25-2.19,5.25-5 c0-2.8-2.2-5-5-5H18.18z M7.82,16c-0.4,1.17-1.49,2-2.82,2c-1.68,0-3-1.32-3-3s1.32-3,3-3c1.33,0,2.42,0.83,2.82,2H5v2H7.82z M14.1,14h-1.4l-0.73-2H15C14.56,12.58,14.24,13.25,14.1,14z M19,18c-1.68,0-3-1.32-3-3c0-0.93,0.41-1.73,1.05-2.28l0.96,2.64 l1.88-0.68l-0.97-2.67c0.03,0,0.06-0.01,0.09-0.01c1.68,0,3,1.32,3,3S20.68,18,19,18z",
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
                d: "M21 2H3v18h6l3 3 3-3h6V2zm-9 3.3c1.49 0 2.7 1.21 2.7 2.7s-1.21 2.7-2.7 2.7S9.3 9.49 9.3 8s1.21-2.7 2.7-2.7zM18 16H6v-.9c0-2 4-3.1 6-3.1s6 1.1 6 3.1v.9z",
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
                d: "M12 11c-1.33 0-4 .67-4 2v.16c.97 1.12 2.4 1.84 4 1.84s3.03-.72 4-1.84V13c0-1.33-2.67-2-4-2zm0-1c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0-8c4.2 0 8 3.22 8 8.2 0 3.32-2.67 7.25-8 11.8-5.33-4.55-8-8.48-8-11.8C4 5.22 7.8 2 12 2z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M16,12v-1h-2.04c-0.04-0.38-0.11-0.74-0.22-1.08l1.71-0.99l-0.5-0.87L13.33,9c-0.24-0.4-0.54-0.74-0.87-1.03 c0.07-0.39,0.13-1.19-0.48-1.99l1.24-1.24l-0.71-0.71l-1.29,1.29c-0.41-0.23-1.35-0.61-2.43,0L7.49,4.04L6.78,4.74l1.24,1.24 C7.41,6.78,7.47,7.58,7.55,7.97C7.21,8.26,6.91,8.6,6.67,9L5.05,8.07l-0.5,0.87l1.71,0.99c-0.11,0.34-0.18,0.7-0.22,1.08H4v1h2.04 c0.04,0.38,0.11,0.74,0.22,1.08l-1.71,0.99l0.5,0.87L6.67,14c0.72,1.21,1.94,2,3.33,2s2.61-0.8,3.33-2l1.62,0.94l0.5-0.87 l-1.71-0.99c0.11-0.34,0.18-0.7,0.22-1.08H16z M10.5,13.5h-1v-4h1V13.5z",
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
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M21,15v-2h-3.07c-0.05-0.39-0.12-0.77-0.22-1.14l2.58-1.49l-1-1.73L16.92,10c-0.28-0.48-0.62-0.91-0.99-1.29 C15.97,8.48,16,8.25,16,8c0-0.8-0.24-1.55-0.65-2.18L17,4.17l-1.41-1.41l-1.72,1.72c-1.68-0.89-3.1-0.33-3.73,0L8.41,2.76L7,4.17 l1.65,1.65C8.24,6.45,8,7.2,8,8c0,0.25,0.03,0.48,0.07,0.72C7.7,9.1,7.36,9.53,7.08,10L4.71,8.63l-1,1.73l2.58,1.49 c-0.1,0.37-0.17,0.75-0.22,1.14H3v2h3.07c0.05,0.39,0.12,0.77,0.22,1.14l-2.58,1.49l1,1.73L7.08,18c1.08,1.81,2.88,3,4.92,3 s3.84-1.19,4.92-3l2.37,1.37l1-1.73l-2.58-1.49c0.1-0.37,0.17-0.75,0.22-1.14H21z M13,17h-2v-6h2V17z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M16.57,13.59l-1.63-1.55C15.31,10.33,13.96,9,12.5,9C11.32,9,10,9.96,10,11.5c0,0.69,0.28,1.32,0.73,1.77l-0.71,0.71 C9.39,13.34,9,12.47,9,11.5c0-1.36,0.78-2.52,1.91-3.1C10.43,8.15,9.91,8,9.36,8C6.44,8,6,10.79,6,11.47c-1.12-0.13-2-1.07-2-2.22 C4,8.01,5.01,7,6.25,7H8.5C9.33,7,10,6.33,10,5.5S9.33,4,8.5,4H7v1h1.5C8.78,5,9,5.22,9,5.5C9,5.78,8.78,6,8.5,6H6.25 C4.46,6,3,7.46,3,9.25c0,1.71,1.32,3.1,3,3.22C6,13.07,6.34,16,9.36,16h6.24C16.86,16,17.47,14.45,16.57,13.59z M14.25,15 c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75S15,13.84,15,14.25C15,14.66,14.66,15,14.25,15z",
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
                path {
                    d: "M21.31,17.38l-2.39-2.13C19.44,12.89,17.56,11,15.5,11c-1.16,0-3.5,0.9-3.5,3.5c0,0.97,0.39,1.84,1.03,2.47l-0.71,0.71 C11.5,16.87,11,15.74,11,14.5c0-1.7,0.96-3.17,2.35-3.93c-0.7-0.36-1.48-0.57-2.28-0.57c-2.38,0-4.37,1.65-4.91,3.87 C4.91,13.5,4,12.36,4,11c0-1.66,1.34-3,3-3c0.94,0,1.56,0,2.5,0C10.88,8,12,6.88,12,5.5C12,4.12,10.88,3,9.5,3H7v2h2.5 C9.78,5,10,5.22,10,5.5C10,5.78,9.78,6,9.5,6C9.47,6,9,6,7,6c-2.76,0-5,2.24-5,5c0,2.42,1.72,4.44,4,4.9v0.03 C6,18.73,8.27,21,11.07,21h8.86C21.8,21,22.74,18.66,21.31,17.38z M18,19c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C19,18.55,18.55,19,18,19z",
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
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M5 20h14v2H5v-2zm7-13c-1.1 0-2 .9-2 2s.9 2 2 2a2 2 0 100-4zm0-5c3.27 0 7 2.46 7 7.15 0 3.12-2.33 6.41-7 9.85-4.67-3.44-7-6.73-7-9.85C5 4.46 8.73 2 12 2z",
                fill_rule: "evenodd",
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
                d: "M12 12c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm0-10c-4.2 0-8 3.22-8 8.2 0 3.32 2.67 7.25 8 11.8 5.33-4.55 8-8.48 8-11.8C20 5.22 16.2 2 12 2z",
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
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M13.6,4.34l2.83,2.83c0.78-0.78,0.78-2.05,0-2.83L13.6,1.51l-3.54,3.54l1.41,1.41L13.6,4.34z",
                    }
                    rect {
                        width: "4",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -5.1599 7.5429)",
                        height: "2",
                        x: "4.53",
                        y: "9",
                    }
                    path {
                        d: "M12.89,6.46l-1.41,1.41L7.94,4.34L6.53,5.76l3.54,3.54l-7.07,7.07l2.12,2.12L13.6,10l0.71,0.71l1.41-1.41L12.89,6.46z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M16.16,5.64l3.54,3.54c1.17-1.17,1.17-3.07,0-4.24l-3.54-3.54l-4.24,4.24l2.12,2.12L16.16,5.64z",
                    }
                    rect {
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -6.2383 8.9393)",
                        width: "5",
                        x: "5.17",
                        y: "10.5",
                        height: "3",
                    }
                    path {
                        d: "M15.45,7.76l-1.41,1.41L9.79,4.93L7.67,7.05l4.24,4.24l-8.49,8.49l2.83,2.83L16.86,12l0.71,0.71l1.41-1.41L15.45,7.76z",
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M4,11V8h7.29c-0.77-2.6,0.21-4.61,0.37-4.97C2.97,2.67,2,5.02,2,7v9.5C2,18.43,3.57,20,5.5,20L4,21v1h12v-1l-1.5-1 c1.93,0,3.5-1.57,3.5-3.5V13c-1.91,0-3.63-0.76-4.89-2H4z M10,17c-0.83,0-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5C11.5,16.33,10.83,17,10,17z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18,3.01V2L4,3.54V9H2c0,3.36,2.07,6.23,5,7.41V18h6v-1.59c2.93-1.19,5-4.06,5-7.41H9V7h9V6H9V3.99L18,3.01z M5,4.43 l1-0.11V6H5V4.43z M5,7h1v2H5V7z M8,9H7V7h1V9z M8,6H7V4.21L8,4.1V6z",
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M22,3.51V2L4,3.99V12H2c0,3.69,2.47,6.86,6,8.25V22h8v-1.75c3.53-1.39,6-4.56,6-8.25H10.5V8H22V6.5H10.5V4.78L22,3.51z M6.5,5.22V6.5h-1V5.34L6.5,5.22z M5.5,8h1v4h-1V8z M9,12H8V8h1V12z M9,6.5H8V5.06l1-0.11V6.5z",
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
                    width: "20",
                    fill: "none",
                    height: "20",
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
                    height: "20",
                    fill: "none",
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
                    fill: "none",
                    width: "24",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M22 2H2v20l4-4h16V2zM6 14v-2.47l6.88-6.88c.2-.2.51-.2.71 0l1.77 1.77c.2.2.2.51 0 .71L8.47 14H6zm12 0h-7.5l2-2H18v2z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        width: "1.5",
                        x: "4",
                        y: "4",
                        height: "12",
                    }
                    rect {
                        y: "4",
                        width: "1.5",
                        x: "14.5",
                        height: "6.5",
                    }
                    rect {
                        y: "8.5",
                        width: "1.5",
                        height: "3",
                        x: "9.25",
                    }
                    rect {
                        x: "9.25",
                        y: "4",
                        width: "1.5",
                        height: "3",
                    }
                    rect {
                        x: "9.25",
                        y: "13",
                        height: "3",
                        width: "1.5",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    rect {
                        x: "18",
                        y: "4",
                        width: "2",
                        height: "9",
                    }
                    rect {
                        x: "4",
                        height: "16",
                        width: "2",
                        y: "4",
                    }
                    rect {
                        height: "4",
                        y: "4",
                        width: "2",
                        x: "11",
                    }
                    rect {
                        height: "4",
                        width: "2",
                        x: "11",
                        y: "10",
                    }
                    rect {
                        width: "2",
                        y: "16",
                        height: "4",
                        x: "11",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M13.25,3c-2.37,0-4.33,1.73-4.69,4L4.87,7l1.19-1.19L5,4.75l-3,3l3,3l1.06-1.06L4.87,8.5L10,8.46c0,0,0-0.12,0-0.71 c0-1.79,1.46-3.25,3.25-3.25s3.25,1.46,3.25,3.25S15.04,11,13.25,11H12.5v6H14v-4.56l0,0c2.27-0.36,4-2.32,4-4.69 C18,5.13,15.87,3,13.25,3",
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
                    d: "M16,13c2.21,0,4-1.79,4-4s-1.79-4-4-4c-2.21,0-4,1.79-4,4l0,1l-6.17,0l1.59,1.59L6,13L2,9l4-4l1.41,1.41L5.83,8l4.25,0 c0.48-2.84,2.94-5,5.92-5c3.31,0,6,2.69,6,6c0,2.97-2.16,5.44-5,5.92L17,21h-2l0-8L16,13z",
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
                    d: "M6.75,3c2.37,0,4.33,1.73,4.69,4l3.69,0l-1.19-1.19L15,4.75l3,3l-3,3l-1.06-1.06l1.19-1.19L10,8.46c0,0,0-0.12,0-0.71 C10,5.96,8.54,4.5,6.75,4.5S3.5,5.96,3.5,7.75S4.96,11,6.75,11H7.5v6H6v-4.56l0,0c-2.27-0.36-4-2.32-4-4.69C2,5.13,4.13,3,6.75,3",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M8,13c-2.21,0-4-1.79-4-4c0-2.21,1.79-4,4-4c2.21,0,4,1.79,4,4l0,1l6.17,0l-1.59,1.59L18,13l4-4l-4-4l-1.41,1.41L18.17,8 l-4.25,0C13.44,5.16,10.97,3,8,3C4.69,3,2,5.69,2,9c0,2.97,2.16,5.44,5,5.92L7,21h2l0-8L8,13z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M15.25,12.13V3h-6v12.5h-3V7.87C7.26,7.55,8,6.62,8,5.5C8,4.12,6.88,3,5.5,3S3,4.12,3,5.5c0,1.12,0.74,2.05,1.75,2.37V17 h6V4.5h3v7.63C12.74,12.45,12,13.38,12,14.5c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5C17,13.38,16.26,12.45,15.25,12.13z",
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
                        d: "M19,15.18V3h-8v16H7V8.82C8.16,8.4,9,7.3,9,6c0-1.66-1.34-3-3-3S3,4.34,3,6c0,1.3,0.84,2.4,2,2.82V21h8V5h4v10.18 c-1.16,0.41-2,1.51-2,2.82c0,1.66,1.34,3,3,3s3-1.34,3-3C21,16.7,20.16,15.6,19,15.18z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C17,6.13,13.87,3,10,3z M11,5c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1 s-1-0.45-1-1C10,5.45,10.45,5,11,5z M13.5,10.62c-0.6,0-1.87-0.38-2.67-1.4l-0.21,1.19L12,11.79V15h-1v-2.79l-1-0.99l-0.42,2.13 l-3.17-0.65l0.2-0.98l2.19,0.45l0.62-3.12L8.5,9.37v1.25h-1V8.67l2.67-0.96c0.46-0.16,0.99,0.01,1.22,0.44 c0.7,1.32,1.77,1.47,2.11,1.47V10.62z",
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
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M13.5,6c0.55,0,1,0.45,1,1 c0,0.55-0.45,1-1,1s-1-0.45-1-1C12.5,6.45,12.95,6,13.5,6z M16,12c-0.7,0-2.01-0.54-2.91-1.76l-0.41,2.35L14,14.03V18h-1v-3.58 l-1.11-1.21l-0.52,2.64L7.6,15.08l0.2-0.98l2.78,0.57l0.96-4.89L10,10.35V12H9V9.65l3.28-1.21c0.49-0.18,1.03,0.06,1.26,0.53 C14.37,10.67,15.59,11,16,11V12z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M10,2L3.5,4.5v4.77c0,4.04,2.77,7.81,6.5,8.73c3.73-0.92,6.5-4.69,6.5-8.73V4.5L10,2z M10,14c-2.21,0-4-1.79-4-4 c0-2.21,1.79-4,4-4s4,1.79,4,4C14,12.21,12.21,14,10,14z M11.15,11.85L9.5,10.21V8h1v1.79l1.35,1.35L11.15,11.85z",
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
                    d: "M12,2L4,5v6.09c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91V5L12,2z M12,17c-2.76,0-5-2.24-5-5s2.24-5,5-5 s5,2.24,5,5S14.76,17,12,17z M13.65,14.35l-2.15-2.15V9h1v2.79l1.85,1.85L13.65,14.35z",
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
                height: "20",
                width: "20",
            }
            path {
                d: "M9.5,11.5V2L3,11.5H9.5z M17,11.5C17,5.5,12.03,1,10.5,1c0,0,0.8,2.5,0.8,5.5s-0.8,5-0.8,5H17z M17.5,17.5H18l0,1.5h-0.5 c-0.86,0-1.71-0.2-2.5-0.6c-1.58,0.8-3.43,0.8-5,0c-1.58,0.8-3.42,0.8-5,0C4.21,18.8,3.36,19,2.5,19H2v-1.5h0.5 c0.87,0,1.74-0.33,2.5-1c1.53,1.33,3.47,1.33,5,0c1.53,1.33,3.48,1.33,5,0C15.76,17.17,16.63,17.5,17.5,17.5z M16.08,15.55 c-0.41-0.27-0.78-0.64-1.08-1.05c-0.61,0.84-1.5,1.5-2.5,1.5s-1.89-0.66-2.5-1.5C9.39,15.34,8.5,16,7.5,16S5.61,15.34,5,14.5 c-0.3,0.41-0.67,0.78-1.08,1.05C2.94,14.83,2.24,13.76,2,12.5h16C17.76,13.76,17.06,14.83,16.08,15.55z",
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
                height: "24",
                fill: "none",
                width: "24",
            }
            path {
                d: "M11,13.5V2L3,13.5H11z M21,13.5C21,6.5,14.5,1,12.5,1c0,0,1,3,1,6.5s-1,6-1,6H21z M22,15H2c0.31,1.53,1.16,2.84,2.33,3.73 C4.98,18.46,5.55,18.01,6,17.5C6.73,18.34,7.8,19,9,19s2.27-0.66,3-1.5c0.73,0.84,1.8,1.5,3,1.5s2.26-0.66,3-1.5 c0.45,0.51,1.02,0.96,1.67,1.23C20.84,17.84,21.69,16.53,22,15z M22,23v-2h-1c-1.04,0-2.08-0.35-3-1c-1.83,1.3-4.17,1.3-6,0 c-1.83,1.3-4.17,1.3-6,0c-0.91,0.65-1.96,1-3,1H2l0,2h1c1.03,0,2.05-0.25,3-0.75c1.89,1,4.11,1,6,0c1.89,1,4.11,1,6,0h0 c0.95,0.5,1.97,0.75,3,0.75H22z",
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
                d: "M21 3H3v18h18V3zM5 4.99h3C8 6.65 6.66 8 5 8V4.99zM5 12v-2c2.76 0 5-2.25 5-5.01h2C12 8.86 8.87 12 5 12zm0 6l3.5-4.5 2.5 3.01L14.5 12l4.5 6H5z",
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
                    d: "M4.56,7.5H6.5V9H2V4.5h1.5v1.94l5.06-5.06L16.18,9h-2.12l-5.5-5.5L4.56,7.5z M15.44,12.5H13.5V11H18v4.5h-1.5v-1.94 l-5.06,5.06L3.82,11h2.12l5.5,5.5L15.44,12.5z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M4,7.59l6.41-6.41L20.24,11h-2.83L10.4,4L5.41,9H8v2H2V5h2V7.59z M20,19h2v-6h-6v2h2.59l-4.99,5l-7.01-7H3.76l9.83,9.83 L20,16.41V19z",
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
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M21.05,17.56L3.08,18.5L3,17l17.98-0.94L21.05,17.56z M21,19.48H3v1.5h18V19.48z M22,3v11H2V3H22z M20,6 c-1.68,0-3.04,0.98-3.21,2.23C16.15,7.5,14.06,5.5,10.25,5.5c-4.67,0-6.75,3-6.75,3s2.08,3,6.75,3c3.81,0,5.9-2,6.54-2.73 C16.96,10.02,18.32,11,20,11V6z",
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
                    width: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M10,2C9.59,2,9.25,2.34,9.25,2.75V3.5H5c-0.55,0-1,0.45-1,1V7c0,0.55,0.45,1,1,1h4.25v1.5h-3.8 c-0.29,0-0.56,0.12-0.75,0.34l-1.11,1.25c-0.34,0.38-0.34,0.95,0,1.33l1.11,1.25C4.89,13.88,5.16,14,5.45,14h3.8v3.25 C9.25,17.66,9.59,18,10,18c0.41,0,0.75-0.34,0.75-0.75V14H15c0.55,0,1-0.45,1-1v-2.5c0-0.55-0.45-1-1-1h-4.25V8h3.8 c0.29,0,0.56-0.12,0.75-0.34l1.11-1.25c0.34-0.38,0.34-0.95,0-1.33L15.3,3.84c-0.19-0.21-0.46-0.34-0.75-0.34h-3.8V2.75 C10.75,2.34,10.41,2,10,2z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M12,2c-0.55,0-1,0.45-1,1v1H5.5C4.67,4,4,4.67,4,5.5v3C4,9.33,4.67,10,5.5,10H11v2H6.62c-0.4,0-0.78,0.16-1.06,0.44 l-1.5,1.5c-0.59,0.59-0.59,1.54,0,2.12l1.5,1.5C5.84,17.84,6.22,18,6.62,18H11v3c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-3h5.5 c0.83,0,1.5-0.67,1.5-1.5v-3c0-0.83-0.67-1.5-1.5-1.5H13v-2h4.38c0.4,0,0.78-0.16,1.06-0.44l1.5-1.5c0.59-0.59,0.59-1.54,0-2.12 l-1.5-1.5C18.16,4.16,17.78,4,17.38,4H13V3C13,2.45,12.55,2,12,2z",
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
                fill: "none",
                height: "20",
            }
            path {
                d: "M18.5,14.5c0,0.55-0.45,1-1,1l-2.22-2.19C17.25,13,19,12,19,12c-1-1.5-7.5-7.5-7.5-7.5H9V6h1.91l1.22,1.14L9.1,9L1.25,8.5 L0,11l4.32,1.27l-3.53,1.91C-0.58,14.92-0.06,17,1.5,17h5c1.93,0,3.5-1.57,3.5-3.5h3.33l2.05,2H12.5V17h5c1.38,0,2.5-1.12,2.5-2.5 H18.5z M6.5,15.5h-5l4.87-2.63L8.5,13.5C8.5,14.6,7.6,15.5,6.5,15.5z",
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
                fill: "none",
                width: "24",
                height: "24",
            }
            path {
                d: "M22,17c0,0.55-0.45,1-1,1h-0.17l-2.2-2.2C21.6,15.18,23,13,23,13l-9-8h-3v2h2.25l1.45,1.3L11,11l-9.5-1L0,13l4.54,1.36 l-3.49,1.88C-0.77,17.22-0.07,20,2,20h6c2.21,0,4-1.79,4-4h4l2,2h-3v2h6c1.66,0,3-1.34,3-3H22z M8,18H2l5.25-2.83L10,16 C10,17.1,9.11,18,8,18z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M7.38,6.5v7h5.25v-7H7.38z M11.12,12H8.88V8h2.25V12z M1.5,12h3v-1.25h-3V6.5H6V8H3v1.25h3v4.25H1.5V12z M14,12h3v-1.25h-3 V6.5h4.5V8h-3v1.25h3v4.25H14V12z",
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
            }
            g {
                path {
                    d: "M15.5,7h-7v10h7V7z M13.5,15h-3V9h3V15z M1,15h4v-2H1V7h6v2H3v2h4v6H1V15z M17,15h4v-2h-4V7h6v2h-4v2h4v6h-6V15z",
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
                    d: "M4.96,11c0,0,0.31-0.42,0.31-1.15c0-0.77-0.77-2.19-0.77-2.79C4.5,6.74,4.54,6.42,4.81,6h0.98 C5.53,6.42,5.48,6.74,5.48,7.06c0,0.59,0.77,2.02,0.77,2.79c0,0.73-0.31,1.15-0.31,1.15H4.96z M8.32,11c0,0,0.31-0.42,0.31-1.15 c0-0.77-0.77-2.19-0.77-2.79C7.86,6.74,7.9,6.42,8.16,6H7.18C6.92,6.42,6.88,6.74,6.88,7.06c0,0.59,0.77,2.02,0.77,2.79 c0,0.73-0.31,1.15-0.31,1.15H8.32z M10.69,11c0,0,0.31-0.42,0.31-1.15c0-0.77-0.77-2.19-0.77-2.79c0-0.31,0.04-0.64,0.31-1.06H9.56 C9.29,6.42,9.25,6.74,9.25,7.06c0,0.59,0.77,2.02,0.77,2.79c0,0.73-0.31,1.15-0.31,1.15H10.69z M17.68,5.49 c0,0,0.06-0.42,0.07-1.12c0.01-1.09-0.71-2.1-1.78-2.32C14.61,1.77,13.4,2.73,13.26,4l-0.94,8.4l-9.34,0C2.97,15.49,5.42,18,8.4,18 c2.78,0,5.07-2.18,5.36-4.99l0.99-8.84c0.04-0.38,0.36-0.67,0.75-0.67c0.41,0,0.75,0.34,0.75,0.75S16.2,5.3,16.2,5.3L17.68,5.49z",
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
                width: "24",
                height: "24",
                fill: "none",
            }
            path {
                d: "M6.4,7C6.06,7.55,6,7.97,6,8.38C6,9.15,7,11,7,12c0,0.95-0.4,1.5-0.4,1.5H5.1c0,0,0.4-0.55,0.4-1.5c0-1-1-2.85-1-3.62 C4.5,7.97,4.56,7.55,4.9,7H6.4z M11.4,7C11.06,7.55,11,7.97,11,8.38C11,9.15,12,11,12,12c0,0.95-0.4,1.5-0.4,1.5h1.5 c0,0,0.4-0.55,0.4-1.5c0-1-1-2.85-1-3.62c0-0.41,0.06-0.83,0.4-1.38H11.4z M8.15,7c-0.34,0.55-0.4,0.97-0.4,1.38 c0,0.77,1,2.63,1,3.62c0,0.95-0.4,1.5-0.4,1.5h1.5c0,0,0.4-0.55,0.4-1.5c0-1-1-2.85-1-3.62c0-0.41,0.06-0.83,0.4-1.38H8.15z M21.47,6.5c0,0,0.13-1.06,0.13-1.5c0-1.65-1.35-3-3-3c-1.54,0-2.81,1.16-2.98,2.65L14.53,15l-11.6,0c-0.02,3.87,3.09,7,6.82,7 c3.48,0,6.34-2.73,6.71-6.23l1.15-10.87C17.66,4.39,18.08,4,18.6,4c0.55,0,1,0.45,1,1c0,0.3-0.1,1.25-0.1,1.25L21.47,6.5z",
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
                    width: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M6,4.5L3,6V3L6,4.5z M15,3v3l3-1.5L15,3z M9,2v3l3-1.5L9,2z M18,8.25c0,0,0,5.87,0,7.25c0,1.11-2.31,2.06-5.5,2.39l0-3.89 h-5l0,3.89C4.31,17.56,2,16.61,2,15.5c0-0.99,0-7.25,0-7.25C2,7.01,5.58,6,10,6S18,7.01,18,8.25z M4.4,8.3C5.51,8.65,7.42,9,10,9 s4.49-0.35,5.6-0.7c0-0.21-2.38-0.8-5.6-0.8S4.4,8.09,4.4,8.3z",
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
                    d: "M7,5L3,7V3L7,5z M18,3v4l4-2L18,3z M11,2v4l4-2L11,2z M5,10.04C6.38,10.53,8.77,11,12,11s5.62-0.47,7-0.96 C19,9.86,16.22,9,12,9S5,9.86,5,10.04z M15,17H9l0,4.88C4.94,21.49,2,20.34,2,19v-9c0-1.66,4.48-3,10-3s10,1.34,10,3v9 c0,1.34-2.94,2.48-7,2.87L15,17z",
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
                d: "M20 4H4v2h16V4zm1 10v-2l-1-5H4l-1 5v2h1v6h10v-6h4v6h2v-6h1zm-9 4H6v-4h6v4z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
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
                    width: "24",
                    fill: "none",
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
            circle {
                r: "1",
                cy: "16",
                cx: "8.5",
            }
            circle {
                cy: "16",
                r: "1",
                cx: "15.5",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M5,7v10h3v-5h4v5h3V7l-5-4L5,7z M11.25,8.25c0,0.69-0.56,1.25-1.25,1.25S8.75,8.94,8.75,8.25S9.31,7,10,7 S11.25,7.56,11.25,8.25z",
                    }
                    path {
                        d: "M17.5,4C16.67,4,16,4.67,16,5.5v1.17h3V5.5C19,4.67,18.33,4,17.5,4z",
                    }
                    rect {
                        width: "3",
                        y: "7.5",
                        height: "9.5",
                        x: "16",
                    }
                    path {
                        d: "M2.5,4C1.67,4,1,4.67,1,5.5v1.17h3V5.5C4,4.67,3.33,4,2.5,4z",
                    }
                    rect {
                        y: "7.5",
                        height: "9.5",
                        width: "3",
                        x: "1",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M6,8v13h4v-7h4v7h4V8l-6-5L6,8z M13.5,10c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5S13.5,9.17,13.5,10z",
                    }
                    path {
                        d: "M3,5C1.9,5,1,5.9,1,7v1h4V7C5,5.9,4.1,5,3,5z",
                    }
                    rect {
                        x: "1",
                        y: "9",
                        width: "4",
                        height: "12",
                    }
                    path {
                        d: "M21,5c-1.1,0-2,0.9-2,2v1h4V7C23,5.9,22.1,5,21,5z",
                    }
                    rect {
                        x: "19",
                        height: "12",
                        y: "9",
                        width: "4",
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
                g {
                    polygon {
                        points: "5.5,16 14.5,16 14.8,10 5.2,10",
                    }
                    polygon {
                        points: "16.44,5.5 14.95,6.99 15,6 12,3 8,3 5,6 5.05,6.99 3.56,5.5 2.5,6.56 4.94,9 15.06,9 17.5,6.56",
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
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    polygon {
                        points: "22,7.46 20.59,6.05 19,7.63 19.03,7.07 14.98,3 9.02,3 4.97,7.07 5,7.57 3.41,6.01 2,7.44 4.66,10 19.35,10",
                    }
                    polygon {
                        points: "5.93,20 18.07,20 18.7,11.55 5.3,11.55",
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
                            d: "M18,13c-1.91,0-3.63-0.76-4.89-2H4.81l1.04-3h5.44C11.1,7.37,11,6.7,11,6s0.1-1.37,0.29-2H8v2H4.43L2,13v9h3v-2h12v2h3 v-9l-0.09-0.27C19.3,12.9,18.66,13,18,13z M6.5,17C5.67,17,5,16.33,5,15.5S5.67,14,6.5,14S8,14.67,8,15.5S7.33,17,6.5,17z M15.5,17c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.33,17,15.5,17z",
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
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M17.5,7.01C17.5,8.11,16.61,9,15.52,9H4.48C3.39,9,2.5,8.11,2.5,7.01H1c0,1.76,1.31,3.2,3,3.44V18h4v-4h4v4h4v-7.55 c0.55-0.08,3-0.77,3-3.45L17.5,7.01z",
                        }
                    }
                    g {
                        polygon {
                            points: "10,1 6.92,5 13.08,5",
                        }
                    }
                    g {
                        path {
                            d: "M5,7.16V8h10V7.16c1.89-0.9,2-2.74,2-3.16l-1.5,0.01c0,1.08-0.88,1.97-1.96,1.98H6.46C5.38,5.98,4.5,5.1,4.5,4.02L3,4 C3,4.42,3.11,6.26,5,7.16z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M21,9.02c0,1.09-0.89,1.98-1.98,1.98H4.98C3.89,11,3,10.11,3,9.02H1c0,1.86,1.28,3.4,3,3.84V22h6v-5h4v5h6v-9.14 c0.55-0.14,3-1.04,3-3.86L21,9.02z",
                        }
                    }
                    g {
                        path {
                            d: "M6,8.86V10h12V8.86c0.55-0.14,3-1.04,3-3.86l-2,0.02C19,6.11,18.11,7,17.02,7H6.98C5.89,7,5,6.11,5,5.02H3 C3,6.87,4.28,8.42,6,8.86z",
                        }
                    }
                    g {
                        polygon {
                            points: "12,1 8.25,6 15.75,6",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    polygon {
                        points: "13.5,7 6.5,7 5.56,9.5 14.44,9.5",
                    }
                    polygon {
                        points: "12,3 12,1 10.5,1 10.5,3 9.5,3 9.5,1 8,1 8,3 7.06,5.5 12.94,5.5",
                    }
                    polygon {
                        points: "16.5,9 16.5,11 3.5,11 3.5,9 2,9 2,18 8,18 8,14 12,14 12,18 18,18 18,9",
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
            }
            g {
                g {
                    polygon {
                        points: "6.6,11 17.4,11 16.5,8 7.5,8",
                    }
                    polygon {
                        points: "20,11 20,13 4,13 4,11 2,11 2,22 10,22 10,17 14,17 14,22 22,22 22,11",
                    }
                    polygon {
                        points: "15.9,6 15,3 15,1 13,1 13,3 10.97,3 10.97,1 8.97,1 8.97,3.12 8.1,6",
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
                d: "M14 6l-3.75 5 2.85 3.8-1.6 1.2C9.81 13.75 7 10 7 10l-6 8h22L14 6z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M1,13c0,2.76,2.24,5,5,5s5-2.24,5-5V7H1V13z M6,14.51c-1.11,0-2-0.67-2-1.51h4C8,13.83,7.11,14.51,6,14.51z M7.5,9.75 c0.41,0,0.75,0.34,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75C6.75,10.09,7.09,9.75,7.5,9.75z M4.5,9.75 c0.41,0,0.75,0.34,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75C3.75,10.09,4.09,9.75,4.5,9.75z",
                        }
                    }
                    g {
                        path {
                            d: "M9,2v4h2h0.95c-0.12-0.13-0.2-0.31-0.2-0.5c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75 c-0.19,0-0.37-0.08-0.5-0.2V7v2.51C12,8.67,12.89,8,14,8s2,0.67,2,1.51h-4v3.07c0.61,0.27,1.29,0.42,2,0.42c2.76,0,5-2.24,5-5V2 H9z M15.5,6.25c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75C16.25,5.91,15.91,6.25,15.5,6.25z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M11,2v5.5h3.5v3.31C14.55,9.8,15.64,9,16.99,9c1.38,0,2.5,0.84,2.5,1.88H14.5v3.56C15.26,14.8,16.11,15,17,15 c3.31,0,6-2.69,6-6V2H11z M15,7.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S15.55,7.5,15,7.5z M19,7.5c-0.55,0-1-0.45-1-1 s0.45-1,1-1s1,0.45,1,1S19.55,7.5,19,7.5z",
                    }
                    path {
                        d: "M1,16c0,3.31,2.69,6,6,6s6-2.69,6-6V9H1V16z M7,17.88c-1.38,0-2.5-0.84-2.5-1.88h5C9.5,17.04,8.38,17.88,7,17.88z M9,12.5 c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S8.45,12.5,9,12.5z M5,12.5c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1 S4.45,12.5,5,12.5z",
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
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M14,5.75c0,0.41,0.34,0.75,0.75,0.75c0.21,0,0.39-0.08,0.53-0.22C15.58,5.98,16,4.5,16,4.5s-1.48,0.42-1.78,0.72 C14.08,5.36,14,5.54,14,5.75z",
                    }
                    path {
                        d: "M14.75,2C12.68,2,11,3.68,11,5.75c0,1.44,0.81,2.69,2,3.32V10h1v5.5h-1.5V11h-3V4.5C9.5,3.67,8.83,3,8,3H3.5 C2.67,3,2,3.67,2,4.5v11C2,16.33,2.67,17,3.5,17H8c0.83,0,1.5-0.67,1.5-1.5v-3H11V17h4.5v-7h1V9.07c1.19-0.63,2-1.88,2-3.32 C18.5,3.68,16.82,2,14.75,2z M5,15.5L3.5,14v-2.12l1.5,1.5V15.5z M5,11.75l-1.5-1.5V8.13L5,9.62V11.75z M5,8L3.5,6.5V4.38L5,5.88 V8z M8,14l-1.5,1.5v-2.12l1.5-1.5V14z M8,10.25l-1.5,1.5V9.62L8,8.13V10.25z M8,6.5L6.5,8V5.88L8,4.38V6.5z M14.75,8 c-1.24,0-2.25-1.01-2.25-2.25c0-1.24,1.01-2.25,2.25-2.25S17,4.51,17,5.75C17,6.99,15.99,8,14.75,8z",
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
            }
            g {
                g {
                    path {
                        d: "M18,7c0,0.55,0.45,1,1,1c0.28,0,0.53-0.11,0.71-0.29c0.4-0.4,1.04-2.46,1.04-2.46s-2.06,0.64-2.46,1.04 C18.11,6.47,18,6.72,18,7z",
                    }
                    path {
                        d: "M19,2c-2.76,0-5,2.24-5,5c0,2.05,1.23,3.81,3,4.58V13h1v6h-2v-6h-4V5c0-1.1-0.9-2-2-2H4C2.9,3,2,3.9,2,5v14 c0,1.1,0.9,2,2,2h6c1.1,0,2-0.9,2-2v-4h2v6h6v-8h1v-1.42c1.77-0.77,3-2.53,3-4.58C24,4.24,21.76,2,19,2z M6,19.5l-2-2v-2.83l2,2 V19.5z M6,14.5l-2-2V9.67l2,2V14.5z M6,9.5l-2-2V4.67l2,2V9.5z M10,17.5l-2,2v-2.83l2-2V17.5z M10,12.5l-2,2v-2.83l2-2V12.5z M10,7.5l-2,2V6.67l2-2V7.5z M19,10c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S20.66,10,19,10z",
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
                d: "M20 10h-3V8.86c1.72-.45 3-2 3-3.86h-3V3H7v2H4c0 1.86 1.28 3.41 3 3.86V10H4c0 1.86 1.28 3.41 3 3.86V15H4c0 1.86 1.28 3.41 3 3.86V21h10v-2.14c1.72-.45 3-2 3-3.86h-3v-1.14c1.72-.45 3-2 3-3.86zm-8 9c-1.11 0-2-.9-2-2s.89-2 2-2c1.1 0 2 .9 2 2s-.89 2-2 2zm0-5c-1.11 0-2-.9-2-2s.89-2 2-2c1.1 0 2 .9 2 2s-.89 2-2 2zm0-5c-1.11 0-2-.9-2-2 0-1.11.89-2 2-2 1.1 0 2 .89 2 2 0 1.1-.89 2-2 2z",
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
                d: "M12 2c-4 0-8 .5-8 4v9.5C4 17.43 5.57 19 7.5 19L6 20.5v.5h2l2-2h4l2 2h2v-.5L16.5 19c1.93 0 3.5-1.57 3.5-3.5V6c0-3.5-4-4-8-4zM7.5 17c-.83 0-1.5-.67-1.5-1.5S6.67 14 7.5 14s1.5.67 1.5 1.5S8.33 17 7.5 17zm3.5-7H6V6h5v4zm5.5 7c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm1.5-7h-5V6h5v4z",
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
                d: "M13 5l.75-1.5H17V2H7v1.5h4.75L11 5c-3.13.09-6 .73-6 3.5V17c0 1.5 1.11 2.73 2.55 2.95L6 21.5v.5h2l2-2h4l2 2h2v-.5l-1.55-1.55C17.89 19.73 19 18.5 19 17V8.5c0-2.77-2.87-3.41-6-3.5zm-1 13.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm5-4.5H7V9h10v5z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                polygon {
                    points: "12,16 13.5,16 13.5,7.5 5.87,7.5 7.06,6.31 6,5.25 3,8.25 6,11.25 7.06,10.19 5.87,9 12,9",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                polygon {
                    points: "6.83,11 8.41,12.59 7,14 3,10 7,6 8.41,7.41 6.83,9 17,9 17,20 15,20 15,11",
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
                polygon {
                    points: "8,16 6.5,16 6.5,7.5 14.13,7.5 12.94,6.31 14,5.25 17,8.25 14,11.25 12.94,10.19 14.13,9 8,9",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                polygon {
                    points: "17.17,11 15.59,12.59 17,14 21,10 17,6 15.59,7.41 17.17,9 7,9 7,20 9,20 9,11",
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
                    height: "20",
                    width: "20",
                }
            }
            g {
                polygon {
                    points: "5.5,5.87 5.5,12 13,12 13,17 14.5,17 14.5,10.5 7,10.5 7,5.87 8.19,7.06 9.25,6 6.25,3 3.25,6 4.31,7.06",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                polygon {
                    points: "6,6.83 4.41,8.41 3,7 7,3 11,7 9.59,8.41 8,6.83 8,13 18,13 18,21 16,21 16,15 6,15",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                polygon {
                    points: "14.5,5.87 14.5,12 7,12 7,17 5.5,17 5.5,10.5 13,10.5 13,5.87 11.81,7.06 10.75,6 13.75,3 16.75,6 15.69,7.06",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                polygon {
                    points: "18,6.83 19.59,8.41 21,7 17,3 13,7 14.41,8.41 16,6.83 16,13 6,13 6,21 8,21 8,15 18,15",
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
                    fill: "none",
                }
            }
            g {
                polygon {
                    points: "12.5,9.94 12.5,16 11,16 11,10.56 7,6.56 7,8.25 5.5,8.25 5.5,4 9.75,4 9.75,5.5 8.06,5.5",
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
                polygon {
                    points: "11.66,6 11.66,4 6,4 6,9.66 8,9.66 8,7.41 13,12.41 13,20 15,20 15,11.59 9.41,6",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                polygon {
                    points: "7.5,9.94 7.5,16 9,16 9,10.56 13,6.56 13,8.25 14.5,8.25 14.5,4 10.25,4 10.25,5.5 11.94,5.5",
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
            }
            g {
                polygon {
                    points: "12.34,6 12.34,4 18,4 18,9.66 16,9.66 16,7.41 11,12.41 11,20 9,20 9,11.59 14.59,6",
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
                    width: "20",
                    y: "0",
                    height: "20",
                    fill: "none",
                    x: "0",
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
                    x: "0",
                    fill_rule: "evenodd",
                    height: "24",
                    width: "24",
                    fill: "none",
                    y: "0",
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
                    fill: "none",
                    width: "20",
                    height: "20",
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
                    height: "24",
                    width: "24",
                    fill: "none",
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
                    fill: "none",
                    width: "20",
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
                    height: "24",
                    width: "24",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    rect {
                        y: "9",
                        height: "9",
                        width: "3",
                        x: "1",
                    }
                    path {
                        d: "M13.25,10.5C15.7,8.28,18,6.41,18,4.61C18,3.15,16.85,2,15.39,2c-0.82,0-1.62,0.39-2.14,0.99C12.74,2.39,11.94,2,11.11,2 C9.65,2,8.5,3.15,8.5,4.61C8.5,6.4,10.8,8.28,13.25,10.5z",
                    }
                    path {
                        d: "M10.5,14l-1.53-0.51l0.36-1.01L10.5,13H14v-1.5L7,9H5.5v7.36l6,1.64l6.5-2.5V14H10.5z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    rect {
                        height: "11",
                        width: "4",
                        x: "1",
                        y: "11",
                    }
                    path {
                        d: "M16,3.25C16.65,2.49,17.66,2,18.7,2C20.55,2,22,3.45,22,5.3c0,2.27-2.91,4.9-6,7.7c-3.09-2.81-6-5.44-6-7.7 C10,3.45,11.45,2,13.3,2C14.34,2,15.35,2.49,16,3.25z",
                    }
                    polygon {
                        points: "22,17 13,17 10.91,16.27 11.24,15.32 13,16 17,16 17,14 8.97,11 7,11 7,20.02 14,22 22,19",
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
                    d: "M14,17h4V6l-8-3L2,6v11h4v-7h8V17z M9.25,17h-1.5v-1.5h1.5V17z M10.75,14.5h-1.5V13h1.5V14.5z M12.25,17h-1.5v-1.5h1.5V17z",
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
                    d: "M22,21V7L12,3L2,7v14h5v-9h10v9H22z M11,19H9v2h2V19z M13,16h-2v2h2V16z M15,19h-2v2h2V19z",
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
                height: "24",
                fill: "none",
                width: "24",
            }
            path {
                d: "M6,3l0,6c0,2.97,2.16,5.43,5,5.91V19H8v2h8v-2h-3v-4.09c2.84-0.48,5-2.94,5-5.91l0-6H6z M16,8H8l0-3h8C16,5,16,8,16,8z",
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
                        d: "M12,8V4.37C11.37,4.13,10.69,4,10,4C7.06,4,4.4,6.25,4.4,9.74c0,2.32,1.87,5.08,5.6,8.26c3.73-3.18,5.6-5.94,5.6-8.26 c0-0.62-0.09-1.2-0.24-1.74H12z M10,11c-0.83,0-1.5-0.67-1.5-1.5S9.17,8,10,8s1.5,0.67,1.5,1.5S10.83,11,10,11z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M14,10V3.26C13.35,3.09,12.68,3,12,3c-4.2,0-8,3.22-8,8.2c0,3.32,2.67,7.25,8,11.8c5.33-4.55,8-8.48,8-11.8 c0-0.41-0.04-0.81-0.09-1.2H14z M12,13c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2C14,12.1,13.1,13,12,13z",
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
                    fill: "none",
                    width: "20",
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
                    height: "24",
                    width: "24",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M15 3l2.3 2.3-2.89 2.87 1.42 1.42L18.7 6.7 21 9V3h-6zM3 9l2.3-2.3 2.87 2.89 1.42-1.42L6.7 5.3 9 3H3v6zm6 12l-2.3-2.3 2.89-2.87-1.42-1.42L5.3 17.3 3 15v6h6zm12-6l-2.3 2.3-2.87-2.89-1.42 1.42 2.89 2.87L15 21h6v-6z",
            }
        }
    }
}

