use dioxus::prelude::*;
use crate::IconProps;
pub fn _18_up_rating_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        height: ".75",
                        x: "11",
                        y: "10.25",
                        width: "1",
                    }
                    rect {
                        width: "1",
                        height: ".75",
                        x: "11",
                        y: "9",
                    }
                    path {
                        d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M8.5,12L8.5,12C8.22,12,8,11.78,8,11.5V9H7.5C7.22,9,7,8.78,7,8.5v0C7,8.22,7.22,8,7.5,8h1C8.78,8,9,8.22,9,8.5v3 C9,11.78,8.78,12,8.5,12z M13,11.5c0,0.28-0.22,0.5-0.5,0.5h-2c-0.28,0-0.5-0.22-0.5-0.5v-3C10,8.22,10.22,8,10.5,8h2 C12.78,8,13,8.22,13,8.5V11.5z",
                    }
                }
            }
        }
    }
}

pub fn _18_up_rating_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    rect {
                        y: "12.5",
                        width: "1.5",
                        height: "1.5",
                        x: "13",
                    }
                    rect {
                        y: "10",
                        width: "1.5",
                        height: "1.5",
                        x: "13",
                    }
                    path {
                        d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M9.25,15L9.25,15 c-0.41,0-0.75-0.34-0.75-0.75V10.5H7.75C7.34,10.5,7,10.16,7,9.75v0C7,9.34,7.34,9,7.75,9H9c0.55,0,1,0.45,1,1v4.25 C10,14.66,9.66,15,9.25,15z M16,14c0,0.55-0.45,1-1,1h-2.5c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1H15c0.55,0,1,0.45,1,1V14z",
                    }
                }
            }
        }
    }
}

pub fn _6_ft_apart_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                x: "0",
                fill: "none",
            }
            path {
                d: "M6,6c1.1,0,2-0.9,2-2S7.1,2,6,2S4,2.9,4,4S4.9,6,6,6z M10,9.43c0-0.81-0.48-1.53-1.22-1.85C7.93,7.21,6.99,7,6,7 C5.01,7,4.07,7.21,3.22,7.58C2.48,7.9,2,8.62,2,9.43V10h8V9.43z M18,6c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S16.9,6,18,6z M22,9.43 c0-0.81-0.48-1.53-1.22-1.85C19.93,7.21,18.99,7,18,7c-0.99,0-1.93,0.21-2.78,0.58C14.48,7.9,14,8.62,14,9.43V10h8V9.43z M21.65,14.35l-1.79,1.79C19.54,16.46,19,16.24,19,15.79v-0.8L5,15v0.79c0,0.45-0.54,0.67-0.85,0.35l-1.79-1.79 c-0.2-0.2-0.2-0.51,0-0.71l1.79-1.79C4.46,11.54,5,11.76,5,12.21v0.8L19,13v-0.79c0-0.45,0.54-0.67,0.85-0.35l1.79,1.79 C21.84,13.84,21.84,14.16,21.65,14.35z M10,18.5L10,18.5c0-0.28-0.22-0.5-0.5-0.5h-2C7.22,18,7,18.22,7,18.5v3 C7,21.78,7.22,22,7.5,22h2c0.28,0,0.5-0.22,0.5-0.5V20c0-0.28-0.22-0.5-0.5-0.5H8V19h1.5C9.78,19,10,18.78,10,18.5z M9,20.5V21H8 v-0.5H9z M17,19h-0.5v2.5c0,0.28-0.22,0.5-0.5,0.5h0c-0.28,0-0.5-0.22-0.5-0.5V19H15c-0.28,0-0.5-0.22-0.5-0.5v0 c0-0.28,0.22-0.5,0.5-0.5h2c0.28,0,0.5,0.22,0.5,0.5v0C17.5,18.78,17.28,19,17,19z M14,18.5c0,0.28-0.22,0.5-0.5,0.5h-1v0.5H13 c0.28,0,0.5,0.22,0.5,0.5c0,0.28-0.22,0.5-0.5,0.5h-0.5v1c0,0.28-0.22,0.5-0.5,0.5c-0.28,0-0.5-0.22-0.5-0.5v-3 c0-0.28,0.22-0.5,0.5-0.5h1.5C13.78,18,14,18.22,14,18.5z",
            }
        }
    }
}

pub fn add_moderator_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M17,10c1.08,0,2.09,0.25,3,0.68v-4.3c0-0.83-0.52-1.58-1.3-1.87l-6-2.25c-0.45-0.17-0.95-0.17-1.4,0l-6,2.25 C4.52,4.81,4,5.55,4,6.39v4.7c0,5.05,3.41,9.76,8,10.91c0.03-0.01,0.05-0.02,0.08-0.02C10.8,20.71,10,18.95,10,17 C10,13.13,13.13,10,17,10z",
                    }
                    path {
                        d: "M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S19.76,12,17,12z M19.5,17.5h-2v2c0,0.28-0.22,0.5-0.5,0.5 s-0.5-0.22-0.5-0.5v-2h-2c-0.28,0-0.5-0.22-0.5-0.5s0.22-0.5,0.5-0.5h2v-2c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5v2h2 c0.28,0,0.5,0.22,0.5,0.5S19.78,17.5,19.5,17.5z",
                    }
                }
            }
        }
    }
}

pub fn add_reaction_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M24,4c0,0.55-0.45,1-1,1h-1v1c0,0.55-0.45,1-1,1s-1-0.45-1-1V5h-1c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h1V2 c0-0.55,0.45-1,1-1s1,0.45,1,1v1h1C23.55,3,24,3.45,24,4z M21.52,8.95C21.83,9.91,22,10.94,22,12c0,5.52-4.48,10-10,10S2,17.52,2,12 C2,6.48,6.48,2,12,2c1.5,0,2.92,0.34,4.2,0.94C16.08,3.27,16,3.62,16,4c0,1.35,0.9,2.5,2.13,2.87C18.5,8.1,19.65,9,21,9 C21.18,9,21.35,8.98,21.52,8.95z M7,9.5C7,10.33,7.67,11,8.5,11S10,10.33,10,9.5S9.33,8,8.5,8S7,8.67,7,9.5z M16.31,14H7.69 c-0.38,0-0.63,0.42-0.44,0.75C8.2,16.39,9.97,17.5,12,17.5s3.8-1.11,4.75-2.75C16.94,14.42,16.7,14,16.31,14z M17,9.5 C17,8.67,16.33,8,15.5,8S14,8.67,14,9.5s0.67,1.5,1.5,1.5S17,10.33,17,9.5z",
            }
        }
    }
}

pub fn architecture_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M6.67,14.89L6.79,16l0.81-0.77l1.92-5.28C9.19,9.9,8.87,9.79,8.58,9.63L6.67,14.89z",
                    }
                    path {
                        d: "M11.42,9.63c-0.29,0.16-0.61,0.27-0.95,0.32l1.92,5.28L13.21,16l0.13-1.11L11.42,9.63z",
                    }
                    path {
                        d: "M11.95,7.47c0.26-1.13-0.44-2.14-1.45-2.4V4.5C10.5,4.22,10.28,4,10,4h0C9.72,4,9.5,4.22,9.5,4.5v0.57 C8.64,5.29,8,6.07,8,7c0,1.26,1.17,2.25,2.48,1.94C11.2,8.78,11.78,8.19,11.95,7.47z M10,8C9.45,8,9,7.55,9,7c0-0.55,0.45-1,1-1 s1,0.45,1,1C11,7.55,10.55,8,10,8z",
                    }
                }
            }
        }
    }
}

pub fn architecture_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M6.36,18.78L6.61,21l1.62-1.54l2.77-7.6c-0.68-0.17-1.28-0.51-1.77-0.98L6.36,18.78z",
                    }
                    path {
                        d: "M14.77,10.88c-0.49,0.47-1.1,0.81-1.77,0.98l2.77,7.6L17.39,21l0.26-2.22L14.77,10.88z",
                    }
                    path {
                        d: "M14.94,8.6c0.3-1.56-0.6-2.94-1.94-3.42V4c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v1.18C9.84,5.6,9,6.7,9,8 c0,1.84,1.66,3.3,3.56,2.95C13.74,10.73,14.71,9.78,14.94,8.6z M12,9c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C13,8.55,12.55,9,12,9z",
                    }
                }
            }
        }
    }
}

pub fn assist_walker_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cx: "10.5",
                        r: "1.5",
                        cy: "4.5",
                    }
                    path {
                        d: "M15.25,15.05l-0.64-4.92c-0.06-0.5-0.49-0.88-0.99-0.88H12c-0.48-0.16-0.93-0.41-1.3-0.78L8.66,6.44c0,0-0.01,0-0.01-0.01 L8.64,6.42l0,0c-0.59-0.57-1.52-0.57-2.1,0.01l-2.31,2.3c-0.25,0.35-0.34,0.8-0.25,1.22l0.76,3.28l-1.86,2.48 c-0.25,0.34-0.18,0.81,0.16,1.06h0c0.33,0.24,0.79,0.17,1.04-0.16l1.86-2.49c0.24-0.33,0.34-0.73,0.28-1.13l-0.09-0.56l1.11,1.08 v2.75C7.25,16.66,7.58,17,8,17h0c0.41,0,0.75-0.34,0.75-0.75v-2.83c0-0.4-0.16-0.78-0.44-1.06L6.55,10.6l2.08-2.08 c1.09,1.09,1.34,1.42,2.23,1.87l-0.78,6.04c-0.04,0.3,0.19,0.56,0.5,0.56h0c0.25,0,0.46-0.19,0.5-0.44L11.4,14h2.71l0.16,1.26 c-0.39,0.29-0.62,0.77-0.47,1.33c0.12,0.41,0.45,0.75,0.87,0.86c0.83,0.22,1.58-0.41,1.58-1.21 C16.25,15.65,15.81,15.17,15.25,15.05z M11.53,13l0.36-2.75h1.73l0,0L13.97,13H11.53z",
                    }
                }
            }
        }
    }
}

pub fn assist_walker_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    circle {
                        cx: "12.5",
                        cy: "4.5",
                        r: "2",
                    }
                    path {
                        d: "M19.77,17.72l-0.64-6.37C19.06,10.58,18.41,10,17.64,10H16c-1.5-0.02-2.86-0.54-3.76-1.44l-2-1.98 C10.08,6.42,9.62,6,8.83,6C8.32,6,7.81,6.2,7.42,6.59L4.08,9.91c-0.53,0.68-0.51,1.57-0.21,2.13l1.43,2.8L2.75,18.1 c-0.34,0.43-0.26,1.06,0.17,1.4l0,0c0.44,0.34,1.07,0.26,1.41-0.17l2.56-3.29c0.33-0.42,0.47-0.95,0.41-1.48l-0.07-0.53L8,14.75 V19c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-4.29c0-0.53-0.21-1.04-0.59-1.41l-1.53-1.53l2.36-2.36c0.94,0.94,1.72,1.82,3.59,2.32 l-0.75,7.46c-0.04,0.44,0.3,0.83,0.75,0.83h0c0.38,0,0.7-0.29,0.75-0.66l0.33-2.84h3.18l0.14,1.22c-0.44,0.26-0.73,0.74-0.73,1.28 c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5C20.5,18.46,20.21,17.98,19.77,17.72z M15.09,15l0.41-3.5h2l0.41,3.5H15.09z",
                    }
                }
            }
        }
    }
}

pub fn back_hand_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M9,10c0.28,0,0.5-0.22,0.5-0.5V1c0-0.55,0.45-1,1-1s1,0.45,1,1v8.5c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5V2.25 c0-0.55,0.45-1,1-1s1,0.45,1,1V9.5c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5V5c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1v8.5 c0,3.59-2.91,6.5-6.5,6.5l0,0c-2.69,0-5.11-1.64-6.11-4.14L2.31,9.37C2,8.59,2.73,7.8,3.53,8.05l0.38,0.12 C4.35,8.29,4.7,8.62,4.87,9.04l0.36,0.9l0.93,2.32c0.06,0.14,0.19,0.23,0.34,0.23v-10c0-0.55,0.45-1,1-1s1,0.45,1,1v7 C8.5,9.78,8.72,10,9,10z",
            }
        }
    }
}

pub fn back_hand_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21,16c0,4.42-3.58,8-8,8c-3.26,0-6.19-1.99-7.4-5.02l-3.03-7.61C2.26,10.58,3,9.79,3.81,10.05l0.79,0.26 c0.56,0.18,1.02,0.61,1.24,1.16l1.28,3.21C7.2,14.88,7.38,15,7.58,15H8V3.25C8,2.56,8.56,2,9.25,2s1.25,0.56,1.25,1.25v8.25 c0,0.28,0.22,0.5,0.5,0.5l0,0c0.28,0,0.5-0.22,0.5-0.5V1.25C11.5,0.56,12.06,0,12.75,0S14,0.56,14,1.25V11.5 c0,0.28,0.22,0.5,0.5,0.5l0,0c0.28,0,0.5-0.22,0.5-0.5V2.75c0-0.69,0.56-1.25,1.25-1.25c0.69,0,1.25,0.56,1.25,1.25v8.75 c0,0.28,0.22,0.5,0.5,0.5l0,0c0.28,0,0.5-0.22,0.5-0.5V5.75c0-0.69,0.56-1.25,1.25-1.25S21,5.06,21,5.75V16z",
            }
        }
    }
}

pub fn blind_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cx: "9.93",
                        cy: "3.75",
                        r: "1.75",
                    }
                    path {
                        d: "M17.24,17.3l-3.74-6.48c0.25-0.12,0.43-0.35,0.43-0.65c0-0.37-0.28-0.68-0.65-0.73c-2.42-0.29-2.44-2.19-3.46-3.01 c-0.26-0.21-0.57-0.34-0.89-0.4C8.56,5.97,8.16,6,7.79,6.15L4.85,7.4C4.29,7.63,3.93,8.18,3.93,8.78v1.46 c0,0.41,0.34,0.75,0.75,0.75c0.41,0,0.75-0.34,0.75-0.75V8.78L6.79,8.2C6.58,8.96,6.43,9.7,6.43,10.49v4l-1.6,2.01 c-0.23,0.28-0.2,0.69,0.05,0.95c0.3,0.3,0.78,0.27,1.05-0.05l1.69-2.03c0.2-0.24,0.32-0.54,0.34-0.85l0.15-2.13l1.31,1.61v3.26 c0,0.41,0.34,0.75,0.75,0.75c0.41,0,0.75-0.34,0.75-0.75V13.5c0-0.33-0.11-0.65-0.31-0.91l-1.16-1.52 c-0.02-0.22-0.12-1.1,0.28-2.34c0.22,0.33,1.19,1.58,2.86,2.06l3.98,6.89c0.1,0.18,0.33,0.24,0.51,0.14 C17.29,17.71,17.35,17.48,17.24,17.3z",
                    }
                }
            }
        }
    }
}

pub fn blind_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    circle {
                        cy: "3.5",
                        r: "2",
                        cx: "11.41",
                    }
                    path {
                        d: "M12.04,7.12c-0.17-0.35-0.44-0.65-0.8-0.85C10.63,5.91,9.9,5.93,9.31,6.24l0-0.01L4.92,8.73 C4.3,9.08,3.91,9.74,3.91,10.46V13c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2.54l1.5-0.85c-0.32,1.1-0.5,2.24-0.5,3.39v5.33 l-2,2.67c-0.33,0.44-0.24,1.07,0.2,1.4l0,0c0.44,0.33,1.07,0.24,1.4-0.2l2.04-2.72c0.23-0.31,0.37-0.69,0.4-1.08l0.18-2.94 L10.91,18v4c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-4.87c0-0.41-0.13-0.81-0.36-1.15l-1.6-2.29c0,0,0,0,0-0.01 c-0.11-1.16,0.07-2.32,0.46-3.4c0.75,1.14,1.88,1.98,3.2,2.41l5.7,9.87c0.14,0.24,0.44,0.32,0.68,0.18l0,0 c0.24-0.14,0.32-0.44,0.18-0.68L15.92,13h0c0.54,0,0.98-0.44,0.98-0.98v-0.05c0-0.5-0.37-0.94-0.87-0.99 c-0.95-0.1-2.37-0.52-3.21-2.18",
                    }
                }
            }
        }
    }
}

pub fn boy_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,12.5h-0.5v3c0,0.28-0.22,0.5-0.5,0.5H9c-0.28,0-0.5-0.22-0.5-0.5v-3H8c-0.28,0-0.5-0.22-0.5-0.5V9 c0-0.83,0.67-1.5,1.5-1.5h2c0.83,0,1.5,0.67,1.5,1.5v3C12.5,12.28,12.28,12.5,12,12.5z M10,6.5c0.69,0,1.25-0.56,1.25-1.25 S10.69,4,10,4S8.75,4.56,8.75,5.25S9.31,6.5,10,6.5z",
                    }
                }
            }
        }
    }
}

pub fn boy_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,7.5c0.97,0,1.75-0.78,1.75-1.75S12.97,4,12,4s-1.75,0.78-1.75,1.75S11.03,7.5,12,7.5z M14,19c0,0.55-0.45,1-1,1h-2 c-0.55,0-1-0.45-1-1v-4c-0.55,0-1-0.45-1-1v-3.5c0-1.1,0.9-2,2-2h2c1.1,0,2,0.9,2,2V14c0,0.55-0.45,1-1,1V19z",
                    }
                }
            }
        }
    }
}

pub fn cake_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12.68 5.88c.7-.24 1.22-.9 1.3-1.64.05-.47-.05-.91-.28-1.27L12.42.75c-.19-.33-.67-.33-.87 0l-1.28 2.22c-.17.3-.27.65-.27 1.03 0 1.32 1.3 2.35 2.68 1.88zm3.85 10.04l-1-1-1.08 1.07c-1.3 1.3-3.58 1.31-4.89 0l-1.07-1.07-1.09 1.07C6.75 16.64 5.88 17 4.96 17c-.73 0-1.4-.23-1.96-.61V20c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-3.61c-.75.51-1.71.75-2.74.52-.66-.14-1.25-.51-1.73-.99zM18 9h-5V8c0-.55-.45-1-1-1s-1 .45-1 1v1H6c-1.66 0-3 1.34-3 3v1.46c0 .85.5 1.67 1.31 1.94.73.24 1.52.06 2.03-.46l2.14-2.13 2.13 2.13c.76.76 2.01.76 2.77 0l2.14-2.13 2.13 2.13c.43.43 1.03.63 1.65.55.99-.13 1.69-1.06 1.69-2.06v-1.42C21 10.34 19.66 9 18 9z",
            }
        }
    }
}

pub fn catching_pokemon_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M12.25,10c0,1.24-1.01,2.25-2.25,2.25S7.75,11.24,7.75,10S8.76,7.75,10,7.75S12.25,8.76,12.25,10z M18,10c0,4.42-3.58,8-8,8 s-8-3.58-8-8s3.58-8,8-8S18,5.58,18,10z M16.5,10h-3.25c0-1.79-1.46-3.25-3.25-3.25S6.75,8.21,6.75,10H3.5c0,3.58,2.92,6.5,6.5,6.5 S16.5,13.58,16.5,10z",
            }
        }
    }
}

pub fn catching_pokemon_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14.5,12c0,1.38-1.12,2.5-2.5,2.5c-1.38,0-2.5-1.12-2.5-2.5s1.12-2.5,2.5-2.5C13.38,9.5,14.5,10.62,14.5,12z M22,12 c0,5.52-4.48,10-10,10C6.48,22,2,17.52,2,12S6.48,2,12,2C17.52,2,22,6.48,22,12z M20,12h-4c0-2.21-1.79-4-4-4c-2.21,0-4,1.79-4,4H4 c0,4.41,3.59,8,8,8C16.41,20,20,16.41,20,12z",
            }
        }
    }
}

pub fn clean_hands_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                path {
                    d: "M14.99,7l1.37-0.63L16.99,5l0.63,1.37L18.99,7l-1.37,0.63L16.99,9l-0.63-1.37L14.99,7z M20,14c1.1,0,2-0.9,2-2 c0-0.78-0.99-2.44-1.58-3.36c-0.2-0.31-0.64-0.31-0.84,0C18.99,9.56,18,11.22,18,12C18,13.1,18.9,14,20,14z M9.24,9.5L15,11.65V11 c0-2.42-1.72-4.44-4-4.9V4h2c0.35,0,0.68,0.06,1,0.18c0.37,0.13,0.78,0.05,1.05-0.22l0,0c0.51-0.51,0.34-1.39-0.33-1.64 C14.19,2.11,13.61,2,13,2H8.5c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1H9v2.11C7.22,6.48,5.8,7.79,5.25,9.5H9.24z M3,11L3,11 c-1.1,0-2,0.9-2,2v7c0,1.1,0.9,2,2,2h0c1.1,0,2-0.9,2-2v-7C5,11.9,4.1,11,3,11z M19.99,17h-6.83c-0.11,0-0.22-0.02-0.33-0.06 l-1.47-0.51c-0.26-0.09-0.39-0.37-0.3-0.63l0,0c0.09-0.26,0.38-0.4,0.64-0.3l1.12,0.43c0.11,0.04,0.24,0.07,0.36,0.07h2.63 c0.65,0,1.18-0.53,1.18-1.18v0c0-0.49-0.31-0.93-0.77-1.11L9.3,11.13C9.08,11.04,8.84,11,8.6,11H7v9.02l6.37,1.81 c0.41,0.12,0.85,0.1,1.25-0.05L22,19l0,0C22,17.89,21.1,17,19.99,17z",
                }
            }
        }
    }
}

pub fn co2_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M11.5,8h-2C9.22,8,9,8.22,9,8.5v3C9,11.78,9.22,12,9.5,12h2c0.28,0,0.5-0.22,0.5-0.5v-3C12,8.22,11.78,8,11.5,8z M11,11h-1 V9h1V11z M16.5,14h-2c-0.28,0-0.5-0.22-0.5-0.5V12c0-0.28,0.22-0.5,0.5-0.5H16V11h-1.5c-0.28,0-0.5-0.22-0.5-0.5v0 c0-0.28,0.22-0.5,0.5-0.5h2c0.28,0,0.5,0.22,0.5,0.5V12c0,0.28-0.22,0.5-0.5,0.5H15V13h1.5c0.28,0,0.5,0.22,0.5,0.5v0 C17,13.78,16.78,14,16.5,14z M7,8.83c0,0.28-0.22,0.5-0.5,0.5C6.28,9.33,6.1,9.19,6.03,9H5v2l1.03,0c0.07-0.2,0.25-0.34,0.47-0.34 c0.28,0,0.5,0.22,0.5,0.5v0.17C7,11.7,6.7,12,6.34,12H4.67C4.3,12,4,11.7,4,11.34V8.67C4,8.3,4.3,8,4.67,8h1.66C6.7,8,7,8.3,7,8.67 V8.83z",
            }
        }
    }
}

pub fn co2_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14,9h-3c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-4C15,9.45,14.55,9,14,9z M13.5,13.5h-2v-3h2V13.5z M20.5,15.5h-2v1h2.25c0.41,0,0.75,0.34,0.75,0.75l0,0c0,0.41-0.34,0.75-0.75,0.75H18c-0.55,0-1-0.45-1-1v-1.5c0-0.55,0.45-1,1-1h2 v-1h-2.25c-0.41,0-0.75-0.34-0.75-0.75v0c0-0.41,0.34-0.75,0.75-0.75h2.75c0.55,0,1,0.45,1,1v1.5C21.5,15.05,21.05,15.5,20.5,15.5z M8,14c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1v0.25C8,10.66,7.66,11,7.25,11 c-0.33,0-0.6-0.21-0.71-0.5c0,0-2.04,0-2.04,0v3l2.04,0c0.1-0.29,0.38-0.5,0.71-0.5C7.66,13,8,13.34,8,13.75V14z",
            }
        }
    }
}

pub fn compost_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M9.5,10.12c0-1.48-1.7-1.93-2.73-2.06C6.44,8.02,6.16,8.3,6.21,8.63C6.23,8.8,6.25,9,6.24,9.25l0.01,0.88 c0,0.9,0.73,1.62,1.62,1.62c0.42,0,0.8-0.16,1.09-0.42c0.16,0.4,0.4,1.1,0.42,1.72c-1.06,0.16-1.96,0.73-2.46,1.51 C5.46,13.57,4.5,11.9,4.5,10c0-3.03,2.47-5.5,5.5-5.5l0,0.85c0,0.44,0.52,0.66,0.84,0.37l2.27-2.1c0.21-0.2,0.21-0.53,0-0.73 l-2.27-2.1C10.52,0.48,10,0.71,10,1.15L10,2c-4.42,0-8,3.58-8,8s3.58,8,8,8c4.24,0,7.71-3.3,7.98-7.46C18,10.25,17.77,10,17.48,10 h-1.5c-0.26,0-0.48,0.2-0.5,0.45c-0.14,1.71-1.06,3.2-2.41,4.11c-0.49-0.77-1.36-1.33-2.39-1.5c0.11-0.95,0.33-1.57,0.63-2.38 C11.63,10.88,12,11,12.4,11c1.93,0,2.43-2.26,2.56-3.8c0.03-0.39-0.38-0.67-0.73-0.49C13.93,6.88,13.53,7,13,7h-0.6 c-1.1,0-2,0.9-2,2c0,0.38,0.11,0.74,0.29,1.04c0.14-0.15,1.05-1.09,1.81-1.54c-0.48,0.4-1.78,1.93-2.5,3.71 C9.42,11.08,8.51,10.17,8.25,10c0.44,0.17,0.99,0.58,1.15,0.69C9.46,10.51,9.5,10.32,9.5,10.12z",
                }
            }
        }
    }
}

pub fn compost_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11.73,21.5c-4.95-0.14-9.08-4.27-9.22-9.22C2.35,6.91,6.67,2.5,12,2.5V1.21c0-0.45,0.54-0.67,0.85-0.35l2.79,2.79 c0.2,0.2,0.2,0.51,0,0.71l-2.79,2.79C12.54,7.46,12,7.24,12,6.79V5.5c-3.58,0-6.5,2.92-6.5,6.5c0,2.21,1.11,4.17,2.81,5.35 c0.51-0.92,1.63-1.62,2.98-1.8c-0.09-0.69-0.26-1.42-0.49-2.03c-0.33,0.28-0.75,0.46-1.22,0.48c-1.14,0.05-2.08-0.99-2.08-2.13 l0-0.86c0-0.29-0.05-0.57-0.14-0.83C7.24,9.84,7.5,9.49,7.86,9.52c1.3,0.09,3.6,0.52,3.64,2.48c0,0.29-0.06,0.56-0.17,0.8 C10.91,12.48,10.47,12.2,10,12c0.58,0.43,1.37,1.37,2,2.6c0.67-1.62,1.68-3.27,3-4.6c-0.76,0.52-1.47,1.12-2.13,1.81 c-0.26-0.42-0.4-0.93-0.36-1.47C12.59,9,13.79,8,15.13,8L16,8c0.56,0,0.97-0.14,1.28-0.31c0.34-0.19,0.76,0.05,0.75,0.44 C17.99,9.87,17.56,13,15,13c-0.49,0-0.94-0.14-1.32-0.38c-0.24,0.64-0.59,1.76-0.76,2.96c1.26,0.22,2.28,0.89,2.77,1.77 c1.57-1.09,2.64-2.85,2.79-4.87C18.5,12.22,18.71,12,18.98,12l1.82,0c0.47,0,0.71,0.24,0.69,0.52 C21.21,17.61,16.91,21.64,11.73,21.5z",
            }
        }
    }
}

pub fn connect_without_contact_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7,4c0-1.11-0.89-2-2-2S3,2.89,3,4s0.89,2,2,2S7,5.11,7,4z M10.19,4.5L10.19,4.5c-0.41,0-0.76,0.25-0.92,0.63 C8.83,6.23,7.76,7,6.5,7h-3C2.67,7,2,7.67,2,8.5V11h6V8.74c1.43-0.45,2.58-1.53,3.12-2.91C11.38,5.19,10.88,4.5,10.19,4.5z M19,17 c1.11,0,2-0.89,2-2s-0.89-2-2-2s-2,0.89-2,2S17.89,17,19,17z M20.5,18h-3c-1.26,0-2.33-0.77-2.77-1.87 c-0.15-0.38-0.51-0.63-0.92-0.63h0c-0.69,0-1.19,0.69-0.94,1.33c0.55,1.38,1.69,2.46,3.12,2.91V22h6v-2.5C22,18.67,21.33,18,20.5,18 z M17.25,11.09c0,0,0-0.01,0.01,0c-1.06,0.27-1.9,1.11-2.17,2.17c0,0,0-0.01,0-0.01C14.98,13.68,14.58,14,14.11,14 c-0.55,0-1-0.45-1-1c0-0.05,0.02-0.14,0.02-0.14c0.43-1.85,1.89-3.31,3.75-3.73c0.04,0,0.08-0.01,0.12-0.01c0.55,0,1,0.45,1,1 C18,10.58,17.68,10.98,17.25,11.09z M18,6.06c0,0.51-0.37,0.92-0.86,0.99c0,0,0,0,0,0c-3.19,0.39-5.7,2.91-6.09,6.1c0,0,0,0,0,0 C10.98,13.63,10.56,14,10.06,14c-0.55,0-1-0.45-1-1c0-0.02,0-0.04,0-0.06c0-0.01,0-0.02,0-0.03c0.5-4.12,3.79-7.38,7.92-7.85 c0,0,0.01,0,0.01,0C17.55,5.06,18,5.51,18,6.06z",
            }
        }
    }
}

pub fn construction_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M11.4,12.13l3.18,3.18c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41l-3.18-3.18L11.4,12.13z",
                    }
                    path {
                        d: "M15.19,8.78c0.74-0.31,1.3-0.96,1.48-1.74c0.16-0.72,0-1.4-0.35-1.94l-2.08,2.08l-0.71-0.71l2.08-2.08 c-0.54-0.35-1.21-0.51-1.93-0.35c-0.78,0.17-1.43,0.74-1.74,1.47c-0.29,0.68-0.24,1.32-0.02,1.88L10.69,8.6L9.63,7.54l0.35-0.35 c0.2-0.2,0.2-0.51,0-0.71L8.93,5.42L10.34,4C9.56,3.22,8.29,3.22,7.51,4L5.04,6.48c-0.2,0.2-0.2,0.51,0,0.71l0.71,0.71l-2.22,0 c-0.22,0-0.33,0.27-0.18,0.43l2.68,2.68c0.16,0.16,0.43,0.05,0.43-0.18L6.46,8.6l0.7,0.7c0.2,0.2,0.51,0.2,0.71,0l0.35-0.35 l1.06,1.06L5.39,13.9C5,14.29,5,14.93,5.39,15.32l0,0c0.39,0.39,1.02,0.39,1.41,0l6.52-6.52C13.87,9.02,14.51,9.06,15.19,8.78z",
                    }
                }
            }
        }
    }
}

pub fn construction_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M20.99,17.99l-4.94-4.94l-2.12,2.12l4.94,4.94c0.59,0.59,1.54,0.59,2.12,0C21.57,19.52,21.57,18.57,20.99,17.99z",
                    }
                    path {
                        d: "M17.65,10c1.93,0,3.5-1.57,3.5-3.5c0-0.58-0.16-1.12-0.41-1.6l-2.7,2.7l-1.49-1.49l2.7-2.7C18.77,3.16,18.23,3,17.65,3 c-1.93,0-3.5,1.57-3.5,3.5c0,0.41,0.08,0.8,0.21,1.16l-1.85,1.85l-1.78-1.78l0,0c0.39-0.39,0.39-1.02,0-1.41l-0.71-0.71l2.12-2.12 c-1.17-1.17-3.07-1.17-4.24,0L5.08,6.32c-0.39,0.39-0.39,1.02,0,1.41l0.71,0.71H3.25c-0.19,0-0.37,0.07-0.5,0.21 c-0.28,0.28-0.28,0.72,0,1l2.54,2.54c0.28,0.28,0.72,0.28,1,0c0.13-0.13,0.21-0.31,0.21-0.5V9.15L7.2,9.85 c0.39,0.39,1.02,0.39,1.41,0l1.78,1.78l-6.35,6.35c-0.59,0.59-0.59,1.54,0,2.12v0c0.59,0.59,1.54,0.59,2.12,0L16.48,9.79 C16.85,9.92,17.24,10,17.65,10z",
                    }
                }
            }
        }
    }
}

pub fn cookie_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M17.6,9.16c-1.11-0.21-2.16-1.26-2-2.59c0.04-0.34-0.21-0.62-0.55-0.59c-1.93,0.17-3.56-1.44-3.44-3.37 c0.02-0.25-0.14-0.48-0.38-0.52C5.69,1.2,2,5.85,2,10c0,4.42,3.58,8,8,8c4.56,0,8.2-3.81,7.99-8.37 C17.98,9.41,17.82,9.2,17.6,9.16z M6.75,12.5c-0.69,0-1.25-0.56-1.25-1.25S6.06,10,6.75,10S8,10.56,8,11.25S7.44,12.5,6.75,12.5z M8.75,8.5C8.06,8.5,7.5,7.94,7.5,7.25C7.5,6.56,8.06,6,8.75,6S10,6.56,10,7.25C10,7.94,9.44,8.5,8.75,8.5z M12.5,13.25 c-0.41,0-0.75-0.34-0.75-0.75s0.34-0.75,0.75-0.75c0.41,0,0.75,0.34,0.75,0.75S12.92,13.25,12.5,13.25z",
                    }
                }
            }
        }
    }
}

pub fn cookie_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M21.27,10.9c-1.21-0.33-2.31-1.46-2.29-2.89c0.01-0.56-0.4-1.02-0.96-1.01C15.83,7.03,14,5.22,14,3.02 c0-0.49-0.35-0.9-0.84-0.96C6.53,1.22,2,6.81,2,12c0,5.52,4.48,10,10,10c5.61,0,10.11-4.62,10-10.18 C21.99,11.38,21.69,11.01,21.27,10.9z M8.5,15C7.67,15,7,14.33,7,13.5S7.67,12,8.5,12s1.5,0.67,1.5,1.5S9.33,15,8.5,15z M10.5,10 C9.67,10,9,9.33,9,8.5S9.67,7,10.5,7S12,7.67,12,8.5S11.33,10,10.5,10z M15,16c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C16,15.55,15.55,16,15,16z",
                    }
                }
            }
        }
    }
}

pub fn coronavirus_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.25,10.5c-0.41,0-0.75,0.34-0.75,0.75h-1.54c-0.15-1.37-0.69-2.63-1.52-3.65l1.09-1.09l0.01,0.01 c0.29,0.29,0.77,0.29,1.06,0s0.29-0.77,0-1.06L18.54,4.4c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.76-0.01,1.05l-1.09,1.09 c-1.02-0.82-2.27-1.36-3.64-1.51V3.5h0.01c0.41,0,0.75-0.34,0.75-0.75C13.5,2.34,13.16,2,12.75,2h-1.5c-0.41,0-0.75,0.34-0.75,0.75 c0,0.41,0.33,0.74,0.74,0.75v1.55C9.87,5.19,8.62,5.74,7.6,6.56L6.51,5.47l0.01-0.01c0.29-0.29,0.29-0.77,0-1.06 c-0.29-0.29-0.77-0.29-1.06,0L4.4,5.46c-0.29,0.29-0.29,0.77,0,1.06c0.29,0.29,0.76,0.29,1.05,0.01l1.09,1.09 c-0.82,1.02-1.36,2.26-1.5,3.63H3.5c0-0.41-0.34-0.75-0.75-0.75C2.34,10.5,2,10.84,2,11.25v1.5c0,0.41,0.34,0.75,0.75,0.75 c0.41,0,0.75-0.34,0.75-0.75h1.54c0.15,1.37,0.69,2.61,1.5,3.63l-1.09,1.09c-0.29-0.29-0.76-0.28-1.05,0.01 c-0.29,0.29-0.29,0.77,0,1.06l1.06,1.06c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06l-0.01-0.01l1.09-1.09 c1.02,0.82,2.26,1.36,3.63,1.51v1.55c-0.41,0.01-0.74,0.34-0.74,0.75c0,0.41,0.34,0.75,0.75,0.75h1.5c0.41,0,0.75-0.34,0.75-0.75 c0-0.41-0.34-0.75-0.75-0.75h-0.01v-1.54c1.37-0.14,2.62-0.69,3.64-1.51l1.09,1.09c-0.29,0.29-0.28,0.76,0.01,1.05 c0.29,0.29,0.77,0.29,1.06,0l1.06-1.06c0.29-0.29,0.29-0.77,0-1.06c-0.29-0.29-0.77-0.29-1.06,0l-0.01,0.01l-1.09-1.09 c0.82-1.02,1.37-2.27,1.52-3.65h1.54c0,0.41,0.34,0.75,0.75,0.75c0.41,0,0.75-0.34,0.75-0.75v-1.5C22,10.84,21.66,10.5,21.25,10.5z M13.75,8c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S13.2,8,13.75,8z M12,13c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C13,12.55,12.55,13,12,13z M10.25,8c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S9.7,8,10.25,8z M8.5,13c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C9.5,12.55,9.05,13,8.5,13z M10.25,16c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C11.25,15.55,10.8,16,10.25,16z M13.75,16c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C14.75,15.55,14.3,16,13.75,16z M14.5,12 c0-0.55,0.45-1,1-1s1,0.45,1,1c0,0.55-0.45,1-1,1S14.5,12.55,14.5,12z",
            }
        }
    }
}

pub fn cruelty_free_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M13.89,12c-0.21-0.17-0.39-0.39-0.68-0.72c0.54-0.73,2.29-3.37,2.29-6.77C15.5,2.94,15.01,2,14,2c-1.23,0-3.17,1.65-4,4.78 C9.17,3.65,7.23,2,6,2C4.99,2,4.5,2.94,4.5,4.5c0,3.4,1.75,6.04,2.29,6.77C6.5,11.61,6.32,11.82,6.11,12 c-0.37,0.31-1.36,1.13-1.36,2.5c0,1.79,1.46,3.25,3.25,3.25c1.19,0,2-0.43,2-0.43s0.81,0.43,2,0.43c1.79,0,3.25-1.46,3.25-3.25 C15.25,13.13,14.26,12.31,13.89,12z M7.87,10.19C7.08,9.04,6,6.96,6,4.5C6,4,6.06,3.69,6.12,3.53C6.77,3.75,8.84,5.57,8.9,9.76 C8.29,9.88,7.87,10.19,7.87,10.19z M8.8,13.8c-0.22,0-0.4-0.27-0.4-0.6c0-0.33,0.18-0.6,0.4-0.6s0.4,0.27,0.4,0.6 C9.2,13.53,9.02,13.8,8.8,13.8z M10,16c-0.44,0-0.8-0.58-0.8-0.8c0-0.22,0.36-0.4,0.8-0.4s0.8,0.18,0.8,0.4 C10.8,15.42,10.44,16,10,16z M11.2,13.8c-0.22,0-0.4-0.27-0.4-0.6c0-0.33,0.18-0.6,0.4-0.6s0.4,0.27,0.4,0.6 C11.6,13.53,11.42,13.8,11.2,13.8z M11.1,9.76c0.06-4.19,2.12-6.01,2.78-6.23C13.94,3.69,14,4,14,4.5c0,2.46-1.08,4.54-1.87,5.69 C12.13,10.19,11.71,9.88,11.1,9.76z",
            }
        }
    }
}

pub fn cruelty_free_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M16.84,14.52c-0.26-0.19-0.62-0.63-0.79-0.84C17.24,12.01,19,8.87,19,5c0-1.95-0.74-3-2-3c-1.54,0-3.96,2.06-5,5.97 C10.96,4.06,8.54,2,7,2C5.74,2,5,3.05,5,5c0,3.87,1.76,7.01,2.95,8.68c-0.17,0.21-0.53,0.65-0.79,0.84 c-0.5,0.41-1.66,1.37-1.66,2.98c0,2.21,1.79,4,4,4c1.55,0,2.5-0.56,2.5-0.56s0.95,0.56,2.5,0.56c2.21,0,4-1.79,4-4 C18.5,15.89,17.34,14.93,16.84,14.52z M9.35,12.2C8.34,10.7,7,8.12,7,5c0-0.49,0.06-0.8,0.12-0.97c0.94,0.31,3.24,2.71,3.38,7.64 C10.03,11.79,9.66,11.97,9.35,12.2z M10.5,16.75c-0.28,0-0.5-0.34-0.5-0.75c0-0.41,0.22-0.75,0.5-0.75S11,15.59,11,16 C11,16.41,10.78,16.75,10.5,16.75z M12,19.5c-0.55,0-1-0.72-1-1c0-0.28,0.45-0.5,1-0.5s1,0.22,1,0.5C13,18.78,12.55,19.5,12,19.5z M13.5,16.75c-0.28,0-0.5-0.34-0.5-0.75c0-0.41,0.22-0.75,0.5-0.75S14,15.59,14,16C14,16.41,13.78,16.75,13.5,16.75z M13.5,11.67 c0.14-4.93,2.44-7.33,3.38-7.64C16.94,4.2,17,4.51,17,5c0,3.12-1.34,5.7-2.35,7.2C14.34,11.97,13.97,11.79,13.5,11.67z",
            }
        }
    }
}

pub fn cyclone_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,13.5c1.93,0,3.5-1.57,3.5-3.5S11.93,6.5,10,6.5S6.5,8.07,6.5,10S8.07,13.5,10,13.5z M10,8c1.1,0,2,0.9,2,2 c0,1.1-0.9,2-2,2s-2-0.9-2-2C8,8.9,8.9,8,10,8z",
                    }
                    path {
                        d: "M18,4.67L18,4.67c0-0.34-0.23-0.65-0.56-0.74C14.15,3.05,11.17,3,10,3C8,3,6.2,3.84,4.93,5.18c0-0.03,0.15-0.97,0.48-2.22 C5.54,2.48,5.17,2,4.67,2h0C4.33,2,4.02,2.23,3.93,2.56C3.05,5.85,3,8.84,3,10c0,2,0.84,3.8,2.18,5.07 c-0.03,0-0.97-0.15-2.22-0.48C2.48,14.46,2,14.83,2,15.33v0c0,0.34,0.23,0.65,0.56,0.74C5.85,16.95,8.83,17,10,17 c2,0,3.8-0.84,5.07-2.18c0,0.03-0.15,0.97-0.48,2.22C14.46,17.52,14.83,18,15.33,18h0c0.34,0,0.65-0.23,0.74-0.56 C16.95,14.15,17,11.17,17,10c0-2-0.84-3.8-2.18-5.07c0.03,0,0.97,0.15,2.22,0.48C17.52,5.54,18,5.17,18,4.67z M15.5,10 c0,3.03-2.47,5.5-5.5,5.5S4.5,13.03,4.5,10S6.97,4.5,10,4.5S15.5,6.97,15.5,10z",
                    }
                }
            }
        }
    }
}

pub fn cyclone_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,8c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4c2.21,0,4-1.79,4-4C16,9.79,14.21,8,12,8z M12,14c-1.1,0-2-0.9-2-2 c0-1.1,0.9-2,2-2s2,0.9,2,2C14,13.1,13.1,14,12,14z",
                    }
                    path {
                        d: "M22,6.11L22,6.11c0-0.46-0.3-0.86-0.74-0.97C19.23,4.6,16.03,4,12,4C9.85,4,7.89,4.86,6.46,6.24 c0.1-0.65,0.28-1.69,0.62-2.96C7.25,2.64,6.78,2,6.11,2h0C5.66,2,5.26,2.3,5.14,2.74C4.6,4.77,4,7.97,4,12 c0,2.15,0.86,4.11,2.24,5.54c-0.65-0.1-1.69-0.28-2.96-0.62C2.64,16.75,2,17.22,2,17.89v0c0,0.46,0.3,0.86,0.74,0.97 C4.77,19.4,7.97,20,12,20c2.15,0,4.11-0.86,5.54-2.24c-0.1,0.65-0.28,1.69-0.62,2.96c-0.17,0.64,0.3,1.28,0.97,1.28h0 c0.46,0,0.86-0.3,0.97-0.74C19.4,19.23,20,16.03,20,12c0-2.15-0.86-4.11-2.24-5.54c0.65,0.1,1.69,0.28,2.96,0.62 C21.36,7.25,22,6.78,22,6.11z M12,18c-3.31,0-6-2.69-6-6s2.69-6,6-6s6,2.69,6,6S15.31,18,12,18z",
                    }
                }
            }
        }
    }
}

pub fn deck_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10.58,3.42c-0.35-0.25-0.81-0.25-1.16,0L3,8h6.5v8.5c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5V8H17L10.58,3.42z",
                    }
                    path {
                        d: "M7.5,13H4.92l-0.47-2.51c-0.05-0.27-0.31-0.45-0.58-0.4c-0.27,0.05-0.45,0.31-0.4,0.58L4,13.5v3C4,16.78,4.22,17,4.5,17 S5,16.78,5,16.5V14h2v2.5C7,16.78,7.22,17,7.5,17S8,16.78,8,16.5v-3C8,13.22,7.78,13,7.5,13z",
                    }
                    path {
                        d: "M16.13,10.09c-0.27-0.05-0.53,0.13-0.58,0.4L15.08,13H12.5c-0.28,0-0.5,0.22-0.5,0.5v3c0,0.28,0.22,0.5,0.5,0.5 s0.5-0.22,0.5-0.5V14h2v2.5c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5v-3l0.53-2.82C16.58,10.4,16.4,10.14,16.13,10.09z",
                    }
                }
            }
        }
    }
}

pub fn deck_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M20.41,9c0.49,0,0.69-0.63,0.29-0.91L13.15,2.8c-0.69-0.48-1.61-0.48-2.29,0L3.3,8.09C2.9,8.37,3.1,9,3.59,9H11v12 c0,0.55,0.45,1,1,1s1-0.45,1-1V9H20.41z",
                    }
                    path {
                        d: "M8,16H4.9l-0.57-3.02c-0.1-0.54-0.62-0.9-1.17-0.8c-0.54,0.1-0.9,0.62-0.8,1.17L3,16.74V21c0,0.55,0.45,1,1,1h0.01 c0.55,0,1-0.44,1-0.99L5.02,18H7v3c0,0.55,0.45,1,1,1s1-0.45,1-1v-4C9,16.45,8.55,16,8,16z",
                    }
                    path {
                        d: "M20.84,12.18c-0.54-0.1-1.06,0.26-1.17,0.8L19.1,16H16c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1s1-0.45,1-1v-3h1.98 l0.02,3.01c0,0.55,0.45,0.99,1,0.99H20c0.55,0,1-0.45,1-1v-4.26l0.64-3.39C21.74,12.81,21.38,12.28,20.84,12.18z",
                    }
                }
            }
        }
    }
}

pub fn diversity_1_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cx: "3.5",
                        cy: "12.5",
                        r: "1.5",
                    }
                    circle {
                        cy: "12.5",
                        r: "1.5",
                        cx: "16.5",
                    }
                    path {
                        d: "M13.49,14.23c-0.97-0.42-2.15-0.73-3.49-0.73c-1.34,0-2.52,0.31-3.49,0.73C5.59,14.63,5,15.53,5,16.53l0,0.97 C5,17.78,5.22,18,5.5,18h9c0.28,0,0.5-0.22,0.5-0.5l0-0.97C15,15.53,14.41,14.63,13.49,14.23z",
                    }
                    path {
                        d: "M19.15,15.61C18.44,15.27,17.54,15,16.5,15c-0.29,0-0.56,0.02-0.82,0.06c0.21,0.45,0.32,0.95,0.32,1.47V18h3.5 c0.28,0,0.5-0.22,0.5-0.5l0-0.54C20,16.39,19.67,15.86,19.15,15.61z",
                    }
                    path {
                        d: "M0.85,15.61C0.33,15.86,0,16.39,0,16.96l0,0.54C0,17.78,0.22,18,0.5,18H4v-1.47c0-0.52,0.11-1.02,0.32-1.47 C4.06,15.02,3.79,15,3.5,15C2.46,15,1.56,15.27,0.85,15.61z",
                    }
                    path {
                        d: "M7.5,10.5c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5C12.5,9.12,11.38,8,10,8S7.5,9.12,7.5,10.5z",
                    }
                    path {
                        d: "M2.33,10.3C1.79,9.36,1.5,8.44,1.5,7.5c0-2.24,1.76-4,4-4c2.53,0,3.77,2.04,4.5,2.9c0.62-0.73,1.91-2.9,4.5-2.9 c2.24,0,4,1.76,4,4c0,0.94-0.29,1.86-0.83,2.8c0.46,0.25,0.84,0.63,1.07,1.1C19.54,10.12,20,8.86,20,7.5C20,4.42,17.58,2,14.5,2 c-1.74,0-3.41,0.81-4.5,2.09C8.91,2.81,7.24,2,5.5,2C2.42,2,0,4.42,0,7.5c0,1.36,0.46,2.62,1.26,3.9 C1.5,10.93,1.87,10.55,2.33,10.3z",
                    }
                }
            }
        }
    }
}

pub fn diversity_1_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    circle {
                        r: "2",
                        cy: "14",
                        cx: "4",
                    }
                    path {
                        d: "M1.22,17.58C0.48,17.9,0,18.62,0,19.43L0,20c0,0.55,0.45,1,1,1l3.5,0v-1.61c0-0.83,0.23-1.61,0.63-2.29 C4.76,17.04,4.39,17,4,17C3.01,17,2.07,17.21,1.22,17.58z",
                    }
                    circle {
                        cy: "14",
                        cx: "20",
                        r: "2",
                    }
                    path {
                        d: "M22.78,17.58C21.93,17.21,20.99,17,20,17c-0.39,0-0.76,0.04-1.13,0.1c0.4,0.68,0.63,1.46,0.63,2.29V21l3.5,0 c0.55,0,1-0.45,1-1l0-0.57C24,18.62,23.52,17.9,22.78,17.58z",
                    }
                    path {
                        d: "M16.24,16.65c-1.17-0.52-2.61-0.9-4.24-0.9c-1.63,0-3.07,0.39-4.24,0.9C6.68,17.13,6,18.21,6,19.39L6,20 c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1l0-0.61C18,18.21,17.32,17.13,16.24,16.65z",
                    }
                    path {
                        d: "M9,12c0,1.66,1.34,3,3,3s3-1.34,3-3c0-1.66-1.34-3-3-3S9,10.34,9,12z",
                    }
                    path {
                        d: "M2.48,10.86C2.17,10.1,2,9.36,2,8.6C2,6.02,4.02,4,6.6,4c2.68,0,3.82,1.74,5.4,3.59C13.57,5.76,14.7,4,17.4,4 C19.98,4,22,6.02,22,8.6c0,0.76-0.17,1.5-0.48,2.26c0.65,0.31,1.18,0.82,1.53,1.44C23.65,11.1,24,9.88,24,8.6 C24,4.9,21.1,2,17.4,2c-2.09,0-4.09,0.97-5.4,2.51C10.69,2.97,8.69,2,6.6,2C2.9,2,0,4.9,0,8.6c0,1.28,0.35,2.5,0.96,3.7 C1.31,11.68,1.84,11.17,2.48,10.86z",
                    }
                }
            }
        }
    }
}

pub fn diversity_2_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M7.99,13c0.3,0.27,0.75,0.26,1.03-0.03c0.3-0.3,0.29-0.8-0.03-1.09C6.74,9.86,5.33,8.48,5.52,7.05 c0.12-0.94,1.03-1.62,1.97-1.54C8.41,5.58,8.85,6.21,9.52,7h0.97c1.26-1.48,1.57-1.5,2.22-1.5c0.81,0,1.46,0.49,1.69,1.19 c0.27-0.03,0.52-0.05,0.77-0.05c0.27,0,0.53,0.02,0.78,0.05C15.67,5.15,14.34,4,12.7,4c-0.09,0-0.17,0.02-0.26,0.03 c0.12-0.55,0.08-1.16-0.33-1.81c-0.4-0.63-1.06-1.11-1.8-1.2C8.79,0.84,7.5,2.02,7.5,3.5c0,0.18,0.02,0.36,0.06,0.53 C7.47,4.02,7.39,4,7.3,4C5.45,4,4,5.45,4,7.3C4,9.27,5.58,10.84,7.99,13z",
                    }
                    path {
                        d: "M18.75,13.09c-0.16-0.09-0.32-0.16-0.49-0.21c0.05-0.07,0.11-0.13,0.15-0.21c0.92-1.6,0.39-3.58-1.21-4.51 c-1.71-0.99-3.85-0.4-6.93,0.61C9.9,8.89,9.68,9.29,9.78,9.67c0.11,0.42,0.55,0.65,0.96,0.52c2.72-0.89,4.56-1.41,5.72-0.74 c0.89,0.51,1.17,1.57,0.66,2.46c-0.53,0.91-1.33,0.97-2.41,1.17l-0.48,0.84c0.65,1.83,0.51,2.11,0.19,2.67 c-0.51,0.89-1.57,1.17-2.46,0.66c-0.18-0.11-0.35-0.25-0.5-0.42c-0.23,0.51-0.48,0.97-0.77,1.35c0.16,0.14,0.33,0.27,0.52,0.38 c1.6,0.92,3.58,0.39,4.51-1.21c0.04-0.08,0.07-0.16,0.1-0.24c0.13,0.12,0.27,0.22,0.43,0.32c1.2,0.69,2.72,0.28,3.42-0.92 C20.35,15.3,19.95,13.78,18.75,13.09z",
                    }
                    path {
                        d: "M11.19,11.36c-0.42-0.11-0.84,0.15-0.93,0.57c-0.63,2.97-1.12,4.87-2.44,5.43c-0.87,0.36-1.92-0.08-2.32-0.94 c-0.4-0.83-0.07-1.53,0.28-2.5l-0.48-0.84c-1.91-0.35-2.08-0.61-2.41-1.17c-0.51-0.89-0.23-1.95,0.66-2.46 C3.31,8.98,3.16,8.5,3.08,8.02c-0.1,0.04-0.2,0.08-0.29,0.13c-1.6,0.92-2.13,2.91-1.21,4.51c0.04,0.08,0.11,0.14,0.15,0.21 c-0.17,0.05-0.33,0.12-0.49,0.21c-1.2,0.69-1.61,2.22-0.92,3.42c0.69,1.2,2.22,1.61,3.42,0.92c0.16-0.09,0.3-0.2,0.43-0.32 c0.04,0.08,0.06,0.16,0.1,0.24c0.92,1.6,2.91,2.13,4.51,1.21c1.71-0.99,2.27-3.14,2.94-6.31C11.81,11.85,11.58,11.46,11.19,11.36z",
                    }
                }
            }
        }
    }
}

pub fn diversity_2_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M10.77,15.23L10.77,15.23c0.4-0.4,0.39-1.07-0.04-1.45C8.17,11.47,6.65,9.9,7.07,8.33C7.29,7.52,8.01,7,8.85,7 c1.11,0,1.54,0.65,2.68,2h0.93c1.12-1.31,1.53-2,2.68-2c0.87,0,1.55,0.54,1.77,1.32c0.35-0.04,0.68-0.06,1-0.06 c0.36,0,0.7,0.03,1.03,0.08C18.7,6.43,17.13,5,15.15,5c-0.12,0-0.23,0.03-0.35,0.04c0.26-0.71,0.31-1.53-0.18-2.43 C14.19,1.8,13.4,1.18,12.5,1.04C10.62,0.74,9,2.18,9,4c0,0.37,0.08,0.71,0.2,1.04C9.08,5.03,8.97,5,8.85,5C6.69,5,5,6.69,5,8.85 c0,2.23,1.73,4.01,4.4,6.41C9.79,15.62,10.4,15.6,10.77,15.23z",
                    }
                    path {
                        d: "M23.52,17.25c-0.54-0.87-1.27-1.23-2.01-1.36c0.07-0.1,0.15-0.18,0.21-0.28c1.08-1.87,0.46-4.18-1.41-5.26 c-1.93-1.11-4.34-0.5-7.75,0.6c-0.51,0.16-0.79,0.7-0.66,1.21v0c0.15,0.55,0.73,0.87,1.28,0.69c3.28-1.06,5.4-1.6,6.55-0.45 c0.59,0.59,0.68,1.48,0.26,2.21c-0.56,0.96-1.33,1-3.07,1.32l-0.47,0.81c0.58,1.62,0.97,2.33,0.39,3.32 c-0.53,0.91-1.61,1.2-2.53,0.68c-0.06-0.03-0.11-0.09-0.17-0.13c-0.3,0.67-0.64,1.24-1.03,1.73c0,0.01,0,0.01,0,0.01 c1.9,1.24,4.34,0.64,5.46-1.29c0.06-0.1,0.09-0.21,0.14-0.32c0.22,0.27,0.48,0.51,0.8,0.69c1.58,0.91,3.64,0.23,4.31-1.55 C24.14,19.02,24,18.03,23.52,17.25z",
                    }
                    path {
                        d: "M12.1,15.04c-0.72,3.38-1.32,5.48-2.89,5.9c-0.81,0.22-1.62-0.15-2.04-0.88c-0.56-0.96-0.2-1.66,0.39-3.32L7.1,15.93 c-1.7-0.31-2.5-0.33-3.07-1.32c-0.53-0.91-0.24-2,0.68-2.53c0.09-0.05,0.19-0.08,0.29-0.11c-0.27-0.42-0.5-0.87-0.67-1.37 c-0.24-0.11-0.36-0.16-0.6-0.26c-1.88,1.07-2.51,3.4-1.43,5.27c0.06,0.1,0.14,0.18,0.21,0.28c-0.75,0.13-1.48,0.49-2.01,1.36 c-0.48,0.78-0.62,1.77-0.3,2.63c0.68,1.78,2.74,2.46,4.31,1.55c0.32-0.18,0.58-0.42,0.8-0.69c0.05,0.11,0.08,0.22,0.14,0.32 c1.08,1.87,3.39,2.49,5.26,1.41c1.93-1.11,2.61-3.51,3.36-7.01c0.11-0.52-0.21-1.04-0.72-1.17l0,0 C12.78,14.14,12.22,14.48,12.1,15.04z",
                    }
                }
            }
        }
    }
}

pub fn diversity_3_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M7.13,11.23c0.65,0.95,1.73,1.52,2.87,1.52s2.22-0.57,2.87-1.52c0.36-0.53,0.93-0.86,1.56-0.94C13.91,9.78,12.01,9,10,9 c-2.02,0-3.91,0.79-4.43,1.29C6.19,10.37,6.76,10.7,7.13,11.23z",
                        }
                    }
                    circle {
                        r: "2.5",
                        cx: "10",
                        cy: "5.5",
                    }
                    circle {
                        cy: "7.5",
                        r: "2.5",
                        cx: "3.5",
                    }
                    circle {
                        cx: "16.5",
                        r: "2.5",
                        cy: "7.5",
                    }
                    path {
                        d: "M17.5,11l-2.78,0c-0.49,0-0.95,0.25-1.23,0.65c-0.81,1.17-2.13,1.85-3.49,1.85c-0.31,0-2.25-0.03-3.49-1.85 C6.27,11.3,5.84,11,5.28,11H2.5C1.67,11,1,11.67,1,12.5v3C1,15.78,1.22,16,1.5,16h5C6.78,16,7,15.78,7,15.5v-1.37 C7.89,14.68,8.92,15,10,15s2.11-0.32,3-0.87v1.37c0,0.28,0.22,0.5,0.5,0.5h5c0.28,0,0.5-0.22,0.5-0.5v-3 C19,11.67,18.33,11,17.5,11z",
                    }
                }
            }
        }
    }
}

pub fn diversity_3_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M6.32,13.01c0.96,0.02,1.85,0.5,2.45,1.34C9.5,15.38,10.71,16,12,16c1.29,0,2.5-0.62,3.23-1.66 c0.6-0.84,1.49-1.32,2.45-1.34C16.96,11.78,14.08,11,12,11C9.93,11,7.04,11.78,6.32,13.01z",
                    }
                    path {
                        d: "M4,13L4,13c1.66,0,3-1.34,3-3c0-1.66-1.34-3-3-3s-3,1.34-3,3C1,11.66,2.34,13,4,13z",
                    }
                    path {
                        d: "M20,13L20,13c1.66,0,3-1.34,3-3c0-1.66-1.34-3-3-3s-3,1.34-3,3C17,11.66,18.34,13,20,13z",
                    }
                    path {
                        d: "M12,10c1.66,0,3-1.34,3-3c0-1.66-1.34-3-3-3S9,5.34,9,7C9,8.66,10.34,10,12,10z",
                    }
                    path {
                        d: "M21,14h-3.27c-0.77,0-1.35,0.45-1.68,0.92C16.01,14.98,14.69,17,12,17c-1.43,0-3.03-0.64-4.05-2.08 C7.56,14.37,6.95,14,6.27,14L3,14c-1.1,0-2,0.9-2,2v3c0,0.55,0.45,1,1,1h5c0.55,0,1-0.45,1-1v-1.26c1.15,0.8,2.54,1.26,4,1.26 s2.85-0.46,4-1.26V19c0,0.55,0.45,1,1,1h5c0.55,0,1-0.45,1-1v-3C23,14.9,22.1,14,21,14z",
                    }
                }
            }
        }
    }
}

pub fn domain_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 7V5c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2h-8zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm0-4H4V9h2v2zm0-4H4V5h2v2zm4 12H8v-2h2v2zm0-4H8v-2h2v2zm0-4H8V9h2v2zm0-4H8V5h2v2zm9 12h-7v-2h2v-2h-2v-2h2v-2h-2V9h7c.55 0 1 .45 1 1v8c0 .55-.45 1-1 1zm-1-8h-2v2h2v-2zm0 4h-2v2h2v-2z",
            }
        }
    }
}

pub fn domain_add_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M5,15.5H3.5V14H5V15.5z M5,12.33H3.5v-1.5H5V12.33z M5,9.17H3.5v-1.5H5V9.17z M5,6H3.5V4.5H5V6z M8.5,15.5H7V14h1.5V15.5z M8.5,12.33H7v-1.5h1.5V12.33z M8.5,9.17H7v-1.5h1.5V9.17z M7,6V4.5h1.5V6H7z M18,12.5V7c0-0.55-0.45-1-1-1h-7V4c0-0.55-0.45-1-1-1 H3C2.45,3,2,3.45,2,4v12c0,0.55,0.45,1,1,1h10.5v-1.5H10v-1.67h1.5v-1.5H10v-1.67h1.5v-1.5H10V7.5h6.5v5H18z M13.5,9.17H15v1.5 h-1.5V9.17z M13.5,12.33H15v1.5h-1.5V12.33z M19.5,16.25c0-0.41-0.34-0.75-0.75-0.75H18v-0.75c0-0.41-0.34-0.75-0.75-0.75 s-0.75,0.34-0.75,0.75v0.75h-0.75c-0.41,0-0.75,0.34-0.75,0.75c0,0.41,0.34,0.75,0.75,0.75h0.75v0.75c0,0.41,0.34,0.75,0.75,0.75 S18,18.16,18,17.75V17h0.75C19.16,17,19.5,16.66,19.5,16.25z",
                }
            }
        }
    }
}

pub fn domain_add_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6,19H4v-2h2V19z M6,15H4v-2h2V15z M6,11H4V9h2V11z M6,7H4V5h2V7z M10,19H8v-2h2V19z M10,15H8v-2h2V15z M10,11H8V9h2V11z M10,7H8V5h2V7z M16,19h-4v-2h2v-2h-2v-2h2v-2h-2V9h8v6h2V8c0-0.55-0.45-1-1-1h-9V4c0-0.55-0.45-1-1-1H3C2.45,3,2,3.45,2,4v16 c0,0.55,0.45,1,1,1h13V19z M18,11h-2v2h2V11z M18,15h-2v2h2V15z M24,20c0,0.55-0.45,1-1,1h-1v1c0,0.55-0.45,1-1,1s-1-0.45-1-1v-1 h-1c-0.55,0-1-0.45-1-1s0.45-1,1-1h1v-1c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1v1h1C23.55,19,24,19.45,24,20z",
                }
            }
        }
    }
}

pub fn downhill_skiing_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M12,3.75C12,2.78,12.78,2,13.75,2s1.75,0.78,1.75,1.75c0,0.97-0.79,1.75-1.75,1.75C12.79,5.5,12,4.72,12,3.75z M15.53,8.16 c-0.26-0.09-0.55,0.06-0.63,0.32l-0.23,0.69c-1.22-0.34-2.17-1.31-2.47-2.55l-0.24-0.97C11.72,4.67,10.61,4.2,9.75,4.7L6.83,6.38 c-0.89,0.5-1.13,1.7-0.37,2.43l3.1,2.94L8,15.8l-5.19-1.87c-0.26-0.09-0.55,0.04-0.64,0.3c-0.09,0.26,0.04,0.55,0.3,0.64l10.38,3.73 C13.52,18.86,14.24,19,15,19c0.84,0,1.63-0.17,2.36-0.48c0.32-0.14,0.4-0.56,0.15-0.81c-0.14-0.14-0.36-0.19-0.55-0.11 C16.36,17.86,15.7,18,15,18c-0.6,0-1.14-0.1-1.67-0.29l-0.35-0.12l1.9-5.83c-0.34-0.04-0.67-0.11-0.98-0.2l0,0l0,0l-1.85,5.7 l-2.62-0.94l1.68-4.37c0.14-0.37,0.05-0.79-0.23-1.07L8.74,8.74L10.9,7.5c0.61,1.68,2.09,2.93,3.89,3.23 c0.24,0.04,0.47-0.12,0.54-0.34l0.52-1.6C15.93,8.52,15.79,8.24,15.53,8.16z",
            }
        }
    }
}

pub fn downhill_skiing_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.5,4.5c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2S18.5,3.4,18.5,4.5z M15.78,20.9l0.76,0.27c0.62,0.21,1.27,0.33,1.96,0.33 c0.68,0,1.34-0.12,1.95-0.33c0.27-0.09,0.57-0.02,0.78,0.18l0,0c0.39,0.4,0.23,1.06-0.3,1.24C20.17,22.86,19.35,23,18.5,23 c-0.86,0-1.68-0.14-2.45-0.41L2.7,17.72c-0.39-0.14-0.59-0.57-0.45-0.95l0,0c0.14-0.39,0.57-0.6,0.96-0.45l6.19,2.25l1.72-4.44 L7.55,10.4C6.65,9.46,6.87,7.93,8,7.28l3.48-2.01c1.1-0.64,2.52-0.1,2.91,1.11l0.33,1.08c0.44,1.42,1.48,2.57,2.83,3.14l0.29-0.89 c0.13-0.39,0.55-0.61,0.94-0.48h0c0.4,0.13,0.61,0.55,0.48,0.95l-0.6,1.85c-0.17,0.52-0.72,0.82-1.24,0.65 c-2.02-0.63-3.64-2.15-4.42-4.1l-2.53,1.45l2.23,2.55c0.49,0.56,0.63,1.34,0.36,2.04l-1.78,4.63l3.09,1.12l2.1-6.44h0l0,0 c0.46,0.18,0.94,0.31,1.44,0.41L15.78,20.9z",
            }
        }
    }
}

pub fn edit_notifications_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                    d: "M16,14.75c0-0.41-0.34-0.75-0.75-0.75H15V9.53l-1.88,1.88C12.75,11.79,12.24,12,11.71,12H11c-1.1,0-2-0.9-2-2V9.33 C9,8.8,9.21,8.29,9.59,7.91l3.12-3.12C12.19,4.47,11.62,4.23,11,4.1V3.03c0-0.53-0.4-1-0.93-1.03C9.48,1.96,9,2.43,9,3v1.1 C6.72,4.56,5,6.58,5,9v5H4.75C4.34,14,4,14.34,4,14.75s0.34,0.75,0.75,0.75h10.5C15.66,15.5,16,15.16,16,14.75z M10,18 c0.83,0,1.5-0.67,1.5-1.5h-3C8.5,17.33,9.17,18,10,18z M14.32,5.27l1.41,1.41l-3.67,3.67c-0.09,0.09-0.22,0.15-0.35,0.15H11 c-0.28,0-0.5-0.22-0.5-0.5l0-0.71c0-0.13,0.05-0.26,0.15-0.35L14.32,5.27z M14.81,4.77l0.63-0.63c0.2-0.2,0.51-0.2,0.71,0 l0.71,0.71c0.2,0.2,0.2,0.51,0,0.71l-0.63,0.63L14.81,4.77z",
                }
            }
        }
    }
}

pub fn edit_notifications_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                path {
                    d: "M17.58,6.25l1.77,1.77l-4.84,4.84C14.42,12.95,14.29,13,14.16,13H13.1c-0.28,0-0.5-0.22-0.5-0.5v-1.06 c0-0.13,0.05-0.26,0.15-0.35L17.58,6.25z M20.85,5.81l-1.06-1.06c-0.2-0.2-0.51-0.2-0.71,0l-0.85,0.85l1.77,1.77l0.85-0.85 C21.05,6.32,21.05,6,20.85,5.81z M20,18c0,0.55-0.45,1-1,1H5c-0.55,0-1-0.45-1-1s0.45-1,1-1h1v-7c0-2.79,1.91-5.14,4.5-5.8V3.5 C10.5,2.67,11.17,2,12,2s1.5,0.67,1.5,1.5v0.7c0.82,0.21,1.57,0.59,2.21,1.09l-4.52,4.52c-0.38,0.38-0.59,0.88-0.59,1.41V13 c0,1.1,0.9,2,2,2h1.77c0.53,0,1.04-0.21,1.41-0.59L18,12.2V17h1C19.55,17,20,17.45,20,18z M10,20h4c0,1.1-0.9,2-2,2S10,21.1,10,20z",
                }
            }
        }
    }
}

pub fn elderly_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11.5,3.5c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S11.5,4.6,11.5,3.5z M18.53,11c-1.57,0.01-2.94-0.9-3.6-2.21 l-0.79-1.67l0,0c-0.17-0.35-0.44-0.65-0.8-0.85c-0.62-0.36-1.35-0.34-1.94-0.03l0-0.01L7.01,8.73C6.39,9.08,6,9.74,6,10.46V13 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2.54l1.5-0.85C9.18,10.71,9,11.85,9,13v5.33L7,21c-0.33,0.44-0.24,1.07,0.2,1.4 c0.44,0.33,1.07,0.24,1.4-0.2l2.04-2.72c0.23-0.31,0.37-0.69,0.4-1.08l0.18-2.94L13,18v4c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1 v-4.87c0-0.41-0.13-0.81-0.36-1.15l-1.6-2.29c0,0,0,0,0-0.01c-0.11-1.16,0.07-2.32,0.46-3.4c0.81,1.23,2.05,2.14,3.51,2.52v0.03V13 c0,0.28,0.22,0.5,0.5,0.5S18,13.28,18,13v-0.16V12.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5v10c0,0.28,0.22,0.5,0.5,0.5 s0.5-0.22,0.5-0.5v-10C20,11.68,19.34,10.99,18.53,11z",
            }
        }
    }
}

pub fn elderly_woman_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14.87,9.02c-0.42-0.09-0.8,0.06-1.06,0.33c-1.91-0.49-1.98-2.15-2.92-2.91c-0.26-0.21-0.57-0.34-0.89-0.4 C6.85,5.45,5.5,14.26,5.5,15h1.6l-1.2,1.5c-0.23,0.28-0.2,0.69,0.05,0.95c0.3,0.3,0.78,0.27,1.05-0.05L9,15h1.5v2.25 c0,0.41,0.34,0.75,0.75,0.75c0.41,0,0.75-0.34,0.75-0.75V15l-1.46-3.93l0.28-2.33l0,0c0,0,0,0,0,0c0.02,0.03,0.91,1.44,2.68,2.02 l0.03,0.01c0.05,0.14,0.19,0.24,0.35,0.24c0.21,0,0.38-0.17,0.38-0.38v-0.5c0-0.21,0.17-0.38,0.38-0.38S15,9.91,15,10.12v7.51 c0,0.21,0.17,0.38,0.38,0.38s0.38-0.17,0.38-0.38v-7.46C15.75,9.63,15.4,9.13,14.87,9.02z",
                    }
                    path {
                        d: "M9.34,3.22C9.29,3.39,9.25,3.56,9.25,3.75c0,0.97,0.79,1.75,1.75,1.75c0.96,0,1.75-0.78,1.75-1.75 C12.75,2.78,11.97,2,11,2c-0.29,0-0.57,0.08-0.81,0.21C10.08,1.94,9.81,1.75,9.5,1.75c-0.41,0-0.75,0.34-0.75,0.75 C8.75,2.86,9.01,3.14,9.34,3.22z",
                    }
                }
            }
        }
    }
}

pub fn elderly_woman_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18.52,11c-1.57,0-2.94-0.9-3.6-2.21l-0.79-1.67l0,0C14.12,7.1,13.63,6,12.34,6l0,0C8.72,6,6,16.69,6,19h2.5L7,21 c-0.33,0.44-0.24,1.07,0.2,1.4c0.44,0.33,1.07,0.24,1.4-0.2L11,19h2v3c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2.71 c0-0.22-0.04-0.43-0.1-0.64L13,13l0.49-2.71c0.81,1.23,2.05,2.14,3.51,2.52V13c0,0.28,0.22,0.5,0.5,0.5S18,13.28,18,13v-0.5 c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5v10c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5v-10C20,11.71,19.38,11,18.52,11z",
                    }
                    path {
                        d: "M11.6,2.91c-0.06,0.19-0.1,0.38-0.1,0.59c0,1.1,0.9,2,2,2s2-0.9,2-2c0-1.1-0.9-2-2-2c-0.21,0-0.4,0.04-0.59,0.1 C12.76,1.25,12.41,1,12,1c-0.55,0-1,0.45-1,1C11,2.41,11.25,2.76,11.6,2.91z",
                    }
                }
            }
        }
    }
}

pub fn emoji_emotions_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M9.99,3C6.13,3,3,6.14,3,10s3.13,7,6.99,7c3.87,0,7.01-3.14,7.01-7S13.86,3,9.99,3z M6,8c0-0.55,0.45-1,1-1s1,0.45,1,1 c0,0.55-0.45,1-1,1S6,8.55,6,8z M13.6,11.71C12.97,13.06,11.6,14,10,14s-2.97-0.94-3.6-2.29C6.24,11.38,6.5,11,6.86,11h6.28 C13.5,11,13.76,11.38,13.6,11.71z M13,9c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C14,8.55,13.55,9,13,9z",
                }
            }
        }
    }
}

pub fn emoji_emotions_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                }
                path {
                    d: "M11.99,2C6.47,2,2,6.48,2,12c0,5.52,4.47,10,9.99,10C17.52,22,22,17.52,22,12C22,6.48,17.52,2,11.99,2z M8.5,8 C9.33,8,10,8.67,10,9.5S9.33,11,8.5,11S7,10.33,7,9.5S7.67,8,8.5,8z M16.71,14.72C15.8,16.67,14.04,18,12,18s-3.8-1.33-4.71-3.28 C7.13,14.39,7.37,14,7.74,14h8.52C16.63,14,16.87,14.39,16.71,14.72z M15.5,11c-0.83,0-1.5-0.67-1.5-1.5S14.67,8,15.5,8 S17,8.67,17,9.5S16.33,11,15.5,11z",
                }
            }
        }
    }
}

pub fn emoji_events_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M15,6h-2c0-0.55-0.45-1-1-1H8C7.45,5,7,5.45,7,6H5C4.45,6,4,6.45,4,7v1c0,1.66,1.34,3,3,3h0.18 c0.36,1.01,1.24,1.77,2.32,1.95V15h-2C7.22,15,7,15.22,7,15.5C7,15.78,7.22,16,7.5,16h2h1h2c0.28,0,0.5-0.22,0.5-0.5 c0-0.28-0.22-0.5-0.5-0.5h-2v-2.05c1.08-0.18,1.96-0.94,2.32-1.95H13c1.66,0,3-1.34,3-3V7C16,6.45,15.55,6,15,6z M7,10 c-1.1,0-2-0.9-2-2V7h2V10z M15,8c0,1.1-0.9,2-2,2V7h2V8z",
                }
            }
        }
    }
}

pub fn emoji_events_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19,5h-2V4c0-0.55-0.45-1-1-1H8C7.45,3,7,3.45,7,4v1H5C3.9,5,3,5.9,3,7v1c0,2.55,1.92,4.63,4.39,4.94 c0.63,1.5,1.98,2.63,3.61,2.96V19H8c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-3v-3.1 c1.63-0.33,2.98-1.46,3.61-2.96C19.08,12.63,21,10.55,21,8V7C21,5.9,20.1,5,19,5z M5,8V7h2v3.82C5.84,10.4,5,9.3,5,8z M19,8 c0,1.3-0.84,2.4-2,2.82V7h2V8z",
            }
        }
    }
}

pub fn emoji_flags_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M14.5,8H12l-0.85-0.85C11.05,7.05,10.93,7,10.79,7H7V5.85C7.29,5.67,7.5,5.37,7.5,5c0-0.55-0.45-1-1-1s-1,0.45-1,1 c0,0.37,0.21,0.67,0.5,0.85v9.65C6,15.78,6.22,16,6.5,16S7,15.78,7,15.5V13h3l0.85,0.85c0.09,0.09,0.22,0.15,0.35,0.15h3.29 c0.28,0,0.5-0.22,0.5-0.5v-5C15,8.22,14.78,8,14.5,8z M14,13h-2.59l-1-1H7V8h3.59l1,1H14V13z",
                }
            }
        }
    }
}

pub fn emoji_flags_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                }
                path {
                    d: "M19,9h-5l-0.72-1.45C13.11,7.21,12.76,7,12.38,7H7V5.72C7.6,5.38,8,4.74,8,4c0-1.1-0.9-2-2-2S4,2.9,4,4 c0,0.74,0.4,1.38,1,1.72V20c0,0.55,0.45,1,1,1s1-0.45,1-1v-3h5l0.72,1.45c0.17,0.34,0.52,0.55,0.89,0.55H19c0.55,0,1-0.45,1-1v-8 C20,9.45,19.55,9,19,9z M18,17h-4l-1-2H7V9h5l1,2h5V17z",
                }
            }
        }
    }
}

pub fn emoji_food_beverage_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                g {
                    path {
                        d: "M15,5H7.75v2.17l1.14,0.76C8.96,7.97,9,8.05,9,8.13v2.62C9,10.89,8.89,11,8.75,11h-2.5C6.11,11,6,10.89,6,10.75V8.13 c0-0.08,0.04-0.16,0.11-0.21l1.14-0.76V5H5v7c0,1.1,0.9,2,2,2h5c1.1,0,2-0.9,2-2V9h1c0.55,0,1-0.45,1-1V6C16,5.45,15.55,5,15,5z M15,8h-1V6h1V8z",
                    }
                    path {
                        d: "M14.5,15h-10C4.22,15,4,15.22,4,15.5C4,15.78,4.22,16,4.5,16h10c0.28,0,0.5-0.22,0.5-0.5C15,15.22,14.78,15,14.5,15z",
                    }
                }
            }
        }
    }
}

pub fn emoji_food_beverage_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M19,19H3c-0.55,0-1,0.45-1,1s0.45,1,1,1h16c0.55,0,1-0.45,1-1S19.55,19,19,19z",
                    }
                    path {
                        d: "M20,3H9v2.4l1.81,1.45C10.93,6.94,11,7.09,11,7.24v4.26c0,0.28-0.22,0.5-0.5,0.5h-4C6.22,12,6,11.78,6,11.5V7.24 c0-0.15,0.07-0.3,0.19-0.39L8,5.4V3H6C4.9,3,4,3.9,4,5v8c0,2.21,1.79,4,4,4h6c2.21,0,4-1.79,4-4v-3h2c1.1,0,2-0.9,2-2V5 C22,3.9,21.1,3,20,3z M20,8h-2V5h2V8z",
                    }
                }
            }
        }
    }
}

pub fn emoji_nature_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                g {
                    path {
                        d: "M16.96,4.8c-0.11-0.33-0.43-0.55-0.79-0.55c-0.02,0-0.04,0-0.06,0H15.5l-0.19-0.6C15.22,3.27,14.88,3,14.5,3h0 c-0.38,0-0.72,0.27-0.81,0.65l-0.19,0.6h-0.61c-0.02,0-0.04,0-0.06,0c-0.36,0-0.68,0.22-0.79,0.55c-0.12,0.35,0.03,0.73,0.35,0.92 l0.54,0.33l-0.24,0.77c-0.14,0.36-0.03,0.78,0.28,1.01C13.12,7.95,13.29,8,13.46,8c0.19,0,0.38-0.07,0.54-0.2l0.51-0.43l0.51,0.43 C15.16,7.93,15.35,8,15.54,8c0.17,0,0.34-0.05,0.49-0.16c0.31-0.23,0.43-0.65,0.28-1.01l-0.24-0.77l0.54-0.33 C16.93,5.53,17.08,5.15,16.96,4.8z M14.5,6C14.22,6,14,5.78,14,5.5C14,5.22,14.22,5,14.5,5S15,5.22,15,5.5C15,5.78,14.78,6,14.5,6 z",
                    }
                    path {
                        d: "M12.75,10h-0.8c-0.09-0.46-0.32-0.91-0.68-1.27S10.46,8.15,10,8.05v-0.8C10,7.11,9.89,7,9.75,7S9.5,7.11,9.5,7.25V8 C8.86,8,8.22,8.24,7.73,8.73C7.56,8.91,7.43,9.1,7.32,9.31l-1.58-0.6C5.23,8.53,4.67,8.56,4.18,8.79 C3.69,9.02,3.32,9.43,3.14,9.95c-0.36,1.01,0.13,2.1,1.09,2.53c-0.41,0.92-0.25,2.04,0.5,2.79c0.76,0.76,1.88,0.92,2.8,0.5 c0.2,0.46,0.54,0.82,0.99,1.04C8.8,16.94,9.1,17,9.4,17c0.23,0,0.46-0.04,0.68-0.12h0c0.51-0.18,0.92-0.55,1.16-1.04 c0.23-0.49,0.26-1.04,0.08-1.56l-0.6-1.6c0.2-0.11,0.39-0.24,0.56-0.41C11.76,11.78,12,11.14,12,10.5h0.75 c0.14,0,0.25-0.11,0.25-0.25S12.89,10,12.75,10z M4.71,11.6c-0.26-0.09-0.47-0.28-0.59-0.53s-0.13-0.53-0.04-0.79 c0.09-0.26,0.28-0.47,0.53-0.59c0.14-0.07,0.29-0.1,0.45-0.1c0.11,0,0.23,0.02,0.34,0.06l2.94,1.11 C7.46,11.31,5.95,12.04,4.71,11.6z M10.33,15.41c-0.12,0.25-0.33,0.44-0.59,0.53S9.2,16.02,8.95,15.9 c-0.25-0.12-0.44-0.33-0.53-0.59c-0.44-1.24,0.29-2.75,0.84-3.63l1.11,2.93C10.46,14.88,10.45,15.16,10.33,15.41z M10.56,11.56 c-0.07,0.07-0.14,0.12-0.21,0.17l-0.49-1.28c-0.01-0.03-0.03-0.04-0.05-0.07c-0.02-0.04-0.04-0.07-0.07-0.1 c-0.03-0.03-0.06-0.05-0.1-0.07c-0.02-0.01-0.04-0.04-0.07-0.05l-1.32-0.5c0.05-0.08,0.11-0.15,0.18-0.22C8.72,9.16,9.1,9,9.5,9 c0.4,0,0.78,0.16,1.06,0.44S11,10.1,11,10.5C11,10.9,10.84,11.28,10.56,11.56z",
                    }
                }
            }
        }
    }
}

pub fn emoji_nature_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                }
                g {
                    path {
                        d: "M21.94,4.88C21.76,4.35,21.25,4,20.68,4c-0.03,0-0.06,0-0.09,0H19.6l-0.31-0.97C19.15,2.43,18.61,2,18,2h0 c-0.61,0-1.15,0.43-1.29,1.04L16.4,4h-0.98c-0.03,0-0.06,0-0.09,0c-0.57,0-1.08,0.35-1.26,0.88c-0.19,0.56,0.04,1.17,0.56,1.48 l0.87,0.52L15.1,8.12c-0.23,0.58-0.04,1.25,0.45,1.62C15.78,9.91,16.06,10,16.33,10c0.31,0,0.61-0.11,0.86-0.32L18,8.98l0.81,0.7 C19.06,9.89,19.36,10,19.67,10c0.27,0,0.55-0.09,0.78-0.26c0.5-0.37,0.68-1.04,0.45-1.62l-0.39-1.24l0.87-0.52 C21.89,6.05,22.12,5.44,21.94,4.88z M18,7c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C19,6.55,18.55,7,18,7z",
                    }
                    path {
                        d: "M15.5,12h-1.09c-0.19-0.54-0.49-1.05-0.93-1.49S12.54,9.78,12,9.59V8.5C12,8.22,11.78,8,11.5,8S11,8.22,11,8.5v0.88 c-0.11-0.01-0.23-0.03-0.34-0.03c-1.02,0-2.05,0.39-2.83,1.17c-0.16,0.16-0.3,0.34-0.43,0.53L6,10.52 c-1.56-0.55-3.28,0.27-3.83,1.82c0,0,0,0,0,0c-0.27,0.75-0.23,1.57,0.12,2.29c0.23,0.48,0.58,0.87,1,1.16 c-0.38,1.35-0.06,2.85,1,3.91c1.06,1.06,2.57,1.38,3.91,1c0.29,0.42,0.68,0.77,1.16,1C9.78,21.9,10.21,22,10.65,22 c0.34,0,0.68-0.06,1.01-0.17c0,0,0,0,0,0c1.56-0.55,2.38-2.27,1.82-3.85l-0.52-1.37c0.18-0.13,0.36-0.27,0.53-0.43 c0.87-0.87,1.24-2.04,1.14-3.17h0.88c0.28,0,0.5-0.22,0.5-0.5C16,12.22,15.78,12,15.5,12z M4.67,14.29 c-0.25-0.09-0.45-0.27-0.57-0.51s-0.13-0.51-0.04-0.76c0.19-0.52,0.76-0.79,1.26-0.61l3.16,1.19C7.33,14.2,5.85,14.71,4.67,14.29z M10.99,19.94c-0.25,0.09-0.52,0.08-0.76-0.04c-0.24-0.11-0.42-0.32-0.51-0.57c-0.42-1.18,0.09-2.65,0.7-3.8l1.18,3.13 C11.78,19.18,11.51,19.76,10.99,19.94z M12.2,14.6l-0.61-1.61c0-0.01-0.01-0.02-0.02-0.03c-0.02-0.04-0.04-0.08-0.06-0.12 c-0.02-0.04-0.04-0.07-0.07-0.11c-0.03-0.03-0.06-0.06-0.09-0.09c-0.03-0.03-0.06-0.06-0.09-0.09c-0.03-0.03-0.07-0.05-0.11-0.07 c-0.04-0.02-0.07-0.05-0.12-0.06c-0.01,0-0.02-0.01-0.03-0.02L9.4,11.8c0.36-0.29,0.79-0.46,1.26-0.46c0.53,0,1.04,0.21,1.41,0.59 C12.8,12.66,12.84,13.81,12.2,14.6z",
                    }
                }
            }
        }
    }
}

pub fn emoji_objects_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                path {
                    d: "M14.91,7.07c-0.01-0.04-0.01-0.08-0.02-0.12c-0.06-0.28-0.15-0.56-0.26-0.82c-0.02-0.05-0.04-0.09-0.06-0.14 c-0.11-0.26-0.25-0.5-0.4-0.74c-0.03-0.04-0.05-0.08-0.08-0.11c-0.16-0.23-0.34-0.46-0.55-0.66c-0.02-0.02-0.03-0.03-0.05-0.05 c-0.2-0.2-0.42-0.38-0.66-0.54c-0.02-0.02-0.04-0.03-0.06-0.05c-0.24-0.16-0.49-0.29-0.75-0.41c-0.04-0.02-0.09-0.04-0.13-0.06 c-0.26-0.11-0.53-0.19-0.81-0.25c-0.05-0.01-0.1-0.02-0.15-0.03C10.64,3.04,10.34,3.01,10.04,3C10.03,3,10.01,3,10,3 C9.9,3,9.79,3.01,9.69,3.02c-0.06,0-0.12,0-0.18,0.01C9.31,3.04,9.12,3.07,8.93,3.11c-1.94,0.4-3.49,2-3.84,3.95 c-0.31,1.72,0.26,3.3,1.33,4.4C6.78,11.84,7,12.34,7,12.87c0,0.77,0,1.6,0,2.14c0,0.55,0.45,1,1,1h0.28l0,0c0.35,0.6,0.98,1,1.72,1 s1.38-0.4,1.72-1l0,0H12c0.55,0,1-0.45,1-1v-2.13c0-0.52,0.21-1.02,0.58-1.4C14.45,10.57,15,9.35,15,8 C15,7.68,14.96,7.37,14.91,7.07z M8.73,6.73L10,8l1.27-1.27l0.71,0.71L10.5,8.91V11h-1V8.91L8.03,7.44L8.73,6.73z M11.5,15 C11.5,15,11.5,15,11.5,15L10,15l0,0l0,0l-1.5,0v0h0C8.22,15,8,14.78,8,14.5C8,14.22,8.22,14,8.5,14h3h0c0.28,0,0.5,0.22,0.5,0.5 C12,14.78,11.78,15,11.5,15z M11.5,13L11.5,13h-3h0C8.22,13,8,12.78,8,12.5C8,12.22,8.22,12,8.5,12h0h3h0c0.28,0,0.5,0.22,0.5,0.5 C12,12.78,11.78,13,11.5,13z",
                }
            }
        }
    }
}

pub fn emoji_objects_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    path {
                        d: "M12,3c-0.46,0-0.93,0.04-1.4,0.14C7.84,3.67,5.64,5.9,5.12,8.66c-0.48,2.61,0.48,5.01,2.22,6.56 C7.77,15.6,8,16.13,8,16.69V19c0,1.1,0.9,2,2,2h0.28c0.35,0.6,0.98,1,1.72,1s1.38-0.4,1.72-1H14c1.1,0,2-0.9,2-2v-2.31 c0-0.55,0.22-1.09,0.64-1.46C18.09,13.95,19,12.08,19,10C19,6.13,15.87,3,12,3z M12.5,14h-1v-2.59L9.67,9.59l0.71-0.71L12,10.5 l1.62-1.62l0.71,0.71l-1.83,1.83V14z M13.5,19c-0.01,0-0.02-0.01-0.03-0.01V19h-2.94v-0.01c-0.01,0-0.02,0.01-0.03,0.01 c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5c0.01,0,0.02,0.01,0.03,0.01V18h2.94v0.01c0.01,0,0.02-0.01,0.03-0.01 c0.28,0,0.5,0.22,0.5,0.5C14,18.78,13.78,19,13.5,19z M13.5,17h-3c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5h3 c0.28,0,0.5,0.22,0.5,0.5C14,16.78,13.78,17,13.5,17z",
                    }
                }
            }
        }
    }
}

pub fn emoji_people_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                g {
                    circle {
                        cy: "5",
                        r: "1",
                        cx: "10",
                    }
                    path {
                        d: "M12.29,7.29C12.11,7.11,11.85,7,11.59,7H8.5C7.12,7,6,5.88,6,4.5H5C5,6.26,6.31,7.75,8,8v4v3.5h1V12h2v3.5h1V12V8.41 l2.47,2.48l0.68-0.74L12.29,7.29z",
                    }
                }
                circle {
                    cx: "8.5",
                    r: ".5",
                    cy: "15.5",
                }
                circle {
                    cx: "11.5",
                    r: ".5",
                    cy: "15.5",
                }
                circle {
                    cx: "5.5",
                    cy: "4.5",
                    r: ".5",
                }
                circle {
                    cy: "10.54",
                    cx: "14.83",
                    r: ".5",
                }
            }
        }
    }
}

pub fn emoji_people_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                }
                g {
                    circle {
                        cy: "4",
                        r: "2",
                        cx: "12",
                    }
                    path {
                        d: "M15.89,8.11C15.5,7.72,14.83,7,13.53,7c-0.21,0-1.42,0-2.54,0C8.53,6.99,6.48,5.2,6.07,2.85C5.99,2.36,5.58,2,5.09,2h0 c-0.61,0-1.09,0.54-1,1.14C4.53,5.8,6.47,7.95,9,8.71V21c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-5h2v5c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1V10.05l3.24,3.24c0.39,0.39,1.02,0.39,1.41,0v0c0.39-0.39,0.39-1.02,0-1.41L15.89,8.11z",
                    }
                }
            }
        }
    }
}

pub fn emoji_symbols_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                }
                g {
                    path {
                        fill: "none",
                        d: "M5.21,12.67l0.35-0.35c0.13-0.13,0.15-0.28,0.15-0.35c0-0.08-0.02-0.23-0.15-0.35s-0.28-0.15-0.35-0.15 c-0.08,0-0.23,0.02-0.35,0.15c-0.19,0.19-0.19,0.51,0,0.71L5.21,12.67z",
                    }
                    path {
                        d: "M4.15,15.15C4.02,15.27,4,15.42,4,15.5s0.02,0.23,0.15,0.35C4.27,15.98,4.42,16,4.5,16s0.23-0.02,0.35-0.15 l0.71-0.71c0,0,0,0,0,0l0.35-0.35l-0.71-0.71L4.15,15.15z",
                        fill: "none",
                    }
                    path {
                        d: "M8.5,5h-4C4.22,5,4,5.22,4,5.5C4,5.78,4.22,6,4.5,6H6v2.5C6,8.78,6.22,9,6.5,9s0.5-0.22,0.5-0.5V6H8.5 C8.78,6,9,5.78,9,5.5C9,5.22,8.78,5,8.5,5z",
                    }
                    path {
                        d: "M8.5,3h-4C4.22,3,4,3.22,4,3.5S4.22,4,4.5,4h4C8.78,4,9,3.78,9,3.5S8.78,3,8.5,3z",
                    }
                    path {
                        d: "M16.35,11.5c-0.19-0.19-0.51-0.19-0.7,0l-4.3,4.3c-0.19,0.19-0.19,0.51,0,0.7c0.19,0.19,0.51,0.19,0.7,0l4.3-4.3 C16.54,12.01,16.54,11.69,16.35,11.5z",
                    }
                    path {
                        d: "M12.71,12.71c0.39-0.39,0.39-1.02,0-1.41c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41 C11.68,13.1,12.32,13.1,12.71,12.71z",
                    }
                    path {
                        d: "M16.71,15.29c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0 C17.1,16.32,17.1,15.68,16.71,15.29z",
                    }
                    path {
                        d: "M12.5,9C13.33,9,14,8.33,14,7.5C14,7.47,14,4,14,4h1.5C15.78,4,16,3.78,16,3.5C16,3.22,15.78,3,15.5,3h-2 C13.22,3,13,3.22,13,3.5v2.59C12.84,6.04,12.68,6,12.5,6C11.67,6,11,6.67,11,7.5S11.67,9,12.5,9z",
                    }
                    path {
                        d: "M8.39,13.03c-0.2-0.2-0.51-0.2-0.71,0l-1.06,1.06l-0.71-0.71l0.35-0.35c0.59-0.59,0.59-1.54,0-2.12 c-0.29-0.29-0.68-0.44-1.06-0.44c-0.38,0-0.77,0.15-1.06,0.44h0c-0.59,0.59-0.59,1.54,0,2.12l0.35,0.35l-1.06,1.06 c-0.59,0.59-0.59,1.54,0,2.12C3.73,16.85,4.12,17,4.5,17c0.38,0,0.77-0.15,1.06-0.44l1.06-1.06l1.06,1.06c0.2,0.2,0.51,0.2,0.71,0 c0.2-0.2,0.2-0.51,0-0.71l-1.06-1.06l1.06-1.06C8.58,13.54,8.58,13.22,8.39,13.03z M4.85,11.61c0.13-0.13,0.28-0.15,0.35-0.15 c0.08,0,0.23,0.02,0.35,0.15s0.15,0.28,0.15,0.35c0,0.08-0.02,0.23-0.15,0.35l-0.35,0.35l-0.35-0.35 C4.66,12.12,4.66,11.81,4.85,11.61z M5.56,15.15l-0.71,0.71C4.73,15.98,4.58,16,4.5,16s-0.23-0.02-0.35-0.15 C4.02,15.73,4,15.58,4,15.5s0.02-0.23,0.15-0.35l1.06-1.06l0.71,0.71C5.91,14.79,5.56,15.15,5.56,15.15z",
                    }
                }
            }
        }
    }
}

pub fn emoji_symbols_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                }
                g {
                    path {
                        d: "M10,5H4C3.45,5,3,5.45,3,6c0,0.55,0.45,1,1,1h2v3c0,0.55,0.45,1,1,1s1-0.45,1-1V7h2c0.55,0,1-0.45,1-1 C11,5.45,10.55,5,10,5z",
                    }
                    path {
                        d: "M10,2H4C3.45,2,3,2.45,3,3s0.45,1,1,1h6c0.55,0,1-0.45,1-1S10.55,2,10,2z",
                    }
                    path {
                        d: "M20.89,13.11c-0.39-0.39-1.02-0.39-1.41,0l-6.36,6.36c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0l6.36-6.36 C21.28,14.13,21.28,13.5,20.89,13.11z",
                    }
                    circle {
                        r: "1.5",
                        cx: "14.5",
                        cy: "14.5",
                    }
                    circle {
                        cy: "19.5",
                        cx: "19.5",
                        r: "1.5",
                    }
                    path {
                        d: "M15.5,11c1.38,0,2.5-1.12,2.5-2.5V4h2c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v3.51 C16.58,6.19,16.07,6,15.5,6C14.12,6,13,7.12,13,8.5C13,9.88,14.12,11,15.5,11z",
                    }
                    path {
                        d: "M10.45,18.09c0.39-0.39,0.39-1.02,0-1.41c-0.39-0.39-1.02-0.39-1.41,0l-0.71,0.71l-0.71-0.71l0.35-0.35 c0.98-0.98,0.98-2.56,0-3.54h0c-0.49-0.49-1.13-0.73-1.77-0.73c-0.64,0-1.28,0.24-1.77,0.73c-0.98,0.98-0.98,2.56,0,3.54 l0.35,0.35l-1.06,1.06c-0.98,0.98-0.98,2.56,0,3.54C4.22,21.76,4.86,22,5.5,22c0.64,0,1.28-0.24,1.77-0.73l1.06-1.06l0.71,0.71 c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41l-0.71-0.71v0L10.45,18.09z M5.85,14.2c0.12-0.12,0.26-0.15,0.35-0.15 s0.23,0.03,0.35,0.15c0.19,0.2,0.19,0.51,0,0.71l-0.35,0.35L5.85,14.9c-0.12-0.12-0.15-0.26-0.15-0.35S5.73,14.32,5.85,14.2z M5.85,19.85C5.73,19.97,5.59,20,5.5,20s-0.23-0.03-0.35-0.15C5.03,19.73,5,19.59,5,19.5s0.03-0.23,0.15-0.35l1.06-1.06l0.71,0.71 L5.85,19.85z",
                    }
                }
            }
        }
    }
}

pub fn emoji_transportation_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    path {
                        d: "M8,5.25C8,5.11,8.11,5,8.25,5h2.5C10.89,5,11,5.11,11,5.25V8h1V4.5C12,4.22,11.78,4,11.5,4h-4C7.22,4,7,4.22,7,4.5V7H4.5 C4.22,7,4,7.22,4,7.5V16h1V8.25C5,8.11,5.11,8,5.25,8H8V5.25z",
                    }
                    rect {
                        y: "9",
                        width: "1",
                        height: "1",
                        x: "6",
                    }
                    rect {
                        width: "1",
                        height: "1",
                        x: "9",
                        y: "6",
                    }
                    rect {
                        height: "1",
                        width: "1",
                        y: "11",
                        x: "6",
                    }
                    rect {
                        width: "1",
                        y: "13",
                        x: "6",
                        height: "1",
                    }
                    path {
                        d: "M15.11,9.34C15.05,9.14,14.85,9,14.64,9H9.36C9.15,9,8.95,9.14,8.89,9.34L8,12v2v0.5V15v0.5C8,15.78,8.22,16,8.5,16 S9,15.78,9,15.5V15h6v0.5c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5V15v-0.5V14v-2L15.11,9.34z M9.72,10h4.56l0.67,2H9.05L9.72,10 z M9.5,14C9.22,14,9,13.78,9,13.5S9.22,13,9.5,13s0.5,0.22,0.5,0.5S9.78,14,9.5,14z M14.5,14c-0.28,0-0.5-0.22-0.5-0.5 s0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5S14.78,14,14.5,14z",
                    }
                }
            }
        }
    }
}

pub fn emoji_transportation_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                }
                g {
                    path {
                        d: "M21.99,14.77l-1.43-4.11c-0.14-0.4-0.52-0.66-0.97-0.66H12.4c-0.46,0-0.83,0.26-0.98,0.66L10,14.77v5.24 c0,0.55,0.45,0.99,1,0.99s1-0.45,1-1v-1h8v1c0,0.55,0.45,1,1,1s0.99-0.44,1-0.99L21.99,14.77z M11.61,13.34l0.69-2 c0.05-0.2,0.24-0.34,0.46-0.34h6.48c0.21,0,0.4,0.14,0.47,0.34l0.69,2c0.11,0.32-0.13,0.66-0.47,0.66h-7.85 C11.74,14,11.5,13.66,11.61,13.34z M11.99,17c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S12.54,17,11.99,17z M19.99,17 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S20.54,17,19.99,17z",
                    }
                    path {
                        d: "M14,4.5V9h1V4c0-0.55-0.45-1-1-1H8C7.45,3,7,3.45,7,4v4H3C2.45,8,2,8.45,2,9v12h1V9.5C3,9.22,3.22,9,3.5,9h4 C7.78,9,8,8.78,8,8.5v-4C8,4.22,8.22,4,8.5,4h5C13.78,4,14,4.22,14,4.5z",
                    }
                    rect {
                        width: "2",
                        height: "2",
                        y: "11",
                        x: "5",
                    }
                    rect {
                        y: "5",
                        height: "2",
                        width: "2",
                        x: "10",
                    }
                    rect {
                        y: "15",
                        height: "2",
                        x: "5",
                        width: "2",
                    }
                    rect {
                        height: "2",
                        width: "2",
                        x: "5",
                        y: "19",
                    }
                }
            }
        }
    }
}

pub fn engineering_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M4.82,8H5h5h0.32c0.14,0,0.25-0.11,0.25-0.25S10.45,7.5,10.32,7.5H10c0-1.02-0.62-1.9-1.5-2.29v0.54 C8.5,5.89,8.39,6,8.25,6S8,5.89,8,5.75v-0.7C7.84,5.02,7.67,5,7.5,5S7.16,5.02,7,5.05v0.7C7,5.89,6.89,6,6.75,6S6.5,5.89,6.5,5.75 V5.21C5.62,5.6,5,6.48,5,7.5H4.82c-0.14,0-0.25,0.11-0.25,0.25S4.68,8,4.82,8z",
                    }
                    path {
                        d: "M7.5,10.5c1.21,0,2.22-0.86,2.45-2h-4.9C5.28,9.64,6.29,10.5,7.5,10.5z",
                    }
                    path {
                        d: "M17.27,5.31l0.66-0.59L17.4,3.79l-0.85,0.28c-0.1-0.08-0.21-0.14-0.33-0.19L16.04,3h-1.07l-0.18,0.87 c-0.12,0.05-0.24,0.12-0.34,0.19L13.6,3.79l-0.54,0.93l0.66,0.59c-0.01,0.13-0.01,0.25,0,0.37l-0.66,0.61l0.54,0.93l0.86-0.27 c0.1,0.07,0.2,0.13,0.31,0.18L14.96,8h1.07l0.19-0.87c0.11-0.05,0.22-0.11,0.32-0.18l0.85,0.27l0.54-0.93l-0.66-0.61 C17.28,5.55,17.28,5.43,17.27,5.31z M15.5,6.39c-0.49,0-0.89-0.4-0.89-0.89c0-0.49,0.4-0.89,0.89-0.89s0.89,0.4,0.89,0.89 C16.39,5.99,15.99,6.39,15.5,6.39z",
                    }
                    path {
                        d: "M15.3,8.72l-0.59,0.2c-0.07-0.05-0.15-0.1-0.23-0.13l-0.13-0.61H13.6l-0.13,0.61c-0.08,0.04-0.17,0.08-0.24,0.14 l-0.59-0.19l-0.38,0.65l0.46,0.41c-0.01,0.09-0.01,0.18,0,0.26l-0.46,0.42l0.38,0.65l0.6-0.19c0.07,0.05,0.14,0.09,0.22,0.12 l0.14,0.62h0.75l0.13-0.61c0.08-0.04,0.15-0.08,0.22-0.13l0.59,0.19l0.38-0.65l-0.46-0.42c0.01-0.09,0.01-0.17,0-0.26l0.47-0.42 L15.3,8.72z M13.97,10.54c-0.35,0-0.62-0.28-0.62-0.62c0-0.35,0.28-0.62,0.62-0.62c0.35,0,0.62,0.28,0.62,0.62 C14.6,10.26,14.32,10.54,13.97,10.54z",
                    }
                    path {
                        d: "M7.5,12C5,12,2,13.24,2,15v0.97h11V15C13,13.24,10,12,7.5,12z",
                    }
                }
            }
        }
    }
}

pub fn engineering_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M9,15c-2.67,0-8,1.34-8,4v1c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1v-1C17,16.34,11.67,15,9,15z",
                    }
                    path {
                        d: "M4.74,9h8.53c0.27,0,0.49-0.22,0.49-0.49V8.49c0-0.27-0.22-0.49-0.49-0.49H13c0-1.48-0.81-2.75-2-3.45V5.5 C11,5.78,10.78,6,10.5,6S10,5.78,10,5.5V4.14C9.68,4.06,9.35,4,9,4S8.32,4.06,8,4.14V5.5C8,5.78,7.78,6,7.5,6S7,5.78,7,5.5V4.55 C5.81,5.25,5,6.52,5,8H4.74C4.47,8,4.25,8.22,4.25,8.49v0.03C4.25,8.78,4.47,9,4.74,9z",
                    }
                    path {
                        d: "M9,13c1.86,0,3.41-1.28,3.86-3H5.14C5.59,11.72,7.14,13,9,13z",
                    }
                    path {
                        d: "M21.98,6.23l0.93-0.83l-0.75-1.3l-1.19,0.39c-0.14-0.11-0.3-0.2-0.47-0.27L20.25,3h-1.5L18.5,4.22 c-0.17,0.07-0.33,0.16-0.48,0.27L16.84,4.1l-0.75,1.3l0.93,0.83C17,6.4,17,6.58,17.02,6.75L16.09,7.6l0.75,1.3l1.2-0.38 c0.13,0.1,0.28,0.18,0.43,0.25L18.75,10h1.5l0.27-1.22c0.16-0.07,0.3-0.15,0.44-0.25l1.19,0.38l0.75-1.3l-0.93-0.85 C22,6.57,21.99,6.4,21.98,6.23z M19.5,7.75c-0.69,0-1.25-0.56-1.25-1.25s0.56-1.25,1.25-1.25s1.25,0.56,1.25,1.25 S20.19,7.75,19.5,7.75z",
                    }
                    path {
                        d: "M19.4,10.79l-0.85,0.28c-0.1-0.08-0.21-0.14-0.33-0.19L18.04,10h-1.07l-0.18,0.87c-0.12,0.05-0.24,0.12-0.34,0.19 l-0.84-0.28l-0.54,0.93l0.66,0.59c-0.01,0.13-0.01,0.25,0,0.37l-0.66,0.61l0.54,0.93l0.86-0.27c0.1,0.07,0.2,0.13,0.31,0.18 L16.96,15h1.07l0.19-0.87c0.11-0.05,0.22-0.11,0.32-0.18l0.85,0.27l0.54-0.93l-0.66-0.61c0.01-0.13,0.01-0.25,0-0.37l0.66-0.59 L19.4,10.79z M17.5,13.39c-0.49,0-0.89-0.4-0.89-0.89c0-0.49,0.4-0.89,0.89-0.89s0.89,0.4,0.89,0.89 C18.39,12.99,17.99,13.39,17.5,13.39z",
                    }
                }
            }
        }
    }
}

pub fn facebook_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22,12c0-5.52-4.48-10-10-10S2,6.48,2,12c0,4.84,3.44,8.87,8,9.8V15H8v-3h2V9.5C10,7.57,11.57,6,13.5,6H16v3h-2 c-0.55,0-1,0.45-1,1v2h3v3h-3v6.95C18.05,21.45,22,17.19,22,12z",
            }
        }
    }
}

pub fn face_2_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18.12,11.5c1.13-0.87,1.87-2.22,1.87-3.75c0-2.62-2.13-4.75-4.75-4.75c-0.28,0-0.55,0.04-0.81,0.08 C13.75,1.29,12.03,0,10,0C7.97,0,6.25,1.29,5.57,3.08C5.3,3.04,5.03,3,4.75,3C2.13,3,0,5.13,0,7.75c0,1.53,0.74,2.88,1.87,3.75 C0.74,12.37,0,13.72,0,15.25C0,17.87,2.13,20,4.75,20c1.24,0,2.36-0.49,3.2-1.28C8.6,18.9,9.29,19,10,19 c0.71,0,1.39-0.1,2.05-0.27C12.89,19.51,14,20,15.24,20c2.62,0,4.75-2.13,4.75-4.75C19.99,13.72,19.25,12.37,18.12,11.5z M10,17.5 c-3.58,0-6.5-2.92-6.5-6.5c0-3.26,2.41-5.96,5.55-6.42C9.29,7.05,11.36,9,13.89,9c0.74,0,1.46-0.17,2.11-0.49 c0.32,0.77,0.5,1.61,0.5,2.49C16.5,14.58,13.58,17.5,10,17.5z",
                    }
                    circle {
                        cx: "7.5",
                        cy: "11.5",
                        r: "1",
                    }
                    circle {
                        cy: "11.5",
                        cx: "12.5",
                        r: "1",
                    }
                }
            }
        }
    }
}

pub fn face_2_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M21.97,13.52c0-0.01,0-0.02,0-0.04C23.21,12.38,24,10.78,24,9c0-3.31-2.69-6-6-6c-0.26,0-0.52,0.02-0.78,0.06 C16.19,1.23,14.24,0,12,0S7.81,1.23,6.78,3.06C6.52,3.02,6.26,3,6,3C2.69,3,0,5.69,0,9c0,1.78,0.79,3.38,2.02,4.48 c0,0.01,0,0.02,0,0.04C0.79,14.62,0,16.22,0,18c0,3.31,2.69,6,6,6c1.39,0,2.67-0.48,3.69-1.28C10.43,22.9,11.2,23,12,23 s1.57-0.1,2.31-0.28C15.33,23.52,16.61,24,18,24c3.31,0,6-2.69,6-6C24,16.22,23.21,14.62,21.97,13.52z M12,21c-4.41,0-8-3.59-8-8 c0-3.72,2.56-6.85,6-7.74c0,0.02,0,0.03,0,0.05c0,3.34,2.72,6.06,6.06,6.06c1.26,0,2.45-0.39,3.45-1.09 C19.82,11.14,20,12.05,20,13C20,17.41,16.41,21,12,21z",
                    }
                    circle {
                        cx: "9",
                        cy: "14",
                        r: "1.25",
                    }
                    circle {
                        r: "1.25",
                        cx: "15",
                        cy: "14",
                    }
                }
            }
        }
    }
}

pub fn face_3_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cx: "7.5",
                        cy: "10.5",
                        r: "1",
                    }
                    circle {
                        cx: "12.5",
                        cy: "10.5",
                        r: "1",
                    }
                    path {
                        d: "M19.81,18.33l-0.93-8.38C18.38,5.42,14.55,2,10,2C5.45,2,1.62,5.42,1.12,9.95l-0.93,8.38C0.09,19.22,0.78,20,1.68,20 h16.65C19.22,20,19.91,19.22,19.81,18.33z M3.9,7.78C4.61,8.24,5.44,8.5,6.32,8.5C7.85,8.5,9.2,7.72,10,6.53 c0.8,1.19,2.15,1.97,3.68,1.97c0.87,0,1.71-0.26,2.42-0.72c0.25,0.7,0.4,1.44,0.4,2.22c0,3.58-2.92,6.5-6.5,6.5S3.5,13.58,3.5,10 C3.5,9.22,3.65,8.47,3.9,7.78z",
                    }
                }
            }
        }
    }
}

pub fn face_3_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    circle {
                        cy: "13",
                        r: "1.25",
                        cx: "9",
                    }
                    circle {
                        cy: "13",
                        r: "1.25",
                        cx: "15",
                    }
                    path {
                        d: "M22.91,11.96C22.39,6.32,17.66,2,12,2S1.61,6.32,1.09,11.96l-0.9,9.86C0.09,22.99,1.01,24,2.19,24h19.62 c1.18,0,2.1-1.01,1.99-2.18L22.91,11.96z M4.54,9.13C5.41,9.68,6.43,10,7.5,10C9.36,10,11,9.07,12,7.65C13,9.07,14.64,10,16.5,10 c1.07,0,2.09-0.32,2.96-0.87C19.8,10.02,20,10.99,20,12c0,4.41-3.59,8-8,8s-8-3.59-8-8C4,10.99,4.2,10.02,4.54,9.13z",
                    }
                }
            }
        }
    }
}

pub fn face_4_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cx: "7.5",
                        cy: "10.5",
                        r: "1",
                    }
                    circle {
                        cx: "12.5",
                        cy: "10.5",
                        r: "1",
                    }
                    path {
                        d: "M10,2C9.18,2,8.38,2.12,7.64,2.36C7.01,0.97,5.62,0,4,0C1.79,0,0,1.79,0,4c0,1.62,0.97,3.01,2.36,3.64 C2.12,8.38,2,9.18,2,10c0,4.42,3.58,8,8,8s8-3.58,8-8S14.42,2,10,2z M10,16.5c-3.58,0-6.5-2.92-6.5-6.5 c0-0.14,0.03-0.28,0.04-0.42C5.66,8.87,7.38,7.3,8.29,5.29c2.28,2.44,5.34,3.04,7.84,2.55c0.24,0.68,0.38,1.4,0.38,2.16 C16.5,13.58,13.58,16.5,10,16.5z",
                    }
                }
            }
        }
    }
}

pub fn face_4_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M12,2c-0.96,0-1.88,0.14-2.75,0.39C8.37,0.96,6.8,0,5,0C2.24,0,0,2.24,0,5c0,1.8,0.96,3.37,2.39,4.25 C2.14,10.12,2,11.04,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.41,0-8-3.59-8-8c0-0.05,0.01-0.1,0.01-0.15 c2.6-0.98,4.68-2.99,5.74-5.55C11.58,8.56,14.37,10,17.5,10c0.75,0,1.47-0.09,2.17-0.24C19.88,10.47,20,11.22,20,12 C20,16.41,16.41,20,12,20z",
                    }
                    circle {
                        r: "1.25",
                        cy: "13",
                        cx: "9",
                    }
                    circle {
                        cy: "13",
                        cx: "15",
                        r: "1.25",
                    }
                }
            }
        }
    }
}

pub fn face_5_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cx: "11",
                        cy: "4.5",
                        r: ".5",
                    }
                    circle {
                        r: ".5",
                        cx: "9",
                        cy: "4.5",
                    }
                    circle {
                        cy: "5.5",
                        cx: "10",
                        r: ".5",
                    }
                    circle {
                        cy: "5.5",
                        cx: "8",
                        r: ".5",
                    }
                    circle {
                        cy: "5.5",
                        cx: "12",
                        r: ".5",
                    }
                    circle {
                        cy: "6.5",
                        r: ".5",
                        cx: "11",
                    }
                    circle {
                        cy: "6.5",
                        cx: "13",
                        r: ".5",
                    }
                    circle {
                        cx: "9",
                        r: ".5",
                        cy: "6.5",
                    }
                    circle {
                        cx: "7",
                        cy: "6.5",
                        r: ".5",
                    }
                    circle {
                        cx: "6",
                        cy: "7.5",
                        r: ".5",
                    }
                    circle {
                        cx: "8",
                        r: ".5",
                        cy: "7.5",
                    }
                    circle {
                        cy: "7.5",
                        r: ".5",
                        cx: "12",
                    }
                    circle {
                        r: ".5",
                        cx: "10",
                        cy: "7.5",
                    }
                    circle {
                        r: ".5",
                        cx: "14",
                        cy: "7.5",
                    }
                    path {
                        d: "M10,2c-4.42,0-8,3.58-8,8s3.58,8,8,8s8-3.58,8-8S14.42,2,10,2z M10,16.5c-3.58,0-6.5-2.92-6.5-6.5 c0-0.71,0.12-1.39,0.33-2.03C3.88,7.99,3.94,8,4,8c0.28,0,0.5-0.22,0.5-0.5c0-0.2-0.12-0.37-0.29-0.45 C4.3,6.88,4.4,6.71,4.51,6.54C4.53,6.8,4.74,7,5,7c0.28,0,0.5-0.22,0.5-0.5C5.5,6.22,5.28,6,5,6C4.95,6,4.91,6.01,4.87,6.03 c0.22-0.28,0.45-0.54,0.71-0.77C5.53,5.33,5.5,5.41,5.5,5.5C5.5,5.78,5.72,6,6,6s0.5-0.22,0.5-0.5C6.5,5.22,6.28,5,6,5 C5.93,5,5.87,5.01,5.81,5.04c0.22-0.19,0.45-0.35,0.7-0.51C6.52,4.79,6.73,5,7,5c0.28,0,0.5-0.22,0.5-0.5 c0-0.17-0.09-0.31-0.21-0.4c0.15-0.07,0.3-0.13,0.45-0.19C7.81,3.96,7.9,4,8,4c0.21,0,0.39-0.13,0.46-0.31 C8.8,3.61,9.15,3.55,9.51,3.53C9.52,3.79,9.73,4,10,4s0.48-0.21,0.49-0.47c0.36,0.03,0.71,0.08,1.04,0.17C11.61,3.87,11.79,4,12,4 c0.1,0,0.19-0.04,0.26-0.09c0.15,0.06,0.3,0.12,0.45,0.19c-0.13,0.09-0.21,0.23-0.21,0.4C12.5,4.78,12.72,5,13,5 c0.27,0,0.48-0.21,0.49-0.47c0.24,0.16,0.48,0.32,0.7,0.51C14.13,5.01,14.07,5,14,5c-0.28,0-0.5,0.22-0.5,0.5 C13.5,5.78,13.72,6,14,6s0.5-0.22,0.5-0.5c0-0.09-0.03-0.17-0.08-0.25c0.26,0.24,0.49,0.5,0.71,0.77C15.09,6.01,15.05,6,15,6 c-0.28,0-0.5,0.22-0.5,0.5C14.5,6.78,14.72,7,15,7c0.26,0,0.47-0.2,0.49-0.46c0.11,0.17,0.2,0.34,0.29,0.51 C15.62,7.13,15.5,7.3,15.5,7.5C15.5,7.78,15.72,8,16,8c0.06,0,0.12-0.01,0.17-0.03C16.38,8.61,16.5,9.29,16.5,10 C16.5,13.58,13.58,16.5,10,16.5z",
                    }
                    circle {
                        cx: "7.5",
                        cy: "10.5",
                        r: "1",
                    }
                    circle {
                        cx: "12.5",
                        cy: "10.5",
                        r: "1",
                    }
                }
            }
        }
    }
}

pub fn face_5_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.41,0-8-3.59-8-8 c0-1.12,0.23-2.18,0.65-3.15C4.74,8.94,4.86,9,5,9c0.28,0,0.5-0.22,0.5-0.5c0-0.25-0.19-0.45-0.43-0.49 c0.15-0.26,0.32-0.51,0.49-0.75C5.53,7.34,5.5,7.41,5.5,7.5C5.5,7.78,5.72,8,6,8s0.5-0.22,0.5-0.5C6.5,7.22,6.28,7,6,7 C5.87,7,5.75,7.05,5.66,7.13c0.52-0.68,1.15-1.28,1.86-1.76C7.51,5.41,7.5,5.45,7.5,5.5C7.5,5.78,7.72,6,8,6s0.5-0.22,0.5-0.5 c0-0.24-0.17-0.43-0.4-0.48c0.16-0.09,0.32-0.17,0.49-0.25C8.68,4.91,8.83,5,9,5c0.28,0,0.5-0.22,0.5-0.5 c0-0.03-0.01-0.06-0.02-0.09c0.39-0.13,0.79-0.23,1.21-0.3C10.58,4.21,10.5,4.34,10.5,4.5C10.5,4.78,10.72,5,11,5 s0.5-0.22,0.5-0.5c0-0.21-0.13-0.38-0.3-0.46C11.46,4.01,11.73,4,12,4s0.54,0.01,0.8,0.04c-0.18,0.08-0.3,0.25-0.3,0.46 C12.5,4.78,12.72,5,13,5s0.5-0.22,0.5-0.5c0-0.16-0.08-0.29-0.19-0.38c0.41,0.07,0.82,0.17,1.21,0.3 C14.51,4.44,14.5,4.47,14.5,4.5C14.5,4.78,14.72,5,15,5c0.17,0,0.32-0.09,0.41-0.23c0.17,0.08,0.33,0.16,0.49,0.25 c-0.23,0.05-0.4,0.24-0.4,0.48C15.5,5.78,15.72,6,16,6s0.5-0.22,0.5-0.5c0-0.05-0.01-0.09-0.03-0.13 c0.71,0.48,1.34,1.08,1.86,1.76C18.25,7.05,18.13,7,18,7c-0.28,0-0.5,0.22-0.5,0.5C17.5,7.78,17.72,8,18,8s0.5-0.22,0.5-0.5 c0-0.09-0.03-0.16-0.07-0.23c0.18,0.24,0.34,0.49,0.49,0.75C18.69,8.05,18.5,8.25,18.5,8.5C18.5,8.78,18.72,9,19,9 c0.14,0,0.26-0.06,0.35-0.15C19.77,9.82,20,10.88,20,12C20,16.41,16.41,20,12,20z",
                    }
                    circle {
                        cy: "13",
                        cx: "9",
                        r: "1.25",
                    }
                    circle {
                        cx: "12",
                        cy: "5.5",
                        r: ".5",
                    }
                    circle {
                        cy: "5.5",
                        r: ".5",
                        cx: "14",
                    }
                    circle {
                        cy: "5.5",
                        r: ".5",
                        cx: "10",
                    }
                    circle {
                        cx: "17",
                        r: ".5",
                        cy: "6.5",
                    }
                    circle {
                        r: ".5",
                        cy: "6.5",
                        cx: "9",
                    }
                    circle {
                        r: ".5",
                        cx: "7",
                        cy: "6.5",
                    }
                    circle {
                        cx: "11",
                        cy: "6.5",
                        r: ".5",
                    }
                    circle {
                        cx: "13",
                        cy: "6.5",
                        r: ".5",
                    }
                    circle {
                        cy: "6.5",
                        cx: "15",
                        r: ".5",
                    }
                    circle {
                        cy: "7.5",
                        r: ".5",
                        cx: "12",
                    }
                    circle {
                        r: ".5",
                        cx: "14",
                        cy: "7.5",
                    }
                    circle {
                        cy: "7.5",
                        cx: "16",
                        r: ".5",
                    }
                    circle {
                        cy: "7.5",
                        cx: "10",
                        r: ".5",
                    }
                    circle {
                        cy: "7.5",
                        cx: "8",
                        r: ".5",
                    }
                    circle {
                        r: ".5",
                        cx: "9",
                        cy: "8.5",
                    }
                    circle {
                        cx: "7",
                        cy: "8.5",
                        r: ".5",
                    }
                    circle {
                        r: ".5",
                        cy: "8.5",
                        cx: "11",
                    }
                    circle {
                        cx: "13",
                        cy: "8.5",
                        r: ".5",
                    }
                    circle {
                        cy: "8.5",
                        r: ".5",
                        cx: "15",
                    }
                    circle {
                        cy: "8.5",
                        r: ".5",
                        cx: "17",
                    }
                    circle {
                        cy: "13",
                        cx: "15",
                        r: "1.25",
                    }
                }
            }
        }
    }
}

pub fn face_6_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,2c-4.42,0-8,3.58-8,8s3.58,8,8,8s8-3.58,8-8S14.42,2,10,2z M10,16.5c-3.58,0-6.5-2.92-6.5-6.5 c0-0.17,0.01-0.34,0.03-0.51c2.39-0.08,3.32-2.16,3.65-2.66C7.32,6.62,7.55,6.5,7.8,6.5h4.39c0.25,0,0.48,0.12,0.62,0.33 c0.34,0.51,1.28,2.58,3.65,2.66C16.49,9.66,16.5,9.83,16.5,10C16.5,13.58,13.58,16.5,10,16.5z",
                    }
                    circle {
                        cy: "10.5",
                        cx: "7.5",
                        r: "1",
                    }
                    circle {
                        cx: "12.5",
                        r: "1",
                        cy: "10.5",
                    }
                }
            }
        }
    }
}

pub fn face_6_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.41,0-8-3.59-8-8c0-0.01,0-0.02,0-0.03 c2.31-0.22,3.43-1.59,4.34-3.41C8.51,8.21,8.85,8,9.24,8h5.53c0.38,0,0.72,0.21,0.89,0.55c0.9,1.8,1.99,3.19,4.34,3.41 c0,0.01,0,0.02,0,0.03C20,16.41,16.41,20,12,20z",
                    }
                    circle {
                        cy: "13",
                        r: "1.25",
                        cx: "9",
                    }
                    circle {
                        cx: "15",
                        cy: "13",
                        r: "1.25",
                    }
                }
            }
        }
    }
}

pub fn female_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10,5.5c1.52,0,2.75,1.23,2.75,2.75S11.52,11,10,11S7.25,9.77,7.25,8.25S8.48,5.5,10,5.5z M9.25,12.43l0,1.57H8.5 c-0.41,0-0.75,0.34-0.75,0.75S8.09,15.5,8.5,15.5h0.75v0.75C9.25,16.66,9.59,17,10,17s0.75-0.34,0.75-0.75V15.5h0.75 c0.41,0,0.75-0.34,0.75-0.75S11.91,14,11.5,14h-0.75v-1.57c1.99-0.35,3.5-2.09,3.5-4.18C14.25,5.9,12.35,4,10,4S5.75,5.9,5.75,8.25 C5.75,10.34,7.26,12.08,9.25,12.43z",
            }
        }
    }
}

pub fn female_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12,6c1.93,0,3.5,1.57,3.5,3.5S13.93,13,12,13s-3.5-1.57-3.5-3.5S10.07,6,12,6z M13,14.91c2.56-0.47,4.5-2.71,4.5-5.41 C17.5,6.46,15.04,4,12,4S6.5,6.46,6.5,9.5c0,2.7,1.94,4.94,4.5,5.41V17h-1c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v1c0,0.55,0.45,1,1,1 s1-0.45,1-1v-1h1c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1V14.91z",
            }
        }
    }
}

pub fn fireplace_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,3H4C3.45,3,3,3.45,3,4v12c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1V4C17,3.45,16.55,3,16,3z M9.45,13.63 c0.57-0.18,1.05-0.78,1.15-1.22c0.1-0.42-0.08-0.78-0.15-1.2c-0.06-0.34-0.05-0.64,0.06-0.96c0.41,0.91,1.61,1.23,1.48,2.39 C11.85,13.9,10.41,14.41,9.45,13.63z M16,15.5c0,0.28-0.22,0.5-0.5,0.5H14v-0.5c0-0.28-0.22-0.5-0.5-0.5h-0.83 c0.81-0.7,1.33-1.71,1.33-2.85c0-1.45-0.87-2.19-1.48-2.59C10.16,8.01,10.8,6,10.8,6C5.42,8.74,5.99,11.74,6,12.15 C6.03,13.1,6.63,14.21,7.53,15H6.5C6.22,15,6,15.22,6,15.5V16H4.5C4.22,16,4,15.78,4,15.5v-11C4,4.22,4.22,4,4.5,4h11 C15.78,4,16,4.22,16,4.5V15.5z",
                }
            }
        }
    }
}

pub fn fireplace_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,19c0,0.55-0.45,1-1,1h-1v-1 c0-0.55-0.45-1-1-1h-1.15c0.71-0.85,1.15-1.89,1.15-3c0-1.89-1.09-2.84-1.85-3.36c-1.86-1.27-2.23-2.78-2.25-3.72 c-0.01-0.4-0.43-0.63-0.77-0.43c-5.8,3.43-5.15,7-5.13,7.51c0.03,0.96,0.49,2.07,1.24,3H7c-0.55,0-1,0.45-1,1v1H5 c-0.55,0-1-0.45-1-1V5c0-0.55,0.45-1,1-1h14c0.55,0,1,0.45,1,1V19z M13.2,13.74c-0.08-0.46-0.07-0.85,0.08-1.28 c0.54,1.21,2.15,1.64,1.98,3.18c-0.19,1.69-2.11,2.37-3.39,1.32c0.76-0.24,1.4-1.04,1.53-1.63C13.52,14.78,13.29,14.29,13.2,13.74z",
                }
            }
        }
    }
}

pub fn fitbit_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M16.31,11.51c0.83,0,1.51-0.68,1.51-1.51s-0.68-1.51-1.51-1.51c-0.83,0-1.51,0.68-1.51,1.51S15.48,11.51,16.31,11.51z M12.92,11.34c0.74,0,1.34-0.6,1.34-1.34s-0.6-1.34-1.34-1.34s-1.34,0.6-1.34,1.34S12.18,11.34,12.92,11.34z M12.92,7.94 c0.74,0,1.34-0.6,1.34-1.34c0-0.74-0.6-1.34-1.34-1.34s-1.34,0.6-1.34,1.34C11.58,7.33,12.18,7.94,12.92,7.94z M12.92,14.74 c0.74,0,1.34-0.6,1.34-1.34c0-0.74-0.6-1.34-1.34-1.34s-1.34,0.6-1.34,1.34C11.58,14.14,12.18,14.74,12.92,14.74z M9.52,11.18 c0.65,0,1.18-0.53,1.18-1.18s-0.53-1.18-1.18-1.18c-0.65,0-1.18,0.53-1.18,1.18S8.87,11.18,9.52,11.18z M9.52,7.77 c0.65,0,1.18-0.53,1.18-1.18s-0.53-1.18-1.18-1.18c-0.65,0-1.18,0.53-1.18,1.18S8.87,7.77,9.52,7.77z M9.52,14.59 c0.65,0,1.18-0.53,1.18-1.18c0-0.65-0.53-1.18-1.18-1.18c-0.65,0-1.18,0.53-1.18,1.18C8.35,14.06,8.87,14.59,9.52,14.59z M9.52,18 c0.65,0,1.18-0.53,1.18-1.18c0-0.65-0.53-1.18-1.18-1.18c-0.65,0-1.18,0.53-1.18,1.18C8.35,17.47,8.87,18,9.52,18z M9.52,4.36 c0.65,0,1.18-0.53,1.18-1.18S10.17,2,9.52,2C8.87,2,8.35,2.53,8.35,3.18S8.87,4.36,9.52,4.36z M6.13,11.01 c0.56,0,1.01-0.45,1.01-1.01S6.69,8.99,6.13,8.99c-0.56,0-1.01,0.45-1.01,1.01S5.57,11.01,6.13,11.01z M6.13,14.41 c0.56,0,1.01-0.45,1.01-1.01c0-0.56-0.45-1.01-1.01-1.01c-0.56,0-1.01,0.45-1.01,1.01C5.12,13.95,5.57,14.41,6.13,14.41z M6.13,7.61 c0.56,0,1.01-0.45,1.01-1.01c0-0.56-0.45-1.01-1.01-1.01c-0.56,0-1.01,0.45-1.01,1.01C5.12,7.16,5.57,7.61,6.13,7.61z M3.04,10.84 c0.46,0,0.84-0.38,0.84-0.84S3.5,9.16,3.04,9.16C2.57,9.16,2.2,9.54,2.2,10S2.57,10.84,3.04,10.84z",
            }
        }
    }
}

pub fn fitbit_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19.89,13.89c1.04,0,1.89-0.85,1.89-1.89s-0.85-1.89-1.89-1.89C18.85,10.11,18,10.96,18,12S18.85,13.89,19.89,13.89z M15.65,13.68c0.93,0,1.68-0.75,1.68-1.68s-0.75-1.68-1.68-1.68c-0.93,0-1.68,0.75-1.68,1.68S14.72,13.68,15.65,13.68z M15.65,9.42 c0.93,0,1.68-0.75,1.68-1.68c0-0.93-0.75-1.68-1.68-1.68c-0.93,0-1.68,0.75-1.68,1.68C13.97,8.67,14.72,9.42,15.65,9.42z M15.65,17.93c0.93,0,1.68-0.75,1.68-1.68c0-0.93-0.75-1.68-1.68-1.68c-0.93,0-1.68,0.75-1.68,1.68 C13.97,17.17,14.72,17.93,15.65,17.93z M11.41,13.47c0.81,0,1.47-0.66,1.47-1.47s-0.66-1.47-1.47-1.47c-0.81,0-1.47,0.66-1.47,1.47 S10.59,13.47,11.41,13.47z M11.41,9.21c0.81,0,1.47-0.66,1.47-1.47s-0.66-1.47-1.47-1.47c-0.81,0-1.47,0.66-1.47,1.47 S10.59,9.21,11.41,9.21z M11.41,17.73c0.81,0,1.47-0.66,1.47-1.47c0-0.81-0.66-1.47-1.47-1.47c-0.81,0-1.47,0.66-1.47,1.47 C9.93,17.07,10.59,17.73,11.41,17.73z M11.41,22c0.81,0,1.47-0.66,1.47-1.47c0-0.81-0.66-1.47-1.47-1.47 c-0.81,0-1.47,0.66-1.47,1.47C9.93,21.34,10.59,22,11.41,22z M11.41,4.94c0.81,0,1.47-0.66,1.47-1.47S12.22,2,11.41,2 c-0.81,0-1.47,0.66-1.47,1.47S10.59,4.94,11.41,4.94z M7.16,13.26c0.7,0,1.26-0.57,1.26-1.26s-0.57-1.26-1.26-1.26 c-0.7,0-1.26,0.57-1.26,1.26S6.46,13.26,7.16,13.26z M7.16,17.51c0.7,0,1.26-0.57,1.26-1.26c0-0.7-0.57-1.26-1.26-1.26 c-0.7,0-1.26,0.57-1.26,1.26C5.9,16.94,6.46,17.51,7.16,17.51z M7.16,9.02c0.7,0,1.26-0.57,1.26-1.26c0-0.7-0.57-1.26-1.26-1.26 c-0.7,0-1.26,0.57-1.26,1.26C5.9,8.45,6.46,9.02,7.16,9.02z M3.29,13.05c0.58,0,1.05-0.47,1.05-1.05s-0.47-1.05-1.05-1.05 c-0.58,0-1.05,0.47-1.05,1.05S2.71,13.05,3.29,13.05z",
            }
        }
    }
}

pub fn flood_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M17.47,16.41c-0.57-0.21-0.9-0.71-2.13-0.71c-1.56,0-1.67,0.8-2.66,0.8c-0.96,0-1.13-0.8-2.66-0.8 c-1.56,0-1.68,0.8-2.67,0.8c-1,0-1.1-0.8-2.66-0.8c-1.23,0-1.57,0.5-2.14,0.71C2.22,16.52,2,16.8,2,17.13 c0,0.52,0.52,0.89,1.01,0.72c0.68-0.23,0.95-0.65,1.66-0.65c1,0,1.1,0.8,2.66,0.8c1.56,0,1.68-0.8,2.67-0.8 c0.96,0,1.13,0.8,2.66,0.8c1.55,0,1.68-0.8,2.66-0.8c0.69,0,0.97,0.42,1.66,0.65c0.49,0.17,1.01-0.2,1.01-0.72 C18,16.8,17.78,16.52,17.47,16.41z",
                    }
                    path {
                        d: "M3.01,14.85c0.68-0.23,0.95-0.65,1.66-0.65c1,0,1.1,0.8,2.66,0.8c1.56,0,1.68-0.8,2.67-0.8c0.96,0,1.13,0.8,2.66,0.8 c1.55,0,1.68-0.8,2.66-0.8c0.75,0,0.83,0.5,1.8,0.71c0.45,0.1,0.87-0.26,0.87-0.72c0-0.34-0.23-0.61-0.53-0.71 c0,0-1.85-6.88-1.85-6.88l0.58,0.23c0.38,0.15,0.82-0.03,0.98-0.42v0c0.15-0.38-0.03-0.82-0.42-0.98L8.87,2.28 C8.46,2.11,7.99,2.24,7.71,2.59L2.46,9.28c-0.26,0.33-0.2,0.8,0.13,1.05c0.33,0.26,0.8,0.2,1.05-0.13l0.39-0.49l0.8,3 c-0.06,0-0.1-0.01-0.16-0.01c-1.23,0-1.57,0.5-2.14,0.71C2.22,13.52,2,13.8,2,14.13C2,14.65,2.52,15.02,3.01,14.85z M12.05,8.73 l1.24,4.65c-1.28,0.56-1.53-0.67-3.28-0.67c-0.43,0-0.74,0.06-1,0.15L8.18,9.76L12.05,8.73z",
                    }
                }
            }
        }
    }
}

pub fn flood_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M18.67,19c-1.95,0-2.09,1-3.33,1c-1.19,0-1.42-1-3.33-1c-1.95,0-2.1,1-3.34,1c-1.24,0-1.38-1-3.33-1 c-1.54,0-1.96,0.62-2.67,0.88C2.27,20.03,2,20.4,2,20.83v0c0,0.71,0.72,1.19,1.38,0.94C4.15,21.48,4.49,21,5.34,21 c1.24,0,1.38,1,3.33,1c1.95,0,2.1-1,3.34-1c1.22,0,1.4,1,3.33,1c1.93,0,2.1-1,3.33-1c0.84,0,1.18,0.47,1.95,0.77 c0.66,0.26,1.38-0.23,1.38-0.94v-0.01c0-0.42-0.27-0.8-0.67-0.94C20.62,19.62,20.21,19,18.67,19z",
                    }
                    path {
                        d: "M8.68,17.5c1.95,0,2.09-1,3.33-1c1.19,0,1.42,1,3.33,1c1.95,0,2.09-1,3.33-1c0.82,0,1.17,0.46,1.93,0.76 c0.66,0.26,1.38-0.23,1.38-0.94v0c0-0.42-0.26-0.79-0.65-0.94c-0.29-0.11-0.54-0.27-0.83-0.43l-2.02-7.53l1.17,0.47 c0.51,0.21,1.09-0.04,1.29-0.55v0c0.21-0.51-0.05-1.1-0.57-1.29l-9.24-3.54c-0.81-0.31-1.72-0.06-2.27,0.61l-6.23,7.7 c-0.35,0.43-0.28,1.06,0.16,1.4l0,0c0.43,0.34,1.06,0.26,1.39-0.17l0.78-1l0.93,3.48c-0.18-0.02-0.35-0.05-0.56-0.05 c-1.54,0-1.95,0.62-2.66,0.88C2.27,15.53,2,15.91,2,16.33v0c0,0.7,0.69,1.19,1.35,0.95c0.8-0.29,1.18-0.78,2-0.78 C6.54,16.5,6.77,17.5,8.68,17.5z M14.04,10.18l1.42,5.31c-1.34,0.09-1.47-0.99-3.47-0.99c-0.36,0-0.65,0.04-0.91,0.1l-0.91-3.39 L14.04,10.18z",
                    }
                }
            }
        }
    }
}

pub fn follow_the_signs_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9.5,5.5c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S8.4,5.5,9.5,5.5z M5.75,8.9L3.23,21.81C3.11,22.43,3.58,23,4.21,23H4.3 c0.47,0,0.88-0.33,0.98-0.79L6.85,15L9,17v5c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-6.14c0-0.27-0.11-0.52-0.29-0.71L8.95,13.4 l0.6-3c1.07,1.32,2.58,2.23,4.31,2.51c0.6,0.1,1.14-0.39,1.14-1v0c0-0.49-0.36-0.9-0.84-0.98c-1.49-0.25-2.75-1.15-3.51-2.38 L9.7,6.95C9.35,6.35,8.7,6,8,6C7.75,6,7.5,6.05,7.25,6.15l-4.63,1.9C2.25,8.2,2,8.57,2,8.97V12c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1V9.65L5.75,8.9 M21,2h-7c-0.55,0-1,0.45-1,1v5c0,0.55,0.45,1,1,1h2.75v13.25c0,0.41,0.34,0.75,0.75,0.75 s0.75-0.34,0.75-0.75V9H21c0.55,0,1-0.45,1-1V3C22,2.45,21.55,2,21,2z M20.15,5.85l-1.28,1.29c-0.31,0.32-0.85,0.09-0.85-0.35V6.25 h-2.76c-0.41,0-0.75-0.34-0.75-0.75s0.34-0.75,0.75-0.75h2.76V4.21c0-0.45,0.54-0.67,0.85-0.35l1.28,1.29 C20.34,5.34,20.34,5.66,20.15,5.85z",
            }
        }
    }
}

pub fn front_hand_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M11.5,8.5C11.5,8.78,11.72,9,12,9h0c0.28,0,0.5-0.22,0.5-0.5v-6c0-0.55,0.45-1,1-1s1,0.45,1,1l0,9.04 c-1.51,0.22-2.71,1.4-2.95,2.89c-0.05,0.3,0.19,0.57,0.5,0.57h0c0.24,0,0.45-0.17,0.49-0.41c0.2-1.18,1.23-2.09,2.47-2.09h0 c0.28,0,0.5-0.22,0.5-0.5V7.5c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1V13c0,3.87-3.13,7-7,7s-7-3.13-7-7V5c0-0.55,0.45-1,1-1 s1,0.45,1,1v4.5C5.5,9.78,5.72,10,6,10h0c0.28,0,0.5-0.22,0.5-0.5V2.25c0-0.55,0.45-1,1-1s1,0.45,1,1V8.5C8.5,8.78,8.72,9,9,9h0 c0.28,0,0.5-0.22,0.5-0.5V1c0-0.55,0.45-1,1-1s1,0.45,1,1V8.5z",
            }
        }
    }
}

pub fn front_hand_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.5,14.5c0,0.28-0.22,0.5-0.5,0.5l0,0c-1.51,0-2.77,1.12-2.97,2.58C15,17.82,14.78,18,14.54,18c-0.3,0-0.54-0.27-0.5-0.57 c0.26-1.77,1.68-3.17,3.46-3.4l0-10.78C17.5,2.56,16.94,2,16.25,2C15.56,2,15,2.56,15,3.25v7.25c0,0.28-0.22,0.5-0.5,0.5 S14,10.78,14,10.5V1.25C14,0.56,13.44,0,12.75,0S11.5,0.56,11.5,1.25v9.25c0,0.28-0.22,0.5-0.5,0.5s-0.5-0.22-0.5-0.5V2.75 c0-0.69-0.56-1.25-1.25-1.25S8,2.06,8,2.75v8.75C8,11.78,7.78,12,7.5,12S7,11.78,7,11.5V5.75C7,5.06,6.44,4.5,5.75,4.5 S4.5,5.06,4.5,5.75v10c0,4.56,3.69,8.25,8.25,8.25S21,20.31,21,15.75v-6.5C21,8.56,20.44,8,19.75,8S18.5,8.56,18.5,9.25V14.5z",
            }
        }
    }
}

pub fn girl_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,6.5c0.69,0,1.25-0.56,1.25-1.25S10.69,4,10,4S8.75,4.56,8.75,5.25S9.31,6.5,10,6.5z M10,7.5 c0.48,0,0.92,0.29,1.1,0.74l1.63,4.07c0.13,0.33-0.11,0.69-0.46,0.69H11.5v2.5c0,0.28-0.22,0.5-0.5,0.5H9 c-0.28,0-0.5-0.22-0.5-0.5V13H7.74c-0.35,0-0.6-0.36-0.46-0.69L8.9,8.24C9.08,7.79,9.52,7.5,10,7.5z",
                    }
                }
            }
        }
    }
}

pub fn girl_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,7.5c0.97,0,1.75-0.78,1.75-1.75S12.97,4,12,4s-1.75,0.78-1.75,1.75S11.03,7.5,12,7.5z M14,16v3c0,0.55-0.45,1-1,1h-2 c-0.55,0-1-0.45-1-1v-3H9.44c-0.7,0-1.18-0.7-0.94-1.35l1.88-5.03C10.63,8.95,11.28,8.5,12,8.5s1.37,0.45,1.62,1.12l1.88,5.03 C15.74,15.3,15.26,16,14.56,16H14z",
                    }
                }
            }
        }
    }
}

pub fn group_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5s-3 1.34-3 3 1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V18c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-1.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05.02.01.03.03.04.04 1.14.83 1.93 1.94 1.93 3.41V18c0 .35-.07.69-.18 1H22c.55 0 1-.45 1-1v-1.5c0-2.33-4.67-3.5-7-3.5z",
            }
        }
    }
}

pub fn groups_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                path {
                    d: "M12,12.75c1.63,0,3.07,0.39,4.24,0.9c1.08,0.48,1.76,1.56,1.76,2.73L18,17c0,0.55-0.45,1-1,1H7c-0.55,0-1-0.45-1-1l0-0.61 c0-1.18,0.68-2.26,1.76-2.73C8.93,13.14,10.37,12.75,12,12.75z M4,13c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2 C2,12.1,2.9,13,4,13z M5.13,14.1C4.76,14.04,4.39,14,4,14c-0.99,0-1.93,0.21-2.78,0.58C0.48,14.9,0,15.62,0,16.43L0,17 c0,0.55,0.45,1,1,1l3.5,0v-1.61C4.5,15.56,4.73,14.78,5.13,14.1z M20,13c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2 C18,12.1,18.9,13,20,13z M24,16.43c0-0.81-0.48-1.53-1.22-1.85C21.93,14.21,20.99,14,20,14c-0.39,0-0.76,0.04-1.13,0.1 c0.4,0.68,0.63,1.46,0.63,2.29V18l3.5,0c0.55,0,1-0.45,1-1L24,16.43z M12,6c1.66,0,3,1.34,3,3c0,1.66-1.34,3-3,3s-3-1.34-3-3 C9,7.34,10.34,6,12,6z",
                }
            }
        }
    }
}

pub fn groups_2_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M8.22,10h3.56c0.63,0,1.1-0.58,0.98-1.2l-0.37-1.84C12.16,5.82,11.16,5,10,5S7.84,5.82,7.61,6.96L7.24,8.8 C7.12,9.42,7.59,10,8.22,10z",
                    }
                    path {
                        d: "M1.63,9.49C1.52,9.7,1.49,9.95,1.54,10.2c0.13,0.55,0.61,0.83,1.22,0.8c0,0,1.19,0,1.56,0C5,11,5.54,10.54,5.54,9.97 c0-0.11-0.02-0.22-0.06-0.32c-0.01-0.02-0.01-0.04,0-0.06c0.07-0.13,0.11-0.27,0.11-0.42c0-0.25-0.11-0.48-0.29-0.66 C5.28,8.49,5.28,8.46,5.29,8.43c0.05-0.16,0.06-0.34,0.01-0.52C5.18,7.56,4.86,7.32,4.51,7.3c-0.02,0-0.04-0.01-0.05-0.03 C4.32,7.11,4.08,7,3.8,7c-0.24,0-0.46,0.08-0.6,0.21C3.18,7.23,3.15,7.23,3.12,7.22C3.01,7.18,2.89,7.15,2.76,7.15 c-0.52,0-0.94,0.39-0.99,0.9c0,0.02-0.01,0.03-0.02,0.05C1.51,8.31,1.38,8.62,1.42,8.94c0.02,0.18,0.09,0.34,0.2,0.48 C1.64,9.44,1.64,9.47,1.63,9.49z",
                    }
                    path {
                        d: "M16.5,11c0.83,0,1.5-0.67,1.5-1.5c0-0.04-0.01-0.07-0.01-0.11H18V9.01c0-0.83-0.68-1.51-1.51-1.51h-1.61 c-0.32,0-0.49,0.37-0.29,0.61l0.65,0.58C15.09,8.93,15,9.2,15,9.5C15,10.33,15.67,11,16.5,11z",
                    }
                    path {
                        d: "M13.9,11.93C12.87,11.41,11.54,11,10,11c-1.54,0-2.87,0.41-3.9,0.93C5.42,12.26,5,12.96,5,13.72L5,14c0,0.55,0.45,1,1,1h8 c0.55,0,1-0.45,1-1l0-0.28C15,12.96,14.58,12.26,13.9,11.93z",
                    }
                    path {
                        d: "M18.74,12.01c-0.64-0.25-1.4-0.41-2.24-0.41c-0.44,0-0.86,0.05-1.25,0.12c0.48,0.54,0.75,1.24,0.75,1.99L16,15h3 c0.55,0,1-0.45,1-1l0-0.13C20,13.05,19.5,12.31,18.74,12.01z",
                    }
                    path {
                        d: "M3.5,11.6c-0.85,0-1.6,0.16-2.24,0.41C0.5,12.31,0,13.05,0,13.87L0,14c0,0.55,0.45,1,1,1h3l0-1.28 c0-0.76,0.28-1.45,0.75-1.99C4.36,11.65,3.94,11.6,3.5,11.6z",
                    }
                }
            }
        }
    }
}

pub fn groups_2_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M10.27,12h3.46c0.93,0,1.63-0.83,1.48-1.75l-0.3-1.79C14.67,7.04,13.44,6,12,6S9.33,7.04,9.09,8.47l-0.3,1.79 C8.64,11.17,9.34,12,10.27,12z",
                    }
                    path {
                        d: "M1.66,11.11c-0.13,0.26-0.18,0.57-0.1,0.88c0.16,0.69,0.76,1.03,1.53,1c0,0,1.49,0,1.95,0c0.83,0,1.51-0.58,1.51-1.29 c0-0.14-0.03-0.27-0.07-0.4c-0.01-0.03-0.01-0.05,0.01-0.08c0.09-0.16,0.14-0.34,0.14-0.53c0-0.31-0.14-0.6-0.36-0.82 c-0.03-0.03-0.03-0.06-0.02-0.1c0.07-0.2,0.07-0.43,0.01-0.65C6.1,8.69,5.71,8.4,5.27,8.38c-0.03,0-0.05-0.01-0.07-0.03 C5.03,8.14,4.72,8,4.37,8C4.07,8,3.8,8.1,3.62,8.26C3.59,8.29,3.56,8.29,3.53,8.28c-0.14-0.06-0.3-0.09-0.46-0.09 c-0.65,0-1.18,0.49-1.24,1.12c0,0.02-0.01,0.04-0.03,0.06c-0.29,0.26-0.46,0.65-0.41,1.05c0.03,0.22,0.12,0.43,0.25,0.6 C1.67,11.04,1.67,11.08,1.66,11.11z",
                    }
                    path {
                        d: "M16.24,13.65c-1.17-0.52-2.61-0.9-4.24-0.9c-1.63,0-3.07,0.39-4.24,0.9C6.68,14.13,6,15.21,6,16.39L6,17 c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1l0-0.61C18,15.21,17.32,14.13,16.24,13.65z",
                    }
                    path {
                        d: "M1.22,14.58C0.48,14.9,0,15.62,0,16.43L0,17c0,0.55,0.45,1,1,1l3.5,0v-1.61c0-0.83,0.23-1.61,0.63-2.29 C4.76,14.04,4.39,14,4,14C3.01,14,2.07,14.21,1.22,14.58z",
                    }
                    path {
                        d: "M22.78,14.58C21.93,14.21,20.99,14,20,14c-0.39,0-0.76,0.04-1.13,0.1c0.4,0.68,0.63,1.46,0.63,2.29V18l3.5,0 c0.55,0,1-0.45,1-1l0-0.57C24,15.62,23.52,14.9,22.78,14.58z",
                    }
                    path {
                        d: "M22,11v-0.5c0-1.1-0.9-2-2-2h-2c-0.42,0-0.65,0.48-0.39,0.81l0.7,0.63C18.12,10.25,18,10.61,18,11c0,1.1,0.9,2,2,2 S22,12.1,22,11z",
                    }
                }
            }
        }
    }
}

pub fn groups_3_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        width: "2.83",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -4.6317 4.818)",
                        x: "2.09",
                        height: "2.83",
                        y: "6.59",
                    }
                    polygon {
                        points: "16.5,6.5 14.5,10 18.5,10",
                    }
                    path {
                        d: "M13.9,10.93C12.87,10.41,11.54,10,10,10c-1.54,0-2.87,0.41-3.9,0.93C5.42,11.26,5,11.96,5,12.72L5,13c0,0.55,0.45,1,1,1h8 c0.55,0,1-0.45,1-1l0-0.28C15,11.96,14.58,11.26,13.9,10.93z",
                    }
                    path {
                        d: "M18.74,11.01c-0.64-0.25-1.4-0.41-2.24-0.41c-0.44,0-0.86,0.05-1.25,0.12c0.48,0.54,0.75,1.24,0.75,1.99L16,14h3 c0.55,0,1-0.45,1-1l0-0.13C20,12.05,19.5,11.31,18.74,11.01z",
                    }
                    path {
                        d: "M3.5,10.6c-0.85,0-1.6,0.16-2.24,0.41C0.5,11.31,0,12.05,0,12.87L0,13c0,0.55,0.45,1,1,1h3l0-1.28 c0-0.76,0.28-1.45,0.75-1.99C4.36,10.65,3.94,10.6,3.5,10.6z",
                    }
                    path {
                        d: "M10,9c1.38,0,2.5-1.12,2.5-2.5C12.5,5.12,11.38,4,10,4S7.5,5.12,7.5,6.5C7.5,7.88,8.62,9,10,9z",
                    }
                }
            }
        }
    }
}

pub fn groups_3_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M16.24,13.65c-1.17-0.52-2.61-0.9-4.24-0.9c-1.63,0-3.07,0.39-4.24,0.9C6.68,14.13,6,15.21,6,16.39L6,17 c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1l0-0.61C18,15.21,17.32,14.13,16.24,13.65z",
                    }
                    path {
                        d: "M1.22,14.58C0.48,14.9,0,15.62,0,16.43L0,17c0,0.55,0.45,1,1,1l3.5,0v-1.61c0-0.83,0.23-1.61,0.63-2.29 C4.76,14.04,4.39,14,4,14C3.01,14,2.07,14.21,1.22,14.58z",
                    }
                    path {
                        d: "M22.78,14.58C21.93,14.21,20.99,14,20,14c-0.39,0-0.76,0.04-1.13,0.1c0.4,0.68,0.63,1.46,0.63,2.29V18l3.5,0 c0.55,0,1-0.45,1-1l0-0.57C24,15.62,23.52,14.9,22.78,14.58z",
                    }
                    path {
                        d: "M12,12c1.66,0,3-1.34,3-3c0-1.66-1.34-3-3-3S9,7.34,9,9C9,10.66,10.34,12,12,12z",
                    }
                    rect {
                        x: "2.23",
                        y: "9.23",
                        width: "3.54",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -6.6066 6.0503)",
                        height: "3.54",
                    }
                    polygon {
                        points: "20,9 17.5,13 22.5,13",
                    }
                }
            }
        }
    }
}

pub fn group_add_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        circle {
                            cx: "6",
                            cy: "7",
                            r: "3",
                        }
                    }
                    g {
                        path {
                            d: "M11.03,12.37C9.56,11.5,7.84,11,6,11s-3.56,0.5-5.03,1.37C0.36,12.72,0,13.39,0,14.09V16h12v-1.91 C12,13.39,11.64,12.72,11.03,12.37z",
                        }
                    }
                    g {
                        path {
                            d: "M15.03,12.37c-0.92-0.54-1.94-0.92-3.02-1.14c0.92,0.64,1.48,1.71,1.48,2.86V16H16v-1.91 C16,13.39,15.64,12.72,15.03,12.37z",
                        }
                    }
                    g {
                        path {
                            d: "M10,10c1.66,0,3-1.34,3-3s-1.34-3-3-3C9.79,4,9.58,4.02,9.38,4.06C10.07,4.85,10.5,5.87,10.5,7 c0,1.13-0.43,2.14-1.12,2.93C9.58,9.98,9.79,10,10,10z",
                        }
                    }
                    g {
                        path {
                            d: "M18.75,7.75h-1v-1C17.75,6.34,17.41,6,17,6l0,0c-0.41,0-0.75,0.34-0.75,0.75v1h-1c-0.41,0-0.75,0.34-0.75,0.75v0 c0,0.41,0.34,0.75,0.75,0.75h1v1c0,0.41,0.34,0.75,0.75,0.75l0,0c0.41,0,0.75-0.34,0.75-0.75v-1h1c0.41,0,0.75-0.34,0.75-0.75v0 C19.5,8.09,19.16,7.75,18.75,7.75z",
                        }
                    }
                }
            }
        }
    }
}

pub fn group_add_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M22,9V8c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v1h-1c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1v1c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-1h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H22z",
                }
                g {
                    path {
                        d: "M8,12c2.21,0,4-1.79,4-4s-1.79-4-4-4S4,5.79,4,8S5.79,12,8,12z",
                    }
                    path {
                        d: "M8,13c-2.67,0-8,1.34-8,4v3h16v-3C16,14.34,10.67,13,8,13z",
                    }
                    path {
                        d: "M12.51,4.05C13.43,5.11,14,6.49,14,8s-0.57,2.89-1.49,3.95C14.47,11.7,16,10.04,16,8S14.47,4.3,12.51,4.05z",
                    }
                    path {
                        d: "M16.53,13.83C17.42,14.66,18,15.7,18,17v3h2v-3C20,15.55,18.41,14.49,16.53,13.83z",
                    }
                }
            }
        }
    }
}

pub fn group_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10.61,8.49C10.86,8.05,11,7.54,11,7c0-1.66-1.34-3-3-3C7.46,4,6.95,4.14,6.51,4.39L10.61,8.49z M11.68,9.56 C12.19,8.83,12.5,7.95,12.5,7c0-1.13-0.43-2.15-1.12-2.94C11.58,4.02,11.79,4,12,4c1.66,0,3,1.34,3,3c0,1.62-1.28,2.94-2.88,3 L11.68,9.56z M15.41,13.29c-0.2-0.83-0.69-1.57-1.39-2.06c1.08,0.22,2.1,0.6,3.02,1.14c0.6,0.36,0.97,1.02,0.97,1.72V15 c0,0.25-0.09,0.47-0.23,0.64L15.41,13.29z M1.69,3.81c-0.29-0.29-0.29-0.77,0-1.06s0.77-0.29,1.06,0l14.5,14.5 c0.29,0.29,0.29,0.77,0,1.06c-0.29,0.29-0.77,0.29-1.06,0l-2.54-2.54C13.47,15.91,13.25,16,13,16H3c-0.55,0-1-0.45-1-1v-0.91 c0-0.7,0.36-1.36,0.97-1.72C4.44,11.5,6.16,11,8,11c0.31,0,0.62,0.01,0.92,0.04l0,0L7.88,10C6.32,9.93,5.07,8.68,5,7.12L1.69,3.81z",
            }
        }
    }
}

pub fn group_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15,8c0-1.42-0.5-2.73-1.33-3.76C14.09,4.1,14.53,4,15,4c2.21,0,4,1.79,4,4s-1.79,4-4,4c-0.06,0-0.12,0-0.18,0l-0.77-0.77 C14.65,10.29,15,9.18,15,8z M7.24,4.41C7.77,4.15,8.37,4,9,4c2.21,0,4,1.79,4,4c0,0.63-0.15,1.23-0.41,1.76L7.24,4.41z M21.19,21.19 c0.39,0.39,0.39,1.02,0,1.41s-1.02,0.39-1.41,0l-2.99-2.99C16.6,19.85,16.32,20,16,20H2c-0.55,0-1-0.45-1-1v-2c0-2.66,5.33-4,8-4 c0.37,0,0.8,0.03,1.25,0.08L9.17,12C9.11,12,9.06,12,9,12c-2.21,0-4-1.79-4-4c0-0.06,0-0.11,0-0.17L1.39,4.22 C1,3.83,1,3.2,1.39,2.81s1.02-0.39,1.41,0L21.19,21.19z M18.89,16.07c-0.29-1.22-1.13-2.19-2.23-2.94C19.42,13.53,23,14.82,23,17v2 c0,0.32-0.15,0.6-0.38,0.79L18.89,16.07z",
            }
        }
    }
}

pub fn group_remove_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M6,10c1.66,0,3-1.34,3-3S7.66,4,6,4S3,5.34,3,7S4.34,10,6,10z M11.03,12.37C9.56,11.5,7.84,11,6,11s-3.56,0.5-5.03,1.37 C0.36,12.72,0,13.39,0,14.09V16h12v-1.91C12,13.39,11.64,12.72,11.03,12.37z M16,14.09V16h-2.5v-1.91c0-1.15-0.56-2.22-1.48-2.86 c1.08,0.22,2.1,0.6,3.02,1.14C15.64,12.72,16,13.39,16,14.09z M9.38,9.93C10.07,9.14,10.5,8.13,10.5,7s-0.43-2.15-1.12-2.94 C9.58,4.02,9.79,4,10,4c1.66,0,3,1.34,3,3s-1.34,3-3,3C9.79,10,9.58,9.98,9.38,9.93z M14.5,8.5c0-0.41,0.34-0.75,0.75-0.75h3.5 c0.41,0,0.75,0.34,0.75,0.75s-0.34,0.75-0.75,0.75h-3.5C14.84,9.25,14.5,8.91,14.5,8.5z",
            }
        }
    }
}

pub fn group_remove_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18,10c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1s-0.45,1-1,1h-4C18.45,11,18,10.55,18,10z M8,4C5.79,4,4,5.79,4,8s1.79,4,4,4 s4-1.79,4-4S10.21,4,8,4z M8,13c-2.67,0-8,1.34-8,4v3h16v-3C16,14.34,10.67,13,8,13z M12.51,4.05C13.43,5.11,14,6.49,14,8 s-0.57,2.89-1.49,3.95C14.47,11.7,16,10.04,16,8S14.47,4.3,12.51,4.05z M16.53,13.83C17.42,14.66,18,15.7,18,17v3h2v-3 C20,15.55,18.41,14.49,16.53,13.83z",
            }
        }
    }
}

pub fn handshake_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17.62,10.17c1.17-1.17,1.17-3.07,0-4.24l-2.55-2.55c-1.17-1.17-3.07-1.17-4.24,0l-0.27,0.27l3.59,3.58 c0.62,0.62,0.62,1.64,0,2.26c-0.62,0.62-1.64,0.62-2.26,0l-3-3l-4.55,4.55c-0.31,0.31-0.31,0.82,0,1.13c0.31,0.31,0.82,0.31,1.13,0 l3.79-3.79l0.57,0.57l-3.79,3.79c-0.31,0.31-0.31,0.82,0,1.13c0.31,0.31,0.82,0.31,1.13,0l3.79-3.79l0.57,0.57l-3.79,3.79 c-0.31,0.31-0.31,0.82,0,1.13c0.31,0.31,0.82,0.31,1.13,0l3.79-3.79l0.57,0.57l-3.79,3.79c-0.31,0.31-0.31,0.82,0,1.13 c0.31,0.31,0.82,0.31,1.13,0L17.62,10.17z M4.48,4.87 M2.38,5.91c-1.17,1.17-1.17,3.07,0,4.24L3.23,11l5.64-5.64l3.57,3.57 c0.31,0.31,0.83,0.31,1.14,0c0.31-0.31,0.31-0.82,0-1.13L9.15,3.38c-1.17-1.17-3.07-1.17-4.24,0L2.38,5.91z",
                }
            }
        }
    }
}

pub fn handshake_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M16.48,10.41c-0.39,0.39-1.04,0.39-1.43,0l-4.47-4.46l-7.05,7.04l-0.66-0.63c-1.17-1.17-1.17-3.07,0-4.24l4.24-4.24 c1.17-1.17,3.07-1.17,4.24,0L16.48,9C16.87,9.39,16.87,10.02,16.48,10.41z M17.18,8.29c0.78,0.78,0.78,2.05,0,2.83 c-1.27,1.27-2.61,0.22-2.83,0l-3.76-3.76l-5.57,5.57c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.42,0l4.62-4.62l0.71,0.71 l-4.62,4.62c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.42,0l4.62-4.62l0.71,0.71l-4.62,4.62c-0.39,0.39-0.39,1.02,0,1.41 c0.39,0.39,1.02,0.39,1.41,0l4.62-4.62l0.71,0.71l-4.62,4.62c-0.39,0.39-0.39,1.02,0,1.41c0.39,0.39,1.02,0.39,1.41,0l8.32-8.34 c1.17-1.17,1.17-3.07,0-4.24l-4.24-4.24c-1.15-1.15-3.01-1.17-4.18-0.06L17.18,8.29z",
                }
            }
        }
    }
}

pub fn health_and_safety_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M10.5,13h-1C8.67,13,8,12.33,8,11.5v0C8,10.67,8.67,10,9.5,10h1V9c0-0.83,0.67-1.5,1.5-1.5h0c0.83,0,1.5,0.67,1.5,1.5v1h1 c0.83,0,1.5,0.67,1.5,1.5v0c0,0.83-0.67,1.5-1.5,1.5h-1v1c0,0.83-0.67,1.5-1.5,1.5h0c-0.83,0-1.5-0.67-1.5-1.5V13z M11.3,2.26 l-6,2.25C4.52,4.81,4,5.55,4,6.39v4.7c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91v-4.7c0-0.83-0.52-1.58-1.3-1.87l-6-2.25 C12.25,2.09,11.75,2.09,11.3,2.26z",
            }
        }
    }
}

pub fn heart_broken_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M13.6,3c-0.77,0-1.52,0.2-2.18,0.56L10,8h1.13c0.68,0,1.16,0.66,0.95,1.3l-1.16,3.66c-0.09,0.29-0.53,0.19-0.49-0.11L11,9 H9.77C9.12,9,8.65,8.39,8.8,7.77L9.63,4.3C8.77,3.49,7.61,3,6.4,3C3.94,3,2,4.95,2,7.43c0,3.08,2.86,5.42,6.99,9.15 c0.57,0.52,1.44,0.52,2.02,0.01C14.92,13.1,18,10.58,18,7.43C18,4.95,16.06,3,13.6,3z",
                    }
                }
            }
        }
    }
}

pub fn heart_broken_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M19.57,3.95c-1.92-1.29-4.08-1.17-5.8-0.26L12,9h1.66c0.67,0,1.15,0.65,0.96,1.29l-1.82,6.07c-0.09,0.29-0.52,0.2-0.49-0.1 L13,10h-1.67c-0.66,0-1.14-0.64-0.96-1.27l1.18-4.11c0,0,0,0,0,0C9.7,2.89,6.71,2.32,4.27,4.04C2.82,5.07,2,6.7,2,8.49 c-0.01,3.81,3.53,6.71,8.66,11.3c0.76,0.68,1.92,0.69,2.69,0.01c4.98-4.42,8.87-7.58,8.64-11.62C21.9,6.45,21,4.92,19.57,3.95z",
                }
            }
        }
    }
}

pub fn hiking_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M9.25,3.75C9.25,2.78,10.03,2,11,2s1.75,0.78,1.75,1.75c0,0.97-0.79,1.75-1.75,1.75C10.04,5.5,9.25,4.72,9.25,3.75z M4.9,10.75l1.64,0.32l0.96-4.91L6.84,6.03C6.03,5.87,5.24,6.4,5.08,7.21l-0.57,2.94C4.46,10.43,4.63,10.69,4.9,10.75z M14.5,8 C14.22,8,14,8.22,14,8.5l0,0.91c-2.11-0.42-2.13-2.19-3.11-2.97c-0.21-0.17-0.45-0.29-0.7-0.36c-0.81-0.23-1.7,0.27-1.88,1.16 c-0.13,0.67-1.46,7.41-1.94,9.87C6.29,17.57,6.64,18,7.11,18c0.36,0,0.67-0.25,0.73-0.6l1.07-5.41l1.58,1.58v3.68 c0,0.41,0.34,0.75,0.75,0.75S12,17.66,12,17.25v-3.89c0-0.27-0.11-0.52-0.29-0.71l-1.17-1.17l0.49-2.48 c0.72,0.95,1.76,1.64,2.97,1.89l0,6.6c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5v-9C15,8.22,14.78,8,14.5,8z",
            }
        }
    }
}

pub fn hiking_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13.5,5.5c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S12.4,5.5,13.5,5.5z M18.25,9c-0.41,0-0.75,0.34-0.75,0.75l0,1.03 c-1.23-0.37-2.22-1.17-2.8-2.18l-1-1.6c-0.41-0.65-1.11-1-1.84-1c-0.78,0-1.59,0.5-1.78,1.44L7.25,21.76C7.12,22.4,7.61,23,8.27,23 c0.49,0,0.91-0.34,1.02-0.81L10.9,15l2.1,2v5c0,0.55,0.45,1,1,1s1-0.45,1-1v-5.64c0-0.55-0.22-1.07-0.62-1.45L12.9,13.5l0.6-3 c1,1.15,2.41,2.01,4,2.34v9.41c0,0.41,0.34,0.75,0.75,0.75S19,22.66,19,22.25V9.75C19,9.34,18.66,9,18.25,9z M7.43,13.13l-2.12-0.41 c-0.54-0.11-0.9-0.63-0.79-1.17l0.76-3.93c0.21-1.08,1.26-1.79,2.34-1.58l1.16,0.23L7.43,13.13z",
            }
        }
    }
}

pub fn history_edu_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M7.5,4C7.22,4,7,4.22,7,4.5v0.23c-0.68-0.32-1.4-0.5-2.13-0.5c-1.1,0-2.21,0.36-3.12,1.09C1.52,5.51,1.5,5.86,1.71,6.07 l1.75,1.75h1.41L4.76,9.13c0.62,0.62,1.42,0.92,2.23,0.92c0,0,0.01,0,0.01,0V12H5.5C5.22,12,5,12.22,5,12.5v2 C5,15.33,5.67,16,6.5,16H14c1.1,0,2-0.9,2-2V4.5C16,4.22,15.78,4,15.5,4H7.5z M6.99,9.05c-0.43,0-0.84-0.12-1.19-0.36l0.15-1.87 H3.87L2.84,5.79c0.61-0.36,1.31-0.55,2.03-0.55c1.07,0,2.07,0.42,2.83,1.17l1.41,1.41l-0.6,0.6C8.1,8.82,7.56,9.05,6.99,9.05z M15,14c0,0.61-0.54,1.09-1.17,0.99C13.34,14.91,13,14.44,13,13.94l0-1.44c0-0.28-0.22-0.5-0.5-0.5H8V9.87 c0.44-0.15,0.86-0.39,1.22-0.74l0.6-0.6L12.29,11H13v-0.71L8.4,5.7C8.28,5.57,8.14,5.48,8,5.37V5h7V14z",
                    }
                }
            }
        }
    }
}

pub fn history_edu_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M9,5v0.38c-0.83-0.33-1.72-0.5-2.61-0.5c-1.42,0-2.84,0.43-4.05,1.29C1.83,6.53,1.77,7.26,2.21,7.7l2.57,2.57h1.11v1.11 c0.86,0.86,1.98,1.31,3.11,1.36V15H7c-0.55,0-1,0.45-1,1v2c0,1.1,0.9,2,2,2h10c1.66,0,3-1.34,3-3V5c0-0.55-0.45-1-1-1H10 C9.45,4,9,4.45,9,5z M7.89,10.41V8.26H5.61L4.57,7.22C5.14,7,5.76,6.88,6.39,6.88c1.34,0,2.59,0.52,3.54,1.46l1.41,1.41l-0.2,0.2 c-0.51,0.51-1.19,0.8-1.92,0.8C8.75,10.75,8.29,10.63,7.89,10.41z M19,17c0,0.55-0.45,1-1,1s-1-0.45-1-1v-1c0-0.55-0.45-1-1-1h-5 v-2.59c0.57-0.23,1.1-0.57,1.56-1.03l0.2-0.2L15.59,14H17v-1.41l-6-5.97V6h8V17z",
                    }
                }
            }
        }
    }
}

pub fn hive_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M8.55,6.66h2.83h0l1.26-2.07c0.1-0.16,0.1-0.36,0-0.52l-1.11-1.84C11.44,2.09,11.28,2,11.1,2H8.83 C8.66,2,8.49,2.09,8.4,2.24L7.29,4.08c-0.1,0.16-0.1,0.36,0,0.52L8.55,6.66L8.55,6.66z",
                    }
                    path {
                        d: "M11.38,13.34H8.55h0l-1.26,2.07c-0.1,0.16-0.1,0.36,0,0.52l1.11,1.84C8.49,17.91,8.66,18,8.83,18h2.27 c0.17,0,0.34-0.09,0.43-0.24l1.11-1.84c0.1-0.16,0.1-0.36,0-0.52L11.38,13.34L11.38,13.34z",
                    }
                    path {
                        d: "M11.38,7.67H8.55h0L7.29,9.74c-0.1,0.16-0.1,0.36,0,0.52l1.26,2.07h0h2.83h0l1.26-2.07c0.1-0.16,0.1-0.36,0-0.52 L11.38,7.67L11.38,7.67z",
                    }
                    path {
                        d: "M13.95,9.5h2.27c0.17,0,0.34-0.09,0.43-0.24l1.11-1.83c0.1-0.16,0.1-0.36,0-0.52l-1.11-1.84 c-0.09-0.15-0.25-0.24-0.43-0.24h-2.27c-0.17,0-0.34,0.09-0.43,0.24l-1.11,1.84c-0.1,0.16-0.1,0.36,0,0.52l1.11,1.83 C13.61,9.41,13.77,9.5,13.95,9.5z",
                    }
                    path {
                        d: "M13.52,10.74l-1.11,1.84c-0.1,0.16-0.1,0.36,0,0.52l1.11,1.84c0.09,0.15,0.25,0.24,0.43,0.24h2.27 c0.17,0,0.34-0.09,0.43-0.24l1.11-1.84c0.1-0.16,0.1-0.36,0-0.52l-1.11-1.84c-0.09-0.15-0.25-0.24-0.43-0.24h-2.27 C13.77,10.5,13.61,10.59,13.52,10.74z",
                    }
                    path {
                        d: "M6.41,9.26l1.11-1.83c0.1-0.16,0.1-0.36,0-0.52L6.41,5.07C6.32,4.92,6.16,4.83,5.98,4.83H3.71 c-0.17,0-0.34,0.09-0.43,0.24L2.17,6.91c-0.1,0.16-0.1,0.36,0,0.52l1.11,1.83C3.38,9.41,3.54,9.5,3.71,9.5h2.27 C6.16,9.5,6.32,9.41,6.41,9.26z",
                    }
                    path {
                        d: "M5.98,10.5H3.71c-0.17,0-0.34,0.09-0.43,0.24l-1.11,1.84c-0.1,0.16-0.1,0.36,0,0.52l1.11,1.84 c0.09,0.15,0.25,0.24,0.43,0.24h2.27c0.17,0,0.34-0.09,0.43-0.24l1.11-1.84c0.1-0.16,0.1-0.36,0-0.52l-1.11-1.84 C6.32,10.59,6.16,10.5,5.98,10.5z",
                    }
                }
            }
        }
    }
}

pub fn hive_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M14.09,7.51l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C13.91,2.18,13.58,2,13.23,2h-2.45c-0.35,0-0.68,0.18-0.86,0.49 l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2C10.09,7.82,10.42,8,10.77,8h2.45C13.58,8,13.91,7.82,14.09,7.51z",
                    }
                    path {
                        d: "M9.91,9.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49h2.46c0.35,0,0.68-0.18,0.86-0.49 l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C13.91,9.18,13.58,9,13.23,9h-2.46C10.42,9,10.09,9.18,9.91,9.49z",
                    }
                    path {
                        d: "M17.01,11.51h2.45c0.35,0,0.68-0.18,0.86-0.49l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2c-0.18-0.3-0.51-0.49-0.86-0.49 h-2.45c-0.35,0-0.68,0.18-0.86,0.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2C16.34,11.33,16.66,11.51,17.01,11.51z",
                    }
                    path {
                        d: "M19.47,12.51h-2.46c-0.35,0-0.68,0.18-0.86,0.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49 h2.46c0.35,0,0.68-0.18,0.86-0.49l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C20.15,12.7,19.82,12.51,19.47,12.51z",
                    }
                    path {
                        d: "M7.84,11.03l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C7.66,5.7,7.34,5.51,6.99,5.51H4.53C4.18,5.51,3.85,5.7,3.67,6 l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49h2.45C7.34,11.51,7.66,11.33,7.84,11.03z",
                    }
                    path {
                        d: "M6.99,12.51H4.53c-0.35,0-0.68,0.18-0.86,0.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49 h2.46c0.35,0,0.68-0.18,0.86-0.49l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2C7.66,12.7,7.34,12.51,6.99,12.51z",
                    }
                    path {
                        d: "M9.91,16.49l-1.2,2c-0.19,0.32-0.19,0.71,0,1.03l1.2,2c0.18,0.3,0.51,0.49,0.86,0.49h2.46c0.35,0,0.68-0.18,0.86-0.49 l1.2-2c0.19-0.32,0.19-0.71,0-1.03l-1.2-2c-0.18-0.3-0.51-0.49-0.86-0.49h-2.46C10.42,16,10.09,16.18,9.91,16.49z",
                    }
                }
            }
        }
    }
}

pub fn ice_skating_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M18.16,14c-0.35,0-0.64,0.24-0.72,0.58c-0.26,1.1-1.25,1.92-2.43,1.92h-1.5V15h1c0.83,0,1.5-0.67,1.5-1.5V12 c0-1.1-0.72-2.08-1.78-2.4l-3.4-1.03c-0.33-0.1-0.6-0.3-0.79-0.57H7.5C7.22,8,7,7.77,7,7.5S7.22,7,7.5,7h2.25V6H7.5 C7.22,6,7,5.77,7,5.5S7.22,5,7.5,5h2.25V4c0-0.83-0.67-1.5-1.5-1.5H4.5C3.67,2.5,3,3.17,3,4v9.5C3,14.33,3.67,15,4.5,15h1v1.5H2.75 C2.34,16.5,2,16.84,2,17.25S2.34,18,2.75,18H15c1.9,0,3.5-1.33,3.9-3.12C19,14.43,18.63,14,18.16,14z M12,16.5H7V15h5V16.5z",
            }
        }
    }
}

pub fn ice_skating_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M21.87,17c-0.47,0-0.85,0.34-0.98,0.8C20.54,19.07,19.38,20,18,20h-2v-2h1c1.1,0,2-0.9,2-2l0-0.88 c0-2.1-1.55-3.53-3.03-3.88l-2.7-0.67C12.4,10.35,11.7,9.76,11.32,9H8.5C8.22,9,8,8.78,8,8.5S8.22,8,8.5,8h2.52L11,7H8.5 C8.22,7,8,6.78,8,6.5C8,6.22,8.22,6,8.5,6H11V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5v11c0,1.1,0.9,2,2,2h1v2H3c-0.55,0-1,0.45-1,1 s0.45,1,1,1h15c2.33,0,4.29-1.6,4.84-3.75C23.01,17.62,22.52,17,21.87,17z M14,20H8v-2h6V20z",
                }
            }
        }
    }
}

pub fn interests_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M6,11c-1.65,0-3,1.35-3,3s1.35,3,3,3s3-1.35,3-3S7.65,11,6,11z M11.5,17h5c0.28,0,0.5-0.22,0.5-0.5v-5 c0-0.28-0.22-0.5-0.5-0.5h-5c-0.28,0-0.5,0.22-0.5,0.5v5C11,16.78,11.22,17,11.5,17z M2.43,8.25C2.24,8.59,2.49,9,2.87,9h6.27 c0.38,0,0.62-0.41,0.43-0.75L6.43,2.76c-0.19-0.34-0.68-0.34-0.87,0L2.43,8.25z M15.72,2.5c-0.81,0-1.39,0.43-1.72,0.89 c-0.33-0.47-0.91-0.89-1.72-0.89c-1.19,0-2.1,0.98-2.1,2.1c0,1.44,1.65,2.49,3.49,4.1c0.19,0.17,0.47,0.17,0.66,0 c1.85-1.61,3.49-2.66,3.49-4.1C17.82,3.48,16.92,2.5,15.72,2.5z",
            }
        }
    }
}

pub fn interests_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7.02,13c-2.21,0-4,1.79-4,4s1.79,4,4,4s4-1.79,4-4S9.23,13,7.02,13z M13,14v6c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-6 c0-0.55-0.45-1-1-1h-6C13.45,13,13,13.45,13,14z M6.13,3.57l-3.3,5.94C2.46,10.18,2.94,11,3.7,11h6.6c0.76,0,1.24-0.82,0.87-1.49 l-3.3-5.94C7.49,2.89,6.51,2.89,6.13,3.57z M19.25,2.5c-1.06,0-1.81,0.56-2.25,1.17c-0.44-0.61-1.19-1.17-2.25-1.17 C13.19,2.5,12,3.78,12,5.25c0,1.83,2.03,3.17,4.35,5.18c0.37,0.32,0.92,0.32,1.3,0C19.97,8.42,22,7.08,22,5.25 C22,3.78,20.81,2.5,19.25,2.5z",
            }
        }
    }
}

pub fn ios_share_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18,8h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v11H6V10h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6c-1.1,0-2,0.9-2,2 v11c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V10C20,8.9,19.1,8,18,8z",
                    }
                    path {
                        d: "M12,16L12,16c0.55,0,1-0.45,1-1V5h1.79c0.45,0,0.67-0.54,0.35-0.85l-2.79-2.79c-0.2-0.2-0.51-0.2-0.71,0L8.85,4.15 C8.54,4.46,8.76,5,9.21,5H11v10C11,15.55,11.45,16,12,16z",
                    }
                }
            }
        }
    }
}

pub fn kayaking_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M11.5,6.25C11.5,7.22,10.72,8,9.75,8S8,7.22,8,6.25S8.78,4.5,9.75,4.5S11.5,5.28,11.5,6.25z M12.88,12.67l3.01-6.77 l0.79-0.3c0.13-0.05,0.23-0.14,0.28-0.26l0.83-1.86c0.11-0.25,0-0.55-0.25-0.66L16.18,2.2c-0.25-0.11-0.55,0-0.66,0.25l-0.83,1.87 c-0.05,0.12-0.06,0.26-0.01,0.38l0.3,0.8l-1.33,3L11.2,9.8l-2-1.12C9.07,8.6,8.92,8.55,8.76,8.52C8.02,8.39,7.3,8.85,7.06,9.57 L6,12.83c-2.04,0.33-3.72,0.83-4.78,1.2c-0.44,0.15-0.44,0.78,0,0.94c0.62,0.22,1.45,0.48,2.45,0.73c0.36-0.19,0.69-0.46,0.98-0.77 c0.19-0.21,0.52-0.21,0.71,0C5.93,15.55,6.68,16,7.5,16c0.82,0,1.56-0.44,2.14-1.06c0.2-0.21,0.53-0.21,0.72,0 C10.94,15.56,11.68,16,12.5,16c0.82,0,1.57-0.45,2.15-1.07c0.19-0.21,0.52-0.21,0.71,0c0.29,0.31,0.62,0.58,0.98,0.77 c1-0.25,1.83-0.51,2.45-0.73c0.44-0.15,0.44-0.78,0-0.94C17.53,13.6,15.42,12.97,12.88,12.67z M11.83,12.57 c-0.59-0.05-1.2-0.07-1.83-0.07c-0.25,0-0.5,0.01-0.74,0.02l0.57-1.77l0.87,0.49c0.3,0.17,0.67,0.17,0.97,0.01l0.98-0.53 L11.83,12.57z M5,18.4c-0.7,0.36-1.46,0.55-2.22,0.59C2.39,19.01,2,18.7,2,18.24c0-0.39,0.3-0.72,0.71-0.75 c0.68-0.04,1.36-0.29,1.98-0.75c0.18-0.14,0.43-0.14,0.61,0c1.37,1.01,3.02,1.01,4.39,0c0.19-0.14,0.43-0.14,0.61,0 c1.37,1.01,3.02,1,4.39,0c0.18-0.14,0.43-0.14,0.61,0c0.62,0.46,1.3,0.7,1.98,0.75C17.7,17.52,18,17.85,18,18.24 c0,0.46-0.39,0.77-0.78,0.75c-0.76-0.04-1.52-0.24-2.22-0.59c-1.57,0.8-3.43,0.8-5,0C8.43,19.2,6.57,19.2,5,18.4z",
            }
        }
    }
}

pub fn kayaking_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M2,22c0-0.55,0.45-1,1-1h0c0.87,0,1.73-0.24,2.53-0.7c0.29-0.16,0.65-0.17,0.94,0c1.59,0.9,3.48,0.9,5.06,0 c0.29-0.16,0.65-0.16,0.94,0c1.59,0.9,3.48,0.9,5.06,0c0.29-0.16,0.65-0.16,0.94,0C19.27,20.76,20.13,21,21,21h0c0.55,0,1,0.45,1,1 s-0.45,1-1,1h0c-1.03,0-2.06-0.25-3-0.75h0c-1.92,1.02-4.18,1-6.09-0.05c-1.79,0.87-3.92,0.98-5.58-0.14C5.3,22.69,4.15,23,3,23h0 C2.45,23,2,22.55,2,22z M12,5.5c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S13.1,5.5,12,5.5z M21.47,18.45c-0.42,0.14-0.9,0.28-1.41,0.42 c-0.53-0.15-1.03-0.43-1.45-0.77c-0.35-0.29-0.87-0.29-1.23,0C16.72,18.63,15.9,19,15,19s-1.72-0.37-2.39-0.91 c-0.35-0.28-0.87-0.28-1.22,0C10.72,18.63,9.9,19,9,19s-1.72-0.37-2.39-0.91c-0.35-0.29-0.87-0.28-1.23,0 c-0.43,0.35-0.92,0.62-1.45,0.77c-0.51-0.14-0.98-0.28-1.4-0.42c-0.92-0.3-0.92-1.6,0-1.9c1.21-0.39,2.79-0.82,4.6-1.13l1.35-4.17 c0.31-0.95,1.32-1.47,2.27-1.16c0.09,0.03,0.19,0.07,0.27,0.11l0,0l2.47,1.3l2.84-1.5l1.65-3.71l-0.36-0.93 c-0.1-0.25-0.09-0.52,0.02-0.76l0.74-1.68c0.22-0.51,0.82-0.73,1.32-0.51l1.37,0.61c0.5,0.23,0.73,0.82,0.5,1.32l-0.75,1.68 c-0.11,0.24-0.31,0.43-0.56,0.53L19.36,6.9l-3.72,8.34c2.33,0.3,4.35,0.84,5.82,1.31C22.39,16.85,22.4,18.15,21.47,18.45z M15.02,12.96l-0.59,0.31c-0.58,0.31-1.28,0.31-1.86,0l-0.81-0.43l-0.71,2.17C11.36,15.01,11.68,15,12,15c0.71,0,1.4,0.03,2.07,0.08 L15.02,12.96z",
            }
        }
    }
}

pub fn king_bed_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,8V6c0-0.55-0.45-1-1-1H5C4.45,5,4,5.45,4,6v2C3.45,8,3,8.45,3,9v4h1l0.75,2h0.5L6,13h8l0.75,2h0.5L16,13h1V9 C17,8.45,16.55,8,16,8z M9.5,8H5V6.5C5,6.22,5.22,6,5.5,6h4V8z M15,8h-4.5V6h4C14.78,6,15,6.22,15,6.5V8z",
                }
            }
        }
    }
}

pub fn king_bed_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    rect {
                        height: "3",
                        x: "6",
                        y: "7",
                        fill: "none",
                        width: "5",
                    }
                    rect {
                        x: "13",
                        y: "7",
                        height: "3",
                        width: "5",
                        fill: "none",
                    }
                    path {
                        d: "M20,10V7c0-1.1-0.9-2-2-2H6C4.9,5,4,5.9,4,7v3c-1.1,0-2,0.9-2,2v5h1.33l0.51,1.53C3.94,18.81,4.2,19,4.5,19h0 c0.3,0,0.56-0.19,0.66-0.47L5.67,17h12.67l0.51,1.53C18.94,18.81,19.2,19,19.5,19l0,0c0.3,0,0.56-0.19,0.66-0.47L20.67,17H22v-5 C22,10.9,21.1,10,20,10z M11,10H6V8c0-0.55,0.45-1,1-1h4V10z M18,10h-5V7h4c0.55,0,1,0.45,1,1V10z",
                    }
                }
            }
        }
    }
}

pub fn kitesurfing_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M5,18.4c-0.7,0.36-1.46,0.55-2.22,0.59C2.39,19.01,2,18.7,2,18.24c0-0.39,0.3-0.72,0.71-0.75c0.68-0.04,1.36-0.29,1.98-0.75 c0.18-0.14,0.43-0.14,0.61,0c1.37,1.01,3.02,1.01,4.39,0c0.19-0.14,0.43-0.14,0.61,0c1.37,1.01,3.02,1,4.39,0 c0.18-0.14,0.43-0.14,0.61,0c0.62,0.46,1.3,0.7,1.98,0.75C17.7,17.52,18,17.85,18,18.24c0,0.46-0.39,0.77-0.78,0.75 c-0.76-0.04-1.52-0.24-2.22-0.59c-1.57,0.8-3.43,0.8-5,0C8.43,19.2,6.57,19.2,5,18.4z M8.26,3c0,0.97-0.78,1.75-1.75,1.75 S4.76,3.97,4.76,3s0.78-1.75,1.75-1.75S8.26,2.03,8.26,3z M12.76,3.87c0.2,0.2,0.51,0.2,0.71,0L16.33,1h-1.41l-2.16,2.16 C12.57,3.35,12.57,3.67,12.76,3.87z M15.52,10c-1.1,0-2.7,0.49-4.39,1.25l-0.83-0.92C10.11,10.12,9.84,10,9.56,10H8V7h1 c1.21,0,2.33-0.43,3.19-1.15c0.34-0.28,0.35-0.8,0.04-1.11c-0.28-0.28-0.71-0.29-1.01-0.04C10.62,5.2,9.84,5.5,9,5.5H6.5 C5.67,5.5,5,6.17,5,7v1.46v2.06c0,0.32,0.07,0.63,0.22,0.91l1.28,2.52c-0.42,0.31-0.83,0.63-1.19,0.94c0,0,0.03,0.02,0.06,0.05 C5.94,15.56,6.68,16,7.5,16c0.82,0,1.56-0.44,2.14-1.06c0.2-0.21,0.53-0.21,0.72,0c0.51,0.55,1.15,0.96,1.86,1.04 C14.85,14.3,17,12.3,17,11.02C17,10.2,16.19,10,15.52,10z M7.88,13L7.3,11.5h2.03l0.39,0.44C9.24,12.19,8.36,12.69,7.88,13z",
            }
        }
    }
}

pub fn kitesurfing_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M6,3c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S6,4.1,6,3z M16.03,3.97c0.29,0.29,0.77,0.29,1.06,0L20.06,1h-2.12 l-1.91,1.91C15.74,3.2,15.74,3.68,16.03,3.97z M19.15,12c-1.29,0-3.11,0.53-5.06,1.38L13,12.16c-0.38-0.42-0.92-0.66-1.49-0.66H9.6 l0-3.5H11c1.52,0,2.94-0.49,4.09-1.32c0.49-0.35,0.52-1.07,0.09-1.5c-0.35-0.35-0.9-0.38-1.3-0.09C13.06,5.66,12.07,6,11,6H8 C6.9,6,6,6.9,6,8v4.04c0,0.64,0.15,1.27,0.45,1.83L8,16.84c-0.53,0.38-1.03,0.78-1.49,1.17C7.19,18.59,8.06,19,9,19 c1.2,0,2.27-0.66,3-1.5c0.73,0.84,1.8,1.5,3,1.5c0.33,0,0.65-0.05,0.96-0.14C18.81,16.9,21,14.72,21,13.28 C21,12.25,19.99,12,19.15,12z M9.83,15.61L9,13.6l2.5-0.1l0.7,0.77C11.64,14.55,10.42,15.23,9.83,15.61z M22,22c0-0.55-0.45-1-1-1 h0c-0.87,0-1.73-0.24-2.53-0.7c-0.29-0.16-0.65-0.17-0.94,0c-1.59,0.9-3.47,0.9-5.06,0c-0.29-0.16-0.65-0.16-0.94,0 c-1.59,0.9-3.47,0.9-5.06,0c-0.29-0.16-0.65-0.16-0.94,0C4.73,20.76,3.87,21,3,21h0c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h0 c1.15,0,2.3-0.31,3.33-0.94c1.66,1.11,3.78,1.01,5.58,0.14c1.91,1.05,4.17,1.07,6.09,0.05h0c0.95,0.5,1.97,0.75,3,0.75h0 C21.55,23,22,22.55,22,22z",
                }
            }
        }
    }
}

pub fn landslide_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M14,4.07v-1.9c0-0.69-0.47-1.29-1.14-1.46l-2.22-0.56c-0.41-0.1-0.84-0.03-1.2,0.21L7.67,1.55C7.25,1.83,7,2.3,7,2.8V4.2 c0,0.5,0.25,0.97,0.67,1.25l1.61,1.07c0.45,0.3,1.02,0.33,1.5,0.09l2.39-1.2C13.68,5.16,14,4.64,14,4.07z",
                        }
                    }
                    g {
                        path {
                            d: "M14.43,6.43L13.6,7.05C13.22,7.33,13,7.78,13,8.25v1.13c0,0.4,0.16,0.78,0.44,1.06l0.8,0.8 c0.46,0.46,1.15,0.57,1.73,0.28l2.2-1.1C18.68,10.16,19,9.64,19,9.07v-0.9c0-0.69-0.47-1.29-1.14-1.46l-2.17-0.54 C15.25,6.06,14.79,6.16,14.43,6.43z",
                        }
                    }
                    g {
                        polygon {
                            points: "12.45,11.73 10.27,10.64 4.94,12.31 2,10.84 2,12.66 5.06,14.19",
                        }
                    }
                    g {
                        path {
                            d: "M4.94,15.81L2,14.34v2.16C2,17.33,2.67,18,3.5,18h11.3c1.27,0,1.97-1.48,1.15-2.46l-2.22-2.66L4.94,15.81z",
                        }
                    }
                    g {
                        path {
                            d: "M8.7,9.56L7.45,7.67C7.17,7.25,6.7,7,6.2,7H3.5C2.67,7,2,7.67,2,8.5v0.66l3.06,1.53L8.7,9.56z",
                        }
                    }
                }
            }
        }
    }
}

pub fn landslide_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    polygon {
                        points: "15.47,13.79 12.89,12.76 6,15.05 2,13.51 2,15.61 6,16.95",
                    }
                    path {
                        d: "M10.57,11.42L8.6,8.8C8.22,8.3,7.63,8,7,8H4c-1.1,0-2,0.9-2,2v1.61l4,1.33L10.57,11.42z",
                    }
                    path {
                        d: "M6,19.05l-4-1.33V20c0,1.1,0.9,2,2,2h14c1.65,0,2.59-1.88,1.6-3.2l-2.57-3.42L6,19.05z",
                    }
                    path {
                        d: "M17,4.65V2.64c0-0.95-0.67-1.77-1.61-1.96l-2.58-0.52c-0.52-0.1-1.06,0-1.5,0.3L9.89,1.41C9.33,1.78,9,2.4,9,3.07v1.86 C9,5.6,9.33,6.22,9.89,6.59l1.23,0.82c0.55,0.37,1.24,0.44,1.85,0.19l2.77-1.11C16.5,6.2,17,5.46,17,4.65z",
                    }
                    path {
                        d: "M17.75,7.6l-1,0.8C16.28,8.78,16,9.35,16,9.96v1.08c0,0.61,0.28,1.18,0.75,1.56l0.8,0.64c0.58,0.47,1.38,0.57,2.06,0.27 l2.2-0.98C22.53,12.21,23,11.49,23,10.7V9.6c0-0.94-0.65-1.75-1.57-1.95l-2-0.44C18.84,7.08,18.22,7.22,17.75,7.6z",
                    }
                }
            }
        }
    }
}

pub fn location_city_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15 11V5.83c0-.53-.21-1.04-.59-1.41L12.7 2.71c-.39-.39-1.02-.39-1.41 0l-1.7 1.7C9.21 4.79 9 5.3 9 5.83V7H5c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-6c0-1.1-.9-2-2-2h-4zm-8 8H5v-2h2v2zm0-4H5v-2h2v2zm0-4H5V9h2v2zm6 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V9h2v2zm0-4h-2V5h2v2zm6 12h-2v-2h2v2zm0-4h-2v-2h2v2z",
            }
        }
    }
}

pub fn luggage_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M17,6h-2V3c0-0.55-0.45-1-1-1h-4C9.45,2,9,2.45,9,3v3H7C5.9,6,5,6.9,5,8v11c0,1.1,0.9,2,2,2c0,0.55,0.45,1,1,1 c0.55,0,1-0.45,1-1h6c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1c1.1,0,2-0.9,2-2V8C19,6.9,18.1,6,17,6z M8.75,18L8.75,18 C8.34,18,8,17.66,8,17.25v-7.5C8,9.34,8.34,9,8.75,9h0C9.16,9,9.5,9.34,9.5,9.75v7.5C9.5,17.66,9.16,18,8.75,18z M12,18L12,18 c-0.41,0-0.75-0.34-0.75-0.75v-7.5C11.25,9.34,11.59,9,12,9h0c0.41,0,0.75,0.34,0.75,0.75v7.5C12.75,17.66,12.41,18,12,18z M13.5,6 h-3V3.5h3V6z M15.25,18L15.25,18c-0.41,0-0.75-0.34-0.75-0.75v-7.5C14.5,9.34,14.84,9,15.25,9h0C15.66,9,16,9.34,16,9.75v7.5 C16,17.66,15.66,18,15.25,18z",
                }
            }
        }
    }
}

pub fn male_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M15,4h-2.75c-0.41,0-0.75,0.34-0.75,0.75c0,0.41,0.34,0.75,0.75,0.75h1.19l-2.76,2.76C9.99,7.78,9.15,7.5,8.25,7.5 C5.9,7.5,4,9.4,4,11.75C4,14.1,5.9,16,8.25,16s4.25-1.9,4.25-4.25c0-0.9-0.28-1.74-0.76-2.43l2.76-2.76v1.19 c0,0.41,0.34,0.75,0.75,0.75S16,8.16,16,7.75V5C16,4.45,15.55,4,15,4z M8.25,14.5c-1.52,0-2.75-1.23-2.75-2.75S6.73,9,8.25,9 S11,10.23,11,11.75S9.77,14.5,8.25,14.5z",
            }
        }
    }
}

pub fn male_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19,4h-4c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h1.58l-3.97,3.97C11.73,9.36,10.66,9,9.5,9C6.46,9,4,11.46,4,14.5 C4,17.54,6.46,20,9.5,20s5.5-2.46,5.5-5.5c0-1.16-0.36-2.23-0.97-3.12L18,7.42V9c0,0.55,0.45,1,1,1s1-0.45,1-1V5 C20,4.45,19.55,4,19,4z M9.5,18C7.57,18,6,16.43,6,14.5C6,12.57,7.57,11,9.5,11s3.5,1.57,3.5,3.5C13,16.43,11.43,18,9.5,18z",
            }
        }
    }
}

pub fn man_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cx: "10",
                        cy: "3.75",
                        r: "1.75",
                    }
                    path {
                        d: "M11.5,7h-3C7.67,7,7,7.67,7,8.5V12c0,0.55,0.45,1,1,1h0.5v4c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-4H12 c0.55,0,1-0.45,1-1V8.5C13,7.67,12.33,7,11.5,7z",
                    }
                }
            }
        }
    }
}

pub fn man_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M14,7h-4C8.9,7,8,7.9,8,9v5c0,0.55,0.45,1,1,1h1v6c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-6h1c0.55,0,1-0.45,1-1V9 C16,7.9,15.1,7,14,7z",
                    }
                    circle {
                        cx: "12",
                        cy: "4",
                        r: "2",
                    }
                }
            }
        }
    }
}

pub fn man_2_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    g {
                        circle {
                            cx: "10",
                            cy: "3.75",
                            r: "1.75",
                        }
                        path {
                            d: "M11.5,7h-3C7.67,7,7,7.67,7,8.5v4C7,12.78,7.22,13,7.5,13H9v4c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-4h1.5 c0.28,0,0.5-0.22,0.5-0.5v-4C13,7.67,12.33,7,11.5,7z",
                        }
                    }
                }
            }
        }
    }
}

pub fn man_2_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M14,7h-4C8.9,7,8,7.9,8,9v5c0,0.55,0.45,1,1,1h1.5v5.5c0,0.83,0.67,1.5,1.5,1.5h0c0.83,0,1.5-0.67,1.5-1.5V15H15 c0.55,0,1-0.45,1-1V9C16,7.9,15.1,7,14,7z",
                    }
                    circle {
                        cx: "12",
                        cy: "4",
                        r: "2",
                    }
                }
            }
        }
    }
}

pub fn man_3_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M11.5,7h-3C7.67,7,7,7.67,7,8.5v4C7,12.78,7.22,13,7.5,13h1v4.5C8.5,17.78,8.72,18,9,18h2c0.28,0,0.5-0.22,0.5-0.5V13h1 c0.28,0,0.5-0.22,0.5-0.5v-4C13,7.67,12.33,7,11.5,7z",
                    }
                    path {
                        d: "M11.65,3.65l-1.29-1.29c-0.2-0.2-0.51-0.2-0.71,0L8.35,3.65c-0.2,0.2-0.2,0.51,0,0.71l1.29,1.29c0.2,0.2,0.51,0.2,0.71,0 l1.29-1.29C11.84,4.16,11.84,3.84,11.65,3.65z",
                    }
                }
            }
        }
    }
}

pub fn man_3_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M14,7h-4C8.9,7,8,7.9,8,9v5c0,0.55,0.45,1,1,1h1v6c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-6h1c0.55,0,1-0.45,1-1V9 C16,7.9,15.1,7,14,7z",
                    }
                    path {
                        d: "M11.65,5.9L10.1,4.35c-0.2-0.2-0.2-0.51,0-0.71l1.54-1.54c0.2-0.2,0.51-0.2,0.71,0l1.54,1.54c0.2,0.2,0.2,0.51,0,0.71 L12.35,5.9C12.16,6.09,11.84,6.09,11.65,5.9z",
                    }
                }
            }
        }
    }
}

pub fn man_4_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M11.75,7h-3.5C7.31,7,6.61,7.85,6.77,8.77l1.57,8.42C8.44,17.66,8.85,18,9.33,18h1.34c0.48,0,0.89-0.34,0.98-0.82 l1.57-8.42C13.39,7.85,12.69,7,11.75,7z",
                    }
                    circle {
                        cy: "3.75",
                        cx: "10",
                        r: "1.75",
                    }
                }
            }
        }
    }
}

pub fn man_4_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M13.75,7h-3.5C9.04,7,8.11,8.07,8.27,9.26L9.82,20.7c0.1,0.74,0.74,1.3,1.49,1.3h1.38c0.75,0,1.39-0.55,1.49-1.3 l1.56-11.44C15.89,8.07,14.96,7,13.75,7z",
                    }
                    circle {
                        cx: "12",
                        cy: "4",
                        r: "2",
                    }
                }
            }
        }
    }
}

pub fn masks_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19.5,6c-1.31,0-2.37,1.01-2.48,2.3C15.14,7.8,14.18,6.5,12,6.5c-2.19,0-3.14,1.3-5.02,1.8C6.87,7.02,5.81,6,4.5,6 C3.12,6,2,7.12,2,8.5V9c0,6,3.6,7.81,6.52,7.98C9.53,17.62,10.72,18,12,18s2.47-0.38,3.48-1.02C18.4,16.81,22,15,22,9V8.5 C22,7.12,20.88,6,19.5,6z M3.5,9V8.5c0-0.55,0.45-1,1-1s1,0.45,1,1v3c0,1.28,0.38,2.47,1.01,3.48C4.99,14.27,3.5,12.65,3.5,9z M14.3,11.01c-0.4-0.17-0.72-0.36-1.01-0.53C12.83,10.2,12.49,10,12,10c-0.49,0-0.84,0.2-1.31,0.48c-0.28,0.17-0.6,0.35-0.98,0.51 C9.37,11.14,9,10.91,9,10.54v0c0-0.2,0.11-0.38,0.29-0.45c0.34-0.14,0.62-0.31,0.88-0.46C10.72,9.3,11.23,9,12,9s1.27,0.3,1.8,0.62 c0.27,0.16,0.55,0.33,0.9,0.48c0.18,0.08,0.29,0.26,0.29,0.45C15,10.91,14.63,11.15,14.3,11.01z M20.5,9c0,3.65-1.49,5.27-3.01,5.98 c0.64-1.01,1.01-2.2,1.01-3.48v-3c0-0.55,0.45-1,1-1s1,0.45,1,1V9z",
            }
        }
    }
}

pub fn military_tech_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13.16,13.29l-2.27-0.19l-0.56-1.31l2.45-1.63C12.92,10.06,13,9.9,13,9.73V3.5C13,3.22,12.78,3,12.5,3h-5 C7.22,3,7,3.22,7,3.5v6.23c0,0.17,0.08,0.32,0.22,0.42l2.45,1.63l-0.56,1.31l-2.27,0.19l1.72,1.49L8.05,17L10,15.82L11.95,17 l-0.52-2.22L13.16,13.29z M9.5,4h1v6.46L10,10.8l-0.5-0.33V4z",
                }
            }
        }
    }
}

pub fn military_tech_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M17,10.43V3c0-0.55-0.45-1-1-1H8C7.45,2,7,2.45,7,3v7.43c0,0.35,0.18,0.68,0.49,0.86l4.18,2.51l-0.99,2.34l-2.22,0.19 C8,16.37,7.82,16.92,8.16,17.21l1.69,1.46l-0.51,2.18c-0.1,0.43,0.37,0.77,0.75,0.54L12,20.23l1.91,1.15 c0.38,0.23,0.85-0.11,0.75-0.54l-0.51-2.18l1.69-1.46c0.33-0.29,0.16-0.84-0.29-0.88l-2.22-0.19l-0.99-2.34l4.18-2.51 C16.82,11.11,17,10.79,17,10.43z M13,12.23l-1,0.6l-1-0.6V3h2V12.23z",
                }
            }
        }
    }
}

pub fn mood_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 6.5c2.03 0 3.8-1.11 4.75-2.75.19-.33-.05-.75-.44-.75H7.69c-.38 0-.63.42-.44.75.95 1.64 2.72 2.75 4.75 2.75z",
            }
        }
    }
}

pub fn mood_bad_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 2.5c-2.03 0-3.8 1.11-4.75 2.75-.19.33.06.75.44.75h8.62c.38 0 .63-.42.44-.75-.95-1.64-2.72-2.75-4.75-2.75z",
            }
        }
    }
}

pub fn nights_stay_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M9.38 10.05C7.75 6.89 9.03 4.11 9.76 3 6 3.13 3 6.21 3 9.99c0 .4.04.78.1 1.16.29-.09.59-.15.9-.15.86 0 1.68.38 2.24 1.01 1.27.12 2.26 1.19 2.26 2.49 0 .83-.41 1.56-1.04 2.02.79.3 1.65.48 2.55.48 2.45 0 4.6-1.26 5.86-3.17-1.66.16-4.89-.67-6.49-3.78z",
                        }
                    }
                    path {
                        d: "M6 13h-.27c-.35-.6-.99-1-1.73-1-1.1 0-2 .9-2 2s.9 2 2 2h2c.83 0 1.5-.67 1.5-1.5S6.83 13 6 13z",
                    }
                }
            }
        }
    }
}

pub fn nights_stay_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M11.1 12.08c-2-3.88-.92-7.36.07-9.27.19-.36-.12-.77-.53-.72-5.02.68-8.86 5.07-8.65 10.32.01 0 .01 0 .01.01.62-.27 1.29-.42 2-.42 1.66 0 3.18.83 4.1 2.15 1.67.48 2.9 2.02 2.9 3.85 0 1.52-.87 2.83-2.12 3.51.98.32 2.03.5 3.11.5 3.13 0 5.92-1.44 7.76-3.69.26-.32.04-.79-.37-.82-2.49-.13-6.28-1.53-8.28-5.42z",
                        }
                    }
                    path {
                        d: "M7 16h-.18C6.4 14.84 5.3 14 4 14c-1.66 0-3 1.34-3 3s1.34 3 3 3h3c1.1 0 2-.9 2-2s-.9-2-2-2z",
                    }
                }
            }
        }
    }
}

pub fn nordic_walking_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M9.25,3.75C9.25,2.78,10.03,2,11,2s1.75,0.78,1.75,1.75c0,0.97-0.79,1.75-1.75,1.75C10.04,5.5,9.25,4.72,9.25,3.75z M11.25,18c0.41,0,0.75-0.34,0.75-0.75v-3.89c0-0.27-0.11-0.52-0.29-0.71l-1.17-1.17l0.49-2.48c0.74,0.98,1.82,1.68,3.07,1.91 c0.42,0.08,0.83-0.2,0.89-0.62c0.06-0.42-0.24-0.79-0.64-0.84c-2.46-0.28-2.43-2.2-3.46-3.03c-0.57-0.46-1.35-0.57-2.03-0.28 L5.61,7.53C5.24,7.69,5,8.05,5,8.46v1.79c0,0.42,0.35,0.76,0.77,0.75c0.41-0.01,0.73-0.37,0.73-0.78l0-1.44l1.65-0.7L6.38,17.1 C6.29,17.57,6.64,18,7.11,18c0.37,0,0.68-0.27,0.75-0.63l1.06-5.39l1.58,1.59v3.68C10.5,17.66,10.84,18,11.25,18z M14.5,18 c0.28,0,0.5-0.22,0.5-0.5V12h-1v5.5C14,17.78,14.22,18,14.5,18z M4.23,18c0.23,0,0.44-0.16,0.49-0.39L6.03,12H5l-1.26,5.38 C3.67,17.7,3.91,18,4.23,18L4.23,18z",
            }
        }
    }
}

pub fn nordic_walking_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.25,23c-0.41,0-0.75-0.34-0.75-0.75V14H19v8.25C19,22.66,18.66,23,18.25,23z M4.93,23c0.35,0,0.66-0.24,0.73-0.59 L7.53,14H6l-1.8,8.09C4.1,22.56,4.45,23,4.93,23z M13.5,5.5c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S12.4,5.5,13.5,5.5z M14,23 c0.55,0,1-0.45,1-1v-5.64c0-0.55-0.22-1.07-0.62-1.45L12.9,13.5l0.6-3c1.07,1.24,2.62,2.13,4.36,2.41c0.6,0.1,1.14-0.38,1.14-0.99 c0-0.49-0.35-0.91-0.83-0.98c-1.53-0.24-2.79-1.14-3.47-2.33l-1-1.6c-0.56-0.89-1.68-1.25-2.66-0.84L7.22,7.78 C6.48,8.1,6,8.82,6,9.62V12c0,0.55,0.45,1,1,1s1-0.45,1-1V9.6l1.8-0.7L7.25,21.76C7.12,22.4,7.61,23,8.27,23 c0.49,0,0.91-0.34,1.02-0.81L10.9,15l2.1,2v5C13,22.55,13.45,23,14,23z",
            }
        }
    }
}

pub fn notifications_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 22c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2zm6-6v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-1.29 1.29c-.63.63-.19 1.71.7 1.71h13.17c.89 0 1.34-1.08.71-1.71L18 16z",
            }
        }
    }
}

pub fn notifications_active_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18 16v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.68-1.5-1.51-1.5S10.5 3.17 10.5 4v.68C7.63 5.36 6 7.92 6 11v5l-1.3 1.29c-.63.63-.19 1.71.7 1.71h13.17c.89 0 1.34-1.08.71-1.71L18 16zm-6.01 6c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2zM6.77 4.73c.42-.38.43-1.03.03-1.43-.38-.38-1-.39-1.39-.02C3.7 4.84 2.52 6.96 2.14 9.34c-.09.61.38 1.16 1 1.16.48 0 .9-.35.98-.83.3-1.94 1.26-3.67 2.65-4.94zM18.6 3.28c-.4-.37-1.02-.36-1.4.02-.4.4-.38 1.04.03 1.42 1.38 1.27 2.35 3 2.65 4.94.07.48.49.83.98.83.61 0 1.09-.55.99-1.16-.38-2.37-1.55-4.48-3.25-6.05z",
            }
        }
    }
}

pub fn notifications_none_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19.29 17.29L18 16v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-1.29 1.29c-.63.63-.19 1.71.7 1.71h13.17c.9 0 1.34-1.08.71-1.71zM16 17H8v-6c0-2.48 1.51-4.5 4-4.5s4 2.02 4 4.5v6zm-4 5c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2z",
            }
        }
    }
}

pub fn notifications_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 22c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2zm6-11c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68c-.24.06-.47.15-.69.23L18 13.1V11zM5.41 3.35L4 4.76l2.81 2.81C6.29 8.57 6 9.73 6 11v5l-1.29 1.29c-.63.63-.19 1.71.7 1.71h12.83l1.74 1.74 1.41-1.41L5.41 3.35z",
            }
        }
    }
}

pub fn notifications_paused_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 22c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2zm7.29-4.71L18 16v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-1.29 1.29c-.63.63-.19 1.71.7 1.71h13.17c.9 0 1.34-1.08.71-1.71zM14.5 9.33c0 .31-.11.6-.3.84l-2.5 3.03h1.9c.5 0 .9.4.9.9s-.4.9-.9.9h-2.78c-.73 0-1.32-.59-1.32-1.32v-.01c0-.31.11-.6.3-.84l2.5-3.03h-1.9c-.5 0-.9-.4-.9-.9s.4-.9.9-.9h2.78c.73 0 1.32.59 1.32 1.33z",
            }
        }
    }
}

pub fn notification_add_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M15,11.03C13.52,10.3,12.5,8.77,12.5,7c0-0.73,0.17-1.41,0.48-2.02C12.4,4.56,11.73,4.25,11,4.1V3.03 c0-0.53-0.4-1-0.93-1.03C9.48,1.96,9,2.43,9,3v1.1C6.72,4.56,5,6.58,5,9v5H4.75C4.34,14,4,14.34,4,14.75v0 c0,0.41,0.34,0.75,0.75,0.75h10.5c0.41,0,0.75-0.34,0.75-0.75v0c0-0.41-0.34-0.75-0.75-0.75H15L15,11.03z M10,18 c0.83,0,1.5-0.67,1.5-1.5h-3C8.5,17.33,9.17,18,10,18z M19.31,6.31h-1.56V4.75C17.75,4.34,17.41,4,17,4l0,0 c-0.41,0-0.75,0.34-0.75,0.75v1.56h-1.56C14.31,6.31,14,6.62,14,7v0c0,0.38,0.31,0.69,0.69,0.69h1.56v1.56 C16.25,9.66,16.59,10,17,10l0,0c0.41,0,0.75-0.34,0.75-0.75V7.69h1.56C19.69,7.69,20,7.38,20,7v0C20,6.62,19.69,6.31,19.31,6.31z",
                }
            }
        }
    }
}

pub fn notification_add_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                y: "0",
                x: "0",
                fill: "none",
                height: "24",
                width: "24",
            }
            g {
                path {
                    d: "M10,20h4c0,1.1-0.9,2-2,2S10,21.1,10,20z M14,9c0,2.61,1.67,4.83,4,5.66L18,17h1c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H5 c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h1v-7c0-2.79,1.91-5.14,4.5-5.8V3.5C10.5,2.67,11.17,2,12,2s1.5,0.67,1.5,1.5v0.7 c0.71,0.18,1.36,0.49,1.95,0.9C14.54,6.14,14,7.51,14,9z M23,8h-2V6c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2h-2 c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h2c0.55,0,1-0.45,1-1v0 C24,8.45,23.55,8,23,8z",
                }
            }
        }
    }
}

pub fn no_adult_content_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,2c-4.42,0-8,3.58-8,8s3.58,8,8,8s8-3.58,8-8S14.42,2,10,2z M3.5,10c0-1.52,0.53-2.92,1.41-4.03L6.44,7.5h2.12 L5.97,4.91C7.08,4.03,8.48,3.5,10,3.5c3.58,0,6.5,2.92,6.5,6.5c0,1.52-0.53,2.92-1.41,4.03l-1.53-1.53h-2.12l2.59,2.59 c-1.11,0.88-2.51,1.41-4.03,1.41C6.42,16.5,3.5,13.58,3.5,10z",
                    }
                    polygon {
                        points: "10.5,10 11.5,8.5 10.5,8.5 10,9.25 9.5,8.5 8.5,8.5 9.5,10 8.5,11.5 9.5,11.5 10,10.75 10.5,11.5 11.5,11.5",
                    }
                    polygon {
                        points: "7,8.5 6.5,9.25 6,8.5 5,8.5 6,10 5,11.5 6,11.5 6.5,10.75 7,11.5 8,11.5 7,10 8,8.5",
                    }
                    polygon {
                        points: "13.25,11.5 13.75,10.75 14.25,11.5 15.25,11.5 14.25,10 15.25,8.5 14.25,8.5 13.75,9.25 13.25,8.5 12.25,8.5 13.25,10 12.25,11.5",
                    }
                }
            }
        }
    }
}

pub fn no_adult_content_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M4,12c0-1.85,0.63-3.54,1.69-4.9L7.59,9h2.83 L7.1,5.69C8.46,4.63,10.15,4,12,4c4.41,0,8,3.59,8,8c0,1.85-0.63,3.54-1.69,4.9l-1.9-1.9h-2.83l3.31,3.31 C15.54,19.37,13.85,20,12,20C7.59,20,4,16.41,4,12z",
                    }
                    polygon {
                        points: "14.25,14 12.75,12 14.25,10 12.75,10 12,11 11.25,10 9.75,10 11.25,12 9.75,14 11.25,14 12,13 12.75,14",
                    }
                    polygon {
                        points: "8,10 7.25,11 6.5,10 5,10 6.5,12 5,14 6.5,14 7.25,13 8,14 9.5,14 8,12 9.5,10",
                    }
                    polygon {
                        points: "16,14 16.75,13 17.5,14 19,14 17.5,12 19,10 17.5,10 16.75,11 16,10 14.5,10 16,12 14.5,14",
                    }
                }
            }
        }
    }
}

pub fn no_luggage_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M20.49,20.49L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l2.92,2.92C5.02,7.9,5,7.95,5,8v11 c0,1.1,0.9,2,2,2c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1h6c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1c0.34,0,0.65-0.09,0.93-0.24 l1.14,1.14c0.39,0.39,1.02,0.39,1.41,0C20.88,21.51,20.88,20.88,20.49,20.49z M8.75,18C8.34,18,8,17.66,8,17.25v-6.42l1.5,1.5v4.92 C9.5,17.66,9.16,18,8.75,18z M12,18c-0.41,0-0.75-0.34-0.75-0.75v-3.17l1.5,1.5v1.67C12.75,17.66,12.41,18,12,18z M12,9 c0.41,0,0.75,0.34,0.75,0.75v0.17l1.75,1.75V9.75C14.5,9.34,14.84,9,15.25,9S16,9.34,16,9.75v3.42l3,3V8c0-1.1-0.9-2-2-2h-2V3 c0-0.55-0.45-1-1-1h-4C9.45,2,9,2.45,9,3v3H8.83l3.03,3.03C11.91,9.02,11.95,9,12,9z M10.5,3.5h3V6h-3V3.5z",
                }
            }
        }
    }
}

pub fn outdoor_grill_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.66,13.88c-0.14-0.21-0.33-0.4-0.54-0.54C14.8,13.13,14.41,13,14,13s-0.8,0.13-1.12,0.34 c-0.21,0.14-0.4,0.33-0.54,0.54c-0.03,0.04-0.04,0.08-0.06,0.12H7.72l1.36-2.09C9.38,11.97,9.69,12,10,12 c0.31,0,0.62-0.03,0.92-0.09l0.77,1.19c0.22-0.26,0.47-0.49,0.76-0.66l-0.54-0.83c1.64-0.68,2.84-2.2,3.05-4.03 C15,7.27,14.78,7,14.47,7H5.53C5.22,7,5,7.27,5.03,7.58c0.21,1.83,1.41,3.35,3.05,4.03l-3,4.62C4.93,16.46,5,16.77,5.23,16.92 c0.23,0.15,0.54,0.08,0.69-0.15L7.07,15H12c0,1.1,0.9,2,2,2s2-0.9,2-2C16,14.59,15.87,14.2,15.66,13.88z M14,16 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C15,15.55,14.55,16,14,16z",
                    }
                    path {
                        d: "M8.7,5.72C8.68,5.87,8.79,6,8.94,6c0.12,0,0.23-0.09,0.24-0.22c0.06-0.54,0.04-0.85-0.56-1.56C8.4,3.96,8.25,3.8,8.3,3.26 C8.32,3.12,8.2,3,8.06,3H8.05C7.93,3,7.82,3.1,7.81,3.22C7.75,3.8,7.92,4.29,8.37,4.78C8.49,4.9,8.77,5.12,8.7,5.72z",
                    }
                    path {
                        d: "M10.2,5.72C10.18,5.87,10.29,6,10.44,6c0.12,0,0.23-0.09,0.24-0.22c0.06-0.54,0.04-0.85-0.56-1.56 C9.9,3.96,9.75,3.8,9.8,3.26C9.82,3.12,9.7,3,9.56,3H9.55C9.43,3,9.32,3.1,9.31,3.22C9.25,3.8,9.42,4.29,9.87,4.78 C9.99,4.9,10.27,5.12,10.2,5.72z",
                    }
                    path {
                        d: "M11.7,5.72C11.68,5.87,11.79,6,11.94,6c0.12,0,0.23-0.09,0.24-0.22c0.06-0.54,0.04-0.85-0.56-1.56 C11.4,3.96,11.25,3.8,11.3,3.26C11.32,3.12,11.2,3,11.06,3h-0.01c-0.13,0-0.23,0.1-0.24,0.22c-0.06,0.57,0.11,1.07,0.56,1.56 C11.49,4.9,11.77,5.12,11.7,5.72z",
                    }
                }
            }
        }
    }
}

pub fn outdoor_grill_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M17,22c1.66,0,3-1.34,3-3s-1.34-3-3-3c-1.3,0-2.4,0.84-2.82,2H9.14l1.99-3.06C11.42,14.98,11.71,15,12,15 s0.58-0.02,0.87-0.06l1.02,1.57c0.42-0.53,0.96-0.95,1.6-1.21l-0.6-0.93c2.1-0.95,3.64-2.9,4.02-5.24C19.01,8.54,18.52,8,17.92,8 C15.31,8,8.69,8,6.08,8C5.48,8,4.99,8.54,5.09,9.14c0.38,2.34,1.93,4.29,4.02,5.24l-3.95,6.08c-0.3,0.46-0.17,1.08,0.29,1.38h0 c0.46,0.3,1.08,0.17,1.38-0.29l1-1.55h6.34C14.6,21.16,15.7,22,17,22z M17,18c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1s-1-0.45-1-1 C16,18.45,16.45,18,17,18z",
                    }
                    path {
                        d: "M9.5,6.47C9.48,6.75,9.68,7,9.96,7H10c0.24,0,0.44-0.18,0.46-0.42c0.1-0.87,0.04-1.39-0.94-2.54 C9.16,3.61,8.92,3.35,8.99,2.49C9.02,2.23,8.8,2,8.53,2H8.48C8.24,2,8.03,2.19,8.01,2.43c-0.08,0.93,0.2,1.74,0.95,2.53 C9.15,5.17,9.6,5.52,9.5,6.47z",
                    }
                    path {
                        d: "M11.99,6.47C11.96,6.75,12.17,7,12.45,7h0.03c0.24,0,0.44-0.18,0.46-0.42c0.1-0.87,0.04-1.39-0.94-2.54 c-0.36-0.43-0.61-0.69-0.53-1.55C11.5,2.23,11.28,2,11.01,2h-0.05c-0.24,0-0.45,0.19-0.47,0.43c-0.08,0.93,0.2,1.74,0.95,2.53 C11.63,5.17,12.08,5.52,11.99,6.47z",
                    }
                    path {
                        d: "M14.5,6.47C14.48,6.75,14.68,7,14.96,7H15c0.24,0,0.44-0.18,0.46-0.42c0.1-0.87,0.04-1.39-0.94-2.54 c-0.36-0.43-0.61-0.69-0.53-1.55C14.02,2.23,13.8,2,13.53,2h-0.05c-0.24,0-0.45,0.19-0.47,0.43c-0.08,0.93,0.2,1.74,0.95,2.53 C14.15,5.17,14.6,5.52,14.5,6.47z",
                    }
                }
            }
        }
    }
}

pub fn pages_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3 5v6h5l-.6-2.38c-.18-.74.48-1.4 1.22-1.22L11 8V3H5c-1.1 0-2 .9-2 2zm5 8H3v6c0 1.1.9 2 2 2h6v-5l-2.38.6c-.73.18-1.4-.48-1.21-1.21L8 13zm7.38 3.6L13 16v5h6c1.1 0 2-.9 2-2v-6h-5l.6 2.38c.18.74-.48 1.4-1.22 1.22zM19 3h-6v5l2.38-.6c.73-.18 1.4.48 1.21 1.21L16 11h5V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn paragliding_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                y: "0",
                height: "20",
                width: "20",
            }
            path {
                d: "M10,14.25c-0.97,0-1.75-0.78-1.75-1.75s0.78-1.75,1.75-1.75s1.75,0.78,1.75,1.75S10.97,14.25,10,14.25z M15.18,11.5 c-0.36,0-0.68,0.26-0.73,0.62c-0.3,1.91-1.96,3.38-3.95,3.38h-1c-1.99,0-3.65-1.47-3.95-3.38c-0.06-0.36-0.37-0.62-0.73-0.62 c-0.46,0-0.82,0.41-0.75,0.87c0.31,1.95,1.64,3.55,3.43,4.25V20h5v-3.38c1.79-0.7,3.12-2.3,3.43-4.25 C16,11.91,15.64,11.5,15.18,11.5z M19,3.48v3.17c0,0.61-0.68,0.95-1.19,0.61c-0.1-0.07-0.2-0.13-0.31-0.19L16,10.5h-1.5L13,5.7 c-0.94-0.13-1.95-0.2-3-0.2S7.94,5.57,7,5.7l-1.5,4.8H4L2.5,7.05C2.39,7.12,2.29,7.18,2.19,7.25C1.68,7.59,1,7.26,1,6.64V3.48 C1,1.56,5.03,0,10,0S19,1.56,19,3.48z M5.84,5.89C4.92,6.08,4.1,6.32,3.4,6.61l1.32,3.03L5.84,5.89z M16.6,6.61 c-0.7-0.29-1.52-0.54-2.44-0.72l1.13,3.75L16.6,6.61z",
            }
        }
    }
}

pub fn paragliding_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12,17c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2S13.1,17,12,17z M17.95,14c-0.52,0-0.94,0.4-0.99,0.92 c-0.2,2.03-1.05,2.68-1.48,3.02C14.68,18.54,14,19,12,19s-2.68-0.46-3.48-1.06c-0.43-0.34-1.28-0.99-1.48-3.02 C6.99,14.4,6.57,14,6.05,14c-0.59,0-1.06,0.51-1,1.09c0.22,2.08,1.07,3.47,2.24,4.41c0.5,0.4,1.1,0.7,1.7,0.9L9,24h6v-3.6 c0.6-0.2,1.2-0.5,1.7-0.9c1.17-0.94,2.03-2.32,2.24-4.41C19.01,14.51,18.53,14,17.95,14z M12,0C5.92,0,1,1.9,1,4.25v3.49 C1,8.55,1.88,9,2.56,8.57C2.7,8.48,2.84,8.39,3,8.31L5,13h2l1.5-6.28C9.6,6.58,10.78,6.5,12,6.5s2.4,0.08,3.5,0.22L17,13h2l2-4.69 c0.16,0.09,0.3,0.17,0.44,0.26C22.12,9,23,8.55,23,7.74V4.25C23,1.9,18.08,0,12,0z M5.88,11.24L4.37,7.69 c0.75-0.28,1.6-0.52,2.53-0.71L5.88,11.24z M18.12,11.24L17.1,6.98c0.93,0.19,1.78,0.43,2.53,0.71L18.12,11.24z",
            }
        }
    }
}

pub fn party_mode_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 4h-3.17l-1.24-1.35c-.37-.41-.91-.65-1.47-.65H9.88c-.56 0-1.1.24-1.48.65L7.17 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-8 3c1.63 0 3.06.79 3.98 2H12c-1.66 0-3 1.34-3 3 0 .35.07.69.18 1H7.1c-.06-.32-.1-.66-.1-1 0-2.76 2.24-5 5-5zm0 10c-1.63 0-3.06-.79-3.98-2H12c1.66 0 3-1.34 3-3 0-.35-.07-.69-.18-1h2.08c.07.32.1.66.1 1 0 2.76-2.24 5-5 5z",
            }
        }
    }
}

pub fn people_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5s-3 1.34-3 3 1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V18c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-1.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05.02.01.03.03.04.04 1.14.83 1.93 1.94 1.93 3.41V18c0 .35-.07.69-.18 1H22c.55 0 1-.45 1-1v-1.5c0-2.33-4.67-3.5-7-3.5z",
            }
        }
    }
}

pub fn people_alt_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    g {
                        circle {
                            r: "2.5",
                            cy: "7.5",
                            cx: "8.5",
                            fill_rule: "evenodd",
                        }
                    }
                    g {
                        path {
                            d: "M10.99,9.95C11.16,9.98,11.33,10,11.5,10c1.38,0,2.5-1.12,2.5-2.5 C14,6.12,12.88,5,11.5,5c-0.17,0-0.34,0.02-0.51,0.05C11.61,5.68,12,6.55,12,7.5S11.61,9.32,10.99,9.95z",
                            fill_rule: "evenodd",
                        }
                    }
                    g {
                        path {
                            d: "M8.5,11C6.66,11,3,11.66,3,13.23v1.27C3,14.78,3.22,15,3.5,15h10 c0.28,0,0.5-0.22,0.5-0.5v-1.27C14,11.66,10.34,11,8.5,11z",
                            fill_rule: "evenodd",
                        }
                    }
                    g {
                        path {
                            d: "M13.73,11.23c0.75,0.48,1.27,1.12,1.27,2V15h1.5c0.28,0,0.5-0.22,0.5-0.5v-1.27 C17,12.18,15.36,11.54,13.73,11.23z",
                            fill_rule: "evenodd",
                        }
                    }
                }
            }
        }
    }
}

pub fn people_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    g {
                        path {
                            fill_rule: "evenodd",
                            d: "M16.67,13.13C18.04,14.06,19,15.32,19,17v3h3c0.55,0,1-0.45,1-1v-2 C23,14.82,19.43,13.53,16.67,13.13z",
                        }
                    }
                    g {
                        circle {
                            cx: "9",
                            cy: "8",
                            fill_rule: "evenodd",
                            r: "4",
                        }
                    }
                    g {
                        path {
                            d: "M15,12c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4c-0.47,0-0.91,0.1-1.33,0.24 C14.5,5.27,15,6.58,15,8s-0.5,2.73-1.33,3.76C14.09,11.9,14.53,12,15,12z",
                            fill_rule: "evenodd",
                        }
                    }
                    g {
                        path {
                            d: "M9,13c-2.67,0-8,1.34-8,4v2c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1v-2 C17,14.34,11.67,13,9,13z",
                            fill_rule: "evenodd",
                        }
                    }
                }
            }
        }
    }
}

pub fn people_outline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9 12c1.93 0 3.5-1.57 3.5-3.5S10.93 5 9 5 5.5 6.57 5.5 8.5 7.07 12 9 12zm0-5c.83 0 1.5.67 1.5 1.5S9.83 10 9 10s-1.5-.67-1.5-1.5S8.17 7 9 7zm0 6.75c-2.34 0-7 1.17-7 3.5V18c0 .55.45 1 1 1h12c.55 0 1-.45 1-1v-.75c0-2.33-4.66-3.5-7-3.5zM4.34 17c.84-.58 2.87-1.25 4.66-1.25s3.82.67 4.66 1.25H4.34zm11.7-3.19c1.16.84 1.96 1.96 1.96 3.44V19h3c.55 0 1-.45 1-1v-.75c0-2.02-3.5-3.17-5.96-3.44zM15 12c1.93 0 3.5-1.57 3.5-3.5S16.93 5 15 5c-.54 0-1.04.13-1.5.35.63.89 1 1.98 1 3.15s-.37 2.26-1 3.15c.46.22.96.35 1.5.35z",
            }
        }
    }
}

pub fn person_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v1c0 .55.45 1 1 1h14c.55 0 1-.45 1-1v-1c0-2.66-5.33-4-8-4z",
            }
        }
    }
}

pub fn personal_injury_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M7,5c0-1.66,1.34-3,3-3s3,1.34,3,3s-1.34,3-3,3S7,6.66,7,5z M14.5,18c0.83,0,1.5-0.67,1.5-1.5v-4.41 c0-0.7-0.36-1.36-0.97-1.72C14.7,10.17,14.36,10,14,9.84V18H14.5z M10.34,14l2.16-4.68C11.7,9.11,10.86,9,10,9 c-1.84,0-3.56,0.5-5.03,1.37C4.36,10.73,4,11.39,4,12.09V18h1.8c-0.19-0.38-0.3-0.8-0.3-1.25C5.5,15.23,6.73,14,8.25,14H10.34z M8.5,18l1.15-2.5h-1.4C7.56,15.5,7,16.06,7,16.75S7.56,18,8.25,18H8.5z",
            }
        }
    }
}

pub fn personal_injury_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                path {
                    d: "M8,6c0-2.21,1.79-4,4-4s4,1.79,4,4c0,2.21-1.79,4-4,4S8,8.21,8,6z M17,22h1c1.1,0,2-0.9,2-2l0-4.78 c0-1.12-0.61-2.15-1.61-2.66c-0.43-0.22-0.9-0.43-1.39-0.62L17,22z M12.34,17L15,11.33C14.07,11.12,13.07,11,12,11 c-2.53,0-4.71,0.7-6.39,1.56C4.61,13.07,4,14.1,4,15.22L4,22h2.34C6.12,21.55,6,21.04,6,20.5C6,18.57,7.57,17,9.5,17H12.34z M10,22 l1.41-3H9.5C8.67,19,8,19.67,8,20.5S8.67,22,9.5,22H10z",
                }
            }
        }
    }
}

pub fn person_2_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M8.27,10h3.46c0.93,0,1.63-0.83,1.48-1.75l-0.3-1.79C12.67,5.04,11.44,4,10,4S7.33,5.04,7.09,6.47l-0.3,1.79 C6.64,9.17,7.34,10,8.27,10z",
                    }
                    path {
                        d: "M15.03,12.37C13.56,11.5,11.84,11,10,11s-3.56,0.5-5.03,1.37C4.36,12.73,4,13.39,4,14.09l0,0.41C4,15.33,4.67,16,5.5,16h9 c0.83,0,1.5-0.67,1.5-1.5l0-0.41C16,13.39,15.64,12.73,15.03,12.37z",
                    }
                }
            }
        }
    }
}

pub fn person_2_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M18.39,14.56C16.71,13.7,14.53,13,12,13c-2.53,0-4.71,0.7-6.39,1.56C4.61,15.07,4,16.1,4,17.22L4,18c0,1.1,0.9,2,2,2h12 c1.1,0,2-0.9,2-2l0-0.78C20,16.1,19.39,15.07,18.39,14.56z",
                    }
                    path {
                        d: "M9.78,12h4.44c1.21,0,2.14-1.06,1.98-2.26l-0.32-2.45C15.57,5.39,13.92,4,12,4S8.43,5.39,8.12,7.29L7.8,9.74 C7.64,10.94,8.57,12,9.78,12z",
                    }
                }
            }
        }
    }
}

pub fn person_3_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M8.4,10c0.14,0,3.06,0,3.2,0c1.33,0,2.4-1.07,2.4-2.4c0-0.59-0.22-1.12-0.57-1.53c0.1-0.27,0.17-0.56,0.17-0.87 c0-1-0.61-1.86-1.49-2.22C11.6,2.39,10.85,2,10,2S8.4,2.39,7.89,2.98C7.01,3.34,6.4,4.2,6.4,5.2c0,0.31,0.06,0.6,0.17,0.87 C6.22,6.48,6,7.01,6,7.6C6,8.93,7.07,10,8.4,10z",
                    }
                    path {
                        d: "M15.03,12.37C13.56,11.5,11.84,11,10,11s-3.56,0.5-5.03,1.37C4.36,12.73,4,13.39,4,14.09l0,0.41C4,15.33,4.67,16,5.5,16h9 c0.83,0,1.5-0.67,1.5-1.5l0-0.41C16,13.39,15.64,12.73,15.03,12.37z",
                    }
                }
            }
        }
    }
}

pub fn person_3_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M18.39,14.56C16.71,13.7,14.53,13,12,13c-2.53,0-4.71,0.7-6.39,1.56C4.61,15.07,4,16.1,4,17.22L4,18c0,1.1,0.9,2,2,2h12 c1.1,0,2-0.9,2-2l0-0.78C20,16.1,19.39,15.07,18.39,14.56z",
                        }
                    }
                    g {
                        path {
                            d: "M10,12c0.17,0,3.83,0,4,0c1.66,0,3-1.34,3-3c0-0.73-0.27-1.4-0.71-1.92C16.42,6.75,16.5,6.38,16.5,6 c0-1.25-0.77-2.32-1.86-2.77C14,2.48,13.06,2,12,2s-2,0.48-2.64,1.23C8.27,3.68,7.5,4.75,7.5,6c0,0.38,0.08,0.75,0.21,1.08 C7.27,7.6,7,8.27,7,9C7,10.66,8.34,12,10,12z",
                        }
                    }
                }
            }
        }
    }
}

pub fn person_4_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,10c1.66,0,3-1.34,3-3c0-1.03,0-2.83,0-2.83v0c0-0.62-0.5-1.12-1.12-1.12c-0.4,0-0.74,0.22-0.94,0.54 C10.74,3.26,10.4,3.04,10,3.04c-0.39,0-0.72,0.21-0.92,0.52C8.88,3.23,8.54,3,8.12,3C7.5,3,7,3.5,7,4.12c0,0,0,1.84,0,2.88 C7,8.66,8.34,10,10,10z",
                    }
                    path {
                        d: "M15.03,12.37C13.56,11.5,11.84,11,10,11s-3.56,0.5-5.03,1.37C4.36,12.73,4,13.39,4,14.09l0,0.41C4,15.33,4.67,16,5.5,16h9 c0.83,0,1.5-0.67,1.5-1.5l0-0.41C16,13.39,15.64,12.73,15.03,12.37z",
                    }
                }
            }
        }
    }
}

pub fn person_4_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M18.39,14.56C16.71,13.7,14.53,13,12,13c-2.53,0-4.71,0.7-6.39,1.56C4.61,15.07,4,16.1,4,17.22L4,18c0,1.1,0.9,2,2,2h12 c1.1,0,2-0.9,2-2l0-0.78C20,16.1,19.39,15.07,18.39,14.56z",
                        }
                    }
                    g {
                        path {
                            d: "M12,12c2.21,0,4-1.79,4-4c0-1.37,0-3.5,0-3.5C16,3.67,15.33,3,14.5,3c-0.52,0-0.98,0.27-1.25,0.67 C12.98,3.27,12.52,3,12,3s-0.98,0.27-1.25,0.67C10.48,3.27,10.02,3,9.5,3C8.67,3,8,3.67,8,4.5c0,0,0,2.12,0,3.5 C8,10.21,9.79,12,12,12z",
                        }
                    }
                }
            }
        }
    }
}

pub fn person_add_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm-9-2V8c0-.55-.45-1-1-1s-1 .45-1 1v2H2c-.55 0-1 .45-1 1s.45 1 1 1h2v2c0 .55.45 1 1 1s1-.45 1-1v-2h2c.55 0 1-.45 1-1s-.45-1-1-1H6zm9 4c-2.67 0-8 1.34-8 4v1c0 .55.45 1 1 1h14c.55 0 1-.45 1-1v-1c0-2.66-5.33-4-8-4z",
            }
        }
    }
}

pub fn person_add_alt_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cx: "8",
                        r: "3",
                        cy: "7",
                    }
                    path {
                        d: "M13.03,12.37C11.56,11.5,9.84,11,8,11s-3.56,0.5-5.03,1.37C2.36,12.72,2,13.39,2,14.09V16h12v-1.91 C14,13.39,13.64,12.72,13.03,12.37z",
                    }
                    path {
                        d: "M17.25,7.75h-1v-1C16.25,6.34,15.91,6,15.5,6s-0.75,0.34-0.75,0.75v1h-1C13.34,7.75,13,8.09,13,8.5 c0,0.41,0.34,0.75,0.75,0.75h1v1c0,0.41,0.34,0.75,0.75,0.75s0.75-0.34,0.75-0.75v-1h1C17.66,9.25,18,8.91,18,8.5 C18,8.09,17.66,7.75,17.25,7.75z",
                    }
                }
            }
        }
    }
}

pub fn person_add_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.39,14.56C13.71,13.7,11.53,13,9,13c-2.53,0-4.71,0.7-6.39,1.56C1.61,15.07,1,16.1,1,17.22V20h16v-2.78 C17,16.1,16.39,15.07,15.39,14.56z M9,12c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4S5,5.79,5,8C5,10.21,6.79,12,9,12z M20,9V7 c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1 v-2h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H20z",
                }
            }
        }
    }
}

pub fn person_add_alt_1_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "16,9 16,7 15,7 15,9 13,9 13,10 15,10 15,12 16,12 16,10 18,10 18,9",
                    }
                    path {
                        d: "M8,11c-2,0-6,0.96-6,2.86v1.64C2,15.78,2.22,16,2.5,16h11c0.28,0,0.5-0.22,0.5-0.5v-1.64C14,11.96,10,11,8,11z",
                    }
                    circle {
                        r: "3",
                        cx: "8",
                        cy: "7",
                    }
                }
            }
        }
    }
}

pub fn person_add_alt_1_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    circle {
                        r: "4",
                        cx: "9",
                        cy: "8",
                    }
                    path {
                        d: "M9,14c-2.67,0-8,1.34-8,4v1c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1v-1C17,15.34,11.67,14,9,14z",
                    }
                    polygon {
                        points: "20,10 20,7 18,7 18,10 15,10 15,12 18,12 18,15 20,15 20,12 23,12 23,10",
                    }
                }
            }
        }
    }
}

pub fn person_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M11.64,9.51C12.46,8.98,13,8.05,13,7c0-1.66-1.34-3-3-3C8.95,4,8.02,4.54,7.49,5.36L11.64,9.51z M15.99,13.87 c-0.07-0.62-0.41-1.18-0.95-1.5c-0.35-0.21-0.72-0.39-1.1-0.56L15.99,13.87z M2.4,3.46L2.4,3.46c-0.29,0.29-0.29,0.77,0,1.06 l6.54,6.54c-1.43,0.15-2.77,0.61-3.96,1.31C4.37,12.72,4,13.38,4,14.09L4,16h9.88l1.6,1.6c0.29,0.29,0.77,0.29,1.06,0l0,0 c0.29-0.29,0.29-0.77,0-1.06L3.46,3.46C3.17,3.17,2.69,3.17,2.4,3.46z",
            }
        }
    }
}

pub fn person_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                path {
                    d: "M8.65,5.82C9.36,4.72,10.6,4,12,4c2.21,0,4,1.79,4,4c0,1.4-0.72,2.64-1.82,3.35L8.65,5.82z M20,17.17 c-0.02-1.1-0.63-2.11-1.61-2.62c-0.54-0.28-1.13-0.54-1.77-0.76L20,17.17z M20.49,20.49L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0l0,0 c-0.39,0.39-0.39,1.02,0,1.41l8.18,8.18c-1.82,0.23-3.41,0.8-4.7,1.46C4.6,15.08,4,16.11,4,17.22L4,20h13.17l1.9,1.9 c0.39,0.39,1.02,0.39,1.41,0l0,0C20.88,21.51,20.88,20.88,20.49,20.49z",
                }
            }
        }
    }
}

pub fn person_outline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 5.9c1.16 0 2.1.94 2.1 2.1s-.94 2.1-2.1 2.1S9.9 9.16 9.9 8s.94-2.1 2.1-2.1m0 9c2.97 0 6.1 1.46 6.1 2.1v1.1H5.9V17c0-.64 3.13-2.1 6.1-2.1M12 4C9.79 4 8 5.79 8 8s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm0 9c-2.67 0-8 1.34-8 4v2c0 .55.45 1 1 1h14c.55 0 1-.45 1-1v-2c0-2.66-5.33-4-8-4z",
            }
        }
    }
}

pub fn person_remove_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M8,11c2,0,6,0.96,6,2.86v1.64c0,0.28-0.22,0.5-0.5,0.5h-11C2.22,16,2,15.78,2,15.5v-1.64C2,11.96,6,11,8,11z",
                    }
                    circle {
                        r: "3",
                        cx: "8",
                        cy: "7",
                    }
                    path {
                        d: "M13.5,10h3c0.28,0,0.5-0.22,0.5-0.5v0C17,9.22,16.78,9,16.5,9h-3C13.22,9,13,9.22,13,9.5v0C13,9.78,13.22,10,13.5,10z",
                    }
                }
            }
        }
    }
}

pub fn person_remove_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M2,18v1c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1v-1 c0-2.66-5.33-4-8-4S2,15.34,2,18z M18,10h4c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h-4c-0.55,0-1-0.45-1-1v0 C17,10.45,17.45,10,18,10z",
                }
            }
        }
    }
}

pub fn person_remove_alt_1_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M8,11c2,0,6,0.96,6,2.86v1.64c0,0.28-0.22,0.5-0.5,0.5h-11C2.22,16,2,15.78,2,15.5v-1.64C2,11.96,6,11,8,11z",
                    }
                    circle {
                        r: "3",
                        cy: "7",
                        cx: "8",
                    }
                    path {
                        d: "M13.5,10h3c0.28,0,0.5-0.22,0.5-0.5v0C17,9.22,16.78,9,16.5,9h-3C13.22,9,13,9.22,13,9.5v0C13,9.78,13.22,10,13.5,10z",
                    }
                }
            }
        }
    }
}

pub fn person_remove_alt_1_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M14,8c0-2.21-1.79-4-4-4S6,5.79,6,8s1.79,4,4,4S14,10.21,14,8z M2,18v1c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1v-1 c0-2.66-5.33-4-8-4S2,15.34,2,18z M18,10h4c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h-4c-0.55,0-1-0.45-1-1v0 C17,10.45,17.45,10,18,10z",
                }
            }
        }
    }
}

pub fn piano_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M7,15.5H4.5v-11H6v7C6,11.78,6.22,12,6.5,12H7V15.5z M12,15.5H8V12h0.5C8.78,12,9,11.78,9,11.5v-7h2v7c0,0.28,0.22,0.5,0.5,0.5H12 V15.5z M15.5,15.5H13V12h0.5c0.28,0,0.5-0.22,0.5-0.5v-7h1.5V15.5z",
                }
            }
        }
    }
}

pub fn piano_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M14,14.5h0.25V19h-4.5v-4.5H10 c0.55,0,1-0.45,1-1V5h2v8.5C13,14.05,13.45,14.5,14,14.5z M5,5h2v8.5c0,0.55,0.45,1,1,1h0.25V19H5V5z M19,19h-3.25v-4.5H16 c0.55,0,1-0.45,1-1V5h2V19z",
            }
        }
    }
}

pub fn piano_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M16.54,17.6c0.29-0.29,0.29-0.77,0-1.06L3.46,3.46c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.77,0,1.06L3,5.12V15.5 C3,16.33,3.67,17,4.5,17h10.38l0.6,0.6C15.77,17.89,16.25,17.89,16.54,17.6z M7,15.5H4.5V6.62L6,8.12v3.38C6,11.78,6.22,12,6.5,12 H7V15.5z M8,15.5V12h0.5C8.78,12,9,11.78,9,11.5v-0.38l3,3v1.38H8z M5.12,3H15.5C16.33,3,17,3.67,17,4.5v10.38l-1.5-1.5V4.5H14v7 c0,0.11-0.04,0.2-0.1,0.28L11,8.88V4.5H9v2.38L5.12,3z",
                }
            }
        }
    }
}

pub fn piano_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20.49,21.9c0.39-0.39,0.39-1.02,0-1.41L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L3,5.83V19 c0,1.1,0.9,2,2,2h13.17l0.9,0.9C19.46,22.29,20.09,22.29,20.49,21.9z M8.25,19H5V7.83l2,2v3.67c0,0.55,0.45,1,1,1h0.25V19z M9.75,19 v-4.5H10c0.46,0,0.82-0.31,0.94-0.73l3.31,3.31V19H9.75z M11,8.17L5.83,3H19c1.1,0,2,0.9,2,2v13.17l-2-2V5h-2v8.5 c0,0.19-0.07,0.36-0.16,0.51L13,10.17V5h-2V8.17z",
            }
        }
    }
}

pub fn pix_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12.79,13.58l-2.41-2.41c-0.1-0.1-0.22-0.12-0.28-0.12c-0.06,0-0.18,0.02-0.28,0.12L7.4,13.59 c-0.71,0.71-1.45,0.7-2.13,0.7l3.02,3.01c0.94,0.94,2.46,0.94,3.39,0l3.03-3.02C14,14.28,13.38,14.17,12.79,13.58z",
                    }
                    path {
                        d: "M7.4,6.41l2.42,2.42C9.9,8.9,10,8.94,10.1,8.94s0.21-0.04,0.28-0.12l2.39-2.39c0.58-0.59,1.21-0.71,1.96-0.71L11.71,2.7 c-0.94-0.94-2.46-0.94-3.39,0L5.29,5.71C6.02,5.71,6.7,5.71,7.4,6.41z",
                    }
                    path {
                        d: "M17.29,8.28l-1.8-1.81h-1.01c-0.44,0-0.86,0.17-1.16,0.48l-2.4,2.4c-0.22,0.22-0.52,0.34-0.81,0.34 c-0.29,0-0.59-0.12-0.81-0.34L6.87,6.94C6.57,6.63,6.15,6.46,5.72,6.46H4.53L2.7,8.3c-0.94,0.94-0.94,2.46,0,3.39l1.83,1.84h1.18 c0.43,0,0.85-0.17,1.16-0.48l2.42-2.42c0.22-0.22,0.52-0.34,0.81-0.34s0.59,0.11,0.81,0.34l2.41,2.41 c0.08,0.08,0.16,0.15,0.25,0.2c0.27,0.18,0.58,0.28,0.91,0.28h1.01l1.8-1.81c0.36-0.36,0.58-0.8,0.67-1.25 C17.99,10.31,18,10.16,18,10C18,9.38,17.76,8.76,17.29,8.28z",
                    }
                }
            }
        }
    }
}

pub fn pix_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.45,16.52l-3.01-3.01c-0.11-0.11-0.24-0.13-0.31-0.13s-0.2,0.02-0.31,0.13L8.8,16.53c-0.34,0.34-0.87,0.89-2.64,0.89 l3.71,3.7c1.17,1.17,3.07,1.17,4.24,0l3.72-3.71C16.92,17.41,16.16,17.23,15.45,16.52z",
                    }
                    path {
                        d: "M8.8,7.47l3.02,3.02c0.08,0.08,0.2,0.13,0.31,0.13s0.23-0.05,0.31-0.13l2.99-2.99c0.71-0.74,1.52-0.91,2.43-0.91 l-3.72-3.71c-1.17-1.17-3.07-1.17-4.24,0l-3.71,3.7C7.95,6.58,8.49,7.16,8.8,7.47z",
                    }
                    path {
                        d: "M21.11,9.85l-2.25-2.26H17.6c-0.54,0-1.08,0.22-1.45,0.61l-3,3c-0.28,0.28-0.65,0.42-1.02,0.42 c-0.36,0-0.74-0.15-1.02-0.42L8.09,8.17c-0.38-0.38-0.9-0.6-1.45-0.6H5.17l-2.29,2.3c-1.17,1.17-1.17,3.07,0,4.24l2.29,2.3h1.48 c0.54,0,1.06-0.22,1.45-0.6l3.02-3.02c0.28-0.28,0.65-0.42,1.02-0.42c0.37,0,0.74,0.14,1.02,0.42l3.01,3.01 c0.38,0.38,0.9,0.6,1.45,0.6h1.26l2.25-2.26C22.3,12.96,22.3,11.04,21.11,9.85z",
                    }
                }
            }
        }
    }
}

pub fn plus_one_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9 8c-.55 0-1 .45-1 1v3H5c-.55 0-1 .45-1 1s.45 1 1 1h3v3c0 .55.45 1 1 1s1-.45 1-1v-3h3c.55 0 1-.45 1-1s-.45-1-1-1h-3V9c0-.55-.45-1-1-1zm5.5-1.21c0 .57.52 1 1.08.89L17 7.4V17c0 .55.45 1 1 1s1-.45 1-1V6.27c0-.65-.6-1.12-1.23-.97l-2.57.62c-.41.09-.7.46-.7.87z",
            }
        }
    }
}

pub fn poll_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM8 17c-.55 0-1-.45-1-1v-5c0-.55.45-1 1-1s1 .45 1 1v5c0 .55-.45 1-1 1zm4 0c-.55 0-1-.45-1-1V8c0-.55.45-1 1-1s1 .45 1 1v8c0 .55-.45 1-1 1zm4 0c-.55 0-1-.45-1-1v-2c0-.55.45-1 1-1s1 .45 1 1v2c0 .55-.45 1-1 1z",
            }
        }
    }
}

pub fn precision_manufacturing_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.29,7.78l-2.4,1.12L11,7.01V6.99l1.89-1.89l2.4,1.12c0.25,0.12,0.55,0.01,0.66-0.24c0.12-0.25,0.01-0.55-0.24-0.66 l-2.72-1.27C12.8,3.96,12.58,4,12.43,4.15L11,5.57V5.5C11,5.22,10.78,5,10.5,5C10.22,5,10,5.22,10,5.5v1H7.93 C7.67,5.49,6.64,4.78,5.49,5.06c-0.7,0.17-1.28,0.76-1.44,1.47C3.82,7.55,4.37,8.47,5.21,8.84L6.5,14H6c-0.55,0-1,0.45-1,1v0 c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1.14L7.56,8.23C7.73,8.02,7.86,7.77,7.93,7.5H10v1 C10,8.78,10.22,9,10.5,9C10.78,9,11,8.78,11,8.5V8.43l1.43,1.43c0.15,0.15,0.37,0.19,0.56,0.1l2.72-1.27 c0.25-0.12,0.36-0.41,0.24-0.66C15.84,7.77,15.54,7.66,15.29,7.78z M6,8C5.45,8,5,7.55,5,7c0-0.55,0.45-1,1-1s1,0.45,1,1 C7,7.55,6.55,8,6,8z",
                }
            }
        }
    }
}

pub fn precision_manufacturing_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M19.93,8.35l-3.6,1.68L14,7.7V6.3l2.33-2.33l3.6,1.68c0.38,0.18,0.82,0.01,1-0.36c0.18-0.38,0.01-0.82-0.36-1l-3.92-1.83 c-0.38-0.18-0.83-0.1-1.13,0.2L13.78,4.4C13.6,4.16,13.32,4,13,4c-0.55,0-1,0.45-1,1v1H8.82C8.34,4.66,6.96,3.75,5.4,4.06 C4.23,4.29,3.27,5.25,3.05,6.42C2.8,7.76,3.45,8.96,4.48,9.58L7.08,18H5.5C4.67,18,4,18.67,4,19.5v0C4,20.33,4.67,21,5.5,21h10 c0.83,0,1.5-0.67,1.5-1.5v0c0-0.83-0.67-1.5-1.5-1.5h-2.12L8.41,8.77C8.58,8.53,8.72,8.28,8.82,8H12v1c0,0.55,0.45,1,1,1 c0.32,0,0.6-0.16,0.78-0.4l1.74,1.74c0.3,0.3,0.75,0.38,1.13,0.2l3.92-1.83c0.38-0.18,0.54-0.62,0.36-1 C20.75,8.34,20.31,8.17,19.93,8.35z M6,8C5.45,8,5,7.55,5,7c0-0.55,0.45-1,1-1s1,0.45,1,1C7,7.55,6.55,8,6,8z",
                }
            }
        }
    }
}

pub fn psychology_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M11.16,4.05C8.58,3.68,6.38,5.5,6.06,7.91l-1.54,2.31C4.3,10.55,4.53,11,4.93,11H6v2c0,0.55,0.45,1,1,1h1v1 c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-2.76c1.39-0.93,2.23-2.62,1.94-4.47C14.64,5.86,13.07,4.32,11.16,4.05z M12.33,8.67 c0,0.09-0.01,0.18-0.02,0.26l0.56,0.44c0.05,0.04,0.07,0.11,0.03,0.17l-0.53,0.92c-0.03,0.06-0.1,0.08-0.16,0.06l-0.66-0.27 c-0.14,0.1-0.29,0.19-0.45,0.26L11,11.22c-0.01,0.07-0.06,0.11-0.13,0.11H9.8c-0.07,0-0.12-0.05-0.13-0.11l-0.1-0.71 c-0.16-0.07-0.31-0.15-0.45-0.26l-0.66,0.27c-0.06,0.02-0.13,0-0.16-0.06L7.76,9.54C7.73,9.48,7.74,9.41,7.79,9.37l0.56-0.44 C8.34,8.84,8.33,8.75,8.33,8.67s0.01-0.18,0.02-0.26L7.79,7.97C7.74,7.93,7.73,7.86,7.76,7.8l0.53-0.92 c0.03-0.06,0.1-0.08,0.16-0.06l0.66,0.27c0.14-0.1,0.29-0.19,0.45-0.26l0.1-0.71C9.68,6.05,9.73,6,9.8,6h1.07 c0.07,0,0.12,0.05,0.13,0.11l0.1,0.71c0.16,0.07,0.31,0.15,0.45,0.26l0.66-0.27c0.06-0.02,0.13,0,0.16,0.06l0.53,0.92 c0.03,0.06,0.02,0.13-0.03,0.17l-0.56,0.44C12.33,8.49,12.33,8.58,12.33,8.67z",
                    }
                    path {
                        d: "M10.33,7.71c-0.52,0-0.95,0.43-0.95,0.95c0,0.52,0.43,0.95,0.95,0.95s0.95-0.43,0.95-0.95 C11.29,8.14,10.86,7.71,10.33,7.71z",
                    }
                }
            }
        }
    }
}

pub fn psychology_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M13,8.57c-0.79,0-1.43,0.64-1.43,1.43s0.64,1.43,1.43,1.43s1.43-0.64,1.43-1.43S13.79,8.57,13,8.57z",
                    }
                    path {
                        d: "M13.21,3c-3.84-0.11-7,2.87-7.19,6.64L4.1,12.2C3.85,12.53,4.09,13,4.5,13H6v3c0,1.1,0.9,2,2,2h1v2c0,0.55,0.45,1,1,1h5 c0.55,0,1-0.45,1-1v-3.68c2.44-1.16,4.1-3.68,4-6.58C19.86,6.12,16.82,3.11,13.21,3z M16,10c0,0.13-0.01,0.26-0.02,0.39l0.83,0.66 c0.08,0.06,0.1,0.16,0.05,0.25l-0.8,1.39c-0.05,0.09-0.16,0.12-0.24,0.09l-0.99-0.4c-0.21,0.16-0.43,0.29-0.67,0.39L14,13.83 c-0.01,0.1-0.1,0.17-0.2,0.17h-1.6c-0.1,0-0.18-0.07-0.2-0.17l-0.15-1.06c-0.25-0.1-0.47-0.23-0.68-0.39l-0.99,0.4 c-0.09,0.03-0.2,0-0.25-0.09l-0.8-1.39c-0.05-0.08-0.03-0.19,0.05-0.25l0.84-0.66C10.01,10.26,10,10.13,10,10 c0-0.13,0.02-0.27,0.04-0.39L9.19,8.95c-0.08-0.06-0.1-0.16-0.05-0.26l0.8-1.38c0.05-0.09,0.15-0.12,0.24-0.09l1,0.4 c0.2-0.15,0.43-0.29,0.67-0.39l0.15-1.06C12.02,6.07,12.1,6,12.2,6h1.6c0.1,0,0.18,0.07,0.2,0.17l0.15,1.06 c0.24,0.1,0.46,0.23,0.67,0.39l1-0.4c0.09-0.03,0.2,0,0.24,0.09l0.8,1.38c0.05,0.09,0.03,0.2-0.05,0.26l-0.85,0.66 C15.99,9.73,16,9.86,16,10z",
                    }
                }
            }
        }
    }
}

pub fn psychology_alt_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.87,3.01C8.08,2.83,5.71,4.72,5.13,7.3L3.2,10.2C2.68,10.97,3.24,12,4.16,12h0.83v1.5c0,0.83,0.67,1.5,1.5,1.5H8v1.25 C8,16.66,8.34,17,8.75,17h4c0.41,0,0.75-0.34,0.75-0.75v-3.14c1.53-1,2.53-2.74,2.5-4.71C15.95,5.59,13.67,3.19,10.87,3.01z M10.25,12c-0.35,0-0.63-0.28-0.63-0.63c0-0.35,0.28-0.62,0.63-0.62c0.35,0,0.62,0.27,0.62,0.62C10.88,11.72,10.61,12,10.25,12z M11.76,8.3c-0.38,0.56-0.74,0.73-0.93,1.09c-0.08,0.14-0.11,0.24-0.11,0.71H9.8c0-0.25-0.04-0.65,0.16-0.99 c0.24-0.44,0.71-0.7,0.98-1.08c0.29-0.41,0.13-1.16-0.68-1.16c-0.53,0-0.79,0.4-0.9,0.74L8.53,7.25C8.76,6.58,9.36,6,10.25,6 c0.74,0,1.25,0.34,1.5,0.76C11.97,7.12,12.1,7.8,11.76,8.3z",
                }
            }
        }
    }
}

pub fn psychology_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M19.94,9.06C19.5,5.73,16.57,3,13,3C9.47,3,6.57,5.61,6.08,9l-1.93,3.48C3.74,13.14,4.22,14,5,14h1l0,2c0,1.1,0.9,2,2,2h1 v2c0,0.55,0.45,1,1,1h5c0.55,0,1-0.45,1-1l0-3.68C18.62,15.07,20.35,12.24,19.94,9.06z M12.5,14c-0.41,0-0.74-0.33-0.74-0.74 c0-0.41,0.33-0.73,0.74-0.73c0.41,0,0.73,0.32,0.73,0.73C13.23,13.67,12.92,14,12.5,14z M14.26,9.68 c-0.44,0.65-0.86,0.85-1.09,1.27c-0.09,0.17-0.13,0.28-0.13,0.82h-1.06c0-0.29-0.04-0.75,0.18-1.16c0.28-0.51,0.83-0.81,1.14-1.26 c0.33-0.47,0.15-1.36-0.8-1.36c-0.62,0-0.92,0.47-1.05,0.86l-0.96-0.4C10.76,7.67,11.46,7,12.5,7c0.86,0,1.45,0.39,1.75,0.88 C14.51,8.31,14.66,9.1,14.26,9.68z",
                }
            }
        }
    }
}

pub fn public_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z",
            }
        }
    }
}

pub fn public_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M11,8.17L6.49,3.66C8.07,2.61,9.96,2,12,2c5.52,0,10,4.48,10,10c0,2.04-0.61,3.93-1.66,5.51l-1.46-1.46 C19.59,14.87,20,13.48,20,12c0-3.35-2.07-6.22-5-7.41V5c0,1.1-0.9,2-2,2h-2V8.17z M20.49,21.9L20.49,21.9 c-0.39,0.39-1.02,0.39-1.41,0l-1.56-1.56c-2.07,1.37-4.68,2-7.45,1.48c-3.95-0.75-7.13-3.92-7.88-7.88 c-0.52-2.77,0.1-5.38,1.48-7.45L2.1,4.93c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0l16.97,16.97 C20.88,20.88,20.88,21.51,20.49,21.9z M11,18c-1.1,0-2-0.9-2-2v-1l-4.79-4.79C4.08,10.79,4,11.38,4,12c0,4.08,3.05,7.44,7,7.93V18z",
                }
            }
        }
    }
}

pub fn real_estate_agent_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M2.5,18C3.33,18,4,17.33,4,16.5v-6C4,9.67,3.33,9,2.5,9S1,9.67,1,10.5v6C1,17.33,1.67,18,2.5,18z M16.38,5.05l-4-2.91 c-0.53-0.38-1.24-0.38-1.76,0l-4,2.91C6.23,5.33,6,5.78,6,6.26V7.5h1.07c0.29,0,0.57,0.05,0.84,0.14l5.93,2.11 c1,0.35,1.66,1.3,1.66,2.36v0.39H17V6.26C17,5.78,16.77,5.33,16.38,5.05z M10.75,8.25c-0.21,0-0.38-0.17-0.38-0.38 s0.17-0.38,0.38-0.38s0.38,0.17,0.38,0.38S10.96,8.25,10.75,8.25z M10.75,6.75c-0.21,0-0.38-0.17-0.38-0.38S10.54,6,10.75,6 s0.38,0.17,0.38,0.38S10.96,6.75,10.75,6.75z M12.25,8.25c-0.21,0-0.38-0.17-0.38-0.38s0.17-0.38,0.38-0.38s0.38,0.17,0.38,0.38 S12.46,8.25,12.25,8.25z M12.25,6.75c-0.21,0-0.38-0.17-0.38-0.38S12.04,6,12.25,6s0.38,0.17,0.38,0.38S12.46,6.75,12.25,6.75z M10.5,14l-1.03-0.34C9.2,13.57,9.05,13.27,9.15,13v0c0.1-0.28,0.42-0.42,0.69-0.3L10.5,13h2.62c0.49,0,0.88-0.39,0.88-0.88v0 c0-0.37-0.23-0.7-0.58-0.83l-6.17-2.2C7.08,9.03,6.91,9,6.74,9H5.5v7.36l5.53,1.51c0.31,0.08,0.64,0.07,0.93-0.05L18,15.5v0 c0-0.83-0.67-1.5-1.5-1.5H10.5z",
            }
        }
    }
}

pub fn real_estate_agent_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3,22c1.1,0,2-0.9,2-2v-7c0-1.1-0.9-2-2-2s-2,0.9-2,2v7C1,21.1,1.9,22,3,22z M11.37,16.43l1.47,0.51 c0.11,0.04,0.22,0.06,0.33,0.06h6.83c1.11,0,2,0.89,2.01,2l0,0l-7.39,2.77c-0.4,0.15-0.84,0.17-1.25,0.05L7,20.02V11H8.6 c0.24,0,0.48,0.04,0.7,0.13l6.93,2.59c0.46,0.17,0.77,0.61,0.77,1.11v0c0,0.65-0.53,1.18-1.18,1.18h-2.63 c-0.12,0-0.24-0.02-0.36-0.07l-1.12-0.43c-0.26-0.1-0.55,0.04-0.64,0.3C10.98,16.06,11.11,16.34,11.37,16.43z M20.16,5.9l-5-3.57 c-0.7-0.5-1.63-0.5-2.32,0l-5,3.57C7.31,6.28,7,6.88,7,7.53V9h1.61c0.25,0,0.51,0.05,0.74,0.14l7.79,3.11 C18.26,12.71,19,13.79,19,15h2V7.53C21,6.88,20.69,6.28,20.16,5.9z M13,10c-0.28,0-0.5-0.22-0.5-0.5C12.5,9.22,12.72,9,13,9 s0.5,0.22,0.5,0.5C13.5,9.78,13.28,10,13,10z M13,8c-0.28,0-0.5-0.22-0.5-0.5C12.5,7.22,12.72,7,13,7s0.5,0.22,0.5,0.5 C13.5,7.78,13.28,8,13,8z M15,10c-0.28,0-0.5-0.22-0.5-0.5C14.5,9.22,14.72,9,15,9s0.5,0.22,0.5,0.5C15.5,9.78,15.28,10,15,10z M15,8c-0.28,0-0.5-0.22-0.5-0.5C14.5,7.22,14.72,7,15,7s0.5,0.22,0.5,0.5C15.5,7.78,15.28,8,15,8z",
            }
        }
    }
}

pub fn recommend_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M18,11.8c0.02,0.17-0.02,0.35-0.1,0.5l-2.1,4.9 c-0.22,0.51-0.74,0.83-1.3,0.8H9c-1.1,0-2-0.9-2-2v-5c-0.02-0.38,0.13-0.74,0.4-1L12,5l0.69,0.69c0.18,0.19,0.29,0.44,0.3,0.7v0.2 L12.41,10H17c0.55,0,1,0.45,1,1V11.8z",
                    }
                }
            }
        }
    }
}

pub fn recycling_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M7.19 7.46 5.47 6.43c-.23-.14-.31-.45-.17-.69l1.51-2.51c.39-.65 1.33-.65 1.72 0l.89 1.49-1.54 2.57c-.15.24-.45.31-.69.17zm8.73 1.27-1.75 1.01c-.24.14-.32.45-.18.69L15.55 13h.85c.57 0 1.09-.32 1.34-.83.23-.46.21-1-.06-1.44l-1.09-1.82c-.14-.24-.44-.31-.67-.18zM4.37 15.08c-.22.37-.24.83-.05 1.22.22.43.66.7 1.14.7H8c.28 0 .5-.22.5-.5v-2c0-.28-.22-.5-.5-.5H5.02l-.65 1.08zm10.1-10.75-.46.27-1.13-1.88c-.26-.44-.75-.72-1.28-.72H8.95l2.49 4.15-.46.28c-.31.18-.24.64.11.73l2.79.7c.21.05.43-.08.48-.29l.7-2.79c.09-.35-.29-.63-.59-.45zM5.3 11.58l.25.15c.38.23.85-.12.74-.55l-.61-2.46c-.07-.27-.34-.43-.61-.36l-2.46.61c-.43.11-.51.68-.13.91l.25.15-.42.69c-.26.44-.28.98-.06 1.44l1.22 2.44 1.83-3.02zM14.4 17c.57 0 1.09-.32 1.34-.83L16.83 14H13v-.57c0-.36-.43-.53-.68-.28l-2.03 2.03c-.16.16-.16.41 0 .57l2.03 2.03c.25.25.68.07.68-.28V17h1.4z",
                }
            }
        }
    }
}

pub fn recycling_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M6.2 7.41c-.23-.14-.31-.45-.17-.68L7.2 4.78l1.03-1.71c.39-.65 1.33-.65 1.72 0l1.48 2.46-1.23 2.06-.72 1.2c-.16.23-.47.31-.71.17L6.2 7.41zm15.52 5.56-1.34-2.24c-.14-.23-.44-.31-.68-.18l-2.6 1.5c-.24.14-.32.45-.18.69L18.87 16h1.09c.61 0 1.2-.26 1.59-.73.3-.37.45-.82.45-1.27 0-.36-.1-.71-.28-1.03zM16 21h1.5c.76 0 1.45-.43 1.79-1.11L20.74 17H16v-.79c0-.45-.54-.67-.85-.35l-2.79 2.79c-.2.2-.2.51 0 .71l2.79 2.79c.31.31.85.09.85-.35V21zm-6.5-4H5.7l-.84 1.41c-.3.5-.32 1.12-.06 1.65.28.57.87.94 1.52.94H9.5c.28 0 .5-.22.5-.5v-3c0-.28-.22-.5-.5-.5zm-3.38-2.65.7.42c.38.23.85-.12.74-.55l-.96-3.84c-.06-.26-.33-.43-.6-.36l-3.83.96c-.43.11-.52.68-.14.91l.66.4-.41.69c-.35.59-.38 1.31-.07 1.92l1.63 3.26 2.28-3.81zm10.9-9.21-1.3-2.17C15.35 2.37 14.7 2 14 2h-3.53l3.12 5.2-.69.41c-.38.23-.3.81.14.91l3.83.96c.27.07.54-.1.61-.36l.96-3.83c.11-.43-.36-.78-.74-.55l-.68.4z",
            }
        }
    }
}

pub fn reduce_capacity_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M16,4c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S16,5.1,16,4z M20.78,7.58C19.93,7.21,18.99,7,18,7c-0.67,0-1.31,0.1-1.92,0.28 C16.66,7.83,17,8.6,17,9.43V10h5V9.43C22,8.62,21.52,7.9,20.78,7.58z M6,6c1.1,0,2-0.9,2-2S7.1,2,6,2S4,2.9,4,4S4.9,6,6,6z M7.92,7.28C7.31,7.1,6.67,7,6,7C5.01,7,4.07,7.21,3.22,7.58C2.48,7.9,2,8.62,2,9.43V10h5V9.43C7,8.6,7.34,7.83,7.92,7.28z M10,4 c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S10,5.1,10,4z M16,10H8V9.43C8,8.62,8.48,7.9,9.22,7.58C10.07,7.21,11.01,7,12,7 c0.99,0,1.93,0.21,2.78,0.58C15.52,7.9,16,8.62,16,9.43V10z M15,16c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S15,17.1,15,16z M21,22h-8 v-0.57c0-0.81,0.48-1.53,1.22-1.85C15.07,19.21,16.01,19,17,19c0.99,0,1.93,0.21,2.78,0.58C20.52,19.9,21,20.62,21,21.43V22z M5,16 c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S5,17.1,5,16z M11,22H3v-0.57c0-0.81,0.48-1.53,1.22-1.85C5.07,19.21,6.01,19,7,19 c0.99,0,1.93,0.21,2.78,0.58C10.52,19.9,11,20.62,11,21.43V22z M13.79,13h-1.04v-1.25c0-0.41-0.34-0.75-0.75-0.75 s-0.75,0.34-0.75,0.75V13h-1.04c-0.45,0-0.67,0.54-0.35,0.85l1.79,1.79c0.2,0.2,0.51,0.2,0.71,0l1.79-1.79 C14.46,13.54,14.24,13,13.79,13z",
            }
        }
    }
}

pub fn remove_moderator_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M20,11.09v-4.7c0-0.83-0.52-1.58-1.3-1.87l-6-2.25c-0.45-0.17-0.95-0.17-1.4,0L6.78,3.96l12.09,12.09 C19.59,14.52,20,12.83,20,11.09z M20.49,20.49L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L4,6.83v4.26 c0,4.83,3.13,9.37,7.43,10.75c0.37,0.12,0.77,0.12,1.14,0c1.49-0.48,2.84-1.35,3.97-2.47l2.53,2.53c0.39,0.39,1.02,0.39,1.41,0 C20.88,21.51,20.88,20.88,20.49,20.49z",
                }
            }
        }
    }
}

pub fn roller_skating_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15,13.5c0.83,0,1.5-0.67,1.5-1.5v-1.5c0-1.1-0.72-2.08-1.78-2.4l-3.4-1.03c-0.33-0.1-0.6-0.3-0.79-0.57H8 C7.72,6.5,7.5,6.27,7.5,6S7.72,5.5,8,5.5h2.25v-1H8C7.72,4.5,7.5,4.27,7.5,4S7.72,3.5,8,3.5h2.25v-1c0-0.83-0.67-1.5-1.5-1.5H5 C4.17,1,3.5,1.67,3.5,2.5V12c0,0.83,0.67,1.5,1.5,1.5H15z M6.5,16.75c0-1.24-1.01-2.25-2.25-2.25S2,15.51,2,16.75S3.01,19,4.25,19 S6.5,17.99,6.5,16.75z M18,16.75c0-1.24-1.01-2.25-2.25-2.25s-2.25,1.01-2.25,2.25S14.51,19,15.75,19S18,17.99,18,16.75z M12.25,16.75c0-1.24-1.01-2.25-2.25-2.25s-2.25,1.01-2.25,2.25S8.76,19,10,19S12.25,17.99,12.25,16.75z",
                }
            }
        }
    }
}

pub fn roller_skating_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18,16c1.1,0,2-0.9,2-2l0-0.88c0-2.1-1.55-3.53-3.03-3.88l-2.7-0.67C13.4,8.35,12.7,7.76,12.32,7H9.5C9.22,7,9,6.78,9,6.5 S9.22,6,9.5,6h2.52L12,5H9.5C9.22,5,9,4.78,9,4.5C9,4.22,9.22,4,9.5,4H12V3c0-1.1-0.9-2-2-2H6C4.9,1,4,1.9,4,3v11c0,1.1,0.9,2,2,2 H18z M5,23c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S6.66,23,5,23z M19,23c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3 S20.66,23,19,23z M12,23c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S13.66,23,12,23z",
                    }
                }
            }
        }
    }
}

pub fn safety_divider_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11,5h2v14h-2V5z M5,12c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2C3,11.1,3.9,12,5,12z M7.78,13.58 C6.93,13.21,5.99,13,5,13s-1.93,0.21-2.78,0.58C1.48,13.9,1,14.62,1,15.43L1,16h8l0-0.57C9,14.62,8.52,13.9,7.78,13.58z M19,12 c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2C17,11.1,17.9,12,19,12z M21.78,13.58C20.93,13.21,19.99,13,19,13s-1.93,0.21-2.78,0.58 C15.48,13.9,15,14.62,15,15.43L15,16h8l0-0.57C23,14.62,22.52,13.9,21.78,13.58z",
            }
        }
    }
}

pub fn sanitizer_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15.5,6.5c0-0.56,0.67-1.49,1.11-2.04c0.2-0.25,0.58-0.25,0.77,0c0.44,0.55,1.11,1.48,1.11,2.04C18.5,7.33,17.83,8,17,8 S15.5,7.33,15.5,6.5z M19.5,15c1.38,0,2.5-1.12,2.5-2.5c0-1.25-1.41-3.16-2.11-4.04c-0.2-0.25-0.57-0.25-0.77,0 C18.41,9.34,17,11.25,17,12.5C17,13.88,18.12,15,19.5,15z M12,14h-1v-1c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v1H8 c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1v1c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-1h1c0.55,0,1-0.45,1-1v0 C13,14.45,12.55,14,12,14z M16,12v8c0,1.1-0.9,2-2,2H6c-1.1,0-2-0.9-2-2v-8c0-2.97,2.16-5.43,5-5.91V4H8C7.45,4,7,3.55,7,3v0 c0-0.55,0.45-1,1-1h5c0.61,0,1.19,0.11,1.72,0.31c0.67,0.25,0.83,1.13,0.33,1.64l0,0C14.77,4.23,14.36,4.31,14,4.18 C13.68,4.06,13.35,4,13,4h-2v2.09C13.84,6.57,16,9.03,16,12z",
            }
        }
    }
}

pub fn scale_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,18c-0.83,0-1.5-0.67-1.5-1.5c0-0.41,0.17-0.79,0.44-1.06l0,0c0.38-0.38,1.9-1.05,2.97-1.5 c0.41-0.17,0.82,0.24,0.65,0.65c-0.45,1.07-1.12,2.59-1.5,2.97C10.79,17.83,10.41,18,10,18z M13,17.25c0-0.41,0.34-0.75,0.75-0.75 l2.65,0c-0.77-5.48-5.79-5.65-6.4-5.65c-0.61,0-5.63,0.17-6.4,5.65l2.65,0C6.66,16.5,7,16.84,7,17.25C7,17.66,6.66,18,6.25,18 l-3.2,0c-0.59,0-1.05-0.52-1-1.11C2.48,11.86,5.91,10,8.5,9.5V7C5.49,6.64,3.07,5.2,2.28,3.34C2.01,2.7,2.53,2,3.22,2h13.56 c0.69,0,1.21,0.7,0.94,1.34C16.93,5.2,14.51,6.64,11.5,7v2.5c2.59,0.5,6.02,2.36,6.45,7.39c0.05,0.59-0.41,1.11-1,1.11l-3.2,0 C13.34,18,13,17.66,13,17.25z",
                }
            }
        }
    }
}

pub fn scale_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,21c0,0.55,0.45,1,1,1l3.43,0c0.87,0,1.58-0.75,1.5-1.62C21.34,14.18,17.4,11.68,14,11V8c3.31-0.42,6.03-1.86,7.27-3.73 C21.92,3.3,21.15,2,19.98,2H4.02C2.85,2,2.08,3.3,2.73,4.27C3.97,6.14,6.69,7.58,10,8l0,3c-3.4,0.68-7.34,3.18-7.93,9.38 C1.99,21.25,2.7,22,3.57,22L7,22c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H4.13c0.93-6.83,6.65-7.2,7.87-7.2s6.94,0.37,7.87,7.2H17 C16.45,20,16,20.45,16,21z M11.5,21.94c-0.7-0.17-1.27-0.74-1.44-1.44c-0.18-0.74,0.06-1.44,0.53-1.91 c0.55-0.55,2.91-1.57,4.33-2.15c0.41-0.17,0.82,0.24,0.65,0.65c-0.58,1.42-1.6,3.78-2.15,4.33C12.95,21.88,12.25,22.12,11.5,21.94z",
                }
            }
        }
    }
}

pub fn school_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5 13.18v2.81c0 .73.4 1.41 1.04 1.76l5 2.73c.6.33 1.32.33 1.92 0l5-2.73c.64-.35 1.04-1.03 1.04-1.76v-2.81l-6.04 3.3c-.6.33-1.32.33-1.92 0L5 13.18zm6.04-9.66-8.43 4.6c-.69.38-.69 1.38 0 1.76l8.43 4.6c.6.33 1.32.33 1.92 0L21 10.09V16c0 .55.45 1 1 1s1-.45 1-1V9.59c0-.37-.2-.7-.52-.88l-9.52-5.19c-.6-.32-1.32-.32-1.92 0z",
            }
        }
    }
}

pub fn science_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M16.68,14.58L12.5,9.49V4.5h0.75C13.66,4.5,14,4.16,14,3.75C14,3.34,13.66,3,13.25,3h-6.5C6.34,3,6,3.34,6,3.75 C6,4.16,6.34,4.5,6.75,4.5H7.5v4.99l-4.18,5.09C2.55,15.57,3.26,17,4.5,17H15.5C16.74,17,17.45,15.57,16.68,14.58z",
            }
        }
    }
}

pub fn science_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20.54,17.73L15,11V5h1c0.55,0,1-0.45,1-1s-0.45-1-1-1H8C7.45,3,7,3.45,7,4s0.45,1,1,1h1v6l-5.54,6.73 C3.14,18.12,3,18.56,3,19c0.01,1.03,0.82,2,2,2H19c1.19,0,2-0.97,2-2C21,18.56,20.86,18.12,20.54,17.73z",
            }
        }
    }
}

pub fn scoreboard_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,11h-1V9h1V11z M13.25,2.5c-0.41,0-0.75,0.34-0.75,0.75V4h-5V3.25c0-0.41-0.34-0.75-0.75-0.75S6,2.84,6,3.25V4H3.5 C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-9C18,4.67,17.33,4,16.5,4H14V3.25 C14,2.84,13.66,2.5,13.25,2.5z M15,11.5c0,0.28-0.22,0.5-0.5,0.5h-2c-0.28,0-0.5-0.22-0.5-0.5v-3C12,8.22,12.22,8,12.5,8h2 C14.78,8,15,8.22,15,8.5V11.5z M10.5,6c0,0.28-0.22,0.5-0.5,0.5S9.5,6.28,9.5,6S9.72,5.5,10,5.5S10.5,5.72,10.5,6z M7.5,8 C7.78,8,8,8.22,8,8.5V10c0,0.28-0.22,0.5-0.5,0.5H6V11h1.5C7.78,11,8,11.22,8,11.5S7.78,12,7.5,12h-2C5.22,12,5,11.78,5,11.5V10 c0-0.28,0.22-0.5,0.5-0.5H7V9H5.5C5.22,9,5,8.78,5,8.5S5.22,8,5.5,8H7.5z M10.5,8.67c0,0.28-0.22,0.5-0.5,0.5s-0.5-0.22-0.5-0.5 s0.22-0.5,0.5-0.5S10.5,8.39,10.5,8.67z M10.5,11.33c0,0.28-0.22,0.5-0.5,0.5s-0.5-0.22-0.5-0.5s0.22-0.5,0.5-0.5 S10.5,11.06,10.5,11.33z M10.5,14c0,0.28-0.22,0.5-0.5,0.5S9.5,14.28,9.5,14s0.22-0.5,0.5-0.5S10.5,13.72,10.5,14z",
                }
            }
        }
    }
}

pub fn scoreboard_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M17.5,13.5H16v-3h1.5V13.5z M16,2c-0.55,0-1,0.45-1,1v1H9V3c0-0.55-0.45-1-1-1S7,2.45,7,3v1H4C2.9,4,2,4.9,2,6v12 c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6c0-1.1-0.9-2-2-2h-3V3C17,2.45,16.55,2,16,2z M9.5,14.25C9.5,14.66,9.16,15,8.75,15H6 c-0.55,0-1-0.45-1-1v-1.5c0-0.55,0.45-1,1-1h2v-1H5.75C5.34,10.5,5,10.16,5,9.75S5.34,9,5.75,9H8.5c0.55,0,1,0.45,1,1v1.5 c0,0.55-0.45,1-1,1h-2v1h2.25C9.16,13.5,9.5,13.84,9.5,14.25z M19,14c0,0.55-0.45,1-1,1h-2.5c-0.55,0-1-0.45-1-1v-4 c0-0.55,0.45-1,1-1H18c0.55,0,1,0.45,1,1V14z M12.75,6.75c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75S11.59,6,12,6 S12.75,6.34,12.75,6.75z M12.75,10.25c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75S11.59,9.5,12,9.5S12.75,9.84,12.75,10.25z M12.75,13.75c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75S11.59,13,12,13S12.75,13.34,12.75,13.75z M12.75,17.25 c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75S11.59,16.5,12,16.5S12.75,16.84,12.75,17.25z",
                }
            }
        }
    }
}

pub fn scuba_diving_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M1,11c0,0.83,0.67,1.5,1.5,1.5S4,11.83,4,11S3.33,9.5,2.5,9.5S1,10.17,1,11z M18.5,2.5c0.28,0.28,0.28,0.72,0,1L17,5 l-1.31,3.49c-0.12,0.33-0.36,0.61-0.67,0.78l-3.89,2.18c-0.09,0.05-0.18,0.09-0.28,0.12l-3.94,1.25l-0.85,2.83l-2.12,2.71 c-0.28,0.35-0.78,0.42-1.14,0.15c-0.37-0.27-0.44-0.8-0.16-1.16l1.71-2.22l0.67-4.04c0.11-0.55,0.53-1.02,1.11-1.17L14,7.5l1.5-3 l2-2C17.78,2.22,18.22,2.22,18.5,2.5z M6.05,7.59L6.05,7.59c-0.19-0.63,0.16-1.3,0.8-1.5l3.11-0.95c0.26-0.08,0.54,0.07,0.62,0.33 l0.41,1.34c0.08,0.26-0.07,0.54-0.33,0.62L7.55,8.39C6.92,8.59,6.25,8.23,6.05,7.59z",
                }
            }
        }
    }
}

pub fn scuba_diving_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M1,13c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S1,14.1,1,13z M8.89,10.11l3.56-0.95c0.53-0.14,0.85-0.69,0.71-1.22L12.9,6.97 c-0.14-0.53-0.69-0.85-1.22-0.71L8.11,7.21c-0.8,0.21-1.28,1.04-1.06,1.84l0,0C7.27,9.85,8.09,10.33,8.89,10.11z M22.52,2.52 c-0.29-0.29-0.75-0.29-1.04,0L19,5l-2,4l-9.48,2.87c-0.82,0.2-1.39,0.89-1.5,1.68L5.24,18L3,21c-0.33,0.44-0.24,1.07,0.2,1.4 c0.44,0.33,1.07,0.24,1.4-0.2L7,19l1.14-3.14l5.57-1.77c0.19-0.06,0.38-0.15,0.54-0.27l4.2-2.94c0.36-0.25,0.62-0.61,0.75-1.02 L20.5,5.9l2.06-2.38C22.81,3.22,22.79,2.79,22.52,2.52z",
                }
            }
        }
    }
}

pub fn self_improvement_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cx: "10",
                        cy: "5.5",
                        r: "1.5",
                    }
                    path {
                        d: "M16,11.48L16,11.48c0-0.26-0.19-0.47-0.45-0.5c-1.46-0.13-2.83-0.9-3.6-2.04C11.58,8.37,10.97,8,10.29,8l-0.5,0 C9.03,8,8.42,8.37,8.04,8.94c-0.76,1.14-2.13,1.91-3.6,2.04C4.19,11,4,11.22,4,11.48v0c0,0.3,0.26,0.52,0.55,0.5 c1.73-0.13,3.05-0.84,3.95-1.94v2.45l-2.72,1.17c-0.38,0.16-0.69,0.5-0.76,0.91C4.88,15.34,5.47,16,6.21,16H8v-0.25 C8,14.78,8.78,14,9.75,14h2c0.14,0,0.25,0.11,0.25,0.25s-0.11,0.25-0.25,0.25h-2c-0.69,0-1.25,0.56-1.25,1.25V16l5.23,0 c0.59,0,1.14-0.39,1.25-0.97c0.11-0.57-0.19-1.13-0.71-1.35L11.5,12.5v-2.45c0.9,1.09,2.22,1.81,3.95,1.94 C15.74,12,16,11.78,16,11.48z",
                    }
                }
            }
        }
    }
}

pub fn self_improvement_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    circle {
                        cy: "6",
                        r: "2",
                        cx: "12",
                    }
                    path {
                        d: "M21,14.94L21,14.94c0-0.5-0.36-0.93-0.85-0.98c-1.88-0.21-3.49-1.13-4.75-2.63l-1.34-1.6C13.68,9.26,13.12,9,12.53,9 h-1.05c-0.59,0-1.15,0.26-1.53,0.72l-1.34,1.6c-1.25,1.5-2.87,2.42-4.75,2.63C3.36,14.01,3,14.44,3,14.94v0 c0,0.6,0.53,1.07,1.13,1c2.3-0.27,4.32-1.39,5.87-3.19V15l-3.76,1.5c-0.65,0.26-1.16,0.83-1.23,1.53C4.91,19.1,5.74,20,6.79,20H9 v-0.5c0-1.38,1.12-2.5,2.5-2.5h3c0.28,0,0.5,0.22,0.5,0.5S14.78,18,14.5,18h-3c-0.83,0-1.5,0.67-1.5,1.5V20l7.1,0 c0.85,0,1.65-0.54,1.85-1.37c0.21-0.89-0.27-1.76-1.08-2.08L14,15v-2.25c1.56,1.8,3.57,2.91,5.87,3.19C20.47,16,21,15.54,21,14.94 z",
                    }
                }
            }
        }
    }
}

pub fn sentiment_dissatisfied_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                r: "1.5",
                cx: "15.5",
                cy: "9.5",
            }
            circle {
                cy: "9.5",
                r: "1.5",
                cx: "8.5",
            }
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm0-6c-1.9 0-3.63.97-4.65 2.58-.22.35-.11.81.24 1.03.35.22.81.11 1.03-.24.74-1.18 2-1.88 3.38-1.88s2.64.7 3.38 1.88c.14.23.39.35.64.35.14 0 .27-.04.4-.11.35-.22.46-.68.24-1.03C15.63 14.96 13.9 14 12 14z",
            }
        }
    }
}

pub fn sentiment_neutral_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9.75 15.5h4.5c.41 0 .75-.34.75-.75s-.34-.75-.75-.75h-4.5c-.41 0-.75.34-.75.75s.34.75.75.75z",
            }
            circle {
                r: "1.5",
                cy: "9.5",
                cx: "15.5",
            }
            circle {
                cy: "9.5",
                r: "1.5",
                cx: "8.5",
            }
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

pub fn sentiment_satisfied_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                cx: "15.5",
                r: "1.5",
                cy: "9.5",
            }
            circle {
                cy: "9.5",
                cx: "8.5",
                r: "1.5",
            }
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm4.41-6.11c-.35-.22-.82-.11-1.03.24-.74 1.17-2 1.87-3.38 1.87s-2.64-.7-3.38-1.88c-.22-.35-.68-.46-1.03-.24-.35.22-.46.68-.24 1.03C8.37 16.54 10.1 17.5 12 17.5s3.63-.97 4.65-2.58c.22-.35.11-.81-.24-1.03z",
            }
        }
    }
}

pub fn sentiment_very_dissatisfied_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 13.5c-2.03 0-3.8 1.11-4.75 2.75-.19.33.06.75.44.75h8.62c.38 0 .63-.42.44-.75-.95-1.64-2.72-2.75-4.75-2.75zm-3.65-2.03l.53-.53.53.53c.29.29.77.29 1.06 0 .29-.29.29-.77 0-1.06l-.53-.53.53-.53c.29-.29.29-.77 0-1.06-.29-.29-.77-.29-1.06 0l-.53.53-.53-.53c-.29-.29-.77-.29-1.06 0-.29.29-.29.77 0 1.06l.53.53-.53.53c-.29.29-.29.77 0 1.06.29.29.77.29 1.06 0zM11.99 2C6.47 2 2 6.47 2 12s4.47 10 9.99 10S22 17.53 22 12 17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm3.65-11.71l-.53.53-.53-.53c-.29-.29-.77-.29-1.06 0-.29.29-.29.77 0 1.06l.53.53-.53.53c-.29.29-.29.77 0 1.06.29.29.77.29 1.06 0l.53-.53.53.53c.29.29.77.29 1.06 0 .29-.29.29-.77 0-1.06l-.53-.53.53-.53c.29-.29.29-.77 0-1.06-.29-.29-.77-.29-1.06 0z",
            }
        }
    }
}

pub fn sentiment_very_satisfied_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M8.88 9.94l.53.53c.29.29.77.29 1.06 0 .29-.29.29-.77 0-1.06l-.88-.88c-.39-.39-1.02-.39-1.41 0l-.89.88c-.29.29-.29.77 0 1.06.29.29.77.29 1.06 0l.53-.53zM12 17.5c2.03 0 3.8-1.11 4.75-2.75.19-.33-.05-.75-.44-.75H7.69c-.38 0-.63.42-.44.75.95 1.64 2.72 2.75 4.75 2.75zm1.53-7.03c.29.29.77.29 1.06 0l.53-.53.53.53c.29.29.77.29 1.06 0 .29-.29.29-.77 0-1.06l-.88-.88c-.39-.39-1.02-.39-1.41 0l-.88.88c-.3.29-.3.77-.01 1.06zM11.99 2C6.47 2 2 6.47 2 12s4.47 10 9.99 10S22 17.53 22 12 17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

pub fn severe_cold_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.25,2c-0.41,0-0.75,0.34-0.75,0.75v2.5C15.5,5.66,15.84,6,16.25,6S17,5.66,17,5.25v-2.5C17,2.34,16.66,2,16.25,2z",
                    }
                    circle {
                        cx: "16.25",
                        r: ".75",
                        cy: "7.75",
                    }
                    path {
                        d: "M9.75,8.81l2.72-2.72c0.29-0.29,0.29-0.77,0-1.06l0,0c-0.29-0.29-0.77-0.29-1.06,0L9.75,6.69V4.75C9.75,4.34,9.41,4,9,4h0 C8.59,4,8.25,4.34,8.25,4.75v1.94L6.59,5.03c-0.29-0.29-0.77-0.29-1.06,0l0,0c-0.29,0.29-0.29,0.77,0,1.06l2.72,2.72v1.44H6.81 L4.09,7.53c-0.29-0.29-0.77-0.29-1.06,0l0,0c-0.29,0.29-0.29,0.77,0,1.06l1.66,1.66H2.75C2.34,10.25,2,10.59,2,11v0 c0,0.41,0.34,0.75,0.75,0.75h1.94l-1.66,1.66c-0.29,0.29-0.29,0.77,0,1.06l0,0c0.29,0.29,0.77,0.29,1.06,0l2.72-2.72h1.44v1.44 l-2.72,2.72c-0.29,0.29-0.29,0.77,0,1.06l0,0c0.29,0.29,0.77,0.29,1.06,0l1.66-1.66v1.94C8.25,17.66,8.59,18,9,18h0 c0.41,0,0.75-0.34,0.75-0.75v-1.94l1.66,1.66c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06l-2.72-2.72v-1.44h1.44 l2.72,2.72c0.29,0.29,0.77,0.29,1.06,0l0,0c0.29-0.29,0.29-0.77,0-1.06l-1.66-1.66h1.94c0.41,0,0.75-0.34,0.75-0.75v0 c0-0.41-0.34-0.75-0.75-0.75h-5.5V8.81z",
                    }
                }
            }
        }
    }
}

pub fn severe_cold_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M20,2c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1V3C21,2.45,20.55,2,20,2z",
                    }
                    circle {
                        cy: "9",
                        r: "1",
                        cx: "20",
                    }
                    path {
                        d: "M12,10.41l3.29-3.29c0.39-0.39,0.39-1.02,0-1.41l0,0c-0.39-0.39-1.02-0.39-1.41,0L12,7.59V5c0-0.55-0.45-1-1-1h0 c-0.55,0-1,0.45-1,1v2.59L8.12,5.71c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41L10,10.41V12H8.41L5.12,8.71 c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41L5.59,12H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2.59l-1.88,1.88 c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0L8.41,14H10v1.59l-3.29,3.29c-0.39,0.39-0.39,1.02,0,1.41l0,0 c0.39,0.39,1.02,0.39,1.41,0L10,18.41V21c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2.59l1.88,1.88c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L12,15.59V14h1.59l3.29,3.29c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41L16.41,14H19 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-7V10.41z",
                    }
                }
            }
        }
    }
}

pub fn share_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18 16.08c-.76 0-1.44.3-1.96.77L8.91 12.7c.05-.23.09-.46.09-.7s-.04-.47-.09-.7l7.05-4.11c.54.5 1.25.81 2.04.81 1.66 0 3-1.34 3-3s-1.34-3-3-3-3 1.34-3 3c0 .24.04.47.09.7L8.04 9.81C7.5 9.31 6.79 9 6 9c-1.66 0-3 1.34-3 3s1.34 3 3 3c.79 0 1.5-.31 2.04-.81l7.12 4.16c-.05.21-.08.43-.08.65 0 1.61 1.31 2.92 2.92 2.92s2.92-1.31 2.92-2.92-1.31-2.92-2.92-2.92z",
            }
        }
    }
}

pub fn sick_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M23,7c0,1.1-0.9,2-2,2s-2-0.9-2-2c0-0.78,0.99-2.44,1.58-3.36c0.2-0.31,0.64-0.31,0.84,0C22.01,4.56,23,6.22,23,7z M21.86,10.38C21.94,10.91,22,11.45,22,12c0,5.52-4.48,10-10.01,10C6.47,22,2,17.52,2,12C2,6.48,6.47,2,11.99,2 c2.45,0,4.69,0.88,6.43,2.34C17.91,5.29,17.5,6.27,17.5,7c0,1.93,1.57,3.5,3.5,3.5C21.3,10.5,21.58,10.45,21.86,10.38z M14.03,10.03 l1.06,1.06c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06L15.62,9.5l0.53-0.53c0.29-0.29,0.29-0.77,0-1.06 s-0.77-0.29-1.06,0l-1.06,1.06C13.74,9.26,13.74,9.74,14.03,10.03z M8.38,9.5l-0.53,0.53c-0.29,0.29-0.29,0.77,0,1.06 c0.29,0.29,0.77,0.29,1.06,0l1.06-1.06c0.29-0.29,0.29-0.77,0-1.06L8.91,7.91c-0.29-0.29-0.77-0.29-1.06,0s-0.29,0.77,0,1.06 L8.38,9.5z M16.47,15.8c-1-1.39-2.62-2.3-4.47-2.3c-0.87,0-1.69,0.2-2.43,0.56L5.99,12c0-0.52-0.26-1.02-0.74-1.29 c-0.8-0.46-1.84-0.11-2.17,0.8c-0.21,0.57-0.03,1.25,0.44,1.64c0.52,0.44,1.2,0.45,1.72,0.16l2.97,1.72 c-0.25,0.24-0.48,0.5-0.68,0.78C7.17,16.3,7.53,17,8.15,17c0.23,0,0.46-0.1,0.6-0.3C9.47,15.68,10.65,15,12,15s2.53,0.68,3.25,1.7 c0.14,0.19,0.36,0.3,0.6,0.3h0C16.47,17,16.83,16.3,16.47,15.8z",
            }
        }
    }
}

pub fn sign_language_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14.88,12.05c0.4,0.38,0.62,0.9,0.62,1.45V17c0,1.1-0.9,2-2,2H5.12c-0.41,0-0.75-0.34-0.75-0.75 c0-0.41,0.34-0.75,0.75-0.75H8.5v-0.75H4c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75h4.5V14.5H3.25 c-0.41,0-0.75-0.34-0.75-0.75C2.5,13.34,2.84,13,3.25,13H8.5v-0.75H4.38c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75 h5.99l-0.7-1.39c-0.28-0.56-0.05-1.23,0.5-1.51c0.12-0.06,0.26-0.04,0.35,0.05L14.88,12.05z M9.84,6.34 C9.21,6.64,8.96,6.8,8.61,7.23L6.59,5.09C6.3,4.79,6.32,4.32,6.62,4.03c0.3-0.29,0.78-0.27,1.06,0.03L9.84,6.34z M8.23,7.91 C8.06,8.32,8.01,8.84,8.08,9.25H7.43l-1.7-1.79C5.45,7.16,5.46,6.68,5.76,6.4c0.3-0.29,0.78-0.27,1.06,0.03L8.23,7.91z M15.86,4.12 l-0.53,1.46L11.2,1.23c-0.29-0.3-0.76-0.31-1.06-0.03c-0.3,0.29-0.31,0.76-0.03,1.06l2.84,2.99l-0.54,0.52L8.79,1.97 c-0.29-0.3-0.76-0.31-1.06-0.03C7.43,2.22,7.42,2.7,7.7,3l2.83,2.99l4.7,4.68l0.46,0.44c0.21,0.2,0.39,0.43,0.55,0.68l0.62-0.58 c0.4-0.38,0.63-0.91,0.63-1.46V3.73c0-0.13-0.08-0.25-0.2-0.29C16.71,3.23,16.07,3.53,15.86,4.12z",
                }
            }
        }
    }
}

pub fn sign_language_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M12.49,13l-0.93-1.86c-0.37-0.74-0.07-1.64,0.67-2.01l0,0c0.16-0.08,0.34-0.05,0.47,0.07l5.53,5.26 c0.5,0.47,0.78,1.13,0.78,1.81v5.23c0,1.38-1.12,2.5-2.5,2.5h-11c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1H10v-1H4c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1h6v-1H3c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h7v-1H4.5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1H12.49z M11.78,7.12c-0.84,0.4-1.17,0.62-1.63,1.19l-2.7-2.85c-0.38-0.4-0.36-1.03,0.04-1.41c0.4-0.38,1.03-0.36,1.41,0.04L11.78,7.12z M9.64,9.21C9.41,9.76,9.35,10.45,9.44,11H8.58L6.31,8.61C5.93,8.21,5.94,7.58,6.35,7.2c0.4-0.38,1.03-0.36,1.41,0.04L9.64,9.21z M20.33,13.91l0.88-0.83c0.5-0.47,0.79-1.13,0.79-1.82V3.64c0-0.17-0.11-0.33-0.27-0.39l0,0c-0.78-0.28-1.64,0.12-1.92,0.9 L19.1,6.11l-5.5-5.8c-0.38-0.4-1.01-0.42-1.41-0.04c-0.4,0.38-0.42,1.01-0.04,1.41l3.79,3.99l-0.73,0.69l-4.82-5.08 c-0.38-0.4-1.01-0.42-1.41-0.04c-0.4,0.38-0.42,1.01-0.04,1.41l3.78,3.98L15.38,9l3.61,3.43l0.61,0.58 C19.89,13.28,20.13,13.58,20.33,13.91z",
                }
            }
        }
    }
}

pub fn single_bed_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15,8V6c0-0.55-0.45-1-1-1H6C5.45,5,5,5.45,5,6v2C4.45,8,4,8.45,4,9v4h1l0.75,2h0.5L7,13h6l0.75,2h0.5L15,13h1V9 C16,8.45,15.55,8,15,8z M9.5,8H6V6.5C6,6.22,6.22,6,6.5,6h3V8z M14,8h-3.5V6h3C13.78,6,14,6.22,14,6.5V8z",
                }
            }
        }
    }
}

pub fn single_bed_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M18,10V7c0-1.1-0.9-2-2-2H8C6.9,5,6,5.9,6,7v3c-1.1,0-2,0.9-2,2v5h1.33l0.51,1.53C5.94,18.81,6.2,19,6.5,19h0 c0.3,0,0.56-0.19,0.66-0.47L7.67,17h8.67l0.51,1.53C16.94,18.81,17.2,19,17.5,19l0,0c0.3,0,0.56-0.19,0.66-0.47L18.67,17H20v-5 C20,10.9,19.1,10,18,10z M11,10H8V8c0-0.55,0.45-1,1-1h2V10z M16,10h-3V7h2c0.55,0,1,0.45,1,1V10z",
                }
            }
        }
    }
}

pub fn skateboarding_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M7.49,19.25c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75s0.34-0.75,0.75-0.75S7.49,18.84,7.49,19.25z M13.26,18.5 c-0.41,0-0.75,0.34-0.75,0.75S12.85,20,13.26,20c0.41,0,0.75-0.34,0.75-0.75S13.67,18.5,13.26,18.5z M12.25,4.5 C13.21,4.5,14,3.72,14,2.75C14,1.78,13.22,1,12.25,1S10.5,1.78,10.5,2.75C10.5,3.72,11.29,4.5,12.25,4.5z M17,16.5 c0,0.12-0.04,0.24-0.13,0.34C16.46,17.29,15.58,18,14.26,18H5.74c-1.25,0-2.13-0.62-2.61-1.16C3.05,16.75,3,16.62,3,16.5 C3,16.24,3.2,16,3.5,16c0.14,0,0.28,0.06,0.38,0.17C4.34,16.68,5,17,5.74,17H6l2-3l-1.23-3.23C6.6,10.33,6.65,9.84,6.9,9.44 L8.73,6.5H6.8L5.7,8.22C5.47,8.59,4.97,8.69,4.61,8.43l0,0c-0.34-0.24-0.43-0.71-0.2-1.06L5.7,5.45C5.89,5.17,6.2,5,6.54,5h3.96 c0.68,0,1.11,0.43,1.3,0.76l0.75,1.27c0.6,1.01,1.63,1.74,2.83,1.92C15.74,9.01,16,9.32,16,9.68v0c0,0.46-0.41,0.82-0.87,0.75 c-1.58-0.25-2.94-1.18-3.77-2.48L10,10.13l3.06,2.07c0.27,0.19,0.44,0.5,0.44,0.83V17h0.76c0.74,0,1.4-0.32,1.86-0.83 c0.1-0.11,0.23-0.17,0.38-0.17C16.8,16,17,16.24,17,16.5z M12,13.4l-3-1.65l0.57,2c0.08,0.28,0.03,0.59-0.13,0.84L7.8,17H12V13.4z",
                }
            }
        }
    }
}

pub fn skateboarding_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13,3c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S13,4.1,13,3z M7.25,22.5c-0.41,0-0.75,0.34-0.75,0.75S6.84,24,7.25,24 S8,23.66,8,23.25S7.66,22.5,7.25,22.5z M15.75,22.5c-0.41,0-0.75,0.34-0.75,0.75S15.34,24,15.75,24s0.75-0.34,0.75-0.75 S16.16,22.5,15.75,22.5z M19.24,19c-0.24,0-0.45,0.11-0.59,0.3c-0.55,0.73-1.42,1.2-2.4,1.2H16v-4.88c0-0.69-0.36-1.34-0.95-1.7 l-3.37-2.08l1.8-2.89c0.96,1.53,2.54,2.64,4.39,2.96c0.6,0.11,1.13-0.39,1.13-0.99v0c0-0.48-0.35-0.89-0.83-0.98 c-1.49-0.28-2.72-1.29-3.3-2.64l-0.52-1.21C14.16,5.64,13.61,5,12.7,5H8.11c-0.69,0-1.33,0.36-1.7,0.94L5.03,8.15 C4.74,8.62,4.88,9.24,5.35,9.53l0,0c0.47,0.29,1.09,0.15,1.38-0.32L8.1,7h2.35l-2.4,3.84c-0.31,0.5-0.39,1.11-0.21,1.67l1.34,4.15 l-3.12,3.76c-0.7-0.16-1.3-0.57-1.71-1.12C4.21,19.11,3.99,19,3.75,19C3.31,19,3,19.36,3,19.75c0,0.15,0.05,0.31,0.15,0.45 c0.82,1.1,2.13,1.8,3.6,1.8h9.5c1.47,0,2.78-0.7,3.6-1.8c0.1-0.14,0.15-0.3,0.15-0.45C20,19.36,19.68,19,19.24,19z M14,20.5H8.6 l2.21-2.67c0.43-0.52,0.57-1.21,0.37-1.86L10.5,13.7l3.5,2.2V20.5z",
            }
        }
    }
}

pub fn sledding_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M8.25,3.75C8.25,2.78,9.03,2,10,2s1.75,0.78,1.75,1.75S10.97,5.5,10,5.5S8.25,4.72,8.25,3.75z M16.76,13.67 c0.1-0.3,0.45-0.44,0.72-0.28c1.36,0.77,1.96,2.52,1.16,4.03c-0.69,1.31-2.29,1.85-3.69,1.39L1.48,14.43 c-0.26-0.09-0.41-0.37-0.32-0.63c0.09-0.26,0.37-0.41,0.63-0.32l2.66,0.86l0.46-1.43l-2.66-0.87c-0.26-0.09-0.41-0.37-0.32-0.63 c0.09-0.26,0.37-0.41,0.63-0.32l0.95,0.31V8.66c0-0.4,0.24-0.76,0.61-0.92L7.9,6.13c0.32-0.14,0.68-0.17,1.04-0.06 c0.79,0.24,1.24,1.08,1,1.87L9.15,10.5l2.09-0.37c0.45-0.08,0.89,0.16,1.08,0.57l1.92,4.2l1.44,0.47c0.26,0.09,0.41,0.37,0.32,0.63 c-0.09,0.26-0.37,0.41-0.63,0.32l-2.38-0.77l-0.46,1.43l2.78,0.9c0.89,0.29,1.91-0.02,2.39-0.82c0.62-1.03,0.23-2.26-0.71-2.8 C16.78,14.14,16.68,13.9,16.76,13.67z M5,11.9l1.17,0.38c-0.31-0.38-0.43-0.89-0.28-1.39l0.8-2.62L5,9V11.9z M12.04,15.24 l-6.18-2.01l-0.46,1.43l6.18,2.01L12.04,15.24z M12.3,14.27l-1.12-2.46L7.7,12.78L12.3,14.27z",
            }
        }
    }
}

pub fn sledding_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14,4.5c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2S14,3.4,14,4.5z M18.92,19.24c-0.13,0.39-0.55,0.61-0.95,0.48l-2.61-0.85 L14.9,20.3l3.32,1.08l0,0c1.31,0.43,2.72-0.29,3.15-1.61c0.36-1.12-0.11-2.32-1.07-2.91c-0.32-0.2-0.45-0.6-0.3-0.95 c0.2-0.44,0.71-0.57,1.12-0.31c1.52,0.95,2.25,2.85,1.68,4.62c-0.68,2.1-2.94,3.25-5.04,2.57h0L1.74,17.6 c-0.39-0.13-0.63-0.54-0.52-0.93c0.12-0.41,0.55-0.63,0.95-0.5l3.22,1.05l0.46-1.43l-3.19-1.04c-0.39-0.13-0.63-0.54-0.52-0.93 c0.12-0.41,0.55-0.63,0.95-0.5L4,13.6v-2.78c0-0.8,0.48-1.52,1.21-1.84c0.75-0.32,4.11-1.76,4.26-1.83 c0.41-0.18,0.89-0.21,1.35-0.04c0.91,0.34,1.37,1.36,1.07,2.28l-1.04,3.2L13,12.25c0.89-0.15,1.76,0.32,2.14,1.14l2.08,4.51 l1.22,0.4C18.84,18.42,19.05,18.84,18.92,19.24z M6,14.25l1.01,0.33c-0.22-0.42-0.28-0.92-0.12-1.4L7.92,10L6,10.82V14.25z M13.94,18.41l-6.66-2.16l-0.46,1.43l6.66,2.16L13.94,18.41z M14.63,17.05l-1.18-2.56l-3.97,0.89L14.63,17.05z",
            }
        }
    }
}

pub fn snowboarding_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M12.75,4.5c0.96,0,1.75-0.78,1.75-1.75C14.5,1.78,13.72,1,12.75,1S11,1.78,11,2.75C11,3.72,11.79,4.5,12.75,4.5z M16.5,9.68c0-0.36-0.26-0.68-0.62-0.73c-1.2-0.19-2.24-0.91-2.83-1.92L12.3,5.76C12.11,5.43,11.68,5,11,5H7.04 C6.7,5,6.39,5.17,6.2,5.45L4.91,7.38c-0.23,0.35-0.14,0.81,0.2,1.06C5.47,8.69,5.97,8.59,6.2,8.22L7.3,6.5h1.93L7.48,9.32 c-0.3,0.48-0.38,1.07-0.22,1.61L8,13.5l-2.75,1.83l-0.66-0.14c-0.72-0.15-1.3-0.61-1.65-1.2c-0.07-0.12-0.19-0.21-0.34-0.24 c-0.29-0.06-0.54,0.13-0.59,0.38c-0.03,0.11,0,0.25,0.06,0.36c0.36,0.63,1.09,1.42,2.31,1.68l8.89,1.89L15,18.42 c1.29,0.27,2.3-0.23,2.79-0.59c0.11-0.08,0.17-0.19,0.2-0.31c0.05-0.25-0.09-0.53-0.39-0.59c-0.14-0.03-0.29,0-0.41,0.09 c-0.55,0.4-1.27,0.58-1.99,0.43L14,17.19l-0.93-4.52c-0.05-0.24-0.18-0.45-0.37-0.59l-2.3-1.77l1.46-2.35 c0.83,1.3,2.19,2.23,3.77,2.48C16.09,10.5,16.5,10.14,16.5,9.68z M7.3,15.76l1.91-1.26c0.33-0.22,0.5-0.61,0.43-1l-0.34-2 l2.34,1.65l0.76,3.7L7.3,15.76z",
                }
            }
        }
    }
}

pub fn snowboarding_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14,3c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S14,4.1,14,3z M6.35,9.53c0.47,0.29,1.09,0.15,1.38-0.32L9.1,7h2.35 l-2.51,3.99c-0.28,0.45-0.37,1-0.25,1.52L9.5,16L6,18.35l-0.47-0.1c-0.96-0.2-1.71-0.85-2.1-1.67c-0.1-0.21-0.28-0.37-0.51-0.42 c-0.43-0.09-0.82,0.2-0.9,0.58C1.98,16.88,2,17.05,2.07,17.2c0.58,1.24,1.71,2.2,3.15,2.51l12.63,2.69c1.44,0.31,2.86-0.11,3.9-1.01 c0.13-0.11,0.21-0.26,0.24-0.41c0.08-0.38-0.16-0.8-0.59-0.89c-0.23-0.05-0.46,0.02-0.64,0.17c-0.69,0.6-1.64,0.88-2.6,0.67 L17,20.69l-0.88-5.43c-0.08-0.49-0.34-0.93-0.72-1.24l-2.72-2.19l1.8-2.89c0.96,1.53,2.54,2.64,4.39,2.96c0.6,0.11,1.13-0.39,1.13-1 c0-0.48-0.35-0.89-0.83-0.98c-1.49-0.28-2.72-1.29-3.3-2.64l-0.52-1.21C15.16,5.64,14.61,5,13.7,5H9.11c-0.69,0-1.33,0.36-1.7,0.94 L6.03,8.15C5.74,8.62,5.88,9.24,6.35,9.53z M8.73,18.93l2.25-1.51c0.47-0.32,0.73-0.88,0.65-1.44l-0.32-2.4l2.84,2.02l0.75,4.64 L8.73,18.93z",
            }
        }
    }
}

pub fn snowshoeing_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10.25,3.75C10.25,2.78,11.03,2,12,2s1.75,0.78,1.75,1.75c0,0.97-0.79,1.75-1.75,1.75C11.04,5.5,10.25,4.72,10.25,3.75z M6.75,11c0.41,0,0.75-0.34,0.75-0.75V8.79l1.65-0.7L8.03,13.8l-1.85,1.34L5.29,14L4.5,14.61l1.82,2.32C6.92,17.71,7.81,18,8.03,18 c0.39,0,0.66-0.47,0.39-0.81c-0.07-0.08-0.16-0.14-0.26-0.17c-0.4-0.11-0.78-0.35-1.06-0.7l1.97-1.42c0.21-0.15,0.35-0.37,0.4-0.62 l0.45-2.3l1.58,1.58V17L10,17v1h3c0.67,0,1.3-0.22,1.8-0.6c0.14-0.11,0.2-0.25,0.2-0.4c0-0.25-0.19-0.5-0.5-0.5 c-0.11,0-0.21,0.04-0.3,0.1C13.87,16.85,13.45,17,13,17v-3.64c0-0.27-0.11-0.52-0.29-0.71l-1.17-1.17l0.49-2.48 c0.75,0.99,1.85,1.7,3.12,1.92C15.59,11,16,10.65,16,10.19c0-0.37-0.28-0.68-0.65-0.73c-2.46-0.28-2.43-2.2-3.46-3.03 c-0.57-0.46-1.35-0.57-2.03-0.28L6.61,7.53C6.24,7.69,6,8.05,6,8.46v1.79C6,10.66,6.34,11,6.75,11z",
            }
        }
    }
}

pub fn snowshoeing_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12.5,3.5c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S12.5,4.6,12.5,3.5z M20,11.91c0-0.49-0.36-0.9-0.84-0.98 c-1.53-0.25-2.79-1.16-3.47-2.35l-1-1.58c-0.4-0.6-1-1-1.7-1c-0.68,0-1.28,0.28-4.77,1.76C7.49,8.07,7,8.8,7,9.6V12 c0,0.55,0.45,1,1,1s1-0.45,1-1V9.58l1.79-0.7L9.2,17l-2.88,2.03l-1.14-1.47L4,18.5l2.38,3.04c0.51,0.65,1.16,1.15,1.88,1.41 c0.28,0.1,0.53,0.04,0.72-0.11c0.3-0.23,0.42-0.7,0.12-1.07c-0.08-0.1-0.2-0.17-0.31-0.22c-0.43-0.18-0.82-0.45-1.14-0.83l-0.08-0.1 l2.85-2.01c0.38-0.27,0.65-0.66,0.77-1.1l0.7-2.53l2.11,2v4.52h-2V23h3.87c0.82,0,1.61-0.21,2.26-0.61 c0.26-0.16,0.37-0.39,0.37-0.64c0-0.38-0.3-0.75-0.77-0.75c-0.13,0-0.26,0.04-0.37,0.1c-0.4,0.23-0.87,0.37-1.36,0.4l0-5.16 c0-0.55-0.23-1.07-0.62-1.45l-1.49-1.41l0.6-3c1.07,1.24,2.63,2.15,4.37,2.43C19.46,13.01,20,12.52,20,11.91z",
            }
        }
    }
}

pub fn social_distance_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M4,5c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S4,6.1,4,5z M8.78,8.58C7.93,8.21,6.99,8,6,8S4.07,8.21,3.22,8.58 C2.48,8.9,2,9.62,2,10.43L2,11h8l0-0.57C10,9.62,9.52,8.9,8.78,8.58z M18,7c1.1,0,2-0.9,2-2c0-1.1-0.9-2-2-2s-2,0.9-2,2 C16,6.1,16.9,7,18,7z M20.78,8.58C19.93,8.21,18.99,8,18,8c-0.99,0-1.93,0.21-2.78,0.58C14.48,8.9,14,9.62,14,10.43L14,11h8l0-0.57 C22,9.62,21.52,8.9,20.78,8.58z M21.65,16.65l-2.79-2.79C18.54,13.54,18,13.76,18,14.21V16H6v-1.79c0-0.45-0.54-0.67-0.85-0.35 l-2.79,2.79c-0.2,0.2-0.2,0.51,0,0.71l2.79,2.79C5.46,20.46,6,20.24,6,19.79V18h12v1.79c0,0.45,0.54,0.67,0.85,0.35l2.79-2.79 C21.84,17.16,21.84,16.84,21.65,16.65z",
            }
        }
    }
}

pub fn south_america_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10,2c-4.42,0-8,3.58-8,8s3.58,8,8,8s8-3.58,8-8S14.42,2,10,2z M3.5,10c0-1.54,0.54-2.95,1.43-4.07L7.5,8.5v1 C7.5,10.33,8.17,11,9,11v4.17c0,0.22,0.07,0.43,0.2,0.6l0.55,0.73C6.28,16.36,3.5,13.5,3.5,10z M10.5,16.48V15l3.32-4.74 C13.94,10.09,14,9.89,14,9.68V9c0-0.55-0.45-1-1-1h-1.2l-1.15-1.44C10.37,6.21,9.93,6,9.48,6H6.5V4.52C7.51,3.88,8.71,3.5,10,3.5 c3.58,0,6.5,2.92,6.5,6.5C16.5,13.42,13.85,16.22,10.5,16.48z",
            }
        }
    }
}

pub fn south_america_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M4,12c0-1.95,0.7-3.74,1.87-5.13L9,10 v1c0,1.1,0.9,2,2,2v5.59c0,0.27,0.11,0.52,0.29,0.71L12,20C7.58,20,4,16.42,4,12z M13,19.94V18l3.75-5.62 c0.16-0.25,0.25-0.54,0.25-0.83V10.5c0-0.55-0.45-1-1-1h-1.5l-1.4-1.75C12.72,7.28,12.15,7,11.54,7H8V5.07C9.18,4.39,10.54,4,12,4 c4.41,0,8,3.59,8,8C20,16.07,16.94,19.44,13,19.94z",
            }
        }
    }
}

pub fn sports_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M9.5,6C8.06,6,6.8,6.68,5.98,7.73C5.99,7.66,6,7.58,6,7.5C6,6.67,5.33,6,4.5,6S3,6.67,3,7.5S3.67,9,4.5,9 c0.33,0,0.62-0.11,0.87-0.28C5,9.57,4.88,10.56,5.13,11.59c0.4,1.66,1.77,2.98,3.45,3.32C11.47,15.48,14,13.29,14,10.5 c0-0.08-0.02-0.16-0.02-0.24c0-0.04,0.01-0.09,0.01-0.12C13.93,9.53,14.37,9,14.97,9h2.53C17.78,9,18,8.78,18,8.5v-2 C18,6.22,17.78,6,17.5,6H9.5z M4.5,8C4.22,8,4,7.78,4,7.5C4,7.22,4.22,7,4.5,7S5,7.22,5,7.5C5,7.78,4.78,8,4.5,8z M11.47,12.02 C11.02,12.61,10.31,13,9.5,13C8.12,13,7,11.88,7,10.5c0-1.27,0.95-2.3,2.17-2.47C9.28,8.02,9.39,8,9.5,8c1.38,0,2.5,1.12,2.5,2.5 C12,11.07,11.8,11.6,11.47,12.02z",
                        }
                    }
                    g {
                        circle {
                            r: "1.5",
                            cx: "9.5",
                            cy: "10.5",
                        }
                    }
                }
            }
        }
    }
}

pub fn sports_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M21,6h-9.77C9.57,6,8.01,6.66,6.87,7.73C6.54,6.73,5.61,6,4.5,6C3.12,6,2,7.12,2,8.5C2,9.88,3.12,11,4.5,11 c0.21,0,0.41-0.03,0.61-0.08c-0.05,0.25-0.09,0.51-0.1,0.78c-0.18,3.68,2.95,6.68,6.68,6.27c2.55-0.28,4.68-2.26,5.19-4.77 c0.15-0.71,0.15-1.4,0.06-2.06c-0.09-0.6,0.38-1.13,0.99-1.13l2.76,0C21.56,10,22,9.55,22,9V7C22,6.45,21.55,6,21,6z M4.5,9 C4.22,9,4,8.78,4,8.5C4,8.22,4.22,8,4.5,8S5,8.22,5,8.5C5,8.78,4.78,9,4.5,9z M11,15c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3 S12.66,15,11,15z",
                        }
                    }
                    g {
                        circle {
                            cx: "11",
                            r: "2",
                            cy: "12",
                        }
                    }
                }
            }
        }
    }
}

pub fn sports_baseball_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M15.22,5.34C13.34,6.06,12,7.87,12,10s1.34,3.94,3.22,4.66C16.32,13.42,17,11.79,17,10C17,8.21,16.32,6.58,15.22,5.34z",
                        }
                    }
                    g {
                        path {
                            d: "M11,10c0-2.39,1.41-4.45,3.43-5.42C13.23,3.59,11.68,3,10,3S6.77,3.59,5.57,4.58C7.59,5.55,9,7.61,9,10 s-1.41,4.45-3.43,5.42C6.77,16.41,8.32,17,10,17s3.23-0.59,4.43-1.58C12.41,14.45,11,12.39,11,10z",
                        }
                    }
                    g {
                        path {
                            d: "M4.78,5.34C3.68,6.58,3,8.21,3,10c0,1.79,0.68,3.42,1.78,4.66C6.66,13.94,8,12.13,8,10S6.66,6.06,4.78,5.34z",
                        }
                    }
                }
            }
        }
    }
}

pub fn sports_baseball_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M3.81,6.28C2.67,7.9,2,9.87,2,12s0.67,4.1,1.81,5.72C6.23,16.95,8,14.68,8,12S6.23,7.05,3.81,6.28z",
                        }
                    }
                    g {
                        path {
                            d: "M20.19,6.28C17.77,7.05,16,9.32,16,12s1.77,4.95,4.19,5.72C21.33,16.1,22,14.13,22,12S21.33,7.9,20.19,6.28z",
                        }
                    }
                    g {
                        path {
                            d: "M14,12c0-3.28,1.97-6.09,4.79-7.33C17.01,3.02,14.63,2,12,2S6.99,3.02,5.21,4.67C8.03,5.91,10,8.72,10,12 s-1.97,6.09-4.79,7.33C6.99,20.98,9.37,22,12,22s5.01-1.02,6.79-2.67C15.97,18.09,14,15.28,14,12z",
                        }
                    }
                }
            }
        }
    }
}

pub fn sports_basketball_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M13.05,9.5h3.92c-0.1-1.43-0.63-2.74-1.47-3.81C14.16,6.48,13.22,7.87,13.05,9.5z",
                        }
                    }
                    g {
                        path {
                            d: "M6.95,10.5H3.03c0.1,1.43,0.63,2.74,1.47,3.81C5.84,13.52,6.78,12.13,6.95,10.5z",
                        }
                    }
                    g {
                        path {
                            d: "M6.95,9.5c-0.17-1.63-1.1-3.02-2.46-3.81C3.66,6.76,3.13,8.07,3.03,9.5H6.95z",
                        }
                    }
                    g {
                        path {
                            d: "M13.05,10.5c0.17,1.63,1.1,3.02,2.46,3.81c0.83-1.07,1.36-2.38,1.47-3.81H13.05z",
                        }
                    }
                    g {
                        path {
                            d: "M12.03,10.5H10.5v6.47c1.67-0.12,3.17-0.82,4.31-1.9C13.26,14.1,12.18,12.43,12.03,10.5z",
                        }
                    }
                    g {
                        path {
                            d: "M12.03,9.5c0.16-1.93,1.23-3.6,2.79-4.58c-1.14-1.08-2.64-1.78-4.31-1.9V9.5H12.03z",
                        }
                    }
                    g {
                        path {
                            d: "M7.97,10.5c-0.16,1.93-1.23,3.6-2.79,4.58c1.14,1.08,2.64,1.78,4.31,1.9V10.5H7.97z",
                        }
                    }
                    g {
                        path {
                            d: "M7.97,9.5H9.5V3.03c-1.67,0.12-3.17,0.82-4.31,1.9C6.74,5.9,7.82,7.57,7.97,9.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn sports_basketball_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M17.09,11h4.86c-0.16-1.61-0.71-3.11-1.54-4.4C18.68,7.43,17.42,9.05,17.09,11z",
                        }
                    }
                    g {
                        path {
                            d: "M6.91,11C6.58,9.05,5.32,7.43,3.59,6.6C2.76,7.89,2.21,9.39,2.05,11H6.91z",
                        }
                    }
                    g {
                        path {
                            d: "M15.07,11c0.32-2.59,1.88-4.79,4.06-6c-1.6-1.63-3.74-2.71-6.13-2.95V11H15.07z",
                        }
                    }
                    g {
                        path {
                            d: "M8.93,11H11V2.05C8.61,2.29,6.46,3.37,4.87,5C7.05,6.21,8.61,8.41,8.93,11z",
                        }
                    }
                    g {
                        path {
                            d: "M15.07,13H13v8.95c2.39-0.24,4.54-1.32,6.13-2.95C16.95,17.79,15.39,15.59,15.07,13z",
                        }
                    }
                    g {
                        path {
                            d: "M3.59,17.4c1.72-0.83,2.99-2.46,3.32-4.4H2.05C2.21,14.61,2.76,16.11,3.59,17.4z",
                        }
                    }
                    g {
                        path {
                            d: "M17.09,13c0.33,1.95,1.59,3.57,3.32,4.4c0.83-1.29,1.38-2.79,1.54-4.4H17.09z",
                        }
                    }
                    g {
                        path {
                            d: "M8.93,13c-0.32,2.59-1.88,4.79-4.06,6c1.6,1.63,3.74,2.71,6.13,2.95V13H8.93z",
                        }
                    }
                }
            }
        }
    }
}

pub fn sports_cricket_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        r: "2.5",
                        cx: "14.5",
                        cy: "5.5",
                    }
                    path {
                        d: "M12.34,11.51L5.97,5.15c-0.2-0.2-0.51-0.2-0.71,0L3.15,7.26c-0.2,0.2-0.2,0.51,0,0.71l6.36,6.36c0.2,0.2,0.51,0.2,0.71,0 l2.12-2.12C12.53,12.02,12.53,11.71,12.34,11.51z",
                    }
                    path {
                        d: "M12.34,13.63l-0.71,0.71l2.47,2.47c0.2,0.2,0.51,0.2,0.71,0s0.2-0.51,0-0.71L12.34,13.63z",
                    }
                }
            }
        }
    }
}

pub fn sports_cricket_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M15.05,12.81L6.56,4.32c-0.39-0.39-1.02-0.39-1.41,0L2.32,7.15c-0.39,0.39-0.39,1.02,0,1.41l8.49,8.49 c0.39,0.39,1.02,0.39,1.41,0l2.83-2.83C15.44,13.83,15.44,13.2,15.05,12.81z",
                        }
                        path {
                            d: "M14.34,17.76l3.53,3.53c0.39,0.39,1.03,0.39,1.42,0l0,0c0.39-0.39,0.39-1.03,0-1.42l-3.53-3.53L14.34,17.76z",
                        }
                    }
                    circle {
                        cx: "18.5",
                        cy: "5.5",
                        r: "3.5",
                    }
                }
            }
        }
    }
}

pub fn sports_esports_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.82,13.55l-0.62-5.76C15.08,6.77,14.23,6,13.21,6H6.79C5.77,6,4.92,6.77,4.81,7.78l-0.62,5.76 C4.09,14.32,4.69,15,5.46,15c0.34,0,0.67-0.14,0.91-0.38L8,13h4l1.62,1.62c0.24,0.24,0.57,0.38,0.91,0.38 C15.31,15,15.91,14.32,15.82,13.55z M9.25,9.75H8V11H7.5V9.75H6.25v-0.5H7.5V8H8v1.25h1.25V9.75z M11.5,9C11.22,9,11,8.78,11,8.5 C11,8.22,11.22,8,11.5,8S12,8.22,12,8.5C12,8.78,11.78,9,11.5,9z M12.5,11c-0.28,0-0.5-0.22-0.5-0.5c0-0.28,0.22-0.5,0.5-0.5 s0.5,0.22,0.5,0.5C13,10.78,12.78,11,12.5,11z",
                    }
                }
            }
        }
    }
}

pub fn sports_esports_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M21.58,16.09l-1.09-7.66C20.21,6.46,18.52,5,16.53,5H7.47C5.48,5,3.79,6.46,3.51,8.43l-1.09,7.66 C2.2,17.63,3.39,19,4.94,19h0c0.68,0,1.32-0.27,1.8-0.75L9,16h6l2.25,2.25c0.48,0.48,1.13,0.75,1.8,0.75h0 C20.61,19,21.8,17.63,21.58,16.09z M11,11H9v2H8v-2H6v-1h2V8h1v2h2V11z M15,10c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C16,9.55,15.55,10,15,10z M17,13c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C18,12.55,17.55,13,17,13z",
                    }
                }
            }
        }
    }
}

pub fn sports_football_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.99,7.51c0.04-1.57-0.22-2.81-0.45-3.05c-0.23-0.23-1.47-0.5-3.05-0.45L15.99,7.51z",
                    }
                    path {
                        d: "M4.01,12.49c-0.04,1.57,0.22,2.81,0.45,3.05s1.47,0.5,3.05,0.45L4.01,12.49z",
                    }
                    path {
                        d: "M11.18,4.11C9.49,4.34,7.65,4.96,6.31,6.31c-1.34,1.34-1.97,3.19-2.19,4.88l4.71,4.71c1.69-0.23,3.53-0.85,4.88-2.19 s1.97-3.19,2.19-4.88L11.18,4.11z M12.12,8.59l-3.54,3.54c-0.2,0.2-0.51,0.2-0.71,0c-0.2-0.2-0.2-0.51,0-0.71l3.54-3.54 c0.2-0.2,0.51-0.2,0.71,0S12.32,8.39,12.12,8.59z",
                    }
                }
            }
        }
    }
}

pub fn sports_football_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M3.02,15.62c-0.08,2.42,0.32,4.34,0.67,4.69s2.28,0.76,4.69,0.67L3.02,15.62z",
                    }
                    path {
                        d: "M13.08,3.28C10.75,3.7,8.29,4.62,6.46,6.46s-2.76,4.29-3.18,6.62l7.63,7.63c2.34-0.41,4.79-1.34,6.62-3.18 s2.76-4.29,3.18-6.62L13.08,3.28z M14.8,10.6l-4.2,4.2c-0.39,0.39-1.01,0.39-1.4,0s-0.39-1.01,0-1.4l4.2-4.2 c0.39-0.39,1.01-0.39,1.4,0S15.19,10.21,14.8,10.6z",
                    }
                    path {
                        d: "M20.98,8.38c0.08-2.42-0.32-4.34-0.67-4.69s-2.28-0.76-4.69-0.67L20.98,8.38z",
                    }
                }
            }
        }
    }
}

pub fn sports_golf_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,3C7.24,3,5,5.24,5,8s2.24,5,5,5s5-2.24,5-5S12.76,3,10,3z M10,12c-2.21,0-4-1.79-4-4c0-2.21,1.79-4,4-4s4,1.79,4,4 C14,10.21,12.21,12,10,12z",
                    }
                    circle {
                        cy: "6.5",
                        r: ".5",
                        cx: "8.5",
                    }
                    circle {
                        cy: "6.5",
                        r: ".5",
                        cx: "11.5",
                    }
                    circle {
                        cx: "10",
                        cy: "5.5",
                        r: ".5",
                    }
                    path {
                        d: "M12.5,14h-5C7.22,14,7,14.22,7,14.5C7,14.78,7.22,15,7.5,15h1c0.55,0,1,0.45,1,1v1h1v-1c0-0.55,0.45-1,1-1h1 c0.28,0,0.5-0.22,0.5-0.5C13,14.22,12.78,14,12.5,14z",
                    }
                }
            }
        }
    }
}

pub fn sports_golf_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,16c3.87,0,7-3.13,7-7c0-3.87-3.13-7-7-7C8.13,2,5,5.13,5,9C5,12.87,8.13,16,12,16z M12,4c2.76,0,5,2.24,5,5 s-2.24,5-5,5s-5-2.24-5-5S9.24,4,12,4z",
                    }
                    circle {
                        r: "1",
                        cx: "10",
                        cy: "8",
                    }
                    circle {
                        cx: "14",
                        cy: "8",
                        r: "1",
                    }
                    circle {
                        cx: "12",
                        cy: "6",
                        r: "1",
                    }
                    path {
                        d: "M16,17H8c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h1c1.1,0,2,0.9,2,2v1h2v-1c0-1.1,0.9-2,2-2h1c0.55,0,1-0.45,1-1 C17,17.45,16.55,17,16,17z",
                    }
                }
            }
        }
    }
}

pub fn sports_gymnastics_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M11.95,4.52c0.29,0.33,0.23,0.84-0.13,1.1L9.2,7.5h2.3l5.93-3.2c0.33-0.18,0.74-0.1,0.98,0.18l0,0 c0.31,0.36,0.24,0.91-0.15,1.18L12,10l-0.45,7.25c-0.03,0.42-0.38,0.75-0.8,0.75c-0.42,0-0.77-0.33-0.8-0.75L9.5,10.3L7,9.5H1.75 C1.34,9.5,1,9.16,1,8.75C1,8.34,1.34,8,1.75,8H6l4.94-3.6C11.26,4.18,11.7,4.23,11.95,4.52z M4,5.5C4,6.33,4.67,7,5.5,7 S7,6.33,7,5.5S6.33,4,5.5,4S4,4.67,4,5.5z",
                }
            }
        }
    }
}

pub fn sports_gymnastics_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M4,6c0-1.1,0.9-2,2-2s2,0.9,2,2S7.1,8,6,8S4,7.1,4,6z M13,22c-0.56,0-1.02-0.44-1.05-1l-0.45-9L8,11H2c-0.55,0-1-0.45-1-1 s0.45-1,1-1h5l6.26-4.47c0.42-0.3,1-0.23,1.34,0.16c0.38,0.45,0.3,1.12-0.18,1.46L11.14,8.5H14l7.09-4.09 c0.41-0.24,0.93-0.15,1.24,0.21c0.36,0.43,0.3,1.07-0.14,1.41L14.5,12l-0.45,9C14.02,21.56,13.56,22,13,22z",
                }
            }
        }
    }
}

pub fn sports_handball_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M13.23,8.6l-3.46-2c-1.43-0.83-1.93-2.67-1.1-4.1l0.25-0.43c0.14-0.24,0.06-0.54-0.18-0.68C8.5,1.25,8.19,1.33,8.05,1.57 L7.8,2C6.7,3.92,7.35,6.36,9.27,7.47l-4.25,7.36c-0.14,0.24-0.06,0.54,0.18,0.68s0.54,0.06,0.68-0.18l1.25-2.17l1.73,1l-2.75,4.76 c-0.14,0.24-0.06,0.54,0.18,0.68s0.54,0.06,0.68-0.18l5.75-9.96c1.43,0.83,1.93,2.67,1.1,4.1L13.58,14 c-0.14,0.24-0.06,0.54,0.18,0.68c0.24,0.14,0.54,0.06,0.68-0.18l0.25-0.43C15.8,12.15,15.14,9.71,13.23,8.6z",
                    }
                    circle {
                        cx: "11",
                        r: "1",
                        cy: "2",
                    }
                    circle {
                        r: "1.5",
                        cx: "12.5",
                        cy: "5.5",
                    }
                }
            }
        }
    }
}

pub fn sports_handball_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M14.27,6C13.72,6.95,14.05,8.18,15,8.73c0.95,0.55,2.18,0.22,2.73-0.73c0.55-0.95,0.22-2.18-0.73-2.73 C16.05,4.72,14.82,5.05,14.27,6z",
                    }
                    path {
                        d: "M15.84,10.41c0,0-1.63-0.94-2.6-1.5c-2.13-1.24-3.01-3.83-2.18-6.07c0.17-0.46-0.01-0.97-0.43-1.21 C10.1,1.33,9.41,1.56,9.2,2.13C8.25,4.64,8.85,7.48,10.66,9.4l-4.65,8.05c-0.28,0.48-0.11,1.09,0.37,1.37 c0.48,0.28,1.09,0.11,1.37-0.37l1-1.73l1.73,1l-2.5,4.33c-0.28,0.48-0.11,1.09,0.37,1.37c0.48,0.28,1.09,0.11,1.37-0.37 l5.79-10.02c0.98,1.34,1.26,3.12,0.66,4.72c-0.17,0.45,0.02,0.96,0.43,1.2c0.53,0.31,1.22,0.08,1.44-0.5 C19.01,15.83,18.45,12.61,15.84,10.41z",
                    }
                    path {
                        d: "M12.75,3.8c0.72,0.41,1.63,0.17,2.05-0.55c0.41-0.72,0.17-1.63-0.55-2.05c-0.72-0.41-1.63-0.17-2.05,0.55 C11.79,2.47,12.03,3.39,12.75,3.8z",
                    }
                }
            }
        }
    }
}

pub fn sports_hockey_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M3,14v2h1v-3C3.45,13,3,13.45,3,14z",
                        }
                    }
                    g {
                        polygon {
                            points: "12.32,13 11.1,10.34 14,4 11.8,4 10,7.94 8.2,4 6,4 8.9,10.34 10,12.74 11.5,16 15,16 15,13",
                        }
                    }
                    g {
                        path {
                            d: "M16,13v3h1v-2C17,13.45,16.55,13,16,13z",
                        }
                    }
                    g {
                        polygon {
                            points: "7.68,13 5,13 5,16 8.5,16 9.45,13.94 8.35,11.54",
                        }
                    }
                }
            }
        }
    }
}

pub fn sports_hockey_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M2,17v3l2,0v-4H3C2.45,16,2,16.45,2,17z",
                    }
                    path {
                        d: "M9,16H5v4l4.69-0.01c0.38,0,0.72-0.21,0.89-0.55l0.87-1.9l-1.59-3.48L9,16z",
                    }
                    g {
                        path {
                            d: "M21.71,16.29C21.53,16.11,21.28,16,21,16h-1v4l2,0v-3C22,16.72,21.89,16.47,21.71,16.29z",
                        }
                    }
                    path {
                        d: "M13.6,12.84L17.65,4H14.3l-1.76,3.97l-0.49,1.1L12,9.21L9.7,4H6.35l4.05,8.84l1.52,3.32L12,16.34l1.42,3.1 c0.17,0.34,0.51,0.55,0.89,0.55L19,20v-4h-4L13.6,12.84z",
                    }
                }
            }
        }
    }
}

pub fn sports_kabaddi_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        r: "1.5",
                        cy: "2.5",
                        cx: "13.5",
                    }
                    path {
                        d: "M12,10.34c-0.81-0.06-1.53-0.32-2.17-0.78l0,0C9.6,9.4,9.38,9.22,9.17,9.01L7.29,7.12c-0.39-0.39-1.02-0.39-1.41,0 L3.07,9.91c-0.22,0.22-0.32,0.52-0.29,0.82l0.42,3.75l-2.75,2.75l0.71,0.71l3.25-3.25l0.04-2L6,14.03L6,18h1v-4.69l-1.96-2.05 l2.51-2.51l0.9,0.9c1.02,1.02,2.13,1.62,3.56,1.69V10.34z",
                    }
                    path {
                        d: "M9.5,6.99c0.82,0,1.5-0.68,1.5-1.5c0-0.77-0.59-1.38-1.34-1.47C9.61,4.01,9.56,3.99,9.5,3.99C8.68,3.99,8,4.66,8,5.49 C8,5.67,8.05,5.84,8.11,6C8.32,6.57,8.86,6.99,9.5,6.99z",
                    }
                    path {
                        d: "M19,9.53V6.18l-3.85-1.53c-0.18-0.08-0.37-0.11-0.56-0.11c-0.5,0-0.96,0.25-1.23,0.68l-0.82,1.32 C12,7.46,11.11,8.15,10.08,8.47c0.24,0.21,0.48,0.39,0.74,0.53c0.06,0.03,0.15,0.08,0.26,0.15c0.73-0.34,1.38-0.82,1.91-1.42 l0.44-0.51l0.67,3.33L12,12.24V18h1v-4.8l2.59-2.16L17.16,18h1.08L15.87,6.09L18,6.81v2.72H19z",
                    }
                }
            }
        }
    }
}

pub fn sports_kabaddi_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    circle {
                        r: "2",
                        cx: "16.5",
                        cy: "2.38",
                    }
                    path {
                        d: "M24,10.88v-3.7l-4.99-2.11c-0.98-0.41-2.12-0.07-2.71,0.81l0,0l-1,1.6c-0.67,1.18-1.91,2.06-3.41,2.32l0.06,0.06 c0.69,0.69,1.52,1.07,2.46,1.17c0.8-0.42,1.52-0.98,2.09-1.64l0.6,3l-1.16,1.1L15,14.38v0.76v5.74c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-5l2.1-2l1.62,7.19c0.11,0.47,0.53,0.81,1.02,0.81h0c0.66,0,1.15-0.6,1.02-1.24l-1.94-9.76l-0.62-3.1l1.8,0.7 v2.4c0,0.55,0.45,1,1,1h0C23.55,11.88,24,11.43,24,10.88z",
                    }
                    path {
                        d: "M10.29,8.09c0.22,0.15,0.47,0.24,0.72,0.29c0.13,0.02,0.25,0.04,0.38,0.04s0.26-0.01,0.38-0.04 c0.13-0.02,0.25-0.06,0.37-0.11c0.24-0.1,0.47-0.24,0.66-0.44c0.49-0.49,0.67-1.17,0.55-1.8C13.28,5.66,13.1,5.29,12.8,5 c-0.19-0.19-0.42-0.34-0.66-0.44c-0.12-0.05-0.24-0.09-0.37-0.11s-0.25-0.04-0.38-0.04c-0.12,0-0.23,0.01-0.35,0.03 c-0.14,0.02-0.28,0.06-0.41,0.11C10.4,4.66,10.17,4.81,9.98,5C9.68,5.29,9.5,5.66,9.43,6.03c-0.12,0.63,0.06,1.31,0.55,1.8 C10.07,7.93,10.18,8.01,10.29,8.09z",
                    }
                    path {
                        d: "M11.24,10.56l-2-2c-0.1-0.1-0.2-0.18-0.31-0.26C8.71,8.16,8.46,8.06,8.21,8.02C8.08,7.99,7.96,7.98,7.83,7.98 c-0.51,0-1.02,0.2-1.41,0.59l-3.34,3.34c-0.41,0.41-0.62,0.98-0.58,1.54C2.5,13.63,2.54,13.82,2.61,14l1.07,2.95l-2.92,2.92 c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0l3.54-3.54v-2.22L7,16.75v4.13c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-5l-2.12-2.12l2.36-2.36l0.71,0.71l0,0c1.02,1,2.28,1.69,3.79,1.94c0.64,0.11,1.21-0.45,1.16-1.1v0 c-0.03-0.48-0.4-0.87-0.87-0.94C12.9,11.83,11.97,11.29,11.24,10.56z",
                    }
                }
            }
        }
    }
}

pub fn sports_martial_arts_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cy: "4.5",
                        r: "1.5",
                        cx: "4.5",
                    }
                    path {
                        d: "M16.42,2.3L9.96,7.54L8.27,6.22l2.17-1.25c0.28-0.16,0.34-0.55,0.1-0.79L7.73,1.35c-0.2-0.2-0.51-0.2-0.71,0l0,0 c-0.2,0.2-0.2,0.51,0,0.71l2.36,2.36L4.43,7.28c-0.11,0.07-0.2,0.17-0.23,0.3l-0.83,3c-0.04,0.13-0.02,0.26,0.05,0.38l1.47,2.61 c0.14,0.24,0.44,0.33,0.68,0.19l0,0c0.24-0.14,0.33-0.44,0.19-0.68l-1.37-2.43l0.49-1.78L8.5,11l0.46,6.51 C8.98,17.78,9.22,18,9.49,18H9.5c0.28,0,0.51-0.22,0.53-0.5L10.5,10l6.64-6.92c0.2-0.21,0.2-0.54-0.01-0.74v0 C16.94,2.15,16.63,2.13,16.42,2.3z",
                    }
                }
            }
        }
    }
}

pub fn sports_martial_arts_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M19.06,2.6L11.6,8.7l-1.21-1.04l2.48-1.43c0.57-0.33,0.67-1.11,0.21-1.57l-2.95-2.95c-0.39-0.39-1.02-0.39-1.41,0l0,0 c-0.39,0.39-0.39,1.02,0,1.41l2.03,2.03L5.35,8.26c-0.23,0.13-0.39,0.35-0.46,0.6l-0.96,3.49c-0.07,0.26-0.04,0.53,0.1,0.77 l1.74,3.02c0.28,0.48,0.89,0.64,1.37,0.37h0c0.48-0.28,0.64-0.89,0.37-1.37l-1.53-2.66l0.36-1.29L9.5,13l0.44,8 c0.03,0.56,0.49,1,1.05,1h0c0.56,0,1.02-0.44,1.05-1l0.45-9l7.87-7.96c0.36-0.36,0.38-0.93,0.05-1.32l0,0 C20.07,2.32,19.47,2.27,19.06,2.6z",
                    }
                    circle {
                        cy: "5",
                        cx: "5",
                        r: "2",
                    }
                }
            }
        }
    }
}

pub fn sports_mma_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M6,15.5C6,15.78,6.22,16,6.5,16h7c0.28,0,0.5-0.22,0.5-0.5V14H6V15.5z",
                    }
                    path {
                        d: "M14,7c-0.55,0-1,0.45-1,1l-0.01-3c0-0.55-0.45-1-1-1H6C5.45,4,5,4.45,5,5v4.57c0.01,0.07,0.01,0.13,0.02,0.2l0.57,2.83 C5.63,12.83,5.84,13,6.08,13h7.85c0.23,0,0.45-0.18,0.49-0.4l0.57-2.83c0.01-0.07,0.01-0.13,0.02-0.2V8C15,7.45,14.55,7,14,7z M10.5,9h-3C7.22,9,7,8.78,7,8.5v-1C7,7.22,7.22,7,7.5,7h3C10.78,7,11,7.22,11,7.5v1C11,8.78,10.78,9,10.5,9z",
                    }
                }
            }
        }
    }
}

pub fn sports_mma_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M7,20c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1v-3H7V20z",
                    }
                    path {
                        d: "M18,7c-0.55,0-1,0.45-1,1V5c0-1.1-0.9-2-2-2H7C5.9,3,5,3.9,5,5v5.8c0,0.13,0.01,0.26,0.04,0.39l0.8,4 c0.09,0.47,0.5,0.8,0.98,0.8h10.36c0.45,0,0.89-0.36,0.98-0.8l0.8-4C18.99,11.06,19,10.93,19,10.8V8C19,7.45,18.55,7,18,7z M14,10 H8c-0.55,0-1-0.45-1-1V8c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1v1C15,9.55,14.55,10,14,10z",
                    }
                }
            }
        }
    }
}

pub fn sports_motorsports_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M9.39,12c0.43,0,0.83-0.17,1.14-0.47c0.39-0.39,0.55-0.92,0.44-1.45C10.86,9.54,10.51,9.11,10,8.91L5.81,7.18 C4.35,8.65,3.64,10.56,3.31,12H9.39z",
                    }
                    path {
                        d: "M10.69,5.02C9,5.15,7.7,5.7,6.67,6.45l3.71,1.53c1.72,0.71,2.17,2.94,0.85,4.25C10.75,12.73,10.08,13,9.39,13H3.12 C3.02,13.65,3,14.07,3,14.07V15c0,0.55,0.45,1,1,1h7.5c3.13,0,5.65-2.62,5.49-5.79C16.83,7.04,13.85,4.78,10.69,5.02z",
                    }
                }
            }
        }
    }
}

pub fn sports_motorsports_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M12,11.39c0-0.65-0.39-1.23-0.98-1.48L5.44,7.55c-1.48,1.68-2.32,3.7-2.8,5.45h7.75C11.28,13,12,12.28,12,11.39z",
                    }
                    path {
                        d: "M21.96,11.22c-0.41-4.41-4.56-7.49-8.98-7.2c-2.51,0.16-4.44,0.94-5.93,2.04l4.74,2.01c1.33,0.57,2.2,1.87,2.2,3.32 c0,1.99-1.62,3.61-3.61,3.61H2.21C2,16.31,2,17.2,2,17.2V18c0,1.1,0.9,2,2,2h10C18.67,20,22.41,15.99,21.96,11.22z",
                    }
                }
            }
        }
    }
}

pub fn sports_rugby_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.3,4.7C15.3,4.7,15.3,4.7,15.3,4.7C15.3,4.7,15.3,4.7,15.3,4.7c-0.35-0.35-1.34-0.6-2.6-0.6 c-1.93,0-4.47,0.6-6.24,2.37C3.54,9.39,3.81,14.42,4.7,15.3c0.35,0.35,1.34,0.6,2.6,0.6c1.93,0,4.47-0.6,6.24-2.37 C16.46,10.61,16.19,5.58,15.3,4.7z M7.17,7.17C8.79,5.55,11,5.19,12.16,5.12C10.8,5.64,9.23,6.53,7.88,7.88 c-1.35,1.35-2.24,2.92-2.76,4.28C5.22,10.58,5.75,8.6,7.17,7.17z M12.83,12.83c-1.62,1.62-3.83,1.98-4.99,2.06 c1.35-0.52,2.93-1.41,4.28-2.76c1.35-1.35,2.24-2.92,2.76-4.28C14.78,9.42,14.25,11.4,12.83,12.83z",
                    }
                }
            }
        }
    }
}

pub fn sports_rugby_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M20.49,3.51c-0.56-0.56-2.15-0.97-4.16-0.97c-3.08,0-7.15,0.96-9.98,3.79C1.66,11.03,2.1,19.07,3.51,20.49 c0.56,0.56,2.15,0.97,4.16,0.97c3.08,0,7.15-0.96,9.98-3.79C22.34,12.97,21.9,4.93,20.49,3.51z M7.76,7.76 c2.64-2.64,6.35-3.12,8.03-3.19c-2.05,0.94-4.46,2.45-6.61,4.61c-2.16,2.16-3.67,4.58-4.61,6.63C4.66,13.33,5.44,10.07,7.76,7.76z M16.24,16.24c-2.64,2.64-6.35,3.12-8.03,3.19c2.05-0.94,4.46-2.45,6.61-4.61c2.16-2.16,3.67-4.58,4.62-6.63 C19.34,10.67,18.56,13.93,16.24,16.24z",
                    }
                }
            }
        }
    }
}

pub fn sports_soccer_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C17,6.13,13.87,3,10,3z M10.5,5.45l1.55-1.08 c1.14,0.41,2.11,1.16,2.81,2.12l-0.48,1.67l-0.68,0.23L10.5,6.15V5.45z M13.01,9.13l-1.14,3.37H8.12L6.99,9.13L10,7.02L13.01,9.13 z M7.95,4.37L9.5,5.45v0.7L6.31,8.39L5.63,8.15L5.15,6.49C5.84,5.53,6.82,4.78,7.95,4.37z M6.8,13.55l-1.52,0.13 c-0.79-1-1.26-2.26-1.27-3.63l1.3-0.95L6,9.34l1.19,3.53L6.8,13.55z M11.53,15.8c-0.49,0.13-1,0.2-1.53,0.2s-1.04-0.08-1.53-0.2 l-0.81-1.74l0.32-0.55h4.04l0.32,0.55L11.53,15.8z M14.72,13.69l-1.52-0.13l-0.4-0.68L14,9.34l0.69-0.24l1.3,0.95 C15.98,11.42,15.51,12.68,14.72,13.69z",
                    }
                }
            }
        }
    }
}

pub fn sports_soccer_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M13,5.3l1.35-0.95 c1.82,0.56,3.37,1.76,4.38,3.34l-0.39,1.34l-1.35,0.46L13,6.7V5.3z M9.65,4.35L11,5.3v1.4L7.01,9.49L5.66,9.03L5.27,7.69 C6.28,6.12,7.83,4.92,9.65,4.35z M7.08,17.11l-1.14,0.1C4.73,15.81,4,13.99,4,12c0-0.12,0.01-0.23,0.02-0.35l1-0.73L6.4,11.4 l1.46,4.34L7.08,17.11z M14.5,19.59C13.71,19.85,12.87,20,12,20s-1.71-0.15-2.5-0.41l-0.69-1.49L9.45,17h5.11l0.64,1.11 L14.5,19.59z M14.27,15H9.73l-1.35-4.02L12,8.44l3.63,2.54L14.27,15z M18.06,17.21l-1.14-0.1l-0.79-1.37l1.46-4.34l1.39-0.47 l1,0.73C19.99,11.77,20,11.88,20,12C20,13.99,19.27,15.81,18.06,17.21z",
                    }
                }
            }
        }
    }
}

pub fn sports_tennis_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14.09,10.28c2.2-2.2,2.55-5.4,0.8-7.16c-1.76-1.76-4.96-1.4-7.16,0.8C6.4,5.24,5.75,6.93,5.82,8.44l0,0 c-0.04,0.82,0.29,2.73-0.98,4.01L2.28,15c-0.2,0.2-0.2,0.52,0,0.72s0.52,0.2,0.72,0l2.56-2.56c1.27-1.27,3.19-0.94,4.01-0.98l0,0 C11.07,12.25,12.76,11.6,14.09,10.28z M7.64,10.36C6.27,9,6.63,6.42,8.43,4.62s4.38-2.16,5.75-0.8c1.36,1.36,1.01,3.94-0.8,5.75 S9,11.73,7.64,10.36z",
                    }
                    path {
                        d: "M16.12,12.88C15.54,12.29,14.77,12,14,12s-1.54,0.29-2.12,0.88c-1.17,1.17-1.17,3.07,0,4.24C12.46,17.71,13.23,18,14,18 s1.54-0.29,2.12-0.88C17.29,15.95,17.29,14.05,16.12,12.88z M15.41,16.41C15.04,16.79,14.53,17,14,17s-1.04-0.21-1.41-0.59 c-0.78-0.78-0.78-2.05,0-2.83C12.96,13.21,13.47,13,14,13s1.04,0.21,1.41,0.59C16.19,14.37,16.19,15.63,15.41,16.41z",
                    }
                }
            }
        }
    }
}

pub fn sports_tennis_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M19.52,2.49c-2.34-2.34-6.62-1.87-9.55,1.06c-1.6,1.6-2.52,3.87-2.54,5.46c-0.02,1.58,0.26,3.89-1.35,5.5l-3.54,3.53 c-0.39,0.39-0.39,1.02,0,1.42l0,0c0.39,0.39,1.02,0.39,1.42,0l3.53-3.54c1.61-1.61,3.92-1.33,5.5-1.35s3.86-0.94,5.46-2.54 C21.38,9.11,21.86,4.83,19.52,2.49z M10.32,11.68c-1.53-1.53-1.05-4.61,1.06-6.72s5.18-2.59,6.72-1.06 c1.53,1.53,1.05,4.61-1.06,6.72S11.86,13.21,10.32,11.68z",
                }
                path {
                    d: "M18,17c0.53,0,1.04,0.21,1.41,0.59c0.78,0.78,0.78,2.05,0,2.83C19.04,20.79,18.53,21,18,21s-1.04-0.21-1.41-0.59 c-0.78-0.78-0.78-2.05,0-2.83C16.96,17.21,17.47,17,18,17 M18,15c-1.02,0-2.05,0.39-2.83,1.17c-1.56,1.56-1.56,4.09,0,5.66 C15.95,22.61,16.98,23,18,23s2.05-0.39,2.83-1.17c1.56-1.56,1.56-4.09,0-5.66C20.05,15.39,19.02,15,18,15L18,15z",
                }
            }
        }
    }
}

pub fn sports_volleyball_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M7.03,16.33C7.93,16.76,8.94,17,10,17c2.41,0,4.54-1.22,5.8-3.08l-2.3-1.33L7.03,16.33z",
                    }
                    path {
                        d: "M9.5,9.71V3.03C8.61,3.09,7.77,3.32,7,3.68v7.47L9.5,9.71z",
                    }
                    path {
                        d: "M10,10.58l-5.8,3.35c0.49,0.72,1.11,1.34,1.82,1.83l6.47-3.74L10,10.58z",
                    }
                    path {
                        d: "M6,4.26C4.19,5.53,3,7.62,3,10c0,1.1,0.26,2.13,0.71,3.05L6,11.73V4.26z",
                    }
                    path {
                        d: "M10.5,6.82v2.89l5.79,3.34c0.37-0.76,0.61-1.6,0.68-2.49L10.5,6.82z",
                    }
                    path {
                        d: "M16.97,9.41c-0.29-3.42-3.03-6.14-6.47-6.39v2.64L16.97,9.41z",
                    }
                }
            }
        }
    }
}

pub fn sports_volleyball_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M6,4.01C3.58,5.84,2,8.73,2,12c0,1.46,0.32,2.85,0.89,4.11L6,14.31V4.01z",
                    }
                    path {
                        d: "M11,11.42V2.05C9.94,2.16,8.93,2.43,8,2.84v10.32L11,11.42z",
                    }
                    path {
                        d: "M12,13.15l-8.11,4.68c0.61,0.84,1.34,1.59,2.18,2.2L15,14.89L12,13.15z",
                    }
                    path {
                        d: "M13,7.96v3.46l8.11,4.68c0.42-0.93,0.7-1.93,0.82-2.98L13,7.96z",
                    }
                    path {
                        d: "M8.07,21.2C9.28,21.71,10.6,22,12,22c3.34,0,6.29-1.65,8.11-4.16L17,16.04L8.07,21.2z",
                    }
                    path {
                        d: "M21.92,10.81c-0.55-4.63-4.26-8.3-8.92-8.76v3.6L21.92,10.81z",
                    }
                }
            }
        }
    }
}

pub fn surfing_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M15.5,3.25C15.5,4.22,14.72,5,13.75,5S12,4.22,12,3.25s0.78-1.75,1.75-1.75S15.5,2.28,15.5,3.25z M16.5,9.18 c0,0.46-0.42,0.83-0.87,0.75C14.35,9.7,13.25,8.99,12.5,8l-0.6-0.8L10,8.48l2.64,2.22c0.23,0.19,0.36,0.47,0.36,0.77v2.62 c0.5,0.35,0.98,0.71,1.41,1.07c-0.54,0.5-1.2,0.84-1.91,0.84c-0.82,0-1.56-0.44-2.14-1.06c-0.2-0.21-0.53-0.21-0.72,0 C9.06,15.56,8.32,16,7.5,16c-0.19,0-0.37-0.02-0.55-0.07C4.71,14.38,3,12.66,3,11.52c0-0.45,0.28-1.02,1.47-1.02 c0.68,0,1.56,0.19,2.53,0.51L6.53,8.28c-0.1-0.57,0.13-1.16,0.63-1.5l1.72-1.16L7.54,5.38L5.86,6.52c-0.34,0.23-0.81,0.14-1.04-0.2 c-0.23-0.34-0.14-0.81,0.2-1.04l1.85-1.26c0.21-0.14,0.47-0.2,0.72-0.16l3.83,0.62c0.36,0.06,0.69,0.23,0.91,0.51l1.38,2.1 c0.52,0.69,1.29,1.19,2.18,1.35C16.24,8.51,16.5,8.81,16.5,9.18z M8.4,9.15l0.35,2.55c0.86,0.37,2.18,1.05,2.74,1.42V11.7L8.4,9.15z M5,18.4c-0.7,0.36-1.46,0.55-2.22,0.59C2.39,19.01,2,18.7,2,18.24c0-0.39,0.3-0.72,0.71-0.75c0.68-0.04,1.36-0.29,1.98-0.75 c0.18-0.14,0.43-0.14,0.61,0c1.37,1.01,3.02,1.01,4.39,0c0.19-0.14,0.43-0.14,0.61,0c1.37,1.01,3.02,1,4.39,0 c0.18-0.14,0.43-0.14,0.61,0c0.62,0.46,1.3,0.7,1.98,0.75C17.7,17.52,18,17.85,18,18.24c0,0.46-0.39,0.77-0.78,0.75 c-0.76-0.04-1.52-0.24-2.22-0.59c-1.57,0.8-3.43,0.8-5,0C8.43,19.2,6.57,19.2,5,18.4z",
            }
        }
    }
}

pub fn surfing_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17,1.5c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S18.1,1.5,17,1.5z M22,22c0,0.55-0.45,1-1,1h0c-1.03,0-2.05-0.25-3-0.75 c-1.92,1.02-4.18,1-6.09-0.05c-1.79,0.87-3.92,0.98-5.58-0.14C5.3,22.69,4.15,23,3,23h0c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h0 c0.87,0,1.73-0.24,2.53-0.7c0.29-0.16,0.65-0.17,0.94,0c1.59,0.9,3.48,0.9,5.06,0c0.29-0.16,0.65-0.16,0.94,0 c1.59,0.9,3.48,0.9,5.06,0c0.29-0.16,0.65-0.16,0.94,0C19.27,20.76,20.13,21,21,21h0C21.55,21,22,21.45,22,22z M8.04,18.86 C8.35,18.95,8.67,19,9,19c0.9,0,1.72-0.37,2.39-0.91c0.35-0.28,0.87-0.28,1.22,0C13.28,18.63,14.1,19,15,19s1.72-0.37,2.39-0.91 c0.03-0.03,0.07-0.05,0.11-0.07c-0.46-0.39-0.97-0.79-1.5-1.17v-2.87c0-0.61-0.28-1.19-0.77-1.57L12.17,10l2.25-1.52 c1.03,1.79,2.82,3.08,4.93,3.43c0.6,0.1,1.14-0.39,1.14-1c0-0.49-0.36-0.9-0.84-0.98c-1.5-0.25-2.78-1.18-3.51-2.46l-0.88-1.55 c-0.29-0.52-0.77-0.8-1.22-0.89L9.31,4.15c-0.52-0.1-1.06,0.02-1.5,0.32L5.82,5.83C5.37,6.15,5.25,6.77,5.56,7.22 C5.88,7.68,6.5,7.8,6.96,7.49l1.99-1.37l2,0.37L8.8,7.94C8.2,8.35,7.91,9.06,8.03,9.73l0.52,3.1C7.13,12.31,5.83,12,4.85,12 C4.01,12,3,12.25,3,13.28C3,14.72,5.19,16.9,8.04,18.86z M14,14v1.5c-0.78-0.48-2.34-1.33-3.26-1.75L10.3,11.1L14,14z",
            }
        }
    }
}

pub fn switch_account_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,20H4V7c0-0.55-0.45-1-1-1S2,6.45,2,7v13c0,1.1,0.9,2,2,2h13c0.55,0,1-0.45,1-1S17.55,20,17,20z M20,2H8 C6.9,2,6,2.9,6,4v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M14,5c1.66,0,3,1.34,3,3c0,1.66-1.34,3-3,3 s-3-1.34-3-3C11,6.34,12.34,5,14,5z M7.76,16c1.47-1.83,3.71-3,6.24-3s4.77,1.17,6.24,3H7.76z",
                }
            }
        }
    }
}

pub fn thumb_down_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                opacity: ".87",
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M10.88 21.94l5.53-5.54c.37-.37.58-.88.58-1.41V5c0-1.1-.9-2-2-2H6c-.8 0-1.52.48-1.83 1.21L.91 11.82C.06 13.8 1.51 16 3.66 16h5.65l-.95 4.58c-.1.5.05 1.01.41 1.37.59.58 1.53.58 2.11-.01zM21 3c-1.1 0-2 .9-2 2v8c0 1.1.9 2 2 2s2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn thumb_up_alt_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                opacity: ".87",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M13.12 2.06L7.58 7.6c-.37.37-.58.88-.58 1.41V19c0 1.1.9 2 2 2h9c.8 0 1.52-.48 1.84-1.21l3.26-7.61C23.94 10.2 22.49 8 20.34 8h-5.65l.95-4.58c.1-.5-.05-1.01-.41-1.37-.59-.58-1.53-.58-2.11.01zM3 21c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2s-2 .9-2 2v8c0 1.1.9 2 2 2z",
            }
        }
    }
}

pub fn thunderstorm_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14.69,6.01c-0.35-2.23-2.25-3.95-4.58-4C10.07,2,10.04,2,10,2C8.19,2,6.6,3.02,5.8,4.52C3.71,4.74,2.05,6.49,2,8.65 C1.95,11,3.81,12.94,6.15,13l8.35,0c1.93,0,3.5-1.57,3.5-3.5C18,7.63,16.54,6.1,14.69,6.01z",
                    }
                    path {
                        d: "M8.5,16.74l-1.29-0.47l1.11-0.96c0.31-0.27,0.35-0.74,0.08-1.06c-0.27-0.31-0.74-0.35-1.06-0.08l-2.08,1.8 c-0.2,0.17-0.3,0.44-0.25,0.7s0.23,0.48,0.48,0.57l1.29,0.47l-1.11,0.96c-0.31,0.27-0.35,0.74-0.08,1.06 C5.75,19.91,5.96,20,6.17,20c0.17,0,0.35-0.06,0.49-0.18l2.08-1.8c0.2-0.17,0.3-0.44,0.25-0.7S8.75,16.83,8.5,16.74z",
                    }
                    path {
                        d: "M13.5,16.74l-1.29-0.47l1.11-0.96c0.31-0.27,0.35-0.74,0.08-1.06c-0.27-0.31-0.75-0.35-1.06-0.08l-2.08,1.8 c-0.2,0.17-0.3,0.44-0.25,0.7s0.23,0.48,0.48,0.57l1.29,0.47l-1.11,0.96c-0.31,0.27-0.35,0.74-0.08,1.06 c0.15,0.17,0.36,0.26,0.57,0.26c0.17,0,0.35-0.06,0.49-0.18l2.08-1.8c0.2-0.17,0.3-0.44,0.25-0.7S13.75,16.83,13.5,16.74z",
                    }
                }
            }
        }
    }
}

pub fn thunderstorm_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M17.92,7.02C17.45,4.18,14.97,2,12,2C9.82,2,7.83,3.18,6.78,5.06C4.09,5.41,2,7.74,2,10.5C2,13.53,4.47,16,7.5,16h10 c2.48,0,4.5-2.02,4.5-4.5C22,9.16,20.21,7.23,17.92,7.02z",
                    }
                    path {
                        d: "M15.95,20.11l-0.84-0.42l0.9-1.03c0.36-0.42,0.32-1.05-0.09-1.41c-0.42-0.36-1.05-0.32-1.41,0.09l-1.75,2 c-0.2,0.23-0.29,0.55-0.23,0.85c0.06,0.3,0.26,0.56,0.53,0.7l0.84,0.42L13,22.34c-0.36,0.42-0.32,1.05,0.09,1.41 c0.19,0.17,0.42,0.25,0.66,0.25c0.28,0,0.55-0.12,0.75-0.34l1.75-2c0.2-0.23,0.29-0.55,0.23-0.85 C16.42,20.5,16.22,20.24,15.95,20.11z",
                    }
                    path {
                        d: "M9.95,20.11L9.1,19.68l0.9-1.03c0.36-0.42,0.32-1.05-0.09-1.41c-0.42-0.36-1.05-0.32-1.41,0.09l-1.75,2 c-0.2,0.23-0.29,0.55-0.23,0.85c0.06,0.3,0.26,0.56,0.53,0.7l0.84,0.42L7,22.34c-0.36,0.42-0.32,1.05,0.09,1.41 C7.28,23.92,7.52,24,7.75,24c0.28,0,0.55-0.12,0.75-0.34l1.75-2c0.2-0.23,0.29-0.55,0.23-0.85C10.42,20.5,10.22,20.24,9.95,20.11z",
                    }
                }
            }
        }
    }
}

pub fn tornado_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        path {
                            d: "M16.6,7l1.04-1.73c0.6-1-0.12-2.27-1.29-2.27H3.65C2.48,3,1.76,4.27,2.36,5.27L3.4,7H16.6z",
                        }
                    }
                    g {
                        path {
                            d: "M6.7,12.5l2.01,3.36c0.58,0.97,1.99,0.97,2.57,0l2.01-3.36H6.7z",
                        }
                    }
                    g {
                        polygon {
                            points: "15.7,8.5 4.3,8.5 5.8,11 14.2,11",
                        }
                    }
                }
            }
        }
    }
}

pub fn tornado_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M20.11,8l1.16-2c0.77-1.33-0.19-3-1.73-3H4.47c-1.54,0-2.5,1.67-1.73,3l1.16,2H20.11z",
                        }
                    }
                    g {
                        path {
                            d: "M7.95,15l2.32,4.01c0.77,1.33,2.69,1.33,3.46,0L16.05,15H7.95z",
                        }
                    }
                    g {
                        polygon {
                            points: "18.95,10 5.05,10 6.79,13 17.21,13",
                        }
                    }
                }
            }
        }
    }
}

pub fn transgender_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17,1.5h-2.75c-0.41,0-0.75,0.34-0.75,0.75C13.5,2.66,13.84,3,14.25,3h1.19l-3.16,3.16C11.62,5.74,10.84,5.5,10,5.5 S8.38,5.74,7.72,6.16l-0.6-0.6l0.53-0.53c0.29-0.29,0.29-0.77,0-1.06s-0.77-0.29-1.06,0L6.06,4.5L4.56,3h1.19 C6.16,3,6.5,2.66,6.5,2.25c0-0.41-0.34-0.75-0.75-0.75H3c-0.55,0-1,0.45-1,1v2.75C2,5.66,2.34,6,2.75,6S3.5,5.66,3.5,5.25V4.06 L5,5.56L4.47,6.09c-0.29,0.29-0.29,0.77,0,1.06c0.29,0.29,0.77,0.29,1.06,0l0.53-0.53l0.56,0.56C6.07,7.89,5.75,8.78,5.75,9.75 c0,2.09,1.51,3.83,3.5,4.18v1.57H8.5c-0.41,0-0.75,0.34-0.75,0.75C7.75,16.66,8.09,17,8.5,17h0.75v0.75c0,0.41,0.34,0.75,0.75,0.75 s0.75-0.34,0.75-0.75V17h0.75c0.41,0,0.75-0.34,0.75-0.75c0-0.41-0.34-0.75-0.75-0.75h-0.75v-1.57c1.99-0.35,3.5-2.09,3.5-4.18 c0-0.97-0.32-1.86-0.87-2.57l3.12-3.12v1.19C16.5,5.66,16.84,6,17.25,6S18,5.66,18,5.25V2.5C18,1.95,17.55,1.5,17,1.5z M10,12.5 c-1.52,0-2.75-1.23-2.75-2.75C7.25,8.23,8.48,7,10,7s2.75,1.23,2.75,2.75C12.75,11.27,11.52,12.5,10,12.5z",
            }
        }
    }
}

pub fn transgender_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.5,1h-4c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h1.58l-3.97,3.97C14.23,6.36,13.16,6,12,6S9.77,6.36,8.89,6.97L8.24,6.32 l0.7-0.7c0.39-0.39,0.39-1.02,0-1.41c-0.39-0.39-1.02-0.39-1.41,0l-0.7,0.7L4.92,3H6.5c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4 c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1s1-0.45,1-1V4.42l1.91,1.9L4.7,7.03c-0.39,0.39-0.39,1.02,0,1.41s1.02,0.39,1.41,0 l0.71-0.71l0.65,0.65C6.86,9.27,6.5,10.34,6.5,11.5c0,2.7,1.94,4.94,4.5,5.41V19h-1c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h1v1 c0,0.55,0.45,1,1,1s1-0.45,1-1v-1h1c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-1v-2.09c2.56-0.47,4.5-2.71,4.5-5.41 c0-1.16-0.36-2.23-0.97-3.12l3.97-3.96V6c0,0.55,0.45,1,1,1s1-0.45,1-1V2C22.5,1.45,22.05,1,21.5,1z M12,15 c-1.93,0-3.5-1.57-3.5-3.5C8.5,9.57,10.07,8,12,8s3.5,1.57,3.5,3.5C15.5,13.43,13.93,15,12,15z",
            }
        }
    }
}

pub fn travel_explore_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17.85,8.5h-1.54c-0.48-2.03-1.93-3.68-3.82-4.48V4.5C12.5,5.33,11.83,6,11,6H9v1c0,0.55-0.45,1-1,1H7v2h1v2H7L3.64,8.64 C3.55,9.08,3.5,9.53,3.5,10c0,3.58,2.92,6.5,6.5,6.5V18c-4.42,0-8-3.58-8-8s3.58-8,8-8C13.91,2,17.15,4.8,17.85,8.5z M17.47,16.97 L17.47,16.97c-0.29,0.29-0.77,0.29-1.06,0l-2.03-2.03c-0.79,0.51-1.8,0.72-2.86,0.42c-1.24-0.35-2.21-1.41-2.46-2.68 c-0.47-2.47,1.66-4.59,4.12-4.12c1.27,0.24,2.33,1.21,2.68,2.46c0.3,1.06,0.09,2.07-0.42,2.86l2.03,2.03 C17.76,16.2,17.76,16.68,17.47,16.97z M14.5,12c0-1.1-0.9-2-2-2s-2,0.9-2,2s0.9,2,2,2S14.5,13.1,14.5,12z",
            }
        }
    }
}

pub fn travel_explore_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19.3,16.9c0.58-1.01,0.95-2.23,0.51-3.65c-0.53-1.72-2.04-3.05-3.84-3.22c-2.87-0.28-5.23,2.07-4.95,4.95 c0.18,1.79,1.5,3.31,3.22,3.84c1.43,0.44,2.64,0.07,3.65-0.51l2.5,2.5c0.39,0.39,1.01,0.39,1.4,0l0,0c0.39-0.39,0.39-1.01,0-1.4 L19.3,16.9z M15.5,17c-1.4,0-2.5-1.1-2.5-2.5s1.1-2.5,2.5-2.5s2.5,1.1,2.5,2.5S16.9,17,15.5,17z M12,20v2C6.48,22,2,17.52,2,12 C2,6.48,6.48,2,12,2c4.84,0,8.87,3.44,9.8,8h-2.07c-0.64-2.46-2.4-4.47-4.73-5.41V5c0,1.1-0.9,2-2,2h-2v2c0,0.55-0.45,1-1,1H8v2h2v3 H9l-4.79-4.79C4.08,10.79,4,11.38,4,12C4,16.41,7.59,20,12,20z",
            }
        }
    }
}

pub fn tsunami_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14.75,14.73c-1.47,1.02-2.64,1.05-4.16,0c-0.35-0.24-0.81-0.24-1.16,0c-1.47,1.02-2.64,1.05-4.17,0 c-0.35-0.24-0.81-0.26-1.15-0.01c-0.68,0.48-1.14,0.67-1.51,0.74C2.26,15.52,2,15.81,2,16.16v0.07c0,0.45,0.41,0.78,0.85,0.7 c0.64-0.12,1.26-0.37,1.83-0.75c1.62,1.11,3.7,1.07,5.38,0.05c1.63,1.04,3.65,1.03,5.27-0.04c0.56,0.37,1.16,0.61,1.78,0.74 C17.57,17,18,16.66,18,16.2l0,0c0-0.35-0.24-0.67-0.59-0.74c-0.37-0.07-0.84-0.26-1.52-0.74C15.55,14.47,15.09,14.49,14.75,14.73z",
                    }
                    path {
                        d: "M16.5,10.5h0.77c0.4,0,0.73-0.33,0.73-0.73V9.73C18,9.33,17.67,9,17.27,9H16.5c-1.6,0-2.9-1.3-2.9-2.9 c0-0.57,0.18-1.31,0.46-1.91l0,0c0.23-0.46-0.06-1.01-0.57-1.08C13,3.04,12.58,3,12.01,3C6.3,3,2,7.08,2,12.5v0.36 c0,0.67,0.64,1.15,1.28,0.96c0.49-0.14,0.96-0.36,1.4-0.65c1.62,1.11,3.7,1.07,5.38,0.05c1.63,1.04,3.65,1.03,5.27-0.04 c0.56,0.37,1.16,0.61,1.78,0.74C17.57,14,18,13.66,18,13.2v0c0-0.35-0.24-0.67-0.59-0.74c-0.37-0.07-0.84-0.26-1.52-0.74 c-0.34-0.24-0.8-0.23-1.15,0.01c-1.46,1.01-2.64,1.05-4.15,0c-0.36-0.25-0.84-0.25-1.19,0.02c-0.03,0.02-0.05,0.04-0.06,0.04 c-0.51-0.77-0.79-1.66-0.79-2.61c0-2.2,1.51-3.92,3.73-4.35c-0.1,0.44-0.17,0.88-0.17,1.27C12.11,8.53,14.08,10.5,16.5,10.5z",
                    }
                }
            }
        }
    }
}

pub fn tsunami_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18.16,17.98c-2.76,1.76-4.67,0.77-5.61,0.08c-0.34-0.24-0.78-0.23-1.12,0.01c-0.97,0.7-2.83,1.65-5.55-0.06 C5.55,17.8,5.13,17.78,4.81,18c-0.91,0.61-1.53,0.85-2,0.94C2.34,19.03,2,19.44,2,19.91v0c0,0.6,0.54,1.09,1.13,0.98 c0.77-0.14,1.51-0.42,2.2-0.83c2.04,1.21,4.63,1.21,6.67,0c2.06,1.22,4.61,1.22,6.67,0c0.69,0.41,1.44,0.69,2.21,0.83 c0.59,0.11,1.13-0.38,1.13-0.98v-0.01c0-0.47-0.33-0.88-0.8-0.97c-0.49-0.1-1.11-0.34-2.02-0.94 C18.88,17.79,18.47,17.78,18.16,17.98z",
                    }
                    path {
                        d: "M19.33,12H21c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1l-1.61,0c-1.86,0-3.4-1.5-3.39-3.36c0-0.37,0.06-0.7,0.16-1.05 C16.53,4.3,15.6,3.03,14.27,3C14.18,3,14.09,3,14,3C7.36,3,2.15,8.03,2.01,14.5l0,0.03c-0.04,1.13,1.07,1.98,2.14,1.6 c0.4-0.14,0.78-0.32,1.15-0.54c2.08,1.2,4.64,1.22,6.7-0.02c2.06,1.22,4.61,1.22,6.67,0c0.68,0.41,1.42,0.68,2.18,0.82 c0.6,0.11,1.16-0.36,1.16-0.98v-0.01c0-0.46-0.32-0.88-0.78-0.97c-0.49-0.09-1.12-0.33-2.03-0.94c-0.31-0.21-0.73-0.22-1.05-0.01 c-2.73,1.74-4.63,0.77-5.58,0.09c-0.35-0.25-0.81-0.26-1.16-0.01c-0.15,0.11-0.09,0.06-0.32,0.2C10.39,12.82,10,11.7,10,10.5 c0-2.58,1.77-4.74,4.21-5.33C14.08,5.68,14,6.19,14,6.67C14,9.61,16.39,12,19.33,12z",
                    }
                }
            }
        }
    }
}

pub fn vaccines_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M15.75,9V8.5c0.41,0,0.75-0.34,0.75-0.75C16.5,7.34,16.16,7,15.75,7h-4C11.34,7,11,7.34,11,7.75c0,0.41,0.34,0.75,0.75,0.75 V9c0,0-1.25,1-1.25,2.5v5c0,0.83,0.67,1.5,1.5,1.5h3.5c0.83,0,1.5-0.67,1.5-1.5v-5.25C17,10,15.75,9,15.75,9z M15.5,16.5H12V15h3.5 V16.5z M15.5,12H12v-0.5c0-1.13,1.25-1.25,1.25-2.5V8.5h1V9c0,1.25,1.25,1.37,1.25,2.5V12z M6,10.5C6,10.78,6.22,11,6.5,11H8v1.5 H4.5v-6H8V8H6.5C6.22,8,6,8.22,6,8.5C6,8.78,6.22,9,6.5,9H8v1H6.5C6.22,10,6,10.22,6,10.5z M6.25,18.5C6.66,18.5,7,18.16,7,17.75V14 h1c0.83,0,1.5-0.67,1.5-1.5l0-6c0.41,0,0.75-0.34,0.75-0.75C10.25,5.34,9.91,5,9.5,5H7V3.5h0.5c0.41,0,0.75-0.34,0.75-0.75 C8.25,2.34,7.91,2,7.5,2H5C4.59,2,4.25,2.34,4.25,2.75C4.25,3.16,4.59,3.5,5,3.5h0.5V5H3C2.59,5,2.25,5.34,2.25,5.75 C2.25,6.16,2.59,6.5,3,6.5l0,6C3,13.33,3.67,14,4.5,14h1v3.75C5.5,18.16,5.84,18.5,6.25,18.5z",
            }
        }
    }
}

pub fn vaccines_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7,22.5c0.55,0,1-0.45,1-1V17h1c1.1,0,2-0.9,2-2V7.5c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H8V4h0.5c0.55,0,1-0.45,1-1 c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1H6v1.5H3c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1V15c0,1.1,0.9,2,2,2h1 v4.5C6,22.05,6.45,22.5,7,22.5z M9,9H7.25C6.84,9,6.5,9.34,6.5,9.75c0,0.41,0.34,0.75,0.75,0.75H9V12H7.25 c-0.41,0-0.75,0.34-0.75,0.75c0,0.41,0.34,0.75,0.75,0.75H9L9,15H5V7.5h4V9z M19.5,10.5V10c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-5 c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1v0.5c0,0.5-1.5,1.16-1.5,3V20c0,1.1,0.9,2,2,2h4c1.1,0,2-0.9,2-2v-6.5 C21,11.66,19.5,11,19.5,10.5z M16.5,10.5V10h1v0.5c0,1.6,1.5,2,1.5,3V14h-4c0-0.21,0-0.39,0-0.5C15,12.5,16.5,12.1,16.5,10.5z M15,20c0,0,0-0.63,0-1.5h4V20H15z",
            }
        }
    }
}

pub fn volcano_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M13.85,7h-3.68C9.48,7,8.88,7.47,8.72,8.14L8,11H5.99c-0.6,0-1.14,0.36-1.38,0.91l-1.71,4C2.47,16.9,3.2,18,4.27,18 h11.76c0.99,0,1.71-0.94,1.45-1.89l-2.18-8C15.12,7.45,14.53,7,13.85,7z",
                        }
                    }
                    g {
                        path {
                            d: "M12,1L12,1c-0.41,0-0.75,0.34-0.75,0.75v1.5C11.25,3.66,11.59,4,12,4h0c0.41,0,0.75-0.34,0.75-0.75v-1.5 C12.75,1.34,12.41,1,12,1z",
                        }
                    }
                    g {
                        path {
                            d: "M16.25,2.74L16.25,2.74c-0.29-0.29-0.77-0.29-1.06,0L14.13,3.8c-0.29,0.29-0.29,0.77,0,1.06l0,0 c0.29,0.29,0.77,0.29,1.06,0l1.06-1.06C16.54,3.51,16.54,3.03,16.25,2.74z",
                        }
                    }
                    g {
                        path {
                            d: "M9.87,3.8L8.81,2.74c-0.29-0.29-0.77-0.29-1.06,0l0,0c-0.29,0.29-0.29,0.77,0,1.06l1.06,1.06c0.29,0.29,0.77,0.29,1.06,0 l0,0C10.16,4.57,10.16,4.09,9.87,3.8z",
                        }
                    }
                }
            }
        }
    }
}

pub fn volcano_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M16.49,8h-4.14c-0.82,0-1.55,0.5-1.86,1.26L9,13H7.3c-0.79,0-1.51,0.47-1.83,1.19l-2.22,5C2.66,20.51,3.63,22,5.08,22 h14.27c1.33,0,2.29-1.27,1.92-2.55l-2.86-10C18.17,8.59,17.38,8,16.49,8z",
                        }
                    }
                    g {
                        path {
                            d: "M14,1L14,1c-0.55,0-1,0.45-1,1v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V2C15,1.45,14.55,1,14,1z",
                        }
                    }
                    g {
                        path {
                            d: "M19.66,3.34L19.66,3.34c-0.39-0.39-1.02-0.39-1.41,0l-1.41,1.41c-0.39,0.39-0.39,1.02,0,1.41v0 c0.39,0.39,1.02,0.39,1.41,0l1.41-1.41C20.05,4.37,20.05,3.73,19.66,3.34z",
                        }
                    }
                    g {
                        path {
                            d: "M11.17,4.76L9.76,3.34c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l1.41,1.41 c0.39,0.39,1.02,0.39,1.41,0l0,0C11.56,5.78,11.56,5.15,11.17,4.76z",
                        }
                    }
                }
            }
        }
    }
}

pub fn wallet_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15,4H5C3.34,4,2,5.34,2,7v6c0,1.66,1.34,3,3,3h10c1.66,0,3-1.34,3-3V7C18,5.34,16.66,4,15,4z M13.3,11.33 c-0.18,0.15-0.43,0.21-0.66,0.15L3.69,9.29C3.94,8.82,4.43,8.5,5,8.5h10c0.43,0,0.82,0.19,1.1,0.49L13.3,11.33z M16.5,7.42 C16.06,7.16,15.55,7,15,7H5C4.45,7,3.94,7.16,3.5,7.42V7c0-0.83,0.67-1.5,1.5-1.5h10c0.83,0,1.5,0.67,1.5,1.5V7.42z",
                }
            }
        }
    }
}

pub fn wallet_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M18,4H6C3.79,4,2,5.79,2,8v8c0,2.21,1.79,4,4,4h12c2.21,0,4-1.79,4-4V8C22,5.79,20.21,4,18,4z M16.14,13.77 c-0.24,0.2-0.57,0.28-0.88,0.2L4.15,11.25C4.45,10.52,5.16,10,6,10h12c0.67,0,1.26,0.34,1.63,0.84L16.14,13.77z M6,6h12 c1.1,0,2,0.9,2,2v0.55C19.41,8.21,18.73,8,18,8H6C5.27,8,4.59,8.21,4,8.55V8C4,6.9,4.9,6,6,6z",
                }
            }
        }
    }
}

pub fn water_drop_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10.65,2.55c-0.38-0.33-0.93-0.33-1.31,0C7.7,4,3.5,8.01,3.5,11.5c0,3.59,2.91,6.5,6.5,6.5s6.5-2.91,6.5-6.5 C16.5,8.01,12.3,4,10.65,2.55z M7.03,11.93c0.24,1.66,1.79,2.77,3.4,2.54c0.3-0.04,0.57,0.19,0.57,0.49c0,0.28-0.2,0.47-0.42,0.5 c-2.23,0.31-4.22-1.23-4.54-3.39C6,11.77,6.23,11.5,6.54,11.5C6.79,11.5,7,11.68,7.03,11.93z",
            }
        }
    }
}

pub fn water_drop_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12.66,2.58c-0.38-0.33-0.95-0.33-1.33,0C6.45,6.88,4,10.62,4,13.8c0,4.98,3.8,8.2,8,8.2s8-3.22,8-8.2 C20,10.62,17.55,6.88,12.66,2.58z M7.83,14c0.37,0,0.67,0.26,0.74,0.62c0.41,2.22,2.28,2.98,3.64,2.87 c0.43-0.02,0.79,0.32,0.79,0.75c0,0.4-0.32,0.73-0.72,0.75c-2.13,0.13-4.62-1.09-5.19-4.12C7.01,14.42,7.37,14,7.83,14z",
            }
        }
    }
}

pub fn waving_hand_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M5.56,2.02C3.7,2.23,2.23,3.7,2.02,5.55C2,5.81,1.79,6,1.53,6c-0.28,0-0.5-0.22-0.5-0.5c0-0.01,0-0.03,0-0.04 c0.25-2.32,2.09-4.17,4.41-4.43c0.02,0,0.04,0,0.06,0c0.28,0,0.5,0.22,0.5,0.5C6,1.78,5.81,2,5.56,2.02z M17.98,14.44 c-0.21,1.85-1.68,3.33-3.53,3.53C14.19,18,14,18.21,14,18.47c0,0.28,0.22,0.5,0.5,0.5c0.01,0,0.03,0,0.04,0 c2.32-0.25,4.17-2.09,4.43-4.41c0-0.02,0-0.04,0-0.06c0-0.28-0.22-0.5-0.5-0.5C18.22,14,18,14.19,17.98,14.44z M10.47,8.11 c-0.2,0.2-0.51,0.2-0.71,0v0c-0.2-0.2-0.2-0.51,0-0.71l4.24-4.24c0.39-0.39,0.39-1.02,0-1.41s-1.02-0.39-1.41,0L6.21,8.14 c0.91,1.22,0.93,2.9,0.04,4.13c-0.18,0.25-0.54,0.27-0.75,0.05l0,0c-0.17-0.17-0.2-0.44-0.06-0.64c0.7-0.98,0.61-2.34-0.27-3.22l0,0 c-0.2-0.2-0.2-0.51,0-0.71l3.18-3.18c0.39-0.39,0.39-1.02,0-1.41s-1.02-0.39-1.41,0L3.05,7.05c-2.73,2.73-2.73,7.17,0,9.9 s7.17,2.73,9.9,0l5.66-5.66c0.39-0.39,0.39-1.02,0-1.41s-1.02-0.39-1.41,0l-3.18,3.18c-0.2,0.2-0.51,0.2-0.71,0l0,0 c-0.2-0.2-0.2-0.51,0-0.71l5.13-5.13c0.39-0.39,0.39-1.02,0-1.41s-1.02-0.39-1.41,0l-4.42,4.42c-0.2,0.2-0.51,0.2-0.71,0l0,0 c-0.2-0.2-0.2-0.51,0-0.71l5.3-5.3c0.39-0.39,0.39-1.02,0-1.41s-1.02-0.39-1.41,0L10.47,8.11z",
            }
        }
    }
}

pub fn waving_hand_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M6.07,9.8c-0.2,0.2-0.2,0.51,0,0.71l0,0c1.07,1.07,1.16,2.75,0.28,3.92c-0.15,0.2-0.12,0.47,0.05,0.65 c0.22,0.22,0.57,0.19,0.76-0.05c1.07-1.43,1.06-3.43-0.04-4.85l7.62-7.62c0.49-0.49,1.28-0.49,1.77,0s0.49,1.28,0,1.77l-5.13,5.13 c-0.2,0.2-0.2,0.51,0,0.71s0.51,0.2,0.71,0l6.54-6.54c0.49-0.49,1.28-0.49,1.77,0s0.49,1.28,0,1.77l-6.54,6.54 c-0.2,0.2-0.2,0.51,0,0.71s0.51,0.2,0.71,0l5.48-5.48c0.49-0.49,1.28-0.49,1.77,0s0.49,1.28,0,1.77l-6.19,6.19 c-0.2,0.2-0.2,0.51,0,0.71c0.2,0.2,0.51,0.2,0.71,0l4.07-4.07c0.49-0.49,1.28-0.49,1.77,0c0.49,0.49,0.49,1.28,0,1.77l-7.07,7.07 c-3.22,3.22-8.45,3.22-11.67,0s-3.22-8.45,0-11.67l4.6-4.6c0.49-0.49,1.28-0.49,1.77,0c0.49,0.49,0.49,1.28,0,1.77L6.07,9.8z M7,1.8 c0-0.41-0.34-0.75-0.75-0.75c-0.04,0-0.07,0-0.11,0.01c-2.63,0.38-4.7,2.46-5.08,5.08c0,0.03-0.01,0.07-0.01,0.11 C1.05,6.66,1.39,7,1.8,7c0.38,0,0.69-0.28,0.74-0.64c0.28-1.97,1.84-3.53,3.81-3.81C6.72,2.49,7,2.18,7,1.8z M17,22.2 c0,0.41,0.34,0.75,0.75,0.75c0.04,0,0.07,0,0.11-0.01c2.63-0.38,4.7-2.46,5.08-5.08c0-0.03,0.01-0.07,0.01-0.11 c0-0.41-0.34-0.75-0.75-0.75c-0.38,0-0.69,0.28-0.74,0.64c-0.28,1.97-1.84,3.53-3.81,3.81C17.28,21.51,17,21.82,17,22.2z",
            }
        }
    }
}

pub fn whatsapp_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M15.69,4.23c-1.51-1.51-3.51-2.34-5.65-2.34c-4.4,0-7.98,3.58-7.98,7.98c0,1.41,0.37,2.78,1.07,3.99L2,18l4.23-1.11 c1.17,0.64,2.48,0.97,3.81,0.97h0c4.4,0,7.98-3.58,7.98-7.98C18.03,7.75,17.2,5.74,15.69,4.23z M10.05,16.51L10.05,16.51 c-1.19,0-2.36-0.32-3.38-0.92l-0.24-0.14L3.92,16.1l0.67-2.45L4.43,13.4c-0.66-1.06-1.01-2.28-1.01-3.53 c0-3.66,2.98-6.63,6.64-6.63c1.77,0,3.44,0.69,4.69,1.95c1.25,1.25,1.94,2.92,1.94,4.69C16.68,13.54,13.71,16.51,10.05,16.51z M13.69,11.55c-0.2-0.1-1.18-0.58-1.36-0.65c-0.18-0.07-0.32-0.1-0.45,0.1c-0.13,0.2-0.52,0.65-0.63,0.78 c-0.12,0.13-0.23,0.15-0.43,0.05c-0.2-0.1-0.84-0.31-1.6-0.99C8.62,10.31,8.22,9.66,8.1,9.46S8.09,9.15,8.19,9.05 c0.09-0.09,0.2-0.23,0.3-0.35s0.13-0.2,0.2-0.33c0.07-0.13,0.03-0.25-0.02-0.35c-0.05-0.1-0.45-1.08-0.61-1.48 C7.89,6.15,7.73,6.2,7.61,6.19C7.49,6.19,7.36,6.19,7.22,6.19c-0.13,0-0.35,0.05-0.53,0.25c-0.18,0.2-0.7,0.68-0.7,1.66 c0,0.98,0.71,1.93,0.81,2.06c0.1,0.13,1.41,2.15,3.41,3.01c0.48,0.21,0.85,0.33,1.14,0.42c0.48,0.15,0.91,0.13,1.26,0.08 c0.38-0.06,1.18-0.48,1.35-0.95c0.17-0.47,0.17-0.87,0.12-0.95C14.02,11.69,13.89,11.64,13.69,11.55z",
                        }
                    }
                }
            }
        }
    }
}

pub fn whatsapp_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    y: "0",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M19.05,4.91C17.18,3.03,14.69,2,12.04,2c-5.46,0-9.91,4.45-9.91,9.91c0,1.75,0.46,3.45,1.32,4.95L2.05,22l5.25-1.38 c1.45,0.79,3.08,1.21,4.74,1.21h0c0,0,0,0,0,0c5.46,0,9.91-4.45,9.91-9.91C21.95,9.27,20.92,6.78,19.05,4.91z M12.04,20.15 L12.04,20.15c-1.48,0-2.93-0.4-4.2-1.15l-0.3-0.18l-3.12,0.82l0.83-3.04l-0.2-0.31c-0.82-1.31-1.26-2.83-1.26-4.38 c0-4.54,3.7-8.24,8.24-8.24c2.2,0,4.27,0.86,5.82,2.42c1.56,1.56,2.41,3.63,2.41,5.83C20.28,16.46,16.58,20.15,12.04,20.15z M16.56,13.99c-0.25-0.12-1.47-0.72-1.69-0.81c-0.23-0.08-0.39-0.12-0.56,0.12c-0.17,0.25-0.64,0.81-0.78,0.97 c-0.14,0.17-0.29,0.19-0.54,0.06c-0.25-0.12-1.05-0.39-1.99-1.23c-0.74-0.66-1.23-1.47-1.38-1.72c-0.14-0.25-0.02-0.38,0.11-0.51 c0.11-0.11,0.25-0.29,0.37-0.43c0.12-0.14,0.17-0.25,0.25-0.41c0.08-0.17,0.04-0.31-0.02-0.43c-0.06-0.12-0.56-1.34-0.76-1.84 c-0.2-0.48-0.41-0.42-0.56-0.43C8.86,7.33,8.7,7.33,8.53,7.33c-0.17,0-0.43,0.06-0.66,0.31C7.65,7.89,7.01,8.49,7.01,9.71 c0,1.22,0.89,2.4,1.01,2.56c0.12,0.17,1.75,2.67,4.23,3.74c0.59,0.26,1.05,0.41,1.41,0.52c0.59,0.19,1.13,0.16,1.56,0.1 c0.48-0.07,1.47-0.6,1.67-1.18c0.21-0.58,0.21-1.07,0.14-1.18S16.81,14.11,16.56,13.99z",
                        }
                    }
                }
            }
        }
    }
}

pub fn whatshot_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17.09 4.56c-.7-1.03-1.5-1.99-2.4-2.85-.35-.34-.94-.02-.84.46.19.94.39 2.18.39 3.29 0 2.06-1.35 3.73-3.41 3.73-1.54 0-2.8-.93-3.35-2.26-.1-.2-.14-.32-.2-.54-.11-.42-.66-.55-.9-.18-.18.27-.35.54-.51.83C4.68 9.08 4 11.46 4 14c0 4.42 3.58 8 8 8s8-3.58 8-8c0-3.49-1.08-6.73-2.91-9.44zM11.71 19c-1.78 0-3.22-1.4-3.22-3.14 0-1.62 1.05-2.76 2.81-3.12 1.47-.3 2.98-.93 4.03-1.92.28-.26.74-.14.82.23.23 1.02.35 2.08.35 3.15.01 2.65-2.14 4.8-4.79 4.8z",
            }
        }
    }
}

pub fn woman_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cy: "3.75",
                        r: "1.75",
                        cx: "10",
                    }
                    path {
                        d: "M11.39,7.92C11.15,7.36,10.61,7,10,7S8.85,7.36,8.61,7.92L6.6,12.61C6.31,13.27,6.8,14,7.52,14H8.5v3c0,0.55,0.45,1,1,1 h1.01c0.55,0,1-0.45,1-1v-3h0.98c0.72,0,1.2-0.73,0.92-1.39L11.39,7.92z",
                    }
                }
            }
        }
    }
}

pub fn woman_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                circle {
                    cy: "4",
                    r: "2",
                    cx: "12",
                }
                path {
                    d: "M16.45,14.63l-2.52-6.32c-0.32-0.79-1.08-1.3-1.94-1.31c-0.85,0-1.62,0.51-1.94,1.31l-2.52,6.32 C7.28,15.29,7.77,16,8.47,16H10v5c0,0.55,0.45,1,1,1h1h1c0.55,0,1-0.45,1-1v-5h1.53C16.23,16,16.72,15.29,16.45,14.63z",
                }
            }
        }
    }
}

pub fn woman_2_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cy: "3.75",
                        cx: "10",
                        r: "1.75",
                    }
                    path {
                        d: "M11.39,7.92C11.15,7.36,10.61,7,10,7S8.85,7.36,8.61,7.92L6.3,13.3C6.16,13.63,6.4,14,6.76,14H9v3.5 C9,17.78,9.22,18,9.5,18h1c0.28,0,0.5-0.22,0.5-0.5V14h2.24c0.36,0,0.6-0.37,0.46-0.7L11.39,7.92z",
                    }
                }
            }
        }
    }
}

pub fn woman_2_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M13.94,8.31c-0.38-0.96-1.42-1.52-2.48-1.24c-0.66,0.17-1.18,0.7-1.43,1.34l-2.48,6.22C7.28,15.29,7.77,16,8.47,16h2.03 v5c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-5h2.03c0.71,0,1.19-0.71,0.93-1.37L13.94,8.31z",
                        }
                        circle {
                            r: "2",
                            cx: "12",
                            cy: "4",
                        }
                    }
                }
            }
        }
    }
}

pub fn workspace_premium_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M14.5,14.4v-1.22c1.23-1.18,2-2.84,2-4.68C16.5,4.91,13.59,2,10,2S3.5,4.91,3.5,8.5c0,1.83,0.76,3.48,1.97,4.66v1.5 M5.47,14.66v3.09c0,0.64,0.59,1.11,1.22,0.98L10,18l3.28,0.73c0.62,0.14,1.22-0.34,1.22-0.98V14.4 M10,13.5c-2.76,0-5-2.24-5-5 s2.24-5,5-5s5,2.24,5,5S12.76,13.5,10,13.5z M9.38,10.41L10,9.94l0.61,0.46c0.39,0.29,0.92-0.08,0.78-0.55l-0.24-0.79l0.73-0.58 c0.37-0.29,0.16-0.89-0.31-0.89h-0.84l-0.25-0.78c-0.15-0.46-0.8-0.46-0.95,0L9.27,7.6H8.43c-0.47,0-0.68,0.6-0.31,0.89l0.73,0.58 L8.6,9.87C8.46,10.33,9,10.71,9.38,10.41z",
            }
        }
    }
}

pub fn workspace_premium_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M10.92,12.75L12,11.93l1.07,0.81c0.39,0.29,0.92-0.08,0.78-0.55l-0.42-1.36l1.2-0.95C15,9.6,14.79,9,14.31,9h-1.4 l-0.43-1.34c-0.15-0.46-0.8-0.46-0.95,0L11.09,9H9.68C9.21,9,9,9.6,9.37,9.89l1.19,0.95l-0.42,1.36 C10,12.67,10.53,13.04,10.92,12.75z M6,21.61c0,0.68,0.67,1.16,1.32,0.95L12,21l4.68,1.56C17.33,22.78,18,22.3,18,21.61v-6.33 c1.24-1.41,2-3.25,2-5.28c0-4.42-3.58-8-8-8s-8,3.58-8,8c0,2.03,0.76,3.87,2,5.28V21.61z M12,4c3.31,0,6,2.69,6,6s-2.69,6-6,6 s-6-2.69-6-6S8.69,4,12,4z",
            }
        }
    }
}

