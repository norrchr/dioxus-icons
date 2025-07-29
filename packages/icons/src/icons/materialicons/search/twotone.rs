use dioxus::prelude::*;
use crate::IconProps;
pub fn bathroom_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M4,20h16V4H4V20z M9,18c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S9.55,18,9,18z M9,15c-0.55,0-1-0.45-1-1 s0.45-1,1-1s1,0.45,1,1S9.55,15,9,15z M12,18c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S12.55,18,12,18z M12,15 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S12.55,15,12,15z M15,18c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S15.55,18,15,18z M15,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S15.55,15,15,15z M7,11c0-2.76,2.24-5,5-5s5,2.24,5,5v1H7V11z",
                    }
                    circle {
                        cy: "14",
                        r: "1",
                        cx: "15",
                    }
                    circle {
                        r: "1",
                        cx: "12",
                        cy: "14",
                    }
                    circle {
                        cx: "15",
                        cy: "17",
                        r: "1",
                    }
                    path {
                        d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,20H4V4h16V20z",
                    }
                    path {
                        d: "M17,11c0-2.76-2.24-5-5-5s-5,2.24-5,5v1h10V11z M8.54,10.5c0.24-1.69,1.7-3,3.46-3s3.22,1.31,3.47,3H8.54z",
                    }
                    circle {
                        cy: "17",
                        cx: "9",
                        r: "1",
                    }
                    circle {
                        cy: "14",
                        r: "1",
                        cx: "9",
                    }
                    circle {
                        cy: "17",
                        r: "1",
                        cx: "12",
                    }
                }
            }
        }
    }
}

pub fn bed_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0 0h24v24H0V0z",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M19 8c0-.55-.45-1-1-1h-4c-.55 0-1 .45-1 1v2h6V8zm-8 0c0-.55-.45-1-1-1H6c-.55 0-1 .45-1 1v2h6V8zm8 4H5c-.55 0-1 .45-1 1v2h16v-2c0-.55-.45-1-1-1z",
                    }
                    path {
                        d: "M21 10.78V8c0-1.65-1.35-3-3-3h-4c-.77 0-1.47.3-2 .78-.53-.48-1.23-.78-2-.78H6C4.35 5 3 6.35 3 8v2.78c-.61.55-1 1.34-1 2.22v6h2v-2h16v2h2v-6c0-.88-.39-1.67-1-2.22zM13 8c0-.55.45-1 1-1h4c.55 0 1 .45 1 1v2h-6V8zM5 8c0-.55.45-1 1-1h4c.55 0 1 .45 1 1v2H5V8zm15 7H4v-2c0-.55.45-1 1-1h14c.55 0 1 .45 1 1v2z",
                    }
                }
            }
        }
    }
}

pub fn bedroom_baby_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M4,20h16V4H4V20z M6.45,10.5l-0.95-1L7,7.76L6,7h3.65l1.73,3H17v1h-1v2.5l0.84,1.46 c0.39-0.28,0.76-0.58,1.1-0.92L19,15.1c-1.87,1.87-4.36,2.9-7,2.9s-5.13-1.03-7-2.9l1.06-1.06c0.34,0.34,0.71,0.65,1.1,0.92 L8,13.5V9.51L6.45,10.5z",
                    }
                    path {
                        d: "M15.53,15.71l-0.03-0.06l-0.81-1.41c-1.74,0.65-3.66,0.65-5.4,0l-0.81,1.41l-0.03,0.06 c1.1,0.52,2.28,0.79,3.53,0.79S14.43,16.22,15.53,15.71z",
                        opacity: ".3",
                    }
                    path {
                        d: "M7.16,14.96c-0.39-0.27-0.76-0.58-1.1-0.92L5,15.1c1.87,1.87,4.36,2.9,7,2.9s5.13-1.03,7-2.9l-1.06-1.06 c-0.34,0.34-0.71,0.64-1.1,0.92L16,13.5V11h1v-1h-5.62L9.65,7H6l1,0.76L5.5,9.5l0.95,1L8,9.51v3.99L7.16,14.96z M8.48,15.65 l0.81-1.41c1.74,0.65,3.66,0.65,5.4,0l0.81,1.41l0.03,0.06c-1.1,0.51-2.3,0.79-3.55,0.79s-2.43-0.27-3.53-0.79L8.48,15.65z",
                    }
                    path {
                        d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,20H4V4h16V20z",
                    }
                }
            }
        }
    }
}

pub fn bedroom_child_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                path {
                    opacity: ".3",
                    d: "M4,20h16V4H4V20z M6,12.87c0-1,0.62-1.85,1.5-2.2V9c0-1.1,0.9-2,2-2h5c1.1,0,2,0.9,2,2v1.67 c0.88,0.35,1.5,1.2,1.5,2.2V17h-1.5v-1.5h-9V17H6V12.87z",
                }
                path {
                    d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,20H4V4h16V20z",
                }
                path {
                    d: "M7.5,15.5h9V17H18v-4.13c0-1-0.62-1.85-1.5-2.2V9c0-1.1-0.9-2-2-2h-5c-1.1,0-2,0.9-2,2v1.67c-0.88,0.35-1.5,1.2-1.5,2.2V17 h1.5V15.5z M9,8.5h6v2H9V8.5z M8.37,12h7.27c0.48,0,0.87,0.39,0.87,0.87V14h-9v-1.13H7.5C7.5,12.39,7.89,12,8.37,12z",
                }
            }
        }
    }
}

pub fn bedroom_parent_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M4,20h16V4H4V20z M5,13.17c0-0.66,0.25-1.26,0.65-1.72V9c0-1.1,0.9-2,2-2H11c0.37,0,0.72,0.12,1,0.32 C12.28,7.12,12.63,7,13,7h3.35c1.1,0,2,0.9,2,2v2.45c0.4,0.46,0.65,1.06,0.65,1.72V17h-1.5v-1.5h-11V17H5V13.17z",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,20H4V4h16V20z",
                    }
                    path {
                        d: "M6.5,15.5h11V17H19v-3.83c0-0.66-0.25-1.26-0.65-1.72V9c0-1.1-0.9-2-2-2H13c-0.37,0-0.72,0.12-1,0.32 C11.72,7.12,11.37,7,11,7H7.65c-1.1,0-2,0.9-2,2v2.45C5.25,11.91,5,12.51,5,13.17V17h1.5V15.5z M12.75,8.5h4v2h-4V8.5z M7.25,8.5 h4v2h-4V8.5z M6.5,13c0-0.55,0.45-1,1-1h9c0.55,0,1,0.45,1,1v1h-11V13z",
                    }
                }
            }
        }
    }
}

pub fn blender_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M13,16h-2c-1.65,0-3,1.35-3,3v1h8v-1C16,17.35,14.65,16,13,16z M12,19c-0.55,0-1-0.45-1-1s0.45-1,1-1 s1,0.45,1,1S12.55,19,12,19z",
                        opacity: ".3",
                    }
                    path {
                        d: "M16.13,15.13L18,3h-4V2h-4v1H5C3.9,3,3,3.9,3,5v4c0,1.1,0.9,2,2,2h2.23l0.64,4.13C6.74,16.05,6,17.43,6,19v1 c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-1C18,17.43,17.26,16.05,16.13,15.13z M5,9V5h1.31l0.62,4H5z M15.67,5l-1.38,9H9.72L8.33,5 H15.67z M16,20H8v-1c0-1.65,1.35-3,3-3h2c1.65,0,3,1.35,3,3V20z",
                    }
                    circle {
                        r: "1",
                        cx: "12",
                        cy: "18",
                    }
                }
            }
        }
    }
}

pub fn camera_indoor_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M6,10v9h12v-9l-6-4.5L6,10z M14,12v1l2-1.06v4.12L14,15v1c0,0.55-0.45,1-1,1H9c-0.55,0-1-0.45-1-1v-4 c0-0.55,0.45-1,1-1h4C13.55,11,14,11.45,14,12z",
                        opacity: ".3",
                    }
                    path {
                        d: "M8,12v4c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-1l2,1.06v-4.12L14,13v-1c0-0.55-0.45-1-1-1H9C8.45,11,8,11.45,8,12z",
                    }
                    path {
                        d: "M12,3L4,9v12h16V9L12,3z M18,19H6v-9l6-4.5l6,4.5V19z",
                    }
                }
            }
        }
    }
}

pub fn camera_outdoor_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                path {
                    d: "M18,13c0-0.55-0.45-1-1-1h-4c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-1l2,1.06v-4.12L18,14V13z M12,3 L4,9v12h16v-2H6v-9l6-4.5l6,4.5v1h2V9L12,3z",
                }
            }
        }
    }
}

pub fn chair_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        enable_background: "new",
                        d: "M7,13h10v-2c0-0.88,0.39-1.67,1-2.22V6c0-0.55-0.45-1-1-1H7C6.45,5,6,5.45,6,6 v2.78C6.61,9.33,7,10.12,7,11V13z",
                        opacity: ".3",
                    }
                    path {
                        enable_background: "new",
                        d: "M20,10c-0.55,0-1,0.45-1,1v4H5v-4c0-0.55-0.45-1-1-1s-1,0.45-1,1v5 c0,0.55,0.45,1,1,1h16c0.55,0,1-0.45,1-1v-5C21,10.45,20.55,10,20,10z",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,8V6c0-1.65-1.35-3-3-3H7C5.35,3,4,4.35,4,6v2c-1.65,0-3,1.35-3,3v5c0,1.65,1.35,3,3,3v1c0,0.55,0.45,1,1,1 c0.55,0,1-0.45,1-1v-1h12v1c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-1c1.65,0,3-1.35,3-3v-5C23,9.35,21.65,8,20,8z M6,6 c0-0.55,0.45-1,1-1h10c0.55,0,1,0.45,1,1v2.78c-0.61,0.55-1,1.34-1,2.22v2H7v-2c0-0.88-0.39-1.67-1-2.22V6z M21,16 c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1v-5c0-0.55,0.45-1,1-1s1,0.45,1,1v4h14v-4c0-0.55,0.45-1,1-1s1,0.45,1,1V16z",
                    }
                }
            }
        }
    }
}

pub fn chair_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        x: "7",
                        y: "14",
                        width: "10",
                        height: "2",
                        opacity: ".3",
                    }
                    rect {
                        y: "5",
                        height: "3",
                        x: "7",
                        opacity: ".3",
                        width: "10",
                    }
                    path {
                        d: "M17,10c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v3c0,1.1,0.9,2,2,2h1v2H7c-1.1,0-2,0.9-2,2v7h2v-3h10v3h2v-7 c0-1.1-0.9-2-2-2h-1v-2H17z M17,14v2H7v-2H17z M10,12v-2h4v2H10z M7,8V5h10v3H7z",
                    }
                }
            }
        }
    }
}

pub fn coffee_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M6,11c0,2.76,2.24,5,5,5s5-2.24,5-5v-1H6V11z",
                        opacity: ".3",
                    }
                    rect {
                        width: "16",
                        x: "4",
                        height: "2",
                        y: "19",
                    }
                    path {
                        d: "M18.5,3H6C4.9,3,4,3.9,4,5v5.71c0,3.83,2.95,7.18,6.78,7.29c3.96,0.12,7.22-3.06,7.22-7v-1h0.5c1.93,0,3.5-1.57,3.5-3.5 S20.43,3,18.5,3z M16,11c0,2.76-2.24,5-5,5s-5-2.24-5-5v-1h10V11z M16,8H6V5h10V8z M18.5,8H18V5h0.5C19.33,5,20,5.67,20,6.5 S19.33,8,18.5,8z",
                    }
                }
            }
        }
    }
}

pub fn coffee_maker_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M13,19c1.65,0,3-1.35,3-3v-3h-6v3C10,17.65,11.35,19,13,19z",
                            opacity: ".3",
                        }
                        path {
                            d: "M9,7h8c0.55,0,1-0.45,1-1V4h2V2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h14v-2h-4.03C17.2,19.09,18,17.64,18,16v-5H8v5 c0,1.64,0.81,3.09,2.03,4H6V4h2v2C8,6.55,8.45,7,9,7z M10,16v-3h6v3c0,1.65-1.35,3-3,3S10,17.65,10,16z",
                        }
                    }
                    circle {
                        cx: "13",
                        r: "1",
                        cy: "9",
                    }
                }
            }
        }
    }
}

pub fn dining_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M4,20h16V4H4V20z M14.75,6c1.38,0,2.5,1.53,2.5,3.41c0,1.48-0.7,2.71-1.67,3.18l-0.08,0.03V19H14v-6.38 l-0.08-0.04c-0.97-0.47-1.67-1.7-1.67-3.18C12.25,7.52,13.38,6,14.75,6z M6.5,6.5C6.5,6.22,6.72,6,7,6s0.5,0.22,0.5,0.5V9h0.75 V6.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5V9H10V6.5C10,6.22,10.23,6,10.5,6C10.78,6,11,6.22,11,6.5v3.8 c0,0.93-0.64,1.71-1.5,1.93V19H8v-6.77c-0.86-0.22-1.5-1-1.5-1.93V6.5z",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,20H4V4h16V20z",
                    }
                    path {
                        d: "M8,12.23V19h1.5v-6.77c0.86-0.22,1.5-1,1.5-1.93V6.5C11,6.22,10.78,6,10.5,6C10.23,6,10,6.22,10,6.5V9H9.25V6.5 c0-0.28-0.22-0.5-0.5-0.5s-0.5,0.22-0.5,0.5V9H7.5V6.5C7.5,6.22,7.28,6,7,6S6.5,6.22,6.5,6.5v3.8C6.5,11.23,7.14,12.01,8,12.23z",
                    }
                    path {
                        d: "M13.92,12.58L14,12.62V19h1.5v-6.38l0.08-0.03c0.97-0.47,1.67-1.7,1.67-3.18c0-1.88-1.12-3.41-2.5-3.41 c-1.37,0-2.5,1.52-2.5,3.4C12.25,10.88,12.95,12.11,13.92,12.58z",
                    }
                }
            }
        }
    }
}

pub fn doorbell_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                g {
                    path {
                        d: "M6,10v9h12v-9l-6-4.5L6,10z M12,17.5c-0.55,0-1-0.45-1-1h2C13,17.05,12.55,17.5,12,17.5z M12.75,9.25V9.5 C14.19,9.84,15,11.12,15,12.66V15h1v1H8v-1h1v-2.34c0-1.54,0.82-2.82,2.25-3.16V9.25c0-0.41,0.34-0.75,0.75-0.75 S12.75,8.84,12.75,9.25z",
                        opacity: ".3",
                    }
                    path {
                        d: "M12,3L4,9v12h16V9L12,3z M18,19H6v-9l6-4.5l6,4.5V19z",
                    }
                    path {
                        d: "M11.25,9.25V9.5C9.82,9.84,9,11.12,9,12.66V15H8v1h8v-1h-1v-2.34c0-1.54-0.81-2.82-2.25-3.16V9.25 c0-0.41-0.34-0.75-0.75-0.75S11.25,8.84,11.25,9.25z",
                    }
                    path {
                        d: "M12,17.5c0.55,0,1-0.45,1-1h-2C11,17.05,11.45,17.5,12,17.5z",
                    }
                }
            }
        }
    }
}

pub fn door_back_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M7,19h10V5H7V19z M9,11h2v2H9V11z",
                        opacity: ".3",
                    }
                    path {
                        d: "M19,19V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v14H3v2h18v-2H19z M17,19H7V5h10V19z",
                    }
                    rect {
                        y: "11",
                        height: "2",
                        x: "9",
                        width: "2",
                    }
                }
            }
        }
    }
}

pub fn door_front_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                g {
                    path {
                        d: "M7,19h10V5H7V19z M13,11h2v2h-2V11z",
                        opacity: ".3",
                    }
                    rect {
                        x: "13",
                        height: "2",
                        y: "11",
                        width: "2",
                    }
                    path {
                        d: "M19,19V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v14H3v2h18v-2H19z M17,19H7V5h10V19z",
                    }
                }
            }
        }
    }
}

pub fn door_sliding_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M13,19h5V5h-5V19z M14,11h2v2h-2V11z",
                        opacity: ".3",
                    }
                    path {
                        d: "M6,19h5V5H6V19z M8,11h2v2H8V11z",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,19V5c0-1.1-0.9-2-2-2H6C4.9,3,4,3.9,4,5v14H3v2h18v-2H20z M11,19H6V5h5V19z M18,19h-5V5h5V19z",
                    }
                    rect {
                        height: "2",
                        width: "2",
                        x: "8",
                        y: "11",
                    }
                    rect {
                        x: "14",
                        width: "2",
                        height: "2",
                        y: "11",
                    }
                }
            }
        }
    }
}

pub fn feed_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                path {
                    opacity: ".3",
                    d: "M15,5H5v14h14V9h-4V5z M7,7h5v2H7V7z M17,17H7v-2h10V17z M17,11v2H7v-2H17z",
                }
                path {
                    d: "M7,13h10v-2H7V13z M7,17h10v-2H7V17z M16,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V8L16,3z M19,19H5V5 h10v4h4V19z M12,7H7v2h5V7z",
                }
            }
        }
    }
}

pub fn flatware_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M16,7.08c0,1.77-0.84,3.25-2,3.82V21h-2V10.9c-1.16-0.57-2-2.05-2-3.82C10.01,4.83,11.35,3,13,3C14.66,3,16,4.83,16,7.08z M17,3v18h2v-8h2V7C21,5.24,19.76,3,17,3z M8.28,3c-0.4,0-0.72,0.32-0.72,0.72V7H6.72V3.72C6.72,3.32,6.4,3,6,3 S5.28,3.32,5.28,3.72V7H4.44V3.72C4.44,3.32,4.12,3,3.72,3S3,3.32,3,3.72V9c0,1.1,0.9,2,2,2v10h2V11c1.1,0,2-0.9,2-2V3.72 C9,3.32,8.68,3,8.28,3z",
                }
            }
        }
    }
}

pub fn garage_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                g {
                    path {
                        d: "M17,11.51V11.5H7v0.01v2.99h10V11.51z M9,14c-0.55,0-1-0.45-1-1s0.45-1,1-1c0.55,0,1,0.45,1,1S9.55,14,9,14 z M15,14c-0.55,0-1-0.45-1-1s0.45-1,1-1c0.55,0,1,0.45,1,1S15.55,14,15,14z",
                        opacity: ".3",
                    }
                    path {
                        d: "M4,20h16V4H4V20z M5,11.19C5.82,8.72,6.34,7.16,6.56,6.5c0.05-0.16,0.12-0.29,0.19-0.4 C6.77,6.08,6.78,6.06,6.8,6.04C7.18,5.51,7.72,5.5,7.72,5.5h8.56c0,0,0.54,0.01,0.92,0.53c0.02,0.03,0.03,0.05,0.05,0.07 c0.07,0.11,0.14,0.24,0.19,0.4c0.22,0.66,0.74,2.23,1.56,4.69v6.5c0,0.45-0.35,0.81-0.78,0.81h-0.44c-0.44,0-0.78-0.36-0.78-0.81 V16.5H7v1.19c0,0.45-0.35,0.81-0.78,0.81H5.78C5.35,18.5,5,18.14,5,17.69V11.19z",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,20H4V4h16V20z",
                    }
                    circle {
                        r: "1",
                        cy: "13",
                        cx: "9",
                    }
                    circle {
                        cx: "15",
                        cy: "13",
                        r: "1",
                    }
                    path {
                        d: "M5.78,18.5h0.44C6.65,18.5,7,18.14,7,17.69V16.5h10v1.19c0,0.45,0.34,0.81,0.78,0.81h0.44c0.43,0,0.78-0.36,0.78-0.81 v-6.5c-0.82-2.46-1.34-4.03-1.56-4.69c-0.05-0.16-0.12-0.29-0.19-0.4c-0.02-0.02-0.03-0.04-0.05-0.07 c-0.38-0.52-0.92-0.53-0.92-0.53H7.72c0,0-0.54,0.01-0.92,0.54C6.78,6.06,6.77,6.08,6.75,6.1C6.68,6.21,6.61,6.34,6.56,6.5 C6.34,7.16,5.82,8.72,5,11.19v6.5C5,18.14,5.35,18.5,5.78,18.5z M8.33,7.5h7.34l0.23,0.69l0.43,1.31H7.67L8.33,7.5z M7,11.51V11.5 h10v0.01v2.99H7V11.51z",
                    }
                }
            }
        }
    }
}

pub fn light_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M10,17c0,1.1,0.9,2,2,2s2-0.9,2-2h-2H10z",
                    }
                    path {
                        d: "M13,6.06V3h-2v3.06c-4.5,0.5-8,4.31-8,8.93C3,16.1,3.9,17,5.01,17H8c0,2.21,1.79,4,4,4s4-1.79,4-4h2.99 C20.1,17,21,16.1,21,14.99C21,10.37,17.5,6.56,13,6.06z M12,19c-1.1,0-2-0.9-2-2h2h2C14,18.1,13.1,19,12,19z M12,15H5 c0-3.86,3.14-7,7-7s7,3.14,7,7H12z",
                    }
                }
            }
        }
    }
}

pub fn living_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M4,20h16V4H4V20z M5,12.5c0-0.92,0.51-1.72,1.25-2.15V9c0-1.66,1.34-3,3-3h5.5c1.66,0,3,1.34,3,3v1.35 C18.49,10.78,19,11.58,19,12.5V16c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V12.5z",
                        opacity: ".3",
                    }
                    path {
                        d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,20H4V4h16V20z M7,18h10 c1.1,0,2-0.9,2-2v-3.5c0-0.92-0.51-1.72-1.25-2.15V9c0-1.66-1.34-3-3-3h-5.5c-1.66,0-3,1.34-3,3v1.35C5.51,10.78,5,11.58,5,12.5 V16C5,17.1,5.9,18,7,18z M7.75,9c0-0.83,0.67-1.5,1.5-1.5h5.5c0.83,0,1.5,0.67,1.5,1.5v1.03C14.99,10.15,14,11.2,14,12.5V13h-4 v-0.5c0-1.3-0.99-2.35-2.25-2.47V9z M6.5,12.5c0-0.55,0.45-1,1-1s1,0.45,1,1v2h7v-2c0-0.55,0.45-1,1-1s1,0.45,1,1V16 c0,0.28-0.22,0.5-0.5,0.5H7c-0.28,0-0.5-0.22-0.5-0.5V12.5z",
                    }
                }
            }
        }
    }
}

pub fn manage_search_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    rect {
                        y: "12",
                        height: "2",
                        width: "5",
                        x: "2",
                    }
                    path {
                        d: "M18.17,13.75C18.69,12.96,19,12.02,19,11c0-2.76-2.24-5-5-5s-5,2.24-5,5s2.24,5,5,5c1.02,0,1.96-0.31,2.76-0.83L20.59,19 L22,17.59L18.17,13.75z M14,14c-1.65,0-3-1.35-3-3c0-1.65,1.35-3,3-3s3,1.35,3,3C17,12.65,15.65,14,14,14z",
                    }
                    rect {
                        height: "2",
                        x: "2",
                        width: "5",
                        y: "7",
                    }
                    rect {
                        width: "10",
                        height: "2",
                        y: "17",
                        x: "2",
                    }
                }
            }
        }
    }
}

pub fn podcasts_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M14,12c0,0.74-0.4,1.38-1,1.72V22h-2v-8.28c-0.6-0.35-1-0.98-1-1.72c0-1.1,0.9-2,2-2S14,10.9,14,12z M12,6 c-3.31,0-6,2.69-6,6c0,1.74,0.75,3.31,1.94,4.4l1.42-1.42C8.53,14.25,8,13.19,8,12c0-2.21,1.79-4,4-4s4,1.79,4,4 c0,1.19-0.53,2.25-1.36,2.98l1.42,1.42C17.25,15.31,18,13.74,18,12C18,8.69,15.31,6,12,6z M12,2C6.48,2,2,6.48,2,12 c0,2.85,1.2,5.41,3.11,7.24l1.42-1.42C4.98,16.36,4,14.29,4,12c0-4.41,3.59-8,8-8s8,3.59,8,8c0,2.29-0.98,4.36-2.53,5.82l1.42,1.42 C20.8,17.41,22,14.85,22,12C22,6.48,17.52,2,12,2z",
                }
            }
        }
    }
}

pub fn shower_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                g {
                    path {
                        opacity: ".3",
                        d: "M12,7c-2.76,0-5,2.24-5,5h10C17,9.24,14.76,7,12,7z",
                    }
                    circle {
                        cy: "20",
                        cx: "8",
                        r: "1",
                    }
                    circle {
                        r: "1",
                        cx: "16",
                        cy: "17",
                    }
                    path {
                        d: "M13,5.08V3h-2v2.08C7.61,5.57,5,8.47,5,12v2h14v-2C19,8.47,16.39,5.57,13,5.08z M7,12c0-2.76,2.24-5,5-5s5,2.24,5,5H7z",
                    }
                    circle {
                        r: "1",
                        cx: "16",
                        cy: "20",
                    }
                    circle {
                        r: "1",
                        cx: "12",
                        cy: "17",
                    }
                    circle {
                        r: "1",
                        cy: "17",
                        cx: "8",
                    }
                    circle {
                        cx: "12",
                        cy: "20",
                        r: "1",
                    }
                }
            }
        }
    }
}

pub fn table_bar_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    ellipse {
                        cx: "10",
                        rx: "6.1",
                        ry: "1",
                        opacity: ".3",
                        cy: "6.5",
                    }
                    path {
                        d: "M18,6.5C18,5.12,14.42,4,10,4S2,5.12,2,6.5c0,1.3,3.18,2.37,7.25,2.49v3.51H7.61c-0.58,0-1.11,0.33-1.35,0.86L5,16h1.5 l1-2h5l1,2H15l-1.26-2.64c-0.25-0.52-0.78-0.86-1.35-0.86h-1.64V8.99C14.82,8.87,18,7.8,18,6.5z M10,7.5c-3.04,0-5.16-0.55-6.1-1 c0.94-0.45,3.06-1,6.1-1s5.16,0.55,6.1,1C15.16,6.95,13.04,7.5,10,7.5z",
                    }
                }
            }
        }
    }
}

pub fn table_bar_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    ellipse {
                        cx: "12",
                        rx: "7.72",
                        ry: "1.5",
                        opacity: ".3",
                        cy: "7.5",
                    }
                    path {
                        d: "M22,7.5C22,5.57,17.52,4,12,4S2,5.57,2,7.5c0,1.81,3.95,3.31,9,3.48V15H9.35c-0.82,0-1.55,0.5-1.86,1.26L6,20h2l1.2-3h5.6 l1.2,3h2l-1.5-3.74C16.2,15.5,15.46,15,14.65,15H13v-4.02C18.05,10.81,22,9.31,22,7.5z M12,9C7.95,9,5.26,8.14,4.28,7.5 C5.26,6.86,7.95,6,12,6s6.74,0.86,7.72,1.5C18.74,8.14,16.05,9,12,9z",
                    }
                }
            }
        }
    }
}

pub fn table_restaurant_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "4.54,5.5 3.74,7.5 16.26,7.5 15.46,5.5",
                        opacity: ".3",
                    }
                    path {
                        d: "M17.93,7.63l-1.2-3C16.58,4.25,16.21,4,15.8,4H4.2C3.79,4,3.42,4.25,3.27,4.63l-1.2,3C1.81,8.29,2.29,9,3,9h1.27L3,16h1.5 l0.73-4h9.32l0.73,4h1.5L15.5,9H17C17.71,9,18.19,8.29,17.93,7.63z M5.5,10.5L5.77,9H14l0.27,1.5H5.5z M3.74,7.5l0.8-2h10.92 l0.8,2H3.74z",
                    }
                }
            }
        }
    }
}

pub fn table_restaurant_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "5.18,6 4.33,9 19.67,9 18.82,6",
                        opacity: ".3",
                    }
                    path {
                        d: "M21.96,9.73l-1.43-5C20.41,4.3,20.02,4,19.57,4H4.43C3.98,4,3.59,4.3,3.47,4.73l-1.43,5C1.86,10.36,2.34,11,3,11h2.2L4,20 h2l0.67-5h10.67L18,20h2l-1.2-9H21C21.66,11,22.14,10.36,21.96,9.73z M6.93,13l0.27-2h9.6l0.27,2H6.93z M4.33,9l0.86-3h13.63 l0.86,3H4.33z",
                    }
                }
            }
        }
    }
}

pub fn window_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        opacity: ".3",
                        height: "4.75",
                        width: "4.75",
                        y: "10.75",
                        x: "10.75",
                    }
                    rect {
                        width: "4.75",
                        x: "4.5",
                        y: "10.75",
                        opacity: ".3",
                        height: "4.75",
                    }
                    rect {
                        height: "4.75",
                        opacity: ".3",
                        width: "4.75",
                        x: "4.5",
                        y: "4.5",
                    }
                    rect {
                        height: "4.75",
                        y: "4.5",
                        x: "10.75",
                        opacity: ".3",
                        width: "4.75",
                    }
                    path {
                        d: "M15.5 3h-11C3.67 3 3 3.67 3 4.5v11c0 .83.67 1.5 1.5 1.5h11c.83 0 1.5-.67 1.5-1.5v-11c0-.83-.67-1.5-1.5-1.5zM9.25 15.5H4.5v-4.75h4.75v4.75zm0-6.25H4.5V4.5h4.75v4.75zm6.25 6.25h-4.75v-4.75h4.75v4.75zm0-6.25h-4.75V4.5h4.75v4.75z",
                    }
                }
            }
        }
    }
}

pub fn window_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        height: "6",
                        x: "13",
                        opacity: ".3",
                        y: "13",
                        width: "6",
                    }
                    rect {
                        width: "6",
                        height: "6",
                        x: "5",
                        opacity: ".3",
                        y: "13",
                    }
                    rect {
                        x: "5",
                        width: "6",
                        y: "5",
                        opacity: ".3",
                        height: "6",
                    }
                    rect {
                        width: "6",
                        opacity: ".3",
                        x: "13",
                        height: "6",
                        y: "5",
                    }
                    path {
                        d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-8 16H5v-6h6v6zm0-8H5V5h6v6zm8 8h-6v-6h6v6zm0-8h-6V5h6v6z",
                    }
                }
            }
        }
    }
}

pub fn yard_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M4,20h16V4H4V20z M8,8.22c0-0.86,0.7-1.56,1.56-1.56c0.33,0,0.64,0.1,0.89,0.28l-0.01-0.12 c0-0.86,0.7-1.56,1.56-1.56s1.56,0.7,1.56,1.56l-0.01,0.12c0.26-0.18,0.56-0.28,0.89-0.28c0.86,0,1.56,0.7,1.56,1.56 c0,0.62-0.37,1.16-0.89,1.4C15.63,9.87,16,10.41,16,11.03c0,0.86-0.7,1.56-1.56,1.56c-0.33,0-0.64-0.11-0.89-0.28l0.01,0.12 c0,0.86-0.7,1.56-1.56,1.56s-1.56-0.7-1.56-1.56l0.01-0.12c-0.26,0.18-0.56,0.28-0.89,0.28C8.7,12.59,8,11.89,8,11.03 c0-0.62,0.37-1.16,0.89-1.4C8.37,9.38,8,8.84,8,8.22z M12,19c0-3.31,2.69-6,6-6C18,16.31,15.31,19,12,19s-6-2.69-6-6 C9.31,13,12,15.69,12,19z",
                        opacity: ".3",
                    }
                    path {
                        d: "M8,11.03c0,0.86,0.7,1.56,1.56,1.56c0.33,0,0.63-0.1,0.89-0.28l-0.01,0.12c0,0.86,0.7,1.56,1.56,1.56s1.56-0.7,1.56-1.56 l-0.01-0.12c0.25,0.17,0.56,0.28,0.89,0.28c0.86,0,1.56-0.7,1.56-1.56c0-0.62-0.37-1.16-0.89-1.41C15.63,9.38,16,8.84,16,8.22 c0-0.86-0.7-1.56-1.56-1.56c-0.33,0-0.63,0.1-0.89,0.28l0.01-0.12c0-0.86-0.7-1.56-1.56-1.56s-1.56,0.7-1.56,1.56l0.01,0.12 C10.2,6.76,9.89,6.66,9.56,6.66C8.7,6.66,8,7.36,8,8.22c0,0.62,0.37,1.16,0.89,1.41C8.37,9.87,8,10.41,8,11.03z M12,8.06 c0.86,0,1.56,0.7,1.56,1.56s-0.7,1.56-1.56,1.56s-1.56-0.7-1.56-1.56S11.14,8.06,12,8.06z",
                    }
                    path {
                        d: "M18,13c-3.31,0-6,2.69-6,6C15.31,19,18,16.31,18,13z",
                    }
                    path {
                        d: "M12,19c0-3.31-2.69-6-6-6C6,16.31,8.69,19,12,19z",
                    }
                    path {
                        d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,20H4V4h16V20z",
                    }
                }
            }
        }
    }
}

