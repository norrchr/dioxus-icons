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
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M20,2H4.01c-1.1,0-2,0.9-2,2L2,19.58c0,0.89,1.08,1.34,1.71,0.71L6,18h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M12,6 c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S10.9,6,12,6z M16,14H8v-0.57c0-0.81,0.48-1.53,1.22-1.85C10.07,11.21,11.01,11,12,11 c0.99,0,1.93,0.21,2.78,0.58C15.52,11.9,16,12.62,16,13.43V14z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M14 8h2v2c0 .55.45 1 1 1s1-.45 1-1V8h2c.55 0 1-.45 1-1s-.45-1-1-1h-2V4c0-.55-.45-1-1-1s-1 .45-1 1v2h-2c-.55 0-1 .45-1 1s.45 1 1 1zm5.21 7.27l-2.54-.29c-.61-.07-1.21.14-1.64.57l-1.84 1.84c-2.83-1.44-5.15-3.75-6.59-6.59l1.85-1.85c.43-.43.64-1.04.57-1.64l-.29-2.52c-.11-1.01-.97-1.78-1.98-1.78H5.02c-1.13 0-2.07.94-2 2.07.53 8.54 7.36 15.36 15.89 15.89 1.13.07 2.07-.87 2.07-2v-1.73c.01-1-.76-1.86-1.77-1.97z",
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
                d: "M12.72 2.03C6.63 1.6 1.6 6.63 2.03 12.72 2.39 18.01 7.01 22 12.31 22H16c.55 0 1-.45 1-1s-.45-1-1-1h-3.67c-3.73 0-7.15-2.42-8.08-6.03-1.49-5.8 3.91-11.21 9.71-9.71C17.58 5.18 20 8.6 20 12.33v1.1c0 .79-.71 1.57-1.5 1.57s-1.5-.78-1.5-1.57v-1.25c0-2.51-1.78-4.77-4.26-5.12-3.4-.49-6.27 2.45-5.66 5.87.34 1.91 1.83 3.49 3.72 3.94 1.84.43 3.59-.16 4.74-1.33.89 1.22 2.67 1.86 4.3 1.21 1.34-.53 2.16-1.9 2.16-3.34v-1.09c0-5.31-3.99-9.93-9.28-10.29zM12 15c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z",
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
                            r: "1.5",
                            cy: "5.5",
                        }
                    }
                    g {
                        circle {
                            r: "1.5",
                            cy: "14.5",
                            cx: "5.5",
                        }
                    }
                    g {
                        circle {
                            cy: "10",
                            r: "1.5",
                            cx: "5.5",
                        }
                    }
                    g {
                        circle {
                            cy: "5.5",
                            r: "1.5",
                            cx: "5.5",
                        }
                    }
                    g {
                        circle {
                            cy: "5.5",
                            cx: "14.5",
                            r: "1.5",
                        }
                    }
                    g {
                        polygon {
                            points: "9,14.49 9,16 10.51,16 15.4,11.1 13.9,9.6",
                        }
                    }
                    g {
                        path {
                            d: "M11.49,9.89C11.43,9.11,10.79,8.5,10,8.5c-0.83,0-1.5,0.67-1.5,1.5c0,0.79,0.61,1.43,1.39,1.49L11.49,9.89z",
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
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    circle {
                        cy: "6",
                        cx: "12",
                        r: "2",
                    }
                    circle {
                        r: "2",
                        cx: "6",
                        cy: "18",
                    }
                    circle {
                        cy: "12",
                        r: "2",
                        cx: "6",
                    }
                    circle {
                        r: "2",
                        cy: "6",
                        cx: "6",
                    }
                    circle {
                        cx: "18",
                        cy: "6",
                        r: "2",
                    }
                    path {
                        d: "M11,18.07v1.43c0,0.28,0.22,0.5,0.5,0.5h1.4c0.13,0,0.26-0.05,0.35-0.15l5.83-5.83l-2.12-2.12l-5.81,5.81 C11.05,17.81,11,17.94,11,18.07z",
                    }
                    path {
                        d: "M12.03,14L14,12.03V12c0-1.1-0.9-2-2-2s-2,0.9-2,2s0.9,2,2,2H12.03z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 7V5c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2h-8zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm0-4H4V9h2v2zm0-4H4V5h2v2zm4 12H8v-2h2v2zm0-4H8v-2h2v2zm0-4H8V9h2v2zm0-4H8V5h2v2zm9 12h-7v-2h2v-2h-2v-2h2v-2h-2V9h7c.55 0 1 .45 1 1v8c0 .55-.45 1-1 1zm-1-8h-2v2h2v-2zm0 4h-2v2h2v-2z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M19.23 15.26l-2.54-.29c-.61-.07-1.21.14-1.64.57l-1.84 1.84c-2.83-1.44-5.15-3.75-6.59-6.59l1.85-1.85c.43-.43.64-1.03.57-1.64l-.29-2.52c-.12-1.01-.97-1.77-1.99-1.77H5.03c-1.13 0-2.07.94-2 2.07.53 8.54 7.36 15.36 15.89 15.89 1.13.07 2.07-.87 2.07-2v-1.73c.01-1.01-.75-1.86-1.76-1.98z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M4.51 15.48l2-1.59c.48-.38.76-.96.76-1.57v-2.6c3.02-.98 6.29-.99 9.32 0v2.61c0 .61.28 1.19.76 1.57l1.99 1.58c.8.63 1.94.57 2.66-.15l1.22-1.22c.8-.8.8-2.13-.05-2.88-6.41-5.66-16.07-5.66-22.48 0-.85.75-.85 2.08-.05 2.88l1.22 1.22c.71.72 1.85.78 2.65.15z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M9 6c0 .56.45 1 1 1h5.59L4.7 17.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L17 8.41V14c0 .55.45 1 1 1s1-.45 1-1V6c0-.55-.45-1-1-1h-8c-.55 0-1 .45-1 1z",
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
                d: "M17.7 19.7c.39-.39.39-1.02 0-1.41l-2.7-2.7L13.59 17l2.7 2.7c.39.39 1.03.39 1.41 0zM8.71 8H11v5.59l-4.71 4.7c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0l5.3-5.3V8h2.29c.45 0 .67-.54.35-.85l-3.29-3.29c-.2-.2-.51-.2-.71 0L8.35 7.15c-.31.31-.09.85.36.85z",
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
                d: "M18.89 7.7L12 14.59 6.41 9H10c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1s1-.45 1-1v-3.59l6.29 6.29c.39.39 1.02.39 1.41 0l7.59-7.59c.39-.39.39-1.02 0-1.41-.38-.38-1.02-.38-1.4 0z",
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
                d: "M3.7 9.11l7.59 7.59c.39.39 1.02.39 1.41 0l6.3-6.3V14c0 .55.45 1 1 1s1-.45 1-1V8c0-.55-.45-1-1-1h-6c-.55 0-1 .45-1 1s.45 1 1 1h3.59L12 14.59 5.11 7.7c-.39-.39-1.02-.39-1.41 0-.38.39-.38 1.03 0 1.41z",
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
                d: "M19.3 4.71c-.39-.39-1.02-.39-1.41 0L7 15.59V10c0-.55-.45-1-1-1s-1 .45-1 1v8c0 .55.45 1 1 1h8c.55 0 1-.45 1-1s-.45-1-1-1H8.41L19.3 6.11c.38-.38.38-1.02 0-1.4z",
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
                d: "M14.85 4.85l1.44 1.44-2.88 2.88 1.42 1.42 2.88-2.88 1.44 1.44c.31.31.85.09.85-.36V4.5c0-.28-.22-.5-.5-.5h-4.29c-.45 0-.67.54-.36.85zM8.79 4H4.5c-.28 0-.5.22-.5.5v4.29c0 .45.54.67.85.35L6.29 7.7 11 12.4V19c0 .55.45 1 1 1s1-.45 1-1v-7c0-.26-.11-.52-.29-.71l-5-5.01 1.44-1.44c.31-.3.09-.84-.36-.84z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 15c0 .55-.45 1-1 1H4c-.55 0-1-.45-1-1V6c0-.55.45-1 1-1h16c.55 0 1 .45 1 1v12zm-5.71-9.3c-.39-.39-1.02-.39-1.41 0L12 10.59 10.11 8.7c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L10.59 12 8.7 13.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 13.41l1.89 1.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 12l1.89-1.89c.38-.38.38-1.02-.01-1.41z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M7.08,10.92L7.08,10.92c0.25-0.25,0.3-0.65,0.1-0.94C6.78,9.42,6.55,8.74,6.55,8s0.23-1.42,0.63-1.98 c0.2-0.29,0.15-0.69-0.1-0.94l0,0C6.75,4.75,6.19,4.8,5.93,5.19C5.37,5.99,5.05,6.96,5.05,8s0.32,2.01,0.88,2.81 C6.19,11.2,6.75,11.25,7.08,10.92z",
                    }
                    path {
                        d: "M4.9,2.91L4.9,2.91C4.59,2.59,4.07,2.61,3.79,2.95C2.67,4.33,2,6.09,2,8s0.67,3.67,1.79,5.05 c0.28,0.34,0.8,0.36,1.11,0.05l0,0c0.27-0.27,0.29-0.71,0.05-1.01C4.04,10.97,3.5,9.55,3.5,8s0.54-2.97,1.45-4.09 C5.19,3.61,5.18,3.18,4.9,2.91z",
                    }
                    path {
                        d: "M12.92,5.08L12.92,5.08c-0.25,0.25-0.3,0.65-0.1,0.94c0.4,0.56,0.63,1.25,0.63,1.98c0,0.74-0.23,1.42-0.63,1.98 c-0.2,0.29-0.15,0.69,0.1,0.94l0,0c0.33,0.33,0.89,0.28,1.15-0.11c0.55-0.8,0.88-1.77,0.88-2.81c0-1.04-0.32-2.01-0.88-2.81 C13.81,4.8,13.25,4.75,12.92,5.08z",
                    }
                    path {
                        d: "M15.1,2.9L15.1,2.9c-0.28,0.28-0.29,0.71-0.05,1.01C15.96,5.03,16.5,6.45,16.5,8c0,1.55-0.54,2.97-1.45,4.09 c-0.24,0.3-0.23,0.73,0.05,1.01l0,0c0.31,0.31,0.83,0.29,1.11-0.05C17.33,11.67,18,9.91,18,8c0-1.91-0.67-3.67-1.79-5.05 C15.93,2.61,15.41,2.59,15.1,2.9z",
                    }
                    path {
                        d: "M12,8c0-1.26-1.16-2.24-2.47-1.95C8.81,6.22,8.22,6.8,8.06,7.52C7.86,8.34,8.18,9.09,8.75,9.55L6.3,17.07 C6.15,17.53,6.5,18,6.98,18h0c0.31,0,0.58-0.2,0.68-0.49l0.33-1.01h3.6l0.27,0.98c0.09,0.31,0.37,0.52,0.69,0.52h0.02 c0.47,0,0.81-0.45,0.69-0.9l-2.06-7.5C11.68,9.23,12,8.66,12,8z M8.47,15l1.46-4.5l1.24,4.5H8.47z",
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
                        d: "M7.9,14.1l0.09-0.09c0.27-0.27,0.32-0.71,0.08-1.01C7.36,12.09,7,11.01,7,10c0-1.08,0.35-2.16,1.04-3.01 c0.25-0.3,0.21-0.75-0.07-1.02L7.9,5.9C7.56,5.56,7,5.6,6.7,5.98C5.79,7.16,5.3,8.58,5.3,10c0,1.42,0.49,2.84,1.4,4.02 C7,14.4,7.56,14.44,7.9,14.1z",
                    }
                    path {
                        d: "M18.51,3.49l-0.08,0.08c-0.3,0.3-0.29,0.76-0.03,1.08c1.26,1.53,1.9,3.48,1.9,5.35c0,1.87-0.63,3.81-1.9,5.35 c-0.28,0.33-0.23,0.83,0.08,1.14v0c0.35,0.35,0.93,0.31,1.24-0.07C21.29,14.54,22,12.31,22,10c0-2.32-0.79-4.55-2.31-6.43 C19.39,3.2,18.84,3.16,18.51,3.49z",
                    }
                    path {
                        d: "M5.57,3.57L5.49,3.49C5.16,3.16,4.61,3.2,4.31,3.57C2.79,5.45,2,7.68,2,10c0,2.32,0.79,4.55,2.31,6.43 c0.3,0.37,0.85,0.42,1.18,0.08l0.08-0.08c0.3-0.3,0.29-0.76,0.03-1.08C4.33,13.81,3.7,11.87,3.7,10c0-1.87,0.63-3.81,1.9-5.35 C5.86,4.33,5.87,3.87,5.57,3.57z",
                    }
                    path {
                        d: "M16.07,14.07c0.36,0.36,0.95,0.32,1.26-0.09c0.9-1.18,1.37-2.58,1.37-3.98c-0.08-1.41-0.51-2.83-1.4-4.01 c-0.29-0.39-0.86-0.43-1.2-0.09l-0.08,0.08c-0.27,0.27-0.32,0.71-0.08,1.01C16.64,7.91,17,8.99,17,10c0,1.07-0.34,2.13-1.01,2.98 C15.73,13.3,15.77,13.77,16.07,14.07L16.07,14.07z",
                    }
                    path {
                        d: "M14.5,10c0-1.6-1.51-2.85-3.18-2.41c-0.8,0.21-1.46,0.85-1.7,1.65c-0.32,1.06,0.06,2.04,0.76,2.64l-2.96,8.87 C7.21,21.37,7.67,22,8.32,22h0c0.41,0,0.77-0.26,0.9-0.65L9.67,20h4.67l0.45,1.35c0.13,0.39,0.49,0.65,0.9,0.65h0 c0.65,0,1.1-0.63,0.9-1.25l-2.96-8.87C14.16,11.42,14.5,10.76,14.5,10z M10.33,18L12,13l1.67,5H10.33z",
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
                    width: "24",
                    height: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M20.29,7.68L7.7,20.29C7.07,20.92,7.52,22,8.41,22H21c0.55,0,1-0.45,1-1V8.39C22,7.5,20.92,7.05,20.29,7.68z M20,20h-2 v-7.22l2-2V20z",
                    }
                    path {
                        d: "M9.61,10.68c-0.28,0.17-0.32,0.56-0.09,0.79l0.82,0.82c0.39,0.39,1.02,0.39,1.41,0l0.82-0.82 c0.23-0.23,0.18-0.62-0.09-0.79C11.61,10.14,10.49,10.14,9.61,10.68z",
                    }
                    path {
                        d: "M8.42,9.3c1.57-1.12,3.7-1.12,5.27,0c0.36,0.26,0.85,0.22,1.16-0.1c0.39-0.39,0.35-1.06-0.1-1.38 c-2.2-1.57-5.19-1.57-7.4,0C6.9,8.14,6.85,8.81,7.25,9.2C7.57,9.52,8.06,9.56,8.42,9.3z",
                    }
                    path {
                        d: "M16.26,6.69c0.34,0.28,0.83,0.28,1.14-0.03l0.12-0.12c0.35-0.35,0.31-0.92-0.08-1.24c-3.67-3.05-9.02-3.07-12.7-0.06 C4.31,5.59,4.27,6.23,4.66,6.61C4.98,6.94,5.5,6.98,5.85,6.69C8.86,4.21,13.25,4.21,16.26,6.69z",
                    }
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM7 9h10c.55 0 1 .45 1 1s-.45 1-1 1H7c-.55 0-1-.45-1-1s.45-1 1-1zm6 5H7c-.55 0-1-.45-1-1s.45-1 1-1h6c.55 0 1 .45 1 1s-.45 1-1 1zm4-6H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1z",
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
                    enable_background: "new",
                    path {
                        d: "M20 4v12H5.17L4 17.17V4h16m0-2H4c-1.1 0-2 .9-2 2v15.59c0 .89 1.08 1.34 1.71.71L6 18h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2z",
                    }
                }
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
                d: "M6 13h12c.55 0 1-.45 1-1s-.45-1-1-1H6c-.55 0-1 .45-1 1s.45 1 1 1zm-2 4h12c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1 .45-1 1s.45 1 1 1zm3-9c0 .55.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1H8c-.55 0-1 .45-1 1z",
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
                d: "M21.99 4c0-1.1-.89-2-1.99-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h14l4 4-.01-18zM17 14H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1zm0-3H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1zm0-3H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1z",
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
                d: "M17.25,17.25L2.75,2.75c-0.29-0.29-0.77-0.29-1.06,0c-0.29,0.29-0.29,0.77,0,1.06L2,4.12v9.38C2,14.33,2.67,15,3.5,15h9.38 l3.31,3.31c0.29,0.29,0.77,0.29,1.06,0S17.54,17.54,17.25,17.25z M5.75,9.25C5.34,9.25,5,8.91,5,8.5c0-0.38,0.28-0.69,0.64-0.74 l1.49,1.49H5.75z M8.75,12C8.34,12,8,11.66,8,11.25c0-0.31,0.18-0.57,0.44-0.68L9.88,12H8.75z M15,5.75C15,5.34,14.66,5,14.25,5 c0,0-7.13,0-7.13,0l-3-3H16.5C17.33,2,18,2.67,18,3.5v12.38L14.12,12h0.13c0.41,0,0.75-0.34,0.75-0.75s-0.34-0.75-0.75-0.75h-1.63 l-1.25-1.25h2.88C14.66,9.25,15,8.91,15,8.5s-0.34-0.75-0.75-0.75H9.87L8.62,6.5h5.63C14.66,6.5,15,6.16,15,5.75z",
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
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M1.39,2.81C1,3.2,1,3.83,1.39,4.22L2,4.83V16c0,1.1,0.9,2,2,2h11.17l4.61,4.61c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41L2.81,2.81C2.42,2.42,1.78,2.42,1.39,2.81z M6.38,9.21L8.17,11H7c-0.55,0-1-0.45-1-1 C6,9.68,6.15,9.4,6.38,9.21z M7,14c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h2.17l2,2H7z M14.83,12l-1-1H17c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1h-5.17l-1-1H17c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H8.83l-4-4H20c1.1,0,2,0.9,2,2v15.17L16.83,14H17 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H14.83z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M19 0H5c-.55 0-1 .45-1 1s.45 1 1 1h14c.55 0 1-.45 1-1s-.45-1-1-1zM5 24h14c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1 .45-1 1s.45 1 1 1zM20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-8 2.75c1.24 0 2.25 1.01 2.25 2.25s-1.01 2.25-2.25 2.25S9.75 10.24 9.75 9 10.76 6.75 12 6.75zM17 17H7v-1.5c0-1.67 3.33-2.5 5-2.5s5 .83 5 2.5V17z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M18.5,3h-17C0.67,3,0,3.67,0,4.5v11C0,16.33,0.67,17,1.5,17h17c0.83,0,1.5-0.67,1.5-1.5v-11C20,3.67,19.33,3,18.5,3z M7,7 c1.38,0,2.5,1.12,2.5,2.5S8.38,12,7,12s-2.5-1.12-2.5-2.5S5.62,7,7,7z M1.64,15.5C2.93,13.97,4.85,13,7,13s4.07,0.97,5.36,2.5H1.64 z M17.17,9.75L17.17,9.75c-0.14,0.24-0.44,0.32-0.68,0.18L15.5,9.37v1.13c0,0.28-0.22,0.5-0.5,0.5h0c-0.28,0-0.5-0.22-0.5-0.5V9.37 l-0.98,0.57c-0.24,0.14-0.54,0.06-0.68-0.18l0,0c-0.14-0.24-0.06-0.54,0.18-0.68L14,8.5l-0.98-0.57c-0.24-0.14-0.32-0.44-0.18-0.68 l0,0c0.14-0.24,0.44-0.32,0.68-0.18l0.98,0.57V6.5C14.5,6.22,14.72,6,15,6h0c0.28,0,0.5,0.22,0.5,0.5v1.13l0.98-0.57 c0.24-0.14,0.54-0.06,0.68,0.18v0c0.14,0.24,0.06,0.54-0.18,0.68L16,8.5l0.98,0.57C17.22,9.21,17.3,9.51,17.17,9.75z",
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
                path {
                    d: "M22,3H2C0.9,3,0,3.9,0,5v14c0,1.1,0.9,2,2,2h20c1.1,0,1.99-0.9,1.99-2L24,5C24,3.9,23.1,3,22,3z M9,8c1.65,0,3,1.35,3,3 s-1.35,3-3,3s-3-1.35-3-3S7.35,8,9,8z M2.08,19c1.38-2.39,3.96-4,6.92-4s5.54,1.61,6.92,4H2.08z M20.6,10.5L20.6,10.5 c-0.21,0.36-0.67,0.48-1.02,0.27l-0.82-0.48v0.95c0,0.41-0.34,0.75-0.75,0.75h0c-0.41,0-0.75-0.34-0.75-0.75V10.3l-0.82,0.48 c-0.36,0.21-0.82,0.08-1.02-0.27l0,0c-0.21-0.36-0.08-0.82,0.27-1.02L16.5,9l-0.82-0.48c-0.36-0.21-0.48-0.67-0.27-1.02l0,0 c0.21-0.36,0.67-0.48,1.02-0.27l0.82,0.48V6.75C17.25,6.34,17.59,6,18,6h0c0.41,0,0.75,0.34,0.75,0.75V7.7l0.82-0.48 c0.36-0.21,0.82-0.08,1.02,0.27v0c0.21,0.36,0.08,0.82-0.27,1.02L19.5,9l0.82,0.48C20.68,9.68,20.81,10.14,20.6,10.5z",
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
                fill: "none",
                d: "M0 0h24v24H0z",
            }
            path {
                d: "M21 8V7l-3 2-3-2v1l2.72 1.82c.17.11.39.11.55 0L21 8zm1-5H2C.9 3 0 3.9 0 5v14c0 1.1.9 2 2 2h20c1.1 0 1.99-.9 1.99-2L24 5c0-1.1-.9-2-2-2zM8 6c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H2v-1c0-2 4-3.1 6-3.1s6 1.1 6 3.1v1zm7.5-6h-7c-.28 0-.5-.22-.5-.5v-5c0-.28.22-.5.5-.5h7c.28 0 .5.22.5.5v5c0 .28-.22.5-.5.5z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M22 3H2C.9 3 0 3.9 0 5v14c0 1.1.9 2 2 2h20c1.1 0 1.99-.9 1.99-2L24 5c0-1.1-.9-2-2-2zM8 6c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H2v-1c0-2 4-3.1 6-3.1s6 1.1 6 3.1v1zm3.85-4h1.39c.16 0 .3.07.4.2l1.1 1.45c.15.2.13.48-.05.65l-1.36 1.36c-.18.18-.48.2-.67.04-1.13-.96-1.97-2.25-2.38-3.71-.18-.63-.28-1.3-.28-1.99s.1-1.36.28-2c.41-1.47 1.25-2.75 2.38-3.71.2-.17.49-.14.67.04l1.36 1.36c.18.18.2.46.05.65l-1.1 1.45c-.09.13-.24.2-.4.2h-1.39c-.22.63-.35 1.3-.35 2s.13 1.38.35 2.01z",
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M17.5,3h-15C1.67,3,1,3.67,1,4.5v6.75C1,11.66,1.34,12,1.75,12h0c0.41,0,0.75-0.34,0.75-0.75V4.5h15V17 c0.83,0,1.5-0.67,1.5-1.5v-11C19,3.67,18.33,3,17.5,3z",
                    }
                    circle {
                        r: "3",
                        cx: "7",
                        cy: "9",
                    }
                    path {
                        d: "M12.03,14.37C10.56,13.5,8.84,13,7,13s-3.56,0.5-5.03,1.37C1.36,14.73,1,15.39,1,16.09l0,0.41C1,17.33,1.67,18,2.5,18h9 c0.83,0,1.5-0.67,1.5-1.5l0-0.41C13,15.39,12.64,14.73,12.03,14.37z",
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
                            d: "M21,3H3C1.9,3,1,3.9,1,5v8h2V5h18v16c1.1,0,2-0.9,2-2V5C23,3.9,22.1,3,21,3z",
                        }
                    }
                    g {
                        circle {
                            r: "4",
                            cx: "9",
                            cy: "10",
                        }
                    }
                    g {
                        path {
                            d: "M15.39,16.56C13.71,15.7,11.53,15,9,15c-2.53,0-4.71,0.7-6.39,1.56C1.61,17.07,1,18.1,1,19.22V22h16v-2.78 C17,18.1,16.39,17.07,15.39,16.56z",
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
                d: "M.31 2c-.39.39-.39 1.02 0 1.41l.69.68V16c0 1.1.9 2 2 2h7v2H9c-.55 0-1 .45-1 1s.45 1 1 1h6c.55 0 1-.45 1-1s-.45-1-1-1h-1v-2h.9l5.29 5.29c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L1.72 2C1.33 1.61.7 1.61.31 2zm2.68 13V6.09L12.9 16H3.99c-.55 0-1-.45-1-1zM4.55 2l2 2H20c.55 0 1 .45 1 1v10c0 .55-.45 1-1 1h-1.45l2 2h.44c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2H4.55z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M16.5 8c.28 0 .5-.22.5-.5v-4c0-.28-.22-.5-.5-.5s-.5.22-.5.5v4c0 .28.22.5.5.5zm-4-1c-.28 0-.5.22-.5.5s.22.5.5.5h1.95c.3 0 .55-.25.55-.55v-1.9c0-.3-.25-.55-.55-.55H13V4h1.5c.28 0 .5-.22.5-.5s-.22-.5-.5-.5h-1.95c-.3 0-.55.25-.55.55v1.89c0 .31.25.56.55.56H14v1h-1.5zm7.95-4h-1.89c-.31 0-.56.25-.56.55V7.5c0 .28.22.5.5.5s.5-.22.5-.5V6h1.45c.3 0 .55-.25.55-.55v-1.9c0-.3-.25-.55-.55-.55zM20 5h-1V4h1v1zm-.79 10.27l-2.54-.29c-.61-.07-1.21.14-1.64.57l-1.84 1.84c-2.83-1.44-5.15-3.75-6.59-6.59l1.85-1.85c.43-.43.64-1.04.57-1.64l-.29-2.52c-.11-1.01-.97-1.78-1.98-1.78H5.02c-1.13 0-2.07.94-2 2.07.53 8.54 7.36 15.36 15.89 15.89 1.13.07 2.07-.87 2.07-2v-1.73c.01-1-.76-1.86-1.77-1.97z",
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
                width: "20",
                height: "20",
                fill: "none",
            }
            path {
                d: "M6,1.75C6,2.16,5.66,2.5,5.25,2.5H3.5v1.75C3.5,4.66,3.16,5,2.75,5C2.34,5,2,4.66,2,4.25v-2.5C2,1.34,2.34,1,2.75,1h2.5 C5.66,1,6,1.34,6,1.75z M14,1.75C14,1.34,14.34,1,14.75,1l2.5,0C17.66,1,18,1.34,18,1.75v2.5C18,4.66,17.66,5,17.25,5 c-0.41,0-0.75-0.34-0.75-0.75V2.5l-1.75,0C14.34,2.5,14,2.16,14,1.75z M17.25,15c0.41,0,0.75,0.34,0.75,0.75v2.5 c0,0.41-0.34,0.75-0.75,0.75h-2.5C14.34,19,14,18.66,14,18.25s0.34-0.75,0.75-0.75h1.75v-1.75C16.5,15.34,16.84,15,17.25,15z M2.75,15c0.41,0,0.75,0.34,0.75,0.75l0,1.75l1.75,0C5.66,17.5,6,17.84,6,18.25C6,18.66,5.66,19,5.25,19l-2.5,0 C2.34,19,2,18.66,2,18.25l0-2.5C2,15.34,2.34,15,2.75,15z M13.57,4H6.43C5.64,4,5,4.67,5,5.5v9C5,15.33,5.64,16,6.43,16h7.14 c0.79,0,1.43-0.67,1.43-1.5v-9C15,4.67,14.36,4,13.57,4z M8,12.25c0-0.41,0.34-0.75,0.75-0.75h2.5c0.41,0,0.75,0.34,0.75,0.75 S11.66,13,11.25,13h-2.5C8.34,13,8,12.66,8,12.25z M8,10c0-0.41,0.34-0.75,0.75-0.75h2.5C11.66,9.25,12,9.59,12,10 s-0.34,0.75-0.75,0.75h-2.5C8.34,10.75,8,10.41,8,10z M8,7.75C8,7.34,8.34,7,8.75,7h2.5C11.66,7,12,7.34,12,7.75 S11.66,8.5,11.25,8.5h-2.5C8.34,8.5,8,8.16,8,7.75z",
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
                fill: "none",
                height: "24",
                width: "24",
            }
            path {
                d: "M3,6C2.45,6,2,5.55,2,5V2c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1S6.55,3,6,3H4v2C4,5.55,3.55,6,3,6z M17,2 c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V2c0-0.55-0.45-1-1-1h-3C17.45,1,17,1.45,17,2z M3,18c-0.55,0-1,0.45-1,1v3 c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H4v-2C4,18.45,3.55,18,3,18z M17,22c0,0.55,0.45,1,1,1h3 c0.55,0,1-0.45,1-1v-3c0-0.55-0.45-1-1-1s-1,0.45-1,1v2h-2C17.45,21,17,21.45,17,22z M19,18c0,1.1-0.9,2-2,2H7c-1.1,0-2-0.9-2-2V6 c0-1.1,0.9-2,2-2h10c1.1,0,2,0.9,2,2V18z M9,9c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,8,9,8.45,9,9z M9,12c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,11,9,11.45,9,12z M9,15c0,0.55,0.45,1,1,1h4 c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-4C9.45,14,9,14.45,9,15z",
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
                d: "M.71 2.39c-.39.39-.39 1.02 0 1.41L2 5.1V19c0 1.1.9 2 2 2h13.9l2.29 2.29c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L2.12 2.39C1.73 2 1.1 2 .71 2.39zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm-2-4V9h2v2H4zm6 8H8v-2h2v2zm-2-4v-2h2v2H8zm4 4v-2h1.9l2 2H12zM8 5h2v2h-.45L12 9.45V9h7c.55 0 1 .45 1 1v7.45l2 2V9c0-1.1-.9-2-2-2h-8V5c0-1.1-.9-2-2-2H5.55L8 5.45V5zm8 6h2v2h-2z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M15,4H5C4.45,4,4,4.45,4,5v10c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V5C16,4.45,15.55,4,15,4z M15,14.5 c0,0.28-0.22,0.5-0.5,0.5h-9C5.22,15,5,14.78,5,14.5V7h10V14.5z",
                    }
                    path {
                        d: "M8.59,13.12c0.2,0.2,0.51,0.2,0.71,0l3.54-3.54c0.2-0.2,0.2-0.51,0-0.71s-0.51-0.2-0.71,0l-3.18,3.18L7.88,11 c-0.2-0.2-0.51-0.2-0.71,0s-0.2,0.51,0,0.71L8.59,13.12z",
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
                        d: "M10.23,15.83c0.39,0.39,1.02,0.39,1.41,0l4.24-4.24c0.39-0.39,0.39-1.02,0-1.42v0c-0.39-0.39-1.02-0.39-1.41,0l-3.54,3.53 l-1.41-1.41c-0.39-0.39-1.02-0.39-1.42,0s-0.39,1.02,0,1.41L10.23,15.83z",
                    }
                    path {
                        d: "M19,4H5C3.89,4,3,4.9,3,6v12c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V6C21,4.9,20.11,4,19,4z M19,17c0,0.55-0.45,1-1,1H6 c-0.55,0-1-0.45-1-1V8h14V17z",
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
                d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-.4 4.25l-7.07 4.42c-.32.2-.74.2-1.06 0L4.4 8.25c-.25-.16-.4-.43-.4-.72 0-.67.73-1.07 1.3-.72L12 11l6.7-4.19c.57-.35 1.3.05 1.3.72 0 .29-.15.56-.4.72z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M20 6h-1v8c0 .55-.45 1-1 1H6v1c0 1.1.9 2 2 2h10l4 4V8c0-1.1-.9-2-2-2zm-3 5V4c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v13l4-4h9c1.1 0 2-.9 2-2z",
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
                    d: "M15,12.5l3,3l-3,3V16h-2.5c-0.28,0-0.5-0.22-0.5-0.5v0c0-0.28,0.22-0.5,0.5-0.5H15V12.5z M11,16H4c-0.55,0-1-0.45-1-1V5 c0-0.55,0.45-1,1-1h12c0.55,0,1,0.45,1,1v7h-1V6.18l-5.46,3.48c-0.33,0.21-0.75,0.21-1.07,0L4,6.18V15h7L11,16z M4,5l6,3.82L16,5H4 z",
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
                    height: "24",
                    width: "24",
                    fill: "none",
                }
                path {
                    d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h9v-2H4V8l6.94,4.34c0.65,0.41,1.47,0.41,2.12,0L20,8v5h2V6C22,4.9,21.1,4,20,4 z M12,11L4,6h16L12,11z M19,16.21c0-0.45,0.54-0.67,0.85-0.35l2.79,2.79c0.2,0.2,0.2,0.51,0,0.71l-2.79,2.79 C19.54,22.46,19,22.24,19,21.79V20h-3c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h3V16.21z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M7,17h6c0.55,0,1-0.45,1-1v-2.59c0-0.27-0.11-0.52-0.29-0.71L11,10l2.71-2.71C13.89,7.11,14,6.85,14,6.59V4 c0-0.55-0.45-1-1-1H7C6.45,3,6,3.45,6,4v2.59c0,0.27,0.11,0.52,0.29,0.71L9,10l-2.71,2.71C6.11,12.89,6,13.15,6,13.41V16 C6,16.55,6.45,17,7,17z M7,6.59V4.5C7,4.22,7.22,4,7.5,4h5C12.78,4,13,4.22,13,4.5v2.09l-3,3L7,6.59z",
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
                path {
                    d: "M16,22c1.1,0,2-0.9,2-2l-0.01-3.18c0-0.53-0.21-1.03-0.58-1.41L14,12l3.41-3.43c0.37-0.37,0.58-0.88,0.58-1.41L18,4 c0-1.1-0.9-2-2-2H8C6.9,2,6,2.9,6,4v3.16C6,7.69,6.21,8.2,6.58,8.58L10,12l-3.41,3.4C6.21,15.78,6,16.29,6,16.82V20 c0,1.1,0.9,2,2,2H16z M8,7.09V5c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1v2.09c0,0.27-0.11,0.52-0.29,0.71L12,11.5L8.29,7.79 C8.11,7.61,8,7.35,8,7.09z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M13,3H7C6.45,3,6,3.45,6,4v2.59c0,0.27,0.11,0.52,0.29,0.71L9,10l-2.71,2.71C6.11,12.89,6,13.15,6,13.41V16 c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-2.59c0-0.27-0.11-0.52-0.29-0.71L11,10l2.71-2.71C13.89,7.11,14,6.85,14,6.59V4 C14,3.45,13.55,3,13,3z M13,13.41v2.09c0,0.28-0.22,0.5-0.5,0.5h-5C7.22,16,7,15.78,7,15.5v-2.09l3-3L13,13.41z",
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
                    d: "M8,2C6.9,2,6,2.9,6,4l0.01,3.18c0,0.53,0.21,1.03,0.58,1.41L10,12l-3.41,3.43c-0.37,0.37-0.58,0.88-0.58,1.41L6,20 c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-3.16c0-0.53-0.21-1.04-0.58-1.41L14,12l3.41-3.4C17.79,8.22,18,7.71,18,7.18V4 c0-1.1-0.9-2-2-2H8z M16,16.91V19c0,0.55-0.45,1-1,1H9c-0.55,0-1-0.45-1-1v-2.09c0-0.27,0.11-0.52,0.29-0.71L12,12.5l3.71,3.71 C15.89,16.39,16,16.65,16,16.91z",
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
                fill: "none",
                width: "20",
                height: "20",
            }
            path {
                d: "M12.71,12.84c0.84-0.88,1.17-2.06,0.99-3.18l1.59-0.48c0.42,0.78,1.25,1.32,2.2,1.32c1.38,0,2.5-1.12,2.5-2.5 s-1.12-2.5-2.5-2.5C16.12,5.5,15,6.62,15,8c0,0.08,0,0.15,0.01,0.22L13.42,8.7c-0.52-1.15-1.61-2-2.92-2.17l0-1.58 c1.14-0.23,2-1.24,2-2.45C12.5,1.12,11.38,0,10,0S7.5,1.12,7.5,2.5c0,1.21,0.86,2.22,2,2.45l0,1.58C8.2,6.71,7.11,7.55,6.58,8.7 L4.99,8.22C5,8.15,5,8.08,5,8c0-1.38-1.12-2.5-2.5-2.5S0,6.62,0,8s1.12,2.5,2.5,2.5c0.95,0,1.78-0.53,2.2-1.32L6.3,9.66 c-0.18,1.12,0.15,2.3,0.99,3.18l-1.15,1.43C5.8,14.1,5.41,14,5,14c-1.38,0-2.5,1.12-2.5,2.5S3.62,19,5,19s2.5-1.12,2.5-2.5 c0-0.61-0.22-1.17-0.58-1.6l1.15-1.43c1.18,0.71,2.68,0.71,3.86,0l1.15,1.43c-0.36,0.43-0.58,0.99-0.58,1.6c0,1.38,1.12,2.5,2.5,2.5 s2.5-1.12,2.5-2.5S16.38,14,15,14c-0.41,0-0.8,0.1-1.14,0.27L12.71,12.84z",
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
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M8.4,18.2C8.78,18.7,9,19.32,9,20c0,1.66-1.34,3-3,3s-3-1.34-3-3s1.34-3,3-3c0.44,0,0.85,0.09,1.23,0.26l1.41-1.77 c-0.92-1.03-1.29-2.39-1.09-3.69l-2.03-0.68C4.98,11.95,4.06,12.5,3,12.5c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3 c0,0.07,0,0.14-0.01,0.21l2.03,0.68c0.64-1.21,1.82-2.09,3.22-2.32l0-2.16C9.96,5.57,9,4.4,9,3c0-1.66,1.34-3,3-3s3,1.34,3,3 c0,1.4-0.96,2.57-2.25,2.91v2.16c1.4,0.23,2.58,1.11,3.22,2.32l2.03-0.68C18,9.64,18,9.57,18,9.5c0-1.66,1.34-3,3-3s3,1.34,3,3 s-1.34,3-3,3c-1.06,0-1.98-0.55-2.52-1.37l-2.03,0.68c0.2,1.29-0.16,2.65-1.09,3.69l1.41,1.77C17.15,17.09,17.56,17,18,17 c1.66,0,3,1.34,3,3s-1.34,3-3,3s-3-1.34-3-3c0-0.68,0.22-1.3,0.6-1.8l-1.41-1.77c-1.35,0.75-3.01,0.76-4.37,0L8.4,18.2z",
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
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M17.5 4.5c-1.95 0-4.05.4-5.5 1.5-1.45-1.1-3.55-1.5-5.5-1.5-1.45 0-2.99.22-4.28.79C1.49 5.62 1 6.33 1 7.14v11.28c0 1.3 1.22 2.26 2.48 1.94.98-.25 2.02-.36 3.02-.36 1.56 0 3.22.26 4.56.92.6.3 1.28.3 1.87 0 1.34-.67 3-.92 4.56-.92 1 0 2.04.11 3.02.36 1.26.33 2.48-.63 2.48-1.94V7.14c0-.81-.49-1.52-1.22-1.85-1.28-.57-2.82-.79-4.27-.79zM21 17.23c0 .63-.58 1.09-1.2.98-.75-.14-1.53-.2-2.3-.2-1.7 0-4.15.65-5.5 1.5V8c1.35-.85 3.8-1.5 5.5-1.5.92 0 1.83.09 2.7.28.46.1.8.51.8.98v9.47z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M8.65 3.35L5.86 6.14c-.32.31-.1.85.35.85H8V13c0 .55.45 1 1 1s1-.45 1-1V6.99h1.79c.45 0 .67-.54.35-.85L9.35 3.35c-.19-.19-.51-.19-.7 0zM16 17.01V11c0-.55-.45-1-1-1s-1 .45-1 1v6.01h-1.79c-.45 0-.67.54-.35.85l2.79 2.78c.2.19.51.19.71 0l2.79-2.78c.32-.31.09-.85-.35-.85H16z",
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
                height: "20",
                fill: "none",
            }
            path {
                d: "M7.08,4.96l2.56-2.6c0.2-0.2,0.52-0.2,0.71,0l4.17,4.23l0,0c1.07,1.1,1.72,2.6,1.72,4.24c0,0.96-0.23,1.86-0.62,2.67 L10,7.88V4.14L8.14,6.02L7.08,4.96z M15.48,17.6l-1.8-1.8C12.65,16.55,11.38,17,10,17c-3.45,0-6.25-2.76-6.25-6.16 c0-1.39,0.47-2.67,1.26-3.7L2.4,4.52c-0.29-0.29-0.29-0.77,0-1.06l0,0c0.29-0.29,0.77-0.29,1.06,0l13.08,13.08 c0.29,0.29,0.29,0.77,0,1.06l0,0C16.25,17.89,15.77,17.89,15.48,17.6z M10,12.12L6.09,8.21c-0.54,0.77-0.84,1.68-0.84,2.63 c0,2.57,2.13,4.66,4.75,4.66V12.12z",
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
                height: "24",
                width: "24",
                fill: "none",
            }
            path {
                d: "M20.49,20.49L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l3.5,3.5c-1,1.31-1.6,2.94-1.6,4.7 C4,17.48,7.58,21,12,21c1.75,0,3.36-0.56,4.67-1.5l2.4,2.4c0.39,0.39,1.02,0.39,1.41,0l0,0C20.88,21.51,20.88,20.88,20.49,20.49z M12,19c-3.31,0-6-2.63-6-5.87c0-1.19,0.36-2.32,1.02-3.28L12,14.83V19z M8.38,5.56l2.91-2.87c0.39-0.38,1.01-0.38,1.4,0l4.95,4.87 l0,0C19.1,8.99,20,10.96,20,13.13c0,1.18-0.27,2.29-0.74,3.3L12,9.17V4.81L9.8,6.97L8.38,5.56z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M17.29,8.5h-6.55c-0.73-2.31-3.07-3.91-5.72-3.41c-1.93,0.37-3.5,1.9-3.91,3.82C0.43,12.15,2.88,15,6,15 c2.24,0,4.11-1.48,4.75-3.5h0.75l1.15,1.15c0.2,0.2,0.51,0.2,0.71,0l1.15-1.15l1.15,1.15c0.2,0.2,0.51,0.2,0.71,0l2.29-2.29 c0.2-0.2,0.2-0.51,0-0.71l-1-1C17.55,8.55,17.43,8.5,17.29,8.5z M6,12.5c-1.38,0-2.5-1.12-2.5-2.5S4.62,7.5,6,7.5S8.5,8.62,8.5,10 S7.38,12.5,6,12.5z",
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
                rect {
                    fill: "none",
                    width: "24",
                    height: "24",
                }
            }
            g {
                path {
                    d: "M20.59,10h-7.94C11.7,7.31,8.89,5.5,5.77,6.12c-2.29,0.46-4.15,2.3-4.63,4.58C0.32,14.58,3.26,18,7,18 c2.61,0,4.83-1.67,5.65-4H13l1.29,1.29c0.39,0.39,1.02,0.39,1.41,0L17,14l1.29,1.29c0.39,0.39,1.03,0.39,1.42,0l2.59-2.61 c0.39-0.39,0.39-1.03-0.01-1.42l-0.99-0.97C21.1,10.1,20.85,10,20.59,10z M7,15c-1.65,0-3-1.35-3-3c0-1.65,1.35-3,3-3s3,1.35,3,3 C10,13.65,8.65,15,7,15z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                path {
                    d: "M10.62,8.5l3.44,3.44l0.44-0.44h0.01l1.15,1.15c0.2,0.2,0.51,0.2,0.71,0l2.29-2.29c0.2-0.2,0.2-0.51,0-0.71l-1-1 c-0.11-0.1-0.23-0.15-0.37-0.15H10.62z M2.4,3.46c0.29-0.29,0.77-0.29,1.06,0l13.08,13.08c0.29,0.29,0.29,0.77,0,1.06 s-0.77,0.29-1.06,0l-5.15-5.15c-1,1.77-3.03,2.88-5.29,2.45c-1.95-0.36-3.55-1.93-3.94-3.88C0.65,8.75,1.75,6.67,3.53,5.65 L2.4,4.52C2.11,4.23,2.11,3.75,2.4,3.46z M5.44,7.56C4.33,7.82,3.5,8.81,3.5,10c0,1.38,1.12,2.5,2.5,2.5 c1.19,0,2.18-0.83,2.44-1.94L5.44,7.56z",
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
                rect {
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                path {
                    d: "M12.83,10l4.09,4.09L17,14l1.29,1.29c0.39,0.39,1.03,0.39,1.42,0l2.59-2.61c0.39-0.39,0.39-1.03-0.01-1.42l-0.99-0.97 C21.1,10.1,20.85,10,20.59,10H12.83z M19.07,21.9c0.39,0.39,1.02,0.39,1.41,0s0.39-1.02,0-1.41L3.51,3.51 c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l1.88,1.88C2.2,7.85,1,9.79,1,12c0,3.31,2.69,6,6,6 c2.21,0,4.15-1.2,5.18-2.99L19.07,21.9z M9.91,12.74C9.58,14.03,8.4,15,7,15c-1.65,0-3-1.35-3-3c0-1.4,0.97-2.58,2.26-2.91 L9.91,12.74z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M12 9h4c.55 0 1-.45 1-1s-.45-1-1-1h-4c-.55 0-1 .45-1 1s.45 1 1 1zm0 4h4c.55 0 1-.45 1-1s-.45-1-1-1h-4c-.55 0-1 .45-1 1s.45 1 1 1zm0 4h4c.55 0 1-.45 1-1s-.45-1-1-1h-4c-.55 0-1 .45-1 1s.45 1 1 1zM7 7h2v2H7zm0 4h2v2H7zm0 4h2v2H7zM20 3H4c-.55 0-1 .45-1 1v16c0 .55.45 1 1 1h16c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1zm-1 16H5V5h14v14z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M19 2H5c-1.11 0-2 .9-2 2v14c0 1.1.9 2 2 2h4l2.29 2.29c.39.39 1.02.39 1.41 0L15 20h4c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-6 16h-2v-2h2v2zm2.07-7.75-.9.92c-.58.59-.99 1.1-1.12 2.06-.06.43-.41.76-.85.76h-.31c-.52 0-.92-.46-.85-.98.11-.91.53-1.72 1.14-2.34l1.24-1.26c.36-.36.58-.86.58-1.41 0-1.1-.9-2-2-2-.87 0-1.62.57-1.89 1.35-.13.37-.44.64-.83.64h-.3c-.58 0-.98-.56-.82-1.12C8.65 5.21 10.18 4 12 4c2.21 0 4 1.79 4 4 0 .88-.36 1.68-.93 2.25z",
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
                d: "M2.71 3.56c-.39.39-.39 1.02 0 1.41l2.47 2.47C5.07 7.95 5 8.47 5 9c0 4.17 4.42 9.92 6.23 12.11.4.48 1.13.48 1.53 0 .65-.78 1.62-2.01 2.61-3.46l2.65 2.65c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L4.12 3.56c-.39-.39-1.02-.39-1.41 0zM12 2c-1.84 0-3.5.71-4.75 1.86l3.19 3.19c.43-.34.97-.55 1.56-.55 1.38 0 2.5 1.12 2.5 2.5 0 .59-.21 1.13-.56 1.56l3.55 3.55C18.37 12.36 19 10.57 19 9c0-3.87-3.13-7-7-7z",
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
                    d: "M12,2c-4.2,0-8,3.22-8,8.2c0,3.18,2.45,6.92,7.34,11.23c0.38,0.33,0.95,0.33,1.33,0C17.55,17.12,20,13.38,20,10.2 C20,5.22,16.2,2,12,2z M12,12c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2c1.1,0,2,0.9,2,2C14,11.1,13.1,12,12,12z",
                }
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
                    height: "20",
                    width: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M18,8.5v-3C18,4.67,17.33,4,16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h10v-4c0-1.93,1.57-3.5,3.5-3.5H18z M9.25,10.57L3.89,7.49C3.65,7.35,3.5,7.1,3.5,6.82v0c0-0.59,0.64-0.96,1.15-0.67L10,9.23l5.35-3.07 c0.51-0.29,1.15,0.08,1.15,0.67v0c0,0.27-0.15,0.53-0.39,0.67l-5.37,3.08C10.28,10.84,9.72,10.84,9.25,10.57z",
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
                    g {
                        path {
                            d: "M22,9.97V6c0-1.1-0.9-2-2-2H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h12v-5.03c0-2.76,2.24-5,5-5H22z M19.6,8.25 l-6.54,4.09c-0.65,0.41-1.47,0.41-2.12,0L4.4,8.25C4.15,8.09,4,7.82,4,7.53v0c0-0.67,0.73-1.07,1.3-0.72L12,11l6.7-4.19 C19.27,6.46,20,6.86,20,7.53v0C20,7.82,19.85,8.09,19.6,8.25z",
                        }
                    }
                    g {
                        path {
                            d: "M23,15v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3 C24,15.45,23.55,15,23,15z M22,15h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V15z",
                        }
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
                d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-1 14H5c-.55 0-1-.45-1-1V8l6.94 4.34c.65.41 1.47.41 2.12 0L20 8v9c0 .55-.45 1-1 1zm-7-7L4 6h16l-8 5z",
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
                    fill: "none",
                    height: "20",
                }
                path {
                    d: "M14.11,14.65c-0.2,0.2-0.51,0.2-0.71,0l-1.41-1.41c-0.2-0.2-0.2-0.51,0-0.71v0c0.2-0.2,0.51-0.2,0.71,0l1.06,1.06 l3.18-3.18c0.2-0.2,0.51-0.2,0.71,0l0,0c0.2,0.2,0.2,0.51,0,0.71L14.11,14.65z M10,13c0-2.76,2.24-5,5-5c0.71,0,1.39,0.15,2,0.42V4 c0-0.55-0.45-1-1-1H4C3.45,3,3,3.45,3,4v13l3-3h4.1C10.04,13.68,10,13.34,10,13z",
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
                    fill: "none",
                    x: "0",
                    height: "24",
                    width: "24",
                }
                path {
                    d: "M18.05,19.29c-0.39,0.39-1.02,0.39-1.41,0l-2.12-2.12c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0 l1.41,1.41l3.54-3.54c0.39-0.39,1.02-0.39,1.41,0l0,0c0.39,0.39,0.39,1.02,0,1.41L18.05,19.29z M12,17c0-3.87,3.13-7,7-7 c1.08,0,2.09,0.25,3,0.68V4c0-1.1-0.9-2-2-2H4C2.9,2,2,2.9,2,4v18l4-4h6v0c0-0.17,0.01-0.33,0.03-0.5C12.01,17.33,12,17.17,12,17z",
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
                    height: "20",
                    fill: "none",
                    y: "0",
                    width: "20",
                }
                path {
                    d: "M17,6.22V13c0,0.55-0.45,1-1,1H6l-3,3V4c0-0.55,0.45-1,1-1h8.18C12.07,3.31,12,3.65,12,4c0,1.66,1.34,3,3,3 C15.77,7,16.47,6.7,17,6.22z M13,4c0,1.1,0.9,2,2,2s2-0.9,2-2s-0.9-2-2-2S13,2.9,13,4z",
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
                    width: "24",
                    y: "0",
                    height: "24",
                    fill: "none",
                }
                path {
                    d: "M22,6.98V16c0,1.1-0.9,2-2,2H6l-4,4V4c0-1.1,0.9-2,2-2h10.1C14.04,2.32,14,2.66,14,3c0,2.76,2.24,5,5,5 C20.13,8,21.16,7.61,22,6.98z M16,3c0,1.66,1.34,3,3,3s3-1.34,3-3s-1.34-3-3-3S16,1.34,16,3z",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
                path {
                    d: "M14.11,16.65c-0.2,0.2-0.51,0.2-0.71,0l-1.41-1.41c-0.2-0.2-0.2-0.51,0-0.71v0c0.2-0.2,0.51-0.2,0.71,0l1.06,1.06 l3.18-3.18c0.2-0.2,0.51-0.2,0.71,0h0c0.2,0.2,0.2,0.51,0,0.71L14.11,16.65z M16,4H4C3.45,4,3,4.45,3,5v10c0,0.55,0.45,1,1,1h6.1 c-0.07-0.32-0.1-0.66-0.1-1c0-2.76,2.24-5,5-5c0.71,0,1.39,0.15,2,0.42V5C17,4.45,16.55,4,16,4z M15.77,6.33l-5.23,3.33 c-0.33,0.21-0.75,0.21-1.07,0L4.23,6.33C4.09,6.24,4,6.08,4,5.91c0-0.39,0.43-0.63,0.76-0.42L10,8.82l5.23-3.33 C15.57,5.28,16,5.51,16,5.91C16,6.08,15.91,6.24,15.77,6.33z",
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
                g {
                    rect {
                        fill: "none",
                        width: "24",
                        height: "24",
                        x: "0",
                    }
                }
                g {
                    path {
                        d: "M18.05,21.29c-0.39,0.39-1.02,0.39-1.41,0l-2.12-2.12c-0.39-0.39-0.39-1.02,0-1.41h0c0.39-0.39,1.02-0.39,1.41,0 l1.41,1.41l3.54-3.54c0.39-0.39,1.02-0.39,1.41,0l0,0c0.39,0.39,0.39,1.02,0,1.41L18.05,21.29z M12.08,20H4c-1.1,0-2-0.9-2-2V6 c0-1.1,0.9-2,2-2h16c1.1,0,2,0.9,2,2v6.68C21.09,12.25,20.08,12,19,12c-3.87,0-7,3.13-7,7C12,19.34,12.03,19.67,12.08,20z M11.47,12.67c0.32,0.2,0.74,0.2,1.06,0l7.07-4.42C19.85,8.09,20,7.82,20,7.53c0-0.67-0.73-1.07-1.3-0.72L12,11L5.3,6.81 C4.73,6.46,4,6.86,4,7.53c0,0.29,0.15,0.56,0.4,0.72L11.47,12.67z",
                    }
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
                    d: "M15,8c0.77,0,1.47-0.3,2-0.78V15c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1V5c0-0.55,0.45-1,1-1h8.18 C12.07,4.31,12,4.65,12,5c0,0.79,0.31,1.5,0.81,2.03L10,8.82L4.76,5.49C4.43,5.28,4,5.51,4,5.91c0,0.17,0.09,0.33,0.23,0.42 l5.23,3.33c0.33,0.21,0.75,0.21,1.07,0l3.12-1.99C14.06,7.87,14.52,8,15,8z M13,5c0,1.1,0.9,2,2,2s2-0.9,2-2s-0.9-2-2-2 S13,3.9,13,5z",
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
                    fill: "none",
                    height: "24",
                    x: "0",
                    width: "24",
                }
                path {
                    d: "M19,10c1.13,0,2.16-0.39,3-1.02V18c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h10.1C14.04,4.32,14,4.66,14,5 c0,1.48,0.65,2.79,1.67,3.71L12,11L5.3,6.81C4.73,6.46,4,6.86,4,7.53c0,0.29,0.15,0.56,0.4,0.72l7.07,4.42 c0.32,0.2,0.74,0.2,1.06,0l4.77-2.98C17.84,9.88,18.4,10,19,10z M16,5c0,1.66,1.34,3,3,3s3-1.34,3-3s-1.34-3-3-3S16,3.34,16,5z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    circle {
                        cy: "2.75",
                        r: "2.5",
                        cx: "15.5",
                    }
                    path {
                        d: "M14.15,6.5h-8.4C5.34,6.5,5,6.16,5,5.75C5,5.34,5.34,5,5.75,5h6.45c-0.33-0.49-0.9-1.55-0.62-3H3.5C2.67,2,2,2.67,2,3.5 v13.29c0,0.45,0.54,0.67,0.85,0.35L5,15h11.5c0.83,0,1.5-0.67,1.5-1.5V5.85c-0.69,0.56-1.55,0.9-2.5,0.9 C15.03,6.75,14.58,6.65,14.15,6.5z M11.25,12h-5.5C5.34,12,5,11.66,5,11.25c0-0.41,0.34-0.75,0.75-0.75h5.5 c0.41,0,0.75,0.34,0.75,0.75C12,11.66,11.66,12,11.25,12z M14.25,9.25h-8.5C5.34,9.25,5,8.91,5,8.5c0-0.41,0.34-0.75,0.75-0.75 h8.5C14.66,7.75,15,8.09,15,8.5C15,8.91,14.66,9.25,14.25,9.25z",
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
                    circle {
                        r: "3",
                        cy: "3",
                        cx: "19",
                    }
                    path {
                        d: "M7,8C6.45,8,6,7.55,6,7c0-0.55,0.45-1,1-1h8.03c-1.21-1.6-1.08-3.21-0.92-4H4.01c-1.1,0-2,0.89-2,2L2,19.58 c0,0.89,1.08,1.34,1.71,0.71L6,18h14c1.1,0,2-0.9,2-2V6.97C21.16,7.61,20.13,8,19,8H7z M13,14H7c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1C14,13.55,13.55,14,13,14z M17,11H7c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h10 c0.55,0,1,0.45,1,1C18,10.55,17.55,11,17,11z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-3 12H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1zm0-3H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1zm0-3H7c-.55 0-1-.45-1-1s.45-1 1-1h10c.55 0 1 .45 1 1s-.45 1-1 1z",
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
                d: "M17 1H7c-1.1 0-1.99.9-1.99 2v18c0 1.1.89 2 1.99 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 18H7V5h10v14zm-4.2-5.78v1.75l2.81-2.62c.21-.2.21-.53 0-.73L12.8 9v1.7c-3.11.43-4.35 2.56-4.8 4.7 1.11-1.5 2.58-2.18 4.8-2.18z",
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
                    width: "20",
                    height: "20",
                    fill: "none",
                }
            }
            g {
                g {
                    path {
                        d: "M8.5,8.53v3.24c0,0.18,0.09,0.34,0.24,0.43l2.52,1.51c0.23,0.14,0.52,0.06,0.66-0.16l0,0c0.14-0.23,0.06-0.53-0.16-0.66 L9.5,11.55V8.53c0-0.26-0.21-0.48-0.48-0.48H8.98C8.71,8.05,8.5,8.26,8.5,8.53z",
                    }
                    path {
                        d: "M13.9,10c0.07,0.32,0.1,0.66,0.1,1c0,2.76-2.24,5-5,5s-5-2.24-5-5s2.24-5,5-5c0.71,0,1.39,0.15,2,0.42V5.35 C10.37,5.13,9.7,5,9,5c-3.31,0-6,2.69-6,6s2.69,6,6,6s6-2.69,6-6c0-0.34-0.04-0.67-0.09-1H13.9z",
                    }
                    path {
                        d: "M15,6V4.5C15,4.22,14.78,4,14.5,4h0C14.22,4,14,4.22,14,4.5V6h0h-1.5C12.22,6,12,6.22,12,6.5v0C12,6.78,12.22,7,12.5,7H14 v1.5C14,8.78,14.22,9,14.5,9h0C14.78,9,15,8.78,15,8.5V7v0h1.5C16.78,7,17,6.78,17,6.5v0C17,6.22,16.78,6,16.5,6H15z",
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
                    path {
                        d: "M10.75,8C10.34,8,10,8.34,10,8.75v4.69c0,0.35,0.18,0.67,0.47,0.85l3.64,2.24c0.33,0.2,0.76,0.11,0.97-0.21 c0.23-0.34,0.12-0.8-0.23-1.01L11.5,13.3V8.75C11.5,8.34,11.16,8,10.75,8z",
                    }
                    path {
                        d: "M17.92,12c0.05,0.33,0.08,0.66,0.08,1c0,3.9-3.1,7-7,7s-7-3.1-7-7c0-3.9,3.1-7,7-7c0.7,0,1.37,0.1,2,0.29V4.23 C12.36,4.08,11.69,4,11,4c-5,0-9,4-9,9s4,9,9,9s9-4,9-9c0-0.34-0.02-0.67-0.06-1H17.92z",
                    }
                    path {
                        d: "M22,5h-2V3c0-0.55-0.45-1-1-1s-1,0.45-1,1v2h-2c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h2v2c0,0.55,0.45,1,1,1s1-0.45,1-1V7 h2c0.55,0,1-0.45,1-1C23,5.45,22.55,5,22,5z",
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
                    fill: "none",
                    height: "20",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M5.93,10.5H10v-1H5.93C5.71,8.64,4.93,8,4,8c-1.1,0-2,0.9-2,2s0.9,2,2,2C4.93,12,5.71,11.36,5.93,10.5z M4,11 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S4.55,11,4,11z",
                    }
                    path {
                        d: "M18,10l-3-2v1.5h-3.03c-0.24-3.45-2.98-6.21-6.43-6.48C5.25,3,5,3.23,5,3.52v0C5,3.78,5.19,4,5.45,4.02 C8.55,4.25,11,6.84,11,10s-2.45,5.75-5.55,5.98C5.19,16,5,16.22,5,16.48l0,0c0,0.3,0.25,0.52,0.55,0.5 c3.44-0.27,6.18-3.03,6.43-6.48H15V12L18,10z",
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
                        d: "M6.82,13H11v-2H6.82C6.4,9.84,5.3,9,4,9c-1.66,0-3,1.34-3,3s1.34,3,3,3C5.3,15,6.4,14.16,6.82,13z M4,13 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C5,12.55,4.55,13,4,13z",
                    }
                    path {
                        d: "M22.47,12.4c0.27-0.2,0.27-0.6,0-0.8L19,9v2h-4.05c-0.47-4.69-4.16-8.42-8.83-8.94C5.52,2,5,2.46,5,3.06v0 c0,0.5,0.37,0.93,0.87,0.99C9.88,4.48,13,7.87,13,12s-3.12,7.52-7.13,7.95C5.37,20.01,5,20.44,5,20.94v0c0,0.6,0.52,1.07,1.11,1 c4.67-0.52,8.37-4.25,8.83-8.94H19v2L22.47,12.4z",
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
                d: "M3.09 4.44c-.39.39-.39 1.02 0 1.41l2.03 2.03-.12.13V19c0 1.1.9 2 2 2h10c.35 0 .68-.1.97-.26l1.17 1.17c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L4.5 4.44c-.39-.39-1.02-.39-1.41 0zM19 16.11V5c0-1.1-.9-2-2-2h-6.99L7.95 5.06 19 16.11z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 15c0 .55-.45 1-1 1H4c-.55 0-1-.45-1-1V6c0-.55.45-1 1-1h16c.55 0 1 .45 1 1v12zM10 8c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1s1-.45 1-1V9c0-.55-.45-1-1-1zm4 0c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1s1-.45 1-1V9c0-.55-.45-1-1-1z",
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
                d: "M14.48 11.95c.17.02.34.05.52.05 2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4c0 .18.03.35.05.52l3.43 3.43zm2.21 2.21l5.74 5.74c.33-.17.57-.5.57-.9v-1c0-2.14-3.56-3.5-6.31-3.84zM2.12 2.42c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L4 7.12V10H2c-.55 0-1 .45-1 1s.45 1 1 1h2v2c0 .55.45 1 1 1s1-.45 1-1v-2h2.88l2.51 2.51C9.19 15.11 7 16.3 7 18v1c0 .55.45 1 1 1h8.88l3.29 3.29c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L2.12 2.42zM6 10v-.88l.88.88H6z",
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
                    width: "20",
                    fill: "none",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M10.14,10.93C7.85,10.59,4,11.52,4,13.21V14c0,0.55,0.45,1,1,1h4.35C9.04,14.36,8.47,12.47,10.14,10.93z",
                    }
                    circle {
                        r: "2.5",
                        cy: "7.5",
                        cx: "9",
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
                    circle {
                        r: "4",
                        cx: "10",
                        cy: "8",
                    }
                    path {
                        d: "M10.35,14.01C7.62,13.91,2,15.27,2,18v1c0,0.55,0.45,1,1,1h8.54C9.07,17.24,10.31,14.11,10.35,14.01z",
                    }
                    path {
                        d: "M19.43,18.02c0.47-0.8,0.7-1.77,0.48-2.82c-0.34-1.64-1.72-2.95-3.38-3.16c-2.63-0.34-4.85,1.87-4.5,4.5 c0.22,1.66,1.52,3.04,3.16,3.38c1.05,0.22,2.02-0.01,2.82-0.48l1.86,1.86c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L19.43,18.02z M16,18c-1.1,0-2-0.9-2-2c0-1.1,0.9-2,2-2s2,0.9,2,2C18,17.1,17.1,18,16,18z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M19.23 15.26l-2.54-.29c-.61-.07-1.21.14-1.64.57l-1.84 1.84c-2.83-1.44-5.15-3.75-6.59-6.59l1.85-1.85c.43-.43.64-1.03.57-1.64l-.29-2.52c-.12-1.01-.97-1.77-1.99-1.77H5.03c-1.13 0-2.07.94-2 2.07.53 8.54 7.36 15.36 15.89 15.89 1.13.07 2.07-.87 2.07-2v-1.73c.01-1.01-.75-1.86-1.76-1.98z",
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
                d: "M12.5 7.7c-.28-.28-.72-.28-1 0L8 11.2 4.5 7.7c-.28-.28-.72-.28-1 0s-.28.72 0 1L7 12.2l-3.5 3.5c-.28.28-.28.72 0 1s.72.28 1 0L8 13.2l3.5 3.5c.28.28.72.28 1 0s.28-.72 0-1L9 12.2l3.5-3.5c.28-.28.28-.72 0-1zM19 1H9c-1.1 0-2 .9-2 2v2c0 .55.45 1 1 1s1-.45 1-1V4h10v16H9v-1c0-.55-.45-1-1-1s-1 .45-1 1v2c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2z",
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
                        d: "M17,18H7V6h10v1h2V3c0-1.1-0.9-2-2-2L7,1.01C5.9,1.01,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2v-4h-2V18z",
                    }
                    path {
                        d: "M20,11v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3 C21,11.45,20.55,11,20,11z M19,11h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V11z",
                    }
                }
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
                d: "M14 1H4c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2zm0 19H4V4h10v16zm6.63-11.74c-.26-.32-.74-.36-1.04-.06l-.03.03c-.25.25-.26.65-.05.93 1.26 1.64 1.25 3.87-.02 5.57-.21.28-.19.67.05.92l.05.05c.29.29.76.26 1.03-.05 1.8-2.13 1.8-5.19.01-7.39zm-3.21 2.11l-.06.06c-.2.2-.26.5-.15.76.21.49.21 1.03 0 1.52-.11.26-.05.56.15.76l.08.08c.32.32.87.25 1.09-.15.49-.89.49-1.94-.01-2.86-.22-.42-.77-.5-1.1-.17z",
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
                d: "M7 3v2c0 .55.45 1 1 1s1-.45 1-1V4h10v16H9v-1c0-.55-.45-1-1-1s-1 .45-1 1v2c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-2-2-2H9c-1.1 0-2 .9-2 2zm2.5 12.5c.29-.12.55-.29.8-.48l-.02.03 1.01.39c.23.09.49 0 .61-.22l.84-1.46c.12-.21.07-.49-.12-.64l-.85-.68-.02.03c.02-.16.05-.32.05-.48s-.03-.32-.05-.48l.02.03.85-.68c.19-.15.24-.43.12-.64l-.84-1.46c-.12-.21-.38-.31-.61-.22l-1.01.39.02.03c-.25-.17-.51-.34-.8-.46l-.17-1.08C9.3 7.18 9.09 7 8.84 7H7.16c-.25 0-.46.18-.49.42L6.5 8.5c-.29.12-.55.29-.8.48l.02-.03-1.02-.39c-.23-.09-.49 0-.61.22l-.84 1.46c-.12.21-.07.49.12.64l.85.68.02-.03c-.02.15-.05.31-.05.47s.03.32.05.48l-.02-.03-.85.68c-.19.15-.24.43-.12.64l.84 1.46c.12.21.38.31.61.22l1.01-.39-.01-.04c.25.19.51.36.8.48l.17 1.07c.03.25.24.43.49.43h1.68c.25 0 .46-.18.49-.42l.17-1.08zM6 12c0-1.1.9-2 2-2s2 .9 2 2-.9 2-2 2-2-.9-2-2z",
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
                        d: "M14.54,17.37c-2.63,2.08-5.89,3.39-9.45,3.61c-1.13,0.07-2.07-0.87-2.07-2v-1.73 c-0.01-1.01,0.75-1.86,1.76-1.98l2.54-0.29c0.61-0.07,1.21,0.14,1.64,0.57l1.84,1.84c0.81-0.41,1.59-0.9,2.31-1.45L2.1,4.93 c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.03-0.39,1.42,0L20.49,20.5c0.39,0.39,0.39,1.02,0,1.41s-1.02,0.39-1.41,0 L14.54,17.37z M17.39,10.8l-1.85-1.85c-0.43-0.43-0.64-1.03-0.57-1.64l0.29-2.52c0.12-1.01,0.97-1.77,1.99-1.77h1.73 c1.13,0,2.07,0.94,2,2.07c-0.22,3.57-1.54,6.83-3.62,9.47l-1.43-1.43C16.48,12.4,16.97,11.62,17.39,10.8z",
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
                height: "24",
                width: "24",
                fill: "none",
            }
            g {
                g {
                    path {
                        d: "M4.78,15.27l2.54-0.29c0.61-0.07,1.21,0.14,1.64,0.57l1.84,1.84c2.83-1.44,5.15-3.75,6.59-6.59l-1.85-1.85 c-0.43-0.43-0.64-1.03-0.57-1.64l0.29-2.52c0.12-1.01,0.97-1.77,1.99-1.77h1.73c1.13,0,2.07,0.94,2,2.07 c-0.53,8.54-7.36,15.36-15.89,15.89c-1.13,0.07-2.07-0.87-2.07-2v-1.73C3.01,16.24,3.77,15.39,4.78,15.27z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M2.71 3.07c-.39.39-.39 1.02 0 1.41L4.1 5.87C2.79 7.57 2 9.69 2 12c0 3.3 1.6 6.22 4.06 8.04.48.35 1.16.21 1.46-.31.25-.43.14-.99-.26-1.29C5.29 16.98 4 14.65 4 12c0-1.76.57-3.38 1.53-4.69l1.43 1.44C6.36 9.68 6 10.8 6 12c0 1.8.8 3.41 2.06 4.51.46.4 1.19.25 1.5-.28l.01-.01c.24-.42.13-.94-.23-1.26C8.52 14.23 8 13.18 8 12c0-.65.17-1.25.44-1.79l1.58 1.58L10 12c0 1.1.9 2 2 2l.21-.02 6.81 6.81c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L4.13 3.07c-.39-.39-1.03-.39-1.42 0zm15 10.75c.18-.57.29-1.19.29-1.82 0-3.31-2.69-6-6-6-.63 0-1.25.11-1.82.29l1.72 1.72c.03 0 .06-.01.1-.01 2.21 0 4 1.79 4 4 0 .04-.01.07-.01.11l1.72 1.71zM12 4c4.42 0 8 3.58 8 8 0 1.2-.29 2.32-.77 3.35l1.49 1.49C21.53 15.4 22 13.76 22 12c0-5.52-4.48-10-10-10-1.76 0-3.4.48-4.84 1.28l1.48 1.48C9.66 4.28 10.8 4 12 4z",
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
                d: "M21 3H3c-1.11 0-2 .89-2 2v14c0 1.11.89 2 2 2h18c1.11 0 2-.89 2-2V5c0-1.11-.89-2-2-2zm-1 16.02H4c-.55 0-1-.45-1-1V5.98c0-.55.45-1 1-1h16c.55 0 1 .45 1 1v12.04c0 .55-.45 1-1 1zM10 12H8l3.65-3.65c.2-.2.51-.2.71 0L16 12h-2v4h-4v-4z",
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
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M2.12 2.32c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L4.98 8C3.33 8.01 2 9.35 2 11v4c0 1.1.9 2 2 2h2v2c0 1.1.9 2 2 2h8c.55 0 1.04-.22 1.4-.58l2.83 2.83c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L2.12 2.32zM15 19H9c-.55 0-1-.45-1-1v-4h2.98l4.72 4.72c-.19.17-.43.28-.7.28zm4-11h-8.37l9 9H20c1.1 0 2-.9 2-2v-4c0-1.66-1.34-3-3-3zm0 4c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm-2-5c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1H7c-.37 0-.68.21-.85.51L9.63 7H17z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M5,9h3c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1H5C4.45,4,4,4.45,4,5v3C4,8.55,4.45,9,5,9z M4.94,4.94h3.12v3.12H4.94V4.94z",
                    }
                    path {
                        d: "M5,16h3c0.55,0,1-0.45,1-1v-3c0-0.55-0.45-1-1-1H5c-0.55,0-1,0.45-1,1v3C4,15.55,4.45,16,5,16z M4.94,11.94h3.12v3.12 H4.94V11.94z",
                    }
                    path {
                        d: "M11,5v3c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h-3C11.45,4,11,4.45,11,5z M15.06,8.06h-3.12V4.94 h3.12V8.06z",
                    }
                    rect {
                        height: "1",
                        width: "1",
                        x: "15",
                        y: "15",
                    }
                    rect {
                        x: "15",
                        height: "1",
                        width: "1",
                        y: "13",
                    }
                    rect {
                        width: "1",
                        y: "11",
                        height: "1",
                        x: "15",
                    }
                    rect {
                        width: "1",
                        height: "1",
                        y: "12",
                        x: "12",
                    }
                    rect {
                        width: "1",
                        y: "11",
                        x: "11",
                        height: "1",
                    }
                    rect {
                        height: "1",
                        x: "13",
                        width: "1",
                        y: "13",
                    }
                    rect {
                        height: "1",
                        y: "14",
                        x: "14",
                        width: "1",
                    }
                    rect {
                        width: "1",
                        y: "11",
                        x: "13",
                        height: "1",
                    }
                    rect {
                        width: "1",
                        y: "12",
                        height: "1",
                        x: "14",
                    }
                    rect {
                        x: "11",
                        width: "1",
                        height: "1",
                        y: "13",
                    }
                    rect {
                        width: "1",
                        x: "12",
                        y: "14",
                        height: "1",
                    }
                    rect {
                        width: "1",
                        height: "1",
                        x: "11",
                        y: "15",
                    }
                    rect {
                        y: "15",
                        width: "1",
                        height: "1",
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
                        d: "M5,11h4c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5v4C3,10.1,3.9,11,5,11z M5,5h4v4H5V5z",
                    }
                    path {
                        d: "M5,21h4c1.1,0,2-0.9,2-2v-4c0-1.1-0.9-2-2-2H5c-1.1,0-2,0.9-2,2v4C3,20.1,3.9,21,5,21z M5,15h4v4H5V15z",
                    }
                    path {
                        d: "M13,5v4c0,1.1,0.9,2,2,2h4c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2h-4C13.9,3,13,3.9,13,5z M19,9h-4V5h4V9z",
                    }
                    path {
                        d: "M21,20.5v-1c0-0.28-0.22-0.5-0.5-0.5h-1c-0.28,0-0.5,0.22-0.5,0.5v1c0,0.28,0.22,0.5,0.5,0.5h1C20.78,21,21,20.78,21,20.5 z",
                    }
                    path {
                        d: "M13,13.5v1c0,0.28,0.22,0.5,0.5,0.5h1c0.28,0,0.5-0.22,0.5-0.5v-1c0-0.28-0.22-0.5-0.5-0.5h-1C13.22,13,13,13.22,13,13.5z",
                    }
                    path {
                        d: "M16.5,15h-1c-0.28,0-0.5,0.22-0.5,0.5v1c0,0.28,0.22,0.5,0.5,0.5h1c0.28,0,0.5-0.22,0.5-0.5v-1C17,15.22,16.78,15,16.5,15 z",
                    }
                    path {
                        d: "M13,17.5v1c0,0.28,0.22,0.5,0.5,0.5h1c0.28,0,0.5-0.22,0.5-0.5v-1c0-0.28-0.22-0.5-0.5-0.5h-1C13.22,17,13,17.22,13,17.5z",
                    }
                    path {
                        d: "M15.5,21h1c0.28,0,0.5-0.22,0.5-0.5v-1c0-0.28-0.22-0.5-0.5-0.5h-1c-0.28,0-0.5,0.22-0.5,0.5v1C15,20.78,15.22,21,15.5,21 z",
                    }
                    path {
                        d: "M17.5,19h1c0.28,0,0.5-0.22,0.5-0.5v-1c0-0.28-0.22-0.5-0.5-0.5h-1c-0.28,0-0.5,0.22-0.5,0.5v1C17,18.78,17.22,19,17.5,19 z",
                    }
                    path {
                        d: "M18.5,13h-1c-0.28,0-0.5,0.22-0.5,0.5v1c0,0.28,0.22,0.5,0.5,0.5h1c0.28,0,0.5-0.22,0.5-0.5v-1C19,13.22,18.78,13,18.5,13 z",
                    }
                    path {
                        d: "M19.5,17h1c0.28,0,0.5-0.22,0.5-0.5v-1c0-0.28-0.22-0.5-0.5-0.5h-1c-0.28,0-0.5,0.22-0.5,0.5v1C19,16.78,19.22,17,19.5,17 z",
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
                    width: "24",
                    fill: "none",
                }
            }
            path {
                d: "M15,21h-2v-2h2V21z M13,14h-2v5h2V14z M21,12h-2v4h2V12z M19,10h-2v2h2V10z M7,12H5v2h2V12z M5,10H3v2h2V10z M12,5h2V3h-2V5 z M4.5,4.5v3h3v-3H4.5z M8,9H4C3.45,9,3,8.55,3,8V4c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v4C9,8.55,8.55,9,8,9z M4.5,16.5v3h3v-3 H4.5z M8,21H4c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v4C9,20.55,8.55,21,8,21z M16.5,4.5v3h3v-3H16.5z M20,9 h-4c-0.55,0-1-0.45-1-1V4c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v4C21,8.55,20.55,9,20,9z M19,19v-3l-4,0v2h2v3h4v-2H19z M17,12 l-4,0v2h4V12z M13,10H7v2h2v2h2v-2h2V10z M14,9V7h-2V5h-2v4L14,9z M6.75,5.25h-1.5v1.5h1.5V5.25z M6.75,17.25h-1.5v1.5h1.5V17.25z M18.75,5.25h-1.5v1.5h1.5V5.25z",
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
                width: "24",
                fill: "none",
                height: "24",
            }
            path {
                d: "M9.5,6.5v3h-3v-3H9.5 M11,5H5v6h6V5L11,5z M9.5,14.5v3h-3v-3H9.5 M11,13H5v6h6V13L11,13z M17.5,6.5v3h-3v-3H17.5 M19,5h-6v6 h6V5L19,5z M13,13h1.5v1.5H13V13z M14.5,14.5H16V16h-1.5V14.5z M16,13h1.5v1.5H16V13z M13,16h1.5v1.5H13V16z M14.5,17.5H16V19h-1.5 V17.5z M16,16h1.5v1.5H16V16z M17.5,14.5H19V16h-1.5V14.5z M17.5,17.5H19V19h-1.5V17.5z M21,7L21,7c-0.55,0-1-0.45-1-1V4h-2 c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1v3C22,6.55,21.55,7,21,7z M22,21v-3c0-0.55-0.45-1-1-1h0 c-0.55,0-1,0.45-1,1v2h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h3C21.55,22,22,21.55,22,21z M3,22h3c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1H4v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v3C2,21.55,2.45,22,3,22z M2,3v3c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1V4h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H3C2.45,2,2,2.45,2,3z",
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
                    height: "20",
                    fill: "none",
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M11,10L8,7v2.5H3.5C3.22,9.5,3,9.72,3,10c0,0.28,0.22,0.5,0.5,0.5H8V13L11,10z",
                    }
                    path {
                        d: "M11.5,8h5C16.78,8,17,7.78,17,7.5C17,7.22,16.78,7,16.5,7h-5C11.22,7,11,7.22,11,7.5C11,7.78,11.22,8,11.5,8z",
                    }
                    path {
                        d: "M16.5,12h-5c-0.28,0-0.5,0.22-0.5,0.5c0,0.28,0.22,0.5,0.5,0.5h5c0.28,0,0.5-0.22,0.5-0.5C17,12.22,16.78,12,16.5,12z",
                    }
                    path {
                        d: "M16.5,9.5h-3C13.22,9.5,13,9.72,13,10c0,0.28,0.22,0.5,0.5,0.5h3c0.28,0,0.5-0.22,0.5-0.5C17,9.72,16.78,9.5,16.5,9.5z",
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
                    path {
                        d: "M14,9h7c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1h-7c-0.55,0-1,0.45-1,1C13,8.55,13.45,9,14,9z",
                    }
                    path {
                        d: "M21,15h-7c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h7c0.55,0,1-0.45,1-1C22,15.45,21.55,15,21,15z",
                    }
                    path {
                        d: "M21,11h-4c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1C22,11.45,21.55,11,21,11z",
                    }
                    path {
                        d: "M8.85,7.85C8.54,7.54,8,7.76,8,8.21V11H3c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1h5v2.79c0,0.45,0.54,0.67,0.85,0.35 l3.79-3.79c0.2-0.2,0.2-0.51,0-0.71L8.85,7.85z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M11.98 7h.03c.55 0 .99-.44.99-.98V2.98c0-.54-.44-.98-.98-.98h-.03c-.55 0-.99.44-.99.98v3.03c0 .55.44.99.98.99zm4.92 2.11c.39.39 1.01.39 1.4 0 .62-.63 1.52-1.54 2.15-2.17.39-.38.39-1.01 0-1.39-.38-.38-1.01-.38-1.39 0L16.89 7.7c-.39.38-.39 1.01 0 1.39l.01.02zM5.71 9.1c.38.39 1.01.39 1.4 0 .38-.38.38-1.01 0-1.39L4.96 5.54c-.38-.39-1.01-.39-1.39 0l-.02.01c-.39.39-.39 1.01 0 1.39.63.62 1.54 1.53 2.16 2.16zm17.58 7.13c-6.41-5.66-16.07-5.66-22.48 0-.85.75-.85 2.08-.05 2.88l1.22 1.22c.72.72 1.86.78 2.66.15l2-1.59c.48-.38.76-.96.76-1.57v-2.6c3.02-.98 6.29-.99 9.32 0v2.61c0 .61.28 1.19.76 1.57l1.99 1.58c.8.63 1.94.57 2.66-.15l1.22-1.22c.79-.8.79-2.13-.06-2.88z",
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
                cx: "6.18",
                cy: "17.82",
                r: "2.18",
            }
            path {
                d: "M5.59 10.23c-.84-.14-1.59.55-1.59 1.4 0 .71.53 1.28 1.23 1.4 2.92.51 5.22 2.82 5.74 5.74.12.7.69 1.23 1.4 1.23.85 0 1.54-.75 1.41-1.59-.68-4.2-3.99-7.51-8.19-8.18zm-.03-5.71C4.73 4.43 4 5.1 4 5.93c0 .73.55 1.33 1.27 1.4 6.01.6 10.79 5.38 11.39 11.39.07.73.67 1.28 1.4 1.28.84 0 1.5-.73 1.42-1.56-.73-7.34-6.57-13.19-13.92-13.92z",
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
                    d: "M8.76,4.69L8.15,8.58c-0.12,0.78,0.48,1.49,1.28,1.49h0c0.64,0,1.18-0.46,1.28-1.09l0.53-3.41h2.58L11.8,18.43h-1.24 c-0.63,0-1.16,0.46-1.26,1.08l0,0.01C9.17,20.3,9.77,21,10.56,21h4.67c0.63,0,1.17-0.46,1.26-1.08l0-0.01 c0.12-0.78-0.48-1.48-1.26-1.48h-0.86l2-12.86h2.58l-0.47,3.01c-0.12,0.78,0.48,1.49,1.28,1.49h0.03c0.64,0,1.18-0.46,1.28-1.09 l0.57-3.67C21.83,4.09,20.89,3,19.66,3h-8.92C9.76,3,8.92,3.72,8.76,4.69z M8,5H4.86C4.36,5,3.94,5.36,3.87,5.85l0,0 C3.77,6.45,4.24,7,4.86,7h2.83L8,5z M7.39,9H4.25C3.75,9,3.33,9.36,3.26,9.85l0,0C3.16,10.45,3.63,11,4.25,11h2.83L7.39,9z M8.31,17H3.17c-0.49,0-0.91,0.36-0.99,0.85l0,0C2.08,18.45,2.55,19,3.17,19H8L8.31,17z M8.93,13H3.79c-0.49,0-0.91,0.36-0.99,0.85 l0,0C2.7,14.45,3.17,15,3.79,15h4.84L8.93,13z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M20 18c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.89 2 2 2H1c-.55 0-1 .45-1 1s.45 1 1 1h22c.55 0 1-.45 1-1s-.45-1-1-1h-3zm-7-3.53v-2.19c-2.78 0-4.61.85-6 2.72.56-2.67 2.11-5.33 6-5.87V7l3.61 3.36c.21.2.21.53 0 .73L13 14.47z",
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
                    width: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M17,11.32V4.5C17,3.67,16.33,3,15.5,3H12c0-1.1-0.9-2-2-2S8,1.9,8,3H4.5C3.67,3,3,3.67,3,4.5V8c1.1,0,2,0.9,2,2 s-0.9,2-2,2v3.5C3,16.33,3.67,17,4.5,17H8c0-0.93,0.64-1.71,1.5-1.93V10V7.57l2.17,1.09L17,11.32z",
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
                        d: "M20,6c0-1.1-0.9-2-2-2h-4c0-1.38-1.12-2.5-2.5-2.5S9,2.62,9,4H5.01c-1.1,0-2,0.9-2,2v3.8C5.7,9.8,6,11.96,6,12.5 c0,0.54-0.29,2.7-3,2.7V19c0,1.1,0.9,2,2,2h3.8c0-2.16,1.37-2.78,2.2-2.94v-9.3l9,4.5V6z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            circle {
                cx: "15.5",
                cy: "9.5",
                r: "1.5",
            }
            circle {
                cy: "9.5",
                r: "1.5",
                cx: "8.5",
            }
            path {
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm4.41-6.11c-.35-.22-.82-.11-1.03.24-.74 1.17-2 1.87-3.38 1.87s-2.64-.7-3.38-1.88c-.22-.35-.68-.46-1.03-.24-.35.22-.46.68-.24 1.03C8.37 16.54 10.1 17.5 12 17.5s3.63-.97 4.65-2.58c.22-.35.11-.81-.24-1.03z",
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
                g {
                    rect {
                        x: "15.5",
                        y: "10.5",
                        width: "2",
                        height: "1",
                    }
                    path {
                        d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M10,9.75 c0,0.41-0.34,0.75-0.75,0.75H6.5v0.75H9c0.55,0,1,0.45,1,1V14c0,0.55-0.45,1-1,1H5.75C5.34,15,5,14.66,5,14.25v0 c0-0.41,0.34-0.75,0.75-0.75H8.5v-0.75H6c-0.55,0-1-0.45-1-1V10c0-0.55,0.45-1,1-1h3.25C9.66,9,10,9.34,10,9.75L10,9.75z M12,15 L12,15c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v4C13,14.55,12.55,15,12,15z M19,12c0,0.55-0.45,1-1,1h-2.5 v1.25c0,0.41-0.34,0.75-0.75,0.75h0C14.34,15,14,14.66,14,14.25V10c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1V12z",
                    }
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
                        d: "M7.76,7.83l0.02,0.02c0.35,0.35,0.89,0.38,1.3,0.09C9.91,7.37,10.92,7.02,12,7.02s2.09,0.35,2.92,0.93 c0.4,0.29,0.95,0.26,1.3-0.09l0.02-0.02c0.42-0.42,0.39-1.14-0.09-1.49C14.98,5.5,13.55,5,12,5S9.02,5.5,7.86,6.34 C7.37,6.69,7.34,7.41,7.76,7.83z",
                    }
                    path {
                        d: "M12,1C9.38,1,6.97,1.93,5.08,3.47C4.62,3.84,4.57,4.53,5,4.96l0,0c0.36,0.36,0.93,0.39,1.32,0.07C7.86,3.76,9.85,3,12,3 s4.14,0.76,5.69,2.03c0.39,0.32,0.96,0.29,1.32-0.07l0,0c0.42-0.42,0.38-1.11-0.08-1.49C17.03,1.93,14.62,1,12,1z",
                    }
                    path {
                        d: "M15,10l-6,0c-0.55,0-1,0.45-1,1v10c0,0.55,0.45,1,1,1h5.99c0.55,0,1-0.45,1-1L16,11C16,10.45,15.55,10,15,10z M15,20H9v-8 h6V20z",
                    }
                }
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
                d: "M10,3C8.34,3,7,4.34,7,6c0,1.66,1.34,3,3,3s3-1.34,3-3C13,4.34,11.66,3,10,3z M14,11c-1.66,0-3,1.34-3,3c0,1.66,1.34,3,3,3 s3-1.34,3-3C17,12.34,15.66,11,14,11z M6,11c-1.66,0-3,1.34-3,3c0,1.66,1.34,3,3,3s3-1.34,3-3C9,12.34,7.66,11,6,11z",
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
                height: "24",
                fill: "none",
            }
            path {
                d: "M16,7c0,2.21-1.79,4-4,4S8,9.21,8,7s1.79-4,4-4S16,4.79,16,7z M7,13c-2.21,0-4,1.79-4,4s1.79,4,4,4s4-1.79,4-4S9.21,13,7,13 z M17,13c-2.21,0-4,1.79-4,4s1.79,4,4,4s4-1.79,4-4S19.21,13,17,13z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
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
                d: "M23 18h-1.2l1.79 1.79c.24-.18.41-.46.41-.79 0-.55-.45-1-1-1zM3.23 2.28c-.39-.39-1.03-.39-1.42 0-.39.39-.39 1.02 0 1.41l.84.86s-.66.57-.66 1.47C2 6.92 2 16 2 16l.01.01c0 1.09.88 1.98 1.97 1.99H1c-.55 0-1 .45-1 1s.45 1 1 1h17.13l2 2c.39.39 1.02.39 1.41 0s.39-1.02 0-1.41L3.23 2.28zM7 15c.31-1.48.94-2.93 2.08-4.05l1.59 1.59C9.13 12.92 7.96 13.71 7 15zm6-5.87v-.98c0-.44.52-.66.84-.37L15 8.87l1.61 1.5c.21.2.21.53 0 .73l-.89.83 5.58 5.58c.43-.37.7-.9.7-1.51V6c0-1.09-.89-1.98-1.98-1.98H7.8l5.14 5.13c.02-.01.04-.02.06-.02z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M17.65 4.35l-2.79 2.79c-.32.32-.1.86.35.86H17v6.88c0 1-.67 1.93-1.66 2.09-1.25.21-2.34-.76-2.34-1.97V8.17c0-2.09-1.53-3.95-3.61-4.15C7.01 3.79 5 5.66 5 8v7H3.21c-.45 0-.67.54-.35.85l2.79 2.79c.2.2.51.2.71 0l2.79-2.79c.31-.31.09-.85-.36-.85H7V8.12c0-1 .67-1.93 1.66-2.09C9.91 5.82 11 6.79 11 8v6.83c0 2.09 1.53 3.95 3.61 4.15C16.99 19.21 19 17.34 19 15V8h1.79c.45 0 .67-.54.35-.85l-2.79-2.79c-.19-.2-.51-.2-.7-.01z",
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM9 11H7V9h2v2zm4 0h-2V9h2v2zm4 0h-2V9h2v2z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M18.5 11.5c.92 0 1.75.26 2.49.69V5c0-1.1-.89-2-1.99-2H5c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h8.55c-.02-.17-.05-.33-.05-.5 0-2.76 2.24-5 5-5zm-5.61-1.45c-.56.28-1.23.28-1.79 0l-5.61-2.8c-.3-.15-.49-.46-.49-.8 0-.66.7-1.1 1.29-.8L12 8.5l5.71-2.85c.59-.3 1.29.13 1.29.8 0 .34-.19.65-.49.8l-5.62 2.8zM18.5 13c-1.93 0-3.5 1.57-3.5 3.5s1.57 3.5 3.5 3.5 3.5-1.57 3.5-3.5-1.57-3.5-3.5-3.5zm2 3.5c0 .28-.22.5-.5.5h-3c-.28 0-.5-.22-.5-.5s.22-.5.5-.5h3c.28 0 .5.22.5.5z",
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
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
                d: "M12.65 10C11.7 7.31 8.9 5.5 5.77 6.12c-2.29.46-4.15 2.29-4.63 4.58C.32 14.57 3.26 18 7 18c2.61 0 4.83-1.67 5.65-4H17v2c0 1.1.9 2 2 2s2-.9 2-2v-2c1.1 0 2-.9 2-2s-.9-2-2-2h-8.35zM7 14c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
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
                    fill: "none",
                }
            }
            g {
                path {
                    d: "M15.48,17.6c0.29,0.29,0.77,0.29,1.06,0c0.29-0.29,0.29-0.77,0-1.06L3.46,3.46c-0.29-0.29-0.77-0.29-1.06,0 c-0.29,0.29-0.29,0.77,0,1.06l1.13,1.13C2.02,6.51,1,8.14,1,10c0,2.76,2.24,5,5,5c1.87,0,3.48-1.03,4.33-2.55L15.48,17.6z M6,12 c-1.1,0-2-0.9-2-2c0-1.06,0.83-1.94,1.88-2L8,10.12C7.93,11.17,7.06,12,6,12z M16.62,14.5c0.24-0.27,0.38-0.61,0.38-1v-2h0.5 c0.83,0,1.5-0.67,1.5-1.5v0c0-0.83-0.67-1.5-1.5-1.5h-6.88L16.62,14.5z",
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
                path {
                    d: "M3.98,6.81C2.2,7.85,1,9.79,1,12c0,3.31,2.69,6,6,6c2.21,0,4.15-1.2,5.18-2.99l6.89,6.89c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41L3.98,6.81z M8.99,11.82 C9,11.88,9,11.94,9,12c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2c0.06,0,0.12,0,0.18,0.01L8.99,11.82z M20.32,17.5 C20.74,17.13,21,16.59,21,16v-2c1.1,0,2-0.9,2-2s-0.9-2-2-2h-8.17L20.32,17.5",
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
                    width: "20",
                    height: "20",
                }
            }
            g {
                g {
                    path {
                        d: "M13.8,12.19l-1.69-0.2c-0.4-0.05-0.81,0.09-1.1,0.38L9.79,13.6c-1.89-0.96-3.43-2.5-4.39-4.39l1.23-1.23 c0.29-0.29,0.43-0.69,0.38-1.1L6.81,5.2C6.73,4.53,6.16,4.02,5.49,4.02H4.34C3.58,4.02,2.96,4.65,3,5.4 C3.36,11.09,7.91,15.64,13.6,16c0.75,0.05,1.38-0.58,1.38-1.33v-1.15C14.98,12.84,14.47,12.27,13.8,12.19z",
                    }
                    path {
                        d: "M16.67,5.3C16.53,5.19,15.11,4,13,4c-2.12,0-3.53,1.19-3.67,1.3L13,10L16.67,5.3z",
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
                rect {
                    width: "24",
                    fill: "none",
                    height: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M22,4.95C21.79,4.78,19.67,3,16.5,3c-3.18,0-5.29,1.78-5.5,1.95L16.5,12L22,4.95z",
                    }
                    path {
                        d: "M19.2,15.28l-2.54-0.29c-0.61-0.07-1.21,0.14-1.64,0.57l-1.84,1.84c-2.83-1.44-5.15-3.75-6.59-6.59l1.85-1.85 c0.43-0.43,0.64-1.04,0.57-1.64L8.72,4.8C8.6,3.79,7.75,3.03,6.73,3.03H5c-1.13,0-2.07,0.94-2,2.07 C3.53,13.64,10.36,20.47,18.9,21c1.13,0.07,2.07-0.87,2.07-2v-1.73C20.97,16.25,20.21,15.4,19.2,15.28z",
                    }
                }
            }
        }
    }
}

