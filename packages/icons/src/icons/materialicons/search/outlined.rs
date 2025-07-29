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
                path {
                    d: "M8,14c0-0.55,0.45-1,1-1s1,0.45,1,1s-0.45,1-1,1S8,14.55,8,14z M12,15c0.55,0,1-0.45,1-1s-0.45-1-1-1s-1,0.45-1,1 S11.45,15,12,15z M15,15c0.55,0,1-0.45,1-1s-0.45-1-1-1s-1,0.45-1,1S14.45,15,15,15z M12,7.5c-1.76,0-3.22,1.31-3.46,3h6.93 C15.22,8.81,13.76,7.5,12,7.5 M12,6c2.76,0,5,2.24,5,5v1H7v-1C7,8.24,9.24,6,12,6z M9,18c0.55,0,1-0.45,1-1s-0.45-1-1-1 s-1,0.45-1,1S8.45,18,9,18z M12,18c0.55,0,1-0.45,1-1s-0.45-1-1-1s-1,0.45-1,1S11.45,18,12,18z M15,18c0.55,0,1-0.45,1-1 s-0.45-1-1-1s-1,0.45-1,1S14.45,18,15,18z M20,4H4v16h16V4 M20,2c1.1,0,2,0.9,2,2v16c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2V4 c0-1.1,0.9-2,2-2H20z",
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
                    d: "M0 0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M21 10.78V8c0-1.65-1.35-3-3-3h-4c-.77 0-1.47.3-2 .78-.53-.48-1.23-.78-2-.78H6C4.35 5 3 6.35 3 8v2.78c-.61.55-1 1.34-1 2.22v6h2v-2h16v2h2v-6c0-.88-.39-1.67-1-2.22zM14 7h4c.55 0 1 .45 1 1v2h-6V8c0-.55.45-1 1-1zM5 8c0-.55.45-1 1-1h4c.55 0 1 .45 1 1v2H5V8zm-1 7v-2c0-.55.45-1 1-1h14c.55 0 1 .45 1 1v2H4z",
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
                path {
                    d: "M17.94,14.04c-0.34,0.34-0.71,0.64-1.1,0.92L16,13.5V11h1v-1h-5.62L9.65,7H6l1,0.76L5.5,9.5l0.95,1L8,9.51v3.99l-0.84,1.46 c-0.39-0.27-0.76-0.58-1.1-0.92L5,15.1c1.87,1.87,4.36,2.9,7,2.9s5.13-1.03,7-2.9L17.94,14.04z M8.45,15.71l0.03-0.06l0.81-1.41 c1.74,0.65,3.66,0.65,5.4,0l0.81,1.41l0.03,0.06c-1.1,0.51-2.3,0.79-3.55,0.79S9.55,16.23,8.45,15.71z M20,4v16H4V4H20 M20,2H4 C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z",
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
                    d: "M20,4v16H4V4H20 M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M16.5,10.67V9 c0-1.1-0.9-2-2-2h-5c-1.1,0-2,0.9-2,2v1.67c-0.88,0.35-1.5,1.2-1.5,2.2V17h1.5v-1.5h9V17H18v-4.13C18,11.87,17.38,11.02,16.5,10.67 z M15,8.5v2H9v-2H15z M7.5,12.87C7.5,12.39,7.89,12,8.37,12h7.27c0.48,0,0.87,0.39,0.87,0.87V14h-9v-1.13H7.5z",
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                path {
                    d: "M18.35,11.45V9c0-1.1-0.9-2-2-2H13c-0.37,0-0.72,0.12-1,0.32C11.72,7.12,11.37,7,11,7H7.65c-1.1,0-2,0.9-2,2v2.45 C5.25,11.91,5,12.51,5,13.17V17h1.5v-1.5h11V17H19v-3.83C19,12.51,18.75,11.91,18.35,11.45z M16.75,10.5h-4v-2h4V10.5z M7.25,8.5h4 v2h-4V8.5z M17.5,14h-11v-1c0-0.55,0.45-1,1-1h9c0.55,0,1,0.45,1,1V14z M20,4v16H4V4H20 M20,2H4C2.9,2,2,2.9,2,4v16 c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z",
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                g {
                    path {
                        d: "M16.13,15.13L18,3h-4V2h-4v1H5C3.9,3,3,3.9,3,5v4c0,1.1,0.9,2,2,2h2.23l0.64,4.13C6.74,16.05,6,17.43,6,19v1 c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-1C18,17.43,17.26,16.05,16.13,15.13z M5,9V5h1.31l0.62,4H5z M15.67,5l-1.38,9H9.72L8.33,5 H15.67z M16,20H8v-1c0-1.65,1.35-3,3-3h2c1.65,0,3,1.35,3,3V20z",
                    }
                    circle {
                        cx: "12",
                        r: "1",
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
                path {
                    d: "M14,13v-1c0-0.55-0.45-1-1-1H9c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-1l2,1.06v-4.12L14,13z M12,5.5l6,4.5v9H6v-9L12,5.5 M12,3L4,9v12h16V9L12,3z",
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
                path {
                    d: "M20,8V6c0-1.65-1.35-3-3-3H7C5.35,3,4,4.35,4,6v2c-1.65,0-3,1.35-3,3v5c0,1.65,1.35,3,3,3v1c0,0.55,0.45,1,1,1 c0.55,0,1-0.45,1-1v-1h12v1c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-1c1.65,0,3-1.35,3-3v-5C23,9.35,21.65,8,20,8z M6,6 c0-0.55,0.45-1,1-1h10c0.55,0,1,0.45,1,1v2.78c-0.61,0.55-1,1.34-1,2.22v2H7v-2c0-0.88-0.39-1.67-1-2.22V6z M21,16 c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1v-5c0-0.55,0.45-1,1-1s1,0.45,1,1v4h14v-4c0-0.55,0.45-1,1-1s1,0.45,1,1V16z",
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
                path {
                    d: "M17,10c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v3c0,1.1,0.9,2,2,2h1v2H7c-1.1,0-2,0.9-2,2v7h2v-3h10v3h2v-7 c0-1.1-0.9-2-2-2h-1v-2H17z M7,8V5h10v3H7z M17,16H7v-2h10V16z M14,12h-4v-2h4V12z",
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
                path {
                    d: "M18.5,3H6C4.9,3,4,3.9,4,5v5.71c0,3.83,2.95,7.18,6.78,7.29c3.96,0.12,7.22-3.06,7.22-7v-1h0.5c1.93,0,3.5-1.57,3.5-3.5 S20.43,3,18.5,3z M16,5v3H6V5H16z M16,10v1c0,2.76-2.24,5-5,5s-5-2.24-5-5v-1 M18.5,8H18V5h0.5C19.33,5,20,5.67,20,6.5 S19.33,8,18.5,8z M4,19h16v2H4V19z",
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
                    path {
                        d: "M18,6V4h2V2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h14v-2h-4.03C17.2,19.09,18,17.64,18,16v-5H8v5c0,1.64,0.81,3.09,2.03,4 H6V4h2v2c0,0.55,0.45,1,1,1h8C17.55,7,18,6.55,18,6z M10,16v-3h6v3c0,1.65-1.35,3-3,3S10,17.65,10,16z",
                    }
                    circle {
                        cy: "9",
                        r: "1",
                        cx: "13",
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
                path {
                    d: "M14.75,6c-1.37,0-2.5,1.52-2.5,3.4c0,1.48,0.7,2.71,1.67,3.18L14,12.62V19h1.5v-6.38l0.08-0.03 c0.97-0.47,1.67-1.7,1.67-3.18C17.25,7.53,16.13,6,14.75,6 M6.5,9.96 M10.5,6C10.23,6,10,6.22,10,6.5V9H9.25V6.5 c0-0.28-0.22-0.5-0.5-0.5s-0.5,0.22-0.5,0.5V9H7.5V6.5C7.5,6.22,7.28,6,7,6S6.5,6.22,6.5,6.5v3.8c0,0.93,0.64,1.71,1.5,1.93V19h1.5 v-6.77c0.86-0.22,1.5-1,1.5-1.93V6.5C11,6.22,10.78,6,10.5,6z M20,4H4v16h16V4 M20,2c1.1,0,2,0.9,2,2v16c0,1.1-0.9,2-2,2H4 c-1.1,0-2-0.9-2-2V4c0-1.1,0.9-2,2-2H20z",
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
                path {
                    d: "M11,16.5h2c0,0.55-0.45,1-1,1S11,17.05,11,16.5z M15,15v-2.34c0-1.54-0.81-2.82-2.25-3.16V9.25c0-0.41-0.34-0.75-0.75-0.75 s-0.75,0.34-0.75,0.75V9.5C9.82,9.84,9,11.12,9,12.66V15H8v1h8v-1H15z M12,5.5L6,10v9h12v-9L12,5.5 M12,3l8,6v12H4V9L12,3z",
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
                        d: "M19,19V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v14H3v2h18v-2H19z M17,19H7V5h10V19z",
                    }
                    rect {
                        height: "2",
                        width: "2",
                        x: "9",
                        y: "11",
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
                path {
                    d: "M19,19V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v14H3v2h18v-2H19z M17,19H7V5h10V19z M13,11h2v2h-2V11z",
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
                path {
                    d: "M10,13H8v-2h2V13z M16,11h-2v2h2V11z M21,19v2H3v-2h1V5c0-1.1,0.9-2,2-2h12c1.1,0,2,0.9,2,2v14H21z M11,5H6v14h5V5z M18,5 h-5v14h5V5z",
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M16,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V8L16,3z M19,19H5V5h10v4h4V19z M7,17h10v-2H7V17z M12,7H7 v2h5V7z M7,13h10v-2H7V13z",
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,20H4V4h16V20z",
                    }
                    circle {
                        r: "1",
                        cx: "9",
                        cy: "13",
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                g {
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
                path {
                    d: "M17.75,10.35V9c0-1.66-1.34-3-3-3h-5.5c-1.66,0-3,1.34-3,3v1.35C5.51,10.78,5,11.58,5,12.5V16c0,1.1,0.9,2,2,2h10 c1.1,0,2-0.9,2-2v-3.5C19,11.58,18.49,10.78,17.75,10.35z M9.25,7.5h5.5c0.83,0,1.5,0.67,1.5,1.5v1.03C14.99,10.15,14,11.2,14,12.5 V13h-4v-0.5c0-1.3-0.99-2.35-2.25-2.47V9C7.75,8.17,8.42,7.5,9.25,7.5z M17.5,16c0,0.28-0.22,0.5-0.5,0.5H7 c-0.28,0-0.5-0.22-0.5-0.5v-3.5c0-0.55,0.45-1,1-1s1,0.45,1,1v2h7v-2c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1V16z M20,4v16H4V4H20 M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z",
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
                path {
                    d: "M7,9H2V7h5V9z M7,12H2v2h5V12z M20.59,19l-3.83-3.83C15.96,15.69,15.02,16,14,16c-2.76,0-5-2.24-5-5s2.24-5,5-5s5,2.24,5,5 c0,1.02-0.31,1.96-0.83,2.75L22,17.59L20.59,19z M17,11c0-1.65-1.35-3-3-3s-3,1.35-3,3s1.35,3,3,3S17,12.65,17,11z M2,19h10v-2H2 V19z",
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
                    d: "M0,0h24v24H0V0z",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M9,17c0,0.55-0.45,1-1,1s-1-0.45-1-1s0.45-1,1-1S9,16.45,9,17z M12,16c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1 S12.55,16,12,16z M16,16c-0.55,0-1,0.45-1,1s0.45,1,1,1c0.55,0,1-0.45,1-1S16.55,16,16,16z M19,12v2H5v-2c0-3.53,2.61-6.43,6-6.92 V3h2v2.08C16.39,5.57,19,8.47,19,12z M17,12c0-2.76-2.24-5-5-5s-5,2.24-5,5H17z M8,19c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1 S8.55,19,8,19z M12,19c-0.55,0-1,0.45-1,1s0.45,1,1,1s1-0.45,1-1S12.55,19,12,19z M16,19c-0.55,0-1,0.45-1,1s0.45,1,1,1 c0.55,0,1-0.45,1-1S16.55,19,16,19z",
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
                    x: "0",
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M18,6.5C18,5.12,14.42,4,10,4S2,5.12,2,6.5c0,1.3,3.18,2.37,7.25,2.49v3.51H7.61c-0.58,0-1.11,0.33-1.35,0.86L5,16h1.5l1-2 h5l1,2H15l-1.26-2.64c-0.25-0.52-0.78-0.86-1.35-0.86h-1.64V8.99C14.82,8.87,18,7.8,18,6.5z M10,5.5c3.04,0,5.16,0.55,6.1,1 c-0.94,0.45-3.06,1-6.1,1s-5.16-0.55-6.1-1C4.84,6.05,6.96,5.5,10,5.5z",
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
                path {
                    d: "M22,7.5C22,5.57,17.52,4,12,4S2,5.57,2,7.5c0,1.81,3.95,3.31,9,3.48V15H9.35c-0.82,0-1.55,0.5-1.86,1.26L6,20h2l1.2-3h5.6 l1.2,3h2l-1.5-3.74C16.2,15.5,15.46,15,14.65,15H13v-4.02C18.05,10.81,22,9.31,22,7.5z M12,6c4.05,0,6.74,0.86,7.72,1.5 C18.74,8.14,16.05,9,12,9S5.26,8.14,4.28,7.5C5.26,6.86,7.95,6,12,6z",
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
                    x: "0",
                }
            }
            g {
                path {
                    d: "M17.93,7.63l-1.2-3C16.58,4.25,16.21,4,15.8,4H4.2C3.79,4,3.42,4.25,3.27,4.63l-1.2,3C1.81,8.29,2.29,9,3,9h1.27L3,16h1.5 l0.73-4h9.32l0.73,4h1.5L15.5,9H17C17.71,9,18.19,8.29,17.93,7.63z M5.5,10.5L5.77,9H14l0.27,1.5H5.5z M3.74,7.5l0.8-2h10.92l0.8,2 H3.74z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
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
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M15.5 3h-11C3.67 3 3 3.67 3 4.5v11c0 .83.67 1.5 1.5 1.5h11c.83 0 1.5-.67 1.5-1.5v-11c0-.83-.67-1.5-1.5-1.5zm0 6.25h-4.75V4.5h4.75v4.75zM9.25 4.5v4.75H4.5V4.5h4.75zM4.5 10.75h4.75v4.75H4.5v-4.75zm6.25 4.75v-4.75h4.75v4.75h-4.75z",
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
                path {
                    d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 8h-6V5h6v6zm-8-6v6H5V5h6zm-6 8h6v6H5v-6zm8 6v-6h6v6h-6z",
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
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
            g {
                path {
                    d: "M18,13c-3.31,0-6,2.69-6,6C15.31,19,18,16.31,18,13z M6,13c0,3.31,2.69,6,6,6C12,15.69,9.31,13,6,13z M8,11.03 c0,0.86,0.7,1.56,1.56,1.56c0.33,0,0.63-0.1,0.89-0.28l-0.01,0.12c0,0.86,0.7,1.56,1.56,1.56s1.56-0.7,1.56-1.56l-0.01-0.12 c0.25,0.17,0.56,0.28,0.89,0.28c0.86,0,1.56-0.7,1.56-1.56c0-0.62-0.37-1.16-0.89-1.41C15.63,9.38,16,8.84,16,8.22 c0-0.86-0.7-1.56-1.56-1.56c-0.33,0-0.63,0.1-0.89,0.28l0.01-0.12c0-0.86-0.7-1.56-1.56-1.56s-1.56,0.7-1.56,1.56l0.01,0.12 C10.2,6.76,9.89,6.66,9.56,6.66C8.7,6.66,8,7.36,8,8.22c0,0.62,0.37,1.16,0.89,1.41C8.37,9.87,8,10.41,8,11.03z M12,8.06 c0.86,0,1.56,0.7,1.56,1.56s-0.7,1.56-1.56,1.56s-1.56-0.7-1.56-1.56S11.14,8.06,12,8.06z M20,4v16H4V4H20 M20,2H4C2.9,2,2,2.9,2,4 v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z",
                }
            }
        }
    }
}

