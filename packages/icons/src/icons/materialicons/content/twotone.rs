use dioxus::prelude::*;
use crate::IconProps;
pub fn add_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z",
            }
        }
    }
}

pub fn add_box_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M5 19h14V5H5v14zm2-8h4V7h2v4h4v2h-4v4h-2v-4H7v-2z",
            }
            path {
                d: "M19 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zm-8-2h2v-4h4v-2h-4V7h-2v4H7v2h4z",
            }
        }
    }
}

pub fn add_circle_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M12 4c-4.41 0-8 3.59-8 8s3.59 8 8 8 8-3.59 8-8-3.59-8-8-8zm5 9h-4v4h-2v-4H7v-2h4V7h2v4h4v2z",
            }
            path {
                d: "M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4zm-1-5C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

pub fn add_circle_outline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13 7h-2v4H7v2h4v4h2v-4h4v-2h-4zm-1-5C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

pub fn add_link_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M8,11h8v2H8V11z M20.1,12H22c0-2.76-2.24-5-5-5h-4v1.9h4C18.71,8.9,20.1,10.29,20.1,12z M3.9,12c0-1.71,1.39-3.1,3.1-3.1h4 V7H7c-2.76,0-5,2.24-5,5s2.24,5,5,5h4v-1.9H7C5.29,15.1,3.9,13.71,3.9,12z M19,12h-2v3h-3v2h3v3h2v-3h3v-2h-3V12z",
                }
            }
        }
    }
}

pub fn amp_stories_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                g {
                    g {
                        opacity: ".3",
                        rect {
                            width: "6",
                            y: "5",
                            height: "9",
                            x: "7",
                        }
                    }
                    path {
                        d: "M6,15h8V4H6V15z M7,5h6v9H7V5z",
                    }
                    rect {
                        y: "5",
                        width: "1",
                        x: "3",
                        height: "9",
                    }
                    rect {
                        x: "16",
                        height: "9",
                        y: "5",
                        width: "1",
                    }
                }
            }
        }
    }
}

pub fn amp_stories_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                g {
                    g {
                        opacity: ".3",
                        rect {
                            x: "9",
                            width: "6",
                            height: "11",
                            y: "6",
                        }
                    }
                    path {
                        d: "M7,19h10V4H7V19z M9,6h6v11H9V6z",
                    }
                    rect {
                        width: "2",
                        height: "11",
                        x: "3",
                        y: "6",
                    }
                    rect {
                        x: "19",
                        width: "2",
                        y: "6",
                        height: "11",
                    }
                }
            }
        }
    }
}

pub fn archive_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5 19h14V8H5v11zm5.55-6v-3h2.91v3H16l-4 4-4-4h2.55z",
                opacity: ".3",
            }
            path {
                d: "M16 13h-2.55v-3h-2.9v3H8l4 4zm4.54-7.77l-1.39-1.68C18.88 3.21 18.47 3 18 3H6c-.47 0-.88.21-1.16.55L3.46 5.23C3.17 5.57 3 6.02 3 6.5V19c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V6.5c0-.48-.17-.93-.46-1.27zM6.24 5h11.52l.81.97H5.44l.8-.97zM19 19H5V8h14v11z",
            }
        }
    }
}

pub fn attribution_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 4c-4.42 0-8 3.58-8 8s3.58 8 8 8 8-3.58 8-8-3.58-8-8-8zm0 1c.83 0 1.5.67 1.5 1.5S12.83 8 12 8s-1.5-.67-1.5-1.5S11.17 5 12 5zm2.75 9.5h-1.5V19h-2.5v-4.5h-1.5V9.88c0-.92 1.84-1.38 2.75-1.38s2.75.47 2.75 1.38v4.62z",
                opacity: ".3",
            }
            path {
                d: "M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm0-11.5c-.91 0-2.75.46-2.75 1.38v4.62h1.5V19h2.5v-4.5h1.5V9.88c0-.91-1.84-1.38-2.75-1.38z",
            }
            circle {
                r: "1.5",
                cy: "6.5",
                cx: "12",
            }
        }
    }
}

pub fn backspace_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M7.06 5L2.4 12l4.67 7H22V5H7.06c.01 0 .01 0 0 0zM9 8.41L10.41 7 14 10.59 17.59 7 19 8.41 15.41 12 19 15.59 17.59 17 14 13.41 10.41 17 9 15.59 12.59 12 9 8.41z",
            }
            path {
                d: "M22 3H7c-.69 0-1.23.35-1.59.88L0 12l5.41 8.11c.36.53.9.89 1.59.89h15c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H7.07L2.4 12l4.66-7H22v14zm-11.59-2L14 13.41 17.59 17 19 15.59 15.41 12 19 8.41 17.59 7 14 10.59 10.41 7 9 8.41 12.59 12 9 15.59z",
            }
        }
    }
}

pub fn ballot_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M7 14h3v3H7zm0-7h3v3H7zM5 19h14V5H5v14zm8-11.5h5v2h-5v-2zm0 7h5v2h-5v-2zM6 6h5v5H6V6zm0 7h5v5H6v-5z",
            }
            path {
                d: "M13 7.5h5v2h-5zm0 7h5v2h-5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zM11 6H6v5h5V6zm-1 4H7V7h3v3zm1 3H6v5h5v-5zm-1 4H7v-3h3v3z",
            }
        }
    }
}

pub fn biotech_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,6.5c0,0.16-0.03,0.32-0.08,0.47l0.44,1.2l0.94-0.34l-1.16-3.18L9.2,4.99l0.09,0.23 C9.71,5.49,10,5.96,10,6.5z",
                        opacity: ".3",
                    }
                    circle {
                        cy: "6.5",
                        r: ".5",
                        cx: "8.5",
                        opacity: ".3",
                    }
                    path {
                        d: "M6,15c-0.55,0-1,0.45-1,1h10c0-0.55-0.45-1-1-1h-4v-2h3c0.55,0,1-0.45,1-1H8.47v0C7.11,11.98,6,10.87,6,9.5 c0-0.93,0.51-1.73,1.26-2.16C7.53,7.74,7.98,8,8.5,8c0.24,0,0.47-0.06,0.67-0.16l0.59,1.62l0.93-0.34l0.33,0.88l0.94-0.34 l-0.33-0.88l0.95-0.34l-1.84-5.06L9.8,3.71L9.47,2.83L8.53,3.17l0.32,0.88L7.92,4.39l0.24,0.65c-0.61,0.15-1.07,0.66-1.14,1.3 C5.83,6.9,5,8.1,5,9.5c0,1.76,1.31,3.2,3,3.45V15H6z M10.14,4.65l1.16,3.18l-0.94,0.34l-0.44-1.2C9.97,6.82,10,6.66,10,6.5 c0-0.54-0.29-1.01-0.72-1.27L9.2,4.99L10.14,4.65z M8.5,6C8.78,6,9,6.22,9,6.5S8.78,7,8.5,7S8,6.78,8,6.5S8.22,6,8.5,6z",
                    }
                }
            }
        }
    }
}

pub fn biotech_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    rect {
                        height: "5",
                        transform: "matrix(0.9397 -0.342 0.342 0.9397 -1.6091 4.9556)",
                        x: "12.75",
                        y: "4.54",
                        opacity: ".3",
                        width: "1",
                    }
                    circle {
                        r: "1",
                        opacity: ".3",
                        cx: "10.5",
                        cy: "8",
                    }
                    path {
                        d: "M7,19c-1.1,0-2,0.9-2,2h14c0-1.1-0.9-2-2-2h-4v-2h3c1.1,0,2-0.9,2-2h-8c-1.66,0-3-1.34-3-3c0-1.09,0.59-2.04,1.47-2.57 c0.41,0.59,1.06,1,1.83,1.06c0.7,0.06,1.36-0.19,1.85-0.62l0.59,1.61l0.94-0.34l0.34,0.94l1.88-0.68l-0.34-0.94l0.94-0.34 L13.76,2.6l-0.94,0.34L12.48,2L10.6,2.68l0.34,0.94L10,3.97l0.56,1.55C9.39,5.48,8.37,6.27,8.08,7.38C6.27,8.14,5,9.92,5,12 c0,2.76,2.24,5,5,5v2H7z M12.86,4.52l1.71,4.7l-0.94,0.34l-1.71-4.7L12.86,4.52z M10.5,7c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1 c-0.55,0-1-0.45-1-1C9.5,7.45,9.95,7,10.5,7z",
                    }
                }
            }
        }
    }
}

pub fn block_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM4 12c0-4.42 3.58-8 8-8 1.85 0 3.55.63 4.9 1.69L5.69 16.9C4.63 15.55 4 13.85 4 12zm8 8c-1.85 0-3.55-.63-4.9-1.69L18.31 7.1C19.37 8.45 20 10.15 20 12c0 4.42-3.58 8-8 8z",
            }
        }
    }
}

pub fn bolt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M11,21h-1l1-7H7.5c-0.88,0-0.33-0.75-0.31-0.78C8.48,10.94,10.42,7.54,13.01,3h1l-1,7h3.51c0.4,0,0.62,0.19,0.4,0.66 C12.97,17.55,11,21,11,21z",
                }
            }
        }
    }
}

pub fn calculate_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M5,15h10V5H5V15z M14,13.25h-3v-1h3V13.25z M14,11.75h-3v-1h3V11.75z M10.73,6.71L11.44,6l1.06,1.06 L13.56,6l0.71,0.71l-1.06,1.06l1.06,1.06l-0.71,0.71L12.5,8.47l-1.06,1.06l-0.71-0.71l1.06-1.06L10.73,6.71z M6,7.27h3v1H6V7.27z M6,11.5h1.5V10h1v1.5H10v1H8.5V14h-1v-1.5H6V11.5z",
                        opacity: ".3",
                    }
                    g {
                        rect {
                            height: "1",
                            y: "12.25",
                            x: "11",
                            width: "3",
                        }
                    }
                    g {
                        rect {
                            width: "3",
                            y: "10.75",
                            x: "11",
                            height: "1",
                        }
                    }
                    g {
                        rect {
                            width: "3",
                            height: "1",
                            y: "7.27",
                            x: "6",
                        }
                    }
                    g {
                        path {
                            d: "M15,4H5C4.45,4,4,4.45,4,5v10c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V5C16,4.45,15.55,4,15,4z M15,15H5V5h10V15z",
                        }
                    }
                    g {
                        polygon {
                            points: "7.5,14 8.5,14 8.5,12.5 10,12.5 10,11.5 8.5,11.5 8.5,10 7.5,10 7.5,11.5 6,11.5 6,12.5 7.5,12.5",
                        }
                    }
                    g {
                        polygon {
                            points: "11.44,9.54 12.5,8.47 13.56,9.54 14.27,8.83 13.21,7.77 14.27,6.71 13.56,6 12.5,7.06 11.44,6 10.73,6.71 11.79,7.77 10.73,8.83",
                        }
                    }
                }
            }
        }
    }
}

pub fn calculate_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M5,19h14V5H5V19z M13.03,7.06L14.09,6l1.41,1.41L16.91,6l1.06,1.06l-1.41,1.41l1.41,1.41l-1.06,1.06 L15.5,9.54l-1.41,1.41l-1.06-1.06l1.41-1.41L13.03,7.06z M13,13.25h5v1.5h-5V13.25z M13,15.75h5v1.5h-5V15.75z M6.25,7.72h5v1.5 h-5V7.72z M6,14.5h2v-2h1.5v2h2V16h-2v2H8v-2H6V14.5z",
                        opacity: ".3",
                    }
                    path {
                        d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z",
                    }
                    rect {
                        y: "7.72",
                        width: "5",
                        x: "6.25",
                        height: "1.5",
                    }
                    rect {
                        x: "13",
                        height: "1.5",
                        width: "5",
                        y: "15.75",
                    }
                    rect {
                        x: "13",
                        width: "5",
                        y: "13.25",
                        height: "1.5",
                    }
                    polygon {
                        points: "8,18 9.5,18 9.5,16 11.5,16 11.5,14.5 9.5,14.5 9.5,12.5 8,12.5 8,14.5 6,14.5 6,16 8,16",
                    }
                    polygon {
                        points: "14.09,10.95 15.5,9.54 16.91,10.95 17.97,9.89 16.56,8.47 17.97,7.06 16.91,6 15.5,7.41 14.09,6 13.03,7.06 14.44,8.47 13.03,9.89",
                    }
                }
            }
        }
    }
}

pub fn change_circle_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10,3.5c-3.58,0-6.5,2.92-6.5,6.5s2.92,6.5,6.5,6.5s6.5-2.92,6.5-6.5S13.58,3.5,10,3.5z M9.88,15.62V14 c-1.02-0.03-1.98-0.44-2.71-1.17C6.42,12.07,6,11.07,6,10c0-0.7,0.18-1.38,0.52-1.98l0.74,0.74C7.09,9.15,7,9.57,7,10 c0,0.8,0.31,1.55,0.88,2.12c0.54,0.53,1.27,0.82,2,0.85v-1.59L12,13.5L9.88,15.62z M13.48,11.98l-0.74-0.74 C12.91,10.85,13,10.43,13,10c0-0.8-0.31-1.55-0.88-2.12c-0.54-0.54-1.24-0.85-2-0.88v1.62L8,6.5l2.12-2.12v1.66 c1,0.03,1.98,0.41,2.71,1.13C13.58,7.93,14,8.93,14,10C14,10.7,13.82,11.38,13.48,11.98z",
                opacity: ".3",
            }
            path {
                d: "M10,3.5c3.58,0,6.5,2.92,6.5,6.5s-2.92,6.5-6.5,6.5S3.5,13.58,3.5,10S6.42,3.5,10,3.5 M10,2c-4.42,0-8,3.58-8,8s3.58,8,8,8 s8-3.58,8-8S14.42,2,10,2L10,2z M9.88,15.62V14c-1.02-0.03-1.98-0.44-2.71-1.17C6.42,12.07,6,11.07,6,10c0-0.7,0.18-1.38,0.52-1.98 l0.74,0.74C7.09,9.15,7,9.57,7,10c0,0.8,0.31,1.55,0.88,2.12c0.54,0.53,1.27,0.82,2,0.85v-1.59L12,13.5L9.88,15.62z M13.48,11.98 l-0.74-0.74C12.91,10.85,13,10.43,13,10c0-0.8-0.31-1.55-0.88-2.12c-0.54-0.54-1.24-0.85-2-0.88v1.62L8,6.5l2.12-2.12v1.66 c1,0.03,1.98,0.41,2.71,1.13C13.58,7.93,14,8.93,14,10C14,10.7,13.82,11.38,13.48,11.98z",
            }
        }
    }
}

pub fn change_circle_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8s-8-3.59-8-8S7.59,4,12,4 M12.06,13.34v2.14 c-0.92,0.02-1.84-0.31-2.54-1.01c-1.12-1.12-1.3-2.8-0.59-4.13l-1.1-1.1c-1.28,1.94-1.07,4.59,0.64,6.29C9.44,16.51,10.72,17,12,17 c0.02,0,0.04,0,0.06,0V19l2.83-2.83L12.06,13.34z M15.54,8.46c-0.99-0.99-2.3-1.46-3.6-1.45V5L9.11,7.83l2.83,2.83V8.51 c0.02,0,0.04,0,0.06,0c0.9,0,1.79,0.34,2.48,1.02c1.12,1.12,1.3,2.8,0.59,4.13l1.1,1.1C17.45,12.82,17.24,10.17,15.54,8.46z",
            }
            path {
                d: "M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8s-8-3.59-8-8S7.59,4,12,4 M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10 S17.52,2,12,2L12,2z M12.06,13.34v2.14c-0.92,0.02-1.84-0.31-2.54-1.01c-1.12-1.12-1.3-2.8-0.59-4.13l-1.1-1.1 c-1.28,1.94-1.07,4.59,0.64,6.29C9.44,16.51,10.72,17,12,17c0.02,0,0.04,0,0.06,0V19l2.83-2.83L12.06,13.34z M15.54,8.46 c-0.99-0.99-2.3-1.46-3.6-1.45V5L9.11,7.83l2.83,2.83V8.51c0.02,0,0.04,0,0.06,0c0.9,0,1.79,0.34,2.48,1.02 c1.12,1.12,1.3,2.8,0.59,4.13l1.1,1.1C17.45,12.82,17.24,10.17,15.54,8.46z",
            }
        }
    }
}

pub fn clear_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41z",
            }
        }
    }
}

pub fn content_copy_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M8 7h11v14H8z",
                opacity: ".3",
            }
            path {
                d: "M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm3 4H8c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z",
            }
        }
    }
}

pub fn content_cut_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 3l-6 6 2 2 7-7V3zm-9 3c0-2.21-1.79-4-4-4S2 3.79 2 6s1.79 4 4 4c.59 0 1.14-.13 1.64-.36L10 12l-2.36 2.36C7.14 14.13 6.59 14 6 14c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4c0-.59-.13-1.14-.36-1.64L12 14l7 7h3v-1L9.64 7.64c.23-.5.36-1.05.36-1.64zM6 8c-1.1 0-2-.89-2-2s.9-2 2-2 2 .89 2 2-.9 2-2 2zm0 12c-1.1 0-2-.89-2-2s.9-2 2-2 2 .89 2 2-.9 2-2 2zm6-8.5c.28 0 .5.22.5.5s-.22.5-.5.5-.5-.22-.5-.5.22-.5.5-.5z",
            }
        }
    }
}

pub fn content_paste_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M17 7H7V4H5v16h14V4h-2z",
            }
            path {
                d: "M19 2h-4.18C14.4.84 13.3 0 12 0S9.6.84 9.18 2H5c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm7 18H5V4h2v3h10V4h2v16z",
            }
        }
    }
}

pub fn content_paste_go_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M9,13c0-2.21,1.79-4,4-4h2.5V4.5H14V7H6V4.5H4.5v11H9V13z",
                        opacity: ".3",
                    }
                    path {
                        d: "M9,15.5H4.5v-11H6V7h8V4.5h1.5V9H17V4.5C17,3.67,16.33,3,15.5,3h-3.57c-0.22-0.86-1-1.5-1.93-1.5 C9.07,1.5,8.29,2.14,8.07,3H4.5C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17H9V15.5z M10,3c0.41,0,0.75,0.33,0.75,0.75 c0,0.41-0.34,0.75-0.75,0.75S9.25,4.16,9.25,3.75C9.25,3.33,9.59,3,10,3z",
                    }
                    polygon {
                        points: "15,11 13.94,12.06 15.13,13.25 11,13.25 11,14.75 15.13,14.75 13.94,15.94 15,17 18,14",
                    }
                }
            }
        }
    }
}

pub fn content_paste_go_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,17c0-3.31,2.69-6,6-6h3V5h-2v3H7V5H5v14h5V17z",
                        opacity: ".3",
                    }
                    path {
                        d: "M10,19H5V5h2v3h10V5h2v6h2V5c0-1.1-0.9-2-2-2h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H5C3.9,3,3,3.9,3,5v14 c0,1.1,0.9,2,2,2h5V19z M12,3c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,3,12,3z",
                    }
                    polygon {
                        points: "18.01,13 16.59,14.41 18.17,15.99 12,15.99 12,17.99 18.17,17.99 16.59,19.58 18.01,20.99 22,16.99",
                    }
                }
            }
        }
    }
}

pub fn content_paste_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M9.12,7H14V4.5h1.5v8.88L9.12,7z M4.5,15.5V6.62l8.88,8.88H4.5z",
            }
            path {
                d: "M17.07,17.07L2.93,2.93L1.87,3.99L3,5.12V15.5C3,16.33,3.67,17,4.5,17h10.38l1.13,1.13L17.07,17.07z M4.5,15.5V6.62 l8.88,8.88H4.5z M14,7V4.5h1.5v8.88l1.5,1.5V4.5C17,3.67,16.33,3,15.5,3h-3.57c-0.22-0.86-1-1.5-1.93-1.5C9.07,1.5,8.29,2.14,8.07,3 H5.12l4,4H14z M10,3c0.41,0,0.75,0.34,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75S9.25,4.16,9.25,3.75C9.25,3.34,9.59,3,10,3z",
            }
        }
    }
}

pub fn content_paste_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M10.83,8H17V5h2v11.17L10.83,8z M5,19V7.83L16.17,19H5z",
                opacity: ".3",
            }
            path {
                d: "M21.19,21.19L2.81,2.81L1.39,4.22L3,5.83V19c0,1.1,0.9,2,2,2h13.17l1.61,1.61L21.19,21.19z M5,19V7.83L16.17,19H5z M17,8V5 h2v11.17l2,2V5c0-1.1-0.9-2-2-2h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H5.83l5,5H17z M12,3c0.55,0,1,0.45,1,1s-0.45,1-1,1 s-1-0.45-1-1S11.45,3,12,3z",
            }
        }
    }
}

pub fn content_paste_search_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.94,15.88c0.35-0.54,0.56-1.19,0.56-1.88c0-1.93-1.57-3.5-3.5-3.5s-3.5,1.57-3.5,3.5s1.57,3.5,3.5,3.5 c0.69,0,1.34-0.21,1.88-0.56L17.94,19L19,17.94L16.94,15.88z M14,16c-1.1,0-2-0.9-2-2c0-0.31,0.07-0.61,0.2-0.87 C12.52,12.46,13.21,12,14,12c1.1,0,2,0.9,2,2S15.1,16,14,16z",
                    }
                    path {
                        d: "M9,14c0-2.76,2.24-5,5-5c0.53,0,1.02,0.1,1.5,0.26V4.5H14V7H6V4.5H4.5v11h4.76C9.1,15.02,9,14.53,9,14z",
                        opacity: ".3",
                    }
                    path {
                        d: "M9.26,15.5H4.5v-11H6V7h8V4.5h1.5v4.76c0.55,0.17,1.05,0.43,1.5,0.77V4.5C17,3.67,16.33,3,15.5,3h-3.57 c-0.22-0.86-1-1.5-1.93-1.5C9.07,1.5,8.29,2.14,8.07,3H4.5C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h5.53 C9.69,16.55,9.43,16.05,9.26,15.5z M10,3c0.41,0,0.75,0.33,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75S9.25,4.16,9.25,3.75 C9.25,3.33,9.59,3,10,3z",
                    }
                }
            }
        }
    }
}

pub fn content_paste_search_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,16.5c0-3.58,2.92-6.5,6.5-6.5c0.89,0,1.73,0.18,2.5,0.5V5h-2v3H7V5H5v14h5.5 C10.18,18.23,10,17.39,10,16.5z",
                        opacity: ".3",
                    }
                    path {
                        d: "M10.5,19H5V5h2v3h10V5h2v5.5c0.75,0.31,1.42,0.76,2,1.32V5c0-1.1-0.9-2-2-2h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3 H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h6.82C11.27,20.42,10.81,19.75,10.5,19z M12,3c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1 S11.45,3,12,3z",
                    }
                    path {
                        d: "M20.3,18.9c0.4-0.7,0.7-1.5,0.7-2.4c0-2.5-2-4.5-4.5-4.5S12,14,12,16.5s2,4.5,4.5,4.5c0.9,0,1.7-0.3,2.4-0.7l2.7,2.7 l1.4-1.4L20.3,18.9z M16.5,19c-1.4,0-2.5-1.1-2.5-2.5c0-1.4,1.1-2.5,2.5-2.5s2.5,1.1,2.5,2.5C19,17.9,17.9,19,16.5,19z",
                    }
                }
            }
        }
    }
}

pub fn copy_all_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
            rect {
                width: "8",
                opacity: ".3",
                height: "10",
                y: "3.5",
                x: "7.5",
            }
            path {
                d: "M15.5,2h-8C6.67,2,6,2.67,6,3.5v10C6,14.33,6.67,15,7.5,15h8c0.83,0,1.5-0.67,1.5-1.5v-10C17,2.67,16.33,2,15.5,2z M15.5,13.5h-8v-10h8V13.5z M3,12v-1.5h1.5V12H3z M3,15v-1.5h1.5V15H3z M9,16.5h1.5V18H9V16.5z M3,7.5h1.5V9H3V7.5z M7.5,18H6v-1.5 h1.5V18z M4.5,18C3.67,18,3,17.33,3,16.5h1.5V18z M4.5,6H3c0-0.83,0.67-1.5,1.5-1.5V6z M13.49,16.5c0,0.83-0.67,1.5-1.5,1.5h0v-1.5 L13.49,16.5L13.49,16.5z",
            }
        }
    }
}

pub fn copy_all_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                height: "12",
                width: "9",
                y: "4",
                opacity: ".3",
                x: "9",
            }
            path {
                d: "M18,2H9C7.9,2,7,2.9,7,4v12c0,1.1,0.9,2,2,2h9c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2z M18,16H9V4h9V16z M3,15v-2h2v2H3z M3,9.5h2v2H3V9.5z M10,20h2v2h-2V20z M3,18.5v-2h2v2H3z M5,22c-1.1,0-2-0.9-2-2h2V22z M8.5,22h-2v-2h2V22z M13.5,22L13.5,22l0-2h2 v0C15.5,21.1,14.6,22,13.5,22z M5,6L5,6l0,2H3v0C3,6.9,3.9,6,5,6z",
            }
        }
    }
}

pub fn create_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M5 18.08V19h.92l9.06-9.06-.92-.92z",
            }
            path {
                d: "M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM5.92 19H5v-.92l9.06-9.06.92.92L5.92 19zM20.71 5.63l-2.34-2.34c-.2-.2-.45-.29-.71-.29s-.51.1-.7.29l-1.83 1.83 3.75 3.75 1.83-1.83c.39-.39.39-1.02 0-1.41z",
            }
        }
    }
}

pub fn delete_sweep_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M5 10h6v8H5z",
            }
            path {
                d: "M15 16h4v2h-4zm0-8h7v2h-7zm0 4h6v2h-6zM3 18c0 1.1.9 2 2 2h6c1.1 0 2-.9 2-2V8H3v10zm2-8h6v8H5v-8zm5-6H6L5 5H2v2h12V5h-3z",
            }
        }
    }
}

pub fn deselect_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,11.88V6H8.12l1.5,1.5h2.88v2.88L14,11.88z M4.5,17v-1.5H3C3,16.33,3.67,17,4.5,17z M3,13.88h1.5v-1.5H3V13.88z M3,10.75h1.5v-1.5H3V10.75z M15.5,10.75H17v-1.5h-1.5V10.75z M15.5,7.62H17v-1.5h-1.5V7.62z M15.5,3v1.5H17 C17,3.67,16.33,3,15.5,3z M12.38,4.5h1.5V3h-1.5V4.5z M9.25,4.5h1.5V3h-1.5V4.5z M6.12,17h1.5v-1.5h-1.5V17z M9.25,17h1.5v-1.5 h-1.5V17z M17.07,17.07L2.93,2.93L1.87,3.99L4,6.12H3v1.5h1.5v-1L6,8.12V14h5.88l1.5,1.5h-1V17h1.5v-1l2.13,2.13L17.07,17.07z M7.62,4.5V3h-1.5v1l0.5,0.5H7.62z M17,13.88v-1.5h-1.5v1l0.5,0.5H17z M10.38,12.5H7.5V9.62L10.38,12.5z",
                }
            }
        }
    }
}

pub fn deselect_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M3,13h2v-2H3V13z M7,21h2v-2H7V21z M13,3h-2v2h2V3z M19,3v2h2C21,3.9,20.1,3,19,3z M5,21v-2H3C3,20.1,3.9,21,5,21z M3,17h2 v-2H3V17z M11,21h2v-2h-2V21z M19,13h2v-2h-2V13z M19,9h2V7h-2V9z M15,5h2V3h-2V5z M7.83,5L7,4.17V3h2v2H7.83z M19.83,17L19,16.17 V15h2v2H19.83z M21.19,21.19L2.81,2.81L1.39,4.22L4.17,7H3v2h2V7.83l2,2V17h7.17l2,2H15v2h2v-1.17l2.78,2.78L21.19,21.19z M9,15 v-3.17L12.17,15H9z M15,12.17V9h-3.17l-2-2H17v7.17L15,12.17z",
                }
            }
        }
    }
}

pub fn drafts_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M12 15.36l-8-5.02V18h16l-.01-7.63z",
            }
            path {
                d: "M21.99 8c0-.72-.37-1.35-.94-1.7L12 1 2.95 6.3C2.38 6.65 2 7.28 2 8v10c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2l-.01-10zM12 3.32L19.99 8v.01L12 13 4 8l8-4.68zM4 18v-7.66l8 5.02 7.99-4.99L20 18H4z",
            }
        }
    }
}

pub fn dynamic_feed_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                g {
                    rect {
                        width: "8",
                        opacity: ".3",
                        y: "7",
                        x: "8",
                        height: "4",
                    }
                    g {
                        path {
                            d: "M4,8H3v7c0,0.55,0.45,1,1,1h9v-1H4V8z",
                        }
                    }
                    g {
                        path {
                            d: "M6,6H5v7c0,0.55,0.45,1,1,1h9v-1H6V6z",
                        }
                    }
                    path {
                        d: "M16,4H8C7.45,4,7,4.45,7,5v6c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1V5C17,4.45,16.55,4,16,4z M16,11H8V7h8V11z",
                    }
                }
            }
            g {
                display: "none",
                g {
                    display: "inline",
                }
                path {
                    display: "inline",
                    d: "M4,15V8H3v7c0,0.55,0.45,1,1,1h9v-1H4z",
                }
                path {
                    display: "inline",
                    d: "M6,13V6H5v7c0,0.55,0.45,1,1,1h9v-1H6z",
                }
                g {
                    display: "inline",
                    path {
                        d: "M16,5v6H8V5H16 M16,4H8C7.45,4,7,4.45,7,5v6c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1V5C17,4.45,16.55,4,16,4L16,4z",
                    }
                }
                g {
                    display: "inline",
                    rect {
                        y: "5",
                        x: "8",
                        height: "2",
                        width: "8",
                    }
                }
            }
        }
    }
}

pub fn dynamic_feed_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    x: "12",
                    opacity: ".3",
                    y: "7",
                    height: "4",
                    width: "8",
                }
                g {
                }
                g {
                    path {
                        d: "M8,8H6v7c0,1.1,0.9,2,2,2h9v-2H8V8z",
                    }
                    path {
                        d: "M20,3h-8c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V5C22,3.9,21.1,3,20,3z M20,11h-8V7h8V11z",
                    }
                    path {
                        d: "M4,12H2v7c0,1.1,0.9,2,2,2h9v-2H4V12z",
                    }
                }
            }
            g {
                display: "none",
                rect {
                    display: "inline",
                    height: "4",
                    width: "8",
                    x: "12",
                    opacity: ".3",
                    y: "7",
                }
                g {
                    display: "inline",
                }
                g {
                    display: "inline",
                    path {
                        d: "M8,8H6v7c0,1.1,0.9,2,2,2h9v-2H8V8z",
                    }
                    path {
                        d: "M20,3h-8c-1.1,0-2,0.9-2,2v6c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V5C22,3.9,21.1,3,20,3z M20,11h-8V7h8V11z",
                    }
                    path {
                        d: "M4,12H2v7c0,1.1,0.9,2,2,2h9v-2H4V12z",
                    }
                }
            }
        }
    }
}

pub fn file_copy_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14 7H8v14h11v-9h-5z",
                opacity: ".3",
            }
            path {
                d: "M16 1H4c-1.1 0-2 .9-2 2v14h2V3h12V1zm-1 4H8c-1.1 0-1.99.9-1.99 2L6 21c0 1.1.89 2 1.99 2H19c1.1 0 2-.9 2-2V11l-6-6zm4 16H8V7h6v5h5v9z",
            }
        }
    }
}

pub fn filter_list_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M10 18h4v-2h-4v2zM3 6v2h18V6H3zm3 7h12v-2H6v2z",
            }
        }
    }
}

pub fn filter_list_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M7.62,5.5H17V7H9.12L7.62,5.5z M12.87,10.75H15v-1.5h-3.63L12.87,10.75z M16.01,18.13l1.06-1.06L2.93,2.93L1.87,3.99 L3.38,5.5H3V7h1.88l2.25,2.25H5v1.5h3.63L10.88,13L8,13v1.5h4v-0.38L16.01,18.13z",
                }
            }
        }
    }
}

pub fn filter_list_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.83,8H21V6H8.83L10.83,8z M15.83,13H18v-2h-4.17L15.83,13z M14,16.83V18h-4v-2h3.17l-3-3H6v-2h2.17l-3-3H3V6h0.17 L1.39,4.22l1.41-1.41l18.38,18.38l-1.41,1.41L14,16.83z",
                }
            }
        }
    }
}

pub fn flag_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12.36 6H7v6h7.24l.4 2H18V8h-5.24z",
                opacity: ".3",
            }
            path {
                d: "M14.4 6L14 4H5v17h2v-7h5.6l.4 2h7V6h-5.6zm3.6 8h-3.36l-.4-2H7V6h5.36l.4 2H18v6z",
            }
        }
    }
}

pub fn flag_circle_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        opacity: ".3",
                        path {
                            d: "M10,3.5c-3.58,0-6.5,2.92-6.5,6.5s2.92,6.5,6.5,6.5s6.5-2.92,6.5-6.5S13.58,3.5,10,3.5z M11,12l-1-1.5H8V15H7V6h4.5 l1,1.5H15V12H11z",
                        }
                    }
                    path {
                        d: "M10,2c-4.42,0-8,3.58-8,8s3.58,8,8,8s8-3.58,8-8S14.42,2,10,2z M10,16.5c-3.58,0-6.5-2.92-6.5-6.5S6.42,3.5,10,3.5 s6.5,2.92,6.5,6.5S13.58,16.5,10,16.5z",
                    }
                    path {
                        d: "M12.5,7.5l-1-1.5H7v9h1v-4.5h2l1,1.5h4V7.5H12.5z M14,11h-2.46l-1-1.5H8V7h2.96l1,1.5H14V11z",
                    }
                }
            }
        }
    }
}

pub fn flag_circle_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        opacity: ".3",
                        path {
                            d: "M12,4c-4.41,0-8,3.59-8,8s3.59,8,8,8s8-3.59,8-8S16.41,4,12,4z M13,15l-1-2H9.5v5H8V7h6l1,2h3v6H13z",
                        }
                    }
                    path {
                        d: "M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.41,0-8-3.59-8-8s3.59-8,8-8s8,3.59,8,8 S16.41,20,12,20z",
                    }
                    path {
                        d: "M15,9l-1-2H8v11h1.5v-5H12l1,2h5V9H15z M16.5,13.5h-2.57l-1-2H9.5v-3h3.57l1,2h2.43V13.5z",
                    }
                }
            }
        }
    }
}

pub fn font_download_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M4 20h16V4H4v16zm7.07-14.5h1.86l5.11 13h-2.09l-1.14-3H9.17l-1.12 3H5.96l5.11-13zM12 7.98L9.93 13.5h4.14z",
                opacity: ".3",
            }
            path {
                d: "M9.17 15.5h5.64l1.14 3h2.09l-5.11-13h-1.86l-5.11 13h2.09l1.12-3zM12 7.98l2.07 5.52H9.93L12 7.98zM20 2H4c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 18H4V4h16v16z",
            }
        }
    }
}

pub fn font_download_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M10.61,8.49l-0.57-1.63H9.95L9.7,7.58L10.61,8.49z M14.38,16.5l-4.06-4.06H7.99L7.08,15H5.35L7.4,9.52 l-3.9-3.9V16.5H14.38z M8.56,6.44L9.1,5h1.79l2.27,6.04l3.34,3.34V3.5H5.62L8.56,6.44z",
            }
            path {
                d: "M9.7,7.58L8.56,6.44L9.1,5h1.79l2.27,6.04l-2.55-2.55l-0.57-1.63H9.95L9.7,7.58z M16.72,18.84L15.88,18H3.5 C2.67,18,2,17.33,2,16.5V4.12L1.16,3.28l1.06-1.06l15.56,15.56L16.72,18.84z M14.38,16.5l-4.06-4.06H7.99L7.08,15H5.35L7.4,9.52 l-3.9-3.9V16.5H14.38z M16.5,3.5v10.88l1.5,1.5V3.5C18,2.67,17.33,2,16.5,2H4.12l1.5,1.5H16.5z",
            }
        }
    }
}

pub fn font_download_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M10.35,7.52L10.92,6h2.14l2.55,6.79L20,17.17V4H6.83L10.35,7.52z M12.58,9.75l-0.54-1.52h-0.1l-0.23,0.66 L12.58,9.75z M17.17,20l-5.07-5.07H9.58L8.49,18H6.41l2.39-6.37L4,6.83V20H17.17z",
                opacity: ".3",
            }
            path {
                d: "M4.83,2H20c1.1,0,2,0.9,2,2v15.17l-2-2V4H6.83L4.83,2z M10.92,6l-0.57,1.52l1.36,1.36l0.23-0.66h0.1l0.54,1.52l3.04,3.04 L13.07,6H10.92z M20.49,23.31L19.17,22H4c-1.1,0-2-0.9-2-2V4.83L0.69,3.51L2.1,2.1l19.8,19.8L20.49,23.31z M17.17,20l-5.07-5.07 H9.58L8.49,18H6.41l2.39-6.37L4,6.83V20H17.17z",
            }
        }
    }
}

pub fn forward_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M14 14v1.17L17.17 12 14 8.83V10H6v4z",
            }
            path {
                d: "M20 12l-8-8v4H4v8h8v4l8-8zM6 14v-4h8V8.83L17.17 12 14 15.17V14H6z",
            }
        }
    }
}

pub fn gesture_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M4.59 6.89c.7-.71 1.4-1.35 1.71-1.22.5.2 0 1.03-.3 1.52-.25.42-2.86 3.89-2.86 6.31 0 1.28.48 2.34 1.34 2.98.75.56 1.74.73 2.64.46 1.07-.31 1.95-1.4 3.06-2.77 1.21-1.49 2.83-3.44 4.08-3.44 1.63 0 1.65 1.01 1.76 1.79-3.78.64-5.38 3.67-5.38 5.37 0 1.7 1.44 3.09 3.21 3.09 1.63 0 4.29-1.33 4.69-6.1H21v-2.5h-2.47c-.15-1.65-1.09-4.2-4.03-4.2-2.25 0-4.18 1.91-4.94 2.84-.58.73-2.06 2.48-2.29 2.72-.25.3-.68.84-1.11.84-.45 0-.72-.83-.36-1.92.35-1.09 1.4-2.86 1.85-3.52.78-1.14 1.3-1.92 1.3-3.28C8.95 3.69 7.31 3 6.44 3 5.12 3 3.97 4 3.72 4.25c-.36.36-.66.66-.88.93l1.75 1.71zm9.29 11.66c-.31 0-.74-.26-.74-.72 0-.6.73-2.2 2.87-2.76-.3 2.69-1.43 3.48-2.13 3.48z",
            }
        }
    }
}

pub fn how_to_reg_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                cx: "11",
                cy: "8",
                opacity: ".3",
                r: "2",
            }
            path {
                d: "M5 18h4.99L9 17l.93-.94C7.55 16.33 5.2 17.37 5 18z",
                opacity: ".3",
            }
            path {
                d: "M11 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0-6c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm-1 12H5c.2-.63 2.55-1.67 4.93-1.94h.03l.46-.45L12 14.06c-.39-.04-.68-.06-1-.06-2.67 0-8 1.34-8 4v2h9l-2-2zm10.6-5.5l-5.13 5.17-2.07-2.08L12 17l3.47 3.5L22 13.91z",
            }
        }
    }
}

pub fn how_to_vote_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M5 19h14v1H5z",
            }
            path {
                d: "M18 13h-.68l-2 2h1.91L19 17H5l1.78-2h2.05l-2-2H6l-3 3v4c0 1.1.89 2 1.99 2H19c1.1 0 2-.89 2-2v-4l-3-3zm1 7H5v-1h14v1z",
            }
            path {
                opacity: ".3",
                d: "M12.048 12.905L8.505 9.362l4.95-4.95 3.543 3.543z",
            }
            path {
                d: "M19.11 7.25L14.16 2.3c-.38-.4-1.01-.4-1.4-.01L6.39 8.66c-.39.39-.39 1.02 0 1.41l4.95 4.95c.39.39 1.02.39 1.41 0l6.36-6.36c.39-.39.39-1.02 0-1.41zm-7.06 5.65L8.51 9.36l4.95-4.95L17 7.95l-4.95 4.95z",
            }
        }
    }
}

pub fn inbox_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12.01 18c-1.48 0-2.75-.81-3.45-2H5v3h14v-3h-3.55c-.69 1.19-1.97 2-3.44 2z",
                opacity: ".3",
            }
            path {
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5v-3h3.56c.69 1.19 1.97 2 3.45 2s2.75-.81 3.45-2H19v3zm0-5h-5c0 1.1-.9 2-2 2s-2-.9-2-2H5V5h14v9z",
            }
        }
    }
}

pub fn insights_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "12.5,8 13.29,6.28 15,5.5 13.29,4.72 12.5,3 11.74,4.72 10,5.5 11.74,6.28",
                    }
                    polygon {
                        points: "4,10 4.4,8.4 6,8 4.4,7.6 4,6 3.6,7.6 2,8 3.6,8.4",
                    }
                    path {
                        d: "M16.5,6c-1.07,0-1.84,1.12-1.35,2.14l-3.01,3.01c-0.52-0.25-0.99-0.14-1.29,0l-1.01-1.01C9.94,9.95,10,9.73,10,9.5 C10,8.67,9.33,8,8.5,8S7,8.67,7,9.5c0,0.23,0.06,0.45,0.15,0.64l-3.01,3.01C3.95,13.06,3.73,13,3.5,13C2.67,13,2,13.67,2,14.5 S2.67,16,3.5,16S5,15.33,5,14.5c0-0.23-0.06-0.45-0.15-0.64l3.01-3.01c0.52,0.25,0.99,0.14,1.29,0l1.01,1.01 C10.06,12.05,10,12.27,10,12.5c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5c0-0.23-0.06-0.45-0.15-0.64l3.01-3.01 C16.89,9.35,18,8.56,18,7.5C18,6.67,17.33,6,16.5,6z",
                    }
                }
            }
        }
    }
}

pub fn insights_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M21,8c-1.45,0-2.26,1.44-1.93,2.51l-3.55,3.56c-0.3-0.09-0.74-0.09-1.04,0l-2.55-2.55C12.27,10.45,11.46,9,10,9 c-1.45,0-2.27,1.44-1.93,2.52l-4.56,4.55C2.44,15.74,1,16.55,1,18c0,1.1,0.9,2,2,2c1.45,0,2.26-1.44,1.93-2.51l4.55-4.56 c0.3,0.09,0.74,0.09,1.04,0l2.55,2.55C12.73,16.55,13.54,18,15,18c1.45,0,2.27-1.44,1.93-2.52l3.56-3.55 C21.56,12.26,23,11.45,23,10C23,8.9,22.1,8,21,8z",
                    }
                    polygon {
                        points: "15,9 15.94,6.93 18,6 15.94,5.07 15,3 14.08,5.07 12,6 14.08,6.93",
                    }
                    polygon {
                        points: "3.5,11 4,9 6,8.5 4,8 3.5,6 3,8 1,8.5 3,9",
                    }
                }
            }
        }
    }
}

pub fn inventory_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M21,11.5l1.5,1.5l-6.99,7L11,15.5l1.5-1.5l3.01,3L21,11.5z",
                    }
                    g {
                        polygon {
                            enable_background: "new",
                            points: "17,5 17,8 7,8 7,5 5,5 5,19 11.68,19 8.17,15.5 12.5,11.17 15.51,14.17 19,10.67 19,5",
                            opacity: ".3",
                        }
                        path {
                            d: "M5,19V5h2v3h10V5h2v5.67l2-2l0,0V5c0-1.1-0.9-2-2-2h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H5C3.9,3,3,3.9,3,5v14 c0,1.1,0.9,2,2,2h8.68l-2-2H5z M12,3c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,3,12,3z",
                        }
                    }
                }
            }
        }
    }
}

pub fn inventory_2_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        enable_background: "new",
                        opacity: ".3",
                        d: "M4.5,16.5h11V8h-11V16.5z M8,10h4v1.5H8V10z",
                    }
                    rect {
                        height: "3",
                        width: "13",
                        y: "3.5",
                        opacity: ".3",
                        x: "3.5",
                        enable_background: "new",
                    }
                    path {
                        d: "M16.5,2h-13C2.67,2,2,2.67,2,3.5v3c0,0.65,0.42,1.2,1,1.41v8.59C3,17.33,3.67,18,4.5,18h11c0.83,0,1.5-0.67,1.5-1.5V7.91 c0.58-0.21,1-0.76,1-1.41v-3C18,2.67,17.33,2,16.5,2z M15.5,16.5h-11V8h11V16.5z M16.5,6.5h-13v-3h13V6.5z",
                    }
                    rect {
                        y: "10",
                        x: "8",
                        width: "4",
                        height: "1.5",
                    }
                }
            }
        }
    }
}

pub fn inventory_2_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    polygon {
                        enable_background: "new",
                        opacity: ".3",
                        points: "4,7 20,7 20,3.98 4,4",
                    }
                    path {
                        opacity: ".3",
                        enable_background: "new",
                        d: "M5,20h14V9H5V20z M9,12h6v2H9V12z",
                    }
                    path {
                        d: "M20,2H4C3,2,2,2.9,2,4v3.01C2,7.73,2.43,8.35,3,8.7V20c0,1.1,1.1,2,2,2h14c0.9,0,2-0.9,2-2V8.7c0.57-0.35,1-0.97,1-1.69V4 C22,2.9,21,2,20,2z M19,20H5V9h14V20z M20,7H4V4l16-0.02V7z",
                    }
                    rect {
                        height: "2",
                        x: "9",
                        y: "12",
                        width: "6",
                    }
                }
            }
        }
    }
}

pub fn link_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 7h-4v2h4c1.65 0 3 1.35 3 3s-1.35 3-3 3h-4v2h4c2.76 0 5-2.24 5-5s-2.24-5-5-5zm-6 8H7c-1.65 0-3-1.35-3-3s1.35-3 3-3h4V7H7c-2.76 0-5 2.24-5 5s2.24 5 5 5h4v-2zm-3-4h8v2H8z",
                opacity: ".87",
            }
        }
    }
}

pub fn link_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14.39 11L16 12.61V11zM17 7h-4v1.9h4c1.71 0 3.1 1.39 3.1 3.1 0 1.27-.77 2.37-1.87 2.84l1.4 1.4C21.05 15.36 22 13.79 22 12c0-2.76-2.24-5-5-5zM2 4.27l3.11 3.11C3.29 8.12 2 9.91 2 12c0 2.76 2.24 5 5 5h4v-1.9H7c-1.71 0-3.1-1.39-3.1-3.1 0-1.59 1.21-2.9 2.76-3.07L8.73 11H8v2h2.73L13 15.27V17h1.73l4.01 4.01 1.41-1.41L3.41 2.86 2 4.27z",
            }
        }
    }
}

pub fn low_priority_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14 5h8v2h-8V5zm0 5.5h8v2h-8v-2zm0 5.5h8v2h-8v-2zM2 11.5C2 15.08 4.92 18 8.5 18H9v2l3-3-3-3v2h-.5C6.02 16 4 13.98 4 11.5S6.02 7 8.5 7H12V5H8.5C4.92 5 2 7.92 2 11.5z",
            }
        }
    }
}

pub fn mail_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M20 6H4l8 4.99zM4 8v10h16V8l-8 5z",
            }
            path {
                d: "M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 2l-8 4.99L4 6h16zm0 12H4V8l8 5 8-5v10z",
            }
        }
    }
}

pub fn markunread_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 6H4l8 5zM4 8v10h16V8l-8 5z",
                opacity: ".3",
            }
            path {
                d: "M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 2l-8 5-8-5h16zm0 12H4V8l8 5 8-5v10z",
            }
        }
    }
}

pub fn move_to_inbox_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12.01 18c-1.48 0-2.75-.81-3.45-2H5v3h14v-3h-3.55c-.69 1.19-1.97 2-3.44 2z",
                opacity: ".3",
            }
            path {
                d: "M16 9h-2.55V6h-2.9v3H8l4 4zm3-6H4.99C3.88 3 3 3.9 3 5v14c0 1.1.88 2 1.99 2H19c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5v-3h3.56c.69 1.19 1.97 2 3.45 2s2.75-.81 3.45-2H19v3zm0-5h-4.99c0 1.1-.9 2-2 2s-2-.9-2-2H5l-.01-9H19v9z",
            }
        }
    }
}

pub fn next_week_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M4 20h16V9H4v11zm6-8.5l1-1 4 4-4 4-1-1 3-3-3-3z",
            }
            path {
                d: "M11 18.5l4-4-4-4-1 1 3 3-3 3zM20 7h-4V5c0-.55-.22-1.05-.59-1.41C15.05 3.22 14.55 3 14 3h-4c-1.1 0-2 .9-2 2v2H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2zM10 5h4v2h-4V5zm10 15H4V9h16v11z",
            }
        }
    }
}

pub fn outlined_flag_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14 6l-1-2H5v17h2v-7h5l1 2h7V6h-6zm4 8h-4l-1-2H7V6h5l1 2h5v6z",
            }
        }
    }
}

pub fn policy_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                g {
                    path {
                        d: "M5,5.72v3.59c0,3.05,2.13,5.86,5,6.65c1.25-0.34,2.35-1.09,3.2-2.06l-1.47-1.47C11.24,12.79,10.65,13,10,13 c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3c0,0.65-0.21,1.24-0.56,1.73l1.38,1.38C14.55,12,15,10.68,15,9.31V5.72l-5-1.67L5,5.72z",
                        opacity: ".3",
                    }
                    path {
                        d: "M10,3L4,5v4.31c0,3.55,2.56,6.88,6,7.69c3.44-0.81,6-4.13,6-7.69V5L10,3z M15,9.31c0,1.37-0.45,2.68-1.18,3.8l-1.38-1.38 C12.79,11.24,13,10.65,13,10c0-1.66-1.34-3-3-3s-3,1.34-3,3s1.34,3,3,3c0.65,0,1.24-0.21,1.73-0.56l1.47,1.47 c-0.86,0.97-1.95,1.71-3.2,2.06c-2.87-0.79-5-3.61-5-6.65V5.72l5-1.67l5,1.67V9.31z M10,12c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2 s2,0.9,2,2C12,11.1,11.1,12,10,12z",
                    }
                }
            }
        }
    }
}

pub fn policy_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                g {
                    path {
                        opacity: ".3",
                        d: "M5,6.3V11c0,4.52,2.98,8.69,7,9.93c1.74-0.53,3.28-1.62,4.47-3.04l-1.72-1.72 c-1.94,1.29-4.58,1.07-6.29-0.64c-1.95-1.95-1.95-5.12,0-7.07c1.95-1.95,5.12-1.95,7.07,0c1.71,1.71,1.92,4.35,0.64,6.29 l1.45,1.45C18.49,14.65,19,12.85,19,11V6.3l-7-3.11L5,6.3z",
                    }
                    path {
                        d: "M12,1L3,5v6c0,5.55,3.84,10.74,9,12c0.65-0.16,1.27-0.38,1.87-0.65c1.8-0.82,3.36-2.13,4.57-3.74 C20.04,16.46,21,13.77,21,11V5L12,1z M19,11c0,1.85-0.51,3.65-1.38,5.21l-1.45-1.45c1.29-1.94,1.07-4.58-0.64-6.29 c-1.95-1.95-5.12-1.95-7.07,0c-1.95,1.95-1.95,5.12,0,7.07c1.71,1.71,4.35,1.92,6.29,0.64l1.72,1.72 c-1.19,1.42-2.73,2.51-4.47,3.04C7.98,19.69,5,15.52,5,11V6.3l7-3.11l7,3.11V11z M15,12c0,1.66-1.34,3-3,3s-3-1.34-3-3s1.34-3,3-3 S15,10.34,15,12z",
                    }
                }
            }
        }
    }
}

pub fn push_pin_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,4H8v4c0,0.77-0.29,1.47-0.77,2h5.53C12.29,9.47,12,8.77,12,8V4z",
                        opacity: ".3",
                    }
                    path {
                        d: "M13,8V4h0.5C13.78,4,14,3.78,14,3.5C14,3.22,13.78,3,13.5,3h-7C6.22,3,6,3.22,6,3.5C6,3.78,6.22,4,6.5,4H7v4 c0,1.1-0.9,2-2,2v1h4.5v5.5L10,17l0.5-0.5V11H15v-1C13.9,10,13,9.1,13,8z M7.23,10C7.71,9.47,8,8.77,8,8V4h4v4 c0,0.77,0.29,1.47,0.77,2H7.23z",
                    }
                }
            }
        }
    }
}

pub fn push_pin_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14,4h-4v5c0,1.1-0.35,2.14-1,3h6c-0.63-0.84-1-1.88-1-3V4z",
                        opacity: ".3",
                    }
                    path {
                        d: "M19,12c-1.66,0-3-1.34-3-3V4l1,0c0,0,0,0,0,0c0.55,0,1-0.45,1-1s-0.45-1-1-1H7C6.45,2,6,2.45,6,3s0.45,1,1,1c0,0,0,0,0,0 l1,0v5c0,1.66-1.34,3-3,3v2h5.97v7l1,1l1-1v-7H19L19,12C19,12,19,12,19,12z M9,12c0.65-0.86,1-1.9,1-3V4h4v5c0,1.12,0.37,2.16,1,3 H9z",
                    }
                }
            }
        }
    }
}

pub fn redo_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.4 10.6C16.55 8.99 14.15 8 11.5 8c-4.65 0-8.58 3.03-9.96 7.22L3.9 16c1.05-3.19 4.05-5.5 7.6-5.5 1.95 0 3.73.72 5.12 1.88L13 16h9V7l-3.6 3.6z",
            }
        }
    }
}

pub fn remove_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 13H5v-2h14v2z",
            }
        }
    }
}

pub fn remove_circle_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 4c-4.41 0-8 3.59-8 8s3.59 8 8 8 8-3.59 8-8-3.59-8-8-8zm5 9H7v-2h10v2z",
                opacity: ".3",
            }
            path {
                d: "M7 11h10v2H7zm5-9C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

pub fn remove_circle_outline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7 11h10v2H7zm5-9C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

pub fn reply_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M10 9V5l-7 7 7 7v-4.1c5 0 8.5 1.6 11 5.1-1-5-4-10-11-11z",
            }
        }
    }
}

pub fn reply_all_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7 8V5l-7 7 7 7v-3l-4-4 4-4zm6 1V5l-7 7 7 7v-4.1c5 0 8.5 1.6 11 5.1-1-5-4-10-11-11z",
            }
        }
    }
}

pub fn report_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9.1 5L5 9.1v5.8L9.1 19h5.8l4.1-4.1V9.1L14.9 5H9.1zM12 17c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm1-3h-2V7h2v7z",
                opacity: ".3",
            }
            path {
                d: "M15.73 3H8.27L3 8.27v7.46L8.27 21h7.46L21 15.73V8.27L15.73 3zM19 14.9L14.9 19H9.1L5 14.9V9.1L9.1 5h5.8L19 9.1v5.8z",
            }
            circle {
                cy: "16",
                cx: "12",
                r: "1",
            }
            path {
                d: "M11 7h2v7h-2z",
            }
        }
    }
}

pub fn report_gmailerrorred_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15.73 3H8.27L3 8.27v7.46L8.27 21h7.46L21 15.73V8.27L15.73 3zM19 14.9L14.9 19H9.1L5 14.9V9.1L9.1 5h5.8L19 9.1v5.8z",
            }
            circle {
                cx: "12",
                cy: "16",
                r: "1",
            }
            path {
                d: "M11 7h2v7h-2z",
            }
        }
    }
}

pub fn report_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 9.1L14.9 5H9.1l-.22.22L11 7.33V7h2v2.33l5.78 5.79.22-.22zM6.05 8.04L5 9.1v5.8L9.1 19h5.8l1.05-1.05-9.9-9.91zM13 16c0 .55-.45 1-1 1s-1-.45-1-1 .45-1 1-1 1 .45 1 1z",
                opacity: ".3",
            }
            path {
                d: "M9.1 5h5.8L19 9.1v5.8l-.22.22 1.42 1.41.8-.8V8.27L15.73 3H8.27l-.8.8 1.41 1.42z",
            }
            circle {
                cy: "16",
                r: "1",
                cx: "12",
            }
            path {
                d: "M13 7h-2v.33l2 2zM2.41 1.58L1 2.99l3.64 3.64L3 8.27v7.46L8.27 21h7.46l1.64-1.64L21.01 23l1.41-1.41L2.41 1.58zM14.9 19H9.1L5 14.9V9.1l1.05-1.05 9.9 9.9L14.9 19z",
            }
        }
    }
}

pub fn save_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5 5v14h14V7.83L16.17 5H5zm7 13c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-8H6V6h9v4z",
                opacity: ".3",
            }
            path {
                d: "M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm2 16H5V5h11.17L19 7.83V19zm-7-7c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3zM6 6h9v4H6z",
            }
        }
    }
}

pub fn save_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 12v7H5v-7H3v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-7h-2zm-6 .67l2.59-2.58L17 11.5l-5 5-5-5 1.41-1.41L11 12.67V3h2v9.67z",
            }
        }
    }
}

pub fn save_as_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13.38,4.5H4.5v11h8.47l2.53-2.54V6.62L13.38,4.5z M10,14.75c-1.24,0-2.25-1.01-2.25-2.25 s1.01-2.25,2.25-2.25s2.25,1.01,2.25,2.25S11.24,14.75,10,14.75z M12.5,8.5h-7v-3h7V8.5z",
                    opacity: ".3",
                }
                path {
                    d: "M17,11.46V6l-3-3H4.5C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h6.97l1.5-1.5H4.5v-11h8.88l2.12,2.12v6.34L17,11.46z M12.5,8.5h-7v-3h7V8.5z M12.25,12.5c0,1.24-1.01,2.25-2.25,2.25s-2.25-1.01-2.25-2.25s1.01-2.25,2.25-2.25S12.25,11.26,12.25,12.5 z M16.82,13.77l1.41,1.41L14.41,19H13l0-1.41L16.82,13.77z M17.31,13.27l0.63-0.63c0.2-0.2,0.51-0.2,0.71,0l0.71,0.71 c0.2,0.2,0.2,0.51,0,0.71l-0.63,0.63L17.31,13.27z",
                }
            }
        }
    }
}

pub fn save_as_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.17,5H5v14h9.4l4.6-4.6V7.83L16.17,5z M12,18c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S13.66,18,12,18z M15,10H6V6h9V10z",
                    opacity: ".3",
                }
                path {
                    d: "M21,12.4V7l-4-4H5C3.89,3,3,3.9,3,5v14c0,1.1,0.89,2,2,2h7.4l2-2H5V5h11.17L19,7.83v6.57L21,12.4z M15,15 c0,1.66-1.34,3-3,3s-3-1.34-3-3s1.34-3,3-3S15,13.34,15,15z M6,6h9v4H6V6z M19.99,16.25l1.77,1.77L16.77,23H15v-1.77L19.99,16.25z M23.25,16.51l-0.85,0.85l-1.77-1.77l0.85-0.85c0.2-0.2,0.51-0.2,0.71,0l1.06,1.06C23.45,16,23.45,16.32,23.25,16.51z",
                }
            }
        }
    }
}

pub fn select_all_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3 5h2V3c-1.1 0-2 .9-2 2zm0 8h2v-2H3v2zm4 8h2v-2H7v2zM3 9h2V7H3v2zm10-6h-2v2h2V3zm6 0v2h2c0-1.1-.9-2-2-2zM5 21v-2H3c0 1.1.9 2 2 2zm-2-4h2v-2H3v2zM9 3H7v2h2V3zm2 18h2v-2h-2v2zm8-8h2v-2h-2v2zm0 8c1.1 0 2-.9 2-2h-2v2zm0-12h2V7h-2v2zm0 8h2v-2h-2v2zm-4 4h2v-2h-2v2zm0-16h2V3h-2v2zM7 17h10V7H7v10zm2-8h6v6H9V9z",
            }
        }
    }
}

pub fn send_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M4 8.25l7.51 1-7.5-3.22zm.01 9.72l7.5-3.22-7.51 1z",
                opacity: ".3",
            }
            path {
                d: "M2.01 3L2 10l15 2-15 2 .01 7L23 12 2.01 3zM4 8.25V6.03l7.51 3.22-7.51-1zm.01 9.72v-2.22l7.51-1-7.51 3.22z",
            }
        }
    }
}

pub fn shield_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        enable_background: "new",
                        opacity: ".3",
                        d: "M6,6.39v4.7c0,4,2.55,7.7,6,8.83c3.45-1.13,6-4.82,6-8.83v-4.7l-6-2.25L6,6.39 z",
                    }
                    path {
                        d: "M12,2L4,5v6.09c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91V5L12,2z M18,11.09c0,4-2.55,7.7-6,8.83 c-3.45-1.13-6-4.82-6-8.83v-4.7l6-2.25l6,2.25V11.09z",
                    }
                }
            }
        }
    }
}

pub fn sort_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3 18h6v-2H3v2zM3 6v2h18V6H3zm0 7h12v-2H3v2z",
            }
        }
    }
}

pub fn square_foot_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    polygon {
                        points: "6,8.83 6,14 11.17,14",
                        opacity: ".3",
                    }
                    g {
                        path {
                            d: "M14.24,14.24l-0.71,0.71l-0.35-0.35l0.71-0.71l-1.68-1.68l-0.71,0.71l-0.35-0.35l0.71-0.71l-1.68-1.68l-0.71,0.71 l-0.35-0.35l0.71-0.71L8.14,8.14L7.44,8.85L7.08,8.5l0.71-0.71L6.11,6.11L5.4,6.82L5.05,6.46l0.71-0.71L4,4v10.5 C4,15.33,4.67,16,5.5,16H16L14.24,14.24z M6,14V8.83L11.17,14H6z",
                        }
                    }
                }
            }
        }
    }
}

pub fn square_foot_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        opacity: ".3",
                        points: "7,17 12.76,17 7,11.24",
                    }
                    g {
                        path {
                            d: "M17.66,17.66l-1.06,1.06l-0.71-0.71l1.06-1.06l-1.94-1.94l-1.06,1.06l-0.71-0.71l1.06-1.06l-1.94-1.94l-1.06,1.06 l-0.71-0.71l1.06-1.06L9.7,9.7l-1.06,1.06l-0.71-0.71l1.06-1.06L7.05,7.05L5.99,8.11L5.28,7.4l1.06-1.06L4,4v14c0,1.1,0.9,2,2,2 h14L17.66,17.66z M7,17v-5.76L12.76,17H7z",
                        }
                    }
                }
            }
        }
    }
}

pub fn stacked_bar_chart_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    rect {
                        y: "12",
                        height: "4",
                        width: "3",
                        x: "13",
                    }
                    rect {
                        x: "4",
                        width: "3",
                        height: "8",
                        y: "8",
                    }
                    rect {
                        x: "4",
                        y: "4",
                        width: "3",
                        height: "3",
                    }
                    rect {
                        y: "6",
                        height: "3",
                        width: "3",
                        x: "8.5",
                    }
                    rect {
                        y: "8",
                        x: "13",
                        height: "3",
                        width: "3",
                    }
                    rect {
                        height: "6",
                        width: "3",
                        x: "8.5",
                        y: "10",
                    }
                }
            }
        }
    }
}

pub fn stacked_bar_chart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    rect {
                        height: "11",
                        width: "4",
                        x: "4",
                        y: "9",
                    }
                    rect {
                        x: "4",
                        y: "4",
                        height: "4",
                        width: "4",
                    }
                    rect {
                        x: "10",
                        height: "4",
                        width: "4",
                        y: "7",
                    }
                    rect {
                        width: "4",
                        x: "16",
                        height: "4",
                        y: "10",
                    }
                    rect {
                        height: "5",
                        width: "4",
                        y: "15",
                        x: "16",
                    }
                    rect {
                        height: "8",
                        x: "10",
                        y: "12",
                        width: "4",
                    }
                }
            }
        }
    }
}

pub fn stream_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cy: "12",
                        r: "2",
                        cx: "20",
                    }
                    circle {
                        cx: "4",
                        r: "2",
                        cy: "12",
                    }
                    circle {
                        r: "2",
                        cx: "12",
                        cy: "20",
                    }
                    rect {
                        transform: "matrix(0.7081 -0.7061 0.7061 0.7081 -0.1146 13.9802)",
                        height: "2",
                        width: "6.22",
                        x: "13.74",
                        y: "6.13",
                    }
                    polygon {
                        points: "8.32,9.68 8.63,10 10.05,8.59 6.03,4.55 6.02,4.55 5.71,4.23 4.29,5.64 8.31,9.69",
                    }
                    polygon {
                        points: "15.41,13.94 14,15.35 17.99,19.36 18.34,19.71 19.76,18.3 15.77,14.29",
                    }
                    polygon {
                        points: "8.59,13.95 4.56,17.96 4.24,18.29 5.65,19.7 9.68,15.68 10.01,15.36",
                    }
                    circle {
                        cy: "4",
                        r: "2",
                        cx: "12",
                    }
                }
            }
        }
    }
}

pub fn tag_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,10V8h-4V4h-2v4h-4V4H8v4H4v2h4v4H4v2h4v4h2v-4h4v4h2v-4h4v-2h-4v-4H20z M14,14h-4v-4h4V14z",
                }
            }
        }
    }
}

pub fn text_format_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5 17v2h14v-2H5zm4.5-4.2h5l.9 2.2h2.1L12.75 4h-1.5L6.5 15h2.1l.9-2.2zM12 5.98L13.87 11h-3.74L12 5.98z",
            }
        }
    }
}

pub fn unarchive_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5 19h14V8H5v11zm7-9l4 4h-2.55v3h-2.91v-3H8l4-4z",
                opacity: ".3",
            }
            path {
                d: "M20.54 5.23l-1.39-1.68C18.88 3.21 18.47 3 18 3H6c-.47 0-.88.21-1.16.55L3.46 5.23C3.17 5.57 3 6.02 3 6.5V19c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V6.5c0-.48-.17-.93-.46-1.27zM6.24 5h11.52l.83 1H5.42l.82-1zM19 19H5V8h14v11zm-8.45-2h2.9v-3H16l-4-4-4 4h2.55z",
            }
        }
    }
}

pub fn undo_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12.5 8c-2.65 0-5.05.99-6.9 2.6L2 7v9h9l-3.62-3.62c1.39-1.16 3.16-1.88 5.12-1.88 3.54 0 6.55 2.31 7.6 5.5l2.37-.78C21.08 11.03 17.15 8 12.5 8z",
            }
        }
    }
}

pub fn upcoming_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        opacity: ".3",
                        d: "M12,17c-2.04,0-3.81-1.24-4.58-3H4v5h16v-5h-3.42C15.81,15.76,14.04,17,12,17z",
                    }
                    path {
                        d: "M21.16,7.26l-1.41-1.41L16.19,9.4l1.41,1.41C17.6,10.81,21.05,7.29,21.16,7.26z",
                    }
                    rect {
                        x: "11",
                        width: "2",
                        y: "3",
                        height: "5",
                    }
                    path {
                        d: "M20,12h-5c0,1.66-1.34,3-3,3s-3-1.34-3-3H4c-1.1,0-2,0.9-2,2v5c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-5 C22,12.9,21.1,12,20,12z M20,19H4v-5h3.42c0.77,1.76,2.54,3,4.58,3s3.81-1.24,4.58-3H20V19z",
                    }
                    path {
                        d: "M6.4,10.81L7.81,9.4L4.26,5.84L2.84,7.26C2.95,7.29,6.4,10.81,6.4,10.81z",
                    }
                }
            }
        }
    }
}

pub fn waves_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 16.99c-1.35 0-2.2.42-2.95.8-.65.33-1.18.6-2.05.6-.9 0-1.4-.25-2.05-.6-.75-.38-1.57-.8-2.95-.8s-2.2.42-2.95.8c-.65.33-1.17.6-2.05.6v1.95c1.35 0 2.2-.42 2.95-.8.65-.33 1.17-.6 2.05-.6s1.4.25 2.05.6c.75.38 1.57.8 2.95.8s2.2-.42 2.95-.8c.65-.33 1.18-.6 2.05-.6.9 0 1.4.25 2.05.6.75.38 1.58.8 2.95.8v-1.95c-.9 0-1.4-.25-2.05-.6-.75-.38-1.6-.8-2.95-.8zm0-4.45c-1.35 0-2.2.43-2.95.8-.65.32-1.18.6-2.05.6-.9 0-1.4-.25-2.05-.6-.75-.38-1.57-.8-2.95-.8s-2.2.43-2.95.8c-.65.32-1.17.6-2.05.6v1.95c1.35 0 2.2-.43 2.95-.8.65-.35 1.15-.6 2.05-.6s1.4.25 2.05.6c.75.38 1.57.8 2.95.8s2.2-.43 2.95-.8c.65-.35 1.15-.6 2.05-.6s1.4.25 2.05.6c.75.38 1.58.8 2.95.8v-1.95c-.9 0-1.4-.25-2.05-.6-.75-.38-1.6-.8-2.95-.8zm2.95-8.08c-.75-.38-1.58-.8-2.95-.8s-2.2.42-2.95.8c-.65.32-1.18.6-2.05.6-.9 0-1.4-.25-2.05-.6-.75-.37-1.57-.8-2.95-.8s-2.2.42-2.95.8c-.65.33-1.17.6-2.05.6v1.93c1.35 0 2.2-.43 2.95-.8.65-.33 1.17-.6 2.05-.6s1.4.25 2.05.6c.75.38 1.57.8 2.95.8s2.2-.43 2.95-.8c.65-.32 1.18-.6 2.05-.6.9 0 1.4.25 2.05.6.75.38 1.58.8 2.95.8V5.04c-.9 0-1.4-.25-2.05-.58zM17 8.09c-1.35 0-2.2.43-2.95.8-.65.35-1.15.6-2.05.6s-1.4-.25-2.05-.6c-.75-.38-1.57-.8-2.95-.8s-2.2.43-2.95.8c-.65.35-1.15.6-2.05.6v1.95c1.35 0 2.2-.43 2.95-.8.65-.32 1.18-.6 2.05-.6s1.4.25 2.05.6c.75.38 1.57.8 2.95.8s2.2-.43 2.95-.8c.65-.32 1.18-.6 2.05-.6.9 0 1.4.25 2.05.6.75.38 1.58.8 2.95.8V9.49c-.9 0-1.4-.25-2.05-.6-.75-.38-1.6-.8-2.95-.8z",
            }
        }
    }
}

pub fn web_stories_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        y: "3.5",
                        opacity: ".3",
                        height: "13",
                        width: "7.5",
                        x: "3.5",
                    }
                    path {
                        d: "M11,2H3.5C2.67,2,2,2.67,2,3.5v13C2,17.33,2.67,18,3.5,18H11c0.83,0,1.5-0.67,1.5-1.5v-13C12.5,2.67,11.83,2,11,2z M11,16.5H3.5v-13H11V16.5z",
                    }
                    path {
                        d: "M14,4v12c0.83,0,1.5-0.67,1.5-1.5v-9C15.5,4.67,14.83,4,14,4z",
                    }
                    path {
                        d: "M17,5v10c0.58-0.21,1-0.76,1-1.41V6.41C18,5.76,17.58,5.21,17,5z",
                    }
                }
            }
        }
    }
}

pub fn web_stories_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    rect {
                        opacity: ".3",
                        height: "16",
                        y: "4",
                        width: "9",
                        x: "4",
                    }
                    path {
                        d: "M17,4v16c1.1,0,2-0.9,2-2V6C19,4.9,18.1,4,17,4z",
                    }
                    path {
                        d: "M13,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h9c1.1,0,2-0.9,2-2V4C15,2.9,14.1,2,13,2z M13,20H4V4h9V20z",
                    }
                    path {
                        d: "M21,6v12c0.83,0,1.5-0.67,1.5-1.5v-9C22.5,6.67,21.83,6,21,6z",
                    }
                }
            }
        }
    }
}

pub fn weekend_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 11c-.55 0-1 .45-1 1v4H4v-4c0-.55-.45-1-1-1s-1 .45-1 1v5c0 .55.45 1 1 1h18c.55 0 1-.45 1-1v-5c0-.55-.45-1-1-1zM6 14h12v-2c0-.88.39-1.67 1-2.22V7c0-.55-.45-1-1-1H6c-.55 0-1 .45-1 1v2.78c.61.55 1 1.34 1 2.22v2z",
                opacity: ".3",
            }
            path {
                d: "M21 9V7c0-1.65-1.35-3-3-3H6C4.35 4 3 5.35 3 7v2c-1.65 0-3 1.35-3 3v5c0 1.65 1.35 3 3 3h18c1.65 0 3-1.35 3-3v-5c0-1.65-1.35-3-3-3zM5 7c0-.55.45-1 1-1h12c.55 0 1 .45 1 1v2.78c-.61.55-1 1.34-1 2.22v2H6v-2c0-.88-.39-1.67-1-2.22V7zm17 10c0 .55-.45 1-1 1H3c-.55 0-1-.45-1-1v-5c0-.55.45-1 1-1s1 .45 1 1v4h16v-4c0-.55.45-1 1-1s1 .45 1 1v5z",
            }
        }
    }
}

pub fn where_to_vote_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".3",
                d: "M12 3C8.69 3 6 5.69 6 9c0 3.54 3.82 8.86 6 11.47 1.75-2.11 6-7.63 6-11.47 0-3.31-2.69-6-6-6zm-1.53 11l-3.18-3.18L8.71 9.4l1.77 1.77 4.6-4.6 1.41 1.41L10.47 14z",
            }
            path {
                d: "M12 1C7.59 1 4 4.59 4 9c0 5.57 6.96 13.34 7.26 13.67l.74.82.74-.82C13.04 22.34 20 14.57 20 9c0-4.41-3.59-8-8-8zm0 19.47C9.82 17.86 6 12.54 6 9c0-3.31 2.69-6 6-6s6 2.69 6 6c0 3.83-4.25 9.36-6 11.47zm3.07-13.9l-4.6 4.6L8.71 9.4l-1.42 1.42L10.47 14l6.01-6.01z",
            }
        }
    }
}

