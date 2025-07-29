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
                    x: "0",
                    height: "20",
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
                    fill: "none",
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
                    x: "0",
                    fill: "none",
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
                    height: "24",
                    fill: "none",
                    width: "24",
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
                    x: "0",
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
                    fill: "none",
                    height: "24",
                    width: "24",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                    x: "0",
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
                    width: "20",
                    fill: "none",
                    x: "0",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M3.5,15.5V8.25L8,4.88l3.54,2.66c0.57-0.26,1.19-0.43,1.84-0.5L8,3L2,7.5V17h7.54c-0.4-0.45-0.73-0.95-0.99-1.5H3.5z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
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
                    x: "0",
                }
            }
            g {
                path {
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M14.5,9.2c-1.66-0.33-2.98-2.26-3.21-4.7h3.21V9.2z M12.65,10 c-1.25,0.84-2.22,2.32-2.65,4.12c-0.43-1.8-1.4-3.28-2.65-4.12C8.6,9.16,9.57,7.68,10,5.88C10.43,7.68,11.4,9.16,12.65,10z M8.71,4.5C8.48,6.94,7.16,8.87,5.5,9.2V4.5H8.71z M5.5,10.8c1.66,0.33,2.98,2.26,3.21,4.7H5.5V10.8z M11.29,15.5 c0.23-2.44,1.55-4.37,3.21-4.7v4.7H11.29z",
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
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M20,19V3H4v16H2v2h20v-2H20z M18,10.86c-2.05-0.58-3.64-2.93-3.94-5.86H18V10.86z M15.81,12c-2.04,1.35-3.5,3.94-3.76,7 h-0.09c-0.26-3.06-1.72-5.65-3.76-7c2.04-1.35,3.5-3.94,3.76-7h0.09C12.31,8.06,13.77,10.65,15.81,12z M9.94,5 C9.64,7.93,8.05,10.27,6,10.86V5H9.94z M6,13.14c2.05,0.58,3.64,2.93,3.94,5.86H6V13.14z M14.06,19c0.3-2.93,1.89-5.27,3.94-5.86 V19H14.06z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                    x: "0",
                }
            }
            g {
                path {
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M11,4.5v11H9v-11H11z M5.5,4.5h2v11h-2V4.5z M12.5,15.5v-11h2v11H12.5z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M20,19V3H4v16H2v2h20v-2H20z M13,5v14h-2V5H13z M6,5h3v14H6V5z M15,19V5h3v14H15z",
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
                    width: "20",
                    x: "0",
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
                    x: "0",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M17,9c0-3.87-3.13-7-7-7C6.13,2,3,5.13,3,9c0,2.98,1.87,5.52,4.5,6.53V18H9v-2.08C9.33,15.97,9.66,16,10,16 s0.67-0.03,1-0.08V18h1.5v-2.47C15.13,14.52,17,11.98,17,9z M10,14.5c-3.03,0-5.5-2.47-5.5-5.5S6.97,3.5,10,3.5s5.5,2.47,5.5,5.5 S13.03,14.5,10,14.5z",
                    }
                    rect {
                        width: "6",
                        x: "7",
                        height: "1.5",
                        y: "6",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M21,11c0-4.97-4.03-9-9-9s-9,4.03-9,9c0,3.92,2.51,7.24,6,8.48V22h2v-2.06c0.33,0.04,0.66,0.06,1,0.06s0.67-0.02,1-0.06 V22h2v-2.52C18.49,18.24,21,14.92,21,11z M12,18c-3.86,0-7-3.14-7-7s3.14-7,7-7s7,3.14,7,7S15.86,18,12,18z",
                    }
                    rect {
                        y: "7",
                        width: "8",
                        x: "8",
                        height: "2",
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
                    x: "0",
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M10,3C10,3,10,3,10,3c-3.73,0-7,3-7,7c0,1.66,0.58,3.19,1.55,4.39L3,15.94L4.06,17l1.55-1.55C6.81,16.42,8.34,17,10,17 c1.79,0,3.58-0.68,4.95-2.05C16.32,13.58,17,11.79,17,10l0-7L10,3z M15.5,10c0,1.47-0.57,2.85-1.61,3.89 c-1.04,1.04-2.42,1.61-3.89,1.61c-3.03,0-5.5-2.47-5.5-5.5c0-1.5,0.58-2.89,1.65-3.92C7.19,5.06,8.56,4.5,10,4.5l5.5,0L15.5,10z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,3C12,3,12,3,12,3c-4.8,0-9,3.86-9,9c0,2.12,0.74,4.07,1.97,5.61L3,19.59L4.41,21l1.97-1.97C7.93,20.26,9.88,21,12,21 c2.3,0,4.61-0.88,6.36-2.64C20.12,16.61,21,14.3,21,12l0-9L12,3z M19,12c0,1.87-0.73,3.63-2.05,4.95C15.63,18.27,13.87,19,12,19 c-3.86,0-7-3.14-7-7c0-1.9,0.74-3.68,2.1-4.99C8.42,5.71,10.16,5,12,5l7,0L19,12z",
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
                    width: "20",
                    x: "0",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M13,3.5h-0.5V2H11v1.5H9V2H7.5v1.5H7c-1.66,0-3,1.34-3,3V15c0,1.66,1.34,3,3,3h6c1.66,0,3-1.34,3-3V6.5 C16,4.84,14.66,3.5,13,3.5z M14.5,15c0,0.83-0.67,1.5-1.5,1.5H7c-0.83,0-1.5-0.67-1.5-1.5V6.5C5.5,5.67,6.17,5,7,5h6 c0.83,0,1.5,0.67,1.5,1.5V15z",
                    }
                    path {
                        d: "M8,12.53c0,1.09,0.9,1.97,2,1.97s2-0.88,2-1.97c0-0.87-0.36-1.13-2-3.03C8.35,11.42,8,11.66,8,12.53z",
                    }
                    rect {
                        width: "6",
                        x: "7",
                        height: "1.5",
                        y: "6.5",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M16,4h-1V2h-2v2h-2V2H9v2H8C5.79,4,4,5.79,4,8v10c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4V8C20,5.79,18.21,4,16,4z M18,18 c0,1.1-0.9,2-2,2H8c-1.1,0-2-0.9-2-2V8c0-1.1,0.9-2,2-2h8c1.1,0,2,0.9,2,2V18z",
                    }
                    path {
                        d: "M9.5,15.54C9.5,16.9,10.62,18,12,18s2.5-1.1,2.5-2.46c0-1.09-0.45-1.41-2.5-3.79C9.93,14.15,9.5,14.46,9.5,15.54z",
                    }
                    rect {
                        y: "8",
                        height: "2",
                        width: "8",
                        x: "8",
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
                    width: "20",
                    height: "20",
                    x: "0",
                    fill: "none",
                }
            }
            g {
                g {
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
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
                    fill: "none",
                    height: "20",
                    width: "20",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M14.8,7c-0.83,0-1.29,0.22-3.01,0.91c-0.14-0.12-0.28-0.22-0.44-0.31c0.22-0.99,0.64-1.47,1.1-1.78 C13.09,5.39,13.4,4.75,13.4,4c0-0.98-0.76-2-2.08-2C8.69,2,7.2,3.34,7.03,4.91l2.13,2.13C8.66,5.8,8.6,5.56,8.6,5.2 c0-0.94,1.12-1.6,2.72-1.6c0.46,0,0.48,0.33,0.48,0.4c0,0.28-0.09,0.39-0.25,0.49c-0.91,0.61-1.49,1.52-1.77,2.77L9.73,7.61 l5.73,5.73c0.17,0.04,0.35,0.06,0.54,0.06c0.98,0,2-0.76,2-2.08C18,8.52,16.49,7,14.8,7z M16,11.8c-0.28,0-0.39-0.09-0.49-0.25 c-0.61-0.91-1.52-1.49-2.77-1.77c-0.01-0.17-0.04-0.34-0.09-0.5c1.52-0.62,1.76-0.68,2.14-0.68c0.94,0,1.6,1.12,1.6,2.72 C16.4,11.78,16.07,11.8,16,11.8z",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18,8c-1.06,0-1.64,0.29-3.91,1.19c-0.19-0.14-0.4-0.27-0.62-0.37c0.25-1.03,0.61-1.53,1.33-2.04 C15.61,6.21,16,5.44,16,4.5C16,3.28,15.05,2,13.4,2c-3.08,0-4.92,1.47-5.32,3.26l2.33,2.33C10.07,6.69,10,6.38,10,6 c0-1.18,1.4-2,3.4-2C13.97,4,14,4.42,14,4.5c0,0.27-0.05,0.43-0.35,0.65c-1.27,0.9-1.83,1.91-2.16,3.39l-0.02,0.1l7.25,7.25 C18.96,15.95,19.22,16,19.5,16c1.22,0,2.5-0.95,2.5-2.6C22,9.91,20.11,8,18,8z M19.5,14c-0.27,0-0.43-0.05-0.65-0.35 c-0.9-1.27-1.91-1.83-3.39-2.16c-0.03-0.22-0.08-0.42-0.15-0.62C17.11,10.12,17.49,10,18,10c1.18,0,2,1.4,2,3.4 C20,13.97,19.58,14,19.5,14z",
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
                    height: "20",
                    width: "20",
                    x: "0",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M13.5,1c-0.06,0-3.79,0.47-3.79,0.47C7.02,1.79,5,4.06,5,6.75s2.02,4.96,4.72,5.28l1.37,0.17l-0.57,0.85 C10.35,13.03,10.18,13,10,13c-2.21,0-4,1.79-4,4v1.5C6,18.78,6.22,19,6.5,19h7c0.28,0,0.5-0.22,0.5-0.5V17 c0-1.48-0.81-2.76-2.01-3.45l0.76-1.13c0.61,0.08,0.63,0.08,0.76,0.08c0.81,0,1.5-0.66,1.5-1.49V2.49C15,1.66,14.32,1,13.5,1z M12.5,17.5h-5V17c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5V17.5z M13.5,11l-3.6-0.46C7.96,10.31,6.5,8.68,6.5,6.75 s1.46-3.56,3.41-3.79L13.5,2.5l0,0L13.5,11z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M16,1c-0.15,0,0.11-0.02-4.28,0.42C8.47,1.75,6,4.48,6,7.75s2.47,6,5.72,6.33l1.9,0.19l-0.56,0.85 C12.71,15.04,12.36,15,12,15c-2.76,0-5,2.24-5,5v2c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v-2c0-1.67-0.83-3.15-2.09-4.06l0.97-1.45 c0.04,0,0.09,0.01,0.13,0.01c1.09,0,2-0.89,2-2V3C18,1.89,17.09,1,16,1z M15,21H9v-1c0-1.65,1.35-3,3-3c1.65,0,3,1.35,3,3V21z M8,7.75c0-2.25,1.69-4.11,3.92-4.34L16,3h0l0.03,9.5l-4.11-0.42C9.69,11.86,8,10,8,7.75z",
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
                    x: "0",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M7.75,10.79C7.75,12.01,8.76,13,10,13c1.24,0,2.25-0.99,2.25-2.21c0-0.98-0.4-1.27-2.25-3.41 C8.14,9.53,7.75,9.81,7.75,10.79z",
                    }
                    path {
                        d: "M16.25,10.75c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75H15.5V4.5h0.75C16.66,4.5,17,4.16,17,3.75S16.66,3,16.25,3 H3.75C3.34,3,3,3.34,3,3.75S3.34,4.5,3.75,4.5H4.5v4.75H3.75C3.34,9.25,3,9.59,3,10s0.34,0.75,0.75,0.75H4.5v4.75H3.75 C3.34,15.5,3,15.84,3,16.25S3.34,17,3.75,17h12.5c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75H15.5v-4.75H16.25z M14,15.5H6 v-4.75c0.41,0,0.75-0.34,0.75-0.75S6.41,9.25,6,9.25V4.5h8v4.75h-0.25C13.34,9.25,13,9.59,13,10s0.34,0.75,0.75,0.75H14V15.5z",
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
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M9,13.05C9,14.68,10.34,16,12,16s3-1.32,3-2.95c0-1.31-0.53-1.69-3-4.55C9.52,11.38,9,11.75,9,13.05z",
                    }
                    path {
                        d: "M20,13c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1V5h1c0.55,0,1-0.45,1-1s-0.45-1-1-1H4C3.45,3,3,3.45,3,4s0.45,1,1,1h1v6H4 c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v6H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h16c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1v-6H20z M17,19H7v-6 c0.55,0,1-0.45,1-1s-0.45-1-1-1V5h10v6c-0.55,0-1,0.45-1,1s0.45,1,1,1V19z",
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
                    height: "20",
                    fill: "none",
                    x: "0",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M14,5h-1V4c0-0.83-0.67-1.5-1.5-1.5h-3C7.67,2.5,7,3.17,7,4v1H6c-2.76,0-5,2.24-5,5s2.24,5,5,5v2h1.5v-2h5v2H14v-2 c2.76,0,5-2.24,5-5S16.76,5,14,5z M8.5,4h3v1h-3V4z M14,13.5H6c-1.93,0-3.5-1.57-3.5-3.5S4.07,6.5,6,6.5h8c1.93,0,3.5,1.57,3.5,3.5 S15.93,13.5,14,13.5z",
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
                    d: "M17,6h-1V5c0-1.1-0.9-2-2-2h-4C8.9,3,8,3.9,8,5v1H7c-3.31,0-6,2.69-6,6s2.69,6,6,6v3h2v-3h6v3h2v-3c3.31,0,6-2.69,6-6 S20.31,6,17,6z M10,5h4v1h-4V5z M17,16H7c-2.21,0-4-1.79-4-4s1.79-4,4-4h10c2.21,0,4,1.79,4,4S19.21,16,17,16z",
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
                    x: "0",
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M14,5.18V3.5C14,2.67,13.33,2,12.5,2h-5C6.67,2,6,2.67,6,3.5v1.68C4.84,5.6,4,6.7,4,8v7c0,1.66,1.34,3,3,3h6 c1.66,0,3-1.34,3-3V8C16,6.7,15.16,5.6,14,5.18z M7.5,3.5h5V5h-1.75c0-0.41-0.34-0.75-0.75-0.75S9.25,4.59,9.25,5H7.5V3.5z M7,6.5 h6c0.83,0,1.5,0.67,1.5,1.5v2.75h-9V8C5.5,7.17,6.17,6.5,7,6.5z M13,16.5H7c-0.83,0-1.5-0.67-1.5-1.5v-2.75h9V15 C14.5,15.83,13.83,16.5,13,16.5z",
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M17,6.14V4c0-1.1-0.9-2-2-2H9C7.9,2,7,2.9,7,4v2.14c-1.72,0.45-3,2-3,3.86v8c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4v-8 C20,8.14,18.72,6.59,17,6.14z M9,4h6v2h-2c0-0.55-0.45-1-1-1s-1,0.45-1,1H9V4z M8,8h8c1.1,0,2,0.9,2,2v3H6v-3C6,8.9,6.9,8,8,8z M16,20H8c-1.1,0-2-0.9-2-2v-3h12v3C18,19.1,17.1,20,16,20z",
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
                    x: "0",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M14.5,4.5v4.75h-9V4.5H14.5z M5.5,15.5v-4.75h3.75v1.53c-0.3,0.23-0.5,0.57-0.5,0.97 c0,0.69,0.56,1.25,1.25,1.25s1.25-0.56,1.25-1.25c0-0.4-0.2-0.75-0.5-0.97v-1.53h3.75v4.75H5.5z",
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
            }
            g {
                path {
                    d: "M20,19V3H4v16H2v2h20v-2H20z M18,5v6H6V5H18z M6,19v-6h5v1.82c-0.45,0.32-0.75,0.84-0.75,1.43c0,0.97,0.78,1.75,1.75,1.75 s1.75-0.78,1.75-1.75c0-0.59-0.3-1.12-0.75-1.43V13h5v6H6z",
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
                    x: "0",
                    fill: "none",
                    width: "20",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M16,15.5V3H4v12.5H2V17h6.75c0,0.69,0.56,1.25,1.25,1.25s1.25-0.56,1.25-1.25H18v-1.5H16z M9.25,15.5H5.5V14h3.75V15.5z M14.5,15.5h-3.75V14h3.75V15.5z M14.5,12.5h-9v-8h9V12.5z",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M20,19V3H4v16H2v2h8.25c0,0.97,0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75H22v-2H20z M18,5v10H6V5H18z M6,19v-2h5v2H6z M13,19v-2h5v2H13z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
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
                    x: "0",
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
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
                    width: "20",
                    fill: "none",
                    height: "20",
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
                    height: "24",
                    fill: "none",
                    width: "24",
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
                    x: "0",
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
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
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
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
                    fill: "none",
                    height: "20",
                    width: "20",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M16.5,10h-13L2,18h16L16.5,10z M15.26,11.5l0.33,1.75h-4.83V11.5H15.26z M9.25,11.5v1.75H4.42l0.33-1.75H9.25z M4.14,14.75h5.11v1.75H3.81L4.14,14.75z M10.75,16.5v-1.75h5.11l0.33,1.75H10.75z",
                    }
                    rect {
                        x: "9.25",
                        y: "7",
                        height: "2",
                        width: "1.5",
                    }
                    rect {
                        x: "13.71",
                        y: "5.46",
                        width: "1.5",
                        height: "2",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.3314 12.1139)",
                    }
                    rect {
                        y: "5.71",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -2.9423 5.8107)",
                        height: "1.5",
                        width: "2",
                        x: "4.54",
                    }
                    rect {
                        width: "2",
                        height: "1.5",
                        x: "3",
                        y: "2.05",
                    }
                    rect {
                        width: "2",
                        x: "15",
                        y: "2.05",
                        height: "1.5",
                    }
                    path {
                        d: "M10,6c2.21,0,4-1.79,4-4h-1.5c0,1.38-1.12,2.5-2.5,2.5S7.5,3.38,7.5,2H6C6,4.21,7.79,6,10,6z",
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
                    path {
                        d: "M20,12H4L2,22h20L20,12z M18.36,14l0.4,2H13v-2H18.36z M11,14v2H5.24l0.4-2H11z M4.84,18H11v2H4.44L4.84,18z M13,20v-2 h6.16l0.4,2H13z",
                    }
                    rect {
                        y: "8",
                        width: "2",
                        height: "3",
                        x: "11",
                    }
                    rect {
                        x: "16.53",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -0.2089 14.6085)",
                        width: "2",
                        height: "3",
                        y: "6.06",
                    }
                    rect {
                        height: "2",
                        y: "6.56",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -3.448 6.7885)",
                        width: "3",
                        x: "4.97",
                    }
                    rect {
                        height: "2",
                        width: "3",
                        y: "2",
                        x: "3",
                    }
                    rect {
                        width: "3",
                        y: "2",
                        height: "2",
                        x: "18",
                    }
                    path {
                        d: "M12,7c2.76,0,5-2.24,5-5h-2c0,1.65-1.35,3-3,3S9,3.65,9,2H7C7,4.76,9.24,7,12,7z",
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
                    x: "0",
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                path {
                    d: "M16,15.5V3H4v12.5H2V17h16v-1.5H16z M11.5,4.5v11h-3v-11H11.5z M5.5,4.5H7v11H5.5V4.5z M13,15.5v-11h1.5v11H13z",
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
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M20,19V3H4v16H2v2h20v-2H20z M14,5v14h-4V5H14z M6,5h2v14H6V5z M16,19V5h2v14H16z",
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
                    x: "0",
                    fill: "none",
                    width: "20",
                    height: "20",
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
                    height: "24",
                    fill: "none",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                    x: "0",
                }
            }
            g {
                g {
                    rect {
                        width: "5",
                        x: "3",
                        y: "3",
                        height: "1.5",
                    }
                    rect {
                        height: "1.5",
                        y: "6.5",
                        width: "4",
                        x: "1",
                    }
                    rect {
                        y: "15.5",
                        height: "1.5",
                        width: "4",
                        x: "3",
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
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    rect {
                        x: "4",
                        height: "2",
                        width: "6",
                        y: "3",
                    }
                    rect {
                        width: "5",
                        height: "2",
                        x: "1",
                        y: "7",
                    }
                    rect {
                        height: "2",
                        width: "5",
                        x: "3",
                        y: "19",
                    }
                    path {
                        d: "M22.21,18.61l-2.28-4.1c-0.27-0.48-0.73-0.83-1.26-0.97l-2.69-0.67c-0.02-0.47-0.14-0.92-0.37-1.33l3.96-6.59 c0.65-1.08,0.3-2.48-0.78-3.13c-0.36-0.22-0.77-0.32-1.17-0.32c-0.56,0-1.12,0.21-1.56,0.62l-3.43,3.21C12.23,5.7,12,6.23,12,6.78 v3.4c-0.47,0.17-0.89,0.45-1.23,0.82H3.28C2.02,11,1,12.02,1,13.28c0,1.02,0.67,1.91,1.65,2.19l4.51,1.29 c0.18,0.05,0.37,0.08,0.55,0.08c0.36,0,0.72-0.1,1.03-0.29l2.24-1.34c0.29,0.26,0.63,0.47,1.02,0.61V21c-1.1,0-2,0.9-2,2h6 c0-1.1-0.9-2-2-2v-4.28l4.61,4.61c0.45,0.45,1.03,0.67,1.61,0.67c0.58,0,1.17-0.22,1.61-0.67h0 C22.55,20.61,22.71,19.5,22.21,18.61z M7.72,14.84L3.2,13.55C3.08,13.52,3,13.4,3,13.28C3,13.13,3.13,13,3.28,13h6.73 c0,0.15,0.01,0.3,0.03,0.44L7.72,14.84z M13,14c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C14,13.55,13.55,14,13,14z M14,10.14V6.78l3.43-3.21c0.05-0.05,0.19-0.12,0.34-0.04c0.13,0.08,0.18,0.25,0.1,0.38l-3.74,6.24L14,10.14z M20.42,19.92 c-0.05,0.05-0.24,0.16-0.4,0l-4.85-4.85c0.08-0.09,0.16-0.18,0.24-0.28l2.78,0.69l2.28,4.1C20.53,19.69,20.51,19.83,20.42,19.92z",
                    }
                }
            }
        }
    }
}

