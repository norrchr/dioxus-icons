use dioxus::prelude::*;
use crate::IconProps;
pub fn auto_mode_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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

pub fn auto_mode_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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

pub fn blinds_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M12.5,7.75h2v1.5h-2V7.75z M11,9.25H5.5v-1.5H11V9.25z M14.5,6.25h-2V4.5h2V6.25z M11,4.5v1.75H5.5V4.5H11z M5.5,15.5v-4.75H11v1.53c-0.3,0.23-0.5,0.57-0.5,0.97c0,0.69,0.56,1.25,1.25,1.25S13,13.94,13,13.25 c0-0.4-0.2-0.75-0.5-0.97v-1.53h2v4.75H5.5z",
                }
            }
        }
    }
}

pub fn blinds_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,19V3H4v16H2v2h20v-2H20z M16,9h2v2h-2V9z M14,11H6V9h8V11z M18,7h-2V5h2V7z M14,5v2H6V5H14z M6,19v-6h8v1.82 c-0.45,0.32-0.75,0.84-0.75,1.43c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75c0-0.59-0.3-1.12-0.75-1.43V13h2v6H6z",
                }
            }
        }
    }
}

pub fn blinds_closed_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,15.5V3H4v12.5H2V17h8.5c0,0.69,0.56,1.25,1.25,1.25S13,17.69,13,17h5v-1.5H16z M14.5,9.25h-2v-1.5h2V9.25z M11,9.25 H5.5v-1.5H11V9.25z M11,10.75v1.5H5.5v-1.5H11z M12.5,10.75h2v1.5h-2V10.75z M14.5,6.25h-2V4.5h2V6.25z M11,4.5v1.75H5.5V4.5H11z M5.5,15.5v-1.75H11v1.75H5.5z M12.5,15.5v-1.75h2v1.75H12.5z",
                }
            }
        }
    }
}

pub fn blinds_closed_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,19V3H4v16H2v2h11.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M18,11h-2V9h2V11z M14,11H6V9h8V11z M14,13v2H6v-2H14z M16,13h2v2h-2V13z M18,7h-2V5h2V7z M14,5v2H6V5H14z M6,19v-2h8v2H6z M16,19v-2h2v2H16z",
                }
            }
        }
    }
}

pub fn broadcast_on_home_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18,5.5C18,4.67,17.33,4,16.5,4H4v1.5h12.5v2.05c0.55,0.25,1.05,0.59,1.5,0.99V5.5z",
                    }
                    path {
                        d: "M6.5,7.5H3c-0.55,0-1,0.45-1,1V15c0,0.55,0.45,1,1,1h3.5c0.55,0,1-0.45,1-1V8.5C7.5,7.95,7.05,7.5,6.5,7.5z M6,14.5H3.5V9 H6V14.5z",
                    }
                    path {
                        d: "M14.5,13.85C14.79,13.67,15,13.37,15,13c0-0.55-0.45-1-1-1s-1,0.45-1,1c0,0.37,0.21,0.67,0.5,0.85V18h1V13.85z",
                    }
                    path {
                        d: "M14,8c-2.76,0-5,2.24-5,5c0,1.38,0.56,2.63,1.46,3.54l0.71-0.71C10.45,15.1,10,14.1,10,13c0-2.21,1.79-4,4-4s4,1.79,4,4 c0,1.18-0.52,2.23-1.33,2.96l0.67,0.74C18.35,15.78,19,14.47,19,13C19,10.24,16.76,8,14,8z",
                    }
                    path {
                        d: "M14,11c1.1,0,2,0.9,2,2c0,0.59-0.26,1.11-0.67,1.48L16,15.22c0.61-0.55,1-1.34,1-2.22c0-1.66-1.34-3-3-3s-3,1.34-3,3 c0,0.83,0.34,1.58,0.88,2.12l0.71-0.71C12.22,14.05,12,13.55,12,13C12,11.9,12.9,11,14,11z",
                    }
                }
            }
        }
    }
}

pub fn broadcast_on_home_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M22,6c0-1.1-0.9-2-2-2H4v2h16v2.59c0.73,0.29,1.4,0.69,2,1.17V6z",
                    }
                    path {
                        d: "M8,9H3c-0.5,0-1,0.5-1,1v9c0,0.5,0.5,1,1,1h5c0.5,0,1-0.5,1-1v-9C9,9.5,8.5,9,8,9z M7,18H4v-7h3V18z",
                    }
                    path {
                        d: "M17.75,16.97c0.3-0.23,0.5-0.57,0.5-0.97c0-0.69-0.56-1.25-1.25-1.25s-1.25,0.56-1.25,1.25c0,0.4,0.2,0.75,0.5,0.97V22 h1.5V16.97z",
                    }
                    path {
                        d: "M17,13.5c1.38,0,2.5,1.12,2.5,2.5c0,0.69-0.28,1.31-0.73,1.76l1.06,1.06C20.55,18.1,21,17.1,21,16c0-2.21-1.79-4-4-4 c-2.21,0-4,1.79-4,4c0,1.1,0.45,2.1,1.17,2.83l1.06-1.06c-0.45-0.45-0.73-1.08-0.73-1.77C14.5,14.62,15.62,13.5,17,13.5z",
                    }
                    path {
                        d: "M17,9.5c-3.59,0-6.5,2.91-6.5,6.5c0,1.79,0.73,3.42,1.9,4.6l1.06-1.06C12.56,18.63,12,17.38,12,16c0-2.76,2.24-5,5-5 s5,2.24,5,5c0,1.37-0.56,2.62-1.46,3.52l1.07,1.06c1.17-1.18,1.89-2.8,1.89-4.58C23.5,12.41,20.59,9.5,17,9.5z",
                    }
                }
            }
        }
    }
}

pub fn broadcast_on_personal_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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

pub fn broadcast_on_personal_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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

pub fn cloudy_snowing_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M4.75,14.25C4.75,14.66,5.09,15,5.5,15s0.75-0.34,0.75-0.75S5.91,13.5,5.5,13.5S4.75,13.84,4.75,14.25z M13.75,14.25 c0,0.41,0.34,0.75,0.75,0.75s0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75S13.75,13.84,13.75,14.25z M7,17.25C7,17.66,7.34,18,7.75,18 s0.75-0.34,0.75-0.75S8.16,16.5,7.75,16.5S7,16.84,7,17.25z M9.25,14.25C9.25,14.66,9.59,15,10,15s0.75-0.34,0.75-0.75 S10.41,13.5,10,13.5S9.25,13.84,9.25,14.25z M11.5,17.25c0,0.41,0.34,0.75,0.75,0.75S13,17.66,13,17.25s-0.34-0.75-0.75-0.75 S11.5,16.84,11.5,17.25z M14.44,5.76C14.09,3.63,12.23,2,10,2C8.36,2,6.87,2.88,6.09,4.3C4.07,4.57,2.5,6.3,2.5,8.37 c0,2.27,1.85,4.12,4.12,4.12h7.5c1.86,0,3.38-1.52,3.38-3.38C17.5,7.37,16.16,5.93,14.44,5.76z",
            }
        }
    }
}

pub fn cloudy_snowing_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5,18c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S5,17.45,5,18z M17,18c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1 S17,17.45,17,18z M8,22c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S8,21.45,8,22z M11,18c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1 S11,17.45,11,18z M14,22c0,0.55,0.45,1,1,1s1-0.45,1-1s-0.45-1-1-1S14,21.45,14,22z M17.5,16h-10C4.47,16,2,13.53,2,10.5 c0-2.76,2.09-5.09,4.78-5.44C7.83,3.18,9.82,2,12,2c2.97,0,5.45,2.18,5.92,5.02C20.21,7.23,22,9.16,22,11.5 C22,13.98,19.98,16,17.5,16z",
            }
        }
    }
}

pub fn curtains_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M7.35,10C8.6,9.16,9.57,7.68,10,5.88c0.43,1.8,1.4,3.28,2.65,4.12 c-1.25,0.84-2.22,2.32-2.65,4.12C9.57,12.32,8.6,10.84,7.35,10z",
                }
            }
        }
    }
}

pub fn curtains_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,19V3H4v16H2v2h20v-2H20z M8.19,12c2.04-1.35,3.5-3.94,3.76-7h0.09c0.26,3.06,1.72,5.65,3.76,7 c-2.04,1.35-3.5,3.94-3.76,7h-0.09C11.69,15.94,10.23,13.35,8.19,12z",
                }
            }
        }
    }
}

pub fn curtains_closed_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M9,4.5h2v11H9V4.5z",
                }
            }
        }
    }
}

pub fn curtains_closed_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,19V3H4v16H2v2h20v-2H20z M11,5h2v14h-2V5z",
                }
            }
        }
    }
}

pub fn electric_bolt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M11.72,2.21l-8.56,7.93c-0.32,0.3-0.12,0.83,0.31,0.86l7.33,0.5l-3.64,5.27c-0.24,0.3-0.21,0.73,0.06,1.01h0 c0.29,0.29,0.76,0.3,1.06,0.01l8.56-7.93c0.32-0.3,0.12-0.83-0.31-0.86L9.2,8.5l3.64-5.27c0.24-0.3,0.21-0.73-0.06-1.01l0,0 C12.49,1.93,12.01,1.93,11.72,2.21z",
                }
            }
        }
    }
}

pub fn electric_bolt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14.69,2.21L4.33,11.49c-0.64,0.58-0.28,1.65,0.58,1.73L13,14l-4.85,6.76c-0.22,0.31-0.19,0.74,0.08,1.01h0 c0.3,0.3,0.77,0.31,1.08,0.02l10.36-9.28c0.64-0.58,0.28-1.65-0.58-1.73L11,10l4.85-6.76c0.22-0.31,0.19-0.74-0.08-1.01l0,0 C15.47,1.93,15,1.92,14.69,2.21z",
                }
            }
        }
    }
}

pub fn electric_meter_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,2C6.13,2,3,5.13,3,9c0,2.98,1.87,5.52,4.5,6.53V18H9v-2.08C9.33,15.97,9.66,16,10,16s0.67-0.03,1-0.08V18h1.5v-2.47 C15.13,14.52,17,11.98,17,9C17,5.13,13.87,2,10,2z M11.75,11.3L9.3,13.75L8.25,12.7l1-1l-1-1l2.45-2.45l1.05,1.05l-1,1L11.75,11.3z M13,7.5H7V6h6V7.5z",
                }
            }
        }
    }
}

pub fn electric_meter_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2c-4.96,0-9,4.04-9,9c0,3.91,2.51,7.24,6,8.47V22h2v-2.06c0.33,0.04,0.66,0.06,1,0.06s0.67-0.02,1-0.06V22h2v-2.53 c3.49-1.24,6-4.57,6-8.47C21,6.04,16.96,2,12,2z M14.25,14l-3,3l-1.5-1.5L11,14.25L9.75,13l3-3l1.5,1.5L13,12.75L14.25,14z M16,9H8 V7h8V9z",
                }
            }
        }
    }
}

pub fn energy_savings_leaf_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,3c-3.73,0-7,3-7,7c0,1.66,0.58,3.19,1.55,4.39L3,15.94L4.06,17l1.55-1.55C6.81,16.42,8.34,17,10,17 c1.79,0,3.58-0.68,4.95-2.05C16.32,13.58,17,11.79,17,10l0-7L10,3z M12.84,10.3l-3.86,3.59c-0.16,0.15-0.4,0.14-0.55-0.01 c-0.14-0.14-0.15-0.37-0.03-0.52l2.02-2.58L7.49,10.6c-0.45-0.03-0.65-0.58-0.32-0.9l3.86-3.59c0.16-0.15,0.4-0.14,0.55,0.01 c0.14,0.14,0.15,0.37,0.03,0.52L9.58,9.22l2.93,0.18C12.97,9.43,13.17,9.99,12.84,10.3z",
                    }
                }
            }
        }
    }
}

pub fn energy_savings_leaf_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,3c-4.8,0-9,3.86-9,9c0,2.12,0.74,4.07,1.97,5.61L3,19.59L4.41,21l1.97-1.97C7.93,20.26,9.88,21,12,21 c2.3,0,4.61-0.88,6.36-2.64C20.12,16.61,21,14.3,21,12l0-9L12,3z M15.83,12.26l-5.16,4.63c-0.16,0.15-0.41,0.14-0.56-0.01 c-0.14-0.14-0.16-0.36-0.04-0.52l2.44-3.33l-4.05-0.4c-0.44-0.04-0.63-0.59-0.3-0.89l5.16-4.63c0.16-0.15,0.41-0.14,0.56,0.01 c0.14,0.14,0.16,0.36,0.04,0.52l-2.44,3.33l4.05,0.4C15.98,11.41,16.16,11.96,15.83,12.26z",
                    }
                }
            }
        }
    }
}

pub fn foggy_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M14.44,5.76C14.09,3.63,12.23,2,10,2C8.36,2,6.87,2.88,6.09,4.3C4.07,4.57,2.5,6.3,2.5,8.37c0,2.27,1.85,4.12,4.12,4.12h7.5 c1.86,0,3.38-1.52,3.38-3.38C17.5,7.37,16.16,5.93,14.44,5.76z M14.5,14c0.41,0,0.75,0.34,0.75,0.75s-0.34,0.75-0.75,0.75 s-0.75-0.34-0.75-0.75S14.09,14,14.5,14z M6.25,16.5C6.66,16.5,7,16.84,7,17.25S6.66,18,6.25,18S5.5,17.67,5.5,17.25 S5.84,16.5,6.25,16.5z M4.75,14.75C4.75,14.34,5.09,14,5.5,14h6.75c0.41,0,0.75,0.34,0.75,0.75s-0.34,0.75-0.75,0.75H5.5 C5.09,15.5,4.75,15.17,4.75,14.75z M7.75,17.25c0-0.41,0.34-0.75,0.75-0.75h5.25c0.41,0,0.75,0.34,0.75,0.75S14.16,18,13.75,18H8.5 C8.09,18,7.75,17.67,7.75,17.25z",
            }
        }
    }
}

pub fn foggy_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17.92,7.02C17.45,4.18,14.97,2,12,2C9.82,2,7.83,3.18,6.78,5.06C4.09,5.41,2,7.74,2,10.5C2,13.53,4.47,16,7.5,16h10 c2.48,0,4.5-2.02,4.5-4.5C22,9.16,20.21,7.23,17.92,7.02z M18,17.01c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1 S17.45,17.01,18,17.01z M7,20.01c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S6.45,20.01,7,20.01z M6,17.01h9c0.55,0,1,0.45,1,1l0,0 c0,0.55-0.45,1-1,1H6c-0.55,0-1-0.45-1-1l0,0C5,17.46,5.45,17.01,6,17.01z M10,20.01h7c0.55,0,1,0.45,1,1l0,0c0,0.55-0.45,1-1,1h-7 c-0.55,0-1-0.45-1-1l0,0C9,20.46,9.45,20.01,10,20.01z",
            }
        }
    }
}

pub fn gas_meter_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13,3.5h-0.5V2H11v1.5H9V2H7.5v1.5H7c-1.66,0-3,1.34-3,3V15c0,1.66,1.34,3,3,3h6c1.66,0,3-1.34,3-3V6.5 C16,4.84,14.66,3.5,13,3.5z M10,14.5c-1.1,0-2-0.88-2-1.97c0-0.87,0.35-1.11,2-3.03c1.64,1.91,2,2.16,2,3.03 C12,13.62,11.1,14.5,10,14.5z M13,8H7V6.5h6V8z",
                }
            }
        }
    }
}

pub fn gas_meter_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,4h-1V2h-2v2h-2V2H9v2H8C5.79,4,4,5.79,4,8v10c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4V8C20,5.79,18.21,4,16,4z M12,18 c-1.38,0-2.5-1.1-2.5-2.46c0-1.09,0.43-1.39,2.5-3.79c2.05,2.38,2.5,2.7,2.5,3.79C14.5,16.9,13.38,18,12,18z M16,10H8V8h8V10z",
                }
            }
        }
    }
}

pub fn heat_pump_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M10.5,6.3c0.66,0.09,1.26,0.34,1.76,0.73L10.5,8.79V6.3z M9.5,6.3v2.49L7.74,7.03C8.24,6.64,8.84,6.39,9.5,6.3z M7.03,7.74 L8.79,9.5H6.3C6.39,8.84,6.64,8.24,7.03,7.74z M6.3,10.5h2.49l-1.76,1.76C6.64,11.76,6.39,11.16,6.3,10.5z M9.5,13.7 c-0.66-0.09-1.26-0.34-1.76-0.73l1.76-1.76V13.7z M9.25,10c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75 c0,0.41-0.34,0.75-0.75,0.75S9.25,10.41,9.25,10z M10.5,13.7v-2.49l1.76,1.76C11.76,13.36,11.16,13.61,10.5,13.7z M12.97,12.26 l-1.76-1.76h2.49C13.61,11.16,13.36,11.76,12.97,12.26z M11.21,9.5l1.76-1.76c0.38,0.5,0.64,1.1,0.73,1.76H11.21z",
                }
            }
        }
    }
}

pub fn heat_pump_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12.75,7.08 c0.82,0.12,1.57,0.44,2.2,0.91l-2.2,2.2V7.08z M11.25,7.08v3.11l-2.2-2.2C9.68,7.52,10.43,7.2,11.25,7.08z M7.99,9.05l2.2,2.2H7.08 C7.2,10.43,7.52,9.68,7.99,9.05z M7.08,12.75h3.11l-2.2,2.2C7.52,14.32,7.2,13.57,7.08,12.75z M11.25,16.92 c-0.82-0.12-1.57-0.44-2.2-0.91l2.2-2.2V16.92z M12,13c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C13,12.55,12.55,13,12,13z M12.75,16.92v-3.11l2.2,2.2C14.32,16.48,13.57,16.8,12.75,16.92z M16.01,14.95l-2.2-2.2h3.11C16.8,13.57,16.48,14.32,16.01,14.95z M13.81,11.25l2.2-2.2c0.47,0.64,0.79,1.39,0.91,2.2H13.81z",
                }
            }
        }
    }
}

pub fn mode_fan_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.46,13.34c0.17,0.04,0.35,0.06,0.54,0.06c0.98,0,2-0.76,2-2.08C18,8.52,16.49,7,14.8,7c-0.83,0-1.29,0.22-3.01,0.91 c-0.14-0.12-0.28-0.22-0.44-0.31c0.22-0.99,0.64-1.47,1.1-1.78C13.09,5.39,13.4,4.75,13.4,4c0-0.98-0.76-2-2.08-2 C8.69,2,7.2,3.34,7.03,4.91L15.46,13.34z",
                    }
                    path {
                        d: "M2.93,2.93L1.87,3.99l2.67,2.67C4.37,6.62,4.19,6.6,4,6.6c-0.98,0-2,0.76-2,2.08C2,11.48,3.51,13,5.2,13 c0.83,0,1.29-0.22,3.01-0.91c0.14,0.12,0.28,0.22,0.44,0.31c-0.22,0.99-0.64,1.47-1.1,1.78C6.91,14.61,6.6,15.25,6.6,16 c0,0.98,0.76,2,2.08,2c2.63,0,4.12-1.34,4.29-2.91l3.04,3.04l1.06-1.06L2.93,2.93z",
                    }
                }
            }
        }
    }
}

pub fn mode_fan_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M16.34,8.36l-2.29,0.82c-0.18-0.13-0.38-0.25-0.58-0.34c0.17-0.83,0.63-1.58,1.36-2.06C16.85,5.44,16.18,2,13.39,2 c-3.08,0-4.9,1.47-5.3,3.26L18.73,15.9c1.5,0.39,3.27-0.51,3.27-2.51C22,9,18.99,7.16,16.34,8.36z",
                        }
                    }
                    g {
                        path {
                            d: "M2.81,2.81L1.39,4.22L5.27,8.1C3.77,7.7,2,8.61,2,10.61c0,4.4,3.01,6.24,5.66,5.03l2.29-0.82 c0.18,0.13,0.38,0.25,0.58,0.34c-0.17,0.83-0.63,1.58-1.36,2.06C7.15,18.56,7.82,22,10.61,22c3.08,0,4.9-1.47,5.3-3.26l3.87,3.87 l1.41-1.41L2.81,2.81z",
                        }
                    }
                }
            }
        }
    }
}

pub fn nest_cam_wired_stand_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M13.31,1.01L9.72,1.47C7.02,1.79,5,4.06,5,6.75c0,2.69,2.02,4.96,4.72,5.28l1.37,0.17l-0.57,0.85 C10.35,13.03,10.18,13,10,13c-2.21,0-4,1.79-4,4v1.5C6,18.78,6.22,19,6.5,19h7c0.28,0,0.5-0.22,0.5-0.5V17 c0-1.48-0.81-2.76-2.01-3.45l0.76-1.13l0.56,0.07c0.9,0.11,1.69-0.58,1.69-1.48V2.49C15,1.59,14.21,0.9,13.31,1.01z",
                    }
                }
            }
        }
    }
}

pub fn nest_cam_wired_stand_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.83,1.01l-4.11,0.42C8.47,1.75,6,4.48,6,7.75s2.47,6,5.72,6.33l1.9,0.19l-0.56,0.85C12.71,15.04,12.36,15,12,15 c-2.76,0-5,2.24-5,5v2c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v-2c0-1.67-0.83-3.15-2.09-4.06l0.97-1.45 C17.02,14.56,18,13.66,18,12.5V3C18,1.83,17,0.91,15.83,1.01z",
                }
            }
        }
    }
}

pub fn oil_barrel_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.25,10.75c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75H15.5V4.5h0.75C16.66,4.5,17,4.16,17,3.75S16.66,3,16.25,3 H3.75C3.34,3,3,3.34,3,3.75S3.34,4.5,3.75,4.5H4.5v4.75H3.75C3.34,9.25,3,9.59,3,10s0.34,0.75,0.75,0.75H4.5v4.75H3.75 C3.34,15.5,3,15.84,3,16.25S3.34,17,3.75,17h12.5c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75H15.5v-4.75H16.25z M10,13 c-1.24,0-2.25-0.99-2.25-2.21c0-0.98,0.39-1.25,2.25-3.41c1.85,2.15,2.25,2.43,2.25,3.41C12.25,12.01,11.24,13,10,13z",
                }
            }
        }
    }
}

pub fn oil_barrel_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,13c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1V5h1c0.55,0,1-0.45,1-1s-0.45-1-1-1H4C3.45,3,3,3.45,3,4s0.45,1,1,1h1v6H4 c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v6H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h16c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1v-6H20z M12,16 c-1.66,0-3-1.32-3-2.95c0-1.3,0.52-1.67,3-4.55c2.47,2.86,3,3.24,3,4.55C15,14.68,13.66,16,12,16z",
                }
            }
        }
    }
}

pub fn propane_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,5h-1V4c0-0.83-0.67-1.5-1.5-1.5h-3C7.67,2.5,7,3.17,7,4v1H6c-2.76,0-5,2.24-5,5s2.24,5,5,5v2h1.5v-2h5v2H14v-2 c2.76,0,5-2.24,5-5S16.76,5,14,5z M8.5,4h3v1h-3V4z",
                }
            }
        }
    }
}

pub fn propane_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,6h-1V5c0-1.1-0.9-2-2-2h-4C8.9,3,8,3.9,8,5v1H7c-3.31,0-6,2.69-6,6s2.69,6,6,6v3h2v-3h6v3h2v-3c3.31,0,6-2.69,6-6 S20.31,6,17,6z M10,5h4v1h-4V5z",
                }
            }
        }
    }
}

pub fn propane_tank_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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

pub fn propane_tank_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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

pub fn roller_shades_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M5.5,15.5v-4.75h3.75v1.53c-0.3,0.23-0.5,0.57-0.5,0.97c0,0.69,0.56,1.25,1.25,1.25 s1.25-0.56,1.25-1.25c0-0.4-0.2-0.75-0.5-0.97v-1.53h3.75v4.75H5.5z",
                }
            }
        }
    }
}

pub fn roller_shades_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,19V3H4v16H2v2h20v-2H20z M6,19v-6h5v1.8c-0.4,0.3-0.8,0.8-0.8,1.4c0,1,0.8,1.8,1.8,1.8s1.8-0.8,1.8-1.8 c0-0.6-0.3-1.1-0.8-1.4V13h5v6H6z",
                }
            }
        }
    }
}

pub fn roller_shades_closed_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,15.5V3H4v12.5H2V17h6.75c0,0.69,0.56,1.25,1.25,1.25s1.25-0.56,1.25-1.25H18v-1.5H16z M9.25,15.5H5.5V14h3.75V15.5z M14.5,15.5h-3.75V14h3.75V15.5z",
                }
            }
        }
    }
}

pub fn roller_shades_closed_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,19V3H4v16H2v2h8.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M6,19v-2h5v2H6z M13,19v-2h5v2H13z",
                    }
                }
            }
        }
    }
}

pub fn sensor_door_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15,3H5C4.45,3,4,3.45,4,4v13h12V4C16,3.45,15.55,3,15,3z M13,11c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C14,10.55,13.55,11,13,11z",
                }
            }
        }
    }
}

pub fn sensor_door_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M18,2H6C4.9,2,4,2.9,4,4v18h16V4C20,2.9,19.1,2,18,2z M15.5,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5 S17,11.17,17,12S16.33,13.5,15.5,13.5z",
                }
            }
        }
    }
}

pub fn sensor_occupied_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,9c1.38,0,2.5-1.12,2.5-2.5C12.5,5.12,11.38,4,10,4S7.5,5.12,7.5,6.5C7.5,7.88,8.62,9,10,9z",
                    }
                    path {
                        d: "M10,10c-1.53,0-2.96,0.42-4.2,1.14C5.3,11.44,5,11.99,5,12.57V14h10v-1.43c0-0.58-0.3-1.14-0.8-1.43 C12.96,10.42,11.53,10,10,10z",
                    }
                }
            }
        }
    }
}

pub fn sensor_occupied_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,11c1.66,0,3-1.34,3-3s-1.34-3-3-3S9,6.34,9,8S10.34,11,12,11z",
                    }
                    path {
                        d: "M12,12c-1.84,0-3.56,0.5-5.03,1.37C6.36,13.72,6,14.39,6,15.09V17h12v-1.91c0-0.7-0.36-1.36-0.97-1.72 C15.56,12.5,13.84,12,12,12z",
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

pub fn sensor_window_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15,4v12H5V4H15 M15,3H5C4.45,3,4,3.45,4,4v12c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V4C16,3.45,15.55,3,15,3L15,3z M6,10 v5h8v-5H6z M8.5,9V8h3v1H14V5H6v4H8.5z",
                }
            }
        }
    }
}

pub fn sensor_window_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M18,4v16H6V4H18 M18,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2L18,2z M7,19h10v-6H7 V19z M10,10h4v1h3V5H7v6h3V10z",
                }
            }
        }
    }
}

pub fn shelves_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.5 1v1.5h-11V1H3v18h1.5v-1.5h11V19H17V1h-1.5zm0 3v5.25H11v-3.5H6v3.5H4.5V4h11zM14 16v-3.5H9V16H4.5v-5.25h11V16H14z",
                }
            }
        }
    }
}

pub fn shelves_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19 1v2H5V1H3v22h2v-2h14v2h2V1h-2zm0 4v6h-6V7H7v4H5V5h14zm-2 14v-4h-6v4H5v-6h14v6h-2z",
                }
            }
        }
    }
}

pub fn shield_moon_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,2L3.5,4.5v4.77c0,4.04,2.77,7.81,6.5,8.73c3.73-0.92,6.5-4.69,6.5-8.73V4.5L10,2z M13.06,11.76 c-1.43,1.69-4.05,1.63-5.41-0.06c-1.71-2.12-0.51-5.23,2.09-5.7c0.27-0.05,0.49,0.21,0.39,0.47c-0.36,0.95-0.3,2.05,0.25,3 s1.47,1.55,2.48,1.71C13.12,11.23,13.24,11.55,13.06,11.76z",
                }
            }
        }
    }
}

pub fn shield_moon_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2L4,5v6.09c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91V5L12,2z M15.97,14.41c-1.84,2.17-5.21,2.1-6.96-0.07 c-2.19-2.72-0.65-6.72,2.69-7.33c0.34-0.06,0.63,0.27,0.51,0.6c-0.46,1.23-0.39,2.64,0.32,3.86c0.71,1.22,1.89,1.99,3.18,2.2 C16.05,13.72,16.2,14.14,15.97,14.41z",
                }
            }
        }
    }
}

pub fn snowing_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M5.5 10.5c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm-1-5c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1zm9 6c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1zm0-6c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1zm-6.75 9c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1zm0-6c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1zm2.25 3c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1zm0-6c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1zm2.25 9c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1zm0-6c0 .55.45 1 1 1s1-.45 1-1-.45-1-1-1-1 .45-1 1z",
                }
            }
        }
    }
}

pub fn snowing_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6 12.75a1.25 1.25 0 110 2.5 1.25 1.25 0 010-2.5zM4.75 6a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm12 8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm0-8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm-9 12a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm0-8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm3 4a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm0-8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm3 12a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm0-8a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0z",
                }
            }
        }
    }
}

pub fn solar_power_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    polygon {
                        points: "10.75,10 10.75,13.25 17.11,13.25 16.5,10",
                    }
                    polygon {
                        points: "2.89,13.25 9.25,13.25 9.25,10 3.5,10",
                    }
                    polygon {
                        points: "2,18 9.25,18 9.25,14.75 2.61,14.75",
                    }
                    polygon {
                        points: "17.39,14.75 10.75,14.75 10.75,18 18,18",
                    }
                    rect {
                        y: "7",
                        height: "2",
                        width: "1.5",
                        x: "9.25",
                    }
                    rect {
                        width: "1.5",
                        height: "2",
                        x: "13.71",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.3314 12.1139)",
                        y: "5.46",
                    }
                    rect {
                        x: "4.54",
                        y: "5.71",
                        width: "2",
                        height: "1.5",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -2.9423 5.8107)",
                    }
                    rect {
                        height: "1.5",
                        width: "2",
                        x: "3",
                        y: "2.05",
                    }
                    rect {
                        y: "2.05",
                        height: "1.5",
                        width: "2",
                        x: "15",
                    }
                    path {
                        d: "M10,6c2.21,0,4-1.79,4-4H6C6,4.21,7.79,6,10,6z",
                    }
                }
            }
        }
    }
}

pub fn solar_power_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "3.33,16 11,16 11,13 4,13",
                    }
                    polygon {
                        points: "13,16 20.67,16 20,13 13,13",
                    }
                    polygon {
                        points: "21.11,18 13,18 13,22 22,22",
                    }
                    polygon {
                        points: "2,22 11,22 11,18 2.89,18",
                    }
                    rect {
                        width: "2",
                        height: "3",
                        x: "11",
                        y: "8",
                    }
                    rect {
                        height: "3",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.2089 14.6085)",
                        y: "6.06",
                        x: "16.53",
                        width: "2",
                    }
                    rect {
                        height: "2",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -3.448 6.7885)",
                        width: "3",
                        y: "6.56",
                        x: "4.97",
                    }
                    rect {
                        height: "2",
                        width: "3",
                        x: "3",
                        y: "2",
                    }
                    rect {
                        height: "2",
                        width: "3",
                        x: "18",
                        y: "2",
                    }
                    path {
                        d: "M12,7c2.76,0,5-2.24,5-5H7C7,4.76,9.24,7,12,7z",
                    }
                }
            }
        }
    }
}

pub fn sunny_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M1,10c0-0.41,0.34-0.75,0.75-0.75h1.5C3.66,9.25,4,9.59,4,10s-0.34,0.75-0.75,0.75h-1.5C1.34,10.75,1,10.41,1,10z M16.75,10.75h1.5c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75h-1.5C16.34,9.25,16,9.59,16,10S16.34,10.75,16.75,10.75z M9.25,1.75v1.5C9.25,3.66,9.59,4,10,4s0.75-0.34,0.75-0.75v-1.5C10.75,1.34,10.41,1,10,1S9.25,1.34,9.25,1.75z M9.25,16.75v1.5 C9.25,18.66,9.59,19,10,19s0.75-0.34,0.75-0.75v-1.5c0-0.41-0.34-0.75-0.75-0.75S9.25,16.34,9.25,16.75z M3.64,4.7L4.7,5.76 c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06L4.7,3.64c-0.29-0.29-0.77-0.29-1.06,0S3.34,4.41,3.64,4.7z M14.24,15.3 l1.06,1.06c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06l-1.06-1.06c-0.29-0.29-0.77-0.29-1.06,0 C13.95,14.53,13.95,15.01,14.24,15.3z M15.3,3.64L14.24,4.7c-0.29,0.29-0.29,0.77,0,1.06c0.29,0.29,0.77,0.29,1.06,0l1.06-1.06 c0.29-0.29,0.29-0.77,0-1.06C16.07,3.34,15.59,3.34,15.3,3.64z M4.7,14.24L3.64,15.3c-0.29,0.29-0.29,0.77,0,1.06 c0.29,0.29,0.77,0.29,1.06,0l1.06-1.06c0.29-0.29,0.29-0.77,0-1.06C5.47,13.95,4.99,13.95,4.7,14.24z M15,10c0,2.76-2.24,5-5,5 s-5-2.24-5-5s2.24-5,5-5S15,7.24,15,10z",
            }
        }
    }
}

pub fn sunny_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11,4V2c0-0.55,0.45-1,1-1s1,0.45,1,1v2c0,0.55-0.45,1-1,1S11,4.55,11,4z M18.36,7.05l1.41-1.42c0.39-0.39,0.39-1.02,0-1.41 c-0.39-0.39-1.02-0.39-1.41,0l-1.41,1.42c-0.39,0.39-0.39,1.02,0,1.41C17.34,7.44,17.97,7.44,18.36,7.05z M22,11h-2 c-0.55,0-1,0.45-1,1s0.45,1,1,1h2c0.55,0,1-0.45,1-1S22.55,11,22,11z M12,19c-0.55,0-1,0.45-1,1v2c0,0.55,0.45,1,1,1s1-0.45,1-1v-2 C13,19.45,12.55,19,12,19z M5.64,7.05L4.22,5.64c-0.39-0.39-0.39-1.03,0-1.41s1.03-0.39,1.41,0l1.41,1.41 c0.39,0.39,0.39,1.03,0,1.41S6.02,7.44,5.64,7.05z M16.95,16.95c-0.39,0.39-0.39,1.03,0,1.41l1.41,1.41c0.39,0.39,1.03,0.39,1.41,0 c0.39-0.39,0.39-1.03,0-1.41l-1.41-1.41C17.98,16.56,17.34,16.56,16.95,16.95z M2,13h2c0.55,0,1-0.45,1-1s-0.45-1-1-1H2 c-0.55,0-1,0.45-1,1S1.45,13,2,13z M5.64,19.78l1.41-1.41c0.39-0.39,0.39-1.03,0-1.41s-1.03-0.39-1.41,0l-1.41,1.41 c-0.39,0.39-0.39,1.03,0,1.41C4.61,20.17,5.25,20.17,5.64,19.78z M12,6c-3.31,0-6,2.69-6,6s2.69,6,6,6s6-2.69,6-6S15.31,6,12,6z",
            }
        }
    }
}

pub fn sunny_snowing_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                circle {
                    cy: "14",
                    r: "1",
                    cx: "5",
                }
                circle {
                    cy: "14",
                    cx: "15",
                    r: "1",
                }
                circle {
                    cy: "17",
                    r: "1",
                    cx: "7.5",
                }
                circle {
                    cx: "10",
                    cy: "14",
                    r: "1",
                }
                circle {
                    cx: "12.5",
                    r: "1",
                    cy: "17",
                }
                path {
                    d: "M10 6c-2.21 0-4 1.79-4 4v1h8v-1c0-2.21-1.79-4-4-4z",
                }
                rect {
                    y: "1.5",
                    height: "3",
                    x: "9.25",
                    width: "1.5",
                }
                rect {
                    x: "16.25",
                    transform: "matrix(-1.836970e-16 1 -1 -1.836970e-16 27 -7)",
                    y: "8.5",
                    width: "1.5",
                    height: "3",
                }
                rect {
                    transform: "matrix(0.7071 0.7071 -0.7071 0.7071 7.9497 -9.0919)",
                    height: "3",
                    x: "14.2",
                    width: "1.5",
                    y: "3.55",
                }
                rect {
                    transform: "matrix(-0.7071 0.7071 -0.7071 -0.7071 12.1924 5.0503)",
                    height: "3",
                    y: "3.55",
                    width: "1.5",
                    x: "4.3",
                }
                rect {
                    x: "1.5",
                    y: "9.25",
                    height: "1.5",
                    width: "3",
                }
            }
        }
    }
}

pub fn sunny_snowing_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13 5h-2V1h2v4zM1 11h4v2H1v-2zm18 2v-2h4v2h-4zm-1.34-5.24-1.41-1.41 2.83-2.83 1.41 1.41-2.83 2.83zM3.51 4.93l1.41-1.41 2.83 2.83-1.41 1.41-2.83-2.83zM4.75 17a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm12 0a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm-9 4a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm3-4a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zm3 4a1.25 1.25 0 102.5 0 1.25 1.25 0 00-2.5 0zM17 13v-1c0-2.76-2.24-5-5-5s-5 2.24-5 5v1h10z",
                }
            }
        }
    }
}

pub fn vertical_shades_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M8.5,4.5h3v11h-3V4.5z",
                }
            }
        }
    }
}

pub fn vertical_shades_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,19V3H4v16H2v2h20v-2H20z M10,19V5h4v14H10z",
                }
            }
        }
    }
}

pub fn vertical_shades_closed_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M10.75,4.5h1.12v11h-1.12V4.5z M9.25,15.5H8.12v-11h1.12V15.5z M5.5,4.5h1.12v11H5.5 V4.5z M13.38,15.5v-11h1.12v11H13.38z",
                }
            }
        }
    }
}

pub fn vertical_shades_closed_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,19V3H4v16H2v2h20v-2H20z M13,5h1.5v14H13V5z M11,19H9.5V5H11V19z M6,5h1.5v14H6V5z M16.5,19V5H18v14H16.5z",
                }
            }
        }
    }
}

pub fn wind_power_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        rect {
                            height: "1.5",
                            width: "5",
                            y: "3",
                            x: "3",
                        }
                    }
                    g {
                        rect {
                            width: "4",
                            x: "1",
                            height: "1.5",
                            y: "6.5",
                        }
                    }
                    g {
                        rect {
                            width: "4",
                            y: "15.5",
                            x: "3",
                            height: "1.5",
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

pub fn wind_power_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        x: "4",
                        width: "6",
                        y: "3",
                        height: "2",
                    }
                    rect {
                        width: "5",
                        y: "7",
                        x: "1",
                        height: "2",
                    }
                    rect {
                        x: "3",
                        width: "5",
                        height: "2",
                        y: "19",
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

