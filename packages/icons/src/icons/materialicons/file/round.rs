use dioxus::prelude::*;
use crate::IconProps;
pub fn approval_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M4,16v4c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-4c0-1.1-0.9-2-2-2H6C4.9,14,4,14.9,4,16z M17,18H7c-0.55,0-1-0.45-1-1v0 c0-0.55,0.45-1,1-1h10c0.55,0,1,0.45,1,1v0C18,17.55,17.55,18,17,18z M12,2C9.54,2,7.48,3.79,7.07,6.13 C6.99,6.65,7.13,7.18,7.43,7.6l3.76,5.26c0.4,0.56,1.23,0.56,1.63,0l3.76-5.26c0.3-0.42,0.44-0.95,0.35-1.47 C16.52,3.79,14.46,2,12,2z",
                    }
                }
            }
        }
    }
}

pub fn attachment_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17.75 16H7.17c-2.09 0-3.95-1.53-4.15-3.61C2.79 10.01 4.66 8 7 8h12.36c1.31 0 2.5.94 2.63 2.24.15 1.5-1.02 2.76-2.49 2.76H9c-.55 0-1-.45-1-1s.45-1 1-1h8.75c.41 0 .75-.34.75-.75s-.34-.75-.75-.75H9.14c-1.31 0-2.5.94-2.63 2.24-.15 1.5 1.02 2.76 2.49 2.76h10.33c2.09 0 3.95-1.53 4.15-3.61.23-2.39-1.64-4.39-3.98-4.39H7.23c-2.87 0-5.44 2.1-5.71 4.96-.3 3.29 2.26 6.04 5.48 6.04h10.75c.41 0 .75-.34.75-.75s-.34-.75-.75-.75z",
            }
        }
    }
}

pub fn attach_email_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M17,9V3.5C17,2.67,16.33,2,15.5,2h-13C1.67,2,1,2.67,1,3.5v9C1,13.33,1.67,14,2.5,14H12v-2c0-1.66,1.34-3,3-3H17z M9.75,8.57c-0.46,0.27-1.03,0.27-1.49,0L2.89,5.49C2.65,5.35,2.5,5.1,2.5,4.82v0c0-0.59,0.64-0.96,1.15-0.67L9,7.23l5.35-3.07 c0.51-0.29,1.15,0.08,1.15,0.67v0c0,0.27-0.15,0.53-0.39,0.67L9.75,8.57z",
                    }
                    path {
                        d: "M17.5,12.75l0,2.17c0,0.81-0.61,1.54-1.42,1.58c-0.87,0.05-1.58-0.64-1.58-1.5v-3c0-0.28,0.22-0.5,0.5-0.5 s0.5,0.22,0.5,0.5v2.25c0,0.41,0.34,0.75,0.75,0.75h0c0.41,0,0.75-0.34,0.75-0.75l0-2.16c0-1.04-0.76-1.98-1.8-2.08 C14,9.9,13,10.83,13,12l0,2.87c0,1.64,1.27,3.08,2.91,3.12c1.7,0.05,3.09-1.31,3.09-3v-2.25c0-0.41-0.34-0.75-0.75-0.75h0 C17.84,12,17.5,12.34,17.5,12.75z",
                    }
                }
            }
        }
    }
}

pub fn attach_email_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        g {
                            path {
                                d: "M21,10V4c0-1.1-0.9-2-2-2H3C1.9,2,1.01,2.9,1.01,4L1,16c0,1.1,0.9,2,2,2h11v-5c0-1.66,1.34-3,3-3H21z M11.53,10.67 c-0.32,0.2-0.74,0.2-1.06,0L3.4,6.25C3.15,6.09,3,5.82,3,5.53c0-0.67,0.73-1.07,1.3-0.72L11,9l6.7-4.19 C18.27,4.46,19,4.86,19,5.53c0,0.29-0.15,0.56-0.4,0.72L11.53,10.67z",
                            }
                            path {
                                d: "M22,14c-0.55,0-1,0.45-1,1v3c0,1.1-0.9,2-2,2s-2-0.9-2-2v-4.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5V17 c0,0.55,0.45,1,1,1s1-0.45,1-1v-3.5c0-1.38-1.12-2.5-2.5-2.5S15,12.12,15,13.5V18c0,2.21,1.79,4,4,4s4-1.79,4-4v-3 C23,14.45,22.55,14,22,14z",
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn cloud_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96z",
            }
        }
    }
}

pub fn cloud_circle_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm4.5 14H8c-1.66 0-3-1.34-3-3s1.34-3 3-3h.14c.44-1.73 1.99-3 3.86-3 2.21 0 4 1.79 4 4h.5c1.38 0 2.5 1.12 2.5 2.5S17.88 16 16.5 16z",
            }
        }
    }
}

pub fn cloud_done_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zm-8.64 6.25c-.39.39-1.02.39-1.41 0L7.2 14.2c-.39-.39-.39-1.02 0-1.41.39-.39 1.02-.39 1.41 0L10 14.18l4.48-4.48c.39-.39 1.02-.39 1.41 0 .39.39.39 1.02 0 1.41l-5.18 5.18z",
            }
        }
    }
}

pub fn cloud_download_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM17 13l-4.65 4.65c-.2.2-.51.2-.71 0L7 13h3V9h4v4h3z",
            }
        }
    }
}

pub fn cloud_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M24 15c0-2.64-2.05-4.78-4.65-4.96C18.67 6.59 15.64 4 12 4c-1.33 0-2.57.36-3.65.97l1.49 1.49C10.51 6.17 11.23 6 12 6c3.04 0 5.5 2.46 5.5 5.5v.5H19c1.66 0 3 1.34 3 3 0 .99-.48 1.85-1.21 2.4l1.41 1.41c1.09-.92 1.8-2.27 1.8-3.81zM3.71 4.56c-.39.39-.39 1.02 0 1.41l2.06 2.06h-.42c-3.28.35-5.76 3.34-5.29 6.79C.46 17.84 3.19 20 6.22 20h11.51l1.29 1.29c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L5.12 4.56c-.39-.39-1.02-.39-1.41 0zM6 18c-2.21 0-4-1.79-4-4s1.79-4 4-4h1.73l8 8H6z",
            }
        }
    }
}

pub fn cloud_queue_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM19 18H6c-2.21 0-4-1.79-4-4s1.79-4 4-4h.71C7.37 7.69 9.48 6 12 6c3.04 0 5.5 2.46 5.5 5.5v.5H19c1.66 0 3 1.34 3 3s-1.34 3-3 3z",
            }
        }
    }
}

pub fn cloud_sync_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18.18,12.25C18.01,10.98,16.92,10,15.6,10c-1.06,0-1.97,0.64-2.35,1.5c-1.24,0-2.25,1.01-2.25,2.25 c0,1.24,1.01,2.25,2.25,2.25h4.88c1.04,0,1.88-0.84,1.88-1.88C20,13.11,19.19,12.28,18.18,12.25z M5.25,16 c-0.41,0-0.75-0.34-0.75-0.75s0.34-0.75,0.75-0.75h0.8C4.8,13.4,4,11.8,4,10c0-2.39,1.4-4.45,3.42-5.41 C7.92,4.35,8.5,4.72,8.5,5.27c0,0.29-0.17,0.56-0.44,0.68C6.55,6.68,5.5,8.21,5.5,10c0,1.32,0.59,2.5,1.5,3.32v-0.57 C7,12.34,7.34,12,7.75,12c0.41,0,0.75,0.34,0.75,0.75v2.5C8.5,15.66,8.16,16,7.75,16H5.25z M13,7.25C13,7.66,12.66,8,12.25,8 S11.5,7.66,11.5,7.25V5c0-0.55,0.45-1,1-1h2.25c0.41,0,0.75,0.34,0.75,0.75S15.16,5.5,14.75,5.5h-0.8 c1.02,0.89,1.73,2.11,1.97,3.5l-1.53,0C14.17,8.09,13.68,7.29,13,6.68V7.25z",
                    }
                }
            }
        }
    }
}

pub fn cloud_sync_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M24,17.48c0,1.38-1.12,2.5-2.5,2.5L15,20c-1.66,0-3-1.34-3-3c0-1.6,1.26-2.9,2.84-2.98C15.4,12.83,16.6,12,18,12 c1.76,0,3.2,1.3,3.45,2.99c0.02,0,0.03-0.01,0.05-0.01C22.88,14.98,24,16.1,24,17.48z M10,15c0-0.55-0.45-1-1-1s-1,0.45-1,1v1.44 c-1.22-1.1-2-2.67-2-4.44c0-2.38,1.39-4.43,3.4-5.4C9.77,6.42,10,6.04,10,5.63c0-0.71-0.73-1.18-1.37-0.88C5.89,6.03,4,8.79,4,12 c0,2.4,1.06,4.54,2.73,6H5c-0.55,0-1,0.45-1,1s0.45,1,1,1h4c0.55,0,1-0.45,1-1V15z M19,6c0.55,0,1-0.45,1-1s-0.45-1-1-1h-4 c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1s1-0.45,1-1V7.56c0.98,0.89,1.68,2.08,1.92,3.44l2.02,0c-0.25-1.99-1.23-3.74-2.66-5H19z",
                    }
                }
            }
        }
    }
}

pub fn cloud_upload_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM14 13v4h-4v-4H7l4.65-4.65c.2-.2.51-.2.71 0L17 13h-3z",
            }
        }
    }
}

pub fn create_new_folder_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 6h-8l-1.41-1.41C10.21 4.21 9.7 4 9.17 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-2 8h-2v2c0 .55-.45 1-1 1s-1-.45-1-1v-2h-2c-.55 0-1-.45-1-1s.45-1 1-1h2v-2c0-.55.45-1 1-1s1 .45 1 1v2h2c.55 0 1 .45 1 1s-.45 1-1 1z",
            }
        }
    }
}

pub fn difference_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M2.75,6C3.16,6,3.5,6.34,3.5,6.75V17.5h8.75c0.41,0,0.75,0.34,0.75,0.75S12.66,19,12.25,19H3.5C2.67,19,2,18.33,2,17.5 V6.75C2,6.34,2.34,6,2.75,6z M17,5.62v8.88c0,0.83-0.67,1.5-1.5,1.5h-9C5.67,16,5,15.33,5,14.5v-12C5,1.67,5.67,1,6.5,1h5.88 c0.4,0,0.78,0.16,1.06,0.44l3.12,3.12C16.84,4.84,17,5.22,17,5.62z M13.25,11.75c0-0.41-0.34-0.75-0.75-0.75h-3 c-0.41,0-0.75,0.34-0.75,0.75v0c0,0.41,0.34,0.75,0.75,0.75h3C12.91,12.5,13.25,12.16,13.25,11.75L13.25,11.75z M12.5,6.5h-0.75 V5.75C11.75,5.34,11.41,5,11,5h0c-0.41,0-0.75,0.34-0.75,0.75V6.5H9.5c-0.41,0-0.75,0.34-0.75,0.75v0C8.75,7.66,9.09,8,9.5,8h0.75 v0.75c0,0.41,0.34,0.75,0.75,0.75h0c0.41,0,0.75-0.34,0.75-0.75V8h0.75c0.41,0,0.75-0.34,0.75-0.75v0 C13.25,6.84,12.91,6.5,12.5,6.5z",
                }
            }
        }
    }
}

pub fn difference_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M3,7c0.55,0,1,0.45,1,1v13h13c0.55,0,1,0.45,1,1s-0.45,1-1,1H4c-1.1,0-2-0.9-2-2V8C2,7.45,2.45,7,3,7z M15.59,1.59 C15.21,1.21,14.7,1,14.17,1H8C6.9,1,6.01,1.9,6.01,3L6,17c0,1.1,0.89,2,1.99,2H19c1.1,0,2-0.9,2-2V7.83c0-0.53-0.21-1.04-0.59-1.41 L15.59,1.59z M15.5,15h-4c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v0C16.5,14.55,16.05,15,15.5,15z M15.5,9h-1 v1c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V9h-1c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h1V6c0-0.55,0.45-1,1-1h0 c0.55,0,1,0.45,1,1v1h1c0.55,0,1,0.45,1,1v0C16.5,8.55,16.05,9,15.5,9z",
                }
            }
        }
    }
}

pub fn download_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16.59 9H15V4c0-.55-.45-1-1-1h-4c-.55 0-1 .45-1 1v5H7.41c-.89 0-1.34 1.08-.71 1.71l4.59 4.59c.39.39 1.02.39 1.41 0l4.59-4.59c.63-.63.19-1.71-.7-1.71zM5 19c0 .55.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1H6c-.55 0-1 .45-1 1z",
            }
        }
    }
}

pub fn downloading_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M17.33,3.55c-0.94-0.6-1.99-1.04-3.12-1.3C13.59,2.11,13,2.59,13,3.23v0c0,0.45,0.3,0.87,0.74,0.97 c0.91,0.2,1.77,0.56,2.53,1.05c0.39,0.25,0.89,0.17,1.22-0.16l0,0C17.94,4.64,17.87,3.89,17.33,3.55z M20.77,11L20.77,11 c0.64,0,1.13-0.59,0.98-1.21c-0.26-1.12-0.7-2.17-1.3-3.12c-0.34-0.54-1.1-0.61-1.55-0.16l0,0c-0.32,0.32-0.4,0.83-0.16,1.22 c0.49,0.77,0.85,1.62,1.05,2.53C19.9,10.7,20.31,11,20.77,11z M18.9,17.49L18.9,17.49c0.45,0.45,1.21,0.38,1.55-0.15 c0.6-0.94,1.04-1.99,1.3-3.11c0.14-0.62-0.35-1.21-0.98-1.21h0c-0.45,0-0.87,0.3-0.97,0.74c-0.2,0.91-0.57,1.76-1.05,2.53 C18.5,16.66,18.58,17.17,18.9,17.49z M13,20.77L13,20.77c0,0.64,0.59,1.13,1.21,0.98c1.12-0.26,2.17-0.7,3.11-1.3 c0.54-0.34,0.61-1.1,0.16-1.55l0,0c-0.32-0.32-0.83-0.4-1.21-0.15c-0.76,0.49-1.61,0.85-2.53,1.05C13.3,19.9,13,20.31,13,20.77z M13,12V8c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v4H9.41c-0.89,0-1.34,1.08-0.71,1.71l2.59,2.59c0.39,0.39,1.02,0.39,1.41,0 l2.59-2.59c0.63-0.63,0.18-1.71-0.71-1.71H13z M11,20.77L11,20.77c0,0.64-0.59,1.13-1.21,0.99C5.33,20.75,2,16.77,2,12 s3.33-8.75,7.79-9.75C10.41,2.11,11,2.59,11,3.23v0c0,0.46-0.31,0.87-0.76,0.97C6.67,5,4,8.19,4,12s2.67,7,6.24,7.8 C10.69,19.9,11,20.31,11,20.77z",
                    }
                }
            }
        }
    }
}

pub fn download_done_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M6 18h12c.55 0 1 .45 1 1s-.45 1-1 1H6c-.55 0-1-.45-1-1s.45-1 1-1zm5.01-4.1c-.78.77-2.04.77-2.82-.01L6 11.7c-.55-.55-.54-1.44.03-1.97.54-.52 1.4-.5 1.92.02L9.6 11.4l6.43-6.43c.54-.54 1.41-.54 1.95 0l.04.04c.54.54.54 1.42-.01 1.96l-7 6.93z",
            }
        }
    }
}

pub fn download_for_offline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2C6.49,2,2,6.49,2,12s4.49,10,10,10s10-4.49,10-10S17.51,2,12,2z M11,10V7c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v3 h1.79c0.45,0,0.67,0.54,0.35,0.85l-2.79,2.79c-0.2,0.2-0.51,0.2-0.71,0l-2.79-2.79C8.54,10.54,8.76,10,9.21,10H11z M16,17H8 c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h8c0.55,0,1,0.45,1,1v0C17,16.55,16.55,17,16,17z",
                }
            }
        }
    }
}

pub fn drive_file_move_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M12,15.79V14H9c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h3v-1.79c0-0.45,0.54-0.67,0.85-0.35l2.79,2.79 c0.2,0.2,0.2,0.51,0,0.71l-2.79,2.79C12.54,16.46,12,16.24,12,15.79z",
                }
            }
        }
    }
}

pub fn drive_file_move_rtl_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M13,11.02c-0.01,0.41-0.37,0.73-0.78,0.73H10v1.04c0,0.45-0.54,0.67-0.85,0.35l-1.79-1.79c-0.2-0.2-0.2-0.51,0-0.71 l1.79-1.79C9.46,8.54,10,8.76,10,9.21v1.04h2.25C12.67,10.25,13.01,10.6,13,11.02z M16.5,6H10L8.44,4.44C8.16,4.16,7.78,4,7.38,4 H3.5C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-7C18,6.67,17.33,6,16.5,6z",
            }
        }
    }
}

pub fn drive_file_move_rtl_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M11.15,16.15l-2.79-2.79c-0.2-0.2-0.2-0.51,0-0.71l2.79-2.79C11.46,9.54,12,9.76,12,10.21V12h3 c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h-3v1.79C12,16.24,11.46,16.46,11.15,16.15z",
            }
        }
    }
}

pub fn drive_file_rename_outline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15,16l-4,4h8c1.1,0,2-0.9,2-2v0c0-1.1-0.9-2-2-2H15z",
                    }
                    path {
                        d: "M12.06,7.19l-8.77,8.77C3.11,16.14,3,16.4,3,16.66V19c0,0.55,0.45,1,1,1h2.34c0.27,0,0.52-0.11,0.71-0.29l8.77-8.77 L12.06,7.19z",
                    }
                    path {
                        d: "M18.71,8.04c0.39-0.39,0.39-1.02,0-1.41l-2.34-2.34c-0.39-0.39-1.02-0.39-1.41,0l-1.83,1.83l3.75,3.75L18.71,8.04z",
                    }
                }
            }
        }
    }
}

pub fn drive_folder_upload_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M13,13v3c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-3H9.21c-0.45,0-0.67-0.54-0.35-0.85l2.8-2.79 c0.2-0.2,0.51-0.19,0.71,0l2.79,2.79C15.46,12.46,15.24,13,14.8,13H13z",
                }
            }
        }
    }
}

pub fn file_download_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.59,9H15V4c0-0.55-0.45-1-1-1h-4C9.45,3,9,3.45,9,4v5H7.41c-0.89,0-1.34,1.08-0.71,1.71l4.59,4.59 c0.39,0.39,1.02,0.39,1.41,0l4.59-4.59C17.92,10.08,17.48,9,16.59,9z M5,19c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1s-0.45-1-1-1H6 C5.45,18,5,18.45,5,19z",
                }
            }
        }
    }
}

pub fn file_download_done_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M19.42,4.71L19.42,4.71c-0.39-0.39-1.02-0.39-1.41,0l-8.48,8.49L5.99,9.66c-0.39-0.39-1.02-0.39-1.41,0l0,0 c-0.39,0.39-0.39,1.02,0,1.41l4.24,4.24c0.39,0.39,1.02,0.39,1.41,0l9.19-9.19C19.82,5.73,19.82,5.1,19.42,4.71z",
                    }
                    path {
                        d: "M6,20h12c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6c-0.55,0-1,0.45-1,1v0C5,19.55,5.45,20,6,20z",
                    }
                }
            }
        }
    }
}

pub fn file_download_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M8,5.88V3.5C8,3.22,8.22,3,8.5,3h3C11.78,3,12,3.22,12,3.5V8h1.79c0.45,0,0.67,0.54,0.35,0.85l-1.59,1.59L8,5.88z M16.54,16.54L3.46,3.46c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.77,0,1.06l3.56,3.56C5.7,8.23,5.61,8.61,5.85,8.85 l3.79,3.79c0.2,0.2,0.51,0.2,0.71,0l0.09-0.09l1.94,1.94H5.75C5.34,14.5,5,14.84,5,15.25C5,15.66,5.34,16,5.75,16h8.13l1.6,1.6 c0.29,0.29,0.77,0.29,1.06,0C16.83,17.31,16.83,16.83,16.54,16.54z",
            }
        }
    }
}

pub fn file_download_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9,6.17V4c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v5h1.59c0.89,0,1.33,1.08,0.7,1.71l-1.88,1.88L9,6.17z M20.49,20.49 L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l4.5,4.5c-0.26,0.37-0.28,0.91,0.1,1.28l4.59,4.59 c0.35,0.35,0.88,0.37,1.27,0.09L15.17,18H6c-0.55,0-1,0.45-1,1s0.45,1,1,1h11.17l1.9,1.9c0.39,0.39,1.02,0.39,1.41,0 C20.88,21.51,20.88,20.88,20.49,20.49z",
            }
        }
    }
}

pub fn file_open_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,16.99V14.5c0-0.55,0.45-1,1-1h2.49c0.41,0,0.75,0.34,0.75,0.75v0c0,0.41-0.34,0.75-0.75,0.75h-0.93l1.68,1.68 c0.29,0.29,0.29,0.77,0,1.06h0c-0.29,0.29-0.77,0.29-1.06,0l-1.68-1.68v0.93c0,0.41-0.34,0.75-0.75,0.75h0 C14.34,17.74,14,17.41,14,16.99z M11.38,2H5.5C4.67,2,4,2.67,4,3.5v13C4,17.33,4.67,18,5.5,18h7v-4.5c0-0.83,0.67-1.5,1.5-1.5h2 V6.62c0-0.4-0.16-0.78-0.44-1.06l-3.12-3.12C12.16,2.16,11.78,2,11.38,2z M11,6V3l4,4h-3C11.45,7,11,6.55,11,6z",
                }
            }
        }
    }
}

pub fn file_open_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13.17,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h9v-6c0-1.1,0.9-2,2-2h3V8.83c0-0.53-0.21-1.04-0.59-1.41l-4.83-4.83 C14.21,2.21,13.7,2,13.17,2z M13,8V3.5L18.5,9H14C13.45,9,13,8.55,13,8z M22.66,17c0,0.55-0.45,1-1,1h-1.24l2.24,2.24 c0.39,0.39,0.39,1.02,0,1.41l0,0c-0.39,0.39-1.02,0.39-1.41,0L19,19.41l0,1.24c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1V17 c0-0.55,0.45-1,1-1h3.66C22.21,16,22.66,16.45,22.66,17z",
                }
            }
        }
    }
}

pub fn file_upload_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M7.4,10h1.59v5c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-5h1.59c0.89,0,1.34-1.08,0.71-1.71L12.7,3.7 c-0.39-0.39-1.02-0.39-1.41,0L6.7,8.29C6.07,8.92,6.51,10,7.4,10z M5,19c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1s-0.45-1-1-1H6 C5.45,18,5,18.45,5,19z",
                }
            }
        }
    }
}

pub fn folder_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M10.59 4.59C10.21 4.21 9.7 4 9.17 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2h-8l-1.41-1.41z",
            }
        }
    }
}

pub fn folder_copy_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M1.75,5L1.75,5C1.34,5,1,5.34,1,5.75v9.75C1,16.33,1.67,17,2.5,17h12.43c0.41,0,0.75-0.34,0.75-0.75l0,0 c0-0.41-0.34-0.75-0.75-0.75H2.5V5.75C2.5,5.34,2.16,5,1.75,5z",
                    }
                    path {
                        d: "M16.5,4H11L9.44,2.44C9.16,2.16,8.78,2,8.38,2H5.5C4.67,2,4,2.67,4,3.5v9C4,13.33,4.67,14,5.5,14h11 c0.83,0,1.5-0.67,1.5-1.5v-7C18,4.67,17.33,4,16.5,4z",
                    }
                }
            }
        }
    }
}

pub fn folder_copy_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M2,6L2,6C1.45,6,1,6.45,1,7v12c0,1.1,0.9,2,2,2h16c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H3V7C3,6.45,2.55,6,2,6z",
                    }
                    path {
                        d: "M21,4h-7l-1.41-1.41C12.21,2.21,11.7,2,11.17,2H7C5.9,2,5.01,2.9,5.01,4L5,15c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V6 C23,4.9,22.1,4,21,4z",
                    }
                }
            }
        }
    }
}

pub fn folder_delete_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,12.5h-1.5V10H14V12.5z M16.5,6H10L8.44,4.44C8.16,4.16,7.78,4,7.38,4H3.5C2.67,4,2,4.67,2,5.5v9 C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-7C18,6.67,17.33,6,16.5,6z M15,10v2.5c0,0.55-0.45,1-1,1h-1.5 c-0.55,0-1-0.45-1-1V10C11.22,10,11,9.78,11,9.5C11,9.22,11.22,9,11.5,9h1c0-0.28,0.22-0.5,0.5-0.5h0.5C13.78,8.5,14,8.72,14,9h1 c0.28,0,0.5,0.22,0.5,0.5C15.5,9.78,15.28,10,15,10z",
                }
            }
        }
    }
}

pub fn folder_delete_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.5,15.5h-2v-4h2V15.5z M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16 c1.1,0,2-0.9,2-2V8C22,6.9,21.1,6,20,6z M18.25,11.5H18v4c0,0.83-0.67,1.5-1.5,1.5h-2c-0.83,0-1.5-0.67-1.5-1.5v-4h-0.25 c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75h1.75V9.75C14.5,9.34,14.84,9,15.25,9h0.5c0.41,0,0.75,0.34,0.75,0.75V10 h1.75c0.41,0,0.75,0.34,0.75,0.75C19,11.16,18.66,11.5,18.25,11.5z",
                }
            }
        }
    }
}

pub fn folder_off_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M2.75,2.75c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.77,0,1.06l0.69,0.69C2.15,4.77,2,5.12,2,5.5v9 C2,15.33,2.67,16,3.5,16h10.38l2.31,2.31c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06L2.75,2.75z",
                    }
                    path {
                        d: "M16.5,6H10L8.44,4.44C8.16,4.16,7.78,4,7.38,4H6.12l11.49,11.49C17.85,15.23,18,14.88,18,14.5v-7C18,6.67,17.33,6,16.5,6z",
                    }
                }
            }
        }
    }
}

pub fn folder_off_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M2.81,2.81c-0.39-0.39-1.02-0.39-1.41,0C1,3.2,1,3.83,1.39,4.22l0.85,0.85C2.1,5.35,2.01,5.66,2.01,6L2,18 c0,1.1,0.9,2,2,2h13.17l2.61,2.61c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41L2.81,2.81z",
                    }
                    path {
                        d: "M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H6.83l14.93,14.93C21.91,18.65,22,18.34,22,18V8C22,6.9,21.1,6,20,6z",
                    }
                }
            }
        }
    }
}

pub fn folder_open_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 6h-8l-1.41-1.41C10.21 4.21 9.7 4 9.17 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-1 12H5c-.55 0-1-.45-1-1V9c0-.55.45-1 1-1h14c.55 0 1 .45 1 1v8c0 .55-.45 1-1 1z",
            }
        }
    }
}

pub fn folder_shared_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 6h-8l-1.41-1.41C10.21 4.21 9.7 4 9.17 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-5 3c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm4 8h-8v-1c0-1.33 2.67-2 4-2s4 .67 4 2v1z",
            }
        }
    }
}

pub fn folder_zip_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.5,6H10L8.44,4.44C8.16,4.16,7.78,4,7.38,4H3.5C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13 c0.83,0,1.5-0.67,1.5-1.5v-7C18,6.67,17.33,6,16.5,6z M15,9h-1.5v1.5H15V12h-1.5v1.5H12V12h1.5v-1.5H12V9h1.5V7.5H15V9z",
                }
            }
        }
    }
}

pub fn folder_zip_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M18,12h-2v2h2v2h-2v2h-2v-2h2v-2h-2v-2h2v-2h-2V8h2v2h2V12z",
                }
            }
        }
    }
}

pub fn format_overline_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15,3.75c0,0.41-0.34,0.75-0.75,0.75h-8.5C5.34,4.5,5,4.16,5,3.75S5.34,3,5.75,3h8.5C14.66,3,15,3.34,15,3.75z M15.5,11.5 c0,3.04-2.46,5.5-5.5,5.5s-5.5-2.46-5.5-5.5C4.5,8.46,6.96,6,10,6S15.5,8.46,15.5,11.5z M13.5,11.5C13.5,9.57,11.93,8,10,8 s-3.5,1.57-3.5,3.5S8.07,15,10,15S13.5,13.43,13.5,11.5z",
                }
            }
        }
    }
}

pub fn format_overline_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M5,4c0-0.55,0.45-1,1-1h12c0.55,0,1,0.45,1,1s-0.45,1-1,1H6C5.45,5,5,4.55,5,4z M12,7c-3.87,0-7,3.13-7,7 c0,3.87,3.13,7,7,7s7-3.13,7-7C19,10.13,15.87,7,12,7z M12,18.5c-2.49,0-4.5-2.01-4.5-4.5S9.51,9.5,12,9.5s4.5,2.01,4.5,4.5 S14.49,18.5,12,18.5z",
                }
            }
        }
    }
}

pub fn grid_view_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M5,11h4c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5v4C3,10.1,3.9,11,5,11z",
                    }
                    path {
                        d: "M5,21h4c1.1,0,2-0.9,2-2v-4c0-1.1-0.9-2-2-2H5c-1.1,0-2,0.9-2,2v4C3,20.1,3.9,21,5,21z",
                    }
                    path {
                        d: "M13,5v4c0,1.1,0.9,2,2,2h4c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2h-4C13.9,3,13,3.9,13,5z",
                    }
                    path {
                        d: "M15,21h4c1.1,0,2-0.9,2-2v-4c0-1.1-0.9-2-2-2h-4c-1.1,0-2,0.9-2,2v4C13,20.1,13.9,21,15,21z",
                    }
                }
            }
        }
    }
}

pub fn newspaper_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.67,4.33l-0.8-0.8c-0.29-0.29-0.77-0.29-1.06,0L14,4.33l-0.8-0.8c-0.29-0.29-0.77-0.29-1.06,0l-0.8,0.8l-0.8-0.8 c-0.29-0.29-0.77-0.29-1.06,0l-0.8,0.8l-0.8-0.8c-0.29-0.29-0.77-0.29-1.06,0L6,4.33l-0.8-0.8c-0.29-0.29-0.77-0.29-1.06,0 l-0.8,0.8L2.43,3.43C2.27,3.27,2,3.38,2,3.6v11.9C2,16.33,2.67,17,3.5,17h13c0.83,0,1.5-0.67,1.5-1.5V3.6 c0-0.22-0.27-0.33-0.43-0.18L16.67,4.33z M16.5,9.5h-13v-3h13V9.5z M10.75,11h5.75v1.5h-5.75V11z M3.5,11h5.75v4.5H3.5V11z M10.75,15.5V14h5.75v1.5H10.75z",
                }
            }
        }
    }
}

pub fn newspaper_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    fill: "none",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M21.15,3.85l-0.82,0.82l-0.95-0.96c-0.39-0.39-1.02-0.39-1.42,0L17,4.67l-0.96-0.96c-0.39-0.39-1.03-0.39-1.42,0 l-0.95,0.96l-0.96-0.96c-0.39-0.39-1.02-0.39-1.41,0l-0.96,0.96L9.38,3.71c-0.39-0.39-1.02-0.39-1.42,0L7,4.67L6.04,3.71 c-0.39-0.39-1.03-0.39-1.42,0L3.67,4.67L2.85,3.85C2.54,3.54,2,3.76,2,4.21V19c0,1.1,0.9,2,2,2l16,0c1.1,0,2-0.9,2-2V4.21 C22,3.76,21.46,3.54,21.15,3.85z M11,19H4v-6h7V19z M20,19h-7v-2h7V19z M20,15h-7v-2h7V15z M20,11H4V8h16V11z",
                }
            }
        }
    }
}

pub fn request_quote_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M19.41,7.41l-4.83-4.83C14.21,2.21,13.7,2,13.17,2H6C4.9,2,4.01,2.9,4.01,4L4,20c0,1.1,0.89,2,1.99,2H18c1.1,0,2-0.9,2-2 V8.83C20,8.3,19.79,7.79,19.41,7.41z M14,13c0.55,0,1,0.45,1,1v3c0,0.55-0.45,1-1,1h-1c0,0.55-0.45,1-1,1s-1-0.45-1-1h-1 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h3v-1h-3c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h1c0-0.55,0.45-1,1-1s1,0.45,1,1h1 c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1h-3v1H14z M14,8c-0.55,0-1-0.45-1-1V3.5L17.5,8H14z",
                }
            }
        }
    }
}

pub fn rule_folder_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M7.12,15.29l-1.41-1.41c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0l0.71,0.71l2.83-2.83 c0.39-0.39,1.02-0.39,1.41,0l0,0c0.39,0.39,0.39,1.02,0,1.41l-3.54,3.54C8.14,15.68,7.51,15.68,7.12,15.29z M17.41,13l0.88,0.88 c0.39,0.39,0.39,1.02,0,1.41l0,0c-0.39,0.39-1.02,0.39-1.41,0L16,14.41l-0.88,0.88c-0.39,0.39-1.02,0.39-1.41,0l0,0 c-0.39-0.39-0.39-1.02,0-1.41L14.59,13l-0.88-0.88c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0L16,11.59l0.88-0.88 c0.39-0.39,1.02-0.39,1.41,0l0,0c0.39,0.39,0.39,1.02,0,1.41L17.41,13z",
                }
            }
        }
    }
}

pub fn snippet_folder_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M15.88,10.5l1.62,1.62v3.38l-3,0v-5H15.88z M22,8v10c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2L2.01,6C2.01,4.9,2.9,4,4,4h5.17 c0.53,0,1.04,0.21,1.41,0.59L12,6h8C21.1,6,22,6.9,22,8z M19,11.91c0-0.27-0.11-0.52-0.29-0.71l-1.91-1.91 C16.61,9.11,16.35,9,16.09,9H14c-0.55,0-1,0.45-1,1v6c0,0.55,0.45,1,1,1l4,0c0.55,0,1-0.45,1-1V11.91z",
                }
            }
        }
    }
}

pub fn text_snippet_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M14.17,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V9.83c0-0.53-0.21-1.04-0.59-1.41l-4.83-4.83 C15.21,3.21,14.7,3,14.17,3L14.17,3z M8,15h8c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H8c-0.55,0-1-0.45-1-1v0C7,15.45,7.45,15,8,15z M8,11h8c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1H8c-0.55,0-1-0.45-1-1v0C7,11.45,7.45,11,8,11z M8,7h5c0.55,0,1,0.45,1,1v0 c0,0.55-0.45,1-1,1H8C7.45,9,7,8.55,7,8v0C7,7.45,7.45,7,8,7z",
                }
            }
        }
    }
}

pub fn topic_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8 C22,6.9,21.1,6,20,6z M13,16H7c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1C14,15.55,13.55,16,13,16z M17,12H7 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h10c0.55,0,1,0.45,1,1C18,11.55,17.55,12,17,12z",
                }
            }
        }
    }
}

pub fn upload_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M10 16h4c.55 0 1-.45 1-1v-5h1.59c.89 0 1.34-1.08.71-1.71L12.71 3.7c-.39-.39-1.02-.39-1.41 0L6.71 8.29c-.63.63-.19 1.71.7 1.71H9v5c0 .55.45 1 1 1zm-4 2h12c.55 0 1 .45 1 1s-.45 1-1 1H6c-.55 0-1-.45-1-1s.45-1 1-1z",
            }
        }
    }
}

pub fn upload_file_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19.41,7.41l-4.83-4.83C14.21,2.21,13.7,2,13.17,2H6C4.9,2,4.01,2.9,4.01,4L4,20c0,1.1,0.89,2,1.99,2H18c1.1,0,2-0.9,2-2 V8.83C20,8.3,19.79,7.79,19.41,7.41z M14.8,15H13v3c0,0.55-0.45,1-1,1s-1-0.45-1-1v-3H9.21c-0.45,0-0.67-0.54-0.35-0.85l2.8-2.79 c0.2-0.19,0.51-0.19,0.71,0l2.79,2.79C15.46,14.46,15.24,15,14.8,15z M14,9c-0.55,0-1-0.45-1-1V3.5L18.5,9H14z",
                }
            }
        }
    }
}

pub fn workspaces_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cx: "5",
                        cy: "14",
                        r: "3",
                    }
                    circle {
                        cy: "6",
                        cx: "10",
                        r: "3",
                    }
                    circle {
                        cx: "15",
                        cy: "14",
                        r: "3",
                    }
                }
            }
        }
    }
}

pub fn workspaces_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M6,13c-2.2,0-4,1.8-4,4s1.8,4,4,4s4-1.8,4-4S8.2,13,6,13z M12,3C9.8,3,8,4.8,8,7s1.8,4,4,4s4-1.8,4-4S14.2,3,12,3z M18,13 c-2.2,0-4,1.8-4,4s1.8,4,4,4s4-1.8,4-4S20.2,13,18,13z",
                    }
                }
            }
        }
    }
}

