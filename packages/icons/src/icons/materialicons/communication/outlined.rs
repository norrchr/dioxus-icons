use dioxus::prelude::*;
use crate::IconProps;
pub fn _3p_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,2H4.01c-1.1,0-2,0.9-2,2L2,22l4-4h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,16H5.17L4,17.17V4h16V16z M12,10 c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S10.9,10,12,10z M16,13.43c0-0.81-0.48-1.53-1.22-1.85C13.93,11.21,12.99,11,12,11 c-0.99,0-1.93,0.21-2.78,0.58C8.48,11.9,8,12.62,8,13.43V14h8V13.43z",
                }
            }
        }
    }
}

pub fn add_ic_call_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 15.45c-1.25 0-2.45-.2-3.57-.57-.1-.03-.21-.05-.31-.05-.26 0-.51.1-.71.29l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.4 8.5 5.2 8.5 3.95c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM5.03 4.95h1.5c.07.88.22 1.75.45 2.58l-1.2 1.21c-.4-1.21-.66-2.47-.75-3.79zM19 18.92c-1.32-.09-2.6-.35-3.8-.76l1.2-1.2c.85.24 1.72.39 2.6.45v1.51zM18 5.95v-3h-2v3h-3v2h3v3h2v-3h3v-2z",
            }
        }
    }
}

pub fn alternate_email_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 1.95c-5.52 0-10 4.48-10 10s4.48 10 10 10h5v-2h-5c-4.34 0-8-3.66-8-8s3.66-8 8-8 8 3.66 8 8v1.43c0 .79-.71 1.57-1.5 1.57s-1.5-.78-1.5-1.57v-1.43c0-2.76-2.24-5-5-5s-5 2.24-5 5 2.24 5 5 5c1.38 0 2.64-.56 3.54-1.47.65.89 1.77 1.47 2.96 1.47 1.97 0 3.5-1.6 3.5-3.57v-1.43c0-5.52-4.48-10-10-10zm0 13c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z",
            }
        }
    }
}

pub fn app_registration_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        rect {
                            width: "3",
                            height: "3",
                            x: "8.5",
                            y: "4",
                        }
                    }
                    g {
                        rect {
                            height: "3",
                            width: "3",
                            x: "4",
                            y: "13",
                        }
                    }
                    g {
                        rect {
                            height: "3",
                            width: "3",
                            y: "8.5",
                            x: "4",
                        }
                    }
                    g {
                        rect {
                            y: "4",
                            width: "3",
                            x: "4",
                            height: "3",
                        }
                    }
                    g {
                        rect {
                            height: "3",
                            width: "3",
                            x: "13",
                            y: "4",
                        }
                    }
                    g {
                        polygon {
                            points: "9,14.49 9,16 10.51,16 15.4,11.1 13.9,9.6",
                        }
                    }
                    g {
                        polygon {
                            points: "11.5,9.88 11.5,8.5 8.5,8.5 8.5,11.5 9.87,11.5",
                        }
                    }
                    g {
                        path {
                            d: "M16.85,8.94l-0.79-0.79c-0.2-0.2-0.51-0.2-0.71,0L14.6,8.9l1.5,1.5l0.75-0.75C17.05,9.45,17.05,9.13,16.85,8.94z",
                        }
                    }
                }
            }
        }
    }
}

pub fn app_registration_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    rect {
                        y: "4",
                        width: "4",
                        height: "4",
                        x: "10",
                    }
                    rect {
                        height: "4",
                        width: "4",
                        y: "16",
                        x: "4",
                    }
                    rect {
                        width: "4",
                        x: "4",
                        y: "10",
                        height: "4",
                    }
                    rect {
                        x: "4",
                        width: "4",
                        y: "4",
                        height: "4",
                    }
                    rect {
                        width: "4",
                        y: "4",
                        height: "4",
                        x: "16",
                    }
                    polygon {
                        points: "11,17.86 11,20 13.1,20 19.08,14.03 16.96,11.91",
                    }
                    polygon {
                        points: "14,12.03 14,10 10,10 10,14 12.03,14",
                    }
                    path {
                        d: "M20.85,11.56l-1.41-1.41c-0.2-0.2-0.51-0.2-0.71,0l-1.06,1.06l2.12,2.12l1.06-1.06C21.05,12.07,21.05,11.76,20.85,11.56z",
                    }
                }
            }
        }
    }
}

pub fn business_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 7V3H2v18h20V7H12zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm0-4H4V9h2v2zm0-4H4V5h2v2zm4 12H8v-2h2v2zm0-4H8v-2h2v2zm0-4H8V9h2v2zm0-4H8V5h2v2zm10 12h-8v-2h2v-2h-2v-2h2v-2h-2V9h8v10zm-2-8h-2v2h2v-2zm0 4h-2v2h2v-2z",
            }
        }
    }
}

pub fn call_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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

pub fn call_end_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.59 10.52c1.05.51 2.04 1.15 2.96 1.91l-1.07 1.07c-.58-.47-1.21-.89-1.88-1.27v-1.71m-13.2 0v1.7c-.65.37-1.28.79-1.87 1.27l-1.07-1.07c.91-.75 1.9-1.38 2.94-1.9M12 7C7.46 7 3.34 8.78.29 11.67c-.18.18-.29.43-.29.71s.11.53.29.7l2.48 2.48c.18.18.43.29.71.29.27 0 .52-.1.7-.28.79-.73 1.68-1.36 2.66-1.85.33-.16.56-.51.56-.9v-3.1C8.85 9.25 10.4 9 12 9s3.15.25 4.59.73v3.1c0 .4.23.74.56.9.98.49 1.88 1.11 2.67 1.85.18.17.43.28.7.28.28 0 .53-.11.71-.29l2.48-2.48c.18-.18.29-.43.29-.71s-.11-.53-.29-.71C20.66 8.78 16.54 7 12 7z",
            }
        }
    }
}

pub fn call_made_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9 5v2h6.59L4 18.59 5.41 20 17 8.41V15h2V5H9z",
            }
        }
    }
}

pub fn call_merge_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 20.41L18.41 19 15 15.59 13.59 17 17 20.41zM7.5 8H11v5.59L5.59 19 7 20.41l6-6V8h3.5L12 3.5 7.5 8z",
            }
        }
    }
}

pub fn call_missed_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19.59 7L12 14.59 6.41 9H11V7H3v8h2v-4.59l7 7 9-9L19.59 7z",
            }
        }
    }
}

pub fn call_missed_outgoing_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3 8.41l9 9 7-7V15h2V7h-8v2h4.59L12 14.59 4.41 7 3 8.41z",
            }
        }
    }
}

pub fn call_received_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 5.41L18.59 4 7 15.59V9H5v10h10v-2H8.41L20 5.41z",
            }
        }
    }
}

pub fn call_split_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14 4l2.29 2.29-2.88 2.88 1.42 1.42 2.88-2.88L20 10V4h-6zm-4 0H4v6l2.29-2.29 4.71 4.7V20h2v-8.41l-5.29-5.3L10 4z",
            }
        }
    }
}

pub fn cancel_presentation_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H3V5h18v14zM9.41 15.95L12 13.36l2.59 2.59L16 14.54l-2.59-2.59L16 9.36l-1.41-1.41L12 10.54 9.41 7.95 8 9.36l2.59 2.59L8 14.54z",
            }
        }
    }
}

pub fn cell_tower_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M6.5,11.5l1.06-1.06C6.94,9.81,6.55,8.95,6.55,8s0.39-1.81,1.01-2.44L6.5,4.5C5.6,5.4,5.05,6.63,5.05,8S5.6,10.6,6.5,11.5 z",
                    }
                    path {
                        d: "M5.41,3.41L4.34,2.34C2.9,3.79,2,5.79,2,8s0.9,4.21,2.34,5.66l1.06-1.06C4.23,11.42,3.5,9.79,3.5,8S4.23,4.58,5.41,3.41z",
                    }
                    path {
                        d: "M13.5,4.5l-1.06,1.06c0.62,0.62,1.01,1.49,1.01,2.44s-0.39,1.81-1.01,2.44l1.06,1.06c0.9-0.9,1.45-2.13,1.45-3.5 S14.4,5.4,13.5,4.5z",
                    }
                    path {
                        d: "M15.66,2.34l-1.06,1.06C15.77,4.58,16.5,6.21,16.5,8s-0.73,3.42-1.91,4.59l1.06,1.06C17.1,12.21,18,10.21,18,8 S17.1,3.79,15.66,2.34z",
                    }
                    path {
                        d: "M12,8c0-1.1-0.9-2-2-2S8,6.9,8,8c0,0.63,0.29,1.18,0.75,1.55L6,18h1.5l0.49-1.5h3.6L12,18h1.5l-2.31-8.4 C11.68,9.23,12,8.66,12,8z M8.47,15l1.46-4.5l1.24,4.5H8.47z",
                    }
                }
            }
        }
    }
}

pub fn cell_tower_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M7.3,14.7l1.2-1.2c-1-1-1.5-2.3-1.5-3.5c0-1.3,0.5-2.6,1.5-3.5L7.3,5.3c-1.3,1.3-2,3-2,4.7S6,13.4,7.3,14.7z",
                    }
                    path {
                        d: "M19.1,2.9l-1.2,1.2c1.6,1.6,2.4,3.8,2.4,5.9c0,2.1-0.8,4.3-2.4,5.9l1.2,1.2c2-2,2.9-4.5,2.9-7.1C22,7.4,21,4.9,19.1,2.9z",
                    }
                    path {
                        d: "M6.1,4.1L4.9,2.9C3,4.9,2,7.4,2,10c0,2.6,1,5.1,2.9,7.1l1.2-1.2c-1.6-1.6-2.4-3.8-2.4-5.9C3.7,7.9,4.5,5.7,6.1,4.1z",
                    }
                    path {
                        d: "M16.7,14.7c1.3-1.3,2-3,2-4.7c-0.1-1.7-0.7-3.4-2-4.7l-1.2,1.2c1,1,1.5,2.3,1.5,3.5c0,1.3-0.5,2.6-1.5,3.5L16.7,14.7z",
                    }
                    path {
                        d: "M14.5,10c0-1.38-1.12-2.5-2.5-2.5S9.5,8.62,9.5,10c0,0.76,0.34,1.42,0.87,1.88L7,22h2l0.67-2h4.67L15,22h2l-3.37-10.12 C14.16,11.42,14.5,10.76,14.5,10z M10.33,18L12,13l1.67,5H10.33z",
                    }
                }
            }
        }
    }
}

pub fn cell_wifi_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6,22h16V5.97L6,22z M20,20h-2v-7.22l2-2V20z M5.22,7.22L3.93,5.93c3.9-3.91,10.24-3.91,14.15,0l-1.29,1.29 C13.6,4.03,8.41,4.03,5.22,7.22z M12.93,11.07L11,13l-1.93-1.93C10.14,10.01,11.86,10.01,12.93,11.07z M14.22,9.79 c-1.78-1.77-4.66-1.77-6.43,0L6.5,8.5c2.48-2.48,6.52-2.48,9,0L14.22,9.79z",
                }
            }
        }
    }
}

pub fn chat_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M4 4h16v12H5.17L4 17.17V4m0-2c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2H4zm2 10h8v2H6v-2zm0-3h12v2H6V9zm0-3h12v2H6V6z",
            }
        }
    }
}

pub fn chat_bubble_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn chat_bubble_outline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H6l-2 2V4h16v12z",
            }
        }
    }
}

pub fn clear_all_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5 13h14v-2H5v2zm-2 4h14v-2H3v2zM7 7v2h14V7H7z",
            }
        }
    }
}

pub fn comment_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.99 4c0-1.1-.89-2-1.99-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4-.01-18zM20 4v13.17L18.83 16H4V4h16zM6 12h12v2H6zm0-3h12v2H6zm0-3h12v2H6z",
            }
        }
    }
}

pub fn comments_disabled_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M18,15.88V3.5C18,2.67,17.33,2,16.5,2H4.12l1.5,1.5H16.5v10.88L18,15.88z M8.62,6.5H15V5H7.12L8.62,6.5z M14.12,12H15v-1.5 h-2.38L14.12,12z M11.37,9.25H15v-1.5H9.87L11.37,9.25z M2.22,2.22L1.16,3.28L2,4.12v9.38C2,14.33,2.67,15,3.5,15h9.38l3.84,3.84 l1.06-1.06L2.22,2.22z M3.5,13.5V5.62l2.13,2.13H5v1.5h2.13l1.25,1.25H8V12h1.88l1.5,1.5H3.5z",
            }
        }
    }
}

pub fn comments_disabled_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.83,16H20V4H6.83l-2-2H20c1.1,0,2,0.9,2,2l0,15.17L18.83,16z M18,6H8.83l2,2H18V6z M18,9h-6.17l2,2H18V9z M18,14v-2 h-3.17l2,2H18z M21.9,21.9l-1.41,1.41L15.17,18H4c-1.1,0-2-0.9-2-2V4.83L0.69,3.51L2.1,2.1L21.9,21.9z M13.17,16l-2-2H6v-2h3.17 l-1-1H6V9h0.17L4,6.83V16H13.17z",
            }
        }
    }
}

pub fn contacts_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V6h16v12zM4 0h16v2H4zm0 22h16v2H4zm8-10c1.38 0 2.5-1.12 2.5-2.5S13.38 7 12 7 9.5 8.12 9.5 9.5 10.62 12 12 12zm0-3.5c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm5 7.49C17 13.9 13.69 13 12 13s-5 .9-5 2.99V17h10v-1.01zm-8.19-.49c.61-.52 2.03-1 3.19-1 1.17 0 2.59.48 3.2 1H8.81z",
            }
        }
    }
}

pub fn contact_emergency_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    path {
                        d: "M18.5,3h-17C0.67,3,0,3.67,0,4.5v11C0,16.33,0.67,17,1.5,17h17c0.83,0,1.5-0.67,1.5-1.5v-11C20,3.67,19.33,3,18.5,3z M3.85,15.5c0.89-0.63,1.98-1,3.15-1s2.26,0.37,3.15,1H3.85z M18.5,15.5h-6.14C11.07,13.97,9.15,13,7,13s-4.07,0.97-5.36,2.5H1.5 v-11h17V15.5z",
                    }
                    path {
                        d: "M7,12c1.38,0,2.5-1.12,2.5-2.5S8.38,7,7,7S4.5,8.12,4.5,9.5S5.62,12,7,12z M7,8.5c0.54,0,1,0.46,1,1s-0.46,1-1,1 s-1-0.46-1-1S6.46,8.5,7,8.5z",
                    }
                    polygon {
                        points: "13.08,10.18 14.5,9.37 14.5,11 15.5,11 15.5,9.37 16.92,10.18 17.42,9.32 16,8.5 17.42,7.68 16.92,6.82 15.5,7.63 15.5,6 14.5,6 14.5,7.63 13.08,6.82 12.58,7.68 14,8.5 12.58,9.32",
                    }
                }
            }
        }
    }
}

pub fn contact_emergency_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M9,14c1.65,0,3-1.35,3-3s-1.35-3-3-3s-3,1.35-3,3S7.35,14,9,14z M9,10c0.54,0,1,0.46,1,1s-0.46,1-1,1s-1-0.46-1-1 S8.46,10,9,10z",
                    }
                    path {
                        d: "M22,3H2C0.9,3,0,3.9,0,5v14c0,1.1,0.9,2,2,2h20c1.1,0,1.99-0.9,1.99-2L24,5C24,3.9,23.1,3,22,3z M4.54,19 c1.1-1.22,2.69-2,4.46-2s3.36,0.78,4.46,2H4.54z M22,19h-6.08c-1.38-2.39-3.96-4-6.92-4s-5.54,1.61-6.92,4H2V5h20V19z",
                    }
                    polygon {
                        points: "15.78,11.15 17.25,10.3 17.25,12 18.75,12 18.75,10.3 20.22,11.15 20.97,9.85 19.5,9 20.97,8.15 20.22,6.85 18.75,7.7 18.75,6 17.25,6 17.25,7.7 15.78,6.85 15.03,8.15 16.5,9 15.03,9.85",
                    }
                }
            }
        }
    }
}

pub fn contact_mail_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22 3H2C.9 3 0 3.9 0 5v14c0 1.1.9 2 2 2h20c1.1 0 1.99-.9 1.99-2L24 5c0-1.1-.9-2-2-2zm0 16H2V5h20v14zM21 6h-7v5h7V6zm-1 2l-2.5 1.75L15 8V7l2.5 1.75L20 7v1zM9 12c1.65 0 3-1.35 3-3s-1.35-3-3-3-3 1.35-3 3 1.35 3 3 3zm0-4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm6 8.59c0-2.5-3.97-3.58-6-3.58s-6 1.08-6 3.58V18h12v-1.41zM5.48 16c.74-.5 2.22-1 3.52-1s2.77.49 3.52 1H5.48z",
            }
        }
    }
}

pub fn contact_phone_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22 3H2C.9 3 0 3.9 0 5v14c0 1.1.9 2 2 2h20c1.1 0 1.99-.9 1.99-2L24 5c0-1.1-.9-2-2-2zm0 16H2V5h20v14zm-2.99-1.01L21 16l-1.51-2h-1.64c-.22-.63-.35-1.3-.35-2s.13-1.37.35-2h1.64L21 8l-1.99-1.99c-1.31.98-2.28 2.37-2.73 3.99-.18.64-.28 1.31-.28 2s.1 1.36.28 2c.45 1.61 1.42 3.01 2.73 3.99zM9 12c1.65 0 3-1.35 3-3s-1.35-3-3-3-3 1.35-3 3 1.35 3 3 3zm0-4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm6 8.59c0-2.5-3.97-3.58-6-3.58s-6 1.08-6 3.58V18h12v-1.41zM5.48 16c.74-.5 2.22-1 3.52-1s2.77.49 3.52 1H5.48z",
            }
        }
    }
}

pub fn co_present_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                g {
                    path {
                        d: "M17.5,3h-15C1.67,3,1,3.67,1,4.5V12h1.5V4.5h15V17c0.83,0,1.5-0.67,1.5-1.5v-11C19,3.67,18.33,3,17.5,3z",
                    }
                    path {
                        d: "M10,9c0-1.66-1.34-3-3-3S4,7.34,4,9s1.34,3,3,3S10,10.66,10,9z M5.5,9c0-0.83,0.67-1.5,1.5-1.5S8.5,8.17,8.5,9 S7.83,10.5,7,10.5S5.5,9.83,5.5,9z",
                    }
                    path {
                        d: "M12.03,14.37C10.56,13.5,8.84,13,7,13s-3.56,0.5-5.03,1.37C1.36,14.72,1,15.39,1,16.09V18h12v-1.91 C13,15.39,12.64,14.72,12.03,14.37z M11.5,16.5h-9c0-0.33-0.05-0.67,0.22-0.84C4.02,14.9,5.5,14.5,7,14.5s2.98,0.4,4.28,1.16 C11.56,15.83,11.5,16.17,11.5,16.5z",
                    }
                }
            }
        }
    }
}

pub fn co_present_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M21,3H3C1.9,3,1,3.9,1,5v8h2V5h18v16c1.1,0,2-0.9,2-2V5C23,3.9,22.1,3,21,3z",
                        }
                    }
                    g {
                        path {
                            d: "M13,10c0-2.21-1.79-4-4-4s-4,1.79-4,4c0,2.21,1.79,4,4,4S13,12.21,13,10z M7,10c0-1.1,0.9-2,2-2s2,0.9,2,2 c0,1.1-0.9,2-2,2S7,11.1,7,10z",
                        }
                    }
                    g {
                        path {
                            d: "M15.39,16.56C13.71,15.7,11.53,15,9,15c-2.53,0-4.71,0.7-6.39,1.56C1.61,17.07,1,18.1,1,19.22V22h16v-2.78 C17,18.1,16.39,17.07,15.39,16.56z M15,20H3c0-0.72-0.1-1.34,0.52-1.66C4.71,17.73,6.63,17,9,17c2.37,0,4.29,0.73,5.48,1.34 C15.11,18.66,15,19.29,15,20z",
                        }
                    }
                }
            }
        }
    }
}

pub fn desktop_access_disabled_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M1.41 1.69L0 3.1l1 .99V16c0 1.1.89 2 1.99 2H10v2H8v2h8v-2h-2v-2h.9l6 6 1.41-1.41-20.9-20.9zM2.99 16V6.09L12.9 16H2.99zM4.55 2l2 2H21v12h-2.45l2 2h.44c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2H4.55z",
            }
        }
    }
}

pub fn dialer_sip_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M16 3h1v5h-1zm-1 2h-2V4h2V3h-3v3h2v1h-2v1h3zm3-2v5h1V6h2V3h-3zm2 2h-1V4h1v1zm0 10.5c-1.25 0-2.45-.2-3.57-.57-.1-.03-.21-.05-.31-.05-.26 0-.51.1-.7.29l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.27-.26.35-.65.24-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM5.03 5h1.5c.07.88.22 1.75.46 2.59L5.79 8.8c-.41-1.21-.67-2.48-.76-3.8zM19 18.97c-1.32-.09-2.59-.35-3.8-.75l1.2-1.2c.85.24 1.71.39 2.59.45v1.5z",
            }
        }
    }
}

pub fn dialpad_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 19c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zM6 1c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm12-8c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm-6 8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm-6 0c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

pub fn document_scanner_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M6,2.5H3.5V5H2V1h4V2.5z M16.5,5l0-2.5l-2.5,0L14,1l4,0l0,4L16.5,5z M14,17.5h2.5V15H18v4h-4V17.5z M3.5,15l0,2.5l2.5,0 L6,19l-4,0l0-4L3.5,15z M13.49,14.5H6.5l0.01-9h6.99L13.49,14.5z M6.43,4C5.64,4,5,4.67,5,5.5v9C5,15.33,5.64,16,6.43,16h7.14 c0.79,0,1.43-0.67,1.43-1.5v-9C15,4.67,14.36,4,13.57,4H6.43z M12,7H8v1.5h4V7z M12,9.25H8v1.5h4V9.25z M12,11.5H8V13h4V11.5z",
            }
        }
    }
}

pub fn document_scanner_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7,3H4v3H2V1h5V3z M22,6V1h-5v2h3v3H22z M7,21H4v-3H2v5h5V21z M20,18v3h-3v2h5v-5H20z M17,6H7v12h10V6z M19,18 c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h10c1.1,0,2,0.9,2,2V18z M15,8H9v2h6V8z M15,11H9v2h6V11z M15,14H9v2h6V14z",
            }
        }
    }
}

pub fn domain_disabled_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M1.41 1.69L0 3.1l2 2V21h15.9l3 3 1.41-1.41-20.9-20.9zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm-2-4V9h2v2H4zm6 8H8v-2h2v2zm-2-4v-2h2v2H8zm4 4v-2h1.9l2 2H12zM8 5h2v2h-.45L12 9.45V9h8v8.45l2 2V7H12V3H5.55L8 5.45zm8 6h2v2h-2z",
            }
        }
    }
}

pub fn domain_verification_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15,4H5C4.45,4,4,4.45,4,5v10c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V5C16,4.45,15.55,4,15,4z M15,15H5V7h10V15z",
                    }
                    polygon {
                        points: "13.18,9.23 12.47,8.53 8.94,12.06 7.53,10.65 6.82,11.35 8.94,13.47",
                    }
                }
            }
        }
    }
}

pub fn domain_verification_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "16.6,10.88 15.18,9.46 10.94,13.71 8.82,11.58 7.4,13 10.94,16.54",
                    }
                    path {
                        d: "M19,4H5C3.89,4,3,4.9,3,6v12c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V6C21,4.9,20.11,4,19,4z M19,18H5V8h14V18z",
                    }
                }
            }
        }
    }
}

pub fn duo_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 2h-8C6.38 2 2 6.66 2 12.28 2 17.5 6.49 22 11.72 22 17.39 22 22 17.62 22 12V4c0-1.1-.9-2-2-2zm-3 13l-3-2v2H7V9h7v2l3-2v6z",
            }
        }
    }
}

pub fn email_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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

pub fn forum_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15 4v7H5.17L4 12.17V4h11m1-2H3c-.55 0-1 .45-1 1v14l4-4h10c.55 0 1-.45 1-1V3c0-.55-.45-1-1-1zm5 4h-2v9H6v2c0 .55.45 1 1 1h11l4 4V7c0-.55-.45-1-1-1z",
            }
        }
    }
}

pub fn forward_to_inbox_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M15,12.5l3,3l-3,3V16h-3v-1h3V12.5z M11,15H4V6.18L10,10l6-3.82V12h1V5c0-0.55-0.45-1-1-1H4C3.45,4,3,4.45,3,5v10 c0,0.55,0.45,1,1,1h7L11,15z M16,5l-6,3.82L4,5H16z",
                }
            }
        }
    }
}

pub fn forward_to_inbox_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h9v-2H4V8l8,5l8-5v5h2V6C22,4.9,21.1,4,20,4z M12,11L4,6h16L12,11z M19,15l4,4 l-4,4v-3h-4v-2h4V15z",
                }
            }
        }
    }
}

pub fn hourglass_bottom_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6,17h8v-4l-3-3l3-3V3H6v4l3,3l-3,3V17z M7,6.59V4h6v2.59l-3,3L7,6.59z",
                }
            }
        }
    }
}

pub fn hourglass_bottom_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M18,22l-0.01-6L14,12l3.99-4.01L18,2H6v6l4,4l-4,3.99V22H18z M8,7.5V4h8v3.5l-4,4L8,7.5z",
                }
            }
        }
    }
}

pub fn hourglass_top_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,3H6v4l3,3l-3,3v4h8v-4l-3-3l3-3V3z M13,13.41V16H7v-2.59l3-3L13,13.41z",
                }
            }
        }
    }
}

pub fn hourglass_top_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6,2l0.01,6L10,12l-3.99,4.01L6,22h12v-6l-4-4l4-3.99V2H6z M16,16.5V20H8v-3.5l4-4L16,16.5z",
                }
            }
        }
    }
}

pub fn hub_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17.5,5.5C16.12,5.5,15,6.62,15,8c0,0.08,0,0.15,0.01,0.22L13.42,8.7c-0.52-1.15-1.61-2-2.92-2.17V4.95 c1.14-0.23,2-1.24,2-2.45C12.5,1.12,11.38,0,10,0S7.5,1.12,7.5,2.5c0,1.21,0.86,2.22,2,2.45v1.58C8.2,6.71,7.11,7.55,6.58,8.7 L4.99,8.22C5,8.15,5,8.08,5,8c0-1.38-1.12-2.5-2.5-2.5S0,6.62,0,8c0,1.38,1.12,2.5,2.5,2.5c0.95,0,1.78-0.53,2.2-1.32L6.3,9.66 c-0.18,1.12,0.15,2.3,0.99,3.18l-1.15,1.43C5.8,14.1,5.41,14,5,14c-1.38,0-2.5,1.12-2.5,2.5C2.5,17.88,3.62,19,5,19 s2.5-1.12,2.5-2.5c0-0.61-0.22-1.17-0.58-1.6l1.15-1.43c1.18,0.71,2.68,0.71,3.86,0l1.15,1.43c-0.36,0.43-0.58,0.99-0.58,1.6 c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5c0-1.38-1.12-2.5-2.5-2.5c-0.41,0-0.8,0.1-1.14,0.27l-1.15-1.43 c0.84-0.88,1.17-2.06,0.99-3.18l1.59-0.48c0.42,0.78,1.25,1.32,2.2,1.32c1.38,0,2.5-1.12,2.5-2.5C20,6.62,18.88,5.5,17.5,5.5z M2.5,9c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C3.5,8.55,3.05,9,2.5,9z M5,17.5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 s1,0.45,1,1C6,17.05,5.55,17.5,5,17.5z M9,2.5c0-0.55,0.45-1,1-1s1,0.45,1,1c0,0.55-0.45,1-1,1S9,3.05,9,2.5z M10,12.5 c-1.24,0-2.25-1.01-2.25-2.25C7.75,9.01,8.76,8,10,8s2.25,1.01,2.25,2.25C12.25,11.49,11.24,12.5,10,12.5z M15,15.5 c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1s-1-0.45-1-1C14,15.95,14.45,15.5,15,15.5z M17.5,9c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 s1,0.45,1,1C18.5,8.55,18.05,9,17.5,9z",
            }
        }
    }
}

pub fn hub_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21,6.5c-1.66,0-3,1.34-3,3c0,0.07,0,0.14,0.01,0.21l-2.03,0.68c-0.64-1.21-1.82-2.09-3.22-2.32V5.91 C14.04,5.57,15,4.4,15,3c0-1.66-1.34-3-3-3S9,1.34,9,3c0,1.4,0.96,2.57,2.25,2.91v2.16c-1.4,0.23-2.58,1.11-3.22,2.32L5.99,9.71 C6,9.64,6,9.57,6,9.5c0-1.66-1.34-3-3-3s-3,1.34-3,3s1.34,3,3,3c1.06,0,1.98-0.55,2.52-1.37l2.03,0.68 c-0.2,1.29,0.17,2.66,1.09,3.69l-1.41,1.77C6.85,17.09,6.44,17,6,17c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3 c0-0.68-0.22-1.3-0.6-1.8l1.41-1.77c1.36,0.76,3.02,0.75,4.37,0l1.41,1.77C15.22,18.7,15,19.32,15,20c0,1.66,1.34,3,3,3s3-1.34,3-3 s-1.34-3-3-3c-0.44,0-0.85,0.09-1.23,0.26l-1.41-1.77c0.93-1.04,1.29-2.4,1.09-3.69l2.03-0.68c0.53,0.82,1.46,1.37,2.52,1.37 c1.66,0,3-1.34,3-3S22.66,6.5,21,6.5z M3,10.5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C4,10.05,3.55,10.5,3,10.5z M6,21 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C7,20.55,6.55,21,6,21z M11,3c0-0.55,0.45-1,1-1s1,0.45,1,1c0,0.55-0.45,1-1,1 S11,3.55,11,3z M12,15c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5C14.5,13.88,13.38,15,12,15z M18,19 c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1s-1-0.45-1-1C17,19.45,17.45,19,18,19z M21,10.5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 s1,0.45,1,1C22,10.05,21.55,10.5,21,10.5z",
            }
        }
    }
}

pub fn import_contacts_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 5c-1.11-.35-2.33-.5-3.5-.5-1.95 0-4.05.4-5.5 1.5-1.45-1.1-3.55-1.5-5.5-1.5S2.45 4.9 1 6v14.65c0 .25.25.5.5.5.1 0 .15-.05.25-.05C3.1 20.45 5.05 20 6.5 20c1.95 0 4.05.4 5.5 1.5 1.35-.85 3.8-1.5 5.5-1.5 1.65 0 3.35.3 4.75 1.05.1.05.15.05.25.05.25 0 .5-.25.5-.5V6c-.6-.45-1.25-.75-2-1zm0 13.5c-1.1-.35-2.3-.5-3.5-.5-1.7 0-4.15.65-5.5 1.5V8c1.35-.85 3.8-1.5 5.5-1.5 1.2 0 2.4.15 3.5.5v11.5z",
            }
        }
    }
}

pub fn import_export_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9 3L5 6.99h3V14h2V6.99h3L9 3zm7 14.01V10h-2v7.01h-3L15 21l4-3.99h-3z",
            }
        }
    }
}

pub fn invert_colors_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M7.08,4.96L10,2l4.53,4.6l0,0c1.07,1.1,1.72,2.6,1.72,4.24c0,0.96-0.23,1.86-0.62,2.67L10,7.88V4.14L8.14,6.02L7.08,4.96z M16.01,18.13l-2.33-2.33C12.65,16.55,11.38,17,10,17c-3.45,0-6.25-2.76-6.25-6.16c0-1.39,0.47-2.67,1.26-3.7L1.87,3.99l1.06-1.06 l14.14,14.14L16.01,18.13z M10,12.12L6.09,8.21c-0.54,0.77-0.84,1.68-0.84,2.63c0,2.57,2.13,4.66,4.75,4.66V12.12z",
            }
        }
    }
}

pub fn invert_colors_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.19,21.19L2.81,2.81L1.39,4.22l4.2,4.2c-1,1.31-1.6,2.94-1.6,4.7C4,17.48,7.58,21,12,21c1.75,0,3.36-0.56,4.67-1.5 l3.1,3.1L21.19,21.19z M12,19c-3.31,0-6-2.63-6-5.87c0-1.19,0.36-2.32,1.02-3.28L12,14.83V19z M8.38,5.56L12,2l5.65,5.56l0,0 C19.1,8.99,20,10.96,20,13.13c0,1.18-0.27,2.29-0.74,3.3L12,9.17V4.81L9.8,6.97L8.38,5.56z",
            }
        }
    }
}

pub fn key_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17.5,8.5h-6.75C10.11,6.48,8.24,5,6,5c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.24,0,4.11-1.48,4.75-3.5h0.75L13,13l1.5-1.5L16,13 l3-3L17.5,8.5z M6,12.5c-1.38,0-2.5-1.12-2.5-2.5S4.62,7.5,6,7.5S8.5,8.62,8.5,10S7.38,12.5,6,12.5z",
                }
            }
        }
    }
}

pub fn key_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M21,10h-8.35C11.83,7.67,9.61,6,7,6c-3.31,0-6,2.69-6,6s2.69,6,6,6c2.61,0,4.83-1.67,5.65-4H13l2,2l2-2l2,2l4-4.04L21,10z M7,15c-1.65,0-3-1.35-3-3c0-1.65,1.35-3,3-3s3,1.35,3,3C10,13.65,8.65,15,7,15z",
                }
            }
        }
    }
}

pub fn key_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.88,9l-5.76,0l-1.5-1.5l7.88,0l0,0L20,10l-3.75,3.5l-1.75-1.25l-0.07,0.06l-1.07-1.07l1.06-0.89l1.69,1.21l1.72-1.6 L16.88,9 M8,10.12C7.94,11.17,7.06,12,6,12c-1.1,0-2-0.9-2-2c0-1.06,0.83-1.94,1.88-2L4.65,6.77C3.39,7.3,2.5,8.55,2.5,10 c0,1.93,1.57,3.5,3.5,3.5c1.45,0,2.7-0.89,3.23-2.15L8,10.12z M10.35,12.47C9.49,13.98,7.86,15,6,15c-2.76,0-5-2.24-5-5 c0-1.86,1.02-3.49,2.53-4.35L1.87,3.99l1.06-1.06l14.14,14.14l-1.06,1.06L10.35,12.47z",
                }
            }
        }
    }
}

pub fn key_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10.7,13.53l-1.71-1.71C9,11.88,9,11.94,9,12c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2c0.06,0,0.12,0,0.18,0.01L5.47,8.3 C4.02,8.9,3,10.33,3,12c0,2.21,1.79,4,4,4C8.67,16,10.1,14.98,10.7,13.53z M12.19,15.02C11.15,16.8,9.21,18,7,18 c-3.31,0-6-2.69-6-6c0-2.21,1.2-4.15,2.98-5.19L1.39,4.22l1.41-1.41l18.38,18.38l-1.41,1.41L12.19,15.02z M16.26,13.43l1.24-0.93 l1.81,1.36L21.17,12l-1-1l-6.34,0l-2-2L21,9l0,0l3,3l-4.5,4.5l-0.69-0.51L16.26,13.43z",
                    }
                }
            }
        }
    }
}

pub fn list_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11 7h6v2h-6zm0 4h6v2h-6zm0 4h6v2h-6zM7 7h2v2H7zm0 4h2v2H7zm0 4h2v2H7zM20.1 3H3.9c-.5 0-.9.4-.9.9v16.2c0 .4.4.9.9.9h16.2c.4 0 .9-.5.9-.9V3.9c0-.5-.5-.9-.9-.9zM19 19H5V5h14v14z",
            }
        }
    }
}

pub fn live_help_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 2H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h4l3 3 3-3h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 16h-4.83l-.59.59L12 20.17l-1.59-1.59-.58-.58H5V4h14v14zm-8-3h2v2h-2zm1-8c1.1 0 2 .9 2 2 0 2-3 1.75-3 5h2c0-2.25 3-2.5 3-5 0-2.21-1.79-4-4-4S8 6.79 8 9h2c0-1.1.9-2 2-2z",
            }
        }
    }
}

pub fn location_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 4c2.76 0 5 2.24 5 5 0 1.06-.39 2.32-1 3.62l1.49 1.49C18.37 12.36 19 10.57 19 9c0-3.87-3.13-7-7-7-1.84 0-3.5.71-4.75 1.86l1.43 1.43C9.56 4.5 10.72 4 12 4zm0 2.5c-.59 0-1.13.21-1.56.56l3.5 3.5c.35-.43.56-.97.56-1.56 0-1.38-1.12-2.5-2.5-2.5zM3.41 2.86L2 4.27l3.18 3.18C5.07 7.95 5 8.47 5 9c0 5.25 7 13 7 13s1.67-1.85 3.38-4.35L18.73 21l1.41-1.41L3.41 2.86zM12 18.88c-2.01-2.58-4.8-6.74-4.98-9.59l6.92 6.92c-.65.98-1.33 1.89-1.94 2.67z",
            }
        }
    }
}

pub fn location_on_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zM7 9c0-2.76 2.24-5 5-5s5 2.24 5 5c0 2.88-2.88 7.19-5 9.88C9.92 16.21 7 11.85 7 9z",
            }
            circle {
                cx: "12",
                r: "2.5",
                cy: "9",
            }
        }
    }
}

pub fn mail_lock_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M3.5,7.27L10,11l6.5-3.73V8.5H18v-3C18,4.67,17.33,4,16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h10v-1.5 h-10V7.27z M16.5,5.5L10,9.23L3.5,5.5H16.5z",
                    }
                    path {
                        d: "M19.5,12H19v-1c0-0.83-0.67-1.5-1.5-1.5S16,10.17,16,11v1h-0.5c-0.28,0-0.5,0.22-0.5,0.5v3c0,0.28,0.22,0.5,0.5,0.5h4 c0.28,0,0.5-0.22,0.5-0.5v-3C20,12.22,19.78,12,19.5,12z M16.75,12v-1c0-0.41,0.34-0.75,0.75-0.75c0.41,0,0.75,0.34,0.75,0.75v1 H16.75z",
                    }
                }
            }
        }
    }
}

pub fn mail_lock_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M4,8l8,5l8-5v2h2V6c0-1.1-0.9-2-2-2H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h12v-2H4V8z M20,6l-8,5L4,6H20z",
                    }
                    path {
                        d: "M23,15v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3 C24,15.45,23.55,15,23,15z M20,15v-1c0-0.55,0.45-1,1-1s1,0.45,1,1v1H20z",
                    }
                }
            }
        }
    }
}

pub fn mail_outline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V8l8 5 8-5v10zm-8-7L4 6h16l-8 5z",
            }
        }
    }
}

pub fn mark_chat_read_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M10,14H6l-3,3V4c0-0.55,0.45-1,1-1h12c0.55,0,1,0.45,1,1v4l-1-0.02V4H4v9l6,0V14z M18,10.76l-0.71-0.71l-3.54,3.54 l-1.41-1.41l-0.71,0.71L13.76,15L18,10.76z",
                }
            }
        }
    }
}

pub fn mark_chat_read_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,18l-6,0l-4,4V4c0-1.1,0.9-2,2-2h16c1.1,0,2,0.9,2,2v7l-2,0V4H4v12l8,0V18z M23,14.34l-1.41-1.41l-4.24,4.24l-2.12-2.12 l-1.41,1.41L17.34,20L23,14.34z",
                }
            }
        }
    }
}

pub fn mark_chat_unread_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    y: "0",
                    height: "20",
                }
                path {
                    d: "M17,6.22V13c0,0.55-0.45,1-1,1H6l-3,3V4c0-0.55,0.45-1,1-1h8.18C12.07,3.31,12,3.65,12,4H4v9h12V6.82 C16.37,6.68,16.71,6.48,17,6.22z M13,4c0,1.1,0.9,2,2,2s2-0.9,2-2s-0.9-2-2-2S13,2.9,13,4z",
                }
            }
        }
    }
}

pub fn mark_chat_unread_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                }
                path {
                    d: "M22,6.98V16c0,1.1-0.9,2-2,2H6l-4,4V4c0-1.1,0.9-2,2-2h10.1C14.04,2.32,14,2.66,14,3s0.04,0.68,0.1,1H4v12h16V7.9 C20.74,7.75,21.42,7.42,22,6.98z M16,3c0,1.66,1.34,3,3,3s3-1.34,3-3s-1.34-3-3-3S16,1.34,16,3z",
                }
            }
        }
    }
}

pub fn mark_email_read_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M10,15H4V6.18L10,10l6-3.82V10h1V5c0-0.55-0.45-1-1-1H4C3.45,4,3,4.45,3,5v10c0,0.55,0.45,1,1,1h6L10,15z M16,5l-6,3.82 L4,5H16z M13.76,17l-2.12-2.12l0.71-0.71l1.41,1.41l3.54-3.54L18,12.76L13.76,17z",
                }
            }
        }
    }
}

pub fn mark_email_read_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M20,4H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h8l0-2H4V8l8,5l8-5v5h2V6C22,4.9,21.1,4,20,4z M12,11L4,6h16L12,11z M17.34,22l-3.54-3.54l1.41-1.41l2.12,2.12l4.24-4.24L23,16.34L17.34,22z",
                }
            }
        }
    }
}

pub fn mark_email_unread_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,7.22V15c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1V5c0-0.55,0.45-1,1-1h8.18C12.07,4.31,12,4.65,12,5H4l6,3.82l2.81-1.79 c0.24,0.26,0.53,0.47,0.85,0.64L10,10L4,6.18V15h12V7.82C16.37,7.68,16.71,7.48,17,7.22z M13,5c0,1.1,0.9,2,2,2s2-0.9,2-2 s-0.9-2-2-2S13,3.9,13,5z",
                }
            }
        }
    }
}

pub fn mark_email_unread_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                }
                path {
                    d: "M22,8.98V18c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2L2.01,6C2.01,4.9,2.9,4,4,4h10.1C14.04,4.32,14,4.66,14,5s0.04,0.68,0.1,1 H4l8,5l3.67-2.29c0.47,0.43,1.02,0.76,1.63,0.98L12,13L4,8v10h16V9.9C20.74,9.75,21.42,9.42,22,8.98z M16,5c0,1.66,1.34,3,3,3 s3-1.34,3-3s-1.34-3-3-3S16,3.34,16,5z",
                }
            }
        }
    }
}

pub fn mark_unread_chat_alt_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.5,13.5h-13v-10h8.07c-0.06-0.29-0.13-0.84,0-1.5L3.5,2C2.67,2,2,2.67,2,3.5V18l3-3h11.5c0.83,0,1.5-0.67,1.5-1.5V5.87 c-0.43,0.35-0.94,0.61-1.5,0.75L16.5,13.5z",
                    }
                    rect {
                        height: "1.5",
                        x: "5",
                        y: "10.5",
                        width: "7",
                    }
                    rect {
                        y: "7.75",
                        width: "10",
                        x: "5",
                        height: "1.5",
                    }
                    circle {
                        cy: "2.75",
                        cx: "15.5",
                        r: "2.5",
                    }
                    path {
                        d: "M5,6.5h9.15c-0.8-0.29-1.49-0.81-1.96-1.5H5V6.5z",
                    }
                }
            }
        }
    }
}

pub fn mark_unread_chat_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,16H4V4h10.1c-0.08-0.39-0.18-1.11,0-2H4C2.9,2,2,2.9,2,4v18l4-4h14c1.1,0,2-0.9,2-2V6.98c-0.58,0.44-1.26,0.77-2,0.92 V16z",
                    }
                    circle {
                        cy: "3",
                        r: "3",
                        cx: "19",
                    }
                    rect {
                        height: "2",
                        width: "8",
                        y: "12",
                        x: "6",
                    }
                    rect {
                        x: "6",
                        width: "12",
                        y: "9",
                        height: "2",
                    }
                    path {
                        d: "M6,8h12V7.9c-1.21-0.25-2.25-0.95-2.97-1.9H6V8z",
                    }
                }
            }
        }
    }
}

pub fn message_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M4 4h16v12H5.17L4 17.17V4m0-2c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2H4zm2 10h12v2H6v-2zm0-3h12v2H6V9zm0-3h12v2H6V6z",
            }
        }
    }
}

pub fn mobile_screen_share_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 1H7c-1.1 0-1.99.85-1.99 1.95v18C5.01 22.05 5.9 23 7 23h10c1.1 0 2-.95 2-2.05v-18C19 1.85 18.1 1 17 1zm0 18H7V5h10v14zm-4.2-5.76v1.75L16 12l-3.2-2.98v1.7c-3.11.43-4.35 2.56-4.8 4.7 1.11-1.5 2.58-2.18 4.8-2.18z",
            }
        }
    }
}

pub fn more_time_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                g {
                    polygon {
                        points: "8.5,8.05 8.5,12.05 11.67,13.95 12.17,13.13 9.5,11.55 9.5,8.05",
                    }
                    path {
                        d: "M13.9,10c0.07,0.32,0.1,0.66,0.1,1c0,2.76-2.24,5-5,5s-5-2.24-5-5s2.24-5,5-5c0.71,0,1.39,0.15,2,0.42V5.35 C10.37,5.13,9.7,5,9,5c-3.31,0-6,2.69-6,6s2.69,6,6,6s6-2.69,6-6c0-0.34-0.04-0.67-0.09-1H13.9z",
                    }
                    polygon {
                        points: "15,6 15,4 14,4 14,6 14,6 12,6 12,7 14,7 14,9 15,9 15,7 15,7 17,7 17,6",
                    }
                }
            }
        }
    }
}

pub fn more_time_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "10,8 10,14 14.7,16.9 15.5,15.7 11.5,13.3 11.5,8",
                    }
                    path {
                        d: "M17.92,12c0.05,0.33,0.08,0.66,0.08,1c0,3.9-3.1,7-7,7s-7-3.1-7-7c0-3.9,3.1-7,7-7c0.7,0,1.37,0.1,2,0.29V4.23 C12.36,4.08,11.69,4,11,4c-5,0-9,4-9,9s4,9,9,9s9-4,9-9c0-0.34-0.02-0.67-0.06-1H17.92z",
                    }
                    polygon {
                        points: "20,5 20,2 18,2 18,5 15,5 15,7 18,7 18,10 20,10 20,7 23,7 23,5",
                    }
                }
            }
        }
    }
}

pub fn nat_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M5.93,10.5H10v-1H5.93C5.71,8.64,4.93,8,4,8c-1.1,0-2,0.9-2,2s0.9,2,2,2C4.93,12,5.71,11.36,5.93,10.5z M4,11 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S4.55,11,4,11z",
                    }
                    path {
                        d: "M18,10l-3-2v1.5h-3.03C11.72,5.87,8.7,3,5,3v1c3.31,0,6,2.69,6,6s-2.69,6-6,6v1c3.7,0,6.72-2.87,6.97-6.5H15V12L18,10z",
                    }
                }
            }
        }
    }
}

pub fn nat_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M6.82,13H11v-2H6.82C6.4,9.84,5.3,9,4,9c-1.66,0-3,1.34-3,3s1.34,3,3,3C5.3,15,6.4,14.16,6.82,13z M4,13 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C5,12.55,4.55,13,4,13z",
                    }
                    path {
                        d: "M23,12l-4-3v2h-4.05C14.45,5.95,10.19,2,5,2v2c4.42,0,8,3.58,8,8s-3.58,8-8,8v2c5.19,0,9.45-3.95,9.95-9H19v2L23,12z",
                    }
                }
            }
        }
    }
}

pub fn no_sim_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.26 21.21L3.79 3.74 2.38 5.15l2.74 2.74-.12.12V19c0 1.1.9 2 2 2h10c.35 0 .68-.1.97-.26l1.88 1.88 1.41-1.41zM7 19V9.77L16.23 19H7zm3.84-14H17v9.11l2 2V5c0-1.1-.9-2-2-2h-6.99L7.95 5.06l1.41 1.41L10.84 5z",
            }
        }
    }
}

pub fn pause_presentation_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 3H3c-1.1 0-2 .85-2 1.95v14c0 1.1.9 2.05 2 2.05h18c1.1 0 2-.95 2-2.05v-14C23 3.85 22.1 3 21 3zm0 16H3V5h18v14zM9 8h2v8H9zm4 0h2v8h-2z",
            }
        }
    }
}

pub fn person_add_disabled_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15 6c1.1 0 2 .9 2 2 0 .99-.73 1.82-1.67 1.97l-2.31-2.31C13.19 6.72 14.01 6 15 6m0-2c-2.21 0-4 1.79-4 4 0 .18.03.35.05.52l3.43 3.43c.17.02.34.05.52.05 2.21 0 4-1.79 4-4s-1.79-4-4-4zm1.69 10.16L22.53 20H23v-2c0-2.14-3.56-3.5-6.31-3.84zm-3.68 1.97L14.88 18H9c.08-.24.88-1.01 2.91-1.57l1.1-.3M1.41 1.71L0 3.12l4 4V10H1v2h3v3h2v-3h2.88l2.51 2.51C9.19 15.11 7 16.3 7 18v2h9.88l4 4 1.41-1.41L1.41 1.71zM6 10v-.88l.88.88H6z",
            }
        }
    }
}

pub fn person_search_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M9.43,11.86c0.36-0.68,0.67-0.89,0.72-0.93C7.85,10.59,4,11.52,4,13.21V15h5.35c-0.15-0.32-0.25-0.66-0.3-1H5v-0.79 C5,12.71,7.33,11.72,9.43,11.86z",
                    }
                    path {
                        d: "M9,10c1.38,0,2.5-1.12,2.5-2.5C11.5,6.12,10.38,5,9,5C7.62,5,6.5,6.12,6.5,7.5C6.5,8.88,7.62,10,9,10z M9,6 c0.83,0,1.5,0.67,1.5,1.5S9.83,9,9,9S7.5,8.33,7.5,7.5S8.17,6,9,6z",
                    }
                    path {
                        d: "M16.06,16.35l-1.48-1.48c0.26-0.4,0.42-0.87,0.42-1.38c0-1.38-1.12-2.5-2.5-2.5S10,12.12,10,13.5c0,1.38,1.12,2.5,2.5,2.5 c0.51,0,0.98-0.15,1.38-0.42l1.48,1.48L16.06,16.35z M12.5,15c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 S13.33,15,12.5,15z",
                    }
                }
            }
        }
    }
}

pub fn person_search_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,12c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4S6,5.79,6,8C6,10.21,7.79,12,10,12z M10,6c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2 S8,9.1,8,8C8,6.9,8.9,6,10,6z",
                    }
                    path {
                        d: "M4,18c0.22-0.72,3.31-2,6-2c0-0.7,0.13-1.37,0.35-1.99C7.62,13.91,2,15.27,2,18v2h9.54c-0.52-0.58-0.93-1.25-1.19-2H4z",
                    }
                    path {
                        d: "M19.43,18.02C19.79,17.43,20,16.74,20,16c0-2.21-1.79-4-4-4s-4,1.79-4,4c0,2.21,1.79,4,4,4c0.74,0,1.43-0.22,2.02-0.57 c0.93,0.93,1.62,1.62,2.57,2.57L22,20.59C20.5,19.09,21.21,19.79,19.43,18.02z M16,18c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2 C18,17.1,17.1,18,16,18z",
                    }
                }
            }
        }
    }
}

pub fn phone_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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

pub fn phonelink_erase_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13 8.2l-1-1-4 4-4-4-1 1 4 4-4 4 1 1 4-4 4 4 1-1-4-4 4-4zM19 1H9c-1.1 0-2 .9-2 2v3h2V4h10v16H9v-2H7v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn phonelink_lock_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 1H9c-1.1 0-2 .9-2 2v3h2V4h10v16H9v-2H7v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm-8.2 10V9.5C10.8 8.1 9.4 7 8 7S5.2 8.1 5.2 9.5V11c-.6 0-1.2.6-1.2 1.2v3.5c0 .7.6 1.3 1.2 1.3h5.5c.7 0 1.3-.6 1.3-1.2v-3.5c0-.7-.6-1.3-1.2-1.3zm-1.3 0h-3V9.5c0-.8.7-1.3 1.5-1.3s1.5.5 1.5 1.3V11z",
            }
        }
    }
}

pub fn phonelink_ring_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20.1 7.7l-1 1c1.8 1.8 1.8 4.6 0 6.5l1 1c2.5-2.3 2.5-6.1 0-8.5zM18 9.8l-1 1c.5.7.5 1.6 0 2.3l1 1c1.2-1.2 1.2-3 0-4.3zM14 1H4c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 19H4V4h10v16z",
            }
        }
    }
}

pub fn phonelink_setup_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7 3v3h2V4h10v16H9v-2H7v3c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2H9c-1.1 0-2 .9-2 2zm2.5 12.5c.29-.12.55-.29.8-.48l-.02.03 1.01.39c.23.09.49 0 .61-.22l.84-1.46c.12-.21.07-.49-.12-.64l-.85-.68-.02.03c.02-.16.05-.32.05-.48s-.03-.32-.05-.48l.02.03.85-.68c.19-.15.24-.43.12-.64l-.84-1.46c-.12-.21-.38-.31-.61-.22l-1.01.39.02.03c-.25-.17-.51-.34-.8-.46l-.17-1.08C9.3 7.18 9.09 7 8.84 7H7.16c-.25 0-.46.18-.49.42L6.5 8.5c-.29.12-.55.29-.8.48l.02-.03-1.02-.39c-.23-.09-.49 0-.61.22l-.84 1.46c-.12.21-.07.49.12.64l.85.68.02-.03c-.02.15-.05.31-.05.47s.03.32.05.48l-.02-.03-.85.68c-.19.15-.24.43-.12.64l.84 1.46c.12.21.38.31.61.22l1.01-.39-.01-.04c.25.19.51.36.8.48l.17 1.07c.03.25.24.43.49.43h1.68c.25 0 .46-.18.49-.42l.17-1.08zM6 12c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2-2-.9-2-2z",
            }
        }
    }
}

pub fn phone_disabled_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
            g {
                g {
                    path {
                        d: "M17.34,14.54l-1.43-1.43c0.56-0.73,1.05-1.5,1.47-2.32l-2.2-2.2c-0.28-0.28-0.36-0.67-0.25-1.02 C15.3,6.45,15.5,5.25,15.5,4c0-0.55,0.45-1,1-1H20c0.55,0,1,0.45,1,1C21,7.98,19.63,11.64,17.34,14.54z M14.52,17.35 C11.63,19.64,7.97,21,4,21c-0.55,0-1-0.45-1-1v-3.49c0-0.55,0.45-1,1-1c1.24,0,2.45-0.2,3.57-0.57c0.1-0.04,0.21-0.05,0.31-0.05 c0.26,0,0.51,0.1,0.71,0.29l2.2,2.2c0.81-0.42,1.58-0.9,2.3-1.46L1.39,4.22l1.42-1.41L21.19,21.2l-1.41,1.41L14.52,17.35z M7.6,17.02c-0.85,0.24-1.72,0.39-2.6,0.45v1.49c1.32-0.09,2.59-0.35,3.8-0.75L7.6,17.02z M17.46,5 c-0.06,0.89-0.21,1.76-0.45,2.59l1.2,1.2c0.41-1.2,0.67-2.47,0.76-3.79H17.46z",
                    }
                }
            }
        }
    }
}

pub fn phone_enabled_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                g {
                    path {
                        d: "M17.46,5c-0.06,0.89-0.21,1.76-0.45,2.59l1.2,1.2c0.41-1.2,0.67-2.47,0.76-3.79H17.46z M7.6,17.02 c-0.85,0.24-1.72,0.39-2.6,0.45v1.49c1.32-0.09,2.59-0.35,3.8-0.75L7.6,17.02z M16.5,3H20c0.55,0,1,0.45,1,1c0,9.39-7.61,17-17,17 c-0.55,0-1-0.45-1-1v-3.49c0-0.55,0.45-1,1-1c1.24,0,2.45-0.2,3.57-0.57c0.1-0.04,0.21-0.05,0.31-0.05c0.26,0,0.51,0.1,0.71,0.29 l2.2,2.2c2.83-1.45,5.15-3.76,6.59-6.59l-2.2-2.2c-0.28-0.28-0.36-0.67-0.25-1.02C15.3,6.45,15.5,5.25,15.5,4 C15.5,3.45,15.95,3,16.5,3z",
                    }
                }
            }
        }
    }
}

pub fn portable_wifi_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3.42 2.36L2.01 3.78 4.1 5.87C2.79 7.57 2 9.69 2 12c0 3.7 2.01 6.92 4.99 8.65l1-1.73C5.61 17.53 4 14.96 4 12c0-1.76.57-3.38 1.53-4.69l1.43 1.44C6.36 9.68 6 10.8 6 12c0 2.22 1.21 4.15 3 5.19l1-1.74c-1.19-.7-2-1.97-2-3.45 0-.65.17-1.25.44-1.79l1.58 1.58L10 12c0 1.1.9 2 2 2l.21-.02 7.52 7.52 1.41-1.41L3.42 2.36zm14.29 11.46c.18-.57.29-1.19.29-1.82 0-3.31-2.69-6-6-6-.63 0-1.25.11-1.82.29l1.72 1.72c.03 0 .06-.01.1-.01 2.21 0 4 1.79 4 4 0 .04-.01.07-.01.11l1.72 1.71zM12 4c4.42 0 8 3.58 8 8 0 1.2-.29 2.32-.77 3.35l1.49 1.49C21.53 15.4 22 13.76 22 12c0-5.52-4.48-10-10-10-1.76 0-3.4.48-4.84 1.28l1.48 1.48C9.66 4.28 10.8 4 12 4z",
            }
        }
    }
}

pub fn present_to_all_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 3H3c-1.11 0-2 .89-2 2v14c0 1.11.89 2 2 2h18c1.11 0 2-.89 2-2V5c0-1.11-.89-2-2-2zm0 16.02H3V4.98h18v14.04zM10 12H8l4-4 4 4h-2v4h-4v-4z",
            }
        }
    }
}

pub fn print_disabled_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M1.41 1.6L0 3.01 5 8c-1.66 0-3 1.34-3 3v6h4v4h12l2.95 2.96 1.41-1.41L1.41 1.6zM6 15H4v-4c0-.55.45-1 1-1h2l3 3H6v2zm2 4v-4h4l4 4H8zM8 5h8v3h-5.34l2 2H19c.55 0 1 .45 1 1v4l-2 .01V13h-2.34l4 4H22v-6c0-1.66-1.34-3-3-3h-1V3H6v.36l2 2V5z",
            }
            circle {
                cy: "11.51",
                cx: "18",
                r: "1",
            }
        }
    }
}

pub fn qr_code_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M4,9h5V4H4V9z M4.94,4.94h3.12v3.12H4.94V4.94z",
                    }
                    path {
                        d: "M4,16h5v-5H4V16z M4.94,11.94h3.12v3.12H4.94V11.94z",
                    }
                    path {
                        d: "M11,4v5h5V4H11z M15.06,8.06h-3.12V4.94h3.12V8.06z",
                    }
                    rect {
                        y: "15",
                        x: "15",
                        height: "1",
                        width: "1",
                    }
                    rect {
                        height: "1",
                        y: "13",
                        x: "15",
                        width: "1",
                    }
                    rect {
                        y: "11",
                        height: "1",
                        width: "1",
                        x: "15",
                    }
                    rect {
                        height: "1",
                        x: "12",
                        y: "12",
                        width: "1",
                    }
                    rect {
                        y: "11",
                        x: "11",
                        width: "1",
                        height: "1",
                    }
                    rect {
                        height: "1",
                        width: "1",
                        x: "13",
                        y: "13",
                    }
                    rect {
                        x: "14",
                        y: "14",
                        height: "1",
                        width: "1",
                    }
                    rect {
                        y: "11",
                        width: "1",
                        x: "13",
                        height: "1",
                    }
                    rect {
                        width: "1",
                        height: "1",
                        y: "12",
                        x: "14",
                    }
                    rect {
                        width: "1",
                        height: "1",
                        y: "13",
                        x: "11",
                    }
                    rect {
                        x: "12",
                        height: "1",
                        width: "1",
                        y: "14",
                    }
                    rect {
                        height: "1",
                        width: "1",
                        y: "15",
                        x: "11",
                    }
                    rect {
                        y: "15",
                        height: "1",
                        width: "1",
                        x: "13",
                    }
                }
            }
        }
    }
}

pub fn qr_code_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M3,11h8V3H3V11z M5,5h4v4H5V5z",
                    }
                    path {
                        d: "M3,21h8v-8H3V21z M5,15h4v4H5V15z",
                    }
                    path {
                        d: "M13,3v8h8V3H13z M19,9h-4V5h4V9z",
                    }
                    rect {
                        width: "2",
                        y: "19",
                        x: "19",
                        height: "2",
                    }
                    rect {
                        y: "13",
                        height: "2",
                        width: "2",
                        x: "13",
                    }
                    rect {
                        width: "2",
                        height: "2",
                        y: "15",
                        x: "15",
                    }
                    rect {
                        height: "2",
                        x: "13",
                        width: "2",
                        y: "17",
                    }
                    rect {
                        x: "15",
                        y: "19",
                        width: "2",
                        height: "2",
                    }
                    rect {
                        x: "17",
                        y: "17",
                        height: "2",
                        width: "2",
                    }
                    rect {
                        x: "17",
                        height: "2",
                        width: "2",
                        y: "13",
                    }
                    rect {
                        y: "15",
                        width: "2",
                        height: "2",
                        x: "19",
                    }
                }
            }
        }
    }
}

pub fn qr_code_2_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
            path {
                d: "M15,21h-2v-2h2V21z M13,14h-2v5h2V14z M21,12h-2v4h2V12z M19,10h-2v2h2V10z M7,12H5v2h2V12z M5,10H3v2h2V10z M12,5h2V3h-2V5 z M4.5,4.5v3h3v-3H4.5z M9,9H3V3h6V9z M4.5,16.5v3h3v-3H4.5z M9,21H3v-6h6V21z M16.5,4.5v3h3v-3H16.5z M21,9h-6V3h6V9z M19,19v-3 l-4,0v2h2v3h4v-2H19z M17,12l-4,0v2h4V12z M13,10H7v2h2v2h2v-2h2V10z M14,9V7h-2V5h-2v4L14,9z M6.75,5.25h-1.5v1.5h1.5V5.25z M6.75,17.25h-1.5v1.5h1.5V17.25z M18.75,5.25h-1.5v1.5h1.5V5.25z",
            }
        }
    }
}

pub fn qr_code_scanner_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9.5,6.5v3h-3v-3H9.5 M11,5H5v6h6V5L11,5z M9.5,14.5v3h-3v-3H9.5 M11,13H5v6h6V13L11,13z M17.5,6.5v3h-3v-3H17.5 M19,5h-6v6 h6V5L19,5z M13,13h1.5v1.5H13V13z M14.5,14.5H16V16h-1.5V14.5z M16,13h1.5v1.5H16V13z M13,16h1.5v1.5H13V16z M14.5,17.5H16V19h-1.5 V17.5z M16,16h1.5v1.5H16V16z M17.5,14.5H19V16h-1.5V14.5z M17.5,17.5H19V19h-1.5V17.5z M22,7h-2V4h-3V2h5V7z M22,22v-5h-2v3h-3v2 H22z M2,22h5v-2H4v-3H2V22z M2,2v5h2V4h3V2H2z",
            }
        }
    }
}

pub fn read_more_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    polygon {
                        points: "11,10 8,7 8,9.5 3,9.5 3,10.5 8,10.5 8,13",
                    }
                    rect {
                        height: "1",
                        width: "6",
                        y: "7",
                        x: "11",
                    }
                    rect {
                        height: "1",
                        y: "12",
                        width: "6",
                        x: "11",
                    }
                    rect {
                        x: "13",
                        y: "9.5",
                        height: "1",
                        width: "4",
                    }
                }
            }
        }
    }
}

pub fn read_more_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        x: "13",
                        width: "9",
                        height: "2",
                        y: "7",
                    }
                    rect {
                        height: "2",
                        y: "15",
                        width: "9",
                        x: "13",
                    }
                    rect {
                        width: "6",
                        height: "2",
                        x: "16",
                        y: "11",
                    }
                    polygon {
                        points: "13,12 8,7 8,11 2,11 2,13 8,13 8,17",
                    }
                }
            }
        }
    }
}

pub fn ring_volume_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M23.71 16.67C20.66 13.78 16.54 12 12 12 7.46 12 3.34 13.78.29 16.67c-.18.18-.29.43-.29.71 0 .28.11.53.29.71l2.48 2.48c.18.18.43.29.71.29.27 0 .52-.11.7-.28.79-.74 1.69-1.36 2.66-1.85.33-.16.56-.5.56-.9v-3.1c1.45-.48 3-.73 4.6-.73 1.6 0 3.15.25 4.6.72v3.1c0 .39.23.74.56.9.98.49 1.87 1.12 2.66 1.85.18.18.43.28.7.28.28 0 .53-.11.71-.29l2.48-2.48c.18-.18.29-.43.29-.71 0-.27-.11-.52-.29-.7zm-18.31.56c-.66.37-1.29.8-1.87 1.27l-1.07-1.07c.91-.75 1.9-1.39 2.95-1.9v1.7zm15.07 1.26c-.59-.48-1.21-.9-1.87-1.27v-1.7c1.04.51 2.03 1.15 2.94 1.9l-1.07 1.07zm.69-12.23l-1.41-1.41-3.56 3.55 1.41 1.41s3.45-3.52 3.56-3.55zM11 2h2v5h-2zM6.4 9.81L7.81 8.4 4.26 4.84 2.84 6.26c.11.03 3.56 3.55 3.56 3.55z",
            }
        }
    }
}

pub fn rss_feed_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                r: "2.18",
                cx: "6.18",
                cy: "17.82",
            }
            path {
                d: "M4 4.44v2.83c7.03 0 12.73 5.7 12.73 12.73h2.83c0-8.59-6.97-15.56-15.56-15.56zm0 5.66v2.83c3.9 0 7.07 3.17 7.07 7.07h2.83c0-5.47-4.43-9.9-9.9-9.9z",
            }
        }
    }
}

pub fn rtt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M9.03,3l-1.11,7.07h2.62l0.7-4.5h2.58L11.8,18.43H9.47L9.06,21h7.27l0.4-2.57h-2.35l2-12.86h2.58l-0.71,4.5h2.65L22,3H9.03 z M8,5H4L3.69,7h4L8,5z M7.39,9h-4l-0.31,2h4L7.39,9z M8.31,17h-6L2,19h6L8.31,17z M8.93,13h-6l-0.31,2h6.01L8.93,13z",
                }
            }
        }
    }
}

pub fn screen_share_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.11-.9-2-2-2H4c-1.11 0-2 .89-2 2v10c0 1.1.89 2 2 2H0v2h24v-2h-4zM4 16V6h16v10.01L4 16zm9-6.87c-3.89.54-5.44 3.2-6 5.87 1.39-1.87 3.22-2.72 6-2.72v2.19l4-3.74L13 7v2.13z",
            }
        }
    }
}

pub fn send_time_extension_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.5,4.5v6.07l1.5,0.75V4.5C17,3.67,16.33,3,15.5,3H12c0-1.1-0.9-2-2-2S8,1.9,8,3H4.5C3.67,3,3,3.67,3,4.5V8 c1.1,0,2,0.9,2,2s-0.9,2-2,2v3.5C3,16.33,3.67,17,4.5,17H8c0-0.93,0.64-1.71,1.5-1.93v-1.53c-1.18,0.17-2.17,0.93-2.66,1.96H4.5 v-2.34c1.18-0.56,2-1.77,2-3.16s-0.82-2.6-2-3.16V4.5h5V3c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5v1.5H15.5z",
                    }
                    polygon {
                        points: "11,10 11,13 14,14 11,15 11,18 19,14",
                    }
                }
            }
        }
    }
}

pub fn send_time_extension_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18,6v6.26l2,1V6c0-1.1-0.9-2-2-2h-4c0-1.38-1.12-2.5-2.5-2.5S9,2.62,9,4H5.01c-1.1,0-2,0.9-2,2v3.8 C5.7,9.8,6,11.96,6,12.5c0,0.54-0.29,2.7-3,2.7V19c0,1.1,0.9,2,2,2h3.8c0-2.16,1.37-2.78,2.2-2.94v-2.03 C9.57,16.2,7.85,17.07,7.13,19H5v-2.13c2.17-0.8,3-2.87,3-4.37c0-1.49-0.83-3.56-2.99-4.37V6H11V4c0-0.28,0.22-0.5,0.5-0.5 S12,3.72,12,4v2H18z",
                    }
                    polygon {
                        points: "13,12 13,16 17,17 13,18 13,22 23,17",
                    }
                }
            }
        }
    }
}

pub fn sentiment_satisfied_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                cy: "9.5",
                cx: "15.5",
                r: "1.5",
            }
            circle {
                cx: "8.5",
                cy: "9.5",
                r: "1.5",
            }
            path {
                d: "M12 16c-1.48 0-2.75-.81-3.45-2H6.88c.8 2.05 2.79 3.5 5.12 3.5s4.32-1.45 5.12-3.5h-1.67c-.69 1.19-1.97 2-3.45 2zm-.01-14C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

pub fn sip_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M4,4h16c1.1,0,2,0.9,2,2v12c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2V6C2,4.9,2.9,4,4,4z M4,6v12h16V6H4z M11,9h2v6h-2V9z M14,9 h4c0.55,0,1,0.45,1,1v2c0,0.55-0.45,1-1,1h-2.5v2H14V9z M17.5,10.5h-2v1h2V10.5z M6.5,11.25H9c0.55,0,1,0.45,1,1V14 c0,0.55-0.45,1-1,1H5v-1.5h3.5v-0.75H6c-0.55,0-1-0.45-1-1V10c0-0.55,0.45-1,1-1h4v1.5H6.5V11.25z",
                }
            }
        }
    }
}

pub fn speaker_phone_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7 7.07L8.43 8.5c.91-.91 2.18-1.48 3.57-1.48s2.66.57 3.57 1.48L17 7.07C15.72 5.79 13.95 5 12 5s-3.72.79-5 2.07zM12 1C8.98 1 6.24 2.23 4.25 4.21l1.41 1.41C7.28 4 9.53 3 12 3s4.72 1 6.34 2.62l1.41-1.41C17.76 2.23 15.02 1 12 1zm2.86 9.01L9.14 10C8.51 10 8 10.51 8 11.14v9.71c0 .63.51 1.14 1.14 1.14h5.71c.63 0 1.14-.51 1.14-1.14v-9.71c.01-.63-.5-1.13-1.13-1.13zM15 20H9v-8h6v8z",
            }
        }
    }
}

pub fn spoke_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10,3C8.34,3,7,4.34,7,6c0,1.66,1.34,3,3,3s3-1.34,3-3C13,4.34,11.66,3,10,3z M10,7.5C9.17,7.5,8.5,6.83,8.5,6 c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5C11.5,6.83,10.83,7.5,10,7.5z M14,11c-1.66,0-3,1.34-3,3c0,1.66,1.34,3,3,3s3-1.34,3-3 C17,12.34,15.66,11,14,11z M14,15.5c-0.83,0-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5 C15.5,14.83,14.83,15.5,14,15.5z M6,11c-1.66,0-3,1.34-3,3c0,1.66,1.34,3,3,3s3-1.34,3-3C9,12.34,7.66,11,6,11z M6,15.5 c-0.83,0-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5C7.5,14.83,6.83,15.5,6,15.5z",
            }
        }
    }
}

pub fn spoke_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M16,7c0-2.21-1.79-4-4-4S8,4.79,8,7c0,2.21,1.79,4,4,4S16,9.21,16,7z M12,9c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2 C14,8.1,13.1,9,12,9z M7,13c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4s4-1.79,4-4C11,14.79,9.21,13,7,13z M7,19c-1.1,0-2-0.9-2-2 c0-1.1,0.9-2,2-2s2,0.9,2,2C9,18.1,8.1,19,7,19z M17,13c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4s4-1.79,4-4C21,14.79,19.21,13,17,13z M17,19c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2C19,18.1,18.1,19,17,19z",
            }
        }
    }
}

pub fn stay_current_landscape_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M1.01 7L1 17c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2H3c-1.1 0-1.99.9-1.99 2zM19 7v10H5V7h14z",
            }
        }
    }
}

pub fn stay_current_portrait_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 1.01L7 1c-1.1 0-1.99.9-1.99 2v18c0 1.1.89 2 1.99 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z",
            }
        }
    }
}

pub fn stay_primary_landscape_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M1.01 7L1 17c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2H3c-1.1 0-1.99.9-1.99 2zM19 7v10H5V7h14z",
            }
        }
    }
}

pub fn stay_primary_portrait_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 1.01L7 1c-1.1 0-1.99.9-1.99 2v18c0 1.1.89 2 1.99 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z",
            }
        }
    }
}

pub fn stop_screen_share_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.79 18l2 2H24v-2h-2.21zM1.11 2.98l1.55 1.56c-.41.37-.66.89-.66 1.48V16c0 1.1.9 2 2.01 2H0v2h18.13l2.71 2.71 1.41-1.41L2.52 1.57 1.11 2.98zM4 6.02h.13l4.95 4.93C7.94 12.07 7.31 13.52 7 15c.96-1.29 2.13-2.08 3.67-2.46l3.46 3.48H4v-10zm16 0v10.19l1.3 1.3c.42-.37.7-.89.7-1.49v-10c0-1.11-.9-2-2-2H7.8l2 2H20zm-7.07 3.13l2.79 2.78 1.28-1.2L13 7v2.13l-.07.02z",
            }
        }
    }
}

pub fn swap_calls_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18 4l-4 4h3v7c0 1.1-.9 2-2 2s-2-.9-2-2V8c0-2.21-1.79-4-4-4S5 5.79 5 8v7H2l4 4 4-4H7V8c0-1.1.9-2 2-2s2 .9 2 2v7c0 2.21 1.79 4 4 4s4-1.79 4-4V8h3l-4-4z",
            }
        }
    }
}

pub fn textsms_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 2H4c-1.1 0-2 .9-2 2v18l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H6l-2 2V4h16v12zM7 9h2v2H7zm4 0h2v2h-2zm4 0h2v2h-2z",
            }
        }
    }
}

pub fn unsubscribe_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20.99 14.04V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h10.05c.28 1.92 2.1 3.35 4.18 2.93 1.34-.27 2.43-1.37 2.7-2.71.25-1.24-.16-2.39-.94-3.18zm-2-9.04L12 8.5 5 5h13.99zm-3.64 10H5V7l7 3.5L19 7v6.05c-.16-.02-.33-.05-.5-.05-1.39 0-2.59.82-3.15 2zm5.15 2h-4v-1h4v1z",
            }
        }
    }
}

pub fn voicemail_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.5 6C15.46 6 13 8.46 13 11.5c0 1.33.47 2.55 1.26 3.5H9.74c.79-.95 1.26-2.17 1.26-3.5C11 8.46 8.54 6 5.5 6S0 8.46 0 11.5 2.46 17 5.5 17h13c3.04 0 5.5-2.46 5.5-5.5S21.54 6 18.5 6zm-13 9C3.57 15 2 13.43 2 11.5S3.57 8 5.5 8 9 9.57 9 11.5 7.43 15 5.5 15zm13 0c-1.93 0-3.5-1.57-3.5-3.5S16.57 8 18.5 8 22 9.57 22 11.5 20.43 15 18.5 15z",
            }
        }
    }
}

pub fn vpn_key_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22 19h-6v-4h-2.68c-1.14 2.42-3.6 4-6.32 4-3.86 0-7-3.14-7-7s3.14-7 7-7c2.72 0 5.17 1.58 6.32 4H24v6h-2v4zm-4-2h2v-4h2v-2H11.94l-.23-.67C11.01 8.34 9.11 7 7 7c-2.76 0-5 2.24-5 5s2.24 5 5 5c2.11 0 4.01-1.34 4.71-3.33l.23-.67H18v4zM7 15c-1.65 0-3-1.35-3-3s1.35-3 3-3 3 1.35 3 3-1.35 3-3 3zm0-4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z",
            }
        }
    }
}

pub fn vpn_key_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M2.93,2.93L1.87,3.99l1.66,1.66C2.02,6.51,1,8.14,1,10c0,2.76,2.24,5,5,5c1.86,0,3.49-1.02,4.35-2.53l5.66,5.66l1.06-1.06 L2.93,2.93z M6,13.5c-1.93,0-3.5-1.57-3.5-3.5c0-1.45,0.89-2.7,2.15-3.23L5.88,8C4.83,8.06,4,8.93,4,10c0,1.1,0.9,2,2,2 c1.07,0,1.94-0.83,2-1.88l1.24,1.24C8.78,12.27,7.87,13.5,6,13.5z M14.5,12.38V10.5h-1.88L14.5,12.38z M17,14.88V12h2V8h-8.88 l1.5,1.5h5.88v1h-2v2.88L17,14.88z",
                }
            }
        }
    }
}

pub fn vpn_key_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M2.81,2.81L1.39,4.22l2.59,2.59C2.2,7.85,1,9.79,1,12c0,3.31,2.69,6,6,6c2.22,0,4.15-1.21,5.19-3l7.59,7.61l1.41-1.41 L2.81,2.81z M7,16c-2.21,0-4-1.79-4-4c0-1.67,1.02-3.1,2.47-3.7l1.71,1.71C7.12,10,7.06,10,7,10c-1.1,0-2,0.9-2,2s0.9,2,2,2 s2-0.9,2-2c0-0.06,0-0.12-0.01-0.18l1.74,1.74C10.22,14.48,9.14,16,7,16z M17,14.17V13h-1.17L17,14.17z M13.83,11H21v2h-2v3l2,2 v-3h2V9H11.83L13.83,11z",
                    }
                }
            }
        }
    }
}

pub fn wifi_calling_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    path {
                        d: "M16.67,5.3C16.53,5.19,15.11,4,13,4c-2.12,0-3.53,1.19-3.67,1.3L13,10L16.67,5.3z M13,5c0.91,0,1.67,0.27,2.21,0.54 L13,8.37l-2.21-2.83C11.33,5.27,12.09,5,13,5z",
                    }
                    path {
                        d: "M14.33,12.33c-0.83,0-1.63-0.13-2.38-0.38c-0.07-0.02-0.14-0.03-0.21-0.03c-0.17,0-0.34,0.06-0.47,0.19l-1.47,1.47 c-1.89-0.96-3.43-2.5-4.39-4.39l1.47-1.47c0.19-0.18,0.24-0.44,0.17-0.67C6.8,6.3,6.67,5.5,6.67,4.67C6.67,4.3,6.37,4,6,4H3.67 C3.3,4,3,4.3,3,4.67C3,10.93,8.07,16,14.33,16C14.7,16,15,15.7,15,15.33V13C15,12.63,14.7,12.33,14.33,12.33z M4.01,5h1.67 C5.7,5.74,5.82,6.46,6.03,7.15L4.72,8.47C4.3,7.39,4.04,6.22,4.01,5z M14,14.99c-1.22-0.04-2.39-0.29-3.47-0.72l1.31-1.31 c0.7,0.21,1.43,0.33,2.16,0.36V14.99z",
                    }
                }
            }
        }
    }
}

pub fn wifi_calling_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M20,15.51c-1.24,0-2.45-0.2-3.57-0.57c-0.1-0.04-0.21-0.05-0.31-0.05c-0.26,0-0.51,0.1-0.71,0.29l-2.2,2.2 c-2.83-1.45-5.15-3.76-6.59-6.59l2.2-2.2C9.1,8.31,9.18,7.92,9.07,7.57C8.7,6.45,8.5,5.25,8.5,4c0-0.55-0.45-1-1-1H4 C3.45,3,3,3.45,3,4c0,9.39,7.61,17,17,17c0.55,0,1-0.45,1-1v-3.49C21,15.96,20.55,15.51,20,15.51z M5.03,5h1.5 C6.6,5.89,6.75,6.76,6.99,7.59l-1.2,1.2C5.38,7.59,5.12,6.32,5.03,5z M19,18.97c-1.32-0.09-2.59-0.35-3.8-0.75l1.19-1.19 c0.85,0.24,1.72,0.39,2.6,0.45V18.97z",
                        }
                    }
                    g {
                        path {
                            d: "M22,4.95C21.79,4.78,19.67,3,16.5,3c-3.18,0-5.29,1.78-5.5,1.95L16.5,12L22,4.95z",
                        }
                    }
                }
            }
        }
    }
}

