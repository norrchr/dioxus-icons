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
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M15.81,3.13c-1.39-1.18-3.14-1.93-5.06-2.09v1.5c1.51,0.15,2.88,0.75,3.99,1.66L15.81,3.13z",
                    }
                    path {
                        d: "M10,17.5c-2.66,0-4.98-1.41-6.31-3.5h2.06v-1.5H1v4.75h1.5v-2.29C4.11,17.39,6.86,19,10,19c4.01,0,7.41-2.63,8.57-6.25 l-1.47-0.34C16.09,15.36,13.29,17.5,10,17.5z",
                    }
                    path {
                        d: "M9.25,2.54v-1.5C7.33,1.19,5.58,1.96,4.2,3.14L5.26,4.2C6.37,3.29,7.74,2.69,9.25,2.54z",
                    }
                    path {
                        d: "M4.2,5.26L3.14,4.2C1.96,5.59,1.2,7.33,1.04,9.25h1.5C2.69,7.74,3.29,6.37,4.2,5.26z",
                    }
                    polygon {
                        points: "6,10 8.75,11.25 10,14 11.25,11.25 14,10 11.25,8.75 10,6 8.75,8.75",
                    }
                    path {
                        d: "M17.46,9.25h1.51c-0.16-1.92-0.92-3.67-2.1-5.06L15.8,5.26C16.71,6.37,17.31,7.74,17.46,9.25z",
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M19.03,3.56c-1.67-1.39-3.74-2.3-6.03-2.51v2.01c1.73,0.19,3.31,0.88,4.61,1.92L19.03,3.56z",
                    }
                    path {
                        d: "M11,3.06V1.05C8.71,1.25,6.64,2.17,4.97,3.56l1.42,1.42C7.69,3.94,9.27,3.25,11,3.06z",
                    }
                    path {
                        d: "M4.98,6.39L3.56,4.97C2.17,6.64,1.26,8.71,1.05,11h2.01C3.25,9.27,3.94,7.69,4.98,6.39z",
                    }
                    path {
                        d: "M20.94,11h2.01c-0.21-2.29-1.12-4.36-2.51-6.03l-1.42,1.42C20.06,7.69,20.75,9.27,20.94,11z",
                    }
                    polygon {
                        points: "7,12 10.44,13.56 12,17 13.56,13.56 17,12 13.56,10.44 12,7 10.44,10.44",
                    }
                    path {
                        d: "M12,21c-3.11,0-5.85-1.59-7.46-4H7v-2H1v6h2v-2.7c1.99,2.84,5.27,4.7,9,4.7c4.87,0,9-3.17,10.44-7.56l-1.96-0.45 C19.25,18.48,15.92,21,12,21z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    rect {
                        height: "1.5",
                        y: "7.75",
                        x: "5.5",
                        width: "5.5",
                        opacity: ".3",
                    }
                    rect {
                        x: "5.5",
                        y: "4.5",
                        opacity: ".3",
                        width: "5.5",
                        height: "1.75",
                    }
                    rect {
                        height: "1.5",
                        opacity: ".3",
                        width: "2",
                        x: "12.5",
                        y: "7.75",
                    }
                    rect {
                        height: "1.75",
                        opacity: ".3",
                        x: "12.5",
                        y: "4.5",
                        width: "2",
                    }
                    path {
                        d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M5.5,4.5H11v1.75H5.5V4.5z M5.5,7.75H11v1.5H5.5V7.75z M14.5,15.5h-9v-4.75H11v1.53 c-0.3,0.23-0.5,0.57-0.5,0.97c0,0.69,0.56,1.25,1.25,1.25S13,13.94,13,13.25c0-0.4-0.2-0.75-0.5-0.97v-1.53h2V15.5z M14.5,9.25h-2 v-1.5h2V9.25z M14.5,6.25h-2V4.5h2V6.25z",
                    }
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
            }
            g {
                g {
                    rect {
                        width: "8",
                        height: "2",
                        x: "6",
                        opacity: ".3",
                        y: "9",
                    }
                    rect {
                        y: "5",
                        opacity: ".3",
                        width: "8",
                        height: "2",
                        x: "6",
                    }
                    rect {
                        opacity: ".3",
                        height: "2",
                        x: "16",
                        y: "9",
                        width: "2",
                    }
                    rect {
                        x: "16",
                        height: "2",
                        y: "5",
                        opacity: ".3",
                        width: "2",
                    }
                    path {
                        d: "M20,19V3H4v16H2v2h20v-2H20z M6,5h8v2H6V5z M6,9h8v2H6V9z M18,19H6v-6h8v1.82c-0.45,0.32-0.75,0.84-0.75,1.43 c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75c0-0.59-0.3-1.12-0.75-1.43V13h2V19z M18,11h-2V9h2V11z M18,7h-2V5h2V7z",
                    }
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    rect {
                        y: "7.75",
                        width: "5.5",
                        x: "5.5",
                        height: "1.5",
                        opacity: ".3",
                    }
                    rect {
                        height: "1.5",
                        y: "10.75",
                        width: "5.5",
                        x: "5.5",
                        opacity: ".3",
                    }
                    rect {
                        width: "5.5",
                        opacity: ".3",
                        y: "4.5",
                        x: "5.5",
                        height: "1.75",
                    }
                    rect {
                        height: "1.75",
                        y: "13.75",
                        x: "5.5",
                        width: "5.5",
                        opacity: ".3",
                    }
                    rect {
                        x: "12.5",
                        width: "2",
                        y: "4.5",
                        opacity: ".3",
                        height: "1.75",
                    }
                    rect {
                        y: "13.75",
                        height: "1.75",
                        width: "2",
                        opacity: ".3",
                        x: "12.5",
                    }
                    rect {
                        opacity: ".3",
                        height: "1.5",
                        x: "12.5",
                        y: "7.75",
                        width: "2",
                    }
                    rect {
                        height: "1.5",
                        x: "12.5",
                        opacity: ".3",
                        width: "2",
                        y: "10.75",
                    }
                    path {
                        d: "M16,15.5V3H4v12.5H2V17h8.5c0,0.69,0.56,1.25,1.25,1.25S13,17.69,13,17h5v-1.5H16z M11,15.5H5.5v-1.75H11V15.5z M11,12.25 H5.5v-1.5H11V12.25z M11,9.25H5.5v-1.5H11V9.25z M11,6.25H5.5V4.5H11V6.25z M14.5,15.5h-2v-1.75h2V15.5z M14.5,12.25h-2v-1.5h2 V12.25z M14.5,9.25h-2v-1.5h2V9.25z M14.5,6.25h-2V4.5h2V6.25z",
                    }
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    rect {
                        opacity: ".3",
                        height: "2",
                        width: "8",
                        x: "6",
                        y: "5",
                    }
                    rect {
                        height: "2",
                        x: "6",
                        width: "8",
                        opacity: ".3",
                        y: "9",
                    }
                    rect {
                        width: "2",
                        height: "2",
                        opacity: ".3",
                        y: "17",
                        x: "16",
                    }
                    rect {
                        y: "13",
                        height: "2",
                        x: "6",
                        opacity: ".3",
                        width: "8",
                    }
                    rect {
                        width: "8",
                        opacity: ".3",
                        x: "6",
                        height: "2",
                        y: "17",
                    }
                    rect {
                        height: "2",
                        x: "16",
                        width: "2",
                        opacity: ".3",
                        y: "13",
                    }
                    rect {
                        height: "2",
                        width: "2",
                        x: "16",
                        opacity: ".3",
                        y: "5",
                    }
                    rect {
                        opacity: ".3",
                        height: "2",
                        width: "2",
                        x: "16",
                        y: "9",
                    }
                    path {
                        d: "M20,19V3H4v16H2v2h11.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M14,19H6v-2h8V19z M14,15H6v-2h8V15z M14,11H6V9h8V11z M14,7H6V5h8V7z M18,19h-2v-2h2V19z M18,15h-2v-2h2V15z M18,11h-2V9h2V11z M18,7h-2V5h2V7z",
                    }
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    rect {
                        x: "3.5",
                        opacity: ".3",
                        y: "9",
                        width: "2.5",
                        height: "5.5",
                    }
                    path {
                        d: "M16.5,7.55c0.55,0.25,1.05,0.59,1.5,0.99V5.5C18,4.67,17.33,4,16.5,4H4v1.5h12.5V7.55z",
                    }
                    path {
                        d: "M6.5,7.5H3c-0.55,0-1,0.45-1,1V15c0,0.55,0.45,1,1,1h3.5c0.55,0,1-0.45,1-1V8.5C7.5,7.95,7.05,7.5,6.5,7.5z M6,14.5H3.5V9 H6V14.5z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    rect {
                        x: "4",
                        y: "11",
                        width: "3",
                        height: "7",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,8.59c0.73,0.29,1.4,0.69,2,1.17V6c0-1.1-0.9-2-2-2H4v2h16V8.59z",
                    }
                    path {
                        d: "M8,9H3c-0.5,0-1,0.5-1,1v9c0,0.5,0.5,1,1,1h5c0.5,0,1-0.5,1-1v-9C9,9.5,8.5,9,8,9z M7,18H4v-7h3V18z",
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
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M3.5,8.25v7.25h5.05C8.2,14.74,8,13.89,8,13c0-2.44,1.45-4.53,3.54-5.47L8,4.88L3.5,8.25z",
                        opacity: ".3",
                    }
                    path {
                        d: "M3.5,15.5V8.25L8,4.88l3.54,2.66c0.57-0.26,1.19-0.43,1.84-0.5L8,3L2,7.5V17h7.54c-0.4-0.45-0.73-0.95-0.99-1.5H3.5z",
                    }
                    g {
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M4,10v9h5.59C9.21,18.07,9,17.06,9,16c0-3.39,2.11-6.27,5.08-7.44L10,5.5L4,10z",
                        opacity: ".3",
                    }
                    path {
                        d: "M4,19v-9l6-4.5l4.08,3.06c0.81-0.32,1.69-0.51,2.61-0.54L10,3L2,9v12h8.76c-0.48-0.6-0.88-1.27-1.17-2H4z",
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
                g {
                    path {
                        opacity: ".3",
                        d: "M8.71,4.5H5.5v4.7C7.16,8.87,8.48,6.94,8.71,4.5z",
                    }
                    path {
                        d: "M5.5,10.8v4.7h3.21C8.48,13.06,7.16,11.13,5.5,10.8z",
                        opacity: ".3",
                    }
                    path {
                        d: "M11.29,15.5h3.21v-4.7C12.84,11.13,11.52,13.06,11.29,15.5z",
                        opacity: ".3",
                    }
                    path {
                        d: "M14.5,9.2V4.5h-3.21C11.52,6.94,12.84,8.87,14.5,9.2z",
                        opacity: ".3",
                    }
                    path {
                        d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M5.5,4.5h3.21C8.48,6.94,7.16,8.87,5.5,9.2V4.5z M5.5,15.5v-4.7 c1.66,0.33,2.98,2.26,3.21,4.7H5.5z M7.35,10C8.6,9.16,9.57,7.68,10,5.88c0.43,1.8,1.4,3.28,2.65,4.12 c-1.25,0.84-2.22,2.32-2.65,4.12C9.57,12.32,8.6,10.84,7.35,10z M14.5,15.5h-3.21c0.23-2.44,1.55-4.37,3.21-4.7V15.5z M14.5,9.2 c-1.66-0.33-2.98-2.26-3.21-4.7h3.21V9.2z",
                    }
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M6,13.14V19h3.94C9.64,16.07,8.05,13.73,6,13.14z",
                        opacity: ".3",
                    }
                    path {
                        d: "M9.94,5H6v5.86C8.05,10.27,9.64,7.93,9.94,5z",
                        opacity: ".3",
                    }
                    path {
                        opacity: ".3",
                        d: "M14.06,19H18v-5.86C15.95,13.73,14.36,16.07,14.06,19z",
                    }
                    path {
                        opacity: ".3",
                        d: "M18,10.86V5h-3.94C14.36,7.93,15.95,10.27,18,10.86z",
                    }
                    path {
                        d: "M20,19V3H4v16H2v2h20v-2H20z M6,5h3.94C9.64,7.93,8.05,10.27,6,10.86V5z M6,19v-5.86c2.05,0.58,3.64,2.93,3.94,5.86H6z M11.95,19c-0.26-3.06-1.72-5.65-3.76-7c2.04-1.35,3.5-3.94,3.76-7h0.09c0.26,3.06,1.72,5.65,3.76,7c-2.04,1.35-3.5,3.94-3.76,7 H11.95z M18,19h-3.94c0.3-2.93,1.89-5.27,3.94-5.86V19z M18,10.86c-2.05-0.58-3.64-2.93-3.94-5.86H18V10.86z",
                    }
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
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        width: "2",
                        opacity: ".3",
                        height: "11",
                        x: "5.5",
                        y: "4.5",
                    }
                    rect {
                        opacity: ".3",
                        width: "2",
                        y: "4.5",
                        height: "11",
                        x: "12.5",
                    }
                    path {
                        d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M7.5,15.5h-2v-11h2V15.5z M11,15.5H9v-11h2V15.5z M14.5,15.5h-2v-11h2V15.5z",
                    }
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    rect {
                        width: "3",
                        opacity: ".3",
                        x: "6",
                        y: "5",
                        height: "14",
                    }
                    rect {
                        width: "3",
                        x: "15",
                        height: "14",
                        y: "5",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,19V3H4v16H2v2h20v-2H20z M9,19H6V5h3V19z M13,19h-2V5h2V19z M18,19h-3V5h3V19z",
                    }
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
                    width: "20",
                    height: "20",
                    fill: "none",
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
                    width: "24",
                    fill: "none",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M10,3.5C6.97,3.5,4.5,5.97,4.5,9s2.47,5.5,5.5,5.5s5.5-2.47,5.5-5.5S13.03,3.5,10,3.5z M11.75,11.3 L9.3,13.75L8.25,12.7l1-1l-1-1l2.45-2.45l1.05,1.05l-1,1L11.75,11.3z M13,7.5H7V6h6V7.5z",
                    }
                    path {
                        d: "M10,2C6.13,2,3,5.13,3,9c0,2.98,1.87,5.52,4.5,6.53V18H9v-2.08C9.33,15.97,9.66,16,10,16s0.67-0.03,1-0.08V18h1.5v-2.47 C15.13,14.52,17,11.98,17,9C17,5.13,13.87,2,10,2z M10,14.5c-3.03,0-5.5-2.47-5.5-5.5S6.97,3.5,10,3.5s5.5,2.47,5.5,5.5 S13.03,14.5,10,14.5z",
                    }
                    rect {
                        height: "1.5",
                        width: "6",
                        y: "6",
                        x: "7",
                    }
                    polygon {
                        points: "10.7,8.25 8.25,10.7 9.25,11.7 8.25,12.7 9.3,13.75 11.75,11.3 10.75,10.3 11.75,9.3",
                    }
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
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M12,4c-3.86,0-7,3.14-7,7s3.14,7,7,7s7-3.14,7-7S15.86,4,12,4z M14.25,14l-3,3l-1.5-1.5L11,14.25L9.75,13 l3-3l1.5,1.5L13,12.75L14.25,14z M16,9H8V7h8V9z",
                    }
                    path {
                        d: "M12,2c-4.97,0-9,4.03-9,9c0,3.92,2.51,7.24,6,8.48V22h2v-2.06c0.33,0.04,0.66,0.06,1,0.06s0.67-0.02,1-0.06V22h2v-2.52 c3.49-1.24,6-4.56,6-8.48C21,6.03,16.97,2,12,2z M12,18c-3.86,0-7-3.14-7-7s3.14-7,7-7s7,3.14,7,7S15.86,18,12,18z",
                    }
                    rect {
                        height: "2",
                        width: "8",
                        x: "8",
                        y: "7",
                    }
                    polygon {
                        points: "12.75,10 9.75,13 11,14.25 9.75,15.5 11.25,17 14.25,14 13,12.75 14.25,11.5",
                    }
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M6.15,6.08C5.08,7.11,4.5,8.5,4.5,10c0,3.03,2.47,5.5,5.5,5.5c1.47,0,2.85-0.57,3.89-1.61 c1.04-1.04,1.61-2.42,1.61-3.89V4.5H10C8.56,4.5,7.19,5.06,6.15,6.08z M11.57,6.12c0.14,0.14,0.15,0.37,0.03,0.52L9.58,9.22 l2.93,0.18c0.45,0.03,0.65,0.58,0.32,0.9l-3.86,3.59c-0.16,0.15-0.4,0.14-0.55-0.01c-0.14-0.14-0.15-0.37-0.03-0.52l2.02-2.58 L7.49,10.6c-0.45-0.03-0.65-0.58-0.32-0.9l3.86-3.59C11.17,5.96,11.42,5.96,11.57,6.12z",
                    }
                    path {
                        d: "M10,3C10,3,10,3,10,3c-3.73,0-7,3-7,7c0,1.66,0.58,3.19,1.55,4.39L3,15.94L4.06,17l1.55-1.55C6.81,16.42,8.34,17,10,17 c1.79,0,3.58-0.68,4.95-2.05C16.32,13.58,17,11.79,17,10V3H10z M15.5,10c0,1.47-0.57,2.85-1.61,3.89 c-1.04,1.04-2.42,1.61-3.89,1.61c-3.03,0-5.5-2.47-5.5-5.5c0-1.5,0.58-2.89,1.65-3.92C7.19,5.06,8.56,4.5,10,4.5h5.5V10z",
                    }
                    path {
                        d: "M7.49,10.6l2.93,0.18L8.4,13.36c-0.12,0.16-0.11,0.38,0.03,0.52c0.15,0.15,0.4,0.15,0.55,0.01l3.86-3.59 c0.33-0.31,0.13-0.87-0.32-0.9L9.58,9.22l2.02-2.58c0.12-0.16,0.11-0.38-0.03-0.52c-0.15-0.15-0.4-0.15-0.55-0.01L7.16,9.7 C6.83,10.01,7.03,10.57,7.49,10.6z",
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        opacity: ".3",
                        path {
                            d: "M7.1,7.01C5.74,8.32,5,10.1,5,12c0,3.86,3.14,7,7,7c1.87,0,3.63-0.73,4.95-2.05C18.27,15.63,19,13.87,19,12V5h-7 C10.16,5,8.42,5.71,7.1,7.01z M13.88,7.12c0.14,0.14,0.16,0.36,0.04,0.52l-2.44,3.33l4.05,0.4c0.44,0.04,0.63,0.59,0.3,0.89 l-5.16,4.63c-0.16,0.15-0.41,0.14-0.56-0.01c-0.14-0.14-0.16-0.36-0.04-0.52l2.44-3.33l-4.05-0.4c-0.44-0.04-0.63-0.59-0.3-0.89 l5.16-4.63C13.48,6.96,13.73,6.97,13.88,7.12z",
                        }
                    }
                    path {
                        d: "M12,3C12,3,12,3,12,3c-4.8,0-9,3.86-9,9c0,2.12,0.74,4.07,1.97,5.61L3,19.59L4.41,21l1.97-1.97C7.93,20.26,9.88,21,12,21 c2.3,0,4.61-0.88,6.36-2.64C20.12,16.61,21,14.3,21,12V3H12z M19,12c0,1.87-0.73,3.63-2.05,4.95C15.63,18.27,13.87,19,12,19 c-3.86,0-7-3.14-7-7c0-1.9,0.74-3.68,2.1-4.99C8.42,5.71,10.16,5,12,5h7V12z",
                    }
                    path {
                        d: "M8.46,12.63l4.05,0.4l-2.44,3.33c-0.11,0.16-0.1,0.38,0.04,0.52c0.15,0.15,0.4,0.16,0.56,0.01l5.16-4.63 c0.33-0.3,0.15-0.85-0.3-0.89l-4.05-0.4l2.44-3.33c0.11-0.16,0.1-0.38-0.04-0.52c-0.15-0.15-0.4-0.16-0.56-0.01l-5.16,4.63 C7.84,12.04,8.02,12.59,8.46,12.63z",
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
                g {
                    path {
                        d: "M13,5H7C6.17,5,5.5,5.67,5.5,6.5V15c0,0.83,0.67,1.5,1.5,1.5h6c0.83,0,1.5-0.67,1.5-1.5V6.5 C14.5,5.67,13.83,5,13,5z M10,14.5c-1.1,0-2-0.88-2-1.97c0-0.87,0.35-1.11,2-3.03c1.64,1.91,2,2.16,2,3.03 C12,13.62,11.1,14.5,10,14.5z M13,8H7V6.5h6V8z",
                        opacity: ".3",
                    }
                    path {
                        d: "M13,3.5h-0.5V2H11v1.5H9V2H7.5v1.5H7c-1.66,0-3,1.34-3,3V15c0,1.66,1.34,3,3,3h6c1.66,0,3-1.34,3-3V6.5 C16,4.84,14.66,3.5,13,3.5z M14.5,15c0,0.83-0.67,1.5-1.5,1.5H7c-0.83,0-1.5-0.67-1.5-1.5V6.5C5.5,5.67,6.17,5,7,5h6 c0.83,0,1.5,0.67,1.5,1.5V15z",
                    }
                    path {
                        d: "M8,12.53c0,1.09,0.9,1.97,2,1.97s2-0.88,2-1.97c0-0.87-0.36-1.13-2-3.03C8.35,11.42,8,11.66,8,12.53z",
                    }
                    rect {
                        y: "6.5",
                        width: "6",
                        height: "1.5",
                        x: "7",
                    }
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M16,6H8C6.9,6,6,6.9,6,8v10c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V8C18,6.9,17.1,6,16,6z M12,18 c-1.38,0-2.5-1.1-2.5-2.46c0-1.09,0.43-1.39,2.5-3.79c2.05,2.38,2.5,2.7,2.5,3.79C14.5,16.9,13.38,18,12,18z M16,10H8V8h8V10z",
                        opacity: ".3",
                    }
                    path {
                        d: "M16,4h-1V2h-2v2h-2V2H9v2H8C5.79,4,4,5.79,4,8v10c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4V8C20,5.79,18.21,4,16,4z M18,18 c0,1.1-0.9,2-2,2H8c-1.1,0-2-0.9-2-2V8c0-1.1,0.9-2,2-2h8c1.1,0,2,0.9,2,2V18z",
                    }
                    path {
                        d: "M9.5,15.54C9.5,16.9,10.62,18,12,18s2.5-1.1,2.5-2.46c0-1.09-0.45-1.41-2.5-3.79C9.93,14.15,9.5,14.46,9.5,15.54z",
                    }
                    rect {
                        height: "2",
                        width: "8",
                        x: "8",
                        y: "8",
                    }
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
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M4.5,15.5h11v-11h-11V15.5z M10,5.5c2.48,0,4.5,2.02,4.5,4.5s-2.02,4.5-4.5,4.5S5.5,12.48,5.5,10 S7.52,5.5,10,5.5z",
                    }
                    path {
                        d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M15.5,15.5h-11v-11h11V15.5z",
                    }
                    path {
                        d: "M10,14.5c2.48,0,4.5-2.02,4.5-4.5S12.48,5.5,10,5.5S5.5,7.52,5.5,10S7.52,14.5,10,14.5z M9.5,12.95 c-0.45-0.08-0.87-0.25-1.23-0.51l1.23-1.23V12.95z M10.5,12.95v-1.74l1.23,1.23C11.37,12.7,10.95,12.87,10.5,12.95z M12.44,11.73 l-1.23-1.23h1.74C12.87,10.95,12.7,11.37,12.44,11.73z M12.95,9.5h-1.74l1.23-1.23C12.7,8.63,12.87,9.05,12.95,9.5z M10.5,7.05 c0.45,0.08,0.87,0.25,1.23,0.51L10.5,8.79V7.05z M10.75,10c0,0.41-0.34,0.75-0.75,0.75S9.25,10.41,9.25,10 c0-0.41,0.34-0.75,0.75-0.75S10.75,9.59,10.75,10z M9.5,7.05v1.74L8.27,7.56C8.63,7.3,9.05,7.13,9.5,7.05z M7.56,8.27L8.79,9.5 H7.05C7.13,9.05,7.3,8.63,7.56,8.27z M8.79,10.5l-1.23,1.23c-0.26-0.36-0.43-0.78-0.51-1.23H8.79z",
                    }
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
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M5,19h14V5H5V19z M12,6c3.31,0,6,2.69,6,6s-2.69,6-6,6s-6-2.69-6-6S8.69,6,12,6z",
                        opacity: ".3",
                    }
                    path {
                        d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z",
                    }
                    path {
                        d: "M12,18c3.31,0,6-2.69,6-6s-2.69-6-6-6s-6,2.69-6,6S8.69,18,12,18z M11.25,15.92c-0.55-0.1-1.05-0.32-1.5-0.62l1.5-1.5 V15.92z M12.75,15.92v-2.11l1.5,1.5C13.8,15.61,13.3,15.82,12.75,15.92z M15.31,14.25l-1.5-1.5h2.11 C15.82,13.3,15.61,13.8,15.31,14.25z M15.92,11.25h-2.11l1.5-1.5C15.61,10.2,15.82,10.7,15.92,11.25z M12.75,8.08 c0.55,0.1,1.05,0.32,1.5,0.62l-1.5,1.5V8.08z M12,11c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1s-1-0.45-1-1C11,11.45,11.45,11,12,11z M11.25,8.08v2.11l-1.5-1.5C10.2,8.39,10.7,8.18,11.25,8.08z M8.69,9.75l1.5,1.5H8.08C8.18,10.7,8.39,10.2,8.69,9.75z M10.19,12.75l-1.5,1.5c-0.3-0.44-0.51-0.95-0.62-1.5H10.19z",
                    }
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
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M4,8.2c-0.07,0-0.4,0.02-0.4,0.48c0,1.6,0.66,2.72,1.6,2.72c0.39,0,0.62-0.07,2.14-0.68 c-0.04-0.16-0.07-0.33-0.09-0.5C6.01,9.94,5.1,9.36,4.49,8.45C4.39,8.29,4.28,8.2,4,8.2z",
                        opacity: ".3",
                    }
                    path {
                        d: "M11.55,4.49C11.71,4.39,11.8,4.28,11.8,4c0-0.07-0.02-0.4-0.48-0.4c-1.6,0-2.72,0.66-2.72,1.6 c0,0.36,0.06,0.6,0.56,1.84l0.57,0.57l0.05-0.35C10.06,6.01,10.64,5.1,11.55,4.49z",
                        opacity: ".3",
                    }
                    path {
                        d: "M8.2,16c0,0.07,0.02,0.4,0.48,0.4c1.6,0,2.72-0.66,2.72-1.6c0-0.36-0.06-0.6-0.56-1.84l-0.28-0.28 c-0.11,0.02-0.23,0.05-0.34,0.06c-0.28,1.25-0.86,2.16-1.77,2.77C8.29,15.61,8.2,15.72,8.2,16z",
                        opacity: ".3",
                    }
                    path {
                        d: "M15.51,11.55c0.11,0.16,0.22,0.25,0.49,0.25c0.07,0,0.4-0.02,0.4-0.48c0-1.6-0.66-2.72-1.6-2.72 c-0.39,0-0.62,0.07-2.14,0.68c0.04,0.16,0.07,0.33,0.09,0.5C13.99,10.06,14.9,10.64,15.51,11.55z",
                        opacity: ".3",
                    }
                    g {
                        g {
                            path {
                                d: "M8.6,5.2c0-0.94,1.12-1.6,2.72-1.6c0.46,0,0.48,0.33,0.48,0.4c0,0.28-0.09,0.39-0.25,0.49 c-0.91,0.61-1.49,1.52-1.77,2.77L9.73,7.61l5.73,5.73c0.17,0.04,0.35,0.06,0.54,0.06c0.98,0,2-0.76,2-2.08 C18,8.52,16.49,7,14.8,7c-0.83,0-1.29,0.22-3.01,0.91c-0.14-0.12-0.28-0.22-0.44-0.31c0.22-0.99,0.64-1.47,1.1-1.78 C13.09,5.39,13.4,4.75,13.4,4c0-0.98-0.76-2-2.08-2C8.69,2,7.2,3.34,7.03,4.91l2.13,2.13C8.66,5.8,8.6,5.56,8.6,5.2z M14.8,8.6 c0.94,0,1.6,1.12,1.6,2.72c0,0.46-0.33,0.48-0.4,0.48c-0.28,0-0.39-0.09-0.49-0.25c-0.61-0.91-1.52-1.49-2.77-1.77 c-0.01-0.17-0.04-0.34-0.09-0.5C14.18,8.67,14.41,8.6,14.8,8.6z",
                            }
                        }
                    }
                    path {
                        d: "M1.87,3.99l2.67,2.67C4.37,6.62,4.19,6.6,4,6.6c-0.98,0-2,0.76-2,2.08C2,11.48,3.51,13,5.2,13c0.83,0,1.29-0.22,3.01-0.91 c0.14,0.12,0.28,0.22,0.44,0.31c-0.22,0.99-0.64,1.47-1.1,1.78C6.91,14.61,6.6,15.25,6.6,16c0,0.98,0.76,2,2.08,2 c2.63,0,4.12-1.34,4.29-2.91l3.04,3.04l1.06-1.06L2.93,2.93L1.87,3.99z M10.22,12.74c0.12-0.01,0.23-0.04,0.34-0.06l0.28,0.28 c0.5,1.24,0.56,1.48,0.56,1.84c0,0.94-1.12,1.6-2.72,1.6c-0.46,0-0.48-0.33-0.48-0.4c0-0.28,0.09-0.39,0.25-0.49 C9.36,14.9,9.94,13.99,10.22,12.74z M7.26,10.22c0.01,0.17,0.04,0.34,0.09,0.5C5.82,11.33,5.59,11.4,5.2,11.4 c-0.94,0-1.6-1.12-1.6-2.72C3.6,8.22,3.93,8.2,4,8.2c0.28,0,0.39,0.09,0.49,0.25C5.1,9.36,6.01,9.94,7.26,10.22z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M4.5,10C4.42,10,4,10.03,4,10.6c0,2,0.82,3.4,2,3.4c0.51,0,0.89-0.12,2.69-0.86 c-0.07-0.2-0.12-0.41-0.15-0.62c-1.48-0.33-2.49-0.89-3.39-2.16C4.93,10.05,4.77,10,4.5,10z",
                        opacity: ".3",
                    }
                    path {
                        d: "M10,19.5c0,0.08,0.03,0.5,0.6,0.5c2,0,3.4-0.82,3.4-2c0-0.38-0.07-0.69-0.42-1.59l-0.97-0.97 c-0.03,0.01-0.06,0.02-0.09,0.02c-0.33,1.48-0.89,2.49-2.16,3.39C10.05,19.07,10,19.23,10,19.5z",
                        opacity: ".3",
                    }
                    path {
                        d: "M13.65,5.15C13.95,4.93,14,4.77,14,4.5C14,4.42,13.97,4,13.4,4c-2,0-3.4,0.82-3.4,2 c0,0.38,0.07,0.69,0.42,1.59l1.05,1.05l0.02-0.1C11.82,7.06,12.37,6.04,13.65,5.15z",
                        opacity: ".3",
                    }
                    path {
                        d: "M18.85,13.65c0.21,0.3,0.38,0.35,0.65,0.35c0.08,0,0.5-0.03,0.5-0.6c0-2-0.82-3.4-2-3.4 c-0.51,0-0.89,0.12-2.69,0.86c0.07,0.2,0.12,0.41,0.15,0.62C16.94,11.82,17.96,12.37,18.85,13.65z",
                        opacity: ".3",
                    }
                    g {
                        g {
                            path {
                                d: "M10,6c0-1.18,1.4-2,3.4-2C13.97,4,14,4.42,14,4.5c0,0.27-0.05,0.43-0.35,0.65c-1.27,0.9-1.83,1.91-2.16,3.39l-0.02,0.1 l7.25,7.25C18.96,15.95,19.22,16,19.5,16c1.22,0,2.5-0.95,2.5-2.6C22,9.91,20.11,8,18,8c-1.06,0-1.64,0.29-3.91,1.19 c-0.19-0.14-0.4-0.27-0.62-0.37c0.25-1.03,0.61-1.53,1.33-2.04C15.61,6.21,16,5.44,16,4.5C16,3.28,15.05,2,13.4,2 c-3.08,0-4.92,1.47-5.32,3.26l2.33,2.33C10.07,6.69,10,6.38,10,6z M18,10c1.18,0,2,1.4,2,3.4c0,0.57-0.42,0.6-0.5,0.6 c-0.27,0-0.43-0.05-0.65-0.35c-0.9-1.27-1.91-1.83-3.39-2.16c-0.03-0.22-0.08-0.42-0.15-0.62C17.11,10.12,17.49,10,18,10z",
                            }
                        }
                    }
                    path {
                        d: "M1.39,4.22l3.89,3.89C5.04,8.05,4.78,8,4.5,8C3.28,8,2,8.95,2,10.6C2,14.09,3.89,16,6,16c1.06,0,1.64-0.29,3.91-1.19 c0.19,0.14,0.4,0.27,0.62,0.37c-0.25,1.03-0.61,1.53-1.33,2.04C8.39,17.79,8,18.56,8,19.5c0,1.22,0.95,2.5,2.6,2.5 c3.08,0,4.92-1.47,5.32-3.26l3.86,3.86l1.41-1.41L2.81,2.81L1.39,4.22z M12.52,15.46c0.03,0,0.06-0.02,0.09-0.02l0.97,0.97 C13.93,17.31,14,17.62,14,18c0,1.18-1.4,2-3.4,2c-0.57,0-0.6-0.42-0.6-0.5c0-0.27,0.05-0.43,0.35-0.65 C11.63,17.96,12.18,16.94,12.52,15.46z M8.54,12.52c0.03,0.22,0.08,0.42,0.15,0.62C6.89,13.88,6.51,14,6,14c-1.18,0-2-1.4-2-3.4 C4,10.03,4.42,10,4.5,10c0.27,0,0.43,0.05,0.65,0.35C6.04,11.63,7.06,12.18,8.54,12.52z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M9.91,2.96C7.96,3.19,6.5,4.82,6.5,6.75s1.46,3.56,3.39,3.79L13.5,11l0-8.5l0,0L9.91,2.96z",
                    }
                    path {
                        d: "M10,14.5c-1.38,0-2.5,1.12-2.5,2.5v0.5h5V17C12.5,15.62,11.38,14.5,10,14.5z",
                        opacity: ".3",
                    }
                    path {
                        d: "M13.5,1c-0.06,0-3.79,0.47-3.79,0.47C7.02,1.79,5,4.06,5,6.75s2.02,4.96,4.72,5.28l1.37,0.17l-0.57,0.85 C10.35,13.03,10.18,13,10,13c-2.21,0-4,1.79-4,4v1.5C6,18.78,6.22,19,6.5,19h7c0.28,0,0.5-0.22,0.5-0.5V17 c0-1.48-0.81-2.76-2.01-3.45l0.76-1.13c0.61,0.08,0.63,0.08,0.76,0.08c0.81,0,1.5-0.66,1.5-1.49V2.49C15,1.66,14.32,1,13.5,1z M12.5,17v0.5h-5V17c0-1.38,1.12-2.5,2.5-2.5S12.5,15.62,12.5,17z M13.5,11l-3.6-0.46C7.96,10.31,6.5,8.68,6.5,6.75 s1.46-3.56,3.41-3.79L13.5,2.5l0,0L13.5,11z",
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
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M12,17c-1.65,0-3,1.35-3,3v1h6v-1C15,18.35,13.65,17,12,17z",
                    }
                    path {
                        d: "M16,3l-4.08,0.41C9.69,3.64,8,5.5,8,7.75s1.69,4.11,3.92,4.34l4.11,0.42L16,3L16,3z",
                        opacity: ".3",
                    }
                    path {
                        d: "M16,1c-0.15,0,0.11-0.02-4.28,0.42C8.47,1.75,6,4.48,6,7.75s2.47,6,5.72,6.33l1.9,0.19l-0.56,0.85 C12.71,15.04,12.36,15,12,15c-2.76,0-5,2.24-5,5v2c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v-2c0-1.67-0.83-3.15-2.09-4.06 l0.97-1.45c0.04,0,0.09,0.01,0.13,0.01c1.09,0,2-0.89,2-2V3C18,1.89,17.09,1,16,1z M15,20v1H9v-1c0-1.65,1.35-3,3-3 C13.65,17,15,18.35,15,20z M11.92,12.09C9.69,11.86,8,10,8,7.75s1.69-4.11,3.92-4.34L16,3h0l0.03,9.5L11.92,12.09z",
                    }
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
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M6,9.25c0.41,0,0.75,0.34,0.75,0.75S6.41,10.75,6,10.75v4.75h8v-4.75h-0.25C13.34,10.75,13,10.41,13,10 s0.34-0.75,0.75-0.75H14V4.5H6V9.25z M10,7.38c1.85,2.15,2.25,2.43,2.25,3.41c0,1.22-1.01,2.21-2.25,2.21 c-1.24,0-2.25-0.99-2.25-2.21C7.75,9.81,8.14,9.53,10,7.38z",
                        opacity: ".3",
                    }
                    path {
                        d: "M10,13c1.24,0,2.25-0.99,2.25-2.21c0-0.98-0.4-1.27-2.25-3.41c-1.86,2.16-2.25,2.43-2.25,3.41C7.75,12.01,8.76,13,10,13z",
                    }
                    path {
                        d: "M16.25,10.75c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75H15.5V4.5h0.75C16.66,4.5,17,4.16,17,3.75S16.66,3,16.25,3 H3.75C3.34,3,3,3.34,3,3.75S3.34,4.5,3.75,4.5H4.5v4.75H3.75C3.34,9.25,3,9.59,3,10s0.34,0.75,0.75,0.75H4.5v4.75H3.75 C3.34,15.5,3,15.84,3,16.25S3.34,17,3.75,17h12.5c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75H15.5v-4.75H16.25z M14,9.25 h-0.25C13.34,9.25,13,9.59,13,10s0.34,0.75,0.75,0.75H14v4.75H6v-4.75c0.41,0,0.75-0.34,0.75-0.75S6.41,9.25,6,9.25V4.5h8V9.25z",
                    }
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
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M7,11c0.55,0,1,0.45,1,1s-0.45,1-1,1v6h10v-6c-0.55,0-1-0.45-1-1s0.45-1,1-1V5H7V11z M12,8.5 c2.47,2.86,3,3.24,3,4.55c0,1.63-1.34,2.95-3,2.95s-3-1.32-3-2.95C9,11.75,9.52,11.38,12,8.5z",
                        opacity: ".3",
                    }
                    path {
                        d: "M12,16c1.66,0,3-1.32,3-2.95c0-1.31-0.53-1.69-3-4.55c-2.48,2.88-3,3.25-3,4.55C9,14.68,10.34,16,12,16z",
                    }
                    path {
                        d: "M20,13c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1V5h1c0.55,0,1-0.45,1-1s-0.45-1-1-1H4C3.45,3,3,3.45,3,4s0.45,1,1,1h1v6H4 c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v6H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h16c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1v-6H20z M17,11 c-0.55,0-1,0.45-1,1s0.45,1,1,1v6H7v-6c0.55,0,1-0.45,1-1s-0.45-1-1-1V5h10V11z",
                    }
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M14,6.5H6c-1.93,0-3.5,1.57-3.5,3.5s1.57,3.5,3.5,3.5h8c1.93,0,3.5-1.57,3.5-3.5S15.93,6.5,14,6.5z",
                    }
                    path {
                        d: "M14,5h-1V4c0-0.83-0.67-1.5-1.5-1.5h-3C7.67,2.5,7,3.17,7,4v1H6c-2.76,0-5,2.24-5,5s2.24,5,5,5v2h1.5v-2h5v2H14v-2 c2.76,0,5-2.24,5-5S16.76,5,14,5z M8.5,4h3v1h-3V4z M14,13.5H6c-1.93,0-3.5-1.57-3.5-3.5S4.07,6.5,6,6.5h8 c1.93,0,3.5,1.57,3.5,3.5S15.93,13.5,14,13.5z",
                    }
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M17,8H7c-2.21,0-4,1.79-4,4s1.79,4,4,4h10c2.21,0,4-1.79,4-4S19.21,8,17,8z",
                        opacity: ".3",
                    }
                    path {
                        d: "M17,6h-1V5c0-1.1-0.9-2-2-2h-4C8.9,3,8,3.9,8,5v1H7c-3.31,0-6,2.69-6,6s2.69,6,6,6v3h2v-3h6v3h2v-3c3.31,0,6-2.69,6-6 S20.31,6,17,6z M10,5h4v1h-4V5z M17,16H7c-2.21,0-4-1.79-4-4s1.79-4,4-4h10c2.21,0,4,1.79,4,4S19.21,16,17,16z",
                    }
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M5.5,15c0,0.83,0.67,1.5,1.5,1.5h6c0.83,0,1.5-0.67,1.5-1.5v-2.75h-9V15z",
                    }
                    path {
                        opacity: ".3",
                        d: "M13,6.5H7C6.17,6.5,5.5,7.17,5.5,8v2.75h9V8C14.5,7.17,13.83,6.5,13,6.5z",
                    }
                    path {
                        d: "M14,5.18V3.5C14,2.67,13.33,2,12.5,2h-5C6.67,2,6,2.67,6,3.5v1.68C4.84,5.6,4,6.7,4,8v7c0,1.66,1.34,3,3,3h6 c1.66,0,3-1.34,3-3V8C16,6.7,15.16,5.6,14,5.18z M7.5,3.5h5V5h-1.75c0-0.41-0.34-0.75-0.75-0.75S9.25,4.59,9.25,5H7.5V3.5z M14.5,15c0,0.83-0.67,1.5-1.5,1.5H7c-0.83,0-1.5-0.67-1.5-1.5v-2.75h9V15z M14.5,10.75h-9V8c0-0.83,0.67-1.5,1.5-1.5h6 c0.83,0,1.5,0.67,1.5,1.5V10.75z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M6,18c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-3H6V18z",
                        opacity: ".3",
                    }
                    path {
                        d: "M16,8H8c-1.1,0-2,0.9-2,2v3h12v-3C18,8.9,17.1,8,16,8z",
                        opacity: ".3",
                    }
                    path {
                        d: "M17,6.14V4c0-1.1-0.9-2-2-2H9C7.9,2,7,2.9,7,4v2.14c-1.72,0.45-3,2-3,3.86v8c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4v-8 C20,8.14,18.72,6.59,17,6.14z M9,4h6v2h-2c0-0.55-0.45-1-1-1s-1,0.45-1,1H9V4z M18,18c0,1.1-0.9,2-2,2H8c-1.1,0-2-0.9-2-2v-3h12 V18z M18,13H6v-3c0-1.1,0.9-2,2-2h8c1.1,0,2,0.9,2,2V13z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    rect {
                        width: "9",
                        x: "5.5",
                        y: "4.5",
                        height: "4.75",
                        opacity: ".3",
                    }
                    path {
                        d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M14.5,15.5h-9v-4.75h3.75v1.53c-0.3,0.23-0.5,0.57-0.5,0.97 c0,0.69,0.56,1.25,1.25,1.25s1.25-0.56,1.25-1.25c0-0.4-0.2-0.75-0.5-0.97v-1.53h3.75V15.5z M14.5,9.25h-9V4.5h9V9.25z",
                    }
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    rect {
                        height: "6",
                        opacity: ".3",
                        width: "12",
                        x: "6",
                        y: "5",
                    }
                    path {
                        d: "M20,19V3H4v16H2v2h20v-2H20z M18,19H6v-6h5v1.82c-0.45,0.32-0.75,0.84-0.75,1.43c0,0.97,0.78,1.75,1.75,1.75 s1.75-0.78,1.75-1.75c0-0.59-0.3-1.12-0.75-1.43V13h5V19z M18,11H6V5h12V11z",
                    }
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        opacity: ".3",
                        rect {
                            height: "8",
                            width: "9",
                            y: "4.5",
                            x: "5.5",
                        }
                    }
                    path {
                        d: "M16,15.5V3H4v12.5H2V17h6.75c0,0.69,0.56,1.25,1.25,1.25s1.25-0.56,1.25-1.25H18v-1.5H16z M9.25,15.5H5.5V14h3.75V15.5z M14.5,15.5h-3.75V14h3.75V15.5z M14.5,12.5h-9v-8h9V12.5z",
                    }
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
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        opacity: ".3",
                        rect {
                            x: "6",
                            y: "5",
                            height: "10",
                            width: "12",
                        }
                    }
                    path {
                        d: "M20,19V3H4v16H2v2h8.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M11,19H6v-2h5V19z M18,19h-5v-2h5V19z M18,15H6V5h12V15z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
                path {
                    d: "M15,4v12H5V4H15 M13,9c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S13.55,9,13,9z",
                    opacity: ".3",
                }
                path {
                    d: "M15,4v12H5V4H15 M15,3H5C4.45,3,4,3.45,4,4v13h12V4C16,3.45,15.55,3,15,3L15,3z M13,9c-0.55,0-1,0.45-1,1s0.45,1,1,1 s1-0.45,1-1S13.55,9,13,9z",
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
                    height: "24",
                    width: "24",
                }
                path {
                    d: "M18,4v16H6V4H18 M15.5,10.5c-0.83,0-1.5,0.67-1.5,1.5s0.67,1.5,1.5,1.5c0.83,0,1.5-0.67,1.5-1.5 S16.33,10.5,15.5,10.5z",
                    opacity: ".3",
                }
                path {
                    d: "M18,4v16H6V4H18 M18,2H6C4.9,2,4,2.9,4,4v18h16V4C20,2.9,19.1,2,18,2L18,2z M15.5,10.5c-0.83,0-1.5,0.67-1.5,1.5 s0.67,1.5,1.5,1.5c0.83,0,1.5-0.67,1.5-1.5S16.33,10.5,15.5,10.5z",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M10,11.5c-1.21,0-2.39,0.32-3.44,0.94c-0.02,0.01-0.04,0.04-0.05,0.06h6.97c-0.01-0.03-0.03-0.05-0.05-0.06 C12.39,11.82,11.21,11.5,10,11.5z",
                    }
                    circle {
                        cx: "10",
                        r: "1",
                        cy: "6.5",
                        opacity: ".3",
                    }
                    path {
                        d: "M17.84,6.73l1.39-0.58c-1.01-2.43-2.96-4.37-5.38-5.38l-0.58,1.39C15.33,3.02,16.98,4.67,17.84,6.73z",
                    }
                    path {
                        d: "M13.27,17.84l0.58,1.39c2.43-1.01,4.37-2.96,5.38-5.38l-1.39-0.58C16.98,15.33,15.33,16.98,13.27,17.84z",
                    }
                    path {
                        d: "M2.16,13.27l-1.39,0.58c1.01,2.43,2.96,4.37,5.38,5.38l0.58-1.39C4.67,16.98,3.02,15.33,2.16,13.27z",
                    }
                    path {
                        d: "M6.73,2.16L6.15,0.77C3.72,1.78,1.78,3.72,0.77,6.15l1.39,0.58C3.02,4.67,4.67,3.02,6.73,2.16z",
                    }
                    path {
                        d: "M10,9c1.38,0,2.5-1.12,2.5-2.5C12.5,5.12,11.38,4,10,4S7.5,5.12,7.5,6.5C7.5,7.88,8.62,9,10,9z M10,5.5c0.55,0,1,0.45,1,1 s-0.45,1-1,1s-1-0.45-1-1S9.45,5.5,10,5.5z",
                    }
                    path {
                        d: "M10,10c-1.53,0-2.96,0.42-4.2,1.14C5.3,11.44,5,11.99,5,12.57V14h10v-1.43c0-0.58-0.3-1.14-0.8-1.43 C12.96,10.42,11.53,10,10,10z M6.51,12.5c0.01-0.03,0.03-0.05,0.05-0.06C7.61,11.82,8.79,11.5,10,11.5c1.21,0,2.39,0.32,3.44,0.94 c0.02,0.01,0.04,0.04,0.05,0.06H6.51z",
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
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M8.14,15h7.7c-1.16-0.65-2.5-1-3.85-1C10.65,14,9.32,14.35,8.14,15z",
                    }
                    circle {
                        cy: "8",
                        cx: "12",
                        opacity: ".3",
                        r: "1",
                    }
                    path {
                        d: "M12,11c1.66,0,3-1.34,3-3s-1.34-3-3-3S9,6.34,9,8S10.34,11,12,11z M12,7c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1 S11.45,7,12,7z",
                    }
                    path {
                        d: "M12,12c-1.84,0-3.56,0.5-5.03,1.37C6.36,13.72,6,14.39,6,15.09V17h12v-1.91c0-0.7-0.36-1.36-0.97-1.72 C15.56,12.5,13.84,12,12,12z M8.14,15c1.18-0.65,2.51-1,3.86-1c1.35,0,2.68,0.35,3.85,1H8.14z",
                    }
                    path {
                        d: "M21.23,8.15l1.85-0.77c-1.22-2.91-3.55-5.25-6.46-6.46l-0.77,1.85C18.27,3.79,20.21,5.73,21.23,8.15z",
                    }
                    path {
                        d: "M8.15,2.77L7.38,0.92C4.47,2.14,2.14,4.47,0.92,7.38l1.85,0.77C3.79,5.73,5.73,3.79,8.15,2.77z",
                    }
                    path {
                        d: "M2.77,15.85l-1.85,0.77c1.22,2.91,3.55,5.25,6.46,6.46l0.77-1.85C5.73,20.21,3.79,18.27,2.77,15.85z",
                    }
                    path {
                        d: "M15.85,21.23l0.77,1.85c2.91-1.22,5.25-3.55,6.46-6.46l-1.85-0.77C20.21,18.27,18.27,20.21,15.85,21.23z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
                path {
                    opacity: ".3",
                    d: "M15,16H5v-6h10V16z M15,9h-3.5V8h-3v1H5V4h10V9z",
                }
                path {
                    d: "M15,3H5C4.45,3,4,3.45,4,4v12c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V4C16,3.45,15.55,3,15,3z M15,16H5v-6h10V16z M15,9 h-3.5V8h-3v1H5V4h10V9z",
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
                    fill: "none",
                    height: "24",
                }
                path {
                    opacity: ".3",
                    d: "M18,4v7h-4v-1h-4v1H6V4H18z M6,20v-7h12v7H6z",
                }
                path {
                    d: "M18,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2z M18,4v7h-4v-1h-4v1H6V4H18z M6,20 v-7h12v7H6z",
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
                g {
                    path {
                        d: "M5,5.53v3.74c0,3.26,2.13,6.27,5,7.17c2.87-0.9,5-3.91,5-7.17V5.53l-5-1.92L5,5.53z M10.14,6.47 c-0.36,0.95-0.3,2.05,0.25,3s1.47,1.55,2.48,1.71c0.27,0.04,0.38,0.37,0.21,0.58c-1.43,1.69-4.05,1.63-5.41-0.06 c-1.71-2.12-0.51-5.23,2.09-5.7C10.01,5.96,10.23,6.22,10.14,6.47z",
                        opacity: ".3",
                    }
                    path {
                        d: "M10,2L3.5,4.5v4.77c0,4.04,2.77,7.81,6.5,8.73c3.73-0.92,6.5-4.69,6.5-8.73V4.5L10,2z M15,9.27c0,3.26-2.13,6.27-5,7.17 c-2.87-0.9-5-3.91-5-7.17V5.53l5-1.92l5,1.92V9.27z",
                    }
                    path {
                        d: "M7.65,11.7c1.36,1.69,3.98,1.75,5.41,0.06c0.18-0.21,0.06-0.53-0.21-0.58c-1-0.17-1.93-0.76-2.48-1.71s-0.6-2.05-0.25-3 c0.1-0.25-0.13-0.51-0.39-0.47C7.14,6.48,5.94,9.59,7.65,11.7z",
                    }
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M6,6.39v4.7c0,4,2.55,7.7,6,8.83c3.45-1.13,6-4.82,6-8.83v-4.7l-6-2.25L6,6.39z M12.21,7.61 c-0.46,1.23-0.39,2.64,0.32,3.86c0.71,1.22,1.89,1.99,3.18,2.2c0.34,0.06,0.49,0.47,0.26,0.74c-1.84,2.17-5.21,2.1-6.96-0.07 c-2.19-2.72-0.65-6.72,2.69-7.33C12.04,6.95,12.33,7.28,12.21,7.61z",
                        opacity: ".3",
                    }
                    path {
                        d: "M12,2L4,5v6.09c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91V5L12,2z M18,11.09c0,4-2.55,7.7-6,8.83 c-3.45-1.13-6-4.82-6-8.83v-4.7l6-2.25l6,2.25V11.09z",
                    }
                    path {
                        d: "M9.01,14.33c1.75,2.17,5.12,2.24,6.96,0.07c0.23-0.27,0.08-0.68-0.26-0.74c-1.29-0.21-2.48-0.98-3.18-2.2 c-0.71-1.22-0.78-2.63-0.32-3.86c0.12-0.33-0.16-0.66-0.51-0.6C8.36,7.62,6.81,11.61,9.01,14.33z",
                    }
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
                    polygon {
                        opacity: ".3",
                        points: "3.81,16.5 9.25,16.5 9.25,14.75 4.14,14.75",
                    }
                    polygon {
                        points: "4.42,13.25 9.25,13.25 9.25,11.5 4.74,11.5",
                        opacity: ".3",
                    }
                    polygon {
                        opacity: ".3",
                        points: "15.26,11.5 10.75,11.5 10.75,13.25 15.58,13.25",
                    }
                    polygon {
                        opacity: ".3",
                        points: "10.75,14.75 10.75,16.5 16.19,16.5 15.86,14.75",
                    }
                    path {
                        d: "M16.5,10h-13L2,18h16L16.5,10z M10.75,11.5h4.51l0.33,1.75h-4.83V11.5z M9.25,16.5H3.81l0.33-1.75h5.11V16.5z M9.25,13.25 H4.42l0.33-1.75h4.51V13.25z M10.75,16.5v-1.75h5.11l0.33,1.75H10.75z",
                    }
                    rect {
                        y: "7",
                        width: "1.5",
                        x: "9.25",
                        height: "2",
                    }
                    rect {
                        height: "2",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.3314 12.1139)",
                        x: "13.71",
                        y: "5.46",
                        width: "1.5",
                    }
                    rect {
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -2.9423 5.8107)",
                        width: "2",
                        x: "4.54",
                        y: "5.71",
                        height: "1.5",
                    }
                    rect {
                        height: "1.5",
                        width: "2",
                        y: "2.05",
                        x: "3",
                    }
                    rect {
                        x: "15",
                        height: "1.5",
                        width: "2",
                        y: "2.05",
                    }
                    path {
                        d: "M10,6c2.21,0,4-1.79,4-4h-1.5c0,1.38-1.12,2.5-2.5,2.5S7.5,3.38,7.5,2H6C6,4.21,7.79,6,10,6z",
                    }
                    path {
                        d: "M12.5,2c0,1.38-1.12,2.5-2.5,2.5S7.5,3.38,7.5,2H12.5z",
                        opacity: ".3",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    polygon {
                        opacity: ".3",
                        points: "4.44,20 11,20 11,18 4.84,18",
                    }
                    polygon {
                        points: "18.36,14 13,14 13,16 18.76,16",
                        opacity: ".3",
                    }
                    polygon {
                        points: "13,18 13,20 19.56,20 19.16,18",
                        opacity: ".3",
                    }
                    polygon {
                        points: "5.24,16 11,16 11,14 5.64,14",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,12H4L2,22h20L20,12z M13,14h5.36l0.4,2H13V14z M11,20H4.44l0.4-2H11V20z M11,16H5.24l0.4-2H11V16z M13,20v-2h6.16 l0.4,2H13z",
                    }
                    rect {
                        x: "11",
                        y: "8",
                        height: "3",
                        width: "2",
                    }
                    rect {
                        y: "6.06",
                        height: "3",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.2089 14.6085)",
                        x: "16.53",
                        width: "2",
                    }
                    rect {
                        height: "2",
                        y: "6.56",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -3.448 6.7885)",
                        width: "3",
                        x: "4.97",
                    }
                    rect {
                        width: "3",
                        y: "2",
                        height: "2",
                        x: "3",
                    }
                    rect {
                        x: "18",
                        width: "3",
                        y: "2",
                        height: "2",
                    }
                    path {
                        d: "M12,7c2.76,0,5-2.24,5-5h-2c0,1.65-1.35,3-3,3S9,3.65,9,2H7C7,4.76,9.24,7,12,7z",
                    }
                    path {
                        d: "M15,2c0,1.66-1.34,3-3,3S9,3.66,9,2H15z",
                        opacity: ".3",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        height: "11",
                        opacity: ".3",
                        width: "1.5",
                        y: "4.5",
                        x: "5.5",
                    }
                    rect {
                        height: "11",
                        x: "13",
                        opacity: ".3",
                        width: "1.5",
                        y: "4.5",
                    }
                    path {
                        d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M7,15.5H5.5v-11H7V15.5z M11.5,15.5h-3v-11h3V15.5z M14.5,15.5H13v-11h1.5V15.5z",
                    }
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        y: "5",
                        opacity: ".3",
                        height: "14",
                        width: "2",
                        x: "6",
                    }
                    rect {
                        y: "5",
                        width: "2",
                        x: "16",
                        height: "14",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,19V3H4v16H2v2h20v-2H20z M8,19H6V5h2V19z M14,19h-4V5h4V19z M18,19h-2V5h2V19z",
                    }
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
                g {
                    rect {
                        height: "11",
                        x: "10.75",
                        opacity: ".2",
                        width: "1.12",
                        y: "4.5",
                    }
                    rect {
                        y: "4.5",
                        width: "1.12",
                        height: "11",
                        opacity: ".2",
                        x: "8.12",
                    }
                    rect {
                        opacity: ".2",
                        height: "11",
                        width: "1.12",
                        x: "5.5",
                        y: "4.5",
                    }
                    rect {
                        height: "11",
                        y: "4.5",
                        x: "13.38",
                        width: "1.12",
                        opacity: ".2",
                    }
                    path {
                        d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M6.62,15.5H5.5v-11h1.12V15.5z M9.25,15.5H8.12v-11h1.12V15.5z M11.88,15.5h-1.12v-11 h1.12V15.5z M14.5,15.5h-1.12v-11h1.12V15.5z",
                    }
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        y: "5",
                        width: "1.5",
                        height: "14",
                        opacity: ".2",
                        x: "9.5",
                    }
                    rect {
                        y: "5",
                        width: "1.5",
                        height: "14",
                        opacity: ".2",
                        x: "6",
                    }
                    rect {
                        width: "1.5",
                        height: "14",
                        x: "13",
                        y: "5",
                        opacity: ".2",
                    }
                    rect {
                        height: "14",
                        opacity: ".2",
                        width: "1.5",
                        x: "16.5",
                        y: "5",
                    }
                    path {
                        d: "M20,19V3H4v16H2v2h20v-2H20z M7.5,19H6V5h1.5V19z M11,19H9.5V5H11V19z M14.5,19H13V5h1.5V19z M18,19h-1.5V5H18V19z",
                    }
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
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    circle {
                        cx: "10.5",
                        r: "1",
                        opacity: ".3",
                        cy: "11",
                    }
                    path {
                        opacity: ".3",
                        d: "M14.6,3.04c-0.16-0.1-0.31-0.01-0.36,0.04L11.5,5.64V8.4l0.21,0.05l2.99-4.99 C14.83,3.25,14.66,3.08,14.6,3.04z",
                    }
                    path {
                        d: "M2.8,10.5c-0.17,0-0.3,0.14-0.3,0.3c0,0.13,0.09,0.25,0.22,0.29l3.6,1.03l1.69-1.01 c0-0.01-0.04-0.22,0.04-0.61H2.8z",
                        opacity: ".3",
                    }
                    path {
                        d: "M14.64,12.78l-2-0.5c-0.1,0.16-0.21,0.32-0.34,0.45l3.68,3.68c0.17,0.17,0.38,0.05,0.43,0 c0.1-0.1,0.12-0.24,0.05-0.36L14.64,12.78z",
                        opacity: ".3",
                    }
                    rect {
                        x: "3",
                        width: "5",
                        y: "3",
                        height: "1.5",
                    }
                    rect {
                        width: "4",
                        height: "1.5",
                        y: "6.5",
                        x: "1",
                    }
                    rect {
                        width: "4",
                        x: "3",
                        height: "1.5",
                        y: "15.5",
                    }
                    path {
                        d: "M17.77,15.32l-1.82-3.27c-0.2-0.36-0.54-0.63-0.95-0.73l-2.03-0.51c-0.03-0.38-0.14-0.73-0.32-1.04l3.32-5.54 c0.51-0.85,0.24-1.96-0.62-2.47c-0.29-0.17-1.29-0.58-2.16,0.23l-2.74,2.56C10.17,4.83,10,5.23,10,5.64v2.91 C9.63,8.62,9.3,8.78,9.01,9H2.8C1.81,9,1,9.81,1,10.8c0,0.8,0.53,1.51,1.31,1.73l3.6,1.03c0.14,0.04,0.27,0.06,0.41,0.06 c0.27,0,0.54-0.07,0.77-0.21l1.45-0.87c0.31,0.39,0.72,0.68,1.21,0.83v4.05H9.58C8.71,17.42,8,18.13,8,19h5 c0-0.87-0.71-1.58-1.58-1.58h-0.17V13.8l3.67,3.67c0.7,0.7,1.85,0.7,2.55,0C18.04,16.9,18.16,16.03,17.77,15.32z M8.01,11.11 l-1.69,1.01l-3.6-1.03C2.59,11.06,2.5,10.94,2.5,10.8c0-0.17,0.14-0.3,0.3-0.3h5.25C7.97,10.89,8.01,11.1,8.01,11.11z M10.5,12 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C11.5,11.55,11.05,12,10.5,12z M11.5,8.4V5.64l2.74-2.56 c0.05-0.05,0.2-0.13,0.36-0.04c0.06,0.04,0.23,0.2,0.1,0.42l-2.99,4.99L11.5,8.4z M16.41,16.41c-0.05,0.05-0.25,0.17-0.43,0 l-3.68-3.68c0.13-0.14,0.24-0.29,0.34-0.45l2,0.5l1.82,3.27C16.53,16.17,16.51,16.32,16.41,16.41z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    circle {
                        cy: "13",
                        cx: "13",
                        r: "1",
                        opacity: ".3",
                    }
                    path {
                        opacity: ".3",
                        d: "M3.28,13C3.13,13,3,13.13,3,13.28c0,0.12,0.08,0.24,0.2,0.27l4.51,1.29l2.33-1.4 c-0.02-0.15-0.03-0.29-0.03-0.44H3.28z",
                    }
                    path {
                        d: "M17.76,3.54c-0.15-0.09-0.29-0.01-0.34,0.04L14,6.78v3.36l0.11,0.03l3.74-6.24 C17.94,3.79,17.89,3.62,17.76,3.54z",
                        opacity: ".3",
                    }
                    path {
                        d: "M18.19,15.48l-2.78-0.69c-0.07,0.1-0.15,0.19-0.24,0.28l4.85,4.85c0.16,0.16,0.35,0.05,0.4,0 c0.09-0.09,0.11-0.23,0.05-0.33L18.19,15.48z",
                        opacity: ".3",
                    }
                    rect {
                        width: "6",
                        x: "4",
                        y: "3",
                        height: "2",
                    }
                    rect {
                        y: "7",
                        height: "2",
                        x: "1",
                        width: "5",
                    }
                    rect {
                        y: "19",
                        height: "2",
                        width: "5",
                        x: "3",
                    }
                    path {
                        d: "M22.21,18.61l-2.28-4.1c-0.27-0.48-0.73-0.83-1.26-0.97l-2.69-0.67c-0.02-0.47-0.14-0.92-0.37-1.33l3.96-6.59 c0.65-1.08,0.3-2.48-0.78-3.13c-0.36-0.22-0.77-0.32-1.17-0.32c-0.56,0-1.12,0.21-1.56,0.62l-3.43,3.21C12.23,5.7,12,6.23,12,6.78 v3.4c-0.47,0.17-0.89,0.45-1.23,0.82H3.28C2.02,11,1,12.02,1,13.28c0,1.02,0.67,1.91,1.65,2.19l4.51,1.29 c0.18,0.05,0.37,0.08,0.55,0.08c0.36,0,0.72-0.1,1.03-0.29l2.24-1.34c0.29,0.26,0.63,0.47,1.02,0.61V21c-1.1,0-2,0.9-2,2h6 c0-1.1-0.9-2-2-2v-4.28l4.61,4.61c0.45,0.45,1.03,0.67,1.61,0.67c0.58,0,1.17-0.22,1.61-0.67h0 C22.55,20.61,22.71,19.5,22.21,18.61z M7.72,14.84L3.2,13.55C3.08,13.52,3,13.4,3,13.28C3,13.13,3.13,13,3.28,13h6.73 c0,0.15,0.01,0.3,0.03,0.44L7.72,14.84z M13,14c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C14,13.55,13.55,14,13,14z M14,10.14V6.78l3.43-3.21c0.05-0.05,0.19-0.12,0.34-0.04c0.13,0.08,0.18,0.25,0.1,0.38l-3.74,6.24L14,10.14z M20.42,19.92 c-0.05,0.05-0.24,0.16-0.4,0l-4.85-4.85c0.08-0.09,0.16-0.18,0.24-0.28l2.78,0.69l2.28,4.1C20.53,19.69,20.51,19.83,20.42,19.92z",
                    }
                }
            }
        }
    }
}

