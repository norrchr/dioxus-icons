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
                    height: "20",
                    width: "20",
                    fill: "none",
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
                    height: "24",
                    width: "24",
                    fill: "none",
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
                    width: "20",
                    fill: "none",
                    height: "20",
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
                    width: "24",
                    fill: "none",
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
                path {
                    d: "M16,15.5V3H4v12.5H2V17h8.5c0,0.69,0.56,1.25,1.25,1.25S13,17.69,13,17h5v-1.5H16z M14.5,9.25h-2v-1.5h2V9.25z M11,9.25 H5.5v-1.5H11V9.25z M11,10.75v1.5H5.5v-1.5H11z M12.5,10.75h2v1.5h-2V10.75z M14.5,6.25h-2V4.5h2V6.25z M11,4.5v1.75H5.5V4.5H11z M5.5,15.5v-1.75H11v1.75H5.5z M12.5,15.5v-1.75h2v1.75H12.5z",
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
            }
            g {
                path {
                    d: "M20,19V3H4v16H2v2h11.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M18,11h-2V9h2V11z M14,11H6V9h8V11z M14,13v2H6v-2H14z M16,13h2v2h-2V13z M18,7h-2V5h2V7z M14,5v2H6V5H14z M6,19v-2h8v2H6z M16,19v-2h2v2H16z",
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
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M18,8.54V4H4v1.5h12.5v2.05C17.05,7.8,17.55,8.14,18,8.54z",
                    }
                    path {
                        d: "M7.5,7.5H2V16h5.5V7.5z M6,14.5H3.5V9H6V14.5z",
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
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M22,9.76V4H4v2h16v2.59C20.73,8.88,21.4,9.28,22,9.76z",
                    }
                    path {
                        d: "M9,9H2v11h7V9z M7,18H4v-7h3V18z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
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
                    width: "24",
                    height: "24",
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
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M7.35,10C8.6,9.16,9.57,7.68,10,5.88c0.43,1.8,1.4,3.28,2.65,4.12 c-1.25,0.84-2.22,2.32-2.65,4.12C9.57,12.32,8.6,10.84,7.35,10z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
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
                    fill: "none",
                    width: "20",
                    height: "20",
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
                    height: "24",
                    fill: "none",
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
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    polygon {
                        points: "12.25,1.75 2.25,11 10.8,11.5 6.75,17.25 7.75,18.25 17.75,9 9.2,8.5 13.25,2.75",
                    }
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
                    polygon {
                        points: "15,2 2.5,13 13,14 8,21 9,22 21.5,11 11,10 16,3",
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
                    d: "M10,2C6.13,2,3,5.13,3,9c0,2.98,1.87,5.52,4.5,6.53V18H9v-2.08C9.33,15.97,9.66,16,10,16s0.67-0.03,1-0.08V18h1.5v-2.47 C15.13,14.52,17,11.98,17,9C17,5.13,13.87,2,10,2z M11.75,11.3L9.3,13.75L8.25,12.7l1-1l-1-1l2.45-2.45l1.05,1.05l-1,1L11.75,11.3z M13,7.5H7V6h6V7.5z",
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
                path {
                    d: "M12,2c-4.96,0-9,4.04-9,9c0,3.91,2.51,7.24,6,8.47V22h2v-2.06c0.33,0.04,0.66,0.06,1,0.06s0.67-0.02,1-0.06V22h2v-2.53 c3.49-1.24,6-4.57,6-8.47C21,6.04,16.96,2,12,2z M14.25,14l-3,3l-1.5-1.5L11,14.25L9.75,13l3-3l1.5,1.5L13,12.75L14.25,14z M16,9H8 V7h8V9z",
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
                        d: "M10,3c-3.73,0-7,3-7,7c0,1.66,0.58,3.19,1.55,4.39L3,15.94L4.06,17l1.55-1.55C6.81,16.42,8.34,17,10,17 c1.79,0,3.58-0.68,4.95-2.05C16.32,13.58,17,11.79,17,10V3H10z M8.75,14l-0.5-0.5l2.15-2.75L6.5,10.5L11.25,6l0.5,0.5L9.6,9.25 l3.9,0.25L8.75,14z",
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
                    path {
                        d: "M12,3c-4.8,0-9,3.86-9,9c0,2.12,0.74,4.07,1.97,5.61L3,19.59L4.41,21l1.97-1.97C7.93,20.26,9.88,21,12,21 c2.3,0,4.61-0.88,6.36-2.64C20.12,16.61,21,14.3,21,12V3H12z M10.5,17L10,16.5l2.5-3.5l-5-0.5l6-5.5L14,7.5L11.5,11l5,0.5L10.5,17 z",
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
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M16,3.5h-3.5V2H11v1.5H9V2H7.5v1.5H4V18h12V3.5z M10,14.5c-1.1,0-2-0.88-2-1.97c0-0.87,0.35-1.11,2-3.03 c1.64,1.91,2,2.16,2,3.03C12,13.62,11.1,14.5,10,14.5z M13,8H7V6.5h6V8z",
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
                path {
                    d: "M20,4h-5V2h-2v2h-2V2H9v2H4v18h16V4z M12,18c-1.38,0-2.5-1.1-2.5-2.46c0-1.09,0.43-1.39,2.5-3.79 c2.05,2.38,2.5,2.7,2.5,3.79C14.5,16.9,13.38,18,12,18z M16,10H8V8h8V10z",
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
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M17,3H3v14h14V3z M10.5,6.3c0.66,0.09,1.26,0.34,1.76,0.73L10.5,8.79V6.3z M9.5,6.3v2.49L7.74,7.03 C8.24,6.64,8.84,6.39,9.5,6.3z M7.03,7.74L8.79,9.5H6.3C6.39,8.84,6.64,8.24,7.03,7.74z M6.3,10.5h2.49l-1.76,1.76 C6.64,11.76,6.39,11.16,6.3,10.5z M9.5,13.7c-0.66-0.09-1.26-0.34-1.76-0.73l1.76-1.76V13.7z M9.25,10c0-0.41,0.34-0.75,0.75-0.75 s0.75,0.34,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75S9.25,10.41,9.25,10z M10.5,13.7v-2.49l1.76,1.76 C11.76,13.36,11.16,13.61,10.5,13.7z M12.97,12.26l-1.76-1.76h2.49C13.61,11.16,13.36,11.76,12.97,12.26z M11.21,9.5l1.76-1.76 c0.38,0.5,0.64,1.1,0.73,1.76H11.21z",
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
                path {
                    d: "M21,3H3v18h18V3z M12.75,7.08c0.82,0.12,1.57,0.44,2.2,0.91l-2.2,2.2V7.08z M11.25,7.08v3.11l-2.2-2.2 C9.68,7.52,10.43,7.2,11.25,7.08z M7.99,9.05l2.2,2.2H7.08C7.2,10.43,7.52,9.68,7.99,9.05z M7.08,12.75h3.11l-2.2,2.2 C7.52,14.32,7.2,13.57,7.08,12.75z M11.25,16.92c-0.82-0.12-1.57-0.44-2.2-0.91l2.2-2.2V16.92z M12,13c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C13,12.55,12.55,13,12,13z M12.75,16.92v-3.11l2.2,2.2C14.32,16.48,13.57,16.8,12.75,16.92z M16.01,14.95l-2.2-2.2h3.11C16.8,13.57,16.48,14.32,16.01,14.95z M13.81,11.25l2.2-2.2c0.47,0.64,0.79,1.39,0.91,2.2H13.81z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M9.72,1.47C7.02,1.79,5,4.06,5,6.75c0,2.69,2.02,4.96,4.72,5.28l1.37,0.17l-0.57,0.85C10.35,13.03,10.18,13,10,13 c-2.21,0-4,1.79-4,4v2h8v-2c0-1.48-0.81-2.76-2.01-3.45l0.76-1.13L15,12.7V0.8L9.72,1.47z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18,0.85L11.98,1.4C8.95,1.7,6.37,4,6.04,7.03c-0.39,3.57,2.2,6.69,5.68,7.04l1.9,0.19l-0.56,0.85 c-0.88-0.19-1.83-0.18-2.85,0.25C8.21,16.21,7,18.25,7,20.41L7,23h10v-3c0-1.67-0.83-3.15-2.09-4.06l0.97-1.45L18,14.72V0.85z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M17,10.75v-1.5h-1.5V4.5H17V3H3v1.5h1.5v4.75H3v1.5h1.5v4.75H3V17h14v-1.5h-1.5v-4.75H17z M10,13 c-1.24,0-2.25-0.99-2.25-2.21c0-0.98,0.39-1.25,2.25-3.41c1.85,2.15,2.25,2.43,2.25,3.41C12.25,12.01,11.24,13,10,13z",
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M21,13v-2h-2V5h2V3H3v2h2v6H3v2h2v6H3v2h18v-2h-2v-6H21z M12,16c-1.66,0-3-1.32-3-2.95c0-1.3,0.52-1.67,3-4.55 c2.47,2.86,3,3.24,3,4.55C15,14.68,13.66,16,12,16z",
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
                    fill: "none",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M13.79,5L13,5V2.5H7V5L6.21,5C3.6,5,1.27,6.91,1.02,9.51C0.74,12.49,3.08,15,6,15v2h1.5v-2h5v2H14v-2 c2.92,0,5.26-2.51,4.98-5.49C18.73,6.91,16.4,5,13.79,5z M8.5,4h3v1h-3V4z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M16.75,6L16,6V3H8v3L7.25,6C3.97,6,1.1,8.53,1,11.82C0.9,15.21,3.62,18,7,18v3h2v-3h6v3h2v-3c3.38,0,6.1-2.79,6-6.18 C22.9,8.53,20.03,6,16.75,6z M10,5h4v1h-4V5z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M4,12.25V15c0,1.66,1.34,3,3,3h6c1.66,0,3-1.34,3-3v-2.75H4z",
                    }
                    path {
                        d: "M16,10.75V8c0-1.3-0.84-2.4-2-2.82V2H6v3.18C4.84,5.6,4,6.7,4,8v2.75H16z M7.5,3.5h5V5h-1.75V4.25h-1.5V5H7.5V3.5z",
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
            }
            g {
                g {
                    path {
                        d: "M4,15v3c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4v-3H4z",
                    }
                    path {
                        d: "M20,13v-3c0-1.86-1.28-3.41-3-3.86V2H7v4.14c-1.72,0.45-3,2-3,3.86v3H20z M9,4h6v2h-2V5h-2v1H9V4z",
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
                    width: "20",
                    height: "20",
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
                    height: "24",
                    fill: "none",
                    width: "24",
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
                    d: "M16,15.5V3H4v12.5H2V17h6.75c0,0.69,0.56,1.25,1.25,1.25s1.25-0.56,1.25-1.25H18v-1.5H16z M9.25,15.5H5.5V14h3.75V15.5z M14.5,15.5h-3.75V14h3.75V15.5z",
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
                    path {
                        d: "M20,19V3H4v16H2v2h8.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M6,19v-2h5v2H6z M13,19v-2h5v2H13z",
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
                    height: "20",
                    fill: "none",
                }
                path {
                    d: "M16,3H4v14h12V3z M13,11c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C14,10.55,13.55,11,13,11z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
                path {
                    d: "M20,2H4v20h16V2z M15.5,13.5c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5S17,11.17,17,12S16.33,13.5,15.5,13.5z",
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
                    width: "20",
                    height: "20",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
                path {
                    d: "M15,4v12H5V4H15 M4,3v14h12V3H4z M6,10v5h8v-5H6z M8.5,9V8h3v1H14V5H6v4H8.5z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
                path {
                    d: "M18,4v16H6V4H18 M4,2v20h16V2H4z M7,19h10v-6H7V19z M10,10h4v1h3V5H7v6h3V10z",
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
                    width: "20",
                    fill: "none",
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
            }
            g {
                path {
                    d: "M12,2L4,5v6.09c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91V5L12,2z M15.97,14.41c-1.84,2.17-5.21,2.1-6.96-0.07 c-2.19-2.72-0.65-6.72,2.69-7.33c0.34-0.06,0.63,0.27,0.51,0.6c-0.46,1.23-0.39,2.64,0.32,3.86c0.71,1.22,1.89,1.99,3.18,2.2 C16.05,13.72,16.2,14.14,15.97,14.41z",
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
                    fill: "none",
                    height: "20",
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
                        x: "9.25",
                        height: "2",
                        width: "1.5",
                        y: "7",
                    }
                    rect {
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.3314 12.1139)",
                        height: "2",
                        x: "13.71",
                        width: "1.5",
                        y: "5.46",
                    }
                    rect {
                        x: "4.54",
                        height: "1.5",
                        y: "5.71",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -2.9423 5.8107)",
                        width: "2",
                    }
                    rect {
                        height: "1.5",
                        x: "3",
                        y: "2.05",
                        width: "2",
                    }
                    rect {
                        x: "15",
                        height: "1.5",
                        y: "2.05",
                        width: "2",
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
                        y: "8",
                        width: "2",
                        x: "11",
                        height: "3",
                    }
                    rect {
                        height: "3",
                        x: "16.53",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.2089 14.6085)",
                        y: "6.06",
                        width: "2",
                    }
                    rect {
                        height: "2",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -3.448 6.7885)",
                        y: "6.56",
                        width: "3",
                        x: "4.97",
                    }
                    rect {
                        x: "3",
                        y: "2",
                        height: "2",
                        width: "3",
                    }
                    rect {
                        y: "2",
                        height: "2",
                        width: "3",
                        x: "18",
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
                    width: "20",
                    fill: "none",
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
                    width: "24",
                    height: "24",
                    fill: "none",
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
                    height: "20",
                    fill: "none",
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
                    fill: "none",
                    height: "24",
                    width: "24",
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
                    g {
                        rect {
                            height: "1.5",
                            x: "3",
                            y: "3",
                            width: "5",
                        }
                    }
                    g {
                        rect {
                            width: "4",
                            y: "6.5",
                            x: "1",
                            height: "1.5",
                        }
                    }
                    g {
                        rect {
                            x: "3",
                            width: "4",
                            y: "15.5",
                            height: "1.5",
                        }
                    }
                    g {
                        circle {
                            cx: "10.5",
                            cy: "11",
                            r: "1.5",
                        }
                    }
                    g {
                        path {
                            d: "M8.54,12.54C8.21,12.11,8,11.58,8,11c0-0.82,0.4-1.54,1.01-2H1v3.16l5.53,1.58L8.54,12.54z",
                        }
                    }
                    g {
                        path {
                            d: "M16.2,18.75l2.24-2.24l-2.8-5.03l-2.66-0.66c0.13,1.71-1.31,2.69-2.48,2.69c-0.26,0-0.51-0.05-0.75-0.13v4.05H9.58 C8.71,17.42,8,18.13,8,19h5c0-0.87-0.71-1.58-1.58-1.58h-0.17V13.8L16.2,18.75z",
                        }
                    }
                    g {
                        path {
                            d: "M12.65,9.74l4.27-7.11L14.2,1L10,4.93v3.62c0.16-0.03,0.33-0.05,0.5-0.05C11.42,8.5,12.21,9,12.65,9.74z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        rect {
                            y: "3",
                            height: "2",
                            width: "6",
                            x: "4",
                        }
                    }
                    g {
                        rect {
                            width: "5",
                            y: "7",
                            height: "2",
                            x: "1",
                        }
                    }
                    g {
                        rect {
                            y: "19",
                            height: "2",
                            width: "5",
                            x: "3",
                        }
                    }
                    g {
                        path {
                            d: "M15.32,12.09l5.42-9.04L17.32,1L12,5.97v4.74c0.31-0.13,0.64-0.21,1-0.21C14.06,10.5,14.96,11.16,15.32,12.09z",
                        }
                    }
                    g {
                        path {
                            d: "M10.5,13c0-0.82,0.4-1.54,1.01-2H1v4l7,2l3.44-2.06C10.87,14.48,10.5,13.79,10.5,13z",
                        }
                    }
                    g {
                        path {
                            d: "M20.17,23L23,20.17l-3.54-6.36l-3.98-1c0,0.06,0.02,0.12,0.02,0.19c0,1.38-1.12,2.5-2.5,2.5c-0.36,0-0.69-0.08-1-0.21V21 c-1.1,0-2,0.9-2,2h6c0-1.1-0.9-2-2-2v-4.17L20.17,23z",
                        }
                    }
                    g {
                        circle {
                            cx: "13",
                            cy: "13",
                            r: "1.5",
                        }
                    }
                }
            }
        }
    }
}

