use dioxus::prelude::*;
use crate::IconProps;
pub fn _123_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6,12H5V9H4V8h2V12z M11,11H9v-0.5h1.5c0.28,0,0.5-0.22,0.5-0.5V8.5C11,8.22,10.78,8,10.5,8H8v1h2v0.5H8.5 C8.22,9.5,8,9.72,8,10v2h3V11z M15.5,11.5v-3C15.5,8.22,15.28,8,15,8h-2.5v1h2v0.5h-1v1h1V11h-2v1H15 C15.28,12,15.5,11.78,15.5,11.5z",
                }
            }
        }
    }
}

pub fn _123_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M7,15H5.5v-4.5H4V9h3V15z M13.5,13.5h-3v-1h2c0.55,0,1-0.45,1-1V10c0-0.55-0.45-1-1-1H9v1.5h3v1h-2c-0.55,0-1,0.45-1,1V15 h4.5V13.5z M19.5,14v-4c0-0.55-0.45-1-1-1H15v1.5h3v1h-2v1h2v1h-3V15h3.5C19.05,15,19.5,14.55,19.5,14z",
                }
            }
        }
    }
}

pub fn _3d_rotation_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M7.52 21.48C4.25 19.94 1.91 16.76 1.55 13H.05C.56 19.16 5.71 24 12 24l.66-.03-3.81-3.81-1.33 1.32zm.89-6.52c-.19 0-.37-.03-.52-.08-.16-.06-.29-.13-.4-.24-.11-.1-.2-.22-.26-.37-.06-.14-.09-.3-.09-.47h-1.3c0 .36.07.68.21.95.14.27.33.5.56.69.24.18.51.32.82.41.3.1.62.15.96.15.37 0 .72-.05 1.03-.15.32-.1.6-.25.83-.44s.42-.43.55-.72c.13-.29.2-.61.2-.97 0-.19-.02-.38-.07-.56-.05-.18-.12-.35-.23-.51-.1-.16-.24-.3-.4-.43-.17-.13-.37-.23-.61-.31.2-.09.37-.2.52-.33.15-.13.27-.27.37-.42.1-.15.17-.3.22-.46.05-.16.07-.32.07-.48 0-.36-.06-.68-.18-.96-.12-.28-.29-.51-.51-.69-.2-.19-.47-.33-.77-.43C9.1 8.05 8.76 8 8.39 8c-.36 0-.69.05-1 .16-.3.11-.57.26-.79.45-.21.19-.38.41-.51.67-.12.26-.18.54-.18.85h1.3c0-.17.03-.32.09-.45s.14-.25.25-.34c.11-.09.23-.17.38-.22.15-.05.3-.08.48-.08.4 0 .7.1.89.31.19.2.29.49.29.86 0 .18-.03.34-.08.49-.05.15-.14.27-.25.37-.11.1-.25.18-.41.24-.16.06-.36.09-.58.09H7.5v1.03h.77c.22 0 .42.02.6.07s.33.13.45.23c.12.11.22.24.29.4.07.16.1.35.1.57 0 .41-.12.72-.35.93-.23.23-.55.33-.95.33zm8.55-5.92c-.32-.33-.7-.59-1.14-.77-.43-.18-.92-.27-1.46-.27H12v8h2.3c.55 0 1.06-.09 1.51-.27.45-.18.84-.43 1.16-.76.32-.33.57-.73.74-1.19.17-.47.26-.99.26-1.57v-.4c0-.58-.09-1.1-.26-1.57-.18-.47-.43-.87-.75-1.2zm-.39 3.16c0 .42-.05.79-.14 1.13-.1.33-.24.62-.43.85-.19.23-.43.41-.71.53-.29.12-.62.18-.99.18h-.91V9.12h.97c.72 0 1.27.23 1.64.69.38.46.57 1.12.57 1.99v.4zM12 0l-.66.03 3.81 3.81 1.33-1.33c3.27 1.55 5.61 4.72 5.96 8.48h1.5C23.44 4.84 18.29 0 12 0z",
            }
        }
    }
}

pub fn abc_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13,8.5C13,8.22,13.22,8,13.5,8h2C15.78,8,16,8.22,16,8.5v0.75h-1V9h-1v2h1v-0.25l1-0.01v0.76c0,0.28-0.22,0.5-0.5,0.5h-2 c-0.28,0-0.5-0.22-0.5-0.5V8.5z M7,8.5V12H6v-1H5v1H4V8.5C4,8.22,4.22,8,4.5,8h2C6.78,8,7,8.22,7,8.5z M6,9H5v1h1V9z M11,10 c0.28,0,0.5,0.22,0.5,0.5v1c0,0.27-0.22,0.5-0.5,0.5H8.5V8H11c0.28,0,0.5,0.22,0.5,0.5v1C11.5,9.77,11.28,10,11,10z M9.5,9v0.5h1V9 H9.5z M10.5,11v-0.5h-1V11H10.5z",
                }
            }
        }
    }
}

pub fn abc_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M21,11h-1.5v-0.5h-2v3h2V13H21v1c0,0.55-0.45,1-1,1h-3c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1V11z M8,10v5H6.5v-1.5h-2V15H3v-5c0-0.55,0.45-1,1-1h3C7.55,9,8,9.45,8,10z M6.5,10.5h-2V12h2V10.5z M13.5,12c0.55,0,1,0.45,1,1v1 c0,0.55-0.45,1-1,1h-4V9h4c0.55,0,1,0.45,1,1v1C14.5,11.55,14.05,12,13.5,12z M11,10.5v0.75h2V10.5H11z M13,12.75h-2v0.75h2V12.75z",
                }
            }
        }
    }
}

pub fn accessibility_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm9 7h-6v13h-2v-6h-2v6H9V9H3V7h18v2z",
            }
        }
    }
}

pub fn accessibility_new_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20.5 6c-2.61.7-5.67 1-8.5 1s-5.89-.3-8.5-1L3 8c1.86.5 4 .83 6 1v13h2v-6h2v6h2V9c2-.17 4.14-.5 6-1l-.5-2zM12 6c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2z",
            }
        }
    }
}

pub fn accessible_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
            circle {
                cx: "12",
                r: "2",
                cy: "4",
            }
            path {
                d: "M19 13v-2c-1.54.02-3.09-.75-4.07-1.83l-1.29-1.43c-.17-.19-.38-.34-.61-.45-.01 0-.01-.01-.02-.01H13c-.35-.2-.75-.3-1.19-.26C10.76 7.11 10 8.04 10 9.09V15c0 1.1.9 2 2 2h5v5h2v-5.5c0-1.1-.9-2-2-2h-3v-3.45c1.29 1.07 3.25 1.94 5 1.95zm-6.17 5c-.41 1.16-1.52 2-2.83 2-1.66 0-3-1.34-3-3 0-1.31.84-2.41 2-2.83V12.1c-2.28.46-4 2.48-4 4.9 0 2.76 2.24 5 5 5 2.42 0 4.44-1.72 4.9-4h-2.07z",
            }
        }
    }
}

pub fn accessible_forward_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
            circle {
                cx: "17",
                cy: "4.54",
                r: "2",
            }
            path {
                d: "M14 17h-2c0 1.65-1.35 3-3 3s-3-1.35-3-3 1.35-3 3-3v-2c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5zm3-3.5h-1.86l1.67-3.67C17.42 8.5 16.44 7 14.96 7h-5.2c-.81 0-1.54.47-1.87 1.2L7.22 10l1.92.53L9.79 9H12l-1.83 4.1c-.6 1.33.39 2.9 1.85 2.9H17v5h2v-5.5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn account_balance_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        y: "10",
                        height: "7",
                        x: "4",
                        width: "3",
                    }
                    rect {
                        height: "7",
                        width: "3",
                        y: "10",
                        x: "10.5",
                    }
                    rect {
                        y: "19",
                        width: "20",
                        height: "3",
                        x: "2",
                    }
                    rect {
                        x: "17",
                        height: "7",
                        width: "3",
                        y: "10",
                    }
                    polygon {
                        points: "12,1 2,6 2,8 22,8 22,6",
                    }
                }
            }
        }
    }
}

pub fn account_balance_wallet_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21 18v1c0 1.1-.9 2-2 2H5c-1.11 0-2-.9-2-2V5c0-1.1.89-2 2-2h14c1.1 0 2 .9 2 2v1h-9c-1.11 0-2 .9-2 2v8c0 1.1.89 2 2 2h9zm-9-2h10V8H12v8zm4-2.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

pub fn account_box_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M10,5.5c1.66,0,3,1.34,3,3s-1.34,3-3,3s-3-1.34-3-3S8.34,5.5,10,5.5z M15.5,15.5h-11c0-0.6,0.03-1.18,0.73-1.62 C6.61,13.01,8.24,12.5,10,12.5s3.39,0.51,4.77,1.38C15.48,14.32,15.5,14.95,15.5,15.5z",
                    }
                }
            }
        }
    }
}

pub fn account_box_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12,6c1.93,0,3.5,1.57,3.5,3.5 c0,1.93-1.57,3.5-3.5,3.5s-3.5-1.57-3.5-3.5C8.5,7.57,10.07,6,12,6z M19,19H5v-0.23c0-0.62,0.28-1.2,0.76-1.58 C7.47,15.82,9.64,15,12,15s4.53,0.82,6.24,2.19c0.48,0.38,0.76,0.97,0.76,1.58V19z",
                }
            }
        }
    }
}

pub fn account_circle_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10 2c-4.42 0-8 3.58-8 8s3.58 8 8 8 8-3.58 8-8-3.58-8-8-8zm0 3.5c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm0 11c-2.05 0-3.87-.95-5.07-2.44 1.45-.98 3.19-1.56 5.07-1.56s3.62.58 5.07 1.56c-1.2 1.49-3.02 2.44-5.07 2.44z",
                    }
                }
            }
        }
    }
}

pub fn account_circle_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 4c1.93 0 3.5 1.57 3.5 3.5S13.93 13 12 13s-3.5-1.57-3.5-3.5S10.07 6 12 6zm0 14c-2.03 0-4.43-.82-6.14-2.88C7.55 15.8 9.68 15 12 15s4.45.8 6.14 2.12C16.43 19.18 14.03 20 12 20z",
                }
            }
        }
    }
}

pub fn addchart_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M22 5v2h-3v3h-2V7h-3V5h3V2h2v3h3zm-3 14H5V5h6V3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2v-6h-2v6zm-4-6v4h2v-4h-2zm-4 4h2V9h-2v8zm-2 0v-6H7v6h2z",
            }
        }
    }
}

pub fn add_card_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16H12v-1.5H3.5V10H18V5.5C18,4.67,17.33,4,16.5,4z M16.5,7h-13V5.5 h13V7z M20,15.5h-2.5V18H16v-2.5h-2.5V14H16v-2.5h1.5V14H20V15.5z",
                }
            }
        }
    }
}

pub fn add_card_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,4H4C2.89,4,2.01,4.89,2.01,6L2,18c0,1.11,0.89,2,2,2h10v-2H4v-6h18V6C22,4.89,21.11,4,20,4z M20,8H4V6h16V8z M24,17v2 h-3v3h-2v-3h-3v-2h3v-3h2v3H24z",
                }
            }
        }
    }
}

pub fn add_home_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M15,9.5c0.34,0,0.68,0.04,1,0.1V7.5L10,3L4,7.5V17h5.88c-0.24-0.62-0.38-1.29-0.38-2C9.5,11.96,11.96,9.5,15,9.5z",
                        }
                    }
                    g {
                        path {
                            d: "M15,11c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4s4-1.79,4-4C19,12.79,17.21,11,15,11z M17,15.5h-1.5V17h-1v-1.5H13v-1h1.5 V13h1v1.5H17V15.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn add_home_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M18,11c0.7,0,1.37,0.1,2,0.29V9l-8-6L4,9v12h7.68C11.25,20.09,11,19.08,11,18C11,14.13,14.13,11,18,11z",
                        }
                    }
                    g {
                        path {
                            d: "M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M21,18.5h-2.5V21h-1v-2.5H15v-1h2.5V15h1v2.5H21V18.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn add_shopping_cart_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M0 0h24v24H0zm18.31 6l-2.76 5z",
            }
            path {
                d: "M11 9h2V6h3V4h-3V1h-2v3H8v2h3v3zm-4 9c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zm10 0c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2zm-9.83-3.25l.03-.12.9-1.63h7.45c.75 0 1.41-.41 1.75-1.03l3.86-7.01L19.42 4h-.01l-1.1 2-2.76 5H8.53l-.13-.27L6.16 6l-.95-2-.94-2H1v2h2l3.6 7.59-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h12v-2H7.42c-.13 0-.25-.11-.25-.25z",
            }
        }
    }
}

pub fn add_task_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22,5.18L10.59,16.6l-4.24-4.24l1.41-1.41l2.83,2.83l10-10L22,5.18z M12,20c-4.41,0-8-3.59-8-8s3.59-8,8-8 c1.57,0,3.04,0.46,4.28,1.25l1.45-1.45C16.1,2.67,14.13,2,12,2C6.48,2,2,6.48,2,12s4.48,10,10,10c1.73,0,3.36-0.44,4.78-1.22 l-1.5-1.5C14.28,19.74,13.17,20,12,20z M19,15h-3v2h3v3h2v-3h3v-2h-3v-3h-2V15z",
            }
        }
    }
}

pub fn add_to_drive_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M13.17,17.5H4.95c-0.54,0-1.03-0.29-1.3-0.75l-1.9-3.31c-0.27-0.46-0.27-1.04,0-1.5L6.8,3.25C7.07,2.78,7.57,2.5,8.1,2.5 h3.8c0.53,0,1.03,0.28,1.3,0.75l3.69,6.34C16.6,9.53,16.3,9.5,16,9.5c-0.29,0-0.58,0.03-0.85,0.08L11.9,4H8.1l-5.05,8.69L4.95,16 h7.02C12.26,16.59,12.66,17.09,13.17,17.5z M19,13.25h-2.25V11h-1.5v2.25H13v1.5h2.25V17h1.5v-2.25H19V13.25z M7.69,12.45L10,8.42 l2.01,3.5c0.25-0.49,0.59-0.92,1-1.28l-2.29-4H9.29l-3.61,6.3l0.57,1h5.25c0.01-0.53,0.1-1.03,0.28-1.5H7.69z",
            }
        }
    }
}

pub fn add_to_drive_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20,21v-3h3v-2h-3v-3h-2v3h-3v2h3v3H20z M15.03,21.5H5.66c-0.72,0-1.38-0.38-1.73-1L1.57,16.4c-0.36-0.62-0.35-1.38,0.01-2 L7.92,3.49C8.28,2.88,8.94,2.5,9.65,2.5h4.7c0.71,0,1.37,0.38,1.73,0.99l4.48,7.71C20.06,11.07,19.54,11,19,11 c-0.28,0-0.56,0.02-0.84,0.06L14.35,4.5h-4.7L3.31,15.41l2.35,4.09h7.89C13.9,20.27,14.4,20.95,15.03,21.5z M13.34,15 C13.12,15.63,13,16.3,13,17H7.25l-0.73-1.27l4.58-7.98h1.8l2.53,4.42c-0.56,0.42-1.05,0.93-1.44,1.51l-2-3.49L9.25,15H13.34z",
            }
        }
    }
}

pub fn admin_panel_settings_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M13,9c0.35,0,0.68,0.06,1,0.14V6.18L9,4L4,6.18v3.27c0,3.03,2.13,5.86,5,6.55c0.35-0.08,0.7-0.2,1.02-0.35 C9.39,14.94,9,14.02,9,13C9,10.79,10.79,9,13,9z",
                    }
                    path {
                        d: "M13,10c-1.66,0-3,1.34-3,3c0,1.66,1.34,3,3,3s3-1.34,3-3C16,11.34,14.66,10,13,10z M13,11.03c0.47,0,0.84,0.38,0.84,0.84 c0,0.46-0.38,0.84-0.84,0.84s-0.84-0.38-0.84-0.84C12.16,11.41,12.53,11.03,13,11.03z M13,15.06c-0.7,0-1.31-0.35-1.68-0.87 c0.04-0.54,1.13-0.81,1.68-0.81s1.64,0.27,1.68,0.81C14.31,14.72,13.7,15.06,13,15.06z",
                    }
                }
            }
        }
    }
}

pub fn admin_panel_settings_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M17,11c0.34,0,0.67,0.04,1,0.09V6.27L10.5,3L3,6.27v4.91c0,4.54,3.2,8.79,7.5,9.82c0.55-0.13,1.08-0.32,1.6-0.55 C11.41,19.47,11,18.28,11,17C11,13.69,13.69,11,17,11z",
                    }
                    path {
                        d: "M17,13c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4s4-1.79,4-4C21,14.79,19.21,13,17,13z M17,14.38c0.62,0,1.12,0.51,1.12,1.12 s-0.51,1.12-1.12,1.12s-1.12-0.51-1.12-1.12S16.38,14.38,17,14.38z M17,19.75c-0.93,0-1.74-0.46-2.24-1.17 c0.05-0.72,1.51-1.08,2.24-1.08s2.19,0.36,2.24,1.08C18.74,19.29,17.93,19.75,17,19.75z",
                    }
                }
            }
        }
    }
}

pub fn ads_click_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17.09,18.5l-3.47-3.47L12.5,18L10,10l8,2.5l-2.97,1.11l3.47,3.47L17.09,18.5z M10,3.5c-3.58,0-6.5,2.92-6.5,6.5 s2.92,6.5,6.5,6.5c0.15,0,0.3-0.01,0.45-0.02l0.46,1.46C10.61,17.98,10.31,18,10,18c-4.42,0-8-3.58-8-8s3.58-8,8-8l0,0 c4.42,0,8,3.58,8,8c0,0.31-0.02,0.61-0.05,0.91l-1.46-0.46c0.01-0.15,0.02-0.3,0.02-0.45C16.5,6.42,13.58,3.5,10,3.5 M10,6.5 c-1.93,0-3.5,1.57-3.5,3.5c0,1.76,1.31,3.23,3.01,3.47L10,15c0,0-0.01,0-0.01,0C7.23,15,5,12.76,5,10c0-2.76,2.24-5,5-5l0,0 c2.76,0,5,2.23,5,4.99c0,0,0,0.01,0,0.01l-1.53-0.49C13.23,7.81,11.76,6.5,10,6.5",
            }
        }
    }
}

pub fn ads_click_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11.71,17.99C8.53,17.84,6,15.22,6,12c0-3.31,2.69-6,6-6c3.22,0,5.84,2.53,5.99,5.71l-2.1-0.63C15.48,9.31,13.89,8,12,8 c-2.21,0-4,1.79-4,4c0,1.89,1.31,3.48,3.08,3.89L11.71,17.99z M22,12c0,0.3-0.01,0.6-0.04,0.9l-1.97-0.59C20,12.21,20,12.1,20,12 c0-4.42-3.58-8-8-8s-8,3.58-8,8s3.58,8,8,8c0.1,0,0.21,0,0.31-0.01l0.59,1.97C12.6,21.99,12.3,22,12,22C6.48,22,2,17.52,2,12 C2,6.48,6.48,2,12,2S22,6.48,22,12z M18.23,16.26L22,15l-10-3l3,10l1.26-3.77l4.27,4.27l1.98-1.98L18.23,16.26z",
            }
        }
    }
}

pub fn alarm_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M22 5.72l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM7.88 3.39L6.6 1.86 2 5.71l1.29 1.53 4.59-3.85zM12.5 8H11v6l4.75 2.85.75-1.23-4-2.37V8zM12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9c4.97 0 9-4.03 9-9s-4.03-9-9-9zm0 16c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7z",
            }
        }
    }
}

pub fn alarm_add_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M7.88 3.39L6.6 1.86 2 5.71l1.29 1.53 4.59-3.85zM22 5.72l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9c4.97 0 9-4.03 9-9s-4.03-9-9-9zm0 16c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7zm1-11h-2v3H8v2h3v3h2v-3h3v-2h-3V9z",
            }
        }
    }
}

pub fn alarm_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 6c3.87 0 7 3.13 7 7 0 .84-.16 1.65-.43 2.4l1.52 1.52c.58-1.19.91-2.51.91-3.92 0-4.97-4.03-9-9-9-1.41 0-2.73.33-3.92.91L9.6 6.43C10.35 6.16 11.16 6 12 6zm10-.28l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM2.92 2.29L1.65 3.57 2.98 4.9l-1.11.93 1.42 1.42 1.11-.94.8.8C3.83 8.69 3 10.75 3 13c0 4.97 4.02 9 9 9 2.25 0 4.31-.83 5.89-2.2l2.2 2.2 1.27-1.27L3.89 3.27l-.97-.98zm13.55 16.1C15.26 19.39 13.7 20 12 20c-3.87 0-7-3.13-7-7 0-1.7.61-3.26 1.61-4.47l9.86 9.86zM8.02 3.28L6.6 1.86l-.86.71 1.42 1.42.86-.71z",
            }
        }
    }
}

pub fn alarm_on_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M22 5.72l-4.6-3.86-1.29 1.53 4.6 3.86L22 5.72zM7.88 3.39L6.6 1.86 2 5.71l1.29 1.53 4.59-3.85zM12 4c-4.97 0-9 4.03-9 9s4.02 9 9 9c4.97 0 9-4.03 9-9s-4.03-9-9-9zm0 16c-3.87 0-7-3.13-7-7s3.13-7 7-7 7 3.13 7 7-3.13 7-7 7zm-1.46-5.47L8.41 12.4l-1.06 1.06 3.18 3.18 6-6-1.06-1.06-4.93 4.95z",
            }
        }
    }
}

pub fn all_inbox_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v7c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 6h-4c0 1.62-1.38 3-3 3s-3-1.38-3-3H5V5h14v4zm-4 7h6v3c0 1.1-.9 2-2 2H5c-1.1 0-2-.9-2-2v-3h6c0 1.66 1.34 3 3 3s3-1.34 3-3z",
            }
        }
    }
}

pub fn all_out_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16.21 4.16l4 4v-4zm4 12l-4 4h4zm-12 4l-4-4v4zm-4-12l4-4h-4zm12.95-.95c-2.73-2.73-7.17-2.73-9.9 0s-2.73 7.17 0 9.9 7.17 2.73 9.9 0 2.73-7.16 0-9.9zm-1.1 8.8c-2.13 2.13-5.57 2.13-7.7 0s-2.13-5.57 0-7.7 5.57-2.13 7.7 0 2.13 5.57 0 7.7z",
            }
            path {
                d: "M.21.16h24v24h-24z",
                fill: "none",
            }
        }
    }
}

pub fn analytics_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-5h2v5zm4 0h-2v-3h2v3zm0-5h-2v-2h2v2zm4 5h-2V7h2v10z",
            }
        }
    }
}

pub fn anchor_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,15l1.55,1.55c-0.96,1.69-3.33,3.04-5.55,3.37V11h3V9h-3V7.82C14.16,7.4,15,6.3,15,5c0-1.65-1.35-3-3-3S9,3.35,9,5 c0,1.3,0.84,2.4,2,2.82V9H8v2h3v8.92c-2.22-0.33-4.59-1.68-5.55-3.37L7,15l-4-3v3c0,3.88,4.92,7,9,7s9-3.12,9-7v-3L17,15z M12,4 c0.55,0,1,0.45,1,1s-0.45,1-1,1s-1-0.45-1-1S11.45,4,12,4z",
                }
            }
        }
    }
}

pub fn android_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 20 20",
            g {
                g {
                    g {
                        rect {
                            width: "20",
                            fill: "none",
                            height: "20",
                        }
                    }
                }
            }
            g {
                path {
                    d: "M14.36,8.14l1.57-2.72c0.14-0.24,0.06-0.54-0.18-0.68c-0.24-0.14-0.54-0.06-0.68,0.18l-1.6,2.78C12.4,7.25,11.23,7,10,7 S7.6,7.25,6.54,7.7l-1.6-2.78C4.8,4.68,4.49,4.6,4.25,4.74C4.01,4.87,3.93,5.18,4.07,5.42l1.57,2.72C3.15,9.52,1.39,12.04,1.06,15 h17.88C18.61,12.04,16.85,9.52,14.36,8.14z M6,13c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S6.55,13,6,13z M14,13 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S14.55,13,14,13z",
                }
            }
        }
    }
}

pub fn android_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    path {
                        fill: "none",
                        d: "M0,0h24v24H0V0z",
                    }
                }
            }
            g {
                g {
                    path {
                        d: "M17.6,9.48l1.84-3.18c0.16-0.31,0.04-0.69-0.26-0.85c-0.29-0.15-0.65-0.06-0.83,0.22l-1.88,3.24 c-2.86-1.21-6.08-1.21-8.94,0L5.65,5.67c-0.19-0.29-0.58-0.38-0.87-0.2C4.5,5.65,4.41,6.01,4.56,6.3L6.4,9.48 C3.3,11.25,1.28,14.44,1,18h22C22.72,14.44,20.7,11.25,17.6,9.48z M7,15.25c-0.69,0-1.25-0.56-1.25-1.25 c0-0.69,0.56-1.25,1.25-1.25S8.25,13.31,8.25,14C8.25,14.69,7.69,15.25,7,15.25z M17,15.25c-0.69,0-1.25-0.56-1.25-1.25 c0-0.69,0.56-1.25,1.25-1.25s1.25,0.56,1.25,1.25C18.25,14.69,17.69,15.25,17,15.25z",
                    }
                }
            }
        }
    }
}

pub fn announcement_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 9h-2V5h2v6zm0 4h-2v-2h2v2z",
            }
        }
    }
}

pub fn api_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,12l-2,2l-2-2l2-2L14,12z M12,6l2.12,2.12l2.5-2.5L12,1L7.38,5.62l2.5,2.5L12,6z M6,12l2.12-2.12l-2.5-2.5L1,12 l4.62,4.62l2.5-2.5L6,12z M18,12l-2.12,2.12l2.5,2.5L23,12l-4.62-4.62l-2.5,2.5L18,12z M12,18l-2.12-2.12l-2.5,2.5L12,23l4.62-4.62 l-2.5-2.5L12,18z",
                }
            }
        }
    }
}

pub fn app_blocking_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18 8c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm-2.5 4c0-1.38 1.12-2.5 2.5-2.5.42 0 .8.11 1.15.29l-3.36 3.36c-.18-.35-.29-.73-.29-1.15zm2.5 2.5c-.42 0-.8-.11-1.15-.29l3.36-3.36c.18.35.29.73.29 1.15 0 1.38-1.12 2.5-2.5 2.5zM17 18H7V6h10v1h2V3c0-1.1-.9-2-2-2H7c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2v-4h-2v1z",
            }
        }
    }
}

pub fn app_shortcut_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14.5,15h-9V5h9v1H16V2.5C16,1.67,15.33,1,14.5,1h-9C4.67,1,4,1.67,4,2.5v15C4,18.33,4.67,19,5.5,19h9 c0.83,0,1.5-0.67,1.5-1.5V14h-1.5V15z",
                    }
                    polygon {
                        points: "17.03,7.97 17.5,9 17.97,7.97 19,7.5 17.97,7.03 17.5,6 17.03,7.03 16,7.5",
                    }
                    polygon {
                        points: "13,7 12.06,9.06 10,10 12.06,10.94 13,13 13.94,10.94 16,10 13.94,9.06",
                    }
                    polygon {
                        points: "17.5,11 17.03,12.03 16,12.5 17.03,12.97 17.5,14 17.97,12.97 19,12.5 17.97,12.03",
                    }
                }
            }
        }
    }
}

pub fn app_shortcut_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M17,18H7V6h10v1h2V3c0-1.1-0.9-2-2-2H7C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2v-4h-2V18z",
                    }
                    polygon {
                        points: "20.38,9.62 21,11 21.62,9.62 23,9 21.62,8.38 21,7 20.38,8.38 19,9",
                    }
                    polygon {
                        points: "16,8 14.75,10.75 12,12 14.75,13.25 16,16 17.25,13.25 20,12 17.25,10.75",
                    }
                    polygon {
                        points: "21,13 20.38,14.38 19,15 20.38,15.62 21,17 21.62,15.62 23,15 21.62,14.38",
                    }
                }
            }
        }
    }
}

pub fn arrow_circle_down_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M10,16c-3.31,0-6-2.69-6-6s2.69-6,6-6s6,2.69,6,6S13.31,16,10,16 M10,17c3.87,0,7-3.13,7-7c0-3.87-3.13-7-7-7 c-3.87,0-7,3.13-7,7C3,13.87,6.13,17,10,17L10,17z M10.5,10V7h-1v3H7l3,3l3-3H10.5z",
                }
            }
        }
    }
}

pub fn arrow_circle_down_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8s-8-3.59-8-8S7.59,4,12,4 M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10 c5.52,0,10-4.48,10-10C22,6.48,17.52,2,12,2L12,2z M13,12l0-4h-2l0,4H8l4,4l4-4H13z",
                }
            }
        }
    }
}

pub fn arrow_circle_left_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M2,10c0,4.42,3.58,8,8,8s8-3.58,8-8s-3.58-8-8-8S2,5.58,2,10z M10,9.25l3,0v1.5l-3,0L10,13l-3-3l3-3L10,9.25z",
                }
            }
        }
    }
}

pub fn arrow_circle_left_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M2,12c0,5.52,4.48,10,10,10s10-4.48,10-10c0-5.52-4.48-10-10-10C6.48,2,2,6.48,2,12z M12,11l4,0v2l-4,0l0,3l-4-4l4-4L12,11 z",
                }
            }
        }
    }
}

pub fn arrow_circle_right_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M18,10c0-4.42-3.58-8-8-8s-8,3.58-8,8s3.58,8,8,8S18,14.42,18,10z M10,10.75H7v-1.5h3V7l3,3l-3,3V10.75z",
                }
            }
        }
    }
}

pub fn arrow_circle_right_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M22,12c0-5.52-4.48-10-10-10S2,6.48,2,12c0,5.52,4.48,10,10,10S22,17.52,22,12z M12,13l-4,0v-2l4,0V8l4,4l-4,4V13z",
                }
            }
        }
    }
}

pub fn arrow_circle_up_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M10,4c3.31,0,6,2.69,6,6s-2.69,6-6,6s-6-2.69-6-6S6.69,4,10,4 M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7 c3.87,0,7-3.13,7-7C17,6.13,13.87,3,10,3L10,3z M9.5,10v3h1v-3H13l-3-3l-3,3H9.5z",
                }
            }
        }
    }
}

pub fn arrow_circle_up_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M12,20c-4.41,0-8-3.59-8-8s3.59-8,8-8s8,3.59,8,8S16.41,20,12,20 M12,22c5.52,0,10-4.48,10-10c0-5.52-4.48-10-10-10 C6.48,2,2,6.48,2,12C2,17.52,6.48,22,12,22L12,22z M11,12l0,4h2l0-4h3l-4-4l-4,4H11z",
                }
            }
        }
    }
}

pub fn arrow_outward_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                polygon {
                    points: "5,5 5,6.5 12.44,6.5 4,14.94 5.06,16 13.5,7.56 13.5,15 15,15 15,5",
                }
            }
        }
    }
}

pub fn arrow_outward_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                polygon {
                    points: "6,6 6,8 14.59,8 5,17.59 6.41,19 16,9.41 16,18 18,18 18,6",
                }
            }
        }
    }
}

pub fn arrow_right_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16.01 11H4v2h12.01v3L20 12l-3.99-4z",
            }
        }
    }
}

pub fn article_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z",
            }
        }
    }
}

pub fn aspect_ratio_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 12h-2v3h-3v2h5v-5zM7 9h3V7H5v5h2V9zm14-6H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02z",
            }
        }
    }
}

pub fn assessment_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z",
            }
        }
    }
}

pub fn assignment_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm2 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z",
            }
        }
    }
}

pub fn assignment_add_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M9.6 14H6v-1.5h4.11c.35-.68.82-1.27 1.41-1.75H6v-1.5h8v.35c.32-.06.66-.1 1-.1.71 0 1.38.14 2 .38V4.5c0-.83-.67-1.5-1.5-1.5h-3.57c-.22-.86-1-1.5-1.93-1.5-.93 0-1.71.64-1.93 1.5H4.5C3.67 3 3 3.67 3 4.5v11c0 .83.67 1.5 1.5 1.5h5.38c-.24-.62-.38-1.29-.38-2 0-.34.04-.68.1-1zM10 3c.28 0 .5.22.5.5s-.22.5-.5.5-.5-.22-.5-.5.22-.5.5-.5zM6 6h8v1.5H6V6z",
                        }
                    }
                    path {
                        d: "M15 11c-2.21 0-4 1.79-4 4s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm2 4.5h-1.5V17h-1v-1.5H13v-1h1.5V13h1v1.5H17v1z",
                    }
                }
            }
        }
    }
}

pub fn assignment_add_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M18 13c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zm3 5.5h-2.5V21h-1v-2.5H15v-1h2.5V15h1v2.5H21v1z",
                        }
                    }
                    g {
                        path {
                            d: "M11.69 15c.36-.75.84-1.43 1.43-2H7v-2h11c1.07 0 2.09.25 3 .69V5c0-1.1-.9-2-2-2h-4.18C14.4 1.84 13.3 1 12 1s-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h6.69c-.44-.91-.69-1.93-.69-3 0-.34.03-.67.08-1H7v-2h4.69zM12 2.75c.41 0 .75.34.75.75s-.34.75-.75.75-.75-.34-.75-.75.34-.75.75-.75zM7 7h10v2H7V7z",
                        }
                    }
                }
            }
        }
    }
}

pub fn assignment_ind_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm0 4c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H6v-1.4c0-2 4-3.1 6-3.1s6 1.1 6 3.1V19z",
            }
        }
    }
}

pub fn assignment_late_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-6 15h-2v-2h2v2zm0-4h-2V8h2v6zm-1-9c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

pub fn assignment_return_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm4 12h-4v3l-5-5 5-5v3h4v4z",
            }
        }
    }
}

pub fn assignment_returned_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm0 15l-5-5h3V9h4v4h3l-5 5z",
            }
        }
    }
}

pub fn assignment_turned_in_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3h-4.18C14.4 1.84 13.3 1 12 1c-1.3 0-2.4.84-2.82 2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm-2 14l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z",
            }
        }
    }
}

pub fn assured_workload_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    rect {
                        width: "1.5",
                        x: "9.25",
                        y: "8",
                        height: "6.5",
                    }
                    rect {
                        x: "4",
                        y: "8",
                        height: "6.5",
                        width: "1.5",
                    }
                    polygon {
                        points: "18,5 10,1 2,5 2,7 18,7",
                    }
                    path {
                        d: "M12.5,15.5H2V17h10.66C12.61,16.78,12.5,16.29,12.5,15.5z",
                    }
                    polygon {
                        points: "16,10.82 16,8 14.5,8 14.5,11.57",
                    }
                    path {
                        d: "M17,12l-3,1.5v2.14c0,2.02,1.28,3.91,3,4.36c1.72-0.46,3-2.35,3-4.36V13.5L17,12z M16.43,17.5l-1.55-1.54l0.71-0.71 l0.82,0.81L18.39,14l0.73,0.68L16.43,17.5z",
                    }
                }
            }
        }
    }
}

pub fn assured_workload_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        y: "10",
                        width: "2",
                        height: "7",
                        x: "5",
                    }
                    rect {
                        y: "10",
                        x: "11",
                        width: "2",
                        height: "7",
                    }
                    polygon {
                        points: "22,6 12,1 2,6 2,8 22,8",
                    }
                    path {
                        d: "M2,19v2h12.4c-0.21-0.64-0.32-1.31-0.36-2H2z",
                    }
                    polygon {
                        points: "19,12.26 19,10 17,10 17,13.26",
                    }
                    path {
                        d: "M20,14l-4,2v2.55c0,2.52,1.71,4.88,4,5.45c2.29-0.57,4-2.93,4-5.45V16L20,14z M19.28,21l-2.03-2.03l1.06-1.06l0.97,0.97 l2.41-2.38l1.06,1.06L19.28,21z",
                    }
                }
            }
        }
    }
}

pub fn autorenew_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 6v3l4-4-4-4v3c-4.42 0-8 3.58-8 8 0 1.57.46 3.03 1.24 4.26L6.7 14.8c-.45-.83-.7-1.79-.7-2.8 0-3.31 2.69-6 6-6zm6.76 1.74L17.3 9.2c.44.84.7 1.79.7 2.8 0 3.31-2.69 6-6 6v-3l-4 4 4 4v-3c4.42 0 8-3.58 8-8 0-1.57-.46-3.03-1.24-4.26z",
            }
        }
    }
}

pub fn backup_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19.35 10.04C18.67 6.59 15.64 4 12 4 9.11 4 6.6 5.64 5.35 8.04 2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h13c2.76 0 5-2.24 5-5 0-2.64-2.05-4.78-4.65-4.96zM14 13v4h-4v-4H7l5-5 5 5h-3z",
            }
        }
    }
}

pub fn backup_table_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M16,6v10H6v1h10c0.55,0,1-0.45,1-1V6H16z",
                        }
                    }
                    g {
                        path {
                            d: "M14,13V4c0-0.55-0.45-1-1-1H4C3.45,3,3,3.45,3,4v9c0,0.55,0.45,1,1,1h9C13.55,14,14,13.55,14,13z M4,4h9v4H4V4z M8,13H4 V9h4V13z M9,9h4v4H9V9z",
                        }
                    }
                }
            }
        }
    }
}

pub fn backup_table_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,6v14H6v2h14c1.1,0,2-0.9,2-2V6H20z",
                    }
                    path {
                        d: "M16,2H4C2.9,2,2,2.9,2,4v12c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C18,2.9,17.1,2,16,2z M9,16H4v-5h5V16z M16,16h-5v-5h5 V16z M16,9H4V4h12V9z",
                    }
                }
            }
        }
    }
}

pub fn balance_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.75,6.37C11.39,6.15,11.9,5.64,12.12,5H15l-2.5,5.75c0,1.24,1.23,2.25,2.75,2.25c1.52,0,2.75-1.01,2.75-2.25L15.5,5H17 V3.5h-4.88C11.81,2.63,10.98,2,10,2S8.19,2.63,7.88,3.5H3V5h1.5L2,10.75C2,11.99,3.23,13,4.75,13s2.75-1.01,2.75-2.25L5,5h2.88 C8.1,5.64,8.61,6.15,9.25,6.37v9.13H2V17h16v-1.5h-7.25V6.37z M16.91,10.75h-3.32l1.66-3.82L16.91,10.75z M6.41,10.75H3.09 l1.66-3.82L6.41,10.75z M10,5C9.59,5,9.25,4.66,9.25,4.25C9.25,3.84,9.59,3.5,10,3.5s0.75,0.34,0.75,0.75C10.75,4.66,10.41,5,10,5z",
                }
            }
        }
    }
}

pub fn balance_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13,7.83c0.85-0.3,1.53-0.98,1.83-1.83H18l-3,7c0,1.66,1.57,3,3.5,3s3.5-1.34,3.5-3l-3-7h2V4h-6.17 C14.42,2.83,13.31,2,12,2S9.58,2.83,9.17,4L3,4v2h2l-3,7c0,1.66,1.57,3,3.5,3S9,14.66,9,13L6,6h3.17c0.3,0.85,0.98,1.53,1.83,1.83 V19H2v2h20v-2h-9V7.83z M20.37,13h-3.74l1.87-4.36L20.37,13z M7.37,13H3.63L5.5,8.64L7.37,13z M12,6c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C13,5.55,12.55,6,12,6z",
                }
            }
        }
    }
}

pub fn barcode_reader_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18.5 7.5h-3v-1h3v1zm-.33 2.59-2.75-1.28-.42.91L17.75 11l.42-.91zm-.42-6.84L15 4.53l.42.91 2.75-1.28-.42-.91zM7.59 16.27A2.335 2.335 0 015.34 18c-.2 0-.4-.03-.61-.08a2.338 2.338 0 01-1.65-2.86l1.46-5.44A2.987 2.987 0 013 7c0-1.66 1.34-3 3-3h6.5c1.12 0 1.84 1.17 1.34 2.17l-1.5 3c-.25.51-.77.83-1.34.83H9.27L9 11c.55 0 1 .45 1 1v1c0 .55-.45 1-1 1h-.8l-.61 2.27z",
                    }
                }
            }
        }
    }
}

pub fn barcode_reader_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M23 8.75h-4v-1.5h4v1.5zm-.38-4.38L22 3l-3.62 1.63L19 6l3.62-1.63zM19 10l-.62 1.37L22 13l.62-1.37L19 10zm-8 3c.55 0 1 .45 1 1v2c0 .55-.45 1-1 1H9.64l-.74 2.78A2.999 2.999 0 016 22a2.995 2.995 0 01-2.9-3.77l1.82-6.8A4.028 4.028 0 013 8c0-2.21 1.79-4 4-4h8c1.49 0 2.45 1.56 1.79 2.89l-2 4C14.45 11.57 13.75 12 13 12h-2.02l-.27 1H11z",
                }
            }
        }
    }
}

pub fn batch_prediction_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,7H6C5.45,7,5,7.45,5,8v8c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1V8C15,7.45,14.55,7,14,7z M10.75,16h-1.5v-1h1.5V16z M10.75,14c0,0-1.15,0-1.5,0c0-1-1.75-2-1.75-3.5C7.5,9.12,8.62,8,10,8c0,0,0,0,0,0c1.38,0,2.5,1.12,2.5,2.5 C12.5,12,10.75,13,10.75,14z M14,6H6v0c0-0.55,0.45-1,1-1h6C13.55,5,14,5.45,14,6L14,6z M13,4H7v0c0-0.55,0.45-1,1-1h4 C12.55,3,13,3.45,13,4L13,4z",
                }
            }
        }
    }
}

pub fn batch_prediction_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                    fill: "none",
                    height: "24",
                }
                path {
                    d: "M17,8H7c-1.1,0-2,0.9-2,2v10c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V10C19,8.9,18.1,8,17,8z M13,20.5h-2V19h2V20.5z M13,18h-2 c0-1.5-2.5-3-2.5-5c0-1.93,1.57-3.5,3.5-3.5c1.93,0,3.5,1.57,3.5,3.5C15.5,15,13,16.5,13,18z M18,6.5H6v0C6,5.67,6.67,5,7.5,5h9 C17.33,5,18,5.67,18,6.5L18,6.5z M17,3.5H7v0C7,2.67,7.67,2,8.5,2h7C16.33,2,17,2.67,17,3.5L17,3.5z",
                }
            }
        }
    }
}

pub fn book_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4z",
            }
        }
    }
}

pub fn bookmark_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn bookmarks_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 18l2 1V3c0-1.1-.9-2-2-2H8.99C7.89 1 7 1.9 7 3h10c1.1 0 2 .9 2 2v13zM15 5H5c-1.1 0-2 .9-2 2v16l7-3 7 3V7c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn bookmark_add_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M15.75,4.25v-1.5h-1.5v1.5h-1.5v1.5h1.5v1.5h1.5v-1.5h1.5v-1.5H15.75z M11,5c0,2.21,1.79,4,4,4v8l-5-2l-5,2V4.5 C5,3.67,5.67,3,6.5,3l5.04,0C11.19,3.59,11,4.27,11,5z",
            }
        }
    }
}

pub fn bookmark_add_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21,7h-2v2h-2V7h-2V5h2V3h2v2h2V7z M19,21l-7-3l-7,3V5c0-1.1,0.9-2,2-2l7,0c-0.63,0.84-1,1.87-1,3c0,2.76,2.24,5,5,5 c0.34,0,0.68-0.03,1-0.1V21z",
            }
        }
    }
}

pub fn bookmark_added_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19,21l-7-3l-7,3V5c0-1.1,0.9-2,2-2l7,0c-0.63,0.84-1,1.87-1,3c0,2.76,2.24,5,5,5c0.34,0,0.68-0.03,1-0.1V21z M17.83,9 L15,6.17l1.41-1.41l1.41,1.41l3.54-3.54l1.41,1.41L17.83,9z",
            }
        }
    }
}

pub fn bookmark_border_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2zm0 15l-5-2.18L7 18V5h10v13z",
            }
        }
    }
}

pub fn bookmark_remove_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M12.75,4.25v1.5h4.5v-1.5H12.75z M11,5c0,2.21,1.79,4,4,4v8l-5-2l-5,2V4.5C5,3.67,5.67,3,6.5,3l5.04,0 C11.19,3.59,11,4.27,11,5z",
            }
        }
    }
}

pub fn bookmark_remove_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21,7h-6V5h6V7z M19,10.9c-0.32,0.07-0.66,0.1-1,0.1c-2.76,0-5-2.24-5-5c0-1.13,0.37-2.16,1-3L7,3C5.9,3,5,3.9,5,5v16l7-3 l7,3V10.9z",
            }
        }
    }
}

pub fn book_online_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                    d: "M17,1H7C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V3C19,1.9,18.1,1,17,1z M7,18V6h10v12H7z M16,11V9.14 C16,8.51,15.55,8,15,8H9C8.45,8,8,8.51,8,9.14l0,1.96c0.55,0,1,0.45,1,1c0,0.55-0.45,1-1,1l0,1.76C8,15.49,8.45,16,9,16h6 c0.55,0,1-0.51,1-1.14V13c-0.55,0-1-0.45-1-1C15,11.45,15.45,11,16,11z M12.5,14.5h-1v-1h1V14.5z M12.5,12.5h-1v-1h1V12.5z M12.5,10.5h-1v-1h1V10.5z",
                }
            }
        }
    }
}

pub fn browse_gallery_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14,3.08V4.6c2.56,0.47,4.5,2.71,4.5,5.4s-1.94,4.93-4.5,5.4v1.52c3.39-0.49,6-3.39,6-6.92C20,6.47,17.39,3.57,14,3.08z",
                    }
                    path {
                        d: "M7,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C14,6.13,10.87,3,7,3z M9.08,12.83L6.25,10V6h1.5v3.38l2.39,2.39 L9.08,12.83z",
                    }
                }
            }
        }
    }
}

pub fn browse_gallery_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M9,3c-4.97,0-9,4.03-9,9s4.03,9,9,9s9-4.03,9-9S13.97,3,9,3z M11.79,16.21L8,12.41V7h2v4.59l3.21,3.21L11.79,16.21z",
                    }
                    path {
                        d: "M17.99,3.52v2.16C20.36,6.8,22,9.21,22,12c0,2.79-1.64,5.2-4.01,6.32v2.16C21.48,19.24,24,15.91,24,12 C24,8.09,21.48,4.76,17.99,3.52z",
                    }
                }
            }
        }
    }
}

pub fn bug_report_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 8h-2.81c-.45-.78-1.07-1.45-1.82-1.96L17 4.41 15.59 3l-2.17 2.17C12.96 5.06 12.49 5 12 5c-.49 0-.96.06-1.41.17L8.41 3 7 4.41l1.62 1.63C7.88 6.55 7.26 7.22 6.81 8H4v2h2.09c-.05.33-.09.66-.09 1v1H4v2h2v1c0 .34.04.67.09 1H4v2h2.81c1.04 1.79 2.97 3 5.19 3s4.15-1.21 5.19-3H20v-2h-2.09c.05-.33.09-.66.09-1v-1h2v-2h-2v-1c0-.34-.04-.67-.09-1H20V8zm-6 8h-4v-2h4v2zm0-4h-4v-2h4v2z",
            }
        }
    }
}

pub fn build_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                clip_rule: "evenodd",
            }
            path {
                d: "M22.7 19l-9.1-9.1c.9-2.3.4-5-1.5-6.9-2-2-5-2.4-7.4-1.3L9 6 6 9 1.6 4.7C.4 7.1.9 10.1 2.9 12.1c1.9 1.9 4.6 2.4 6.9 1.5l9.1 9.1c.4.4 1 .4 1.4 0l2.3-2.3c.5-.4.5-1.1.1-1.4z",
            }
        }
    }
}

pub fn build_circle_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,3c-3.86,0-7,3.14-7,7s3.14,7,7,7s7-3.14,7-7S13.86,3,10,3z M12.02,13.5 l-2.26-2.26c-0.84,0.26-1.79,0.08-2.45-0.59C6.54,9.88,6.4,8.73,6.88,7.81l1.63,1.63l1.03-1.03L7.91,6.78 c0.92-0.48,2.07-0.34,2.84,0.43c0.74,0.74,0.9,1.84,0.48,2.74l2.17,2.17L12.02,13.5z",
                        fill_rule: "evenodd",
                    }
                }
            }
        }
    }
}

pub fn build_circle_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        fill_rule: "evenodd",
                        d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10 C22,6.48,17.52,2,12,2z M16.9,15.49l-1.4,1.4c-0.2,0.2-0.51,0.2-0.71,0l-3.41-3.41c-1.22,0.43-2.64,0.17-3.62-0.81 c-1.11-1.11-1.3-2.79-0.59-4.1l2.35,2.35l1.41-1.41L8.58,7.17c1.32-0.71,2.99-0.52,4.1,0.59c0.98,0.98,1.24,2.4,0.81,3.62 l3.41,3.41C17.09,14.98,17.09,15.3,16.9,15.49z",
                    }
                }
            }
        }
    }
}

pub fn cached_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "m19 8-4 4h3c0 3.31-2.69 6-6 6-1.01 0-1.97-.25-2.8-.7l-1.46 1.46C8.97 19.54 10.43 20 12 20c4.42 0 8-3.58 8-8h3l-4-4zM6 12c0-3.31 2.69-6 6-6 1.01 0 1.97.25 2.8.7l1.46-1.46C15.03 4.46 13.57 4 12 4c-4.42 0-8 3.58-8 8H1l4 4 4-4H6z",
            }
        }
    }
}

pub fn calendar_month_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.5,4H14V2h-1.5v2h-5V2H6v2H4.5C3.67,4,3,4.68,3,5.5v11C3,17.32,3.67,18,4.5,18h11c0.83,0,1.5-0.68,1.5-1.5v-11 C17,4.68,16.33,4,15.5,4z M15.5,16.5h-11V9h11V16.5z M7.5,12H6v-1.5h1.5V12z M10.75,12h-1.5v-1.5h1.5V12z M14,12h-1.5v-1.5H14V12z M7.5,15H6v-1.5h1.5V15z M10.75,15h-1.5v-1.5h1.5V15z M14,15h-1.5v-1.5H14V15z",
                }
            }
        }
    }
}

pub fn calendar_month_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,4h-1V2h-2v2H8V2H6v2H5C3.89,4,3.01,4.9,3.01,6L3,20c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V6C21,4.9,20.1,4,19,4z M19,20 H5V10h14V20z M9,14H7v-2h2V14z M13,14h-2v-2h2V14z M17,14h-2v-2h2V14z M9,18H7v-2h2V18z M13,18h-2v-2h2V18z M17,18h-2v-2h2V18z",
                }
            }
        }
    }
}

pub fn calendar_today_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 3h-1V1h-2v2H7V1H5v2H4c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 18H4V8h16v13z",
            }
        }
    }
}

pub fn calendar_view_day_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 17h18v2H3zm0-7h18v5H3zm0-4h18v2H3z",
            }
        }
    }
}

pub fn calendar_view_month_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M8,11H4V6h4V11z M14,11h-4V6h4V11z M20,11h-4V6h4V11z M8,18H4v-5h4V18z M14,18h-4v-5h4V18z M20,18h-4v-5h4V18z",
                }
            }
        }
    }
}

pub fn calendar_view_week_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M13,6h2.5v12H13V6z M11,18H8.5V6H11 V18z M4,6h2.5v12H4V6z M20,18h-2.5V6H20V18z",
                }
            }
        }
    }
}

pub fn camera_enhance_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M9 3L7.17 5H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2h-3.17L15 3H9zm3 15c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5z",
            }
            path {
                d: "M12 17l1.25-2.75L16 13l-2.75-1.25L12 9l-1.25 2.75L8 13l2.75 1.25z",
            }
        }
    }
}

pub fn cancel_schedule_send_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                x: "0",
                y: "0",
            }
            g {
                path {
                    d: "M2,12.31l6.91-1.48c-0.38,0.6-0.65,1.27-0.79,1.99l-0.51,0.2L2,15.47V12.31z M2,7.69V4.53l9.35,4.09 c-0.44,0.22-0.86,0.49-1.23,0.81L2,7.69z",
                }
                path {
                    d: "M3,6.06l3.7,1.62L3,6.88V6.06 M6.7,12.32L3,13.94v-0.83L6.7,12.32 M1,3v5.5L8,10l-7,1.5V17l7-3.06c0,0.02,0,0.04,0,0.06 c0,3.31,2.69,6,6,6s6-2.69,6-6s-2.69-6-6-6c-0.43,0-0.85,0.05-1.26,0.14L1,3L1,3z M14,19c-2.76,0-5-2.24-5-5s2.24-5,5-5s5,2.24,5,5 S16.76,19,14,19L14,19z",
                }
            }
            polygon {
                points: "15.41,11.88 14,13.29 12.59,11.88 11.88,12.59 13.29,14 11.88,15.41 12.59,16.12 14,14.71 15.41,16.12 16.12,15.41 14.71,14 16.12,12.59",
            }
        }
    }
}

pub fn cancel_schedule_send_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.5,9c-0.42,0-0.83,0.04-1.24,0.11L1.01,3L1,10l9,2l-9,2l0.01,7l8.07-3.46C9.59,21.19,12.71,24,16.5,24 c4.14,0,7.5-3.36,7.5-7.5S20.64,9,16.5,9z M16.5,22c-3.03,0-5.5-2.47-5.5-5.5s2.47-5.5,5.5-5.5s5.5,2.47,5.5,5.5S19.53,22,16.5,22 z",
                    }
                    polygon {
                        points: "18.27,14.03 16.5,15.79 14.73,14.03 14.03,14.73 15.79,16.5 14.03,18.27 14.73,18.97 16.5,17.21 18.27,18.97 18.97,18.27 17.21,16.5 18.97,14.73",
                    }
                }
            }
        }
    }
}

pub fn card_giftcard_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 6h-2.18c.11-.31.18-.65.18-1 0-1.66-1.34-3-3-3-1.05 0-1.96.54-2.5 1.35l-.5.67-.5-.68C10.96 2.54 10.05 2 9 2 7.34 2 6 3.34 6 5c0 .35.07.69.18 1H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-5-2c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zM9 4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm11 15H4v-2h16v2zm0-5H4V8h5.08L7 10.83 8.62 12 11 8.76l1-1.36 1 1.36L15.38 12 17 10.83 14.92 8H20v6z",
            }
        }
    }
}

pub fn card_membership_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 2H4c-1.11 0-2 .89-2 2v11c0 1.11.89 2 2 2h4v5l4-2 4 2v-5h4c1.11 0 2-.89 2-2V4c0-1.11-.89-2-2-2zm0 13H4v-2h16v2zm0-5H4V4h16v6z",
            }
        }
    }
}

pub fn card_travel_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 6h-3V4c0-1.11-.89-2-2-2H9c-1.11 0-2 .89-2 2v2H4c-1.11 0-2 .89-2 2v11c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zM9 4h6v2H9V4zm11 15H4v-2h16v2zm0-5H4V8h3v2h2V8h6v2h2V8h3v6z",
            }
        }
    }
}

pub fn change_history_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 7.77L18.39 18H5.61L12 7.77M12 4L2 20h20L12 4z",
            }
        }
    }
}

pub fn check_circle_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z",
            }
        }
    }
}

pub fn check_circle_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M0 0h24v24H0V0zm0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M16.59 7.58L10 14.17l-3.59-3.58L5 12l5 5 8-8zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
        }
    }
}

pub fn chrome_reader_mode_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M-74 29h48v48h-48V29zM0 0h24v24H0V0zm0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M13 12h7v1.5h-7zm0-2.5h7V11h-7zm0 5h7V16h-7zM21 4H3c-1.1 0-2 .9-2 2v13c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 15h-9V6h9v13z",
            }
        }
    }
}

pub fn circle_notifications_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 16.5c-.83 0-1.5-.67-1.5-1.5h3c0 .83-.67 1.5-1.5 1.5zm5-2.5H7v-1l1-1v-2.61C8 9.27 9.03 7.47 11 7v-.5c0-.57.43-1 1-1s1 .43 1 1V7c1.97.47 3 2.28 3 4.39V14l1 1v1z",
            }
        }
    }
}

pub fn class_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM6 4h5v8l-2.5-1.5L6 12V4z",
            }
        }
    }
}

pub fn close_fullscreen_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22,3.41l-5.29,5.29L20,12h-8V4l3.29,3.29L20.59,2L22,3.41z M3.41,22l5.29-5.29L12,20v-8H4l3.29,3.29L2,20.59L3.41,22z",
            }
        }
    }
}

pub fn code_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M9.4 16.6L4.8 12l4.6-4.6L8 6l-6 6 6 6 1.4-1.4zm5.2 0l4.6-4.6-4.6-4.6L16 6l6 6-6 6-1.4-1.4z",
            }
        }
    }
}

pub fn code_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M18,10l-2.94,2.94L14,11.88L15.88,10l-3.94-3.94L13,5L18,10z M16.01,18.13l1.06-1.06L2.93,2.93L1.87,3.99l3.07,3.07L2,10 l5,5l1.06-1.06L4.12,10L6,8.12L16.01,18.13z",
            }
        }
    }
}

pub fn code_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19.17,12l-4.58-4.59L16,6l6,6l-3.59,3.59L17,14.17L19.17,12z M1.39,4.22l4.19,4.19L2,12l6,6l1.41-1.41L4.83,12L7,9.83 l12.78,12.78l1.41-1.41L2.81,2.81L1.39,4.22z",
            }
        }
    }
}

pub fn comment_bank_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,3H4C3.45,3,3,3.45,3,4v13l3-3h10c0.55,0,1-0.45,1-1V4C17,3.45,16.55,3,16,3z M15,11l-2-1.31L11,11V5h4V11z",
                }
            }
        }
    }
}

pub fn comment_bank_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,2H4C2.9,2,2,2.9,2,4v18l4-4h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M19,13l-2.5-1.5L14,13V5h5V13z",
                }
            }
        }
    }
}

pub fn commit_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13.93,9.25C13.58,7.4,11.95,6,10,6S6.42,7.4,6.07,9.25H2v1.5h4.07C6.42,12.6,8.05,14,10,14s3.58-1.4,3.93-3.25H18v-1.5 H13.93z M10,12.5c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5s2.5,1.12,2.5,2.5C12.5,11.38,11.38,12.5,10,12.5z",
                }
            }
        }
    }
}

pub fn commit_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.9,11L16.9,11c-0.46-2.28-2.48-4-4.9-4s-4.44,1.72-4.9,4h0H2v2h5.1h0c0.46,2.28,2.48,4,4.9,4s4.44-1.72,4.9-4h0H22v-2 H16.9z M12,15c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3S13.66,15,12,15z",
                }
            }
        }
    }
}

pub fn commute_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 4H5C3.34 4 2 5.34 2 7v8c0 1.66 1.34 3 3 3l-1 1v1h1l2-2.03L9 18v-5H4V5.98L13 6v2h2V7c0-1.66-1.34-3-3-3zM5 14c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm15.57-4.34c-.14-.4-.52-.66-.97-.66h-7.19c-.46 0-.83.26-.98.66L10 13.77l.01 5.51c0 .38.31.72.69.72h.62c.38 0 .68-.38.68-.76V18h8v1.24c0 .38.31.76.69.76h.61c.38 0 .69-.34.69-.72l.01-1.37v-4.14l-1.43-4.11zm-8.16.34h7.19l1.03 3h-9.25l1.03-3zM12 16c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm8 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

pub fn compare_arrows_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    x: "0",
                    height: "24",
                    width: "24",
                    fill: "none",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M9.01,14H2v2h7.01v3L13,15l-3.99-4V14z M14.99,13v-3H22V8h-7.01V5L11,9L14.99,13z",
                        }
                    }
                }
            }
        }
    }
}

pub fn compress_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M8 19h3v3h2v-3h3l-4-4-4 4zm8-15h-3V1h-2v3H8l4 4 4-4zM4 9v2h16V9H4z",
            }
            path {
                d: "M4 12h16v2H4z",
            }
        }
    }
}

pub fn contactless_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C17,6.13,13.87,3,10,3z M7.63,11.71l-0.91-0.41 C6.91,10.86,7.01,10.42,7,9.98C6.99,9.53,6.9,9.11,6.72,8.71l0.91-0.42C7.87,8.82,7.99,9.38,8,9.97 C8.01,10.56,7.88,11.14,7.63,11.71z M9.82,12.79L8.95,12.3c0.37-0.65,0.55-1.39,0.55-2.22c0-0.84-0.19-1.64-0.57-2.39l0.89-0.45 c0.45,0.89,0.67,1.85,0.67,2.84C10.5,11.08,10.27,12,9.82,12.79z M12.07,13.72l-0.9-0.44c0.55-1.13,0.83-2.24,0.83-3.3 c0-1.06-0.28-2.16-0.83-3.26l0.9-0.44C12.69,7.52,13,8.77,13,9.99C13,11.2,12.69,12.46,12.07,13.72z",
                }
            }
        }
    }
}

pub fn contactless_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M8.46,14.45L7.1,13.83 c0.28-0.61,0.41-1.24,0.4-1.86c-0.01-0.63-0.14-1.24-0.4-1.8l1.36-0.63c0.35,0.75,0.53,1.56,0.54,2.4 C9.01,12.8,8.83,13.64,8.46,14.45z M11.53,16.01l-1.3-0.74c0.52-0.92,0.78-1.98,0.78-3.15c0-1.19-0.27-2.33-0.8-3.4l1.34-0.67 c0.64,1.28,0.96,2.65,0.96,4.07C12.51,13.55,12.18,14.86,11.53,16.01z M14.67,17.33l-1.35-0.66c0.78-1.6,1.18-3.18,1.18-4.69 c0-1.51-0.4-3.07-1.18-4.64l1.34-0.67C15.56,8.45,16,10.23,16,11.98C16,13.72,15.56,15.52,14.67,17.33z",
                }
            }
        }
    }
}

pub fn contact_page_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8L14,2z M12,10c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2s-2-0.9-2-2 C10,10.9,10.9,10,12,10z M16,18H8v-0.57c0-0.81,0.48-1.53,1.22-1.85C10.07,15.21,11.01,15,12,15c0.99,0,1.93,0.21,2.78,0.58 C15.52,15.9,16,16.62,16,17.43V18z",
            }
        }
    }
}

pub fn contact_support_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11.5 2C6.81 2 3 5.81 3 10.5S6.81 19 11.5 19h.5v3c4.86-2.34 8-7 8-11.5C20 5.81 16.19 2 11.5 2zm1 14.5h-2v-2h2v2zm0-3.5h-2c0-3.25 3-3 3-5 0-1.1-.9-2-2-2s-2 .9-2 2h-2c0-2.21 1.79-4 4-4s4 1.79 4 4c0 2.5-3 2.75-3 5z",
            }
        }
    }
}

pub fn copyright_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
            }
            g {
                g {
                    g {
                        path {
                            d: "M11.88,9.14c1.28,0.06,1.61,1.15,1.63,1.66h1.79c-0.08-1.98-1.49-3.19-3.45-3.19C9.64,7.61,8,9,8,12.14 c0,1.94,0.93,4.24,3.84,4.24c2.22,0,3.41-1.65,3.44-2.95h-1.79c-0.03,0.59-0.45,1.38-1.63,1.44C10.55,14.83,10,13.81,10,12.14 C10,9.25,11.28,9.16,11.88,9.14z M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.41,0-8-3.59-8-8 s3.59-8,8-8s8,3.59,8,8S16.41,20,12,20z",
                        }
                    }
                }
            }
        }
    }
}

pub fn credit_card_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zm0 14H4v-6h16v6zm0-10H4V6h16v2z",
            }
        }
    }
}

pub fn credit_card_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M6.12,4H16.5C17.33,4,18,4.67,18,5.5v9c0,0.38-0.15,0.73-0.38,1l-5.5-5.5h4.38V7H9.12L6.12,4z M16.72,18.84L13.88,16H3.5 C2.67,16,2,15.33,2,14.5v-9c0-0.38,0.15-0.73,0.38-1L1.16,3.28l1.06-1.06l15.56,15.56L16.72,18.84z M7.88,10l-3-3H3.5v3H7.88z",
            }
        }
    }
}

pub fn credit_card_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.9,21.9L2.1,2.1L0.69,3.51l1.55,1.55C2.09,5.34,2.01,5.66,2.01,6L2,18c0,1.11,0.89,2,2,2h13.17l3.31,3.31L21.9,21.9z M4,12V8h1.17l4,4H4z M6.83,4H20c1.11,0,2,0.89,2,2v12c0,0.34-0.08,0.66-0.23,0.94L14.83,12H20V8h-9.17L6.83,4z",
            }
        }
    }
}

pub fn css_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M8.5,11.5v-0.75h1V11h1v-0.75H9c-0.28,0-0.5-0.22-0.5-0.5V8.5C8.5,8.22,8.72,8,9,8h2c0.28,0,0.5,0.22,0.5,0.5v0.75h-1V9h-1 v0.75H11c0.28,0,0.5,0.22,0.5,0.5v1.25c0,0.28-0.22,0.5-0.5,0.5H9C8.72,12,8.5,11.78,8.5,11.5z M13.5,12h2c0.28,0,0.5-0.22,0.5-0.5 v-1.25c0-0.28-0.22-0.5-0.5-0.5H14V9h1v0.25h1V8.5C16,8.22,15.78,8,15.5,8h-2C13.22,8,13,8.22,13,8.5v1.25c0,0.28,0.22,0.5,0.5,0.5 H15V11h-1v-0.25h-1v0.75C13,11.78,13.22,12,13.5,12z M6.5,12C6.77,12,7,11.78,7,11.5v-0.75H6V11H5V9h1v0.25h1L7,8.5 C7,8.22,6.78,8,6.5,8h-2C4.22,8,4,8.22,4,8.5v3C4,11.78,4.22,12,4.5,12H6.5z",
                }
            }
        }
    }
}

pub fn css_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M9.5,14v-1H11v0.5h2v-1h-2.5c-0.55,0-1-0.45-1-1V10c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1v1H13v-0.5h-2v1h2.5 c0.55,0,1,0.45,1,1V14c0,0.55-0.45,1-1,1h-3C9.95,15,9.5,14.55,9.5,14z M17,15h3c0.55,0,1-0.45,1-1v-1.5c0-0.55-0.45-1-1-1h-2.5v-1 h2V11H21v-1c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v1.5c0,0.55,0.45,1,1,1h2.5v1h-2V13H16v1C16,14.55,16.45,15,17,15z M8,10 c0-0.55-0.45-1-1-1H4c-0.55,0-1,0.45-1,1v4c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-1H6.5v0.5h-2v-3h2V11H8V10z",
                }
            }
        }
    }
}

pub fn currency_exchange_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13.06,12c0,2.06-1.96,2.46-2.37,2.53v0.98H9.31v-1.02c-0.58-0.14-1.88-0.61-2.37-2.33l1.3-0.53 c0.05,0.17,0.46,1.64,1.89,1.64c0.73,0,1.56-0.37,1.56-1.26c0-0.76-0.55-1.15-1.79-1.59c-0.86-0.31-2.63-0.81-2.63-2.6 c0-0.08,0.01-1.89,2.06-2.33V4.5h1.38v0.98c1.45,0.25,1.98,1.4,2.09,1.75l-1.24,0.53c-0.09-0.28-0.47-1.05-1.49-1.05 c-0.55,0-1.42,0.29-1.42,1.09c0,0.74,0.67,1.03,2.07,1.5C12.58,9.94,13.06,10.9,13.06,12z M6,14l0-1.5l-5,0l0,5l1.5,0l0-2.52 C4.11,17.4,6.87,19,10,19c4.97,0,9-4.03,9-9h-1.5c0,4.14-3.36,7.5-7.5,7.5c-2.67,0-5.02-1.4-6.35-3.5L6,14z M17.5,5.02 C15.89,2.6,13.13,1,10,1c-4.97,0-9,4.03-9,9h1.5c0-4.14,3.36-7.5,7.5-7.5c2.67,0,5.02,1.4,6.35,3.5L14,6v1.5h5v-5h-1.5L17.5,5.02z",
                }
            }
        }
    }
}

pub fn currency_exchange_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12.89,11.1c-1.78-0.59-2.64-0.96-2.64-1.9c0-1.02,1.11-1.39,1.81-1.39c1.31,0,1.79,0.99,1.9,1.34l1.58-0.67 C15.39,8.03,14.72,6.56,13,6.24V5h-2v1.26C8.52,6.82,8.51,9.12,8.51,9.22c0,2.27,2.25,2.91,3.35,3.31 c1.58,0.56,2.28,1.07,2.28,2.03c0,1.13-1.05,1.61-1.98,1.61c-1.82,0-2.34-1.87-2.4-2.09L8.1,14.75c0.63,2.19,2.28,2.78,2.9,2.96V19 h2v-1.24c0.4-0.09,2.9-0.59,2.9-3.22C15.9,13.15,15.29,11.93,12.89,11.1z M3,21H1v-6h6v2l-2.48,0c1.61,2.41,4.36,4,7.48,4 c4.97,0,9-4.03,9-9h2c0,6.08-4.92,11-11,11c-3.72,0-7.01-1.85-9-4.67L3,21z M1,12C1,5.92,5.92,1,12,1c3.72,0,7.01,1.85,9,4.67L21,3 h2v6h-6V7l2.48,0C17.87,4.59,15.12,3,12,3c-4.97,0-9,4.03-9,9H1z",
                }
            }
        }
    }
}

pub fn dangerous_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h20v20H0z",
            }
            path {
                d: "M13.3 2L18 6.7v6.6L13.3 18H6.7L2 13.3V6.7L6.7 2h6.6zm.7 5l-1-1-3 3-3-3-1 1 3 3-3 3 1 1 3-3 3 3 1-1-3-3 3-3z",
            }
        }
    }
}

pub fn dangerous_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15.73 3H8.27L3 8.27v7.46L8.27 21h7.46L21 15.73V8.27L15.73 3zM17 15.74L15.74 17 12 13.26 8.26 17 7 15.74 10.74 12 7 8.26 8.26 7 12 10.74 15.74 7 17 8.26 13.26 12 17 15.74z",
            }
        }
    }
}

pub fn dashboard_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z",
            }
        }
    }
}

pub fn dashboard_customize_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 3h8v8H3zm10 0h8v8h-8zM3 13h8v8H3zm15 0h-2v3h-3v2h3v3h2v-3h3v-2h-3z",
            }
        }
    }
}

pub fn data_exploration_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10,2c-4.42,0-8,3.58-8,8c0,1.17,0.25,2.29,0.71,3.29L7,9l2.49,2.44l2.94-2.94H11V7h4v4h-1.5V9.55l-4,4l-2.49-2.44 l-3.53,3.53C4.93,16.67,7.31,18,10,18l0,0h6.5c0.83,0,1.5-0.67,1.5-1.5V10C18,5.58,14.42,2,10,2z M16,16.75 c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75c0.41,0,0.75,0.34,0.75,0.75C16.75,16.41,16.41,16.75,16,16.75z",
            }
        }
    }
}

pub fn data_exploration_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,1.33,0.26,2.61,0.74,3.77L8,10.5l3.3,2.78L14.58,10H13V8h5v5h-2v-1.58L11.41,16l-3.29-2.79 l-4.4,4.4C5.52,20.26,8.56,22,12,22h8c1.1,0,2-0.9,2-2v-8C22,6.48,17.52,2,12,2z M19.5,20.5c-0.55,0-1-0.45-1-1s0.45-1,1-1 s1,0.45,1,1S20.05,20.5,19.5,20.5z",
            }
        }
    }
}

pub fn data_thresholding_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M15.5,15.5h-1.08l1.08-1.08V15.5z M15.5,13L15.5,13L13,15.5h-1.07l2.5-2.5h-1.41l-2.5,2.5H9.46l2.5-2.5h-1.41l-2.5,2.5H6.98 l2.5-2.5H8.07l-2.5,2.5H4.5L7,13H5.59L4.5,14.09V4.5h11V13z",
                    }
                    g {
                        polygon {
                            points: "10.69,10.62 14.25,7.06 13.19,6 10.69,8.5 9.19,7 5.75,10.44 6.81,11.5 9.19,9.12",
                        }
                    }
                }
            }
        }
    }
}

pub fn data_thresholding_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M10.67,8.17l2,2l3.67-3.67 l1.41,1.41L12.67,13l-2-2l-3,3l-1.41-1.41L10.67,8.17z M5,16h1.72L5,17.72V16z M5.84,19l3-3h1.83l-3,3H5.84z M9.8,19l3-3h1.62l-3,3 H9.8z M13.53,19l3-3h1.62l-3,3H13.53z M19,19h-1.73L19,17.27V19z",
                }
            }
        }
    }
}

pub fn date_range_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M9 11H7v2h2v-2zm4 0h-2v2h2v-2zm4 0h-2v2h2v-2zm2-7h-1V2h-2v2H8V2H6v2H5c-1.11 0-1.99.9-1.99 2L3 20c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 16H5V9h14v11z",
            }
        }
    }
}

pub fn delete_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM19 4h-3.5l-1-1h-5l-1 1H5v2h14V4z",
            }
        }
    }
}

pub fn delete_forever_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zm2.46-7.12l1.41-1.41L12 12.59l2.12-2.12 1.41 1.41L13.41 14l2.12 2.12-1.41 1.41L12 15.41l-2.12 2.12-1.41-1.41L10.59 14l-2.13-2.12zM15.5 4l-1-1h-5l-1 1H5v2h14V4z",
            }
        }
    }
}

pub fn delete_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6v12zM8 9h8v10H8V9zm7.5-5l-1-1h-5l-1 1H5v2h14V4z",
            }
        }
    }
}

pub fn density_large_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        height: "1.5",
                        width: "14",
                        x: "3",
                        y: "3",
                    }
                    rect {
                        x: "3",
                        y: "15.5",
                        height: "1.5",
                        width: "14",
                    }
                }
            }
        }
    }
}

pub fn density_large_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        x: "3",
                        height: "2",
                        y: "3",
                        width: "18",
                    }
                    rect {
                        height: "2",
                        width: "18",
                        x: "3",
                        y: "19",
                    }
                }
            }
        }
    }
}

pub fn density_medium_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        height: "1.5",
                        width: "14",
                        x: "3",
                        y: "9.25",
                    }
                    rect {
                        width: "14",
                        y: "3",
                        height: "1.5",
                        x: "3",
                    }
                    rect {
                        y: "15.5",
                        width: "14",
                        height: "1.5",
                        x: "3",
                    }
                }
            }
        }
    }
}

pub fn density_medium_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        y: "3",
                        width: "18",
                        height: "2",
                        x: "3",
                    }
                    rect {
                        x: "3",
                        width: "18",
                        height: "2",
                        y: "19",
                    }
                    rect {
                        y: "11",
                        height: "2",
                        x: "3",
                        width: "18",
                    }
                }
            }
        }
    }
}

pub fn density_small_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        height: "1.5",
                        x: "3",
                        width: "14",
                        y: "11.5",
                    }
                    rect {
                        height: "1.5",
                        y: "7",
                        x: "3",
                        width: "14",
                    }
                    rect {
                        width: "14",
                        height: "1.5",
                        y: "2.5",
                        x: "3",
                    }
                    rect {
                        height: "1.5",
                        y: "16",
                        width: "14",
                        x: "3",
                    }
                }
            }
        }
    }
}

pub fn density_small_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        x: "3",
                        height: "2",
                        width: "18",
                        y: "2",
                    }
                    rect {
                        width: "18",
                        height: "2",
                        x: "3",
                        y: "20",
                    }
                    rect {
                        x: "3",
                        height: "2",
                        y: "14",
                        width: "18",
                    }
                    rect {
                        height: "2",
                        y: "8",
                        x: "3",
                        width: "18",
                    }
                }
            }
        }
    }
}

pub fn description_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z",
            }
        }
    }
}

pub fn disabled_by_default_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3,3v18h18V3H3z M17,15.59L15.59,17L12,13.41L8.41,17L7,15.59L10.59,12L7,8.41L8.41,7L12,10.59L15.59,7L17,8.41L13.41,12 L17,15.59z",
            }
        }
    }
}

pub fn disabled_visible_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17.99,10.38C18,10.26,18,10.13,18,10c0-4.42-3.58-8-8-8s-8,3.58-8,8c0,4.24,3.3,7.71,7.47,7.98 c-0.73-0.53-1.35-1.18-1.83-1.93C5.22,15.11,3.5,12.75,3.5,10c0-1.52,0.53-2.92,1.41-4.03l4.85,4.85c0.43-0.29,0.9-0.53,1.39-0.73 L5.97,4.91C7.08,4.03,8.48,3.5,10,3.5c3.51,0,6.39,2.81,6.5,6.3C17.02,9.95,17.52,10.14,17.99,10.38z M15.5,14.5 c0,0.69-0.56,1.25-1.25,1.25S13,15.19,13,14.5s0.56-1.25,1.25-1.25S15.5,13.81,15.5,14.5z M14.25,11c-2.61,0-4.85,1.45-5.75,3.5 c0.9,2.05,3.14,3.5,5.75,3.5s4.85-1.45,5.75-3.5C19.1,12.45,16.86,11,14.25,11z M14.25,16.5c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2 S15.35,16.5,14.25,16.5z",
            }
        }
    }
}

pub fn disabled_visible_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.99,12.34C22,12.23,22,12.11,22,12c0-5.52-4.48-10-10-10S2,6.48,2,12c0,5.17,3.93,9.43,8.96,9.95 c-0.93-0.73-1.72-1.64-2.32-2.68C5.9,18,4,15.22,4,12c0-1.85,0.63-3.55,1.69-4.9l5.66,5.66c0.56-0.4,1.17-0.73,1.82-1L7.1,5.69 C8.45,4.63,10.15,4,12,4c4.24,0,7.7,3.29,7.98,7.45C20.69,11.67,21.37,11.97,21.99,12.34z M17,13c-3.18,0-5.9,1.87-7,4.5 c1.1,2.63,3.82,4.5,7,4.5s5.9-1.87,7-4.5C22.9,14.87,20.18,13,17,13z M17,20c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5 s2.5,1.12,2.5,2.5C19.5,18.88,18.38,20,17,20z M18.5,17.5c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5 S18.5,16.67,18.5,17.5z",
            }
        }
    }
}

pub fn display_settings_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.5,3h-13C2.67,3,2,3.67,2,4.5v9C2,14.33,2.67,15,3.5,15H7v2h6v-2h3.5c0.83,0,1.5-0.67,1.5-1.5v-9 C18,3.67,17.33,3,16.5,3z M16.5,13.5h-13v-9h13V13.5z",
                    }
                    rect {
                        x: "5",
                        y: "7",
                        width: "7",
                        height: "1",
                    }
                    polygon {
                        points: "14,8 15,8 15,7 14,7 14,6 13,6 13,9 14,9",
                    }
                    rect {
                        y: "10",
                        x: "8",
                        height: "1",
                        width: "7",
                    }
                    polygon {
                        points: "6,12 7,12 7,9 6,9 6,10 5,10 5,11 6,11",
                    }
                }
            }
        }
    }
}

pub fn display_settings_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,3H4C2.89,3,2,3.89,2,5v12c0,1.1,0.89,2,2,2h4v2h8v-2h4c1.1,0,2-0.9,2-2V5C22,3.89,21.1,3,20,3z M20,17H4V5h16V17z",
                    }
                    rect {
                        width: "8",
                        x: "6",
                        height: "1.5",
                        y: "8.25",
                    }
                    polygon {
                        points: "16.5,9.75 18,9.75 18,8.25 16.5,8.25 16.5,7 15,7 15,11 16.5,11",
                    }
                    rect {
                        y: "12.25",
                        x: "10",
                        height: "1.5",
                        width: "8",
                    }
                    polygon {
                        points: "7.5,15 9,15 9,11 7.5,11 7.5,12.25 6,12.25 6,13.75 7.5,13.75",
                    }
                }
            }
        }
    }
}

pub fn dns_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 13H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1v-6c0-.55-.45-1-1-1zM7 19c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zM20 3H4c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h16c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1zM7 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

pub fn done_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z",
            }
        }
    }
}

pub fn done_all_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18 7l-1.41-1.41-6.34 6.34 1.41 1.41L18 7zm4.24-1.41L11.66 16.17 7.48 12l-1.41 1.41L11.66 19l12-12-1.42-1.41zM.41 13.41L6 19l1.41-1.41L1.83 12 .41 13.41z",
            }
        }
    }
}

pub fn done_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19.77 5.03l1.4 1.4L8.43 19.17l-5.6-5.6 1.4-1.4 4.2 4.2L19.77 5.03m0-2.83L8.43 13.54l-4.2-4.2L0 13.57 8.43 22 24 6.43 19.77 2.2z",
            }
        }
    }
}

pub fn donut_large_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M11,5.08V2C6,2.5,2,6.81,2,12s4,9.5,9,10v-3.08c-3-0.48-6-3.4-6-6.92S8,5.56,11,5.08z M18.97,11H22c-0.47-5-4-8.53-9-9 v3.08C16,5.51,18.54,8,18.97,11z M13,18.92V22c5-0.47,8.53-4,9-9h-3.03C18.54,16,16,18.49,13,18.92z",
                        }
                    }
                }
            }
        }
    }
}

pub fn donut_small_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11 9.16V2c-5 .5-9 4.79-9 10s4 9.5 9 10v-7.16c-1-.41-2-1.52-2-2.84s1-2.43 2-2.84zM14.86 11H22c-.48-4.75-4-8.53-9-9v7.16c1 .3 1.52.98 1.86 1.84zM13 14.84V22c5-.47 8.52-4.25 9-9h-7.14c-.34.86-.86 1.54-1.86 1.84z",
            }
        }
    }
}

pub fn drag_indicator_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11 18c0 1.1-.9 2-2 2s-2-.9-2-2 .9-2 2-2 2 .9 2 2zm-2-8c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0-6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm6 4c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm0 2c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm0 6c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

pub fn dynamic_form_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,20v-9h-2V4h7l-2,5h2L17,20z M15,13v7H4c-1.1,0-2-0.9-2-2v-3c0-1.1,0.9-2,2-2H15z M6.25,15.75h-1.5v1.5h1.5V15.75z M13,4v7H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2H13z M6.25,6.75h-1.5v1.5h1.5V6.75z",
                }
            }
        }
    }
}

pub fn eco_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M5.53,7.04c-2.05,2.05-2.05,5.36-0.01,7.41c1.11-2.55,3.07-4.68,5.53-5.95C8.97,10.26,7.51,12.71,7,15.49 c1.95,0.92,4.35,0.59,5.96-1.03C15.57,11.86,16,4,16,4S8.14,4.43,5.53,7.04z",
                    }
                }
            }
        }
    }
}

pub fn eco_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M6.05,8.05c-2.73,2.73-2.73,7.15-0.02,9.88c1.47-3.4,4.09-6.24,7.36-7.93c-2.77,2.34-4.71,5.61-5.39,9.32 c2.6,1.23,5.8,0.78,7.95-1.37C19.43,14.47,20,4,20,4S9.53,4.57,6.05,8.05z",
                    }
                }
            }
        }
    }
}

pub fn edit_calendar_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M9.5,18h-5C3.67,18,3,17.32,3,16.5v-11C3,4.68,3.67,4,4.5,4H6V2h1.5v2h5V2H14v2h1.5C16.33,4,17,4.68,17,5.5V10h-1.5V9h-11 v7.5h5V18z M17.78,13.99l0.65-0.65c0.29-0.29,0.29-0.77,0-1.06l-0.71-0.71c-0.29-0.29-0.77-0.29-1.06,0l-0.65,0.65L17.78,13.99z M17.19,14.58L12.77,19H11v-1.77l4.42-4.42L17.19,14.58z",
            }
        }
    }
}

pub fn edit_calendar_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12,22H5c-1.11,0-2-0.9-2-2L3.01,6c0-1.1,0.88-2,1.99-2h1V2h2v2h8V2h2v2h1c1.1,0,2,0.9,2,2v6h-2v-2H5v10h7V22z M22.13,16.99 l0.71-0.71c0.39-0.39,0.39-1.02,0-1.41l-0.71-0.71c-0.39-0.39-1.02-0.39-1.41,0l-0.71,0.71L22.13,16.99z M21.42,17.7l-5.3,5.3H14 v-2.12l5.3-5.3L21.42,17.7z",
            }
        }
    }
}

pub fn edit_document_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        polygon {
                            points: "11.5,16.23 11.5,18 13.27,18 17.44,13.83 15.67,12.06",
                        }
                    }
                    g {
                        path {
                            d: "M16,6l-4-4H5.5C4.67,2,4,2.67,4,3.5v13C4,17.33,4.67,18,5.5,18H10v-2.39l6-6V6z M11,7V3l4,4H11z",
                        }
                    }
                    g {
                        path {
                            d: "M18.85,11.71l-1.06-1.06c-0.2-0.2-0.51-0.2-0.71,0l-0.71,0.71l1.77,1.77l0.71-0.71C19.05,12.22,19.05,11.9,18.85,11.71z",
                        }
                    }
                }
            }
        }
    }
}

pub fn edit_document_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        polygon {
                            points: "14,19.88 14,22 16.12,22 21.29,16.83 19.17,14.71",
                        }
                    }
                    g {
                        path {
                            d: "M20,8l-6-6H6C4.9,2,4.01,2.9,4.01,4L4,20c0,1.1,0.89,2,1.99,2H12v-2.95l8-8V8z M13,9V3.5L18.5,9H13z",
                        }
                    }
                    g {
                        path {
                            d: "M22.71,14L22,13.29c-0.39-0.39-1.02-0.39-1.41,0L19.88,14L22,16.12l0.71-0.71C23.1,15.02,23.1,14.39,22.71,14z",
                        }
                    }
                }
            }
        }
    }
}

pub fn edit_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            title {
                "ic_edit_off_24px"
            }
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M12.126 8.125l1.937-1.937 3.747 3.747-1.937 1.938zM20.71 5.63l-2.34-2.34a1 1 0 0 0-1.41 0l-1.83 1.83 3.75 3.75L20.71 7a1 1 0 0 0 0-1.37zM2 5l6.63 6.63L3 17.25V21h3.75l5.63-5.62L18 21l2-2L4 3 2 5z",
            }
        }
    }
}

pub fn edit_square_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18.56,2.5L17.5,1.44c-0.59-0.59-1.54-0.59-2.12,0l-0.71,0.71l3.18,3.18l0.71-0.71C19.15,4.03,19.15,3.08,18.56,2.5z",
                    }
                    polygon {
                        points: "8,12 11.18,12 16.79,6.39 13.61,3.21 8,8.82",
                    }
                    path {
                        d: "M6.5,13.5V8.2L11.7,3H4.5C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5V8.3l-5.2,5.2H6.5z",
                    }
                }
            }
        }
    }
}

pub fn edit_square_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M7,17V9.93L13.93,3l0,0H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v-8.93L14.07,17H7z",
                    }
                    polygon {
                        points: "9,15 13.24,15 20.44,7.8 16.2,3.56 9,10.76",
                    }
                    path {
                        d: "M22.91,2.49l-1.41-1.41c-0.78-0.78-2.05-0.78-2.83,0l-1.06,1.06l4.24,4.24l1.06-1.06C23.7,4.54,23.7,3.27,22.91,2.49z",
                    }
                }
            }
        }
    }
}

pub fn eject_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M0 24V0h24v24H0z",
            }
            path {
                d: "M5 17h14v2H5zm7-12L5.33 15h13.34z",
            }
        }
    }
}

pub fn euro_symbol_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15 18.5c-2.51 0-4.68-1.42-5.76-3.5H15v-2H8.58c-.05-.33-.08-.66-.08-1s.03-.67.08-1H15V9H9.24C10.32 6.92 12.5 5.5 15 5.5c1.61 0 3.09.59 4.23 1.57L21 5.3C19.41 3.87 17.3 3 15 3c-3.92 0-7.24 2.51-8.48 6H3v2h3.06c-.04.33-.06.66-.06 1 0 .34.02.67.06 1H3v2h3.52c1.24 3.49 4.56 6 8.48 6 2.31 0 4.41-.87 6-2.3l-1.78-1.77c-1.13.98-2.6 1.57-4.22 1.57z",
            }
        }
    }
}

pub fn event_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17 12h-5v5h5v-5zM16 1v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2h-1V1h-2zm3 18H5V8h14v11z",
            }
        }
    }
}

pub fn event_repeat_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,10.5v-5C17,4.68,16.33,4,15.5,4H14V2h-1.5v2h-5V2H6v2H4.5C3.67,4,3,4.68,3,5.5v11C3,17.32,3.67,18,4.5,18h6v-1.5h-6V9 h11v1.5H17z M12,12h1l0,1.35c0.73-0.83,1.81-1.35,3-1.35c2.21,0,4,1.79,4,4c0,2.21-1.79,4-4,4c-2.04,0-3.72-1.53-3.97-3.5l1.01,0 C13.28,17.92,14.51,19,16,19c1.66,0,3-1.34,3-3s-1.34-3-3-3c-0.89,0-1.69,0.39-2.24,1L15,14v1h-3V12z",
                }
            }
        }
    }
}

pub fn event_repeat_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M21,12V6c0-1.1-0.9-2-2-2h-1V2h-2v2H8V2H6v2H5C3.9,4,3,4.9,3,6v14c0,1.1,0.9,2,2,2h7v-2H5V10h14v2H21z M15.64,20 c0.43,1.45,1.77,2.5,3.36,2.5c1.93,0,3.5-1.57,3.5-3.5s-1.57-3.5-3.5-3.5c-0.95,0-1.82,0.38-2.45,1l1.45,0V18h-4v-4h1.5l0,1.43 C16.4,14.55,17.64,14,19,14c2.76,0,5,2.24,5,5s-2.24,5-5,5c-2.42,0-4.44-1.72-4.9-4L15.64,20z",
                }
            }
        }
    }
}

pub fn event_seat_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M4,18v3h3v-3h10v3h3v-6H4V18z M19,10h3v3h-3V10z M2,10h3v3H2V10z M17,13H7V5c0-1.1,0.9-2,2-2h6c1.1,0,2,0.9,2,2V13z",
                        }
                    }
                }
            }
        }
    }
}

pub fn exit_to_app_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M10.09 15.59L11.5 17l5-5-5-5-1.41 1.41L12.67 11H3v2h9.67l-2.58 2.59zM19 3H5c-1.11 0-2 .9-2 2v4h2V5h14v14H5v-4H3v4c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn expand_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                fill: "none",
                d: "M0 0h24v24H0V0z",
            }
            path {
                d: "M4 20h16v2H4zM4 2h16v2H4zm9 7h3l-4-4-4 4h3v6H8l4 4 4-4h-3z",
            }
        }
    }
}

pub fn explore_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 10.9c-.61 0-1.1.49-1.1 1.1s.49 1.1 1.1 1.1c.61 0 1.1-.49 1.1-1.1s-.49-1.1-1.1-1.1zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm2.19 12.19L6 18l3.81-8.19L18 6l-3.81 8.19z",
            }
        }
    }
}

pub fn explore_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M14.19 14.19l-1.41-1.41-1.56-1.56L11 11 9.81 9.81 4.93 4.93 2.27 2.27 1 3.54l2.78 2.78c-.11.16-.21.32-.31.48-.04.07-.09.14-.13.21-.09.15-.17.31-.25.47-.05.1-.1.21-.16.32-.06.14-.13.28-.19.43-.1.24-.19.48-.27.73l-.09.3c-.05.2-.1.39-.14.59-.02.11-.04.22-.07.33-.04.2-.07.4-.09.61-.01.1-.03.2-.03.3-.03.29-.05.6-.05.91 0 5.52 4.48 10 10 10 .31 0 .62-.02.92-.05l.3-.03c.2-.02.41-.06.61-.09.11-.02.22-.04.33-.07.2-.04.39-.09.58-.15.1-.03.2-.05.3-.09.25-.08.49-.17.73-.27.15-.06.29-.13.43-.19.11-.05.22-.1.33-.16.16-.08.31-.16.46-.25.07-.04.14-.09.21-.13.16-.1.32-.2.48-.31L20.46 23l1.27-1.27-2.66-2.66-4.88-4.88zM6 18l3-6.46L12.46 15 6 18zm16-6c0 .31-.02.62-.05.92l-.03.3c-.02.2-.06.41-.09.61-.02.11-.04.22-.07.33-.04.2-.09.39-.15.58-.03.1-.05.21-.09.31-.08.25-.17.49-.27.73-.06.15-.13.29-.19.43-.05.11-.1.22-.16.33-.08.16-.16.31-.25.46-.04.07-.09.14-.13.21-.1.16-.2.32-.31.48L15 12.46 18 6l-6.46 3-5.22-5.22c.16-.11.32-.21.48-.31.07-.04.14-.09.21-.13.15-.09.31-.17.46-.25.11-.05.22-.1.33-.16.14-.06.28-.13.43-.19.24-.1.48-.19.73-.27l.31-.09c.19-.05.38-.11.58-.15.11-.02.22-.04.33-.07.2-.04.4-.07.61-.09.1-.01.2-.03.3-.03.29-.02.6-.04.91-.04 5.52 0 10 4.48 10 10z",
            }
        }
    }
}

pub fn extension_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20.5 11H19V7c0-1.1-.9-2-2-2h-4V3.5C13 2.12 11.88 1 10.5 1S8 2.12 8 3.5V5H4c-1.1 0-1.99.9-1.99 2v3.8H3.5c1.49 0 2.7 1.21 2.7 2.7s-1.21 2.7-2.7 2.7H2V20c0 1.1.9 2 2 2h3.8v-1.5c0-1.49 1.21-2.7 2.7-2.7 1.49 0 2.7 1.21 2.7 2.7V22H17c1.1 0 2-.9 2-2v-4h1.5c1.38 0 2.5-1.12 2.5-2.5S21.88 11 20.5 11z",
            }
        }
    }
}

pub fn extension_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M16.01,18.13L14.88,17H12c0-1.1-0.9-2-2-2s-2,0.9-2,2H4.5C3.67,17,3,16.33,3,15.5V12c1.1,0,2-0.9,2-2S4.1,8,3,8V5.12 L1.87,3.99l1.06-1.06l0.51,0.51l13.63,13.63L16.01,18.13z M17,8V4.5C17,3.67,16.33,3,15.5,3H12c0-1.1-0.9-2-2-2S8,1.9,8,3H5.12 L17,14.88V12c1.1,0,2-0.9,2-2S18.1,8,17,8z",
            }
        }
    }
}

pub fn extension_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19.78,22.61l-1.63-1.63C18.1,20.98,18.05,21,18,21h-3.8c0-2.71-2.16-3-2.7-3s-2.7,0.29-2.7,3H5c-1.1,0-2-0.9-2-2v-3.8 c2.71,0,3-2.16,3-2.7c0-0.54-0.3-2.7-2.99-2.7V6c0-0.05,0.02-0.09,0.02-0.14L1.39,4.22l1.41-1.41l18.38,18.38L19.78,22.61z M20,17.17V15c1.38,0,2.5-1.12,2.5-2.5S21.38,10,20,10V6c0-1.1-0.9-2-2-2h-4c0-1.38-1.12-2.5-2.5-2.5S9,2.62,9,4H6.83L20,17.17z",
            }
        }
    }
}

pub fn face_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M9 11.75c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zm6 0c-.69 0-1.25.56-1.25 1.25s.56 1.25 1.25 1.25 1.25-.56 1.25-1.25-.56-1.25-1.25-1.25zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8 0-.29.02-.58.05-.86 2.36-1.05 4.23-2.98 5.21-5.37C11.07 8.33 14.05 10 17.42 10c.78 0 1.53-.09 2.25-.26.21.71.33 1.47.33 2.26 0 4.41-3.59 8-8 8z",
            }
        }
    }
}

pub fn fact_check_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,4H4C3.45,4,3,4.45,3,5v10c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1V5 C17,4.45,16.55,4,16,4z M9,13H5v-1h4V13z M9,10.5H5v-1h4V10.5z M9,8H5V7h4V8z M11.97,11.77l-1.41-1.41l0.71-0.71l0.71,0.71 l2.12-2.12l0.71,0.71L11.97,11.77z",
                    fill_rule: "evenodd",
                }
            }
        }
    }
}

pub fn fact_check_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,3H4C2.9,3,2,3.9,2,5v14c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V5 C22,3.9,21.1,3,20,3z M10,17H5v-2h5V17z M10,13H5v-2h5V13z M10,9H5V7h5V9z M14.82,15L12,12.16l1.41-1.41l1.41,1.42L17.99,9 l1.42,1.42L14.82,15z",
                        fill_rule: "evenodd",
                    }
                }
            }
        }
    }
}

pub fn favorite_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z",
            }
        }
    }
}

pub fn favorite_border_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16.5 3c-1.74 0-3.41.81-4.5 2.09C10.91 3.81 9.24 3 7.5 3 4.42 3 2 5.42 2 8.5c0 3.78 3.4 6.86 8.55 11.54L12 21.35l1.45-1.32C18.6 15.36 22 12.28 22 8.5 22 5.42 19.58 3 16.5 3zm-4.4 15.55l-.1.1-.1-.1C7.14 14.24 4 11.39 4 8.5 4 6.5 5.5 5 7.5 5c1.54 0 3.04.99 3.57 2.36h1.87C13.46 5.99 14.96 5 16.5 5c2 0 3.5 1.5 3.5 3.5 0 2.89-3.14 5.74-7.9 10.05z",
            }
        }
    }
}

pub fn fax_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M4,6.5c-1.1,0-2,0.9-2,2V15c0,1.1,0.9,2,2,2s2-0.9,2-2V8.5C6,7.4,5.1,6.5,4,6.5z",
                    }
                    path {
                        d: "M16,7.5h-1.4V4H7v12h11V9.5C18,8.4,17.1,7.5,16,7.5z M8.5,5.5h4.6v2H8.5V5.5z M11,13.5H8V10h3V13.5z M12.5,13.5 c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75C13.25,13.16,12.91,13.5,12.5,13.5z M12.5,11.5 c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75C13.25,11.16,12.91,11.5,12.5,11.5z M14.75,13.5 c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75C15.5,13.16,15.16,13.5,14.75,13.5z M14.75,11.5 c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75C15.5,11.16,15.16,11.5,14.75,11.5z",
                    }
                }
            }
        }
    }
}

pub fn fax_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M19,9h-1V4H8v14.5V20h14v-8C22,10.34,20.66,9,19,9z M10,6h6v3h-6V6z M14,17h-4v-5h4V17z M16,17c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C17,16.55,16.55,17,16,17z M16,14c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C17,13.55,16.55,14,16,14z M19,17c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C20,16.55,19.55,17,19,17z M19,14 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C20,13.55,19.55,14,19,14z",
                    }
                    path {
                        d: "M4.5,8C3.12,8,2,9.12,2,10.5v8C2,19.88,3.12,21,4.5,21S7,19.88,7,18.5v-8C7,9.12,5.88,8,4.5,8z",
                    }
                }
            }
        }
    }
}

pub fn feedback_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 12h-2v-2h2v2zm0-4h-2V6h2v4z",
            }
        }
    }
}

pub fn file_present_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V7l-5-5zM6 20V4h8v4h4v12H6zm10-10v5c0 2.21-1.79 4-4 4s-4-1.79-4-4V8.5c0-1.47 1.26-2.64 2.76-2.49 1.3.13 2.24 1.32 2.24 2.63V15h-2V8.5c0-.28-.22-.5-.5-.5s-.5.22-.5.5V15c0 1.1.9 2 2 2s2-.9 2-2v-5h2z",
            }
        }
    }
}

pub fn filter_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M0,0h24 M24,24H0",
                }
                path {
                    d: "M4.25,5.61C6.27,8.2,10,13,10,13v6c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-6c0,0,3.72-4.8,5.74-7.39 C20.25,4.95,19.78,4,18.95,4H5.04C4.21,4,3.74,4.95,4.25,5.61z",
                }
                path {
                    fill: "none",
                    d: "M0,0h24v24H0V0z",
                }
            }
        }
    }
}

pub fn filter_alt_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.89,4.82C16.15,4.49,15.92,4,15.5,4H6.12l5.73,5.73L15.89,4.82z",
                    }
                    path {
                        d: "M2.93,2.93L1.87,3.99L9,11.12v4.38C9,15.78,9.22,16,9.5,16h1c0.28,0,0.5-0.22,0.5-0.5v-2.38l5.01,5.01l1.06-1.06 L2.93,2.93z",
                    }
                }
            }
        }
    }
}

pub fn filter_alt_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M19.79,5.61C20.3,4.95,19.83,4,19,4H6.83l7.97,7.97L19.79,5.61z",
                    }
                    path {
                        d: "M2.81,2.81L1.39,4.22L10,13v6c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-2.17l5.78,5.78l1.41-1.41L2.81,2.81z",
                    }
                }
            }
        }
    }
}

pub fn filter_list_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M0 0h24m0 24H0",
                fill: "none",
            }
            path {
                d: "M4.25 5.66c.1.13 5.74 7.33 5.74 7.33V19c0 .55.45 1 1.01 1h2.01c.55 0 1.01-.45 1.01-1v-6.02s5.49-7.02 5.75-7.34C20.03 5.32 20 5 20 5c0-.55-.45-1-1.01-1H5.01C4.4 4 4 4.48 4 5c0 .2.06.44.25.66z",
            }
        }
    }
}

pub fn find_in_page_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 19.59V8l-6-6H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c.45 0 .85-.15 1.19-.4l-4.43-4.43c-.8.52-1.74.83-2.76.83-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5c0 1.02-.31 1.96-.83 2.75L20 19.59zM9 13c0 1.66 1.34 3 3 3s3-1.34 3-3-1.34-3-3-3-3 1.34-3 3z",
            }
        }
    }
}

pub fn find_replace_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11 6c1.38 0 2.63.56 3.54 1.46L12 10h6V4l-2.05 2.05C14.68 4.78 12.93 4 11 4c-3.53 0-6.43 2.61-6.92 6H6.1c.46-2.28 2.48-4 4.9-4zm5.64 9.14c.66-.9 1.12-1.97 1.28-3.14H15.9c-.46 2.28-2.48 4-4.9 4-1.38 0-2.63-.56-3.54-1.46L10 12H4v6l2.05-2.05C7.32 17.22 9.07 18 11 18c1.55 0 2.98-.51 4.14-1.36L20 21.49 21.49 20l-4.85-4.86z",
            }
        }
    }
}

pub fn fingerprint_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17.81 4.47c-.08 0-.16-.02-.23-.06C15.66 3.42 14 3 12.01 3c-1.98 0-3.86.47-5.57 1.41-.24.13-.54.04-.68-.2-.13-.24-.04-.55.2-.68C7.82 2.52 9.86 2 12.01 2c2.13 0 3.99.47 6.03 1.52.25.13.34.43.21.67-.09.18-.26.28-.44.28zM3.5 9.72c-.1 0-.2-.03-.29-.09-.23-.16-.28-.47-.12-.7.99-1.4 2.25-2.5 3.75-3.27C9.98 4.04 14 4.03 17.15 5.65c1.5.77 2.76 1.86 3.75 3.25.16.22.11.54-.12.7-.23.16-.54.11-.7-.12-.9-1.26-2.04-2.25-3.39-2.94-2.87-1.47-6.54-1.47-9.4.01-1.36.7-2.5 1.7-3.4 2.96-.08.14-.23.21-.39.21zm6.25 12.07c-.13 0-.26-.05-.35-.15-.87-.87-1.34-1.43-2.01-2.64-.69-1.23-1.05-2.73-1.05-4.34 0-2.97 2.54-5.39 5.66-5.39s5.66 2.42 5.66 5.39c0 .28-.22.5-.5.5s-.5-.22-.5-.5c0-2.42-2.09-4.39-4.66-4.39-2.57 0-4.66 1.97-4.66 4.39 0 1.44.32 2.77.93 3.85.64 1.15 1.08 1.64 1.85 2.42.19.2.19.51 0 .71-.11.1-.24.15-.37.15zm7.17-1.85c-1.19 0-2.24-.3-3.1-.89-1.49-1.01-2.38-2.65-2.38-4.39 0-.28.22-.5.5-.5s.5.22.5.5c0 1.41.72 2.74 1.94 3.56.71.48 1.54.71 2.54.71.24 0 .64-.03 1.04-.1.27-.05.53.13.58.41.05.27-.13.53-.41.58-.57.11-1.07.12-1.21.12zM14.91 22c-.04 0-.09-.01-.13-.02-1.59-.44-2.63-1.03-3.72-2.1-1.4-1.39-2.17-3.24-2.17-5.22 0-1.62 1.38-2.94 3.08-2.94 1.7 0 3.08 1.32 3.08 2.94 0 1.07.93 1.94 2.08 1.94s2.08-.87 2.08-1.94c0-3.77-3.25-6.83-7.25-6.83-2.84 0-5.44 1.58-6.61 4.03-.39.81-.59 1.76-.59 2.8 0 .78.07 2.01.67 3.61.1.26-.03.55-.29.64-.26.1-.55-.04-.64-.29-.49-1.31-.73-2.61-.73-3.96 0-1.2.23-2.29.68-3.24 1.33-2.79 4.28-4.6 7.51-4.6 4.55 0 8.25 3.51 8.25 7.83 0 1.62-1.38 2.94-3.08 2.94s-3.08-1.32-3.08-2.94c0-1.07-.93-1.94-2.08-1.94s-2.08.87-2.08 1.94c0 1.71.66 3.31 1.87 4.51.95.94 1.86 1.46 3.27 1.85.27.07.42.35.35.61-.05.23-.26.38-.47.38z",
            }
        }
    }
}

pub fn fit_screen_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17 4h3c1.1 0 2 .9 2 2v2h-2V6h-3V4zM4 8V6h3V4H4c-1.1 0-2 .9-2 2v2h2zm16 8v2h-3v2h3c1.1 0 2-.9 2-2v-2h-2zM7 18H4v-2H2v2c0 1.1.9 2 2 2h3v-2zM18 8H6v8h12V8z",
            }
        }
    }
}

pub fn flaky_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C17,6.13,13.87,3,10,3z M6,6.71L6.71,6l1.06,1.06L8.83,6l0.71,0.71L8.47,7.77l1.06,1.06L8.83,9.54L7.77,8.47L6.71,9.54L6,8.83l1.06-1.06L6,6.71z M10,16 c-1.65,0-3.15-0.67-4.24-1.76l8.48-8.48C15.33,6.85,16,8.35,16,10C16,13.31,13.31,16,10,16z",
                            fill_rule: "evenodd",
                        }
                    }
                    g {
                        polygon {
                            fill_rule: "evenodd",
                            points: "11.41,12.59 10.41,11.59 9.71,12.29 11.41,14 14.21,11.21 13.5,10.5",
                        }
                    }
                }
            }
        }
    }
}

pub fn flaky_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14.05,17.58l-0.01,0.01l-2.4-2.4l1.06-1.06l1.35,1.35L16.54,13l1.06,1.06 l-3.54,3.54L14.05,17.58z M12,2C6.5,2,2,6.5,2,12s4.5,10,10,10s10-4.5,10-10S17.5,2,12,2z M7.34,6.28l1.41,1.41l1.41-1.41 l1.06,1.06L9.81,8.75l1.41,1.41l-1.06,1.06L8.75,9.81l-1.41,1.41l-1.06-1.06l1.41-1.41L6.28,7.34L7.34,6.28z M12,20 c-2.2,0-4.2-0.9-5.7-2.3L17.7,6.3C19.1,7.8,20,9.8,20,12C20,16.4,16.4,20,12,20z",
                    fill_rule: "evenodd",
                }
            }
        }
    }
}

pub fn flight_land_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M2.5,19h19v2h-19V19z M19.34,15.85c0.8,0.21,1.62-0.26,1.84-1.06c0.21-0.8-0.26-1.62-1.06-1.84l-5.31-1.42l-2.76-9.02 L10.12,2v8.28L5.15,8.95L4.22,6.63L2.77,6.24v5.17L19.34,15.85z",
                        }
                    }
                }
            }
        }
    }
}

pub fn flight_takeoff_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M2.5,19h19v2h-19V19z M22.07,9.64c-0.21-0.8-1.04-1.28-1.84-1.06L14.92,10l-6.9-6.43L6.09,4.08l4.14,7.17l-4.97,1.33 l-1.97-1.54l-1.45,0.39l2.59,4.49c0,0,7.12-1.9,16.57-4.43C21.81,11.26,22.28,10.44,22.07,9.64z",
                        }
                    }
                }
            }
        }
    }
}

pub fn flip_to_back_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M9 7H7v2h2V7zm0 4H7v2h2v-2zm0-8c-1.11 0-2 .9-2 2h2V3zm4 12h-2v2h2v-2zm6-12v2h2c0-1.1-.9-2-2-2zm-6 0h-2v2h2V3zM9 17v-2H7c0 1.1.89 2 2 2zm10-4h2v-2h-2v2zm0-4h2V7h-2v2zm0 8c1.1 0 2-.9 2-2h-2v2zM5 7H3v12c0 1.1.89 2 2 2h12v-2H5V7zm10-2h2V3h-2v2zm0 12h2v-2h-2v2z",
            }
        }
    }
}

pub fn flip_to_front_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 13h2v-2H3v2zm0 4h2v-2H3v2zm2 4v-2H3c0 1.1.89 2 2 2zM3 9h2V7H3v2zm12 12h2v-2h-2v2zm4-18H9c-1.11 0-2 .9-2 2v10c0 1.1.89 2 2 2h10c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 12H9V5h10v10zm-8 6h2v-2h-2v2zm-4 0h2v-2H7v2z",
            }
        }
    }
}

pub fn flutter_dash_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                    d: "M9.26,9.56c0.23-0.32,0.65-0.45,1.01-0.3c0.14,0.06,0.26,0.15,0.35,0.26c0.18,0.22,0.2,0.47,0.17,0.68 c-0.04,0.27-0.2,0.5-0.43,0.63c0,0-3.89,2.36-4.06,2.15C6.14,12.78,9.26,9.56,9.26,9.56z M18,8.2c0,2-0.8,2.4-1.2,2.4 c-0.19,0-0.35-0.08-0.5-0.21c-0.38,2.65-1.89,4.25-4.26,4.79c0.08,0.35,0.39,0.62,0.76,0.62l0,0h0.46c0.18,0,0.33,0.12,0.38,0.29 c0.14,0.42,0.53,0.82,0.82,1.06c0.2,0.17,0.19,0.47-0.02,0.62c-0.27,0.19-0.72,0.39-1.43,0.43c-0.14,0.01-0.28-0.06-0.36-0.18 c-0.11-0.17-0.26-0.45-0.26-0.82c0-0.24,0.03-0.45,0.08-0.64c-0.62-0.13-1.11-0.62-1.24-1.25c-0.39,0.05-0.8,0.08-1.23,0.08 c-0.71,0-1.36-0.07-1.96-0.2C8.02,15.26,8,15.33,8,15.4c0,0.44,0.36,0.8,0.8,0.8l0,0h0.46c0.18,0,0.33,0.12,0.38,0.29 c0.14,0.42,0.53,0.82,0.82,1.06c0.2,0.17,0.19,0.47-0.02,0.62c-0.27,0.19-0.72,0.39-1.43,0.43c-0.14,0.01-0.28-0.06-0.36-0.18 C8.54,18.25,8.4,17.97,8.4,17.6c0-0.24,0.03-0.45,0.08-0.64c-0.73-0.15-1.28-0.8-1.28-1.57c0-0.15,0.03-0.29,0.06-0.42 c-1.97-0.69-3.22-2.22-3.57-4.59c-0.15,0.13-0.31,0.21-0.5,0.21C2.8,10.6,2,10.2,2,8.2c0-1.81,1.36-3.6,2.4-3.6 c0.34,0,0.39,0.39,0.4,0.68c1.02-1.43,2.61-2.41,4.44-2.63C9.4,1.88,10.1,1.4,10.8,1.4v0.8c0,0,0.26-0.4,0.8-0.4 c0.54,0,0.8,0.4,0.8,0.4c-0.39,0-0.68,0.28-0.76,0.62c1.46,0.39,2.71,1.27,3.57,2.46c0.01-0.29,0.06-0.68,0.4-0.68 C16.64,4.6,18,6.39,18,8.2z M4.4,9c0,0.65,0.08,1.23,0.2,1.77c0.15-0.55,0.37-1.07,0.67-1.54C5.1,8.85,5,8.44,5,8 c0-1.66,1.34-3,3-3c0.77,0,1.47,0.3,2,0.78C10.53,5.3,11.23,5,12,5c1.66,0,3,1.34,3,3c0,0.44-0.1,0.85-0.27,1.23 c0.3,0.47,0.53,0.99,0.67,1.55c0.12-0.54,0.2-1.12,0.2-1.78c0-3.09-2.51-5.6-5.6-5.6C6.91,3.4,4.4,5.91,4.4,9z M14.78,12.44 c0-0.08,0.02-0.15,0.02-0.24c0-0.8-0.21-1.56-0.56-2.22C13.69,10.6,12.9,11,12,11c-0.22,0-0.43-0.02-0.63-0.07 c0.11-0.19,0.18-0.4,0.21-0.62c0.01-0.05,0.01-0.1,0.01-0.16c0.13,0.03,0.27,0.04,0.4,0.04c1.21,0,2.2-0.99,2.2-2.2 S13.21,5.8,12,5.8c-0.53,0-1.05,0.2-1.46,0.57L10,6.86L9.46,6.37C9.05,6,8.53,5.8,8,5.8C6.79,5.8,5.8,6.79,5.8,8 c0,1.07,0.77,1.97,1.79,2.16l-0.61,0.66c-0.48-0.17-0.89-0.47-1.22-0.84C5.41,10.65,5.2,11.4,5.2,12.2c0,0.07,0.01,0.12,0.01,0.19 C6.1,13.85,7.71,14.6,10,14.6C12.3,14.6,13.9,13.87,14.78,12.44z M13.2,8c0,0.77-0.54,1.4-1.2,1.4S10.8,8.77,10.8,8 s0.54-1.4,1.2-1.4S13.2,7.23,13.2,8z M12.6,7.3c0-0.17-0.13-0.3-0.3-0.3C12.13,7,12,7.13,12,7.3s0.13,0.3,0.3,0.3 C12.47,7.6,12.6,7.47,12.6,7.3z M9.2,8c0,0.77-0.54,1.4-1.2,1.4S6.8,8.77,6.8,8S7.34,6.6,8,6.6S9.2,7.23,9.2,8z M8.6,7.3 C8.6,7.13,8.47,7,8.3,7C8.13,7,8,7.13,8,7.3s0.13,0.3,0.3,0.3C8.47,7.6,8.6,7.47,8.6,7.3z",
                }
            }
        }
    }
}

pub fn flutter_dash_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                path {
                    d: "M11.07,11.7c0.29-0.39,0.81-0.56,1.27-0.37c0.17,0.07,0.32,0.18,0.43,0.33c0.22,0.28,0.25,0.59,0.22,0.85 c-0.05,0.33-0.25,0.63-0.54,0.79c0,0-4.87,2.95-5.07,2.69S11.07,11.7,11.07,11.7z M22,10c0,2.5-1,3-1.5,3 c-0.23,0-0.44-0.1-0.62-0.26c-0.48,3.32-2.36,5.31-5.33,5.99c0.11,0.44,0.48,0.77,0.95,0.77l0,0h0.58c0.22,0,0.41,0.15,0.48,0.36 c0.17,0.52,0.66,1.02,1.02,1.32c0.25,0.21,0.24,0.59-0.03,0.78c-0.34,0.24-0.9,0.49-1.79,0.53c-0.18,0.01-0.35-0.07-0.45-0.22 C15.18,22.07,15,21.71,15,21.26c0-0.3,0.04-0.57,0.09-0.8c-0.78-0.16-1.39-0.78-1.55-1.56c-0.49,0.06-1,0.1-1.54,0.1 c-0.88,0-1.7-0.09-2.45-0.25C9.53,18.83,9.5,18.91,9.5,19c0,0.55,0.45,1,1,1l0,0h0.58c0.22,0,0.41,0.15,0.48,0.36 c0.17,0.52,0.66,1.02,1.02,1.32c0.25,0.21,0.24,0.59-0.03,0.78c-0.34,0.24-0.9,0.49-1.79,0.53c-0.18,0.01-0.35-0.07-0.45-0.22 C10.18,22.57,10,22.21,10,21.76c0-0.3,0.04-0.57,0.09-0.8C9.19,20.77,8.5,19.96,8.5,19c0-0.18,0.03-0.36,0.08-0.53 c-2.46-0.86-4.03-2.78-4.46-5.74C3.94,12.9,3.74,13,3.5,13C3,13,2,12.5,2,10c0-2.27,1.7-4.5,3-4.5c0.43,0,0.49,0.49,0.5,0.85 c1.28-1.78,3.26-3.02,5.55-3.29C11.25,2.1,12.13,1.5,13,1.5v1c0,0,0.33-0.5,1-0.5c0.67,0,1,0.5,1,0.5c-0.49,0-0.85,0.35-0.96,0.77 c1.82,0.48,3.39,1.59,4.46,3.08C18.51,5.99,18.57,5.5,19,5.5C20.3,5.5,22,7.73,22,10z M5,11c0,0.81,0.1,1.53,0.25,2.21 c0.18-0.69,0.46-1.33,0.83-1.92c-0.21-0.47-0.34-0.99-0.34-1.54C5.75,7.68,7.43,6,9.5,6c0.96,0,1.84,0.37,2.5,0.97 C12.66,6.37,13.54,6,14.5,6c2.07,0,3.75,1.68,3.75,3.75c0,0.55-0.12,1.07-0.34,1.54c0.37,0.59,0.66,1.24,0.84,1.94 C18.9,12.55,19,11.82,19,11c0-3.86-3.14-7-7-7C8.14,4,5,7.14,5,11z M17.98,15.29c0-0.1,0.02-0.19,0.02-0.29 c0-1.01-0.26-1.95-0.7-2.78c-0.69,0.78-1.68,1.28-2.8,1.28c-0.27,0-0.54-0.03-0.79-0.09c0.14-0.23,0.23-0.49,0.27-0.77 c0.01-0.07,0.01-0.13,0.02-0.19c0.17,0.03,0.33,0.05,0.5,0.05c1.52,0,2.75-1.23,2.75-2.75S16.02,7,14.5,7 c-0.67,0-1.32,0.25-1.83,0.72L12,8.32l-0.67-0.6C10.82,7.25,10.17,7,9.5,7C7.98,7,6.75,8.23,6.75,9.75c0,1.34,0.96,2.46,2.23,2.7 l-0.76,0.83c-0.6-0.22-1.12-0.59-1.53-1.05C6.26,13.06,6,14,6,15c0,0.08,0.01,0.15,0.01,0.24C7.13,17.06,9.14,18,12,18 C14.88,18,16.88,17.09,17.98,15.29z M16,9.75c0,0.97-0.67,1.75-1.5,1.75S13,10.72,13,9.75S13.67,8,14.5,8S16,8.78,16,9.75z M15.25,8.88c0-0.21-0.17-0.38-0.38-0.38S14.5,8.67,14.5,8.88s0.17,0.38,0.38,0.38S15.25,9.08,15.25,8.88z M11,9.75 c0,0.97-0.67,1.75-1.5,1.75S8,10.72,8,9.75S8.67,8,9.5,8S11,8.78,11,9.75z M10.25,8.88c0-0.21-0.17-0.38-0.38-0.38 S9.5,8.67,9.5,8.88s0.17,0.38,0.38,0.38S10.25,9.08,10.25,8.88z",
                }
            }
        }
    }
}

pub fn free_cancellation_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M9.21,16.5H4.5V9h11v3.09l1.5-1.5V5.5C17,4.68,16.33,4,15.5,4H14V2h-1.5v2h-5V2H6v2H4.5C3.67,4,3,4.68,3,5.5v11 C3,17.32,3.67,18,4.5,18h6.21L9.21,16.5z M13.33,16.38l3.54-3.54l1.06,1.06l-4.6,4.6l-2.83-2.83l1.06-1.06L13.33,16.38z M8.81,12.25 L10,13.44L8.94,14.5l-1.19-1.19L6.56,14.5L5.5,13.44l1.19-1.19L5.5,11.06L6.56,10l1.19,1.19L8.94,10L10,11.06L8.81,12.25z",
            }
        }
    }
}

pub fn free_cancellation_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11.21,20H5V10h14v4.38l2-2V6c0-1.1-0.9-2-2-2h-1V2h-2v2H8V2H6v2H5C3.89,4,3.01,4.9,3.01,6L3,20c0,1.1,0.89,2,2,2h8.21 L11.21,20z M16.54,22.5L13,18.96l1.41-1.41l2.12,2.12l4.24-4.24l1.41,1.41L16.54,22.5z M10.41,14L12,15.59L10.59,17L9,15.41L7.41,17 L6,15.59L7.59,14L6,12.41L7.41,11L9,12.59L10.59,11L12,12.41L10.41,14z",
            }
        }
    }
}

pub fn gavel_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    x: "0",
                    height: "24",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    rect {
                        width: "4",
                        x: "11.73",
                        y: "3.73",
                        height: "20",
                        transform: "matrix(0.7075 -0.7067 0.7067 0.7075 -5.6854 13.7194)",
                    }
                    rect {
                        height: "8",
                        x: "11.73",
                        transform: "matrix(0.707 -0.7072 0.7072 0.707 0.3157 11.246)",
                        width: "4",
                        y: "1.24",
                    }
                    rect {
                        y: "9.73",
                        x: "3.24",
                        width: "4",
                        transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -8.1722 7.7256)",
                        height: "8",
                    }
                    rect {
                        x: "1",
                        height: "2",
                        y: "21",
                        width: "12",
                    }
                }
            }
        }
    }
}

pub fn generating_tokens_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M15.06,4.94L13,4l2.06-0.94L16,1l0.94,2.06L19,4l-2.06,0.94L16,7L15.06,4.94z M16,19l0.94-2.06L19,16l-2.06-0.94L16,13 l-0.94,2.06L13,16l2.06,0.94L16,19z M7.5,3.5c3.59,0,6.5,2.91,6.5,6.5s-2.91,6.5-6.5,6.5S1,13.59,1,10S3.91,3.5,7.5,3.5L7.5,3.5z M10,7.5H5v1.25h1.75V13h1.5V8.75H10V7.5z",
            }
        }
    }
}

pub fn generating_tokens_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9,4c-4.42,0-8,3.58-8,8c0,4.42,3.58,8,8,8s8-3.58,8-8C17,7.58,13.42,4,9,4z M12,10.5h-2v5H8v-5H6V9h6V10.5z M20.25,3.75 L23,5l-2.75,1.25L19,9l-1.25-2.75L15,5l2.75-1.25L19,1L20.25,3.75z M20.25,17.75L23,19l-2.75,1.25L19,23l-1.25-2.75L15,19l2.75-1.25 L19,15L20.25,17.75z",
            }
        }
    }
}

pub fn get_app_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 9h-4V3H9v6H5l7 7 7-7zM5 18v2h14v-2H5z",
            }
        }
    }
}

pub fn gif_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                }
            }
            g {
                g {
                    rect {
                        y: "9",
                        width: "1.5",
                        height: "6",
                        x: "11.5",
                    }
                    path {
                        d: "M9,9H6c-0.6,0-1,0.5-1,1v4c0,0.5,0.4,1,1,1h3c0.6,0,1-0.5,1-1v-2H8.5v1.5h-2v-3H10V10C10,9.5,9.6,9,9,9z",
                    }
                    polygon {
                        points: "19,10.5 19,9 14.5,9 14.5,15 16,15 16,13 18,13 18,11.5 16,11.5 16,10.5",
                    }
                }
            }
        }
    }
}

pub fn gif_box_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M7.5,11v-1h1v1c0,0.55-0.45,1-1,1h-1c-0.55,0-1-0.45-1-1V9c0-0.55,0.45-1,1-1h1c0.55,0,1,0.45,1,1h-2v2H7.5z M10.5,12h-1V8h1V12z M14.5,9h-2v0.5H14v1h-1.5V12h-1V8h3V9z",
            }
        }
    }
}

pub fn gif_box_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M9.5,13v-1h1v1c0,0.55-0.45,1-1,1h-1 c-0.55,0-1-0.45-1-1v-2c0-0.55,0.45-1,1-1h1c0.55,0,1,0.45,1,1h-2v2H9.5z M12.5,14h-1v-4h1V14z M16.5,11h-2v0.5H16v1h-1.5V14h-1v-4 h3V11z",
            }
        }
    }
}

pub fn grade_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z",
            }
        }
    }
}

pub fn grading_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        y: "4",
                        height: "1",
                        width: "10",
                        x: "5",
                    }
                    rect {
                        height: "1",
                        x: "5",
                        y: "15",
                        width: "4.5",
                    }
                    rect {
                        y: "12.25",
                        height: "1",
                        width: "4.5",
                        x: "5",
                    }
                    rect {
                        x: "5",
                        y: "6.75",
                        width: "10",
                        height: "1",
                    }
                    rect {
                        height: "1",
                        y: "9.5",
                        width: "10",
                        x: "5",
                    }
                    polygon {
                        points: "12.17,14.59 11.46,13.88 10.76,14.59 12.17,16 15,13.17 14.29,12.46",
                    }
                }
            }
        }
    }
}

pub fn grading_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M4,7h16v2H4V7z M4,13h16v-2H4V13z M4,17h7v-2H4V17z M4,21h7v-2H4V21z M15.41,18.17L14,16.75l-1.41,1.41L15.41,21L20,16.42 L18.58,15L15.41,18.17z M4,3v2h16V3H4z",
                }
            }
        }
    }
}

pub fn group_work_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM8 17.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5zM9.5 8c0-1.38 1.12-2.5 2.5-2.5s2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5S9.5 9.38 9.5 8zm6.5 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z",
            }
        }
    }
}

pub fn g_translate_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21 4H11l-1-3H3c-1.1 0-2 .9-2 2v15c0 1.1.9 2 2 2h8l1 3h9c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zM7 16c-2.76 0-5-2.24-5-5s2.24-5 5-5c1.35 0 2.48.5 3.35 1.3L9.03 8.57c-.38-.36-1.04-.78-2.03-.78-1.74 0-3.15 1.44-3.15 3.21S5.26 14.21 7 14.21c2.01 0 2.84-1.44 2.92-2.41H7v-1.71h4.68c.07.31.12.61.12 1.02C11.8 13.97 9.89 16 7 16zm6.17-5.42h3.7c-.43 1.25-1.11 2.43-2.05 3.47-.31-.35-.6-.72-.86-1.1l-.79-2.37zm8.33 9.92c0 .55-.45 1-1 1H14l2-2.5-1.04-3.1 3.1 3.1.92-.92-3.3-3.25.02-.02c1.13-1.25 1.93-2.69 2.4-4.22H20v-1.3h-4.53V8h-1.29v1.29h-1.44L11.46 5.5h9.04c.55 0 1 .45 1 1v14z",
            }
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
                fill: "none",
            }
        }
    }
}

pub fn help_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13 15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41 0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36 1.68-.93 2.25z",
            }
        }
    }
}

pub fn help_center_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12.01,18 c-0.7,0-1.26-0.56-1.26-1.26c0-0.71,0.56-1.25,1.26-1.25c0.71,0,1.25,0.54,1.25,1.25C13.25,17.43,12.72,18,12.01,18z M15.02,10.6 c-0.76,1.11-1.48,1.46-1.87,2.17c-0.16,0.29-0.22,0.48-0.22,1.41h-1.82c0-0.49-0.08-1.29,0.31-1.98c0.49-0.87,1.42-1.39,1.96-2.16 c0.57-0.81,0.25-2.33-1.37-2.33c-1.06,0-1.58,0.8-1.8,1.48L8.56,8.49C9.01,7.15,10.22,6,11.99,6c1.48,0,2.49,0.67,3.01,1.52 C15.44,8.24,15.7,9.59,15.02,10.6z",
                }
            }
        }
    }
}

pub fn help_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11 18h2v-2h-2v2zm1-16C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm0-14c-2.21 0-4 1.79-4 4h2c0-1.1.9-2 2-2s2 .9 2 2c0 2-3 1.75-3 5h2c0-2.25 3-2.5 3-5 0-2.21-1.79-4-4-4z",
            }
        }
    }
}

pub fn hide_source_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M2.81,2.81L1.39,4.22l2.27,2.27C2.61,8.07,2,9.96,2,12c0,5.52,4.48,10,10,10c2.04,0,3.93-0.61,5.51-1.66l2.27,2.27 l1.41-1.41L2.81,2.81z M12,20c-4.41,0-8-3.59-8-8c0-1.48,0.41-2.86,1.12-4.06l10.94,10.94C14.86,19.59,13.48,20,12,20z M7.94,5.12 L6.49,3.66C8.07,2.61,9.96,2,12,2c5.52,0,10,4.48,10,10c0,2.04-0.61,3.93-1.66,5.51l-1.46-1.46C19.59,14.86,20,13.48,20,12 c0-4.41-3.59-8-8-8C10.52,4,9.14,4.41,7.94,5.12z",
                }
            }
        }
    }
}

pub fn highlight_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17 5h-2V3h2v2zm-2 16h2v-2.59L19.59 21 21 19.59 18.41 17H21v-2h-6v6zm4-12h2V7h-2v2zm0 4h2v-2h-2v2zm-8 8h2v-2h-2v2zM7 5h2V3H7v2zM3 17h2v-2H3v2zm2 4v-2H3c0 1.1.9 2 2 2zM19 3v2h2c0-1.1-.9-2-2-2zm-8 2h2V3h-2v2zM3 9h2V7H3v2zm4 12h2v-2H7v2zm-4-8h2v-2H3v2zm0-8h2V3c-1.1 0-2 .9-2 2z",
            }
        }
    }
}

pub fn highlight_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M14.59 8L12 10.59 9.41 8 8 9.41 10.59 12 8 14.59 9.41 16 12 13.41 14.59 16 16 14.59 13.41 12 16 9.41 14.59 8zM12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z",
            }
        }
    }
}

pub fn history_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42C8.27 19.99 10.51 21 13 21c4.97 0 9-4.03 9-9s-4.03-9-9-9zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z",
            }
        }
    }
}

pub fn history_toggle_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M7.45,4.58l-0.5-0.87C7.73,3.33,8.59,3.09,9.5,3.03v1C8.77,4.09,8.08,4.28,7.45,4.58z M15.97,9.5h1 c-0.06-0.91-0.3-1.77-0.69-2.55l-0.87,0.5C15.72,8.08,15.91,8.77,15.97,9.5z M12.55,4.58l0.5-0.87c-0.78-0.38-1.64-0.62-2.55-0.69 v1C11.23,4.09,11.92,4.28,12.55,4.58z M6.58,5.08l-0.5-0.87c-0.74,0.5-1.37,1.14-1.87,1.87l0.87,0.5C5.48,5.99,5.99,5.48,6.58,5.08 z M14.92,6.58l0.87-0.5c-0.5-0.74-1.14-1.37-1.87-1.87l-0.5,0.87C14.01,5.49,14.52,5.99,14.92,6.58z M7.45,15.42l-0.5,0.87 c0.78,0.38,1.64,0.62,2.55,0.69v-1C8.77,15.91,8.08,15.72,7.45,15.42z M4.03,10.5h-1c0.06,0.91,0.3,1.77,0.69,2.55l0.87-0.5 C4.28,11.92,4.09,11.23,4.03,10.5z M10.5,15.97v1c0.91-0.06,1.77-0.3,2.55-0.69l-0.5-0.87C11.92,15.72,11.23,15.91,10.5,15.97z M5.08,13.42l-0.87,0.5c0.5,0.74,1.14,1.37,1.87,1.87l0.5-0.87C5.99,14.52,5.48,14.01,5.08,13.42z M4.58,7.45l-0.87-0.5 C3.33,7.73,3.09,8.59,3.03,9.5h1C4.09,8.77,4.28,8.08,4.58,7.45z M15.97,10.5c-0.06,0.73-0.25,1.42-0.55,2.05l0.87,0.5 c0.38-0.78,0.62-1.64,0.69-2.55H15.97z M13.42,14.92l0.5,0.87c0.74-0.5,1.37-1.14,1.87-1.87l-0.87-0.5 C14.52,14.01,14.01,14.51,13.42,14.92z M10.5,7.05h-1v4l3.17,1.9l0.5-0.82l-2.67-1.58V7.05z",
                }
            }
        }
    }
}

pub fn history_toggle_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.1,19.37l1,1.74c-0.96,0.44-2.01,0.73-3.1,0.84v-2.02C13.74,19.84,14.44,19.65,15.1,19.37z M4.07,13H2.05 c0.11,1.1,0.4,2.14,0.84,3.1l1.74-1C4.35,14.44,4.16,13.74,4.07,13z M15.1,4.63l1-1.74C15.14,2.45,14.1,2.16,13,2.05v2.02 C13.74,4.16,14.44,4.35,15.1,4.63z M19.93,11h2.02c-0.11-1.1-0.4-2.14-0.84-3.1l-1.74,1C19.65,9.56,19.84,10.26,19.93,11z M8.9,19.37l-1,1.74c0.96,0.44,2.01,0.73,3.1,0.84v-2.02C10.26,19.84,9.56,19.65,8.9,19.37z M11,4.07V2.05 c-1.1,0.11-2.14,0.4-3.1,0.84l1,1.74C9.56,4.35,10.26,4.16,11,4.07z M18.36,7.17l1.74-1.01c-0.63-0.87-1.4-1.64-2.27-2.27 l-1.01,1.74C17.41,6.08,17.92,6.59,18.36,7.17z M4.63,8.9l-1.74-1C2.45,8.86,2.16,9.9,2.05,11h2.02C4.16,10.26,4.35,9.56,4.63,8.9z M19.93,13c-0.09,0.74-0.28,1.44-0.56,2.1l1.74,1c0.44-0.96,0.73-2.01,0.84-3.1H19.93z M16.83,18.36l1.01,1.74 c0.87-0.63,1.64-1.4,2.27-2.27l-1.74-1.01C17.92,17.41,17.41,17.92,16.83,18.36z M7.17,5.64L6.17,3.89 C5.29,4.53,4.53,5.29,3.9,6.17l1.74,1.01C6.08,6.59,6.59,6.08,7.17,5.64z M5.64,16.83L3.9,17.83c0.63,0.87,1.4,1.64,2.27,2.27 l1.01-1.74C6.59,17.92,6.08,17.41,5.64,16.83z M13,7h-2v5.41l4.29,4.29l1.41-1.41L13,11.59V7z",
                }
            }
        }
    }
}

pub fn hls_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6,8h1v4H6v-1.5H5V12H4V8h1v1.5h1V8z M13.5,12h2c0.28,0,0.5-0.22,0.5-0.5v-1.25c0-0.28-0.22-0.5-0.5-0.5H14V9h1v0.25h1V8.5 C16,8.22,15.78,8,15.5,8h-2C13.22,8,13,8.22,13,8.5v1.25c0,0.28,0.22,0.5,0.5,0.5H15V11h-1v-0.25h-1v0.75 C13,11.78,13.22,12,13.5,12z M11.5,11H10V8H9v4l2.5,0V11z",
                }
            }
        }
    }
}

pub fn hls_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6.5,9H8v6H6.5v-2.5h-2V15H3V9h1.5v2h2V9z M16.5,15h3c0.55,0,1-0.45,1-1v-1.5c0-0.55-0.45-1-1-1H17v-1h2V11h1.5v-1 c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v1.5c0,0.55,0.45,1,1,1H19v1h-2V13h-1.5v1C15.5,14.55,15.95,15,16.5,15z M14,15v-1.5h-2.5 V9H10v6H14z",
                }
            }
        }
    }
}

pub fn hls_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.5,12c0.28,0,0.5-0.22,0.5-0.5v-1.25c0-0.28-0.22-0.5-0.5-0.5H14V9h1v0.25h1V8.5C16,8.22,15.78,8,15.5,8h-2 C13.22,8,13,8.22,13,8.5v1.25c0,0.28,0.22,0.5,0.5,0.5H15V11h-1v-0.25h-1v0.13L14.12,12H15.5z",
                    }
                    polygon {
                        points: "2.93,2.93 1.87,3.99 6,8.12 6,9.5 5,9.5 5,8 4,8 4,12 5,12 5,10.5 6,10.5 6,12 7,12 7,9.12 9,11.12 9,12 9.88,12 16.01,18.13 17.07,17.07",
                    }
                }
            }
        }
    }
}

pub fn hls_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17.83,15h1.67c0.55,0,1-0.45,1-1v-1.5c0-0.55-0.45-1-1-1H17v-1h2V11h1.5v-1c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v1.5 c0,0.55,0.45,1,1,1H19v1h-2V13h-1.17L17.83,15z M8,10.83V15H6.5v-2.5h-2V15H3V9h1.5v2h2V9.33L1.39,4.22l1.41-1.41l18.38,18.38 l-1.41,1.41L12.17,15H10v-2.17L8,10.83z",
                }
            }
        }
    }
}

pub fn home_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z",
            }
        }
    }
}

pub fn home_filled_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 3L4 9v12h5v-7h6v7h5V9z",
            }
        }
    }
}

pub fn horizontal_split_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 19h18v-6H3v6zm0-8h18V9H3v2zm0-6v2h18V5H3z",
            }
        }
    }
}

pub fn hotel_class_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M7.06 8H1l4.94 3.93L4.06 18 9 14.25 13.94 18l-1.88-6.07L17 8h-6.06L9 2 7.06 8zm4.3-3.56.67 2.06h1.58L12.15 2l-.79 2.44zM16.9 10l-3.1 2.47 1.23 3.97L17.08 18l-1.55-5 3.77-3h-2.4z",
            }
        }
    }
}

pub fn hotel_class_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M8.58 10H1l6.17 4.41L4.83 22 11 17.31 17.18 22l-2.35-7.59L21 10h-7.58L11 2l-2.42 8zm12.78 12-1.86-6.01L23.68 13h-3.44l-3.08 2.2 1.46 4.72L21.36 22zM17 8l-1.82-6-1.04 3.45.77 2.55H17z",
            }
        }
    }
}

pub fn hourglass_disabled_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "7,4 13,4 13,6.59 10.5,9.09 11.21,9.79 14,7 14,7 14,3 6,3 6,4.59 7,5.59",
                    }
                    path {
                        d: "M2.93,2.93L2.22,3.64l6.57,6.57L6,13h0v4h8v-1.59l2.36,2.36l0.71-0.71L2.93,2.93z M13,16H7l0-2.59l2.5-2.5l3.5,3.5V16z",
                    }
                }
            }
        }
    }
}

pub fn hourglass_disabled_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "8,4 16,4 16,7.5 13.16,10.34 14.41,11.59 18,8.01 17.99,8 18,8 18,2 6,2 6,3.17 8,5.17",
                    }
                    path {
                        d: "M2.1,2.1L0.69,3.51l8.9,8.9L6,16l0.01,0.01H6V22h12v-1.17l2.49,2.49l1.41-1.41L2.1,2.1z M16,20H8v-3.5l2.84-2.84L16,18.83 V20z",
                    }
                }
            }
        }
    }
}

pub fn hourglass_empty_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M6 2v6h.01L6 8.01 10 12l-4 4 .01.01H6V22h12v-5.99h-.01L18 16l-4-4 4-3.99-.01-.01H18V2H6zm10 14.5V20H8v-3.5l4-4 4 4zm-4-5l-4-4V4h8v3.5l-4 4z",
            }
        }
    }
}

pub fn hourglass_full_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M6 2v6h.01L6 8.01 10 12l-4 4 .01.01H6V22h12v-5.99h-.01L18 16l-4-4 4-3.99-.01-.01H18V2H6z",
            }
        }
    }
}

pub fn html_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M4,8h1v4H4v-1.5H3V12H2V8h1v1.5h1V8z M6,9h1v3h1V9h1V8H6V9z M18.5,11H17V8h-1v4l2.5,0V11z M14.5,8h-4 C10.22,8,10,8.22,10,8.5V12h1V9h1v2h1V9h1v3h1V8.5C15,8.22,14.78,8,14.5,8z",
                }
            }
        }
    }
}

pub fn html_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M3.5,9H5v6H3.5v-2.5h-2V15H0V9h1.5v2h2V9z M17.5,9H13c-0.55,0-1,0.45-1,1v5h1.5v-4.5h1V14H16v-3.51h1V15h1.5v-5 C18.5,9.45,18.05,9,17.5,9z M11,9H6v1.5h1.75V15h1.5v-4.5H11V9z M24,15v-1.5h-2.5V9H20v6H24z",
                }
            }
        }
    }
}

pub fn http_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M4.5 11h-2V9H1v6h1.5v-2.5h2V15H6V9H4.5v2zm2.5-.5h1.5V15H10v-4.5h1.5V9H7v1.5zm5.5 0H14V15h1.5v-4.5H17V9h-4.5v1.5zm9-1.5H18v6h1.5v-2h2c.8 0 1.5-.7 1.5-1.5v-1c0-.8-.7-1.5-1.5-1.5zm0 2.5h-2v-1h2v1z",
            }
            path {
                d: "M24 24H0V0h24v24z",
                fill: "none",
            }
        }
    }
}

pub fn https_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z",
            }
        }
    }
}

pub fn important_devices_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M23 11.01L18 11c-.55 0-1 .45-1 1v9c0 .55.45 1 1 1h5c.55 0 1-.45 1-1v-9c0-.55-.45-.99-1-.99zM23 20h-5v-7h5v7zM20 2H2C.89 2 0 2.89 0 4v12c0 1.1.89 2 2 2h7v2H7v2h8v-2h-2v-2h2v-2H2V4h18v5h2V4c0-1.11-.9-2-2-2zm-8.03 7L11 6l-.97 3H7l2.47 1.76-.94 2.91 2.47-1.8 2.47 1.8-.94-2.91L15 9h-3.03z",
            }
        }
    }
}

pub fn info_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z",
            }
        }
    }
}

pub fn info_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                path {
                    d: "M11,7h2v2h-2V7z M11,11h2v6h-2V11z M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20 c-4.41,0-8-3.59-8-8s3.59-8,8-8s8,3.59,8,8S16.41,20,12,20z",
                }
            }
        }
    }
}

pub fn input_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21 3.01H3c-1.1 0-2 .9-2 2V9h2V4.99h18v14.03H3V15H1v4.01c0 1.1.9 1.98 2 1.98h18c1.1 0 2-.88 2-1.98v-14c0-1.11-.9-2-2-2zM11 16l4-4-4-4v3H1v2h10v3z",
            }
        }
    }
}

pub fn install_desktop_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.5,13.5h-13v-9H11V3H3.5C2.67,3,2,3.67,2,4.5v9C2,14.33,2.67,15,3.5,15H7v2h6v-2h3.5c0.83,0,1.5-0.67,1.5-1.5V11h-1.5 V13.5z",
                    }
                    polygon {
                        points: "14.48,11 18,7.48 16.94,6.42 15.25,8.11 15.25,3 13.75,3 13.75,8.14 12.06,6.45 11,7.52",
                    }
                }
            }
        }
    }
}

pub fn install_desktop_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,17H4V5h8V3H4C2.89,3,2,3.89,2,5v12c0,1.1,0.89,2,2,2h4v2h8v-2h4c1.1,0,2-0.9,2-2v-3h-2V17z",
                    }
                    polygon {
                        points: "17,14 22,9 20.59,7.59 18,10.17 18,3 16,3 16,10.17 13.41,7.59 12,9",
                    }
                }
            }
        }
    }
}

pub fn install_mobile_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14.5,15h-9V5H12V1H5.5C4.67,1,4,1.67,4,2.5v15C4,18.33,4.67,19,5.5,19h9c0.83,0,1.5-0.67,1.5-1.5V13h-1.5V15z",
                    }
                    polygon {
                        points: "15.23,11 18.75,7.48 17.69,6.42 16,8.11 16,3 14.5,3 14.5,8.14 12.81,6.45 11.75,7.52",
                    }
                }
            }
        }
    }
}

pub fn install_mobile_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M17,18H7V6h7V1H7C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2v-5h-2V18z",
                    }
                    polygon {
                        points: "18,14 23,9 21.59,7.59 19,10.17 19,3 17,3 17,10.17 14.41,7.59 13,9",
                    }
                }
            }
        }
    }
}

pub fn integration_instructions_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                circle {
                    r: ".75",
                    fill: "none",
                    cy: "3.5",
                    cx: "12",
                }
                circle {
                    cy: "3.5",
                    cx: "12",
                    r: ".75",
                    fill: "none",
                }
                circle {
                    cx: "12",
                    fill: "none",
                    r: ".75",
                    cy: "3.5",
                }
                path {
                    d: "M19,3h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H5C4.86,3,4.73,3.01,4.6,3.04C4.21,3.12,3.86,3.32,3.59,3.59 c-0.18,0.18-0.33,0.4-0.43,0.64C3.06,4.46,3,4.72,3,5v14c0,0.27,0.06,0.54,0.16,0.78c0.1,0.24,0.25,0.45,0.43,0.64 c0.27,0.27,0.62,0.47,1.01,0.55C4.73,20.99,4.86,21,5,21h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M11,14.17l-1.41,1.42L6,12 l3.59-3.59L11,9.83L8.83,12L11,14.17z M12,4.25c-0.41,0-0.75-0.34-0.75-0.75S11.59,2.75,12,2.75s0.75,0.34,0.75,0.75 S12.41,4.25,12,4.25z M14.41,15.59L13,14.17L15.17,12L13,9.83l1.41-1.42L18,12L14.41,15.59z",
                }
            }
        }
    }
}

pub fn invert_colors_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M14.53,6.59L14.53,6.59L10,2L5.5,6.56c-1.08,1.11-1.75,2.62-1.75,4.28c0,3.4,2.8,6.16,6.25,6.16s6.25-2.76,6.25-6.16 C16.25,9.19,15.6,7.7,14.53,6.59z M5.25,10.84c0-1.21,0.47-2.35,1.32-3.22L10,4.14V15.5C7.38,15.5,5.25,13.41,5.25,10.84z",
            }
        }
    }
}

pub fn invert_colors_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M12,4.81L12,19c-3.31,0-6-2.63-6-5.87c0-1.56,0.62-3.03,1.75-4.14L12,4.81 M6.35,7.56L6.35,7.56C4.9,8.99,4,10.96,4,13.13 C4,17.48,7.58,21,12,21c4.42,0,8-3.52,8-7.87c0-2.17-0.9-4.14-2.35-5.57l0,0L12,2L6.35,7.56z",
                }
            }
        }
    }
}

pub fn javascript_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.5,11.5v-0.75h1V11h1v-0.75H11c-0.28,0-0.5-0.22-0.5-0.5V8.5C10.5,8.22,10.72,8,11,8h2c0.28,0,0.5,0.22,0.5,0.5v0.75h-1 V9h-1v0.75H13c0.28,0,0.5,0.22,0.5,0.5v1.25c0,0.28-0.22,0.5-0.5,0.5h-2C10.72,12,10.5,11.78,10.5,11.5z M8,8v3H7v-1H6v1.5 C6,11.78,6.22,12,6.5,12h2C8.78,12,9,11.78,9,11.5V8H8z",
                }
            }
        }
    }
}

pub fn javascript_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,14v-1h1.5v0.5h2v-1H13c-0.55,0-1-0.45-1-1V10c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1v1h-1.5v-0.5h-2v1H16 c0.55,0,1,0.45,1,1V14c0,0.55-0.45,1-1,1h-3C12.45,15,12,14.55,12,14z M9,9v4.5H7.5v-1H6v1C6,14.33,6.67,15,7.5,15H9 c0.83,0,1.5-0.67,1.5-1.5V9C10.5,9,9.83,9,9,9z",
                }
            }
        }
    }
}

pub fn join_full_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,5.76C8.27,7.19,8,9.08,8,10c0,0.91,0.27,2.81,2,4.24c1.73-1.43,2-3.33,2-4.24C12,9.08,11.73,7.19,10,5.76z",
                    }
                    path {
                        d: "M13.5,4.5c-0.96,0-1.86,0.25-2.65,0.69C12.77,6.92,13,9.11,13,10c0,0.89-0.23,3.08-2.15,4.81 c0.79,0.43,1.69,0.69,2.65,0.69c3.04,0,5.5-2.46,5.5-5.5C19,6.96,16.54,4.5,13.5,4.5z",
                    }
                    path {
                        d: "M9.15,5.19C8.36,4.75,7.46,4.5,6.5,4.5C3.46,4.5,1,6.96,1,10c0,3.04,2.46,5.5,5.5,5.5c0.96,0,1.86-0.25,2.65-0.69 C7.23,13.08,7,10.89,7,10C7,9.11,7.23,6.92,9.15,5.19z",
                    }
                }
            }
        }
    }
}

pub fn join_full_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    ellipse {
                        ry: "5.74",
                        cx: "12",
                        cy: "12",
                        rx: "3",
                    }
                    path {
                        d: "M7.5,12c0-0.97,0.23-4.16,3.03-6.5C9.75,5.19,8.9,5,8,5c-3.86,0-7,3.14-7,7s3.14,7,7,7c0.9,0,1.75-0.19,2.53-0.5 C7.73,16.16,7.5,12.97,7.5,12z",
                    }
                    path {
                        d: "M16,5c-0.9,0-1.75,0.19-2.53,0.5c2.8,2.34,3.03,5.53,3.03,6.5c0,0.97-0.23,4.16-3.03,6.5C14.25,18.81,15.1,19,16,19 c3.86,0,7-3.14,7-7S19.86,5,16,5z",
                    }
                }
            }
        }
    }
}

pub fn join_inner_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,5.76C8.27,7.19,8,9.08,8,10c0,0.91,0.27,2.81,2,4.24c1.73-1.43,2-3.33,2-4.24C12,9.08,11.73,7.19,10,5.76z",
                    }
                    path {
                        d: "M13.5,4.5c-0.96,0-1.86,0.25-2.65,0.69c0.42,0.38,0.77,0.77,1.04,1.18C12.38,6.14,12.92,6,13.5,6c2.21,0,4,1.79,4,4 s-1.79,4-4,4c-0.58,0-1.12-0.14-1.61-0.36c-0.27,0.4-0.62,0.8-1.04,1.18c0.79,0.43,1.69,0.69,2.65,0.69c3.04,0,5.5-2.46,5.5-5.5 C19,6.96,16.54,4.5,13.5,4.5z",
                    }
                    path {
                        d: "M6.5,14c-2.21,0-4-1.79-4-4s1.79-4,4-4c0.58,0,1.12,0.14,1.61,0.36c0.27-0.4,0.62-0.8,1.04-1.18 C8.36,4.75,7.46,4.5,6.5,4.5C3.46,4.5,1,6.96,1,10c0,3.04,2.46,5.5,5.5,5.5c0.96,0,1.86-0.25,2.65-0.69 c-0.42-0.38-0.77-0.77-1.04-1.18C7.62,13.86,7.08,14,6.5,14z",
                    }
                }
            }
        }
    }
}

pub fn join_inner_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                ellipse {
                    cx: "12",
                    rx: "3",
                    cy: "12",
                    ry: "5.74",
                }
                g {
                    path {
                        d: "M9.04,16.87C8.71,16.95,8.36,17,8,17c-2.76,0-5-2.24-5-5s2.24-5,5-5c0.36,0,0.71,0.05,1.04,0.13 c0.39-0.56,0.88-1.12,1.49-1.63C9.75,5.19,8.9,5,8,5c-3.86,0-7,3.14-7,7s3.14,7,7,7c0.9,0,1.75-0.19,2.53-0.5 C9.92,17.99,9.43,17.43,9.04,16.87z",
                    }
                }
                path {
                    d: "M16,5c-0.9,0-1.75,0.19-2.53,0.5c0.61,0.51,1.1,1.07,1.49,1.63C15.29,7.05,15.64,7,16,7c2.76,0,5,2.24,5,5s-2.24,5-5,5 c-0.36,0-0.71-0.05-1.04-0.13c-0.39,0.56-0.88,1.12-1.49,1.63C14.25,18.81,15.1,19,16,19c3.86,0,7-3.14,7-7S19.86,5,16,5z",
                }
            }
        }
    }
}

pub fn join_left_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,5.76C8.27,7.19,8,9.08,8,10c0,0.91,0.27,2.81,2,4.24c1.73-1.43,2-3.33,2-4.24C12,9.08,11.73,7.19,10,5.76z",
                    }
                    path {
                        d: "M9.15,5.19C8.36,4.75,7.46,4.5,6.5,4.5C3.46,4.5,1,6.96,1,10c0,3.04,2.46,5.5,5.5,5.5c0.96,0,1.86-0.25,2.65-0.69 C7.23,13.08,7,10.89,7,10C7,9.11,7.23,6.92,9.15,5.19z",
                    }
                    path {
                        d: "M13.5,4.5c-0.96,0-1.86,0.25-2.65,0.69c0.42,0.38,0.77,0.77,1.04,1.18C12.38,6.14,12.92,6,13.5,6c2.21,0,4,1.79,4,4 s-1.79,4-4,4c-0.58,0-1.12-0.14-1.61-0.36c-0.27,0.4-0.62,0.8-1.04,1.18c0.79,0.43,1.69,0.69,2.65,0.69c3.04,0,5.5-2.46,5.5-5.5 C19,6.96,16.54,4.5,13.5,4.5z",
                    }
                }
            }
        }
    }
}

pub fn join_left_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        ellipse {
                            cx: "12",
                            rx: "3",
                            ry: "5.74",
                            cy: "12",
                        }
                    }
                    g {
                        path {
                            d: "M7.5,12c0-0.97,0.23-4.16,3.03-6.5C9.75,5.19,8.9,5,8,5c-3.86,0-7,3.14-7,7s3.14,7,7,7c0.9,0,1.75-0.19,2.53-0.5 C7.73,16.16,7.5,12.97,7.5,12z",
                        }
                    }
                    g {
                        path {
                            d: "M16,5c-0.9,0-1.75,0.19-2.53,0.5c0.61,0.51,1.1,1.07,1.49,1.63C15.29,7.05,15.64,7,16,7c2.76,0,5,2.24,5,5s-2.24,5-5,5 c-0.36,0-0.71-0.05-1.04-0.13c-0.39,0.56-0.88,1.12-1.49,1.63C14.25,18.81,15.1,19,16,19c3.86,0,7-3.14,7-7S19.86,5,16,5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn join_right_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,14.24c1.73-1.43,2-3.32,2-4.24c0-0.91-0.27-2.81-2-4.24C8.27,7.19,8,9.09,8,10C8,10.92,8.27,12.81,10,14.24z",
                    }
                    path {
                        d: "M10.85,14.81c0.79,0.43,1.69,0.69,2.65,0.69c3.04,0,5.5-2.46,5.5-5.5c0-3.04-2.46-5.5-5.5-5.5 c-0.96,0-1.86,0.25-2.65,0.69C12.77,6.92,13,9.11,13,10C13,10.89,12.77,13.08,10.85,14.81z",
                    }
                    path {
                        d: "M6.5,15.5c0.96,0,1.86-0.25,2.65-0.69c-0.42-0.38-0.77-0.77-1.04-1.18C7.62,13.86,7.08,14,6.5,14c-2.21,0-4-1.79-4-4 s1.79-4,4-4c0.58,0,1.12,0.14,1.61,0.36c0.27-0.4,0.62-0.8,1.04-1.18C8.36,4.75,7.46,4.5,6.5,4.5C3.46,4.5,1,6.96,1,10 C1,13.04,3.46,15.5,6.5,15.5z",
                    }
                }
            }
        }
    }
}

pub fn join_right_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        ellipse {
                            cx: "12",
                            rx: "3",
                            ry: "5.74",
                            cy: "12",
                        }
                    }
                    g {
                        path {
                            d: "M16.5,12c0,0.97-0.23,4.16-3.03,6.5C14.25,18.81,15.1,19,16,19c3.86,0,7-3.14,7-7s-3.14-7-7-7c-0.9,0-1.75,0.19-2.53,0.5 C16.27,7.84,16.5,11.03,16.5,12z",
                        }
                    }
                    g {
                        path {
                            d: "M8,19c0.9,0,1.75-0.19,2.53-0.5c-0.61-0.51-1.1-1.07-1.49-1.63C8.71,16.95,8.36,17,8,17c-2.76,0-5-2.24-5-5s2.24-5,5-5 c0.36,0,0.71,0.05,1.04,0.13c0.39-0.56,0.88-1.12,1.49-1.63C9.75,5.19,8.9,5,8,5c-3.86,0-7,3.14-7,7S4.14,19,8,19z",
                        }
                    }
                }
            }
        }
    }
}

pub fn label_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17.63 5.84C17.27 5.33 16.67 5 16 5L5 5.01C3.9 5.01 3 5.9 3 7v10c0 1.1.9 1.99 2 1.99L16 19c.67 0 1.27-.33 1.63-.84L22 12l-4.37-6.16z",
            }
        }
    }
}

pub fn label_important_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3.5 18.99l11 .01c.67 0 1.27-.33 1.63-.84L20.5 12l-4.37-6.16c-.36-.51-.96-.84-1.63-.84l-11 .01L8.34 12 3.5 18.99z",
            }
        }
    }
}

pub fn label_important_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15 19H3l4.5-7L3 5h12c.65 0 1.26.31 1.63.84L21 12l-4.37 6.16c-.37.52-.98.84-1.63.84zm-8.5-2H15l3.5-5L15 7H6.5l3.5 5-3.5 5z",
            }
        }
    }
}

pub fn label_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3.25 2.75l17 17L19 21l-2-2H5c-1.1 0-2-.9-2-2V7c0-.55.23-1.05.59-1.41L2 4l1.25-1.25zM22 12l-4.37-6.16C17.27 5.33 16.67 5 16 5H8l11 11 3-4z",
            }
        }
    }
}

pub fn label_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17.63 5.84C17.27 5.33 16.67 5 16 5L5 5.01C3.9 5.01 3 5.9 3 7v10c0 1.1.9 1.99 2 1.99L16 19c.67 0 1.27-.33 1.63-.84L22 12l-4.37-6.16zM16 17H5V7h11l3.55 5L16 17z",
            }
        }
    }
}

pub fn language_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zm6.93 6h-2.95c-.32-1.25-.78-2.45-1.38-3.56 1.84.63 3.37 1.91 4.33 3.56zM12 4.04c.83 1.2 1.48 2.53 1.91 3.96h-3.82c.43-1.43 1.08-2.76 1.91-3.96zM4.26 14C4.1 13.36 4 12.69 4 12s.1-1.36.26-2h3.38c-.08.66-.14 1.32-.14 2 0 .68.06 1.34.14 2H4.26zm.82 2h2.95c.32 1.25.78 2.45 1.38 3.56-1.84-.63-3.37-1.9-4.33-3.56zm2.95-8H5.08c.96-1.66 2.49-2.93 4.33-3.56C8.81 5.55 8.35 6.75 8.03 8zM12 19.96c-.83-1.2-1.48-2.53-1.91-3.96h3.82c-.43 1.43-1.08 2.76-1.91 3.96zM14.34 14H9.66c-.09-.66-.16-1.32-.16-2 0-.68.07-1.35.16-2h4.68c.09.65.16 1.32.16 2 0 .68-.07 1.34-.16 2zm.25 5.56c.6-1.11 1.06-2.31 1.38-3.56h2.95c-.96 1.65-2.49 2.93-4.33 3.56zM16.36 14c.08-.66.14-1.32.14-2 0-.68-.06-1.34-.14-2h3.38c.16.64.26 1.31.26 2s-.1 1.36-.26 2h-3.38z",
            }
        }
    }
}

pub fn launch_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z",
            }
        }
    }
}

pub fn leaderboard_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                path {
                    d: "M7.5,21H2V9h5.5V21z M14.75,3h-5.5v18h5.5V3z M22,11h-5.5v10H22V11z",
                }
            }
        }
    }
}

pub fn lightbulb_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M9 21c0 .5.4 1 1 1h4c.6 0 1-.5 1-1v-1H9v1zm3-19C8.1 2 5 5.1 5 9c0 2.4 1.2 4.5 3 5.7V17c0 .5.4 1 1 1h6c.6 0 1-.5 1-1v-2.3c1.8-1.3 3-3.4 3-5.7 0-3.9-3.1-7-7-7z",
            }
        }
    }
}

pub fn lightbulb_circle_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,2c-4.42,0-8,3.58-8,8s3.58,8,8,8s8-3.58,8-8S14.42,2,10,2z M10,15.5c-0.55,0-1-0.45-1-1h2 C11,15.05,10.55,15.5,10,15.5z M12.5,13.75h-5v-1h5V13.75z M12.62,12H7.38C6.54,11.27,6,10.2,6,9c0-2.21,1.79-4,4-4s4,1.79,4,4 C14,10.2,13.46,11.27,12.62,12z",
                    }
                }
            }
        }
    }
}

pub fn lightbulb_circle_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M12,19c-0.83,0-1.5-0.67-1.5-1.5h3 C13.5,18.33,12.83,19,12,19z M15,16.5H9V15h6V16.5z M14.97,14H9.03C7.8,13.09,7,11.64,7,10c0-2.76,2.24-5,5-5s5,2.24,5,5 C17,11.64,16.2,13.09,14.97,14z",
                    }
                }
            }
        }
    }
}

pub fn lightbulb_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    g {
                        path {
                            d: "M9,21c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-1H9V21z M12,2C8.14,2,5,5.14,5,9c0,2.38,1.19,4.47,3,5.74V17 c0,0.55,0.45,1,1,1h6c0.55,0,1-0.45,1-1v-2.26c1.81-1.27,3-3.36,3-5.74C19,5.14,15.86,2,12,2z M14,13.7V16h-4v-2.3 C8.48,12.63,7,11.53,7,9c0-2.76,2.24-5,5-5s5,2.24,5,5C17,11.49,15.49,12.65,14,13.7z",
                        }
                    }
                }
            }
        }
    }
}

pub fn line_style_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M3,16h5v-2H3V16z M9.5,16h5v-2h-5V16z M16,16h5v-2h-5V16z M3,20h2v-2H3V20z M7,20h2v-2H7V20z M11,20h2v-2h-2V20z M15,20 h2v-2h-2V20z M19,20h2v-2h-2V20z M3,12h8v-2H3V12z M13,12h8v-2h-8V12z M3,4v4h18V4H3z",
                        }
                    }
                }
            }
        }
    }
}

pub fn line_weight_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    width: "24",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M3,17h18v-2H3V17z M3,20h18v-1H3V20z M3,13h18v-3H3V13z M3,4v4h18V4H3z",
                        }
                    }
                }
            }
        }
    }
}

pub fn list_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z",
            }
        }
    }
}

pub fn lock_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm-6 9c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zm3.1-9H8.9V6c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2z",
            }
        }
    }
}

pub fn lock_clock_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h20v20H0V0z",
                fill: "none",
            }
            path {
                d: "M12.5 8H11v5l3.6 2.2.8-1.3-2.9-1.7V8zM12 5c-.3 0-.7 0-1 .1V5c0-2.1-1.9-4-4-4S3 2.9 3 5v1h-.5C1.7 6 1 6.7 1 7.5v8c0 .8.7 1.5 1.5 1.5h4.6c1.3 1.2 3 2 4.9 2 3.9 0 7-3.1 7-7s-3.1-7-7-7zM5 5c0-1.1.9-2 2-2s2 .9 2 2v1H5V5zm7 12c-2.8 0-5-2.2-5-5s2.2-5 5-5 5 2.2 5 5-2.2 5-5 5z",
            }
        }
    }
}

pub fn lock_clock_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M14.5 14.2l2.9 1.7-.8 1.3L13 15v-5h1.5v4.2zM22 14c0 4.41-3.59 8-8 8-2.02 0-3.86-.76-5.27-2H4c-1.15 0-2-.85-2-2V9c0-1.12.89-1.96 2-2v-.5C4 4.01 6.01 2 8.5 2c2.34 0 4.24 1.79 4.46 4.08.34-.05.69-.08 1.04-.08 4.41 0 8 3.59 8 8zM6 7h5v-.74C10.88 4.99 9.8 4 8.5 4 7.12 4 6 5.12 6 6.5V7zm14 7c0-3.31-2.69-6-6-6s-6 2.69-6 6 2.69 6 6 6 6-2.69 6-6z",
            }
        }
    }
}

pub fn lock_open_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 17c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm6-9h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6h1.9c0-1.71 1.39-3.1 3.1-3.1 1.71 0 3.1 1.39 3.1 3.1v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zm0 12H6V10h12v10z",
            }
        }
    }
}

pub fn lock_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                    height: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M12,17c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S10.9,17,12,17z M18,8h-1V6c0-2.76-2.24-5-5-5S7,3.24,7,6v2H6 c-1.1,0-2,0.9-2,2v10c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V10C20,8.9,19.1,8,18,8z M8.9,6c0-1.71,1.39-3.1,3.1-3.1 s3.1,1.39,3.1,3.1v2H8.9V6z M18,20H6V10h12V20z",
                        }
                    }
                }
            }
        }
    }
}

pub fn lock_person_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15,9.5c0.34,0,0.68,0.04,1,0.1V8.5C16,7.67,15.33,7,14.5,7H14V5c0-2.21-1.79-4-4-4S6,2.79,6,5v2H5.5C4.67,7,4,7.67,4,8.5 v8C4,17.33,4.67,18,5.5,18h4.89c-0.56-0.86-0.89-1.89-0.89-3C9.5,11.96,11.96,9.5,15,9.5z M7.5,5c0-1.38,1.12-2.5,2.5-2.5 s2.5,1.12,2.5,2.5v2h-5V5z",
                    }
                    path {
                        d: "M15,11c-2.21,0-4,1.79-4,4s1.79,4,4,4s4-1.79,4-4S17.21,11,15,11z M15,12c0.83,0,1.5,0.67,1.5,1.5S15.83,15,15,15 c-0.83,0-1.5-0.67-1.5-1.5S14.17,12,15,12z M15,18c-1.09,0-2.03-0.58-2.56-1.45c0.73-0.5,1.61-0.8,2.56-0.8s1.83,0.3,2.56,0.8 C17.03,17.42,16.09,18,15,18z",
                    }
                }
            }
        }
    }
}

pub fn lock_person_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18,11c0.7,0,1.37,0.1,2,0.29V10c0-1.1-0.9-2-2-2h-1V6c0-2.76-2.24-5-5-5S7,3.24,7,6v2H6c-1.1,0-2,0.9-2,2v10 c0,1.1,0.9,2,2,2h6.26C11.47,20.87,11,19.49,11,18C11,14.13,14.13,11,18,11z M8.9,6c0-1.71,1.39-3.1,3.1-3.1s3.1,1.39,3.1,3.1v2 H8.9V6z",
                    }
                    path {
                        d: "M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M18,15c0.83,0,1.5,0.67,1.5,1.5S18.83,18,18,18 s-1.5-0.67-1.5-1.5S17.17,15,18,15z M18,21c-1.03,0-1.94-0.52-2.48-1.32C16.25,19.26,17.09,19,18,19s1.75,0.26,2.48,0.68 C19.94,20.48,19.03,21,18,21z",
                    }
                }
            }
        }
    }
}

pub fn lock_reset_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17,10c0,3.87-3.13,7-7,7c-2.04,0-3.87-0.88-5.15-2.27l1.06-1.06C6.92,14.79,8.38,15.5,10,15.5c3.03,0,5.5-2.47,5.5-5.5 S13.03,4.5,10,4.5S4.5,6.97,4.5,10h2.25l-3,3l-3-3H3c0-3.87,3.13-7,7-7C13.87,3,17,6.13,17,10z M12.25,10v2.25 c0,0.41-0.34,0.75-0.75,0.75h-3c-0.41,0-0.75-0.34-0.75-0.75V10c0-0.41,0.34-0.75,0.75-0.75V8.5C8.5,7.67,9.17,7,10,7 s1.5,0.67,1.5,1.5v0.75C11.91,9.25,12.25,9.59,12.25,10z M10.75,8.5c0-0.41-0.34-0.75-0.75-0.75S9.25,8.09,9.25,8.5v0.75h1.5V8.5z",
            }
        }
    }
}

pub fn lock_reset_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13,3c-4.97,0-9,4.03-9,9H1l4,4l4-4H6c0-3.86,3.14-7,7-7s7,3.14,7,7s-3.14,7-7,7c-1.9,0-3.62-0.76-4.88-1.99L6.7,18.42 C8.32,20.01,10.55,21,13,21c4.97,0,9-4.03,9-9S17.97,3,13,3z M15,11v-1c0-1.1-0.9-2-2-2s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3 c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3C16,11.45,15.55,11,15,11z M14,11h-2v-1c0-0.55,0.45-1,1-1s1,0.45,1,1V11z",
            }
        }
    }
}

pub fn login_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,4v1h6v10h-6v1h6c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1H10z",
                }
                polygon {
                    points: "9.5,6.5 8.79,7.21 11.09,9.5 3,9.5 3,10.5 11.09,10.5 8.79,12.79 9.5,13.5 13,10",
                }
            }
        }
    }
}

pub fn login_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M11,7L9.6,8.4l2.6,2.6H2v2h10.2l-2.6,2.6L11,17l5-5L11,7z M20,19h-8v2h8c1.1,0,2-0.9,2-2V5c0-1.1-0.9-2-2-2h-8v2h8V19z",
                }
            }
        }
    }
}

pub fn logout_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17 7l-1.41 1.41L18.17 11H8v2h10.17l-2.58 2.58L17 17l5-5zM4 5h8V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8v-2H4V5z",
            }
        }
    }
}

pub fn loyalty_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21.41 11.58l-9-9C12.05 2.22 11.55 2 11 2H4c-1.1 0-2 .9-2 2v7c0 .55.22 1.05.59 1.42l9 9c.36.36.86.58 1.41.58.55 0 1.05-.22 1.41-.59l7-7c.37-.36.59-.86.59-1.41 0-.55-.23-1.06-.59-1.42zM5.5 7C4.67 7 4 6.33 4 5.5S4.67 4 5.5 4 7 4.67 7 5.5 6.33 7 5.5 7zm11.77 8.27L13 19.54l-4.27-4.27C8.28 14.81 8 14.19 8 13.5c0-1.38 1.12-2.5 2.5-2.5.69 0 1.32.28 1.77.74l.73.72.73-.73c.45-.45 1.08-.73 1.77-.73 1.38 0 2.5 1.12 2.5 2.5 0 .69-.28 1.32-.73 1.77z",
            }
        }
    }
}

pub fn manage_accounts_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    circle {
                        cy: "8",
                        r: "4",
                        cx: "10",
                    }
                    path {
                        d: "M10.67,13.02C10.45,13.01,10.23,13,10,13c-2.42,0-4.68,0.67-6.61,1.82C2.51,15.34,2,16.32,2,17.35V20h9.26 C10.47,18.87,10,17.49,10,16C10,14.93,10.25,13.93,10.67,13.02z",
                    }
                    path {
                        d: "M20.75,16c0-0.22-0.03-0.42-0.06-0.63l1.14-1.01l-1-1.73l-1.45,0.49c-0.32-0.27-0.68-0.48-1.08-0.63L18,11h-2l-0.3,1.49 c-0.4,0.15-0.76,0.36-1.08,0.63l-1.45-0.49l-1,1.73l1.14,1.01c-0.03,0.21-0.06,0.41-0.06,0.63s0.03,0.42,0.06,0.63l-1.14,1.01 l1,1.73l1.45-0.49c0.32,0.27,0.68,0.48,1.08,0.63L16,21h2l0.3-1.49c0.4-0.15,0.76-0.36,1.08-0.63l1.45,0.49l1-1.73l-1.14-1.01 C20.72,16.42,20.75,16.22,20.75,16z M17,18c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2S18.1,18,17,18z",
                    }
                }
            }
        }
    }
}

pub fn manage_history_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M18.95,14.89l0.85-0.79l-0.67-1.15l-1.12,0.33c-0.31-0.28-0.67-0.48-1.07-0.62l-0.28-1.16h-1.33l-0.27,1.17 c-0.39,0.13-0.75,0.34-1.05,0.61l-1.14-0.34L12.2,14.1l0.87,0.81c-0.11,0.54-0.05,0.97,0,1.21l-0.87,0.83l0.67,1.15l1.16-0.36 c0.3,0.26,0.64,0.46,1.03,0.59l0.27,1.17h1.33l0.28-1.16c0.39-0.13,0.75-0.33,1.05-0.6l1.15,0.35l0.67-1.15l-0.86-0.81 C19.07,15.53,18.98,15.07,18.95,14.89z M16.01,17c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S16.84,17,16.01,17z M10.49,16.98C10.33,16.99,10.17,17,10,17c-3.87,0-7-3.13-7-7h1.5c0,2.91,2.27,5.3,5.13,5.49L10.49,16.98z M15.5,10 c0-3.03-2.47-5.5-5.5-5.5c-1.7,0-3.22,0.78-4.22,2H8V8H3V3h1.5v2.71C5.78,4.07,7.76,3,10,3c3.87,0,7,3.13,7,7H15.5z M10.75,6v3.38 l1.55,1.55l-0.78,1.34L9.25,10V6H10.75z",
                }
            }
        }
    }
}

pub fn manage_history_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M22.69,18.37l1.14-1l-1-1.73l-1.45,0.49c-0.32-0.27-0.68-0.48-1.08-0.63L20,14h-2l-0.3,1.49c-0.4,0.15-0.76,0.36-1.08,0.63 l-1.45-0.49l-1,1.73l1.14,1c-0.08,0.5-0.08,0.76,0,1.26l-1.14,1l1,1.73l1.45-0.49c0.32,0.27,0.68,0.48,1.08,0.63L18,24h2l0.3-1.49 c0.4-0.15,0.76-0.36,1.08-0.63l1.45,0.49l1-1.73l-1.14-1C22.77,19.13,22.77,18.87,22.69,18.37z M19,21c-1.1,0-2-0.9-2-2s0.9-2,2-2 s2,0.9,2,2S20.1,21,19,21z M11,7v5.41l2.36,2.36l1.04-1.79L13,11.59V7H11z M21,12c0-4.97-4.03-9-9-9C9.17,3,6.65,4.32,5,6.36V4H3v6 h6V8H6.26C7.53,6.19,9.63,5,12,5c3.86,0,7,3.14,7,7H21z M10.86,18.91C7.87,18.42,5.51,16.01,5.08,13H3.06c0.5,4.5,4.31,8,8.94,8 c0.02,0,0.05,0,0.07,0L10.86,18.91z",
                }
            }
        }
    }
}

pub fn markunread_mailbox_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M-618-3000H782V600H-618zM0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M20 6H10v6H8V4h6V0H6v6H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn mark_as_unread_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                fill: "none",
                d: "M0 0h20v20H0V0z",
            }
            path {
                d: "M17.5 6h-12C4.67 6 4 6.67 4 7.5v9c0 .83.67 1.5 1.5 1.5h12c.83 0 1.5-.67 1.5-1.5v-9c0-.83-.67-1.5-1.5-1.5zM17 9.67L11.5 13 6 9.67V8l5.5 3.33L17 8v1.67zM17 5h-3L9 3 3 5.4V15h-.5c-.83 0-1.5-.67-1.5-1.5V5.17c0-.53.3-1.09.76-1.34L9 1l7.24 2.83c.41.23.7.7.76 1.17z",
            }
        }
    }
}

pub fn mark_as_unread_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18.83 7h-2.6L10.5 4 4 7.4V17c-1.1 0-2-.9-2-2V7.17c0-.53.32-1.09.8-1.34L10.5 2l7.54 3.83c.43.23.73.7.79 1.17zM20 8H7c-1.1 0-2 .9-2 2v9c0 1.1.9 2 2 2h13c1.1 0 2-.9 2-2v-9c0-1.1-.9-2-2-2zm0 3.67L13.5 15 7 11.67V10l6.5 3.33L20 10v1.67z",
            }
        }
    }
}

pub fn maximize_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 3h18v2H3z",
            }
        }
    }
}

pub fn mediation_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M22 12l-4 4-1.41-1.41L18.17 13h-5.23c-.34 3.1-2.26 5.72-4.94 7.05C7.96 21.69 6.64 23 5 23c-1.66 0-3-1.34-3-3s1.34-3 3-3c.95 0 1.78.45 2.33 1.14 1.9-1.03 3.26-2.91 3.58-5.14h-3.1C7.4 14.16 6.3 15 5 15c-1.66 0-3-1.34-3-3s1.34-3 3-3c1.3 0 2.4.84 2.82 2h3.1c-.32-2.23-1.69-4.1-3.59-5.14C6.78 6.55 5.95 7 5 7 3.34 7 2 5.66 2 4s1.34-3 3-3c1.64 0 2.96 1.31 2.99 2.95 2.68 1.33 4.6 3.95 4.94 7.05h5.23l-1.58-1.59L18 8l4 4z",
            }
        }
    }
}

pub fn minimize_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M6 19h12v2H6z",
            }
        }
    }
}

pub fn model_training_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                }
                path {
                    d: "M9.25,16h1.5v-1h-1.5V16z M10,8L10,8L10,8C10,8,10,8,10,8c-1.38,0-2.5,1.12-2.5,2.5C7.5,12,9.25,13,9.25,14 c0.35,0,1.5,0,1.5,0c0-1,1.75-2,1.75-3.5C12.5,9.12,11.38,8,10,8z M6.47,13.53l-0.71,0.71C4.67,13.16,4,11.66,4,10 c0-3.31,2.69-6,6-6V2l2.5,2.5L10,7V5c-2.76,0-5,2.24-5,5C5,11.38,5.56,12.63,6.47,13.53z M14.24,14.24l-0.71-0.71 C14.44,12.63,15,11.38,15,10s-0.56-2.63-1.47-3.53l0.71-0.71C15.33,6.84,16,8.34,16,10S15.33,13.16,14.24,14.24z",
                }
            }
        }
    }
}

pub fn model_training_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.5,13.5c0,2-2.5,3.5-2.5,5h-2c0-1.5-2.5-3-2.5-5c0-1.93,1.57-3.5,3.5-3.5h0C13.93,10,15.5,11.57,15.5,13.5z M13,19.5h-2 V21h2V19.5z M19,13c0,1.68-0.59,3.21-1.58,4.42l1.42,1.42C20.18,17.27,21,15.23,21,13c0-2.74-1.23-5.19-3.16-6.84l-1.42,1.42 C17.99,8.86,19,10.82,19,13z M16,5l-4-4v3c0,0,0,0,0,0c-4.97,0-9,4.03-9,9c0,2.23,0.82,4.27,2.16,5.84l1.42-1.42 C5.59,16.21,5,14.68,5,13c0-3.86,3.14-7,7-7c0,0,0,0,0,0v3L16,5z",
                }
            }
        }
    }
}

pub fn network_ping_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M4,13l0,1.5h12V13h-5l4.18-4.18C15.43,8.94,15.71,9,16,9c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2 c0,0.26,0.05,0.51,0.14,0.74L10,11.88L3.06,4.94L2,6l7,7H4z",
                }
            }
        }
    }
}

pub fn network_ping_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,14.67L3.41,6.09L2,7.5l8.5,8.5H4v2h16v-2h-6.5l5.15-5.15C18.91,10.95,19.2,11,19.5,11c1.38,0,2.5-1.12,2.5-2.5 S20.88,6,19.5,6S17,7.12,17,8.5c0,0.35,0.07,0.67,0.2,0.97L12,14.67z",
                }
            }
        }
    }
}

pub fn new_label_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17,10l-3.55,4.44C13.17,14.79,12.73,15,12.28,15H10.5v-4.5h-2v-2H4v-2C4,5.67,4.67,5,5.5,5h6.78 c0.46,0,0.89,0.21,1.17,0.56L17,10z M9,12H7v-2H5.5v2h-2v1.5h2v2H7v-2h2V12z",
            }
        }
    }
}

pub fn new_label_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21,12l-4.37,6.16C16.26,18.68,15.65,19,15,19h-3l0-6H9v-3H3V7c0-1.1,0.9-2,2-2h10c0.65,0,1.26,0.31,1.63,0.84L21,12z M10,15H7v-3H5v3H2v2h3v3h2v-3h3V15z",
            }
        }
    }
}

pub fn next_plan_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C17,6.13,13.87,3,10,3z M14,11.5h-2.99l1.19-1.19 C11.59,9.81,10.84,9.5,10,9.5c-1.39,0-2.58,0.82-3.15,1.99l-0.96-0.32C6.59,9.6,8.17,8.5,10,8.5c1.12,0,2.13,0.41,2.91,1.09L14,8.5 V11.5z",
                }
            }
        }
    }
}

pub fn next_plan_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M18,13.97h-5l2.26-2.26 c-0.91-1.06-2.25-1.74-3.76-1.74c-2.37,0-4.35,1.66-4.86,3.88l-0.96-0.32c0.64-2.62,3-4.56,5.82-4.56c1.78,0,3.37,0.79,4.47,2.03 L18,8.97V13.97z",
                }
            }
        }
    }
}

pub fn nightlight_round_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12.01 12c0-3.57 2.2-6.62 5.31-7.87.89-.36.75-1.69-.19-1.9-1.1-.24-2.27-.3-3.48-.14-4.51.6-8.12 4.31-8.59 8.83C4.44 16.93 9.13 22 15.01 22c.73 0 1.43-.08 2.12-.23.95-.21 1.1-1.53.2-1.9-3.22-1.29-5.33-4.41-5.32-7.87z",
            }
        }
    }
}

pub fn noise_aware_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        r: "1.25",
                        cy: "10.38",
                        cx: "11.25",
                    }
                    path {
                        d: "M13,12.75h-1.5c0,0.41-0.34,0.75-0.75,0.75c-0.43,0-0.66-0.31-0.73-0.58c-0.22-0.9-0.77-1.7-1.69-2.44 c-0.7-0.56-0.98-1.43-0.76-2.33C7.71,7.6,8.34,6.5,9.75,6.5c0.98,0,1.8,0.63,2.11,1.5h1.56c-0.35-1.71-1.86-3-3.67-3 C7.81,5,6.47,6.37,6.12,7.8c-0.38,1.56,0.2,2.99,1.28,3.85c0.53,0.43,1.01,0.96,1.17,1.62C8.8,14.22,9.63,15,10.75,15 C11.99,15,13,13.99,13,12.75z",
                    }
                    path {
                        d: "M3.88,7.83L2.41,7.5C2.25,7.97,2.14,8.46,2.08,8.96L3.54,9.3C3.59,8.79,3.71,8.3,3.88,7.83z",
                    }
                    path {
                        d: "M7.88,3.86L7.22,2.51C6.75,2.68,6.3,2.9,5.87,3.16l0.65,1.36C6.95,4.25,7.4,4.03,7.88,3.86z",
                    }
                    path {
                        d: "M5.42,5.39L4.25,4.45C3.9,4.81,3.59,5.21,3.31,5.62l1.18,0.94C4.76,6.14,5.07,5.74,5.42,5.39z",
                    }
                    path {
                        d: "M10.75,3.55V2.04C10.5,2.01,10.25,2,10,2S9.5,2.01,9.25,2.04v1.51C9.5,3.52,9.75,3.5,10,3.5S10.5,3.52,10.75,3.55z",
                    }
                    path {
                        d: "M16.46,9.29l1.46-0.33c-0.07-0.5-0.18-0.99-0.33-1.46l-1.47,0.34C16.29,8.3,16.4,8.79,16.46,9.29z",
                    }
                    path {
                        d: "M14.13,3.16c-0.43-0.26-0.88-0.48-1.35-0.65l-0.65,1.36c0.48,0.17,0.93,0.39,1.35,0.65L14.13,3.16z",
                    }
                    path {
                        d: "M16.69,5.62c-0.28-0.42-0.59-0.81-0.94-1.17l-1.17,0.94c0.36,0.35,0.67,0.75,0.93,1.18L16.69,5.62z",
                    }
                    path {
                        d: "M16.46,10.7c-0.05,0.51-0.17,1-0.34,1.46l1.47,0.34c0.16-0.47,0.27-0.96,0.33-1.46L16.46,10.7z",
                    }
                    path {
                        d: "M12.12,16.14l0.65,1.36c0.47-0.18,0.92-0.39,1.35-0.65l-0.65-1.36C13.05,15.75,12.6,15.97,12.12,16.14z",
                    }
                    path {
                        d: "M9.25,16.45v1.51C9.5,17.99,9.75,18,10,18s0.5-0.01,0.75-0.04v-1.51c-0.25,0.03-0.5,0.05-0.75,0.05S9.5,16.48,9.25,16.45z",
                    }
                    path {
                        d: "M14.58,14.61l1.18,0.94c0.35-0.36,0.66-0.75,0.94-1.17l-1.18-0.94C15.24,13.86,14.93,14.26,14.58,14.61z",
                    }
                    path {
                        d: "M3.54,10.71l-1.46,0.33c0.07,0.5,0.18,0.99,0.33,1.46l1.47-0.34C3.71,11.7,3.6,11.21,3.54,10.71z",
                    }
                    path {
                        d: "M3.31,14.38c0.28,0.42,0.59,0.81,0.94,1.17l1.17-0.94c-0.36-0.35-0.67-0.75-0.93-1.18L3.31,14.38z",
                    }
                    path {
                        d: "M5.87,16.84c0.43,0.26,0.88,0.48,1.35,0.65l0.65-1.36c-0.48-0.17-0.93-0.39-1.35-0.65L5.87,16.84z",
                    }
                }
            }
        }
    }
}

pub fn noise_aware_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16,15h-2c0,0.55-0.45,1-1,1c-0.43,0-0.81-0.27-0.95-0.68c-0.15-0.44-0.4-1.08-0.93-1.61l-1.36-1.36 C9.28,11.87,9,11.19,9,10.5C9,9.12,10.12,8,11.5,8c1.21,0,2.22,0.86,2.45,2h2.02c-0.25-2.25-2.16-4-4.47-4C9.02,6,7,8.02,7,10.5 c0,1.22,0.49,2.41,1.35,3.27l1.36,1.36c0.17,0.17,0.31,0.44,0.44,0.82C10.56,17.17,11.71,18,13,18C14.65,18,16,16.65,16,15z",
                    }
                    circle {
                        cy: "12.5",
                        r: "1.5",
                        cx: "13.5",
                    }
                    path {
                        d: "M3.6,6.58l1.58,1.26c0.35-0.57,0.77-1.1,1.24-1.57L4.85,5.02C4.38,5.49,3.97,6.02,3.6,6.58z",
                    }
                    path {
                        d: "M9.46,4.42L8.59,2.61c-0.63,0.23-1.24,0.52-1.8,0.87l0.87,1.81C8.22,4.93,8.82,4.64,9.46,4.42z",
                    }
                    path {
                        d: "M4.49,9.26L2.53,8.81c-0.21,0.63-0.36,1.28-0.44,1.95l1.96,0.45C4.11,10.53,4.27,9.88,4.49,9.26z",
                    }
                    path {
                        d: "M20.4,6.58c-0.36-0.56-0.78-1.09-1.25-1.56l-1.58,1.26c0.48,0.47,0.89,0.99,1.24,1.57L20.4,6.58z",
                    }
                    path {
                        d: "M4.04,12.79l-1.96,0.45c0.08,0.67,0.23,1.33,0.44,1.95l1.97-0.45C4.27,14.12,4.11,13.47,4.04,12.79z",
                    }
                    path {
                        d: "M17.21,3.48c-0.57-0.35-1.17-0.64-1.8-0.87l-0.87,1.81c0.64,0.22,1.24,0.51,1.8,0.87L17.21,3.48z",
                    }
                    path {
                        d: "M13,4.07V2.05C12.67,2.02,12.34,2,12,2s-0.67,0.02-1,0.05v2.02C11.33,4.03,11.66,4,12,4S12.67,4.03,13,4.07z",
                    }
                    path {
                        d: "M11,19.93v2.02c0.33,0.03,0.66,0.05,1,0.05s0.67-0.02,1-0.05v-2.02C12.67,19.97,12.34,20,12,20S11.33,19.97,11,19.93z",
                    }
                    path {
                        d: "M19.51,14.74l1.97,0.45c0.21-0.63,0.36-1.28,0.44-1.95l-1.96-0.45C19.89,13.47,19.73,14.12,19.51,14.74z",
                    }
                    path {
                        d: "M19.96,11.21l1.96-0.45c-0.08-0.67-0.23-1.33-0.44-1.95l-1.97,0.45C19.73,9.88,19.89,10.53,19.96,11.21z",
                    }
                    path {
                        d: "M17.58,17.73l1.58,1.26c0.47-0.48,0.88-1,1.25-1.56l-1.58-1.26C18.47,16.73,18.05,17.26,17.58,17.73z",
                    }
                    path {
                        d: "M6.79,20.52c0.57,0.35,1.17,0.64,1.8,0.87l0.87-1.81c-0.64-0.22-1.24-0.51-1.8-0.87L6.79,20.52z",
                    }
                    path {
                        d: "M14.54,19.58l0.87,1.81c0.63-0.23,1.24-0.52,1.8-0.87l-0.87-1.81C15.78,19.07,15.18,19.36,14.54,19.58z",
                    }
                    path {
                        d: "M3.6,17.42c0.36,0.56,0.78,1.09,1.25,1.56l1.58-1.26c-0.48-0.47-0.89-0.99-1.24-1.57L3.6,17.42z",
                    }
                }
            }
        }
    }
}

pub fn noise_control_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cx: "11.25",
                        cy: "10.38",
                        r: "1.25",
                    }
                    path {
                        d: "M3.39,5.51C2.51,6.79,2,8.34,2,10s0.51,3.21,1.39,4.49l1.09-1.09C3.86,12.41,3.5,11.25,3.5,10s0.36-2.41,0.97-3.41 L3.39,5.51z",
                    }
                    path {
                        d: "M16.61,5.51l-1.09,1.09C16.14,7.59,16.5,8.75,16.5,10s-0.36,2.41-0.97,3.41l1.09,1.09C17.49,13.21,18,11.66,18,10 S17.49,6.79,16.61,5.51z",
                    }
                    path {
                        d: "M10,16.5c-1.25,0-2.41-0.36-3.41-0.97l-1.09,1.09C6.79,17.49,8.34,18,10,18s3.21-0.51,4.49-1.39l-1.09-1.09 C12.41,16.14,11.25,16.5,10,16.5z",
                    }
                    path {
                        d: "M5.51,3.39l1.09,1.09C7.59,3.86,8.75,3.5,10,3.5s2.41,0.36,3.41,0.97l1.09-1.09C13.21,2.51,11.66,2,10,2 S6.79,2.51,5.51,3.39z",
                    }
                    path {
                        d: "M9.75,5C7.81,5,6.47,6.37,6.12,7.8c-0.38,1.56,0.2,2.99,1.28,3.85c0.53,0.43,1.01,0.96,1.17,1.62 C8.8,14.22,9.63,15,10.75,15c1.24,0,2.25-1.01,2.25-2.25h-1.5c0,0.41-0.34,0.75-0.75,0.75c-0.43,0-0.66-0.31-0.73-0.58 c-0.22-0.9-0.77-1.7-1.69-2.44c-0.7-0.56-0.98-1.43-0.76-2.33C7.71,7.6,8.34,6.5,9.75,6.5c0.98,0,1.8,0.63,2.11,1.5h1.56 C13.08,6.29,11.56,5,9.75,5z",
                    }
                }
            }
        }
    }
}

pub fn noise_control_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,4c1.44,0,2.79,0.38,3.95,1.05l1.45-1.45C15.85,2.59,13.99,2,12,2S8.15,2.59,6.59,3.59l1.45,1.45 C9.21,4.38,10.56,4,12,4z",
                    }
                    path {
                        d: "M20,12c0,1.44-0.38,2.79-1.05,3.95l1.45,1.45C21.41,15.85,22,13.99,22,12s-0.59-3.85-1.59-5.41l-1.45,1.45 C19.62,9.21,20,10.56,20,12z",
                    }
                    path {
                        d: "M12,20c-1.44,0-2.79-0.38-3.95-1.05l-1.45,1.45C8.15,21.41,10.01,22,12,22s3.85-0.59,5.41-1.59l-1.45-1.45 C14.79,19.62,13.44,20,12,20z",
                    }
                    path {
                        d: "M4,12c0-1.44,0.38-2.79,1.05-3.95L3.59,6.59C2.59,8.15,2,10.01,2,12s0.59,3.85,1.59,5.41l1.45-1.45 C4.38,14.79,4,13.44,4,12z",
                    }
                    path {
                        d: "M11.5,6C9.02,6,7,8.02,7,10.5c0,1.22,0.49,2.41,1.35,3.27l1.36,1.36c0.17,0.17,0.31,0.44,0.44,0.82 C10.56,17.17,11.71,18,13,18c1.65,0,3-1.35,3-3h-2c0,0.55-0.45,1-1,1c-0.43,0-0.81-0.27-0.95-0.68c-0.15-0.44-0.4-1.08-0.93-1.61 l-1.36-1.36C9.28,11.87,9,11.19,9,10.5C9,9.12,10.12,8,11.5,8c1.21,0,2.22,0.86,2.45,2h2.02C15.72,7.75,13.81,6,11.5,6z",
                    }
                    circle {
                        cy: "12.5",
                        r: "1.5",
                        cx: "13.5",
                    }
                }
            }
        }
    }
}

pub fn note_add_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 14h-3v3h-2v-3H8v-2h3v-3h2v3h3v2zm-3-7V3.5L18.5 9H13z",
            }
        }
    }
}

pub fn not_accessible_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.95,14h1c-0.24,1.69-1.69,3-3.45,3C6.57,17,5,15.43,5,13.5c0-1.76,1.31-3.21,3-3.45v1c-1.14,0.23-2,1.24-2,2.45 C6,14.88,7.12,16,8.5,16C9.71,16,10.72,15.14,10.95,14z M8.5,4.5C8.5,5.33,9.17,6,10,6s1.5-0.67,1.5-1.5S10.83,3,10,3 S8.5,3.67,8.5,4.5z M11,9.59V8.96l0.47,0.43c0.17,0.16,0.35,0.3,0.53,0.43c0.77,0.57,1.68,0.89,2.66,0.89v-1 c-0.94,0-1.81-0.36-2.49-1.04L12,8.52l-1.15-1.05c0,0-0.49-0.47-0.85-0.47C9.57,7,9.21,7.28,9.07,7.66L11,9.59z M16.36,16.36 L3.64,3.64L2.93,4.34L9,10.41V12c0,0.55,0.45,1,1,1h1.59l4.07,4.07L16.36,16.36z",
                }
            }
        }
    }
}

pub fn not_accessible_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M14,11.05l-3.42-3.42c0.32-0.34,0.74-0.57,1.23-0.61c0.48-0.04,0.84,0.07,1.2,0.26c0.19,0.1,0.39,0.22,0.63,0.46l1.29,1.43 c0.98,1.08,2.53,1.85,4.07,1.83v2C17.25,12.99,15.29,12.12,14,11.05z M12,6c1.1,0,2-0.9,2-2s-0.9-2-2-2c-1.1,0-2,0.9-2,2 S10.9,6,12,6z M2.81,2.81L1.39,4.22L10,12.83V15c0,1.1,0.9,2,2,2h2.17l5.61,5.61l1.41-1.41L2.81,2.81z M10,20c-1.66,0-3-1.34-3-3 c0-1.31,0.84-2.41,2-2.83V12.1c-2.28,0.46-4,2.48-4,4.9c0,2.76,2.24,5,5,5c2.42,0,4.44-1.72,4.9-4h-2.07 C12.42,19.16,11.31,20,10,20z",
                }
            }
        }
    }
}

pub fn not_started_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                x: "0",
                y: "0",
            }
            path {
                d: "M10,3c-3.86,0-7,3.14-7,7s3.14,7,7,7s7-3.14,7-7S13.86,3,10,3z M9,13H8V7h1V13z M10,13V7l4,3L10,13z",
            }
        }
    }
}

pub fn not_started_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M11,16H9V8h2V16z M12,16V8l5,4L12,16z",
                }
            }
        }
    }
}

pub fn no_accounts_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.18,10.94c0.2-0.44,0.32-0.92,0.32-1.44C15.5,7.57,13.93,6,12,6c-0.52,0-1,0.12-1.44,0.32L15.18,10.94z",
                    }
                    path {
                        d: "M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,15c-2.32,0-4.45,0.8-6.14,2.12 C4.7,15.73,4,13.95,4,12c0-1.85,0.63-3.55,1.69-4.9l2.86,2.86c0.21,1.56,1.43,2.79,2.99,2.99l2.2,2.2C13.17,15.05,12.59,15,12,15z M18.31,16.9L7.1,5.69C8.45,4.63,10.15,4,12,4c4.42,0,8,3.58,8,8C20,13.85,19.37,15.54,18.31,16.9z",
                    }
                }
            }
        }
    }
}

pub fn offline_bolt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2.02c-5.51 0-9.98 4.47-9.98 9.98s4.47 9.98 9.98 9.98 9.98-4.47 9.98-9.98S17.51 2.02 12 2.02zM11.48 20v-6.26H8L13 4v6.26h3.35L11.48 20z",
            }
        }
    }
}

pub fn offline_pin_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        path {
                            d: "M12,2C6.5,2,2,6.5,2,12s4.5,10,10,10s10-4.5,10-10S17.5,2,12,2z M17,18H7v-2h10V18z M10.3,14L7,10.7l1.4-1.4l1.9,1.9 l5.3-5.3L17,7.3L10.3,14z",
                        }
                    }
                }
            }
        }
    }
}

pub fn online_prediction_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    y: "0",
                    height: "20",
                    fill: "none",
                    x: "0",
                }
                path {
                    d: "M9.25,14h1.5v1h-1.5V14z M10,7L10,7L10,7C10,7,10,7,10,7C8.62,7,7.5,8.12,7.5,9.5C7.5,11,9.25,12,9.25,13 c0.35,0,1.5,0,1.5,0c0-1,1.75-2,1.75-3.5C12.5,8.12,11.38,7,10,7z M4,10c0-1.65,0.67-3.15,1.76-4.24L5.05,5.05 C3.78,6.32,3,8.07,3,10c0,1.93,0.78,3.68,2.05,4.95l0.71-0.71C4.67,13.15,4,11.65,4,10z M17,10c0-1.93-0.78-3.68-2.05-4.95 l-0.71,0.71C15.33,6.85,16,8.35,16,10c0,1.65-0.67,3.15-1.76,4.24l0.71,0.71C16.22,13.68,17,11.93,17,10z M6.46,13.54l0.71-0.71 C6.45,12.1,6,11.1,6,10c0-1.1,0.45-2.1,1.17-2.83L6.46,6.46C5.56,7.37,5,8.62,5,10C5,11.38,5.56,12.63,6.46,13.54z M14,10 c0,1.1-0.45,2.1-1.17,2.83l0.71,0.71C14.44,12.63,15,11.38,15,10c0-1.38-0.56-2.63-1.46-3.54l-0.71,0.71C13.55,7.9,14,8.9,14,10z",
                }
            }
        }
    }
}

pub fn online_prediction_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.5,11.5c0,2-2.5,3.5-2.5,5h-2c0-1.5-2.5-3-2.5-5C8.5,9.57,10.07,8,12,8S15.5,9.57,15.5,11.5z M13,17.5h-2V19h2V17.5z M22,12c0-2.76-1.12-5.26-2.93-7.07l-1.06,1.06C19.55,7.53,20.5,9.66,20.5,12c0,2.34-0.95,4.47-2.49,6.01l1.06,1.06 C20.88,17.26,22,14.76,22,12z M3.5,12c0-2.34,0.95-4.47,2.49-6.01L4.93,4.93C3.12,6.74,2,9.24,2,12c0,2.76,1.12,5.26,2.93,7.07 l1.06-1.06C4.45,16.47,3.5,14.34,3.5,12z M17.5,12c0,1.52-0.62,2.89-1.61,3.89l1.06,1.06C18.22,15.68,19,13.93,19,12 c0-1.93-0.78-3.68-2.05-4.95l-1.06,1.06C16.88,9.11,17.5,10.48,17.5,12z M7.05,16.95l1.06-1.06c-1-1-1.61-2.37-1.61-3.89 c0-1.52,0.62-2.89,1.61-3.89L7.05,7.05C5.78,8.32,5,10.07,5,12C5,13.93,5.78,15.68,7.05,16.95z",
                }
            }
        }
    }
}

pub fn on_device_training_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14.5,1h-9C4.67,1,4,1.67,4,2.5v15C4,18.33,4.67,19,5.5,19h9c0.83,0,1.5-0.67,1.5-1.5v-15C16,1.67,15.33,1,14.5,1z M14.5,15h-9V5h9V15z",
                    }
                    rect {
                        width: "1.5",
                        x: "9.25",
                        height: ".75",
                        y: "13.25",
                    }
                    path {
                        d: "M10,9.5c-0.83,0-1.5,0.67-1.5,1.5c0,0.55,0.3,1.03,0.75,1.29v0.46h1.5v-0.46c0.45-0.26,0.75-0.74,0.75-1.29 C11.5,10.17,10.83,9.5,10,9.5z",
                    }
                    path {
                        d: "M12.97,13c0.48-0.63,0.78-1.41,0.78-2.26c0-0.85-0.3-1.63-0.78-2.26l-0.71,0.71c0.31,0.44,0.49,0.98,0.49,1.56 s-0.18,1.11-0.49,1.56L12.97,13z",
                    }
                    path {
                        d: "M7.74,12.31c-0.31-0.44-0.49-0.98-0.49-1.56C7.25,9.23,8.48,8,10,8v1l1.5-1.5L10,6v1c-2.07,0-3.75,1.68-3.75,3.75 c0,0.85,0.3,1.63,0.78,2.26L7.74,12.31z",
                    }
                }
            }
        }
    }
}

pub fn on_device_training_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        height: "1",
                        y: "16",
                        width: "2",
                        x: "11",
                    }
                    path {
                        d: "M12,11c-1.1,0-2,0.9-2,2c0,0.74,0.4,1.38,1,1.72v0.78h2v-0.78c0.6-0.35,1-0.98,1-1.72C14,11.9,13.1,11,12,11z",
                    }
                    path {
                        d: "M18,1.01L6,1C4.9,1,4,1.9,4,3v18c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V3C20,1.9,19.1,1.01,18,1.01z M18,18H6V6h12V18z",
                    }
                    path {
                        d: "M16.01,15.95C16.63,15.12,17,14.11,17,13s-0.37-2.12-0.99-2.95l-1.07,1.07c0.35,0.54,0.56,1.19,0.56,1.88 s-0.21,1.34-0.56,1.88L16.01,15.95z",
                    }
                    path {
                        d: "M9.06,14.88C8.71,14.34,8.5,13.69,8.5,13c0-1.93,1.57-3.5,3.5-3.5v1.25l2.25-2l-2.25-2V8c-2.76,0-5,2.24-5,5 c0,1.11,0.37,2.12,0.99,2.95L9.06,14.88z",
                    }
                }
            }
        }
    }
}

pub fn opacity_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M24 0H0v24h24V0zm0 0H0v24h24V0zM0 24h24V0H0v24z",
            }
            path {
                d: "M17.66 8L12 2.35 6.34 8C4.78 9.56 4 11.64 4 13.64s.78 4.11 2.34 5.67 3.61 2.35 5.66 2.35 4.1-.79 5.66-2.35S20 15.64 20 13.64 19.22 9.56 17.66 8zM6 14c.01-2 .62-3.27 1.76-4.4L12 5.27l4.24 4.38C17.38 10.77 17.99 12 18 14H6z",
            }
        }
    }
}

pub fn open_in_browser_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 4H5c-1.11 0-2 .9-2 2v12c0 1.1.89 2 2 2h4v-2H5V8h14v10h-4v2h4c1.1 0 2-.9 2-2V6c0-1.1-.89-2-2-2zm-7 6l-4 4h3v6h2v-6h3l-4-4z",
            }
        }
    }
}

pub fn open_in_full_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
            polygon {
                points: "21,11 21,3 13,3 16.29,6.29 6.29,16.29 3,13 3,21 11,21 7.71,17.71 17.71,7.71",
            }
        }
    }
}

pub fn open_in_new_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 19H5V5h7V3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2v-7h-2v7zM14 3v2h3.59l-9.83 9.83 1.41 1.41L19 6.41V10h2V3h-7z",
            }
        }
    }
}

pub fn open_in_new_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17,14.88l-1.5-1.5V10H17V14.88z M10,4.5V3H5.12l1.5,1.5H10z M15.03,6.03L17,8V3h-5l1.97,1.97l-3.44,3.44l1.06,1.06 L15.03,6.03z M16.01,18.13L14.88,17H4.5C3.67,17,3,16.33,3,15.5V5.12L1.87,3.99l1.06-1.06l14.14,14.14L16.01,18.13z M13.38,15.5 l-3.91-3.91L8.06,13L7,11.94l1.41-1.41L4.5,6.62v8.88H13.38z",
            }
        }
    }
}

pub fn open_in_new_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M16.79,5.8L14,3h7v7l-2.79-2.8l-4.09,4.09l-1.41-1.41L16.79,5.8z M19,12v4.17l2,2V12H19z M19.78,22.61L18.17,21H5 c-1.11,0-2-0.9-2-2V5.83L1.39,4.22l1.41-1.41l18.38,18.38L19.78,22.61z M16.17,19l-4.88-4.88L9.7,15.71L8.29,14.3l1.59-1.59L5,7.83 V19H16.17z M7.83,5H12V3H5.83L7.83,5z",
            }
        }
    }
}

pub fn open_with_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M10 9h4V6h3l-5-5-5 5h3v3zm-1 1H6V7l-5 5 5 5v-3h3v-4zm14 2l-5-5v3h-3v4h3v3l5-5zm-9 3h-4v3H7l5 5 5-5h-3v-3z",
            }
        }
    }
}

pub fn outbond_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                x: "0",
                fill: "none",
                height: "24",
                width: "24",
                y: "0",
            }
            path {
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M13.88,11.54l-4.96,4.96l-1.41-1.41 l4.96-4.96L10.34,8l5.65,0.01L16,13.66L13.88,11.54z",
            }
        }
    }
}

pub fn outbound_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                x: "0",
                y: "0",
                fill: "none",
                width: "24",
            }
            path {
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M13.88,11.54l-4.96,4.96l-1.41-1.41 l4.96-4.96L10.34,8l5.65,0.01L16,13.66L13.88,11.54z",
            }
        }
    }
}

pub fn outbox_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h20v20H0V0z",
                fill: "none",
            }
            path {
                d: "M7.5 13c0 1.38 1.12 2.5 2.5 2.5s2.5-1.12 2.5-2.5H16V4H4v9h3.5zM4 2h12c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2zm2 7h2v3h4V9h2l-4-4-4 4z",
            }
        }
    }
}

pub fn outbox_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3H4.99c-1.11 0-1.98.9-1.98 2L3 19c0 1.1.88 2 1.99 2H19c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 12h-4c0 1.66-1.35 3-3 3s-3-1.34-3-3H4.99V5H19v10zM8 11h2v3h4v-3h2l-4-4-4 4z",
            }
        }
    }
}

pub fn outgoing_mail_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15,10.5c0.73,0,1.41,0.21,2,0.55V5c0-0.55-0.45-1-1-1H4C3.45,4,3,4.45,3,5v10c0,0.55,0.45,1,1,1h7.3 c-0.19-0.46-0.3-0.97-0.3-1.5C11,12.29,12.79,10.5,15,10.5z M4,6.18V5l6,3.75L16,5v1.18l-6,3.75L4,6.18z",
                    }
                    polygon {
                        points: "15.17,11.67 14.46,12.38 16.09,14 12,14 12,15 16.09,15 14.46,16.62 15.17,17.33 18,14.5",
                    }
                }
            }
        }
    }
}

pub fn outgoing_mail_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18.5,11c0.17,0,0.34,0.01,0.5,0.03V6.87C19,5.84,18.16,5,17.13,5H3.87C2.84,5,2,5.84,2,6.87v10.26 C2,18.16,2.84,19,3.87,19h9.73C13.22,18.25,13,17.4,13,16.5C13,13.46,15.46,11,18.5,11z M10.4,13L4,9.19V7h0.23l6.18,3.68L16.74,7 H17v2.16L10.4,13z",
                    }
                    polygon {
                        points: "19,13 17.59,14.41 19.17,16 15,16 15,18 19.17,18 17.59,19.59 19,21 23,17",
                    }
                }
            }
        }
    }
}

pub fn outlet_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M9,12c-0.55,0-1-0.45-1-1V8 c0-0.55,0.45-1,1-1s1,0.45,1,1v3C10,11.55,9.55,12,9,12z M14,18h-4v-2c0-1.1,0.9-2,2-2c1.1,0,2,0.9,2,2V18z M16,11 c0,0.55-0.45,1-1,1c-0.55,0-1-0.45-1-1V8c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1V11z",
            }
        }
    }
}

pub fn output_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "14,14 18,10 14,6 12.94,7.06 15.13,9.25 8,9.25 8,10.75 15.13,10.75 12.94,12.94",
                    }
                    path {
                        d: "M15.5,15.5h-11v-11h11V6H17V4.5C17,3.67,16.33,3,15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11 c0.83,0,1.5-0.67,1.5-1.5V14h-1.5V15.5z",
                    }
                }
            }
        }
    }
}

pub fn output_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "17,17 22,12 17,7 15.59,8.41 18.17,11 9,11 9,13 18.17,13 15.59,15.59",
                    }
                    path {
                        d: "M19,19H5V5h14v2h2V5c0-1.1-0.89-2-2-2H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.11,0,2-0.9,2-2v-2h-2V19z",
                    }
                }
            }
        }
    }
}

pub fn pageview_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11.5 9C10.12 9 9 10.12 9 11.5s1.12 2.5 2.5 2.5 2.5-1.12 2.5-2.5S12.88 9 11.5 9zM20 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm-3.21 14.21l-2.91-2.91c-.69.44-1.51.7-2.39.7C9.01 16 7 13.99 7 11.5S9.01 7 11.5 7 16 9.01 16 11.5c0 .88-.26 1.69-.7 2.39l2.91 2.9-1.42 1.42z",
            }
        }
    }
}

pub fn paid_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12.88,17.76V19h-1.75v-1.29 c-0.74-0.18-2.39-0.77-3.02-2.96l1.65-0.67c0.06,0.22,0.58,2.09,2.4,2.09c0.93,0,1.98-0.48,1.98-1.61c0-0.96-0.7-1.46-2.28-2.03 c-1.1-0.39-3.35-1.03-3.35-3.31c0-0.1,0.01-2.4,2.62-2.96V5h1.75v1.24c1.84,0.32,2.51,1.79,2.66,2.23l-1.58,0.67 c-0.11-0.35-0.59-1.34-1.9-1.34c-0.7,0-1.81,0.37-1.81,1.39c0,0.95,0.86,1.31,2.64,1.9c2.4,0.83,3.01,2.05,3.01,3.45 C15.9,17.17,13.4,17.67,12.88,17.76z",
                }
            }
        }
    }
}

pub fn pan_tool_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    g {
                        path {
                            d: "M23,5.5V20c0,2.2-1.8,4-4,4h-7.3c-1.08,0-2.1-0.43-2.85-1.19L1,14.83c0,0,1.26-1.23,1.3-1.25 c0.22-0.19,0.49-0.29,0.79-0.29c0.22,0,0.42,0.06,0.6,0.16C3.73,13.46,8,15.91,8,15.91V4c0-0.83,0.67-1.5,1.5-1.5S11,3.17,11,4v7 h1V1.5C12,0.67,12.67,0,13.5,0S15,0.67,15,1.5V11h1V2.5C16,1.67,16.67,1,17.5,1S19,1.67,19,2.5V11h1V5.5C20,4.67,20.67,4,21.5,4 S23,4.67,23,5.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn pan_tool_alt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12.66,9H11V4.25C11,3.56,10.44,3,9.75,3h0C9.06,3,8.5,3.56,8.5,4.25V13l-2.12-0.71c-0.54-0.18-1.13-0.04-1.54,0.36L4.5,13 l3.56,3.56C8.34,16.84,8.72,17,9.12,17h5.27c0.73,0,1.36-0.53,1.48-1.25l0.61-3.65c0.11-0.65-0.22-1.29-0.81-1.59L12.66,9z",
                }
            }
        }
    }
}

pub fn pan_tool_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19.98,14.82l-0.63,4.46C19.21,20.27,18.36,21,17.37,21h-6.16c-0.53,0-1.29-0.21-1.66-0.59L5,15.62l0.83-0.84 c0.24-0.24,0.58-0.35,0.92-0.28L10,15.24V4.5C10,3.67,10.67,3,11.5,3S13,3.67,13,4.5v6h0.91c0.31,0,0.62,0.07,0.89,0.21l4.09,2.04 C19.66,13.14,20.1,13.97,19.98,14.82z",
                }
            }
        }
    }
}

pub fn payment_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 4H4c-1.11 0-1.99.89-1.99 2L2 18c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6c0-1.11-.89-2-2-2zm0 14H4v-6h16v6zm0-10H4V6h16v2z",
            }
        }
    }
}

pub fn pending_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C17,6.13,13.87,3,10,3z M6.5,11c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C7.5,10.55,7.05,11,6.5,11z M10,11c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C11,10.55,10.55,11,10,11z M13.5,11c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C14.5,10.55,14.05,11,13.5,11z",
                }
            }
        }
    }
}

pub fn pending_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M7,13.5c-0.83,0-1.5-0.67-1.5-1.5 c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5C8.5,12.83,7.83,13.5,7,13.5z M12,13.5c-0.83,0-1.5-0.67-1.5-1.5 c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5C13.5,12.83,12.83,13.5,12,13.5z M17,13.5c-0.83,0-1.5-0.67-1.5-1.5 c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5C18.5,12.83,17.83,13.5,17,13.5z",
                }
            }
        }
    }
}

pub fn pending_actions_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.65,19.35l-2.15-2.15V14h1v2.79l1.85,1.85 L18.65,19.35z M18,3h-3.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H6C4.9,3,4,3.9,4,5v15c0,1.1,0.9,2,2,2h6.11 c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v3h8V5h2v5.08c0.71,0.1,1.38,0.31,2,0.6V5C20,3.9,19.1,3,18,3z M12,5c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z",
                }
            }
        }
    }
}

pub fn percent_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M6.5,4C5.12,4,4,5.12,4,6.5C4,7.88,5.12,9,6.5,9S9,7.88,9,6.5C9,5.12,7.88,4,6.5,4z M6.5,7.5c-0.55,0-1-0.45-1-1 c0-0.55,0.45-1,1-1s1,0.45,1,1C7.5,7.05,7.05,7.5,6.5,7.5z M13.5,11c-1.38,0-2.5,1.12-2.5,2.5c0,1.38,1.12,2.5,2.5,2.5 s2.5-1.12,2.5-2.5C16,12.12,14.88,11,13.5,11z M13.5,14.5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1 C14.5,14.05,14.05,14.5,13.5,14.5z M5.06,16L4,14.94L14.94,4L16,5.06L5.06,16z",
                    }
                }
            }
        }
    }
}

pub fn percent_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                                d: "M7.5,11C9.43,11,11,9.43,11,7.5S9.43,4,7.5,4S4,5.57,4,7.5S5.57,11,7.5,11z M7.5,6C8.33,6,9,6.67,9,7.5S8.33,9,7.5,9 S6,8.33,6,7.5S6.67,6,7.5,6z",
                            }
                        }
                    }
                    g {
                        rect {
                            y: "11",
                            x: "1.69",
                            transform: "matrix(0.7071 -0.7071 0.7071 0.7071 -4.9706 12)",
                            height: "2",
                            width: "20.63",
                        }
                    }
                    g {
                        g {
                            path {
                                d: "M16.5,13c-1.93,0-3.5,1.57-3.5,3.5s1.57,3.5,3.5,3.5s3.5-1.57,3.5-3.5S18.43,13,16.5,13z M16.5,18 c-0.83,0-1.5-0.67-1.5-1.5s0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5S17.33,18,16.5,18z",
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn perm_camera_mic_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 5h-3.17L15 3H9L7.17 5H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h7v-2.09c-2.83-.48-5-2.94-5-5.91h2c0 2.21 1.79 4 4 4s4-1.79 4-4h2c0 2.97-2.17 5.43-5 5.91V21h7c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm-6 8c0 1.1-.9 2-2 2s-2-.9-2-2V9c0-1.1.9-2 2-2s2 .9 2 2v4z",
            }
        }
    }
}

pub fn perm_contact_calendar_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 3c1.66 0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H6v-1c0-2 4-3.1 6-3.1s6 1.1 6 3.1v1z",
            }
        }
    }
}

pub fn perm_data_setting_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18.99 11.5c.34 0 .67.03 1 .07L20 0 0 20h11.56c-.04-.33-.07-.66-.07-1 0-4.14 3.36-7.5 7.5-7.5zm3.71 7.99c.02-.16.04-.32.04-.49 0-.17-.01-.33-.04-.49l1.06-.83c.09-.08.12-.21.06-.32l-1-1.73c-.06-.11-.19-.15-.31-.11l-1.24.5c-.26-.2-.54-.37-.85-.49l-.19-1.32c-.01-.12-.12-.21-.24-.21h-2c-.12 0-.23.09-.25.21l-.19 1.32c-.3.13-.59.29-.85.49l-1.24-.5c-.11-.04-.24 0-.31.11l-1 1.73c-.06.11-.04.24.06.32l1.06.83c-.02.16-.03.32-.03.49 0 .17.01.33.03.49l-1.06.83c-.09.08-.12.21-.06.32l1 1.73c.06.11.19.15.31.11l1.24-.5c.26.2.54.37.85.49l.19 1.32c.02.12.12.21.25.21h2c.12 0 .23-.09.25-.21l.19-1.32c.3-.13.59-.29.84-.49l1.25.5c.11.04.24 0 .31-.11l1-1.73c.06-.11.03-.24-.06-.32l-1.07-.83zm-3.71 1.01c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

pub fn perm_device_information_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M13 7h-2v2h2V7zm0 4h-2v6h2v-6zm4-9.99L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14z",
            }
        }
    }
}

pub fn perm_identity_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 5.9c1.16 0 2.1.94 2.1 2.1s-.94 2.1-2.1 2.1S9.9 9.16 9.9 8s.94-2.1 2.1-2.1m0 9c2.97 0 6.1 1.46 6.1 2.1v1.1H5.9V17c0-.64 3.13-2.1 6.1-2.1M12 4C9.79 4 8 5.79 8 8s1.79 4 4 4 4-1.79 4-4-1.79-4-4-4zm0 9c-2.67 0-8 1.34-8 4v3h16v-3c0-2.66-5.33-4-8-4z",
            }
        }
    }
}

pub fn perm_media_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M2 6H0v5h.01L0 20c0 1.1.9 2 2 2h18v-2H2V6zm20-2h-8l-2-2H6c-1.1 0-1.99.9-1.99 2L4 16c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zM7 15l4.5-6 3.5 4.51 2.5-3.01L21 15H7z",
            }
        }
    }
}

pub fn perm_phone_msg_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 15.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.58l2.2-2.21c.28-.27.36-.66.25-1.01C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM12 3v10l3-3h6V3h-9z",
            }
        }
    }
}

pub fn perm_scan_wifi_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 3C6.95 3 3.15 4.85 0 7.23L12 22 24 7.25C20.85 4.87 17.05 3 12 3zm1 13h-2v-6h2v6zm-2-8V6h2v2h-2z",
            }
        }
    }
}

pub fn pets_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
            circle {
                r: "2.5",
                cx: "4.5",
                cy: "9.5",
            }
            circle {
                cy: "5.5",
                cx: "9",
                r: "2.5",
            }
            circle {
                r: "2.5",
                cy: "5.5",
                cx: "15",
            }
            circle {
                r: "2.5",
                cx: "19.5",
                cy: "9.5",
            }
            path {
                d: "M17.34 14.86c-.87-1.02-1.6-1.89-2.48-2.91-.46-.54-1.05-1.08-1.75-1.32-.11-.04-.22-.07-.33-.09-.25-.04-.52-.04-.78-.04s-.53 0-.79.05c-.11.02-.22.05-.33.09-.7.24-1.28.78-1.75 1.32-.87 1.02-1.6 1.89-2.48 2.91-1.31 1.31-2.92 2.76-2.62 4.79.29 1.02 1.02 2.03 2.33 2.32.73.15 3.06-.44 5.54-.44h.18c2.48 0 4.81.58 5.54.44 1.31-.29 2.04-1.31 2.33-2.32.31-2.04-1.3-3.49-2.61-4.8z",
            }
        }
    }
}

pub fn php_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.5,8h1v4h-1v-1.5h-1V12h-1V8h1v1.5h1V8z M7,8.5V10c0,0.28-0.22,0.5-0.5,0.5H5V12H4V8h2.5C6.78,8,7,8.22,7,8.5z M6,9H5 v0.5h1V9z M16,8.5V10c0,0.28-0.22,0.5-0.5,0.5H14V12h-1V8h2.5C15.78,8,16,8.22,16,8.5z M15,9h-1v0.5h1V9z",
                }
            }
        }
    }
}

pub fn php_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13,9h1.5v6H13v-2.5h-2V15H9.5V9H11v2h2V9z M8,10.5v1C8,12.3,7.3,13,6.5,13h-2v2H3V9h3.5C7.3,9,8,9.7,8,10.5z M6.5,10.5h-2 v1h2V10.5z M21.5,10.5v1c0,0.8-0.7,1.5-1.5,1.5h-2v2h-1.5V9H20C20.8,9,21.5,9.7,21.5,10.5z M20,10.5h-2v1h2V10.5z",
                }
            }
        }
    }
}

pub fn picture_in_picture_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 7h-8v6h8V7zm2-4H3c-1.1 0-2 .9-2 2v14c0 1.1.9 1.98 2 1.98h18c1.1 0 2-.88 2-1.98V5c0-1.1-.9-2-2-2zm0 16.01H3V4.98h18v14.03z",
            }
        }
    }
}

pub fn picture_in_picture_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 11h-8v6h8v-6zm4 8V4.98C23 3.88 22.1 3 21 3H3c-1.1 0-2 .88-2 1.98V19c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2zm-2 .02H3V4.97h18v14.05z",
            }
        }
    }
}

pub fn pinch_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.66,11H14V6.25C14,5.56,13.44,5,12.75,5h0c-0.69,0-1.25,0.56-1.25,1.25V15L9.38,14.3c-0.54-0.18-1.13-0.04-1.54,0.36 L7.5,15l3.56,3.56c0.28,0.28,0.66,0.44,1.06,0.44h5.27c0.73,0,1.36-0.53,1.48-1.25l0.61-3.65c0.11-0.65-0.22-1.29-0.81-1.59 L15.66,11z M5,1v1h2.29L2,7.3V5H1v4h4V8H2.71L8,2.71V5h1V1H5z",
                }
            }
        }
    }
}

pub fn pinch_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6,2.5V1h5v5H9.5V3.56L3.56,9.5H6V11H1V6h1.5v2.44L8.44,2.5H6z M22.98,16.82l-0.63,4.46C22.21,22.27,21.36,23,20.37,23 h-6.16c-0.53,0-1.29-0.21-1.66-0.59L8,17.62l0.83-0.84c0.24-0.24,0.58-0.35,0.92-0.28L13,17.24V6.5C13,5.67,13.67,5,14.5,5 S16,5.67,16,6.5v6h0.91c0.31,0,0.62,0.07,0.89,0.21l4.09,2.04C22.66,15.14,23.1,15.97,22.98,16.82z",
                }
            }
        }
    }
}

pub fn pin_end_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M16.5,10V5.5h-13v9H12c0.01,0.02,0,1.5,0,1.5H3.5C2.67,16,2,15.33,2,14.5v-9C2,4.67,2.67,4,3.5,4h13C17.33,4,18,4.67,18,5.5 V10H16.5z M15.75,11.5c-1.24,0-2.25,1.01-2.25,2.25s1,2.25,2.25,2.25c1.24,0,2.25-1.01,2.25-2.25S16.99,11.5,15.75,11.5z M12.74,7 H8.5v4.24l1.59-1.59l2.12,2.12l1.06-1.06l-2.12-2.12L12.74,7z",
            }
        }
    }
}

pub fn pin_end_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20,12V6H4v12h10l0,2H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h16c1.1,0,2,0.9,2,2v6H20z M19,14c-1.66,0-3,1.34-3,3s1.34,3,3,3 c1.66,0,3-1.34,3-3S20.66,14,19,14z M14.66,8H9v5.66l2.12-2.12l2.83,2.83l1.41-1.41l-2.83-2.83L14.66,8z",
            }
        }
    }
}

pub fn pin_invoke_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M18,10v4.5c0,0.83-0.67,1.5-1.5,1.5h-13C2.67,16,2,15.33,2,14.5v-9C2,4.67,2.67,4,3.5,4H12c0,0,0.01,1.48,0,1.5H3.5v9h13V10 H18z M18,6.25C18,5.01,16.99,4,15.75,4C14.5,4,13.5,5.01,13.5,6.25s1.01,2.25,2.25,2.25S18,7.49,18,6.25z M9.35,10.09l-2.12,2.12 l1.06,1.06l2.12-2.12L12,12.74V8.5H7.76L9.35,10.09z",
            }
        }
    }
}

pub fn pin_invoke_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22,12v6c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h10l0,2H4v12h16v-6H22z M22,7c0-1.66-1.34-3-3-3 c-1.66,0-3,1.34-3,3s1.34,3,3,3C20.66,10,22,8.66,22,7z M11.47,12.12l-2.83,2.83l1.41,1.41l2.83-2.83L15,15.66V10H9.34L11.47,12.12z",
            }
        }
    }
}

pub fn plagiarism_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        circle {
                            r: "1.5",
                            cx: "9.5",
                            cy: "11.5",
                        }
                    }
                    g {
                        path {
                            d: "M12,3H5C4.45,3,4,3.45,4,4v12c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V7L12,3z M12.33,15.04l-1.45-1.45 c-0.97,0.64-2.29,0.54-3.15-0.32c-0.98-0.98-0.98-2.56,0-3.54s2.56-0.98,3.54,0c0.86,0.86,0.96,2.18,0.32,3.15l1.45,1.45 L12.33,15.04z M11,8V4l4,4H11z",
                        }
                    }
                }
            }
        }
    }
}

pub fn plagiarism_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M14,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.89,2,1.99,2H18c1.1,0,2-0.9,2-2V8L14,2z M15.04,19.45l-1.88-1.88 c-1.33,0.71-3.01,0.53-4.13-0.59c-1.37-1.37-1.37-3.58,0-4.95c1.37-1.37,3.58-1.37,4.95,0c1.12,1.12,1.31,2.8,0.59,4.13l1.88,1.88 L15.04,19.45z M13,9V3.5L18.5,9H13z",
                    }
                    circle {
                        cy: "14.5",
                        cx: "11.5",
                        r: "1.5",
                    }
                }
            }
        }
    }
}

pub fn play_for_work_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11 5v5.59H7.5l4.5 4.5 4.5-4.5H13V5h-2zm-5 9c0 3.31 2.69 6 6 6s6-2.69 6-6h-2c0 2.21-1.79 4-4 4s-4-1.79-4-4H6z",
            }
        }
    }
}

pub fn polymer_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 4h-4L7.11 16.63 4.5 12 9 4H5L.5 12 5 20h4l7.89-12.63L19.5 12 15 20h4l4.5-8z",
            }
        }
    }
}

pub fn power_settings_new_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M13 3h-2v10h2V3zm4.83 2.17l-1.42 1.42C17.99 7.86 19 9.81 19 12c0 3.87-3.13 7-7 7s-7-3.13-7-7c0-2.19 1.01-4.14 2.58-5.42L6.17 5.17C4.23 6.82 3 9.26 3 12c0 4.97 4.03 9 9 9s9-4.03 9-9c0-2.74-1.23-5.18-3.17-6.83z",
            }
        }
    }
}

pub fn pregnant_woman_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M9,4c0-1.11,0.89-2,2-2s2,0.89,2,2s-0.89,2-2,2S9,5.11,9,4z M16,13c-0.01-1.34-0.83-2.51-2-3c0-1.66-1.34-3-3-3 s-3,1.34-3,3v7h2v5h3v-5h3V13z",
                        }
                    }
                }
            }
        }
    }
}

pub fn preview_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,3H5C3.89,3,3,3.9,3,5v14c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.11,3,19,3z M19,19H5V7h14V19z M13.5,13 c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5S13.5,12.17,13.5,13z M12,9c-2.73,0-5.06,1.66-6,4 c0.94,2.34,3.27,4,6,4s5.06-1.66,6-4C17.06,10.66,14.73,9,12,9z M12,15.5c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5 c1.38,0,2.5,1.12,2.5,2.5C14.5,14.38,13.38,15.5,12,15.5z",
                }
            }
        }
    }
}

pub fn print_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 8H5c-1.66 0-3 1.34-3 3v6h4v4h12v-4h4v-6c0-1.66-1.34-3-3-3zm-3 11H8v-5h8v5zm3-7c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm-1-9H6v4h12V3z",
            }
        }
    }
}

pub fn privacy_tip_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    y: "0",
                }
                path {
                    d: "M10,2.5L4,5.2v4.15c0,3.63,2.56,7.33,6,8.15c3.44-0.82,6-4.52,6-8.15V5.2L10,2.5z M10.5,13h-1V9h1V13z M10.5,8h-1V7h1V8z",
                }
            }
        }
    }
}

pub fn privacy_tip_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,1L3,5v6c0,5.55,3.84,10.74,9,12c5.16-1.26,9-6.45,9-12V5L12,1L12,1z M11,7h2v2h-2V7z M11,11h2v6h-2V11z",
                }
            }
        }
    }
}

pub fn private_connectivity_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M15.45,9.25C15.08,6.57,12.78,4.5,10,4.5S4.92,6.57,4.55,9.25H2v1.5h2.55C4.92,13.43,7.22,15.5,10,15.5s5.08-2.07,5.45-4.75 H18v-1.5H15.45z M12.5,12c0,0.28-0.22,0.5-0.5,0.5H8c-0.28,0-0.5-0.22-0.5-0.5V9c0-0.28,0.22-0.5,0.5-0.5h0.5V8 c0-0.82,0.67-1.5,1.5-1.5s1.5,0.68,1.5,1.5v0.5H12c0.28,0,0.5,0.22,0.5,0.5V12z M10.75,8v0.5h-1.5V8c0-0.41,0.34-0.75,0.75-0.75 S10.75,7.59,10.75,8z M9.4,10.5c0-0.33,0.27-0.6,0.6-0.6s0.6,0.27,0.6,0.6c0,0.33-0.27,0.6-0.6,0.6S9.4,10.83,9.4,10.5z",
            }
        }
    }
}

pub fn private_connectivity_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.93,11c-0.49-3.39-3.4-6-6.93-6s-6.44,2.61-6.93,6H2v2h3.07c0.49,3.39,3.4,6,6.93,6s6.44-2.61,6.93-6H22v-2H18.93z M15,14.5c0,0.55-0.45,1-1,1h-4c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1v-1c0-1.21,1.08-2.18,2.34-1.97C13.32,7.69,14,8.61,14,9.61 v0.89c0.55,0,1,0.45,1,1V14.5z M12.75,13c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75 S12.75,12.59,12.75,13z M13,9.5v1h-2v-1c0-0.55,0.45-1,1-1S13,8.95,13,9.5z",
            }
        }
    }
}

pub fn production_quantity_limits_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13,10h-2V8h2V10z M13,6h-2V1h2V6z M7,18c-1.1,0-1.99,0.9-1.99,2S5.9,22,7,22s2-0.9,2-2S8.1,18,7,18z M17,18 c-1.1,0-1.99,0.9-1.99,2s0.89,2,1.99,2s2-0.9,2-2S18.1,18,17,18z M8.1,13h7.45c0.75,0,1.41-0.41,1.75-1.03L21,4.96L19.25,4l-3.7,7 H8.53L4.27,2H1v2h2l3.6,7.59l-1.35,2.44C4.52,15.37,5.48,17,7,17h12v-2H7L8.1,13z",
            }
        }
    }
}

pub fn published_with_changes_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17.66,9.53l-7.07,7.07l-4.24-4.24l1.41-1.41l2.83,2.83l5.66-5.66L17.66,9.53z M4,12c0-2.33,1.02-4.42,2.62-5.88L9,8.5v-6H3 l2.2,2.2C3.24,6.52,2,9.11,2,12c0,5.19,3.95,9.45,9,9.95v-2.02C7.06,19.44,4,16.07,4,12z M22,12c0-5.19-3.95-9.45-9-9.95v2.02 c3.94,0.49,7,3.86,7,7.93c0,2.33-1.02,4.42-2.62,5.88L15,15.5v6h6l-2.2-2.2C20.76,17.48,22,14.89,22,12z",
            }
        }
    }
}

pub fn query_builder_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
            path {
                d: "M12.5 7H11v6l5.25 3.15.75-1.23-4.5-2.67z",
            }
        }
    }
}

pub fn question_answer_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21 6h-2v9H6v2c0 .55.45 1 1 1h11l4 4V7c0-.55-.45-1-1-1zm-4 6V3c0-.55-.45-1-1-1H3c-.55 0-1 .45-1 1v14l4-4h10c.55 0 1-.45 1-1z",
            }
        }
    }
}

pub fn question_mark_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M9.29,10.58c0.6-1.07,1.73-1.7,2.39-2.65c0.7-0.99,0.31-2.85-1.67-2.85c-1.3,0-1.94,0.98-2.2,1.8l-2-0.84 C6.35,4.41,7.83,3,9.99,3c1.81,0,3.05,0.82,3.68,1.85c0.54,0.88,0.86,2.54,0.02,3.77c-0.92,1.36-1.81,1.78-2.28,2.65 c-0.19,0.35-0.27,0.58-0.27,1.72H8.91C8.91,12.4,8.82,11.42,9.29,10.58z M11.5,16c0,0.83-0.67,1.5-1.5,1.5 c-0.83,0-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5C10.83,14.5,11.5,15.17,11.5,16z",
                }
            }
        }
    }
}

pub fn question_mark_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M11.07,12.85c0.77-1.39,2.25-2.21,3.11-3.44c0.91-1.29,0.4-3.7-2.18-3.7c-1.69,0-2.52,1.28-2.87,2.34L6.54,6.96 C7.25,4.83,9.18,3,11.99,3c2.35,0,3.96,1.07,4.78,2.41c0.7,1.15,1.11,3.3,0.03,4.9c-1.2,1.77-2.35,2.31-2.97,3.45 c-0.25,0.46-0.35,0.76-0.35,2.24h-2.89C10.58,15.22,10.46,13.95,11.07,12.85z M14,20c0,1.1-0.9,2-2,2s-2-0.9-2-2c0-1.1,0.9-2,2-2 S14,18.9,14,20z",
                }
            }
        }
    }
}

pub fn quickreply_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M17,4c0-0.55-0.45-1-1-1H4C3.45,3,3,3.45,3,4v13l3-3h7V9h4V4z",
                        }
                    }
                    g {
                        polygon {
                            points: "18,13 16.4,13 17.64,10 14,10 14,14.36 15.45,14.36 15.45,18",
                        }
                    }
                }
            }
        }
    }
}

pub fn quickreply_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    g {
                        path {
                            d: "M22,4c0-1.1-0.9-2-2-2H4C2.9,2,2.01,2.9,2.01,4L2,22l4-4h9v-8h7V4z",
                        }
                    }
                    g {
                        polygon {
                            points: "22.5,16 20.3,16 22,12 17,12 17,18 19,18 19,23",
                        }
                    }
                }
            }
        }
    }
}

pub fn rebase_edit_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.56 9.15c-.2-.2-.51-.2-.71 0l-1.21 1.21 2 2 1.21-1.21c.2-.2.2-.51 0-.71l-1.29-1.29z",
                    }
                    polygon {
                        points: "9,16 9,18 11,18 15.94,13.06 13.94,11.06",
                    }
                    circle {
                        cx: "15.5",
                        r: "2.5",
                        cy: "4.5",
                    }
                    path {
                        d: "M9.17 7.33 12 4.5 9.17 1.67 8.11 2.73l1.02 1.02H6.87C6.55 2.74 5.62 2 4.5 2 3.12 2 2 3.12 2 4.5c0 1.12.74 2.05 1.75 2.37v6.25C2.74 13.45 2 14.38 2 15.5 2 16.88 3.12 18 4.5 18S7 16.88 7 15.5c0-1.12-.74-2.05-1.75-2.37V6.87c.77-.24 1.38-.85 1.62-1.62h2.26L8.11 6.27l1.06 1.06z",
                    }
                }
            }
        }
    }
}

pub fn rebase_edit_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cy: "5",
                        cx: "19",
                        r: "3",
                    }
                    path {
                        d: "M10.76 9.24 15 5 10.76.76 9.34 2.17 11.17 4H7.82C7.4 2.84 6.3 2 5 2 3.34 2 2 3.34 2 5c0 1.3.84 2.4 2 2.82v8.37C2.84 16.6 2 17.7 2 19c0 1.66 1.34 3 3 3s3-1.34 3-3c0-1.3-.84-2.4-2-2.82V7.82c.85-.31 1.51-.97 1.82-1.82h3.36L9.34 7.83l1.42 1.41z",
                    }
                    polygon {
                        points: "10,19 10,22 13,22 19.26,15.74 16.26,12.74",
                    }
                    path {
                        d: "M20.12 10.29c-.39-.39-1.02-.39-1.41 0l-1.38 1.38 3 3 1.38-1.38c.39-.39.39-1.02 0-1.41l-1.59-1.59z",
                    }
                }
            }
        }
    }
}

pub fn receipt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18 17H6v-2h12v2zm0-4H6v-2h12v2zm0-4H6V7h12v2zM3 22l1.5-1.5L6 22l1.5-1.5L9 22l1.5-1.5L12 22l1.5-1.5L15 22l1.5-1.5L18 22l1.5-1.5L21 22V2l-1.5 1.5L18 2l-1.5 1.5L15 2l-1.5 1.5L12 2l-1.5 1.5L9 2 7.5 3.5 6 2 4.5 3.5 3 2v20z",
            }
        }
    }
}

pub fn record_voice_over_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
            circle {
                cy: "9",
                cx: "9",
                r: "4",
            }
            path {
                d: "M9 15c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4zm7.76-9.64l-1.68 1.69c.84 1.18.84 2.71 0 3.89l1.68 1.69c2.02-2.02 2.02-5.07 0-7.27zM20.07 2l-1.63 1.63c2.77 3.02 2.77 7.56 0 10.74L20.07 16c3.9-3.89 3.91-9.95 0-14z",
            }
        }
    }
}

pub fn redeem_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 6h-2.18c.11-.31.18-.65.18-1 0-1.66-1.34-3-3-3-1.05 0-1.96.54-2.5 1.35l-.5.67-.5-.68C10.96 2.54 10.05 2 9 2 7.34 2 6 3.34 6 5c0 .35.07.69.18 1H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-5-2c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zM9 4c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm11 15H4v-2h16v2zm0-5H4V8h5.08L7 10.83 8.62 12 11 8.76l1-1.36 1 1.36L15.38 12 17 10.83 14.92 8H20v6z",
            }
        }
    }
}

pub fn remove_done_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M0 0h24v24H0zm0 0h24v24H0V0z",
                fill: "none",
            }
            path {
                d: "M1.79 12l5.58 5.59L5.96 19 .37 13.41 1.79 12zm.45-7.78L12.9 14.89l-1.28 1.28L7.44 12l-1.41 1.41L11.62 19l2.69-2.69 4.89 4.89 1.41-1.41L3.65 2.81 2.24 4.22zm14.9 9.27L23.62 7 22.2 5.59l-6.48 6.48 1.42 1.42zM17.96 7l-1.41-1.41-3.65 3.66 1.41 1.41L17.96 7z",
            }
        }
    }
}

pub fn remove_shopping_cart_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M22.73 22.73L2.77 2.77 2 2l-.73-.73L0 2.54l4.39 4.39 2.21 4.66-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h7.46l1.38 1.38c-.5.36-.83.95-.83 1.62 0 1.1.89 2 1.99 2 .67 0 1.26-.33 1.62-.84L21.46 24l1.27-1.27zM7.42 15c-.14 0-.25-.11-.25-.25l.03-.12.9-1.63h2.36l2 2H7.42zm8.13-2c.75 0 1.41-.41 1.75-1.03l3.58-6.49c.08-.14.12-.31.12-.48 0-.55-.45-1-1-1H6.54l9.01 9zM7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

pub fn reorder_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 15h18v-2H3v2zm0 4h18v-2H3v2zm0-8h18V9H3v2zm0-6v2h18V5H3z",
            }
        }
    }
}

pub fn repartition_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M5,8.25l1.06-1.06L4.87,6H14c0.83,0,1.5,0.67,1.5,1.5S14.83,9,14,9H3v1.5h11c1.65,0,3-1.35,3-3s-1.35-3-3-3H4.87 l1.19-1.19L5,2.25l-3,3L5,8.25z",
                    }
                    path {
                        d: "M3,17h14v-5H3V17z M12.83,13.5h2.67v2h-2.67V13.5z M8.67,13.5h2.67v2H8.67V13.5z M4.5,13.5h2.67v2H4.5V13.5z",
                    }
                }
            }
        }
    }
}

pub fn repartition_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M3,21h18v-6H3V21z M10.33,19v-2h3.33v2H10.33z M19,19h-3.33v-2H19V19z M5,17h3.33v2H5V17z",
                    }
                    path {
                        d: "M6,10l1.42-1.42L5.83,7H17c1.1,0,2,0.9,2,2s-0.9,2-2,2H3v2h14c2.21,0,4-1.79,4-4s-1.79-4-4-4H5.83l1.59-1.59L6,2L2,6L6,10 z",
                    }
                }
            }
        }
    }
}

pub fn report_problem_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z",
            }
        }
    }
}

pub fn request_page_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14,2H6C4.9,2,4,2.9,4,4v16c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8L14,2z M15,11h-4v1h3c0.55,0,1,0.45,1,1v3 c0,0.55-0.45,1-1,1h-1v1h-2v-1H9v-2h4v-1h-3c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h1V8h2v1h2V11z",
            }
        }
    }
}

pub fn restore_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M13 3c-4.97 0-9 4.03-9 9H1l3.89 3.89.07.14L9 12H6c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.93 0-3.68-.79-4.94-2.06l-1.42 1.42C8.27 19.99 10.51 21 13 21c4.97 0 9-4.03 9-9s-4.03-9-9-9zm-1 5v5l4.28 2.54.72-1.21-3.5-2.08V8H12z",
            }
        }
    }
}

pub fn restore_from_trash_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 4h-3.5l-1-1h-5l-1 1H5v2h14zM6 7v12c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V7H6zm8 7v4h-4v-4H8l4-4 4 4h-2z",
            }
        }
    }
}

pub fn restore_page_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm-2 16c-2.05 0-3.81-1.24-4.58-3h1.71c.63.9 1.68 1.5 2.87 1.5 1.93 0 3.5-1.57 3.5-3.5S13.93 9.5 12 9.5c-1.35 0-2.52.78-3.1 1.9l1.6 1.6h-4V9l1.3 1.3C8.69 8.92 10.23 8 12 8c2.76 0 5 2.24 5 5s-2.24 5-5 5z",
            }
        }
    }
}

pub fn rocket_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M7.98,16c-0.1-0.24-1.23-3.01-1.23-5.5c0-5.46,2.49-7.52,3.25-8c0.76,0.48,3.25,2.54,3.25,8c0,2.49-1.13,5.26-1.23,5.5 H7.98z M11.5,9c0-0.82-0.67-1.5-1.5-1.5C9.18,7.5,8.5,8.18,8.5,9c0,0.83,0.68,1.5,1.5,1.5C10.83,10.5,11.5,9.83,11.5,9z M4.58,11.8 C4.21,12.08,4,12.52,4,12.98V18l3.09-1.54c-0.5-1.18-1.24-3.4-1.33-5.57L4.58,11.8z M15.42,11.8c0.37,0.28,0.58,0.72,0.58,1.18V18 l-3.09-1.54c0.5-1.18,1.24-3.4,1.33-5.57L15.42,11.8z",
                }
            }
        }
    }
}

pub fn rocket_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2.5c0,0,4.5,2.04,4.5,10.5c0,2.49-1.04,5.57-1.6,7H9.1c-0.56-1.43-1.6-4.51-1.6-7C7.5,4.54,12,2.5,12,2.5z M14,11 c0-1.1-0.9-2-2-2s-2,0.9-2,2s0.9,2,2,2S14,12.1,14,11z M7.69,20.52c-0.48-1.23-1.52-4.17-1.67-6.87l-1.13,0.75 C4.33,14.78,4,15.4,4,16.07V22L7.69,20.52z M20,22v-5.93c0-0.67-0.33-1.29-0.89-1.66l-1.13-0.75c-0.15,2.69-1.2,5.64-1.67,6.87 L20,22z",
                }
            }
        }
    }
}

pub fn rocket_launch_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6.47,10.67c0.1-0.24,1.26-3,3.02-4.76c3.86-3.86,7.08-3.56,7.95-3.36c0.2,0.88,0.5,4.09-3.36,7.95 c-1.76,1.76-4.52,2.92-4.76,3.02L6.47,10.67z M12.5,9C13.32,9,14,8.33,14,7.5S13.33,6,12.5,6C11.68,6,11,6.67,11,7.5 S11.67,9,12.5,9z M7.04,5.29c-0.46-0.06-0.92,0.1-1.25,0.43L2.25,9.27l3.28,1.09c0.48-1.18,1.53-3.29,3-4.88L7.04,5.29z M14.71,12.96c0.06,0.46-0.1,0.92-0.43,1.25l-3.55,3.55l-1.09-3.28c1.18-0.48,3.29-1.53,4.88-3L14.71,12.96z M5.25,12.5 c-0.62,0-1.18,0.25-1.59,0.66C2.44,14.38,2,18,2,18s3.62-0.44,4.84-1.66c0.41-0.41,0.66-0.97,0.66-1.59 C7.5,13.51,6.49,12.5,5.25,12.5z",
                }
            }
        }
    }
}

pub fn rocket_launch_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M9.19,6.35c-2.04,2.29-3.44,5.58-3.57,5.89L2,10.69l4.05-4.05c0.47-0.47,1.15-0.68,1.81-0.55L9.19,6.35L9.19,6.35z M11.17,17c0,0,3.74-1.55,5.89-3.7c5.4-5.4,4.5-9.62,4.21-10.57c-0.95-0.3-5.17-1.19-10.57,4.21C8.55,9.09,7,12.83,7,12.83 L11.17,17z M17.65,14.81c-2.29,2.04-5.58,3.44-5.89,3.57L13.31,22l4.05-4.05c0.47-0.47,0.68-1.15,0.55-1.81L17.65,14.81 L17.65,14.81z M9,18c0,0.83-0.34,1.58-0.88,2.12C6.94,21.3,2,22,2,22s0.7-4.94,1.88-6.12C4.42,15.34,5.17,15,6,15 C7.66,15,9,16.34,9,18z M13,9c0-1.1,0.9-2,2-2s2,0.9,2,2s-0.9,2-2,2S13,10.1,13,9z",
                    }
                }
            }
        }
    }
}

pub fn room_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2C8.13 2 5 5.13 5 9c0 5.25 7 13 7 13s7-7.75 7-13c0-3.87-3.13-7-7-7zm0 9.5c-1.38 0-2.5-1.12-2.5-2.5s1.12-2.5 2.5-2.5 2.5 1.12 2.5 2.5-1.12 2.5-2.5 2.5z",
            }
        }
    }
}

pub fn rounded_corner_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M19,19h2v2h-2V19z M19,17h2v-2h-2V17z M3,13h2v-2H3V13z M3,17h2v-2H3V17z M3,9h2V7H3V9z M3,5h2V3H3V5z M7,5h2V3H7V5z M15,21h2v-2h-2V21z M11,21h2v-2h-2V21z M15,21h2v-2h-2V21z M7,21h2v-2H7V21z M3,21h2v-2H3V21z M21,8c0-2.76-2.24-5-5-5h-5v2h5 c1.65,0,3,1.35,3,3v5h2V8z",
                        }
                    }
                }
            }
        }
    }
}

pub fn rowing_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M8.5,14.5L4,19l1.5,1.5L9,17h2L8.5,14.5z M15,1c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S16.1,1,15,1z M21,21.01L18,24 l-2.99-3.01V19.5l-7.1-7.09C7.6,12.46,7.3,12.48,7,12.48v-2.16c1.66,0.03,3.61-0.87,4.67-2.04l1.4-1.55 C13.42,6.34,14.06,6,14.72,6h0.03C15.99,6.01,17,7.02,17,8.26v5.75c0,0.84-0.35,1.61-0.92,2.16l-3.58-3.58v-2.27 c-0.63,0.52-1.43,1.02-2.29,1.39L16.5,18H18L21,21.01z",
                        }
                    }
                }
            }
        }
    }
}

pub fn rule_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.54,11L13,7.46l1.41-1.41l2.12,2.12l4.24-4.24l1.41,1.41L16.54,11z M11,7H2v2h9V7z M21,13.41L19.59,12L17,14.59 L14.41,12L13,13.41L15.59,16L13,18.59L14.41,20L17,17.41L19.59,20L21,18.59L18.41,16L21,13.41z M11,15H2v2h9V15z",
                }
            }
        }
    }
}

pub fn satellite_alt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17.5,12H19c0,3.87-3.13,7-7,7v-1.5C15.03,17.5,17.5,15.03,17.5,12z M12,14.5V16c2.21,0,4-1.79,4-4h-1.5 C14.5,13.38,13.38,14.5,12,14.5z M15.29,0.44l3.18,3.18c0.59,0.59,0.59,1.54,0,2.12l-2.83,2.83c-0.59,0.59-1.54,0.59-2.12,0 l-1.24-1.24l-0.71,0.71l1.24,1.24c0.59,0.59,0.59,1.54,0,2.12l-1.41,1.41c-0.59,0.59-1.54,0.59-2.12,0l-1.24-1.24l-0.71,0.71 l1.24,1.24c0.59,0.59,0.59,1.54,0,2.12l-2.83,2.83c-0.59,0.59-1.54,0.59-2.12,0l-3.18-3.18c-0.59-0.59-0.59-1.54,0-2.12l2.83-2.83 c0.59-0.59,1.54-0.59,2.12,0l1.24,1.24l0.71-0.71L6.1,9.63c-0.59-0.59-0.59-1.54,0-2.12L7.51,6.1c0.59-0.59,1.54-0.59,2.12,0 l1.24,1.24l0.71-0.71l-1.24-1.24c-0.59-0.59-0.59-1.54,0-2.12l2.83-2.83C13.75-0.15,14.7-0.15,15.29,0.44z M2.56,13.17L1.5,14.23 l3.18,3.18l1.06-1.06L2.56,13.17z M4.33,11.4l-1.06,1.06l3.18,3.18l1.06-1.06L4.33,11.4z M12.46,3.27L11.4,4.33l3.18,3.18 l1.06-1.06L12.46,3.27z M14.23,1.5l-1.06,1.06l3.18,3.18l1.06-1.06L14.23,1.5z",
                }
            }
        }
    }
}

pub fn satellite_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.44,0.59l-3.18,3.18c-0.78,0.78-0.78,2.05,0,2.83l1.24,1.24l-0.71,0.71L11.55,7.3c-0.78-0.78-2.05-0.78-2.83,0 L7.3,8.72c-0.78,0.78-0.78,2.05,0,2.83l1.24,1.24l-0.71,0.71L6.6,12.25c-0.78-0.78-2.05-0.78-2.83,0l-3.18,3.18 c-0.78,0.78-0.78,2.05,0,2.83l3.54,3.54c0.78,0.78,2.05,0.78,2.83,0l3.18-3.18c0.78-0.78,0.78-2.05,0-2.83l-1.24-1.24l0.71-0.71 l1.24,1.24c0.78,0.78,2.05,0.78,2.83,0l1.41-1.41c0.78-0.78,0.78-2.05,0-2.83L13.84,9.6l0.71-0.71l1.24,1.24 c0.78,0.78,2.05,0.78,2.83,0l3.18-3.18c0.78-0.78,0.78-2.05,0-2.83l-3.54-3.54C17.48-0.2,16.22-0.2,15.44,0.59z M6.6,19.32 l-1.06,1.06L2,16.85l1.06-1.06L6.6,19.32z M8.72,17.2l-1.06,1.06l-3.54-3.54l1.06-1.06L8.72,17.2z M18.26,7.66L17.2,8.72 l-3.54-3.54l1.06-1.06L18.26,7.66z M20.38,5.54L19.32,6.6l-3.54-3.54L16.85,2L20.38,5.54z M14,21l0,2c4.97,0,9-4.03,9-9l-2,0 C21,17.87,17.87,21,14,21z M14,17l0,2c2.76,0,5-2.24,5-5l-2,0C17,15.66,15.66,17,14,17z",
                    }
                }
            }
        }
    }
}

pub fn saved_search_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14zm-2.17-1.5l2.14-1.53 2.14 1.53-.83-2.46 2.15-1.5h-2.62L9.47 6l-.84 2.54H6l2.14 1.49z",
            }
        }
    }
}

pub fn savings_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M16.5,6.5l-2-2C14.5,4.14,15,3.52,15,3c0-0.55-0.45-1-1-1c-2.17,0-3.35,1.5-3.35,1.5H6.5C4.05,3.5,2,5.47,2,8 c0,2.33,2,9,2,9h4.5v-1.5H10V17h4.5l1.25-4.5L18,12V6.5H16.5z M11,7.5H7V6h4V7.5z M13.25,9c-0.41,0-0.75-0.34-0.75-0.75 c0-0.41,0.34-0.75,0.75-0.75S14,7.84,14,8.25C14,8.66,13.66,9,13.25,9z",
            }
        }
    }
}

pub fn savings_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                    d: "M19.83,7.5l-2.27-2.27c0.07-0.42,0.18-0.81,0.32-1.15C17.96,3.9,18,3.71,18,3.5C18,2.67,17.33,2,16.5,2 c-1.64,0-3.09,0.79-4,2l-5,0C4.46,4,2,6.46,2,9.5S4.5,21,4.5,21l5.5,0v-2h2v2l5.5,0l1.68-5.59L22,14.47V7.5H19.83z M13,9H8V7h5V9z M16,11c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C17,10.55,16.55,11,16,11z",
                }
            }
        }
    }
}

pub fn schedule_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zM12 20c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z",
            }
            path {
                d: "M12.5 7H11v6l5.25 3.15.75-1.23-4.5-2.67z",
            }
        }
    }
}

pub fn schedule_send_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16.5 12.5H15v4l3 2 .75-1.23-2.25-1.52V12.5zM16 9L2 3v7l9 2-9 2v7l7.27-3.11C10.09 20.83 12.79 23 16 23c3.86 0 7-3.14 7-7s-3.14-7-7-7zm0 12c-2.75 0-4.98-2.22-5-4.97v-.07c.02-2.74 2.25-4.97 5-4.97 2.76 0 5 2.24 5 5S18.76 21 16 21z",
            }
        }
    }
}

pub fn search_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z",
            }
        }
    }
}

pub fn search_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12,11.29c0.62-0.77,1-1.73,1-2.79C13,6.01,10.99,4,8.5,4C6.19,4,4.3,5.75,4.05,8h1C5.29,6.31,6.74,5,8.5,5 C10.43,5,12,6.57,12,8.5S10.43,12,8.5,12c-0.17,0-0.34-0.03-0.5-0.05v1C8.17,12.97,8.33,13,8.5,13c1.06,0,2.02-0.38,2.79-1 l3.57,3.57l0.71-0.71L12,11.29z",
                    }
                    polygon {
                        points: "5.91,9.38 4.5,10.79 3.09,9.38 2.38,10.09 3.79,11.5 2.38,12.91 3.09,13.62 4.5,12.21 5.91,13.62 6.62,12.91 5.21,11.5 6.62,10.09",
                    }
                }
            }
        }
    }
}

pub fn search_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.5,14h-0.79l-0.28-0.27C15.41,12.59,16,11.11,16,9.5C16,5.91,13.09,3,9.5,3C6.08,3,3.28,5.64,3.03,9h2.02 C5.3,6.75,7.18,5,9.5,5C11.99,5,14,7.01,14,9.5S11.99,14,9.5,14c-0.17,0-0.33-0.03-0.5-0.05v2.02C9.17,15.99,9.33,16,9.5,16 c1.61,0,3.09-0.59,4.23-1.57L14,14.71v0.79l5,4.99L20.49,19L15.5,14z",
                    }
                    polygon {
                        points: "6.47,10.82 4,13.29 1.53,10.82 0.82,11.53 3.29,14 0.82,16.47 1.53,17.18 4,14.71 6.47,17.18 7.18,16.47 4.71,14 7.18,11.53",
                    }
                }
            }
        }
    }
}

pub fn segment_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M9 18h12v-2H9v2zM3 6v2h18V6H3zm6 7h12v-2H9v2z",
            }
        }
    }
}

pub fn send_and_archive_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("20".to_string()),
            height: props.height.unwrap_or("20".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h20v20H0V0z",
                fill: "none",
            }
            path {
                d: "M18 8h-3.5L1 2v6l8 2-8 2v6l7-3v3c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2v-8c0-1.1-.9-2-2-2zm0 10h-8v-8h8v8zm-4-1l-3-3.5h2V11h2v2.5h2L14 17z",
            }
        }
    }
}

pub fn send_and_archive_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21 10h-3L2 3v7l9 2-9 2v7l8-3.5V21c0 1.1.9 2 2 2h9c1.1 0 2-.9 2-2v-9c0-1.1-.9-2-2-2zm0 11h-9v-9h9v9zm-4.5-1L13 16h2v-3h3v3h2l-3.5 4z",
            }
        }
    }
}

pub fn sensors_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M5.41,14.59l-1.06,1.06C2.9,14.21,2,12.21,2,10c0-2.21,0.9-4.21,2.34-5.66l1.06,1.06C4.23,6.58,3.5,8.21,3.5,10 S4.23,13.42,5.41,14.59z M16.5,10c0,1.79-0.73,3.42-1.91,4.59l1.06,1.06C17.1,14.21,18,12.21,18,10c0-2.21-0.9-4.21-2.34-5.66 l-1.06,1.06C15.77,6.58,16.5,8.21,16.5,10z M13.5,10c0,0.96-0.39,1.84-1.03,2.47l1.06,1.06C14.44,12.63,15,11.38,15,10 c0-1.38-0.56-2.63-1.46-3.54l-1.06,1.06C13.11,8.16,13.5,9.04,13.5,10z M6.5,10c0-0.96,0.39-1.84,1.03-2.47L6.46,6.46 C5.56,7.37,5,8.62,5,10c0,1.38,0.56,2.63,1.46,3.54l1.06-1.06C6.89,11.84,6.5,10.96,6.5,10z M10,8.25c-0.97,0-1.75,0.78-1.75,1.75 s0.78,1.75,1.75,1.75s1.75-0.78,1.75-1.75S10.97,8.25,10,8.25z",
            }
        }
    }
}

pub fn sensors_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7.76,16.24C6.67,15.16,6,13.66,6,12s0.67-3.16,1.76-4.24l1.42,1.42C8.45,9.9,8,10.9,8,12c0,1.1,0.45,2.1,1.17,2.83 L7.76,16.24z M16.24,16.24C17.33,15.16,18,13.66,18,12s-0.67-3.16-1.76-4.24l-1.42,1.42C15.55,9.9,16,10.9,16,12 c0,1.1-0.45,2.1-1.17,2.83L16.24,16.24z M12,10c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S13.1,10,12,10z M20,12 c0,2.21-0.9,4.21-2.35,5.65l1.42,1.42C20.88,17.26,22,14.76,22,12s-1.12-5.26-2.93-7.07l-1.42,1.42C19.1,7.79,20,9.79,20,12z M6.35,6.35L4.93,4.93C3.12,6.74,2,9.24,2,12s1.12,5.26,2.93,7.07l1.42-1.42C4.9,16.21,4,14.21,4,12S4.9,7.79,6.35,6.35z",
            }
        }
    }
}

pub fn sensors_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M6.71,8.84C6.58,9.2,6.5,9.59,6.5,10c0,0.96,0.39,1.84,1.03,2.47l-1.06,1.06C5.56,12.63,5,11.38,5,10 c0-0.83,0.22-1.61,0.58-2.3L4.47,6.59C3.86,7.59,3.5,8.75,3.5,10c0,1.79,0.73,3.42,1.91,4.59l-1.06,1.06C2.9,14.21,2,12.21,2,10 c0-1.67,0.51-3.21,1.38-4.5L1.87,3.99l1.06-1.06l14.14,14.14l-1.06,1.06L6.71,8.84z M14.59,5.41l1.06-1.06C17.1,5.79,18,7.79,18,10 c0,1.67-0.51,3.22-1.38,4.5l-1.09-1.09c0.61-0.99,0.97-2.16,0.97-3.41C16.5,8.21,15.77,6.58,14.59,5.41z M12.47,7.53l1.06-1.06 C14.44,7.37,15,8.62,15,10c0,0.83-0.2,1.62-0.57,2.31l-1.14-1.14c0.13-0.37,0.2-0.76,0.2-1.18C13.5,9.04,13.11,8.16,12.47,7.53z",
            }
        }
    }
}

pub fn sensors_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M8.14,10.96C8.05,11.29,8,11.64,8,12c0,1.1,0.45,2.1,1.17,2.83l-1.42,1.42C6.67,15.16,6,13.66,6,12 c0-0.93,0.21-1.8,0.58-2.59L5.11,7.94C4.4,9.13,4,10.52,4,12c0,2.21,0.9,4.21,2.35,5.65l-1.42,1.42C3.12,17.26,2,14.76,2,12 c0-2.04,0.61-3.93,1.66-5.51L1.39,4.22l1.41-1.41l18.38,18.38l-1.41,1.41L8.14,10.96z M17.42,14.59C17.79,13.8,18,12.93,18,12 c0-1.66-0.67-3.16-1.76-4.24l-1.42,1.42C15.55,9.9,16,10.9,16,12c0,0.36-0.05,0.71-0.14,1.04L17.42,14.59z M20,12 c0,1.48-0.4,2.87-1.11,4.06l1.45,1.45C21.39,15.93,22,14.04,22,12c0-2.76-1.12-5.26-2.93-7.07l-1.42,1.42C19.1,7.79,20,9.79,20,12z",
            }
        }
    }
}

pub fn settings_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                path {
                    d: "M19.14,12.94c0.04-0.3,0.06-0.61,0.06-0.94c0-0.32-0.02-0.64-0.07-0.94l2.03-1.58c0.18-0.14,0.23-0.41,0.12-0.61 l-1.92-3.32c-0.12-0.22-0.37-0.29-0.59-0.22l-2.39,0.96c-0.5-0.38-1.03-0.7-1.62-0.94L14.4,2.81c-0.04-0.24-0.24-0.41-0.48-0.41 h-3.84c-0.24,0-0.43,0.17-0.47,0.41L9.25,5.35C8.66,5.59,8.12,5.92,7.63,6.29L5.24,5.33c-0.22-0.08-0.47,0-0.59,0.22L2.74,8.87 C2.62,9.08,2.66,9.34,2.86,9.48l2.03,1.58C4.84,11.36,4.8,11.69,4.8,12s0.02,0.64,0.07,0.94l-2.03,1.58 c-0.18,0.14-0.23,0.41-0.12,0.61l1.92,3.32c0.12,0.22,0.37,0.29,0.59,0.22l2.39-0.96c0.5,0.38,1.03,0.7,1.62,0.94l0.36,2.54 c0.05,0.24,0.24,0.41,0.48,0.41h3.84c0.24,0,0.44-0.17,0.47-0.41l0.36-2.54c0.59-0.24,1.13-0.56,1.62-0.94l2.39,0.96 c0.22,0.08,0.47,0,0.59-0.22l1.92-3.32c0.12-0.22,0.07-0.47-0.12-0.61L19.14,12.94z M12,15.6c-1.98,0-3.6-1.62-3.6-3.6 s1.62-3.6,3.6-3.6s3.6,1.62,3.6,3.6S13.98,15.6,12,15.6z",
                }
            }
        }
    }
}

pub fn settings_accessibility_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20.5,4c-2.61,0.7-5.67,1-8.5,1S6.11,4.7,3.5,4L3,6c1.86,0.5,4,0.83,6,1v12h2v-6h2v6h2V7c2-0.17,4.14-0.5,6-1L20.5,4z M12,4c1.1,0,2-0.9,2-2s-0.9-2-2-2s-2,0.9-2,2S10.9,4,12,4z M7,24h2v-2H7V24z M11,24h2v-2h-2V24z M15,24h2v-2h-2V24z",
                }
            }
        }
    }
}

pub fn settings_applications_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 10c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zm7-7H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.11 0 2-.9 2-2V5c0-1.1-.89-2-2-2zm-1.75 9c0 .23-.02.46-.05.68l1.48 1.16c.13.11.17.3.08.45l-1.4 2.42c-.09.15-.27.21-.43.15l-1.74-.7c-.36.28-.76.51-1.18.69l-.26 1.85c-.03.17-.18.3-.35.3h-2.8c-.17 0-.32-.13-.35-.29l-.26-1.85c-.43-.18-.82-.41-1.18-.69l-1.74.7c-.16.06-.34 0-.43-.15l-1.4-2.42c-.09-.15-.05-.34.08-.45l1.48-1.16c-.03-.23-.05-.46-.05-.69 0-.23.02-.46.05-.68l-1.48-1.16c-.13-.11-.17-.3-.08-.45l1.4-2.42c.09-.15.27-.21.43-.15l1.74.7c.36-.28.76-.51 1.18-.69l.26-1.85c.03-.17.18-.3.35-.3h2.8c.17 0 .32.13.35.29l.26 1.85c.43.18.82.41 1.18.69l1.74-.7c.16-.06.34 0 .43.15l1.4 2.42c.09.15.05.34-.08.45l-1.48 1.16c.03.23.05.46.05.69z",
            }
        }
    }
}

pub fn settings_backup_restore_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M14 12c0-1.1-.9-2-2-2s-2 .9-2 2 .9 2 2 2 2-.9 2-2zm-2-9c-4.97 0-9 4.03-9 9H0l4 4 4-4H5c0-3.87 3.13-7 7-7s7 3.13 7 7-3.13 7-7 7c-1.51 0-2.91-.49-4.06-1.3l-1.42 1.44C8.04 20.3 9.94 21 12 21c4.97 0 9-4.03 9-9s-4.03-9-9-9z",
            }
        }
    }
}

pub fn settings_bluetooth_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11 24h2v-2h-2v2zm-4 0h2v-2H7v2zm8 0h2v-2h-2v2zm2.71-18.29L12 0h-1v7.59L6.41 3 5 4.41 10.59 10 5 15.59 6.41 17 11 12.41V20h1l5.71-5.71-4.3-4.29 4.3-4.29zM13 3.83l1.88 1.88L13 7.59V3.83zm1.88 10.46L13 16.17v-3.76l1.88 1.88z",
            }
        }
    }
}

pub fn settings_brightness_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02zM8 16h2.5l1.5 1.5 1.5-1.5H16v-2.5l1.5-1.5-1.5-1.5V8h-2.5L12 6.5 10.5 8H8v2.5L6.5 12 8 13.5V16zm4-7c1.66 0 3 1.34 3 3s-1.34 3-3 3V9z",
            }
        }
    }
}

pub fn settings_cell_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M7 24h2v-2H7v2zm4 0h2v-2h-2v2zm4 0h2v-2h-2v2zM16 .01L8 0C6.9 0 6 .9 6 2v16c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V2c0-1.1-.9-1.99-2-1.99zM16 16H8V4h8v12z",
            }
        }
    }
}

pub fn settings_ethernet_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M7.77 6.76L6.23 5.48.82 12l5.41 6.52 1.54-1.28L3.42 12l4.35-5.24zM7 13h2v-2H7v2zm10-2h-2v2h2v-2zm-6 2h2v-2h-2v2zm6.77-7.52l-1.54 1.28L20.58 12l-4.35 5.24 1.54 1.28L23.18 12l-5.41-6.52z",
            }
        }
    }
}

pub fn settings_input_antenna_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 5c-3.87 0-7 3.13-7 7h2c0-2.76 2.24-5 5-5s5 2.24 5 5h2c0-3.87-3.13-7-7-7zm1 9.29c.88-.39 1.5-1.26 1.5-2.29 0-1.38-1.12-2.5-2.5-2.5S9.5 10.62 9.5 12c0 1.02.62 1.9 1.5 2.29v3.3L7.59 21 9 22.41l3-3 3 3L16.41 21 13 17.59v-3.3zM12 1C5.93 1 1 5.93 1 12h2c0-4.97 4.03-9 9-9s9 4.03 9 9h2c0-6.07-4.93-11-11-11z",
            }
        }
    }
}

pub fn settings_input_component_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M5 2c0-.55-.45-1-1-1s-1 .45-1 1v4H1v6h6V6H5V2zm4 14c0 1.3.84 2.4 2 2.82V23h2v-4.18c1.16-.41 2-1.51 2-2.82v-2H9v2zm-8 0c0 1.3.84 2.4 2 2.82V23h2v-4.18C6.16 18.4 7 17.3 7 16v-2H1v2zM21 6V2c0-.55-.45-1-1-1s-1 .45-1 1v4h-2v6h6V6h-2zm-8-4c0-.55-.45-1-1-1s-1 .45-1 1v4H9v6h6V6h-2V2zm4 14c0 1.3.84 2.4 2 2.82V23h2v-4.18c1.16-.41 2-1.51 2-2.82v-2h-6v2z",
            }
        }
    }
}

pub fn settings_input_composite_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M5 2c0-.55-.45-1-1-1s-1 .45-1 1v4H1v6h6V6H5V2zm4 14c0 1.3.84 2.4 2 2.82V23h2v-4.18c1.16-.41 2-1.51 2-2.82v-2H9v2zm-8 0c0 1.3.84 2.4 2 2.82V23h2v-4.18C6.16 18.4 7 17.3 7 16v-2H1v2zM21 6V2c0-.55-.45-1-1-1s-1 .45-1 1v4h-2v6h6V6h-2zm-8-4c0-.55-.45-1-1-1s-1 .45-1 1v4H9v6h6V6h-2V2zm4 14c0 1.3.84 2.4 2 2.82V23h2v-4.18c1.16-.41 2-1.51 2-2.82v-2h-6v2z",
            }
        }
    }
}

pub fn settings_input_hdmi_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18 7V4c0-1.1-.9-2-2-2H8c-1.1 0-2 .9-2 2v3H5v6l3 6v3h8v-3l3-6V7h-1zM8 4h8v3h-2V5h-1v2h-2V5h-1v2H8V4z",
            }
        }
    }
}

pub fn settings_input_svideo_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M8 11.5c0-.83-.67-1.5-1.5-1.5S5 10.67 5 11.5 5.67 13 6.5 13 8 12.33 8 11.5zm7-5c0-.83-.67-1.5-1.5-1.5h-3C9.67 5 9 5.67 9 6.5S9.67 8 10.5 8h3c.83 0 1.5-.67 1.5-1.5zM8.5 15c-.83 0-1.5.67-1.5 1.5S7.67 18 8.5 18s1.5-.67 1.5-1.5S9.33 15 8.5 15zM12 1C5.93 1 1 5.93 1 12s4.93 11 11 11 11-4.93 11-11S18.07 1 12 1zm0 20c-4.96 0-9-4.04-9-9s4.04-9 9-9 9 4.04 9 9-4.04 9-9 9zm5.5-11c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5zm-2 5c-.83 0-1.5.67-1.5 1.5s.67 1.5 1.5 1.5 1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z",
            }
        }
    }
}

pub fn settings_overscan_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12.01 5.5L10 8h4l-1.99-2.5zM18 10v4l2.5-1.99L18 10zM6 10l-2.5 2.01L6 14v-4zm8 6h-4l2.01 2.5L14 16zm7-13H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16.01H3V4.99h18v14.02z",
            }
        }
    }
}

pub fn settings_phone_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M13 9h-2v2h2V9zm4 0h-2v2h2V9zm3 6.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.58l2.2-2.21c.28-.27.36-.66.25-1.01C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM19 9v2h2V9h-2z",
            }
        }
    }
}

pub fn settings_power_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M7 24h2v-2H7v2zm4 0h2v-2h-2v2zm2-22h-2v10h2V2zm3.56 2.44l-1.45 1.45C16.84 6.94 18 8.83 18 11c0 3.31-2.69 6-6 6s-6-2.69-6-6c0-2.17 1.16-4.06 2.88-5.12L7.44 4.44C5.36 5.88 4 8.28 4 11c0 4.42 3.58 8 8 8s8-3.58 8-8c0-2.72-1.36-5.12-3.44-6.56zM15 24h2v-2h-2v2z",
            }
        }
    }
}

pub fn settings_remote_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15 9H9c-.55 0-1 .45-1 1v12c0 .55.45 1 1 1h6c.55 0 1-.45 1-1V10c0-.55-.45-1-1-1zm-3 6c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2zM7.05 6.05l1.41 1.41C9.37 6.56 10.62 6 12 6s2.63.56 3.54 1.46l1.41-1.41C15.68 4.78 13.93 4 12 4s-3.68.78-4.95 2.05zM12 0C8.96 0 6.21 1.23 4.22 3.22l1.41 1.41C7.26 3.01 9.51 2 12 2s4.74 1.01 6.36 2.64l1.41-1.41C17.79 1.23 15.04 0 12 0z",
            }
        }
    }
}

pub fn settings_voice_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M7 24h2v-2H7v2zm5-11c1.66 0 2.99-1.34 2.99-3L15 4c0-1.66-1.34-3-3-3S9 2.34 9 4v6c0 1.66 1.34 3 3 3zm-1 11h2v-2h-2v2zm4 0h2v-2h-2v2zm4-14h-1.7c0 3-2.54 5.1-5.3 5.1S6.7 13 6.7 10H5c0 3.41 2.72 6.23 6 6.72V20h2v-3.28c3.28-.49 6-3.31 6-6.72z",
            }
        }
    }
}

pub fn shop_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16 6V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H2v13c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V6h-6zm-6-2h4v2h-4V4zM9 18V9l7.5 4L9 18z",
            }
        }
    }
}

pub fn shopping_bag_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M18 6h-2c0-2.21-1.79-4-4-4S8 3.79 8 6H6c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-8 4c0 .55-.45 1-1 1s-1-.45-1-1V8h2v2zm2-6c1.1 0 2 .9 2 2h-4c0-1.1.9-2 2-2zm4 6c0 .55-.45 1-1 1s-1-.45-1-1V8h2v2z",
                }
            }
        }
    }
}

pub fn shopping_basket_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17.21 9l-4.38-6.56c-.19-.28-.51-.42-.83-.42-.32 0-.64.14-.83.43L6.79 9H2c-.55 0-1 .45-1 1 0 .09.01.18.04.27l2.54 9.27c.23.84 1 1.46 1.92 1.46h13c.92 0 1.69-.62 1.93-1.46l2.54-9.27L23 10c0-.55-.45-1-1-1h-4.79zM9 9l3-4.4L15 9H9zm3 8c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

pub fn shopping_cart_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-.9-2-2-2zM1 2v2h2l3.6 7.59-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h12v-2H7.42c-.14 0-.25-.11-.25-.25l.03-.12.9-1.63h7.45c.75 0 1.41-.41 1.75-1.03l3.58-6.49c.08-.14.12-.31.12-.48 0-.55-.45-1-1-1H5.21l-.94-2H1zm16 16c-1.1 0-1.99.9-1.99 2s.89 2 1.99 2 2-.9 2-2-.9-2-2-2z",
            }
        }
    }
}

pub fn shopping_cart_checkout_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M7,16.5C7,17.33,6.33,18,5.5,18S4,17.33,4,16.5S4.67,15,5.5,15S7,15.67,7,16.5z M14.5,15c-0.83,0-1.5,0.67-1.5,1.5 s0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5S15.33,15,14.5,15z M6.6,11h6.67c0.6,0,1.14-0.36,1.38-0.91L17,4.59L15.62,4l-2.35,5.5H6.99 L3.77,2H1v1.5h1.79l2.73,6.37l-1.08,1.88c-0.58,1,0.14,2.25,1.3,2.25H16v-1.5H5.73L6.6,11z M8.94,3.06l1.19,1.19H7v1.5h3.13 L8.94,6.94L10,8l3-3l-3-3L8.94,3.06z",
                }
            }
        }
    }
}

pub fn shopping_cart_checkout_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M7,18c-1.1,0-1.99,0.9-1.99,2S5.9,22,7,22s2-0.9,2-2S8.1,18,7,18z M17,18c-1.1,0-1.99,0.9-1.99,2s0.89,2,1.99,2s2-0.9,2-2 S18.1,18,17,18z M8.1,13h7.45c0.75,0,1.41-0.41,1.75-1.03L21,4.96L19.25,4l-3.7,7H8.53L4.27,2H1v2h2l3.6,7.59l-1.35,2.44 C4.52,15.37,5.48,17,7,17h12v-2H7L8.1,13z M12,2l4,4l-4,4l-1.41-1.41L12.17,7L8,7l0-2l4.17,0l-1.59-1.59L12,2z",
                }
            }
        }
    }
}

pub fn shop_2_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M2.5,7H1v9.5C1,17.33,1.67,18,2.5,18H16v-1.5H2.5V7z",
                    }
                    path {
                        d: "M15,4V2.5C15,1.67,14.33,1,13.5,1h-4C8.67,1,8,1.67,8,2.5V4H4v9.5C4,14.33,4.67,15,5.5,15h12c0.83,0,1.5-0.67,1.5-1.5V4 H15z M9.5,2.5h4V4h-4V2.5z M10,12.5v-6l4.5,3L10,12.5z",
                    }
                }
            }
        }
    }
}

pub fn shop_2_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M3,9H1v11c0,1.11,0.89,2,2,2h16v-2H3V9z",
                    }
                    path {
                        d: "M18,5V3c0-1.11-0.89-2-2-2h-4c-1.11,0-2,0.89-2,2v2H5v11c0,1.11,0.89,2,2,2h14c1.11,0,2-0.89,2-2V5H18z M12,3h4v2h-4V3z M12,15V8l5.5,3.5L12,15z",
                    }
                }
            }
        }
    }
}

pub fn shop_two_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 9H1v11c0 1.11.89 2 2 2h14c1.11 0 2-.89 2-2H3V9zm15-4V3c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H5v11c0 1.11.89 2 2 2h14c1.11 0 2-.89 2-2V5h-5zm-6-2h4v2h-4V3zm0 12V8l5.5 3-5.5 4z",
            }
        }
    }
}

pub fn smart_button_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M22,9v6c0,1.1-0.9,2-2,2h-1l0-2h1V9H4v6h6v2H4c-1.1,0-2-0.9-2-2V9c0-1.1,0.9-2,2-2h16C21.1,7,22,7.9,22,9z M14.5,19 l1.09-2.41L18,15.5l-2.41-1.09L14.5,12l-1.09,2.41L11,15.5l2.41,1.09L14.5,19z M17,14l0.62-1.38L19,12l-1.38-0.62L17,10l-0.62,1.38 L15,12l1.38,0.62L17,14z M14.5,19l1.09-2.41L18,15.5l-2.41-1.09L14.5,12l-1.09,2.41L11,15.5l2.41,1.09L14.5,19z M17,14l0.62-1.38 L19,12l-1.38-0.62L17,10l-0.62,1.38L15,12l1.38,0.62L17,14z",
                }
            }
        }
    }
}

pub fn source_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,6h-8l-2-2H4C2.9,4,2.01,4.9,2.01,6L2,18c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V8C22,6.9,21.1,6,20,6z M14,16H6v-2h8V16z M18,12H6v-2h12V12z",
                }
            }
        }
    }
}

pub fn space_dashboard_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10.75,10H17v5.5c0,0.83-0.67,1.5-1.5,1.5h-4.75V10z M4.5,3C3.67,3,3,3.67,3,4.5v10.94C3,16.3,3.7,17,4.56,17h4.69V3H4.5z M17,8.5v-4C17,3.67,16.33,3,15.5,3h-4.75v5.5H17z",
            }
        }
    }
}

pub fn space_dashboard_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11,21H5c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2h6V21z M13,21h6c1.1,0,2-0.9,2-2v-7h-8V21z M21,10V5c0-1.1-0.9-2-2-2h-6v7H21z",
            }
        }
    }
}

pub fn spatial_audio_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cy: "8",
                        cx: "8",
                        r: "3",
                    }
                    path {
                        d: "M13.03,13.37C11.56,12.5,9.84,12,8,12s-3.56,0.5-5.03,1.37C2.36,13.72,2,14.39,2,15.09V17h12v-1.91 C14,14.39,13.64,13.72,13.03,13.37z",
                    }
                    path {
                        d: "M12.5,1H11c0,4.42,3.58,8,8,8V7.5C15.42,7.5,12.5,4.58,12.5,1z",
                    }
                    path {
                        d: "M15.5,1H14c0,2.76,2.24,5,5,5V4.5C17.07,4.5,15.5,2.93,15.5,1z",
                    }
                }
            }
        }
    }
}

pub fn spatial_audio_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        r: "4",
                        cx: "10",
                        cy: "9",
                    }
                    path {
                        d: "M16.39,15.56C14.71,14.7,12.53,14,10,14c-2.53,0-4.71,0.7-6.39,1.56C2.61,16.07,2,17.1,2,18.22V21h16v-2.78 C18,17.1,17.39,16.07,16.39,15.56z",
                    }
                    path {
                        d: "M16,1h-2c0,4.97,4.03,9,9,9V8C19.14,8,16,4.86,16,1z",
                    }
                    path {
                        d: "M20,1h-2c0,2.76,2.24,5,5,5V4C21.35,4,20,2.65,20,1z",
                    }
                }
            }
        }
    }
}

pub fn spatial_audio_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cx: "8",
                        r: "3",
                        cy: "8",
                    }
                    path {
                        d: "M13.03,13.37C11.56,12.5,9.84,12,8,12s-3.56,0.5-5.03,1.37C2.36,13.72,2,14.39,2,15.09V17h12v-1.91 C14,14.39,13.64,13.72,13.03,13.37z",
                    }
                    path {
                        d: "M16.66,1L15.6,2.06c2.53,2.53,2.53,6.66,0,9.19l1.06,1.06C19.78,9.19,19.78,4.12,16.66,1z",
                    }
                    path {
                        d: "M14.54,3.12l-1.06,1.06c1.36,1.36,1.36,3.59,0,4.95l1.06,1.06C16.49,8.24,16.49,5.07,14.54,3.12z",
                    }
                }
            }
        }
    }
}

pub fn spatial_audio_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        r: "4",
                        cy: "9",
                        cx: "10",
                    }
                    path {
                        d: "M16.39,15.56C14.71,14.7,12.53,14,10,14c-2.53,0-4.71,0.7-6.39,1.56C2.61,16.07,2,17.1,2,18.22V21h16v-2.78 C18,17.1,17.39,16.07,16.39,15.56z",
                    }
                    path {
                        d: "M20.36,1l-1.41,1.41c2.73,2.73,2.73,7.17,0,9.9l1.41,1.41C23.88,10.21,23.88,4.51,20.36,1z",
                    }
                    path {
                        d: "M17.54,10.9c1.95-1.95,1.95-5.12,0-7.07l-1.41,1.41c1.17,1.17,1.17,3.07,0,4.24L17.54,10.9z",
                    }
                }
            }
        }
    }
}

pub fn spatial_tracking_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cy: "8",
                        r: "3",
                        cx: "8",
                    }
                    path {
                        d: "M13.03,13.37C11.56,12.5,9.84,12,8,12s-3.56,0.5-5.03,1.37C2.36,13.72,2,14.39,2,15.09V17h12v-1.91 C14,14.39,13.64,13.72,13.03,13.37z",
                    }
                    path {
                        d: "M16.4,2.06L15.34,1c-3.12,3.12-3.12,8.19,0,11.31l1.06-1.06C13.87,8.72,13.87,4.59,16.4,2.06z",
                    }
                    path {
                        d: "M18.53,4.18l-1.06-1.06c-1.95,1.95-1.95,5.12,0,7.07l1.06-1.06C17.16,7.77,17.16,5.55,18.53,4.18z",
                    }
                }
            }
        }
    }
}

pub fn spatial_tracking_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    circle {
                        cy: "9",
                        cx: "10",
                        r: "4",
                    }
                    path {
                        d: "M16.39,15.56C14.71,14.7,12.53,14,10,14c-2.53,0-4.71,0.7-6.39,1.56C2.61,16.07,2,17.1,2,18.22V21h16v-2.78 C18,17.1,17.39,16.07,16.39,15.56z",
                    }
                    path {
                        d: "M20.05,2.41L18.64,1c-3.51,3.51-3.51,9.21,0,12.73l1.41-1.41C17.32,9.58,17.32,5.14,20.05,2.41z",
                    }
                    path {
                        d: "M22.88,5.24l-1.41-1.41c-1.95,1.95-1.95,5.12,0,7.07l1.41-1.41C21.71,8.32,21.71,6.41,22.88,5.24z",
                    }
                }
            }
        }
    }
}

pub fn speaker_notes_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM8 14H6v-2h2v2zm0-3H6V9h2v2zm0-3H6V6h2v2zm7 6h-5v-2h5v2zm3-3h-8V9h8v2zm0-3h-8V6h8v2z",
            }
        }
    }
}

pub fn speaker_notes_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M10.54 11l-.54-.54L7.54 8 6 6.46 2.38 2.84 1.27 1.73 0 3l2.01 2.01L2 22l4-4h9l5.73 5.73L22 22.46 17.54 18l-7-7zM8 14H6v-2h2v2zm-2-3V9l2 2H6zm14-9H4.08L10 7.92V6h8v2h-7.92l1 1H18v2h-4.92l6.99 6.99C21.14 17.95 22 17.08 22 16V4c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn spellcheck_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12.45 16h2.09L9.43 3H7.57L2.46 16h2.09l1.12-3h5.64l1.14 3zm-6.02-5L8.5 5.48 10.57 11H6.43zm15.16.59l-8.09 8.09L9.83 16l-1.41 1.41 5.09 5.09L23 13l-1.41-1.41z",
            }
        }
    }
}

pub fn stars_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22 12S17.52 2 11.99 2zm4.24 16L12 15.45 7.77 18l1.12-4.81-3.73-3.23 4.92-.42L12 5l1.92 4.53 4.92.42-3.73 3.23L16.23 18z",
            }
        }
    }
}

pub fn star_rate_18px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 18 18".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("18".to_string()),
            height: props.height.unwrap_or("18".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M9 11.3l3.71 2.7-1.42-4.36L15 7h-4.55L9 2.5 7.55 7H3l3.71 2.64L5.29 14z",
            }
            path {
                fill: "none",
                d: "M0 0h18v18H0z",
            }
        }
    }
}

pub fn star_rate_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                }
                polygon {
                    points: "14.43,10 12,2 9.57,10 2,10 8.18,14.41 5.83,22 12,17.31 18.18,22 15.83,14.41 22,10",
                }
            }
        }
    }
}

pub fn sticky_note_2_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19,3H4.99C3.89,3,3,3.9,3,5l0.01,14c0,1.1,0.89,2,1.99,2h10l6-6V5C21,3.9,20.1,3,19,3z M7,8h10v2H7V8z M12,14H7v-2h5V14z M14,19.5V14h5.5L14,19.5z",
            }
        }
    }
}

pub fn store_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 4H4v2h16V4zm1 10v-2l-1-5H4l-1 5v2h1v6h10v-6h4v6h2v-6h1zm-9 4H6v-4h6v4z",
            }
        }
    }
}

pub fn subject_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M14 17H4v2h10v-2zm6-8H4v2h16V9zM4 15h16v-2H4v2zM4 5v2h16V5H4z",
            }
        }
    }
}

pub fn subtitles_off_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M16,12v1h-1.59l2.46,2.46C16.95,15.32,17,15.17,17,15V5c0-0.55-0.45-1-1-1H5.41l8,8H16z",
                        }
                        path {
                            d: "M2.93,2.93L2.22,3.64l0.9,0.9C3.05,4.68,3,4.83,3,5v10c0,0.55,0.45,1,1,1h10.59l1.78,1.78l0.71-0.71L2.93,2.93z M8,12 h2.59l1,1H8V12z M4,12h3v1H4V12z M12,15H4v-1h8V15z",
                        }
                    }
                }
            }
        }
    }
}

pub fn subtitles_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,4H6.83l8,8H20v2h-3.17l4.93,4.93C21.91,18.65,22,18.34,22,18V6C22,4.9,21.1,4,20,4z",
                    }
                    path {
                        d: "M1.04,3.87l1.2,1.2C2.09,5.35,2,5.66,2,6v12c0,1.1,0.9,2,2,2h13.17l2.96,2.96l1.41-1.41L2.45,2.45L1.04,3.87z M8,12v2H4 v-2H8z M14,16.83V18H4v-2h9.17L14,16.83z",
                    }
                }
            }
        }
    }
}

pub fn supervised_user_circle_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M11.99 2c-5.52 0-10 4.48-10 10s4.48 10 10 10 10-4.48 10-10-4.48-10-10-10zm3.61 6.34c1.07 0 1.93.86 1.93 1.93 0 1.07-.86 1.93-1.93 1.93-1.07 0-1.93-.86-1.93-1.93-.01-1.07.86-1.93 1.93-1.93zm-6-1.58c1.3 0 2.36 1.06 2.36 2.36 0 1.3-1.06 2.36-2.36 2.36s-2.36-1.06-2.36-2.36c0-1.31 1.05-2.36 2.36-2.36zm0 9.13v3.75c-2.4-.75-4.3-2.6-5.14-4.96 1.05-1.12 3.67-1.69 5.14-1.69.53 0 1.2.08 1.9.22-1.64.87-1.9 2.02-1.9 2.68zM11.99 20c-.27 0-.53-.01-.79-.04v-4.07c0-1.42 2.94-2.13 4.4-2.13 1.07 0 2.92.39 3.84 1.15-1.17 2.97-4.06 5.09-7.45 5.09z",
            }
        }
    }
}

pub fn supervisor_account_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16.5 12c1.38 0 2.49-1.12 2.49-2.5S17.88 7 16.5 7C15.12 7 14 8.12 14 9.5s1.12 2.5 2.5 2.5zM9 11c1.66 0 2.99-1.34 2.99-3S10.66 5 9 5C7.34 5 6 6.34 6 8s1.34 3 3 3zm7.5 3c-1.83 0-5.5.92-5.5 2.75V19h11v-2.25c0-1.83-3.67-2.75-5.5-2.75zM9 13c-2.33 0-7 1.17-7 3.5V19h7v-2.25c0-.85.33-2.34 2.37-3.47C10.5 13.1 9.66 13 9 13z",
            }
        }
    }
}

pub fn support_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,3c-3.87,0-7,3.13-7,7c0,3.87,3.13,7,7,7s7-3.13,7-7C17,6.13,13.87,3,10,3z M12.16,4.41c1.57,0.61,2.82,1.85,3.43,3.42 l-2.78,1.15c-0.3-0.83-0.96-1.49-1.79-1.79L12.16,4.41z M7.84,4.41L9,7.18c-0.85,0.3-1.51,0.96-1.82,1.81v0L4.41,7.84 C5.02,6.27,6.27,5.02,7.84,4.41z M7.83,15.59c-1.57-0.61-2.82-1.86-3.43-3.43l2.78-1.15v0c0.3,0.84,0.97,1.51,1.81,1.81L7.83,15.59 z M8,10c0-1.1,0.9-2,2-2s2,0.9,2,2c0,1.1-0.9,2-2,2S8,11.1,8,10z M12.17,15.59l-1.15-2.78c0.84-0.3,1.5-0.97,1.79-1.81l2.77,1.16 C14.98,13.74,13.74,14.98,12.17,15.59z",
                }
            }
        }
    }
}

pub fn support_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M19.46,9.12l-2.78,1.15 c-0.51-1.36-1.58-2.44-2.95-2.94l1.15-2.78C16.98,5.35,18.65,7.02,19.46,9.12z M12,15c-1.66,0-3-1.34-3-3s1.34-3,3-3s3,1.34,3,3 S13.66,15,12,15z M9.13,4.54l1.17,2.78c-1.38,0.5-2.47,1.59-2.98,2.97L4.54,9.13C5.35,7.02,7.02,5.35,9.13,4.54z M4.54,14.87 l2.78-1.15c0.51,1.38,1.59,2.46,2.97,2.96l-1.17,2.78C7.02,18.65,5.35,16.98,4.54,14.87z M14.88,19.46l-1.15-2.78 c1.37-0.51,2.45-1.59,2.95-2.97l2.78,1.17C18.65,16.98,16.98,18.65,14.88,19.46z",
                }
            }
        }
    }
}

pub fn swap_horiz_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M6.99 11L3 15l3.99 4v-3H14v-2H6.99v-3zM21 9l-3.99-4v3H10v2h7.01v3L21 9z",
            }
        }
    }
}

pub fn swap_horizontal_circle_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M22 12c0-5.52-4.48-10-10-10S2 6.48 2 12s4.48 10 10 10 10-4.48 10-10zm-7-5.5l3.5 3.5-3.5 3.5V11h-4V9h4V6.5zm-6 11L5.5 14 9 10.5V13h4v2H9v2.5z",
            }
        }
    }
}

pub fn swap_vert_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16 17.01V10h-2v7.01h-3L15 21l4-3.99h-3zM9 3L5 6.99h3V14h2V6.99h3L9 3z",
            }
        }
    }
}

pub fn swap_vertical_circle_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM6.5 9L10 5.5 13.5 9H11v4H9V9H6.5zm11 6L14 18.5 10.5 15H13v-4h2v4h2.5z",
            }
        }
    }
}

pub fn swipe_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.51,3.49C14.87,1.96,12.56,1,10,1S5.13,1.96,3.49,3.49L2,2v4h4L4.69,4.69C6.02,3.47,7.91,2.7,10,2.7 s3.98,0.77,5.31,1.99L14,6h4V2L16.51,3.49z",
                    }
                    path {
                        d: "M12.16,11H10.5V6.25C10.5,5.56,9.94,5,9.25,5h0C8.56,5,8,5.56,8,6.25V15l-2.12-0.71c-0.54-0.18-1.13-0.04-1.54,0.36L4,15 l3.56,3.56C7.84,18.84,8.22,19,8.62,19h5.27c0.73,0,1.36-0.53,1.48-1.25l0.61-3.65c0.11-0.65-0.22-1.29-0.81-1.59L12.16,11z",
                    }
                }
            }
        }
    }
}

pub fn swipe_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                    fill: "none",
                    width: "24",
                }
            }
            g {
                g {
                    path {
                        d: "M18.89,14.75l-4.09-2.04c-0.28-0.14-0.58-0.21-0.89-0.21H13v-6C13,5.67,12.33,5,11.5,5S10,5.67,10,6.5v10.74L6.75,16.5 c-0.33-0.07-0.68,0.03-0.92,0.28L5,17.62l4.54,4.79C9.92,22.79,10.68,23,11.21,23h6.16c1,0,1.84-0.73,1.98-1.72l0.63-4.46 C20.1,15.97,19.66,15.14,18.89,14.75z",
                    }
                    path {
                        d: "M20.13,3.87C18.69,2.17,15.6,1,12,1S5.31,2.17,3.87,3.87L2,2v5h5L4.93,4.93c1-1.29,3.7-2.43,7.07-2.43 s6.07,1.14,7.07,2.43L17,7h5V2L20.13,3.87z",
                    }
                }
            }
        }
    }
}

pub fn swipe_down_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M4.4,10.51l1.54-1.54l0.71,0.71L3.83,12.5L1,9.67l0.71-0.71l1.69,1.69C3.14,9.82,3,8.93,3,8c0-2.31,0.87-4.41,2.29-6 L6,2.71C4.75,4.12,4,5.97,4,8C4,8.88,4.14,9.72,4.4,10.51z M14.02,8.84L12.5,9.52l-1.92-4.34c-0.28-0.63-1.02-0.92-1.65-0.64l0,0 C8.29,4.82,8.01,5.56,8.29,6.19l3.54,8L9.61,14.4c-0.57,0.05-1.05,0.42-1.26,0.95l-0.18,0.45l4.7,1.81 c0.37,0.14,0.78,0.13,1.15-0.03l4.82-2.13c0.67-0.3,1.03-1.04,0.84-1.75l-0.92-3.59C18.6,9.49,18.03,9.03,17.38,9L14.02,8.84z",
                }
            }
        }
    }
}

pub fn swipe_down_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M3.8,12.18c-0.2-0.86-0.3-1.76-0.3-2.68c0-2.84,0.99-5.45,2.63-7.5L7.2,3.07C5.82,4.85,5,7.08,5,9.5 c0,0.88,0.11,1.74,0.32,2.56l1.62-1.62L8,11.5L4.5,15L1,11.5l1.06-1.06L3.8,12.18z M13.85,11.62l-2.68-5.37 c-0.37-0.74-1.27-1.04-2.01-0.67C8.41,5.96,8.11,6.86,8.48,7.6l4.81,9.6L10.05,18c-0.33,0.09-0.59,0.33-0.7,0.66L9,19.78l6.19,2.25 c0.5,0.17,1.28,0.02,1.75-0.22l5.51-2.75c0.89-0.45,1.32-1.48,1-2.42l-1.43-4.27c-0.27-0.82-1.04-1.37-1.9-1.37h-4.56 c-0.31,0-0.62,0.07-0.89,0.21L13.85,11.62",
                }
            }
        }
    }
}

pub fn swipe_down_alt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.75,11.93C12.6,11.58,14,9.95,14,8c0-2.21-1.79-4-4-4C7.79,4,6,5.79,6,8c0,1.95,1.4,3.58,3.25,3.93l0,3.2l-1.19-1.19 L7,15l3,3l3-3l-1.06-1.06l-1.19,1.19L10.75,11.93z",
                }
            }
        }
    }
}

pub fn swipe_down_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13,13.9c2.28-0.46,4-2.48,4-4.9c0-2.76-2.24-5-5-5S7,6.24,7,9c0,2.42,1.72,4.44,4,4.9v4.27l-1.59-1.59L8,18l4,4l4-4 l-1.41-1.41L13,18.17V13.9z",
                }
            }
        }
    }
}

pub fn swipe_left_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.93,6C15.89,3.63,13.11,2,10,2C7.35,2,4.94,3.18,3.64,5L6,5v1H2V2h1l0,2.19C4.49,2.27,7.07,1,10,1c3.78,0,6.97,2.11,8,5 L16.93,6z M12.66,11H11V6.25C11,5.56,10.44,5,9.75,5h0C9.06,5,8.5,5.56,8.5,6.25V15l-2.12-0.71c-0.54-0.18-1.13-0.04-1.54,0.36 L4.5,15l3.56,3.56C8.34,18.84,8.72,19,9.12,19h5.27c0.73,0,1.36-0.53,1.48-1.25l0.61-3.65c0.11-0.65-0.22-1.29-0.81-1.59L12.66,11z",
                }
            }
        }
    }
}

pub fn swipe_left_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19.98,16.82l-0.63,4.46C19.21,22.27,18.36,23,17.37,23h-6.16c-0.53,0-1.29-0.21-1.66-0.59L5,17.62l0.83-0.84 c0.24-0.24,0.58-0.35,0.92-0.28L10,17.24V6.5C10,5.67,10.67,5,11.5,5S13,5.67,13,6.5v6h0.91c0.31,0,0.62,0.07,0.89,0.21l4.09,2.04 C19.66,15.14,20.1,15.97,19.98,16.82z M12,2.5c4.74,0,7.67,2.52,8.43,4.5H22c-0.73-2.88-4.51-6-10-6C8.78,1,5.82,2.13,3.5,4.02V2H2 v5h5V5.5H4.09C6.21,3.64,8.97,2.5,12,2.5z",
                }
            }
        }
    }
}

pub fn swipe_left_alt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M8.07,10.75C8.42,12.6,10.05,14,12,14c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4c-1.95,0-3.58,1.4-3.93,3.25l-3.2,0l1.19-1.19 L5,7l-3,3l3,3l1.06-1.06l-1.19-1.19L8.07,10.75z",
                }
            }
        }
    }
}

pub fn swipe_left_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.1,13c0.46,2.28,2.48,4,4.9,4c2.76,0,5-2.24,5-5s-2.24-5-5-5c-2.42,0-4.44,1.72-4.9,4H5.83l1.59-1.59L6,8l-4,4l4,4 l1.41-1.41L5.83,13H10.1z",
                }
            }
        }
    }
}

pub fn swipe_right_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M2,6c1.03-2.89,4.22-5,8-5c2.93,0,5.51,1.27,7,3.19L17,2h1v4h-4V5l2.36,0C15.06,3.18,12.65,2,10,2C6.89,2,4.11,3.63,3.07,6 L2,6z M12.66,11H11V6.25C11,5.56,10.44,5,9.75,5h0C9.06,5,8.5,5.56,8.5,6.25V15l-2.12-0.71c-0.54-0.18-1.13-0.04-1.54,0.36L4.5,15 l3.56,3.56C8.34,18.84,8.72,19,9.12,19h5.27c0.73,0,1.36-0.53,1.48-1.25l0.61-3.65c0.11-0.65-0.22-1.29-0.81-1.59L12.66,11z",
                }
            }
        }
    }
}

pub fn swipe_right_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19.98,16.82l-0.63,4.46C19.21,22.27,18.36,23,17.37,23h-6.16c-0.53,0-1.29-0.21-1.66-0.59L5,17.62l0.83-0.84 c0.24-0.24,0.58-0.35,0.92-0.28L10,17.24V6.5C10,5.67,10.67,5,11.5,5S13,5.67,13,6.5v6h0.91c0.31,0,0.62,0.07,0.89,0.21l4.09,2.04 C19.66,15.14,20.1,15.97,19.98,16.82z M19.91,5.5H17V7h5V2h-1.5v2.02C18.18,2.13,15.22,1,12,1C6.51,1,2.73,4.12,2,7h1.57 C4.33,5.02,7.26,2.5,12,2.5C15.03,2.5,17.79,3.64,19.91,5.5z",
                }
            }
        }
    }
}

pub fn swipe_right_alt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M11.93,9.25C11.58,7.4,9.95,6,8,6c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4c1.95,0,3.58-1.4,3.93-3.25l3.2,0l-1.19,1.19L15,13 l3-3l-3-3l-1.06,1.06l1.19,1.19L11.93,9.25z",
                }
            }
        }
    }
}

pub fn swipe_right_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13.9,11C13.44,8.72,11.42,7,9,7c-2.76,0-5,2.24-5,5s2.24,5,5,5c2.42,0,4.44-1.72,4.9-4h4.27l-1.59,1.59L18,16l4-4l-4-4 l-1.41,1.41L18.17,11H13.9z",
                }
            }
        }
    }
}

pub fn swipe_up_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M4,5.5c0,2.03,0.75,3.88,2,5.29L5.29,11.5C3.87,9.91,3,7.81,3,5.5c0-0.93,0.14-1.82,0.4-2.66L1.71,4.53L1,3.83L3.83,1 l2.83,2.83L5.94,4.53L4.4,2.99C4.14,3.78,4,4.62,4,5.5z M14.02,8.84L12.5,9.52l-1.92-4.34c-0.28-0.63-1.02-0.92-1.65-0.64l0,0 C8.29,4.82,8.01,5.56,8.29,6.19l3.54,8L9.61,14.4c-0.57,0.05-1.05,0.42-1.26,0.95l-0.18,0.45l4.7,1.81 c0.37,0.14,0.78,0.13,1.15-0.03l4.82-2.13c0.67-0.3,1.03-1.04,0.84-1.75l-0.92-3.59C18.6,9.49,18.03,9.03,17.38,9L14.02,8.84z",
                }
            }
        }
    }
}

pub fn swipe_up_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M2.06,5.56L1,4.5L4.5,1L8,4.5L6.94,5.56L5.32,3.94C5.11,4.76,5,5.62,5,6.5c0,2.42,0.82,4.65,2.2,6.43L6.13,14 C4.49,11.95,3.5,9.34,3.5,6.5c0-0.92,0.1-1.82,0.3-2.68L2.06,5.56z M13.85,11.62l-2.68-5.37c-0.37-0.74-1.27-1.04-2.01-0.67 C8.41,5.96,8.11,6.86,8.48,7.6l4.81,9.6L10.05,18c-0.33,0.09-0.59,0.33-0.7,0.66L9,19.78l6.19,2.25c0.5,0.17,1.28,0.02,1.75-0.22 l5.51-2.75c0.89-0.45,1.32-1.48,1-2.42l-1.43-4.27c-0.27-0.82-1.04-1.37-1.9-1.37h-4.56c-0.31,0-0.62,0.07-0.89,0.21L13.85,11.62",
                }
            }
        }
    }
}

pub fn swipe_up_alt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M9.25,8.07C7.4,8.42,6,10.05,6,12c0,2.21,1.79,4,4,4c2.21,0,4-1.79,4-4c0-1.95-1.4-3.58-3.25-3.93l0-3.2l1.19,1.19L13,5 l-3-3L7,5l1.06,1.06l1.19-1.19L9.25,8.07z",
                }
            }
        }
    }
}

pub fn swipe_up_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13,5.83l1.59,1.59L16,6l-4-4L8,6l1.41,1.41L11,5.83v4.27c-2.28,0.46-4,2.48-4,4.9c0,2.76,2.24,5,5,5s5-2.24,5-5 c0-2.42-1.72-4.44-4-4.9V5.83z",
                }
            }
        }
    }
}

pub fn swipe_vertical_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14.02,8.84L12.5,9.52l-1.92-4.34c-0.28-0.63-1.02-0.92-1.65-0.64l0,0C8.29,4.82,8.01,5.56,8.29,6.19l3.54,8L9.61,14.4 c-0.57,0.05-1.05,0.42-1.26,0.95l-0.18,0.45l4.7,1.81c0.37,0.14,0.78,0.13,1.15-0.03l4.82-2.13c0.67-0.3,1.03-1.04,0.84-1.75 l-0.92-3.59C18.6,9.49,18.03,9.03,17.38,9L14.02,8.84z M1,10c0,2.69,1.2,5.05,3,6.37V14h1v4H1v-1h2.18C1.26,15.47,0,12.91,0,10 s1.26-5.47,3.18-7H1V2h4v4H4V3.63C2.2,4.95,1,7.31,1,10z",
                }
            }
        }
    }
}

pub fn swipe_vertical_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M1,3.5h2.02C1.13,5.82,0,8.78,0,12s1.13,6.18,3.02,8.5H1V22h5v-5H4.5v2.91c-1.86-2.11-3-4.88-3-7.91s1.14-5.79,3-7.91V7H6 V2H1V3.5z M13.85,11.62l-2.68-5.37c-0.37-0.74-1.27-1.04-2.01-0.67C8.41,5.96,8.11,6.86,8.48,7.6l4.81,9.6L10.05,18 c-0.33,0.09-0.59,0.33-0.7,0.66L9,19.78l6.19,2.25c0.5,0.17,1.28,0.02,1.75-0.22l5.51-2.75c0.89-0.45,1.32-1.48,1-2.42l-1.43-4.27 c-0.27-0.82-1.04-1.37-1.9-1.37h-4.56c-0.31,0-0.62,0.07-0.89,0.21L13.85,11.62",
                }
            }
        }
    }
}

pub fn switch_access_shortcut_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M6.22,7.28L4.5,6.5l1.72-0.78L7,4l0.78,1.72L9.5,6.5L7.78,7.28L7,9L6.22,7.28z M7,17l0.78-1.72L9.5,14.5l-1.72-0.78L7,12 l-0.78,1.72L4.5,14.5l1.72,0.78L7,17z M4.5,12l0.47-1.03L6,10.5l-1.03-0.47L4.5,9l-0.47,1.03L3,10.5l1.03,0.47L4.5,12z M15.5,18 c-3.21-1.23-5.5-4.36-5.5-8c0-2.2,0.83-4.24,2.2-5.8L10,2h5.5v5.5l-2.23-2.23C12.17,6.55,11.5,8.21,11.5,10c0,2.81,1.64,5.23,4,6.37 L15.5,18z",
            }
        }
    }
}

pub fn switch_access_shortcut_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7.06,8.94L5,8l2.06-0.94L8,5l0.94,2.06L11,8L8.94,8.94L8,11L7.06,8.94z M8,21l0.94-2.06L11,18l-2.06-0.94L8,15l-0.94,2.06 L5,18l2.06,0.94L8,21z M4.37,12.37L3,13l1.37,0.63L5,15l0.63-1.37L7,13l-1.37-0.63L5,11L4.37,12.37z M12,12 c0-2.73,1.08-5.27,2.75-7.25L12,2h7v7l-2.82-2.82C14.84,7.82,14,9.88,14,12c0,3.32,2.1,6.36,5,7.82L19,22 C14.91,20.41,12,16.35,12,12z",
            }
        }
    }
}

pub fn switch_access_shortcut_add_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M6.22,7.28L4.5,6.5l1.72-0.78L7,4l0.78,1.72L9.5,6.5L7.78,7.28L7,9L6.22,7.28z M7,17l0.78-1.72L9.5,14.5l-1.72-0.78L7,12 l-0.78,1.72L4.5,14.5l1.72,0.78L7,17z M4.5,12l0.47-1.03L6,10.5l-1.03-0.47L4.5,9l-0.47,1.03L3,10.5l1.03,0.47L4.5,12z M15.5,18 c-3.21-1.23-5.5-4.36-5.5-8c0-2.2,0.83-4.24,2.2-5.8L10,2h5.5v5.5l-2.23-2.23C12.17,6.55,11.5,8.21,11.5,10c0,2.81,1.64,5.23,4,6.37 L15.5,18z M20,11.5h-1.5V10H17v1.5h-1.5V13H17v1.5h1.5V13H20V11.5z",
            }
        }
    }
}

pub fn switch_access_shortcut_add_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M24,14h-2v-2h-2v2h-2v2h2v2h2v-2h2V14z M7.06,8.94L5,8l2.06-0.94L8,5l0.94,2.06L11,8L8.94,8.94L8,11L7.06,8.94z M8,21 l0.94-2.06L11,18l-2.06-0.94L8,15l-0.94,2.06L5,18l2.06,0.94L8,21z M4.37,12.37L3,13l1.37,0.63L5,15l0.63-1.37L7,13l-1.37-0.63L5,11 L4.37,12.37z M12,12c0-2.73,1.08-5.27,2.75-7.25L12,2h7v7l-2.82-2.82C14.84,7.82,14,9.88,14,12c0,3.32,2.1,6.36,5,7.82L19,22 C14.91,20.41,12,16.35,12,12z",
            }
        }
    }
}

pub fn sync_alt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    polygon {
                        points: "18,7 15,4 15,6.25 3,6.25 3,7.75 15,7.75 15,10",
                    }
                    polygon {
                        points: "5,10 2,13 5,16 5,13.75 17,13.75 17,12.25 5,12.25",
                    }
                }
            }
        }
    }
}

pub fn sync_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    x: "0",
                    fill: "none",
                    height: "24",
                    width: "24",
                }
            }
            g {
                g {
                    polygon {
                        points: "18,12 22,8 18,4 18,7 3,7 3,9 18,9",
                    }
                    polygon {
                        points: "6,12 2,16 6,20 6,17 21,17 21,15 6,15",
                    }
                }
            }
        }
    }
}

pub fn system_update_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M0 .5h24v24H0z",
                fill: "none",
            }
            path {
                d: "M12 16.5l4-4h-3v-9h-2v9H8l4 4zm9-13h-6v1.99h6v14.03H3V5.49h6V3.5H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2v-14c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn tab_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H3V5h10v4h8v10z",
            }
        }
    }
}

pub fn table_view_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,7H9C7.9,7,7,7.9,7,9v10c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V9C21,7.9,20.1,7,19,7z M19,9v2H9V9H19z M13,15v-2h2v2H13z M15,17v2h-2v-2H15z M11,15H9v-2h2V15z M17,13h2v2h-2V13z M9,17h2v2H9V17z M17,19v-2h2v2H17z M6,17H5c-1.1,0-2-0.9-2-2V5 c0-1.1,0.9-2,2-2h10c1.1,0,2,0.9,2,2v1h-2V5H5v10h1V17z",
                }
            }
        }
    }
}

pub fn tab_unselected_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M1 9h2V7H1v2zm0 4h2v-2H1v2zm0-8h2V3c-1.1 0-2 .9-2 2zm8 16h2v-2H9v2zm-8-4h2v-2H1v2zm2 4v-2H1c0 1.1.9 2 2 2zM21 3h-8v6h10V5c0-1.1-.9-2-2-2zm0 14h2v-2h-2v2zM9 5h2V3H9v2zM5 21h2v-2H5v2zM5 5h2V3H5v2zm16 16c1.1 0 2-.9 2-2h-2v2zm0-8h2v-2h-2v2zm-8 8h2v-2h-2v2zm4 0h2v-2h-2v2z",
            }
        }
    }
}

pub fn task_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22,5.18L10.59,16.6l-4.24-4.24l1.41-1.41l2.83,2.83l10-10L22,5.18z M19.79,10.22C19.92,10.79,20,11.39,20,12 c0,4.42-3.58,8-8,8s-8-3.58-8-8c0-4.42,3.58-8,8-8c1.58,0,3.04,0.46,4.28,1.25l1.44-1.44C16.1,2.67,14.13,2,12,2C6.48,2,2,6.48,2,12 c0,5.52,4.48,10,10,10s10-4.48,10-10c0-1.19-0.22-2.33-0.6-3.39L19.79,10.22z",
            }
        }
    }
}

pub fn terminal_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-9C18,4.67,17.33,4,16.5,4z M3.5,14.5V7h13v7.5H3.5z M15,13.5h-5V12h5V13.5z M6.25,13.5l-1.06-1.06l1.69-1.69L5.19,9.06L6.25,8L9,10.75L6.25,13.5z",
                }
            }
        }
    }
}

pub fn terminal_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,4H4C2.89,4,2,4.9,2,6v12c0,1.1,0.89,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.11,4,20,4z M20,18H4V8h16V18z M18,17h-6v-2 h6V17z M7.5,17l-1.41-1.41L8.67,13l-2.59-2.59L7.5,9l4,4L7.5,17z",
                }
            }
        }
    }
}

pub fn text_rotate_up_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 12v1.5l11 4.75v-2.1l-2.2-.9v-5l2.2-.9v-2.1L3 12zm7 2.62l-5.02-1.87L10 10.88v3.74zm8-10.37l-3 3h2v12.5h2V7.25h2l-3-3z",
            }
        }
    }
}

pub fn text_rotate_vertical_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15.75 5h-1.5L9.5 16h2.1l.9-2.2h5l.9 2.2h2.1L15.75 5zm-2.62 7L15 6.98 16.87 12h-3.74zM6 19.75l3-3H7V4.25H5v12.5H3l3 3z",
            }
        }
    }
}

pub fn text_rotation_angledown_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19.4 4.91l-1.06-1.06L7.2 8.27l1.48 1.48 2.19-.92 3.54 3.54-.92 2.19 1.48 1.48L19.4 4.91zm-6.81 3.1l4.87-2.23-2.23 4.87-2.64-2.64zM14.27 21v-4.24l-1.41 1.41-8.84-8.84-1.42 1.42 8.84 8.84L10.03 21h4.24z",
            }
        }
    }
}

pub fn text_rotation_angleup_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M4.49 4.21L3.43 5.27 7.85 16.4l1.48-1.48-.92-2.19 3.54-3.54 2.19.92 1.48-1.48L4.49 4.21zm3.09 6.8L5.36 6.14l4.87 2.23-2.65 2.64zm12.99-1.68h-4.24l1.41 1.41-8.84 8.84L10.32 21l8.84-8.84 1.41 1.41V9.33z",
            }
        }
    }
}

pub fn text_rotation_down_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21 12v-1.5L10 5.75v2.1l2.2.9v5l-2.2.9v2.1L21 12zm-7-2.62l5.02 1.87L14 13.12V9.38zM6 19.75l3-3H7V4.25H5v12.5H3l3 3z",
            }
        }
    }
}

pub fn text_rotation_none_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12.75 3h-1.5L6.5 14h2.1l.9-2.2h5l.9 2.2h2.1L12.75 3zm-2.62 7L12 4.98 13.87 10h-3.74zm10.37 8l-3-3v2H5v2h12.5v2l3-3z",
            }
        }
    }
}

pub fn theaters_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18 3v2h-2V3H8v2H6V3H4v18h2v-2h2v2h8v-2h2v2h2V3h-2zM8 17H6v-2h2v2zm0-4H6v-2h2v2zm0-4H6V7h2v2zm10 8h-2v-2h2v2zm0-4h-2v-2h2v2zm0-4h-2V7h2v2z",
            }
        }
    }
}

pub fn thumbs_up_down_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 6c0-.55-.45-1-1-1H5.82l.66-3.18.02-.23c0-.31-.13-.59-.33-.8L5.38 0 .44 4.94C.17 5.21 0 5.59 0 6v6.5c0 .83.67 1.5 1.5 1.5h6.75c.62 0 1.15-.38 1.38-.91l2.26-5.29c.07-.17.11-.36.11-.55V6zm10.5 4h-6.75c-.62 0-1.15.38-1.38.91l-2.26 5.29c-.07.17-.11.36-.11.55V18c0 .55.45 1 1 1h5.18l-.66 3.18-.02.24c0 .31.13.59.33.8l.79.78 4.94-4.94c.27-.27.44-.65.44-1.06v-6.5c0-.83-.67-1.5-1.5-1.5z",
            }
        }
    }
}

pub fn thumb_down_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15 3H6c-.83 0-1.54.5-1.84 1.22l-3.02 7.05c-.09.23-.14.47-.14.73v2c0 1.1.9 2 2 2h6.31l-.95 4.57-.03.32c0 .41.17.79.44 1.06L9.83 23l6.59-6.59c.36-.36.58-.86.58-1.41V5c0-1.1-.9-2-2-2zm4 0v12h4V3h-4z",
            }
        }
    }
}

pub fn thumb_down_off_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M24 24H0V0h24v24z",
            }
            path {
                d: "M10.89 18.28l.57-2.89c.12-.59-.04-1.2-.42-1.66-.38-.46-.94-.73-1.54-.73H4v-1.08L6.57 6h8.09c.18 0 .34.16.34.34v7.84l-4.11 4.1M10 22l6.41-6.41c.38-.38.59-.89.59-1.42V6.34C17 5.05 15.95 4 14.66 4h-8.1c-.71 0-1.36.37-1.72.97l-2.67 6.15c-.11.25-.17.52-.17.8V13c0 1.1.9 2 2 2h5.5l-.92 4.65c-.05.22-.02.46.08.66.23.45.52.86.88 1.22L10 22zm10-7h2V4h-2c-.55 0-1 .45-1 1v9c0 .55.45 1 1 1z",
            }
        }
    }
}

pub fn thumb_up_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M1 21h4V9H1v12zm22-11c0-1.1-.9-2-2-2h-6.31l.95-4.57.03-.32c0-.41-.17-.79-.44-1.06L14.17 1 7.59 7.59C7.22 7.95 7 8.45 7 9v10c0 1.1.9 2 2 2h9c.83 0 1.54-.5 1.84-1.22l3.02-7.05c.09-.23.14-.47.14-.73v-2z",
            }
        }
    }
}

pub fn thumb_up_off_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M13.11 5.72l-.57 2.89c-.12.59.04 1.2.42 1.66.38.46.94.73 1.54.73H20v1.08L17.43 18H9.34c-.18 0-.34-.16-.34-.34V9.82l4.11-4.1M14 2L7.59 8.41C7.21 8.79 7 9.3 7 9.83v7.83C7 18.95 8.05 20 9.34 20h8.1c.71 0 1.36-.37 1.72-.97l2.67-6.15c.11-.25.17-.52.17-.8V11c0-1.1-.9-2-2-2h-5.5l.92-4.65c.05-.22.02-.46-.08-.66-.23-.45-.52-.86-.88-1.22L14 2zM4 9H2v11h2c.55 0 1-.45 1-1v-9c0-.55-.45-1-1-1z",
            }
        }
    }
}

pub fn timeline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M23,8c0,1.1-0.9,2-2,2c-0.18,0-0.35-0.02-0.51-0.07l-3.56,3.55C16.98,13.64,17,13.82,17,14c0,1.1-0.9,2-2,2s-2-0.9-2-2 c0-0.18,0.02-0.36,0.07-0.52l-2.55-2.55C10.36,10.98,10.18,11,10,11s-0.36-0.02-0.52-0.07l-4.55,4.56C4.98,15.65,5,15.82,5,16 c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2c0.18,0,0.35,0.02,0.51,0.07l4.56-4.55C8.02,9.36,8,9.18,8,9c0-1.1,0.9-2,2-2s2,0.9,2,2 c0,0.18-0.02,0.36-0.07,0.52l2.55,2.55C14.64,12.02,14.82,12,15,12s0.36,0.02,0.52,0.07l3.55-3.56C19.02,8.35,19,8.18,19,8 c0-1.1,0.9-2,2-2S23,6.9,23,8z",
                        }
                    }
                }
            }
        }
    }
}

pub fn tips_and_updates_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M4.5,14h6v1.5h-6V14z M13.5,8c0,2.09-1.07,3.93-2.69,5H4.19C2.57,11.93,1.5,10.09,1.5,8c0-3.31,2.69-6,6-6S13.5,4.69,13.5,8 z M7.5,18C8.33,18,9,17.33,9,16.5H6C6,17.33,6.67,18,7.5,18z M18.5,8l0.47-1.03L20,6.5l-1.03-0.47L18.5,5l-0.47,1.03L17,6.5 l1.03,0.47L18.5,8z M15.5,5l0.78-1.72L18,2.5l-1.72-0.78L15.5,0l-0.78,1.72L13,2.5l1.72,0.78L15.5,5z",
            }
        }
    }
}

pub fn tips_and_updates_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                y: "0",
            }
            path {
                d: "M7,20h4c0,1.1-0.9,2-2,2S7,21.1,7,20z M5,19h8v-2H5V19z M16.5,9.5c0,3.82-2.66,5.86-3.77,6.5H5.27 C4.16,15.36,1.5,13.32,1.5,9.5C1.5,5.36,4.86,2,9,2S16.5,5.36,16.5,9.5z M21.37,7.37L20,8l1.37,0.63L22,10l0.63-1.37L24,8 l-1.37-0.63L22,6L21.37,7.37z M19,6l0.94-2.06L22,3l-2.06-0.94L19,0l-0.94,2.06L16,3l2.06,0.94L19,6z",
            }
        }
    }
}

pub fn toc_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 9h14V7H3v2zm0 4h14v-2H3v2zm0 4h14v-2H3v2zm16 0h2v-2h-2v2zm0-10v2h2V7h-2zm0 6h2v-2h-2v2z",
            }
        }
    }
}

pub fn today_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zM7 10h5v5H7z",
            }
        }
    }
}

pub fn token_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                y: "0",
                width: "20",
                fill: "none",
            }
            path {
                d: "M15.98,5.82L10,2.5L4.02,5.82l3.8,2.11C8.37,7.36,9.14,7,10,7s1.63,0.36,2.17,0.93L15.98,5.82z M8.5,10 c0-0.83,0.67-1.5,1.5-1.5s1.5,0.67,1.5,1.5s-0.67,1.5-1.5,1.5S8.5,10.83,8.5,10z M9.25,17.08l-6-3.33V7.11L7.1,9.24 C7.03,9.49,7,9.74,7,10c0,1.4,0.96,2.57,2.25,2.91V17.08z M10.75,17.08v-4.18C12.04,12.57,13,11.4,13,10c0-0.26-0.03-0.51-0.1-0.76 l3.85-2.14l0,6.64L10.75,17.08z",
            }
        }
    }
}

pub fn token_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                y: "0",
            }
            path {
                d: "M19.97,6.43L12,2L4.03,6.43L9.1,9.24C9.83,8.48,10.86,8,12,8s2.17,0.48,2.9,1.24L19.97,6.43z M10,12c0-1.1,0.9-2,2-2 s2,0.9,2,2s-0.9,2-2,2S10,13.1,10,12z M11,21.44L3,17V8.14l5.13,2.85C8.04,11.31,8,11.65,8,12c0,1.86,1.27,3.43,3,3.87V21.44z M13,21.44v-5.57c1.73-0.44,3-2.01,3-3.87c0-0.35-0.04-0.69-0.13-1.01L21,8.14L21,17L13,21.44z",
            }
        }
    }
}

pub fn toll_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15 4c-4.42 0-8 3.58-8 8s3.58 8 8 8 8-3.58 8-8-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6s2.69-6 6-6 6 2.69 6 6-2.69 6-6 6z",
            }
            path {
                d: "M3 12c0-2.61 1.67-4.83 4-5.65V4.26C3.55 5.15 1 8.27 1 12s2.55 6.85 6 7.74v-2.09c-2.33-.82-4-3.04-4-5.65z",
            }
        }
    }
}

pub fn touch_app_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                    width: "24",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M9,11.24V7.5C9,6.12,10.12,5,11.5,5S14,6.12,14,7.5v3.74c1.21-0.81,2-2.18,2-3.74C16,5.01,13.99,3,11.5,3S7,5.01,7,7.5 C7,9.06,7.79,10.43,9,11.24z M18.84,15.87l-4.54-2.26c-0.17-0.07-0.35-0.11-0.54-0.11H13v-6C13,6.67,12.33,6,11.5,6 S10,6.67,10,7.5v10.74c-3.6-0.76-3.54-0.75-3.67-0.75c-0.31,0-0.59,0.13-0.79,0.33l-0.79,0.8l4.94,4.94 C9.96,23.83,10.34,24,10.75,24h6.79c0.75,0,1.33-0.55,1.44-1.28l0.75-5.27c0.01-0.07,0.02-0.14,0.02-0.2 C19.75,16.63,19.37,16.09,18.84,15.87z",
                        }
                    }
                }
            }
        }
    }
}

pub fn tour_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M21 4H7V2H5v20h2v-8h14l-2-5 2-5zm-6 5c0 1.1-.9 2-2 2s-2-.9-2-2 .9-2 2-2 2 .9 2 2z",
            }
        }
    }
}

pub fn track_changes_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M19.07 4.93l-1.41 1.41C19.1 7.79 20 9.79 20 12c0 4.42-3.58 8-8 8s-8-3.58-8-8c0-4.08 3.05-7.44 7-7.93v2.02C8.16 6.57 6 9.03 6 12c0 3.31 2.69 6 6 6s6-2.69 6-6c0-1.66-.67-3.16-1.76-4.24l-1.41 1.41C15.55 9.9 16 10.9 16 12c0 2.21-1.79 4-4 4s-4-1.79-4-4c0-1.86 1.28-3.41 3-3.86v2.14c-.6.35-1 .98-1 1.72 0 1.1.9 2 2 2s2-.9 2-2c0-.74-.4-1.38-1-1.72V2h-1C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10c0-2.76-1.12-5.26-2.93-7.07z",
            }
        }
    }
}

pub fn transcribe_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M8,11c1.66,0,3-1.34,3-3S9.66,5,8,5S5,6.34,5,8S6.34,11,8,11z",
                    }
                    path {
                        d: "M13.03,13.37C11.56,12.5,9.84,12,8,12s-3.56,0.5-5.03,1.37C2.36,13.72,2,14.39,2,15.09V17h12v-1.91 C14,14.39,13.64,13.72,13.03,13.37z",
                    }
                    path {
                        d: "M15.16,12.61l0.97-1.17C15.14,10.61,14.5,9.39,14.5,8s0.64-2.61,1.63-3.44l-0.97-1.17C13.84,4.49,13,6.15,13,8 S13.84,11.51,15.16,12.61z",
                    }
                    path {
                        d: "M17.09,5.71C16.43,6.26,16,7.07,16,8s0.43,1.74,1.09,2.29l0.96-1.15C17.71,8.87,17.5,8.46,17.5,8s0.21-0.87,0.54-1.15 L17.09,5.71z",
                    }
                }
            }
        }
    }
}

pub fn transcribe_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17.93,16l1.63-1.63c-2.77-3.02-2.77-7.56,0-10.74L17.93,2C14.03,5.89,14.02,11.95,17.93,16z M22.92,10.95 c-0.84-1.18-0.84-2.71,0-3.89l-1.68-1.69c-2.02,2.02-2.02,5.07,0,7.27L22.92,10.95z M9,13c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4 S5,6.79,5,9C5,11.21,6.79,13,9,13z M15.39,15.56C13.71,14.7,11.53,14,9,14c-2.53,0-4.71,0.7-6.39,1.56C1.61,16.07,1,17.1,1,18.22 V21h16v-2.78C17,17.1,16.39,16.07,15.39,15.56z",
                }
            }
        }
    }
}

pub fn translate_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12.87 15.07l-2.54-2.51.03-.03c1.74-1.94 2.98-4.17 3.71-6.53H17V4h-7V2H8v2H1v1.99h11.17C11.5 7.92 10.44 9.75 9 11.35 8.07 10.32 7.3 9.19 6.69 8h-2c.73 1.63 1.73 3.17 2.98 4.56l-5.09 5.02L4 19l5-5 3.11 3.11.76-2.04zM18.5 10h-2L12 22h2l1.12-3h4.75L21 22h2l-4.5-12zm-2.62 7l1.62-4.33L19.12 17h-3.24z",
            }
        }
    }
}

pub fn trending_down_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16 18l2.29-2.29-4.88-4.88-4 4L2 7.41 3.41 6l6 6 4-4 6.3 6.29L22 12v6z",
            }
        }
    }
}

pub fn trending_flat_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M22 12l-4-4v3H3v2h15v3z",
            }
        }
    }
}

pub fn trending_up_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M16 6l2.29 2.29-4.88 4.88-4-4L2 16.59 3.41 18l6-6 4 4 6.3-6.29L22 12V6z",
            }
        }
    }
}

pub fn troubleshoot_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18.01,16.94l-4.27-4.27C14.53,11.65,15,10.38,15,9c0-3.31-2.69-6-6-6C6.03,3,3.57,5.16,3.09,8h1.53 C5.08,6,6.86,4.5,9,4.5c2.48,0,4.5,2.02,4.5,4.5s-2.02,4.5-4.5,4.5c-1.76,0-3.27-1.02-4.01-2.5H3.35c0.82,2.33,3.04,4,5.65,4 c1.39,0,2.66-0.48,3.68-1.27L16.95,18L18.01,16.94z",
                    }
                    polygon {
                        points: "7,7.56 8.11,12 9.36,12 10.29,9.2 11.19,11 12.5,11 12.5,10 11.81,10 10.81,8 9.64,8 8.78,10.57 7.64,6 6.36,6 5.61,9 1,9 1,10 6.39,10",
                    }
                }
            }
        }
    }
}

pub fn troubleshoot_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M22,20.59l-4.69-4.69C18.37,14.55,19,12.85,19,11c0-4.42-3.58-8-8-8c-4.08,0-7.44,3.05-7.93,7h2.02C5.57,7.17,8.03,5,11,5 c3.31,0,6,2.69,6,6s-2.69,6-6,6c-2.42,0-4.5-1.44-5.45-3.5H3.4C4.45,16.69,7.46,19,11,19c1.85,0,3.55-0.63,4.9-1.69L20.59,22 L22,20.59z",
                    }
                    polygon {
                        points: "8.43,9.69 9.65,15 11.29,15 12.55,11.22 13.5,13.5 15.5,13.5 15.5,12 14.5,12 13.25,9 11.71,9 10.59,12.37 9.35,7 7.7,7 6.45,11 1,11 1,12.5 7.55,12.5",
                    }
                }
            }
        }
    }
}

pub fn try_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                    y: "0",
                }
            }
            g {
                path {
                    d: "M20,2H4C2.9,2,2,2.9,2,4v18l4-4h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M13.57,11.57L12,15l-1.57-3.43L7,10l3.43-1.57 L12,5l1.57,3.43L17,10L13.57,11.57z",
                }
            }
        }
    }
}

pub fn turned_in_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2z",
            }
        }
    }
}

pub fn turned_in_not_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M17 3H7c-1.1 0-1.99.9-1.99 2L5 21l7-3 7 3V5c0-1.1-.9-2-2-2zm0 15l-5-2.18L7 18V5h10v13z",
            }
        }
    }
}

pub fn unfold_less_double_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "13.37,1.06 12.31,0 10,2.31 7.69,0 6.63,1.06 10,4.43",
                    }
                    polygon {
                        points: "13.37,5.06 12.31,4 10,6.31 7.69,4 6.63,5.06 10,8.43",
                    }
                    polygon {
                        points: "6.63,14.94 7.69,16 10,13.69 12.31,16 13.37,14.94 10,11.57",
                    }
                    polygon {
                        points: "6.63,18.94 7.69,20 10,17.69 12.31,20 13.37,18.94 10,15.57",
                    }
                }
            }
        }
    }
}

pub fn unfold_less_double_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "16.58,1.41 15.16,0 11.99,3.17 8.82,0 7.41,1.41 11.99,6",
                    }
                    polygon {
                        points: "16.58,6.41 15.16,5 11.99,8.17 8.82,5 7.41,6.41 11.99,11",
                    }
                    polygon {
                        points: "7.42,17.59 8.84,19 12.01,15.83 15.18,19 16.59,17.59 12.01,13",
                    }
                    polygon {
                        points: "7.42,22.59 8.84,24 12.01,20.83 15.18,24 16.59,22.59 12.01,18",
                    }
                }
            }
        }
    }
}

pub fn unfold_more_double_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        points: "6.63,7.37 7.69,8.43 10,6.12 12.31,8.43 13.37,7.37 10,4",
                    }
                    polygon {
                        points: "6.63,3.37 7.69,4.43 10,2.12 12.31,4.43 13.37,3.37 10,0",
                    }
                    polygon {
                        points: "13.37,16.63 12.31,15.57 10,17.88 7.69,15.57 6.63,16.63 10,20",
                    }
                    polygon {
                        points: "13.37,12.63 12.31,11.57 10,13.88 7.69,11.57 6.63,12.63 10,16",
                    }
                }
            }
        }
    }
}

pub fn unfold_more_double_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    g {
                        path {
                            d: "M12,7.83L15.17,11l1.41-1.41L12,5L7.41,9.59L8.83,11L12,7.83z M12,2.83L15.17,6l1.41-1.41L12,0L7.41,4.59L8.83,6L12,2.83 z M12,21.17L8.83,18l-1.41,1.41L12,24l4.59-4.59L15.17,18L12,21.17z M12,16.17L8.83,13l-1.41,1.41L12,19l4.59-4.59L15.17,13 L12,16.17z",
                        }
                    }
                }
            }
        }
    }
}

pub fn unpublished_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.19,21.19L2.81,2.81L1.39,4.22l2.27,2.27C2.61,8.07,2,9.96,2,12c0,5.52,4.48,10,10,10c2.04,0,3.93-0.61,5.51-1.66 l2.27,2.27L21.19,21.19z M10.59,16.6l-4.24-4.24l1.41-1.41l2.83,2.83l0.18-0.18l1.41,1.41L10.59,16.6z M13.59,10.76l-7.1-7.1 C8.07,2.61,9.96,2,12,2c5.52,0,10,4.48,10,10c0,2.04-0.61,3.93-1.66,5.51l-5.34-5.34l2.65-2.65l-1.41-1.41L13.59,10.76z",
            }
        }
    }
}

pub fn update_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    x: "0",
                }
            }
            g {
                g {
                    g {
                        path {
                            d: "M21,10.12h-6.78l2.74-2.82c-2.73-2.7-7.15-2.8-9.88-0.1c-2.73,2.71-2.73,7.08,0,9.79s7.15,2.71,9.88,0 C18.32,15.65,19,14.08,19,12.1h2c0,1.98-0.88,4.55-2.64,6.29c-3.51,3.48-9.21,3.48-12.72,0c-3.5-3.47-3.53-9.11-0.02-12.58 s9.14-3.47,12.65,0L21,3V10.12z M12.5,8v4.25l3.5,2.08l-0.72,1.21L11,13V8H12.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn update_disabled_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M10.75,6v2.63l-1.5-1.5V6H10.75z M7.32,5.2C8.11,4.75,9.03,4.5,10,4.5c1.64,0,3.11,0.74,4.12,1.88L12.5,8H17V3.5l-1.82,1.82 C13.9,3.9,12.06,3,10,3C8.61,3,7.31,3.41,6.23,4.1L7.32,5.2z M15.9,13.77c0.7-1.09,1.1-2.38,1.1-3.77h-1.5 c0,0.97-0.25,1.89-0.7,2.68L15.9,13.77z M17.07,17.07l-1.06,1.06l-2.24-2.24C12.69,16.59,11.39,17,10,17c-3.87,0-7-3.13-7-7 c0-1.39,0.41-2.69,1.1-3.77L1.87,3.99l1.06-1.06L17.07,17.07z M12.68,14.8L5.2,7.32C4.75,8.11,4.5,9.03,4.5,10 c0,3.03,2.47,5.5,5.5,5.5C10.97,15.5,11.89,15.25,12.68,14.8z",
            }
        }
    }
}

pub fn update_disabled_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
            path {
                d: "M8.67,5.84L7.22,4.39C8.6,3.51,10.24,3,12,3c2.74,0,5.19,1.23,6.84,3.16L21,4v6h-6l2.41-2.41C16.12,6.02,14.18,5,12,5 C10.8,5,9.66,5.31,8.67,5.84z M13,7h-2v1.17l2,2V7z M19.78,22.61l-3-3C15.39,20.48,13.76,21,12,21c-4.97,0-9-4.03-9-9 c0-1.76,0.51-3.4,1.39-4.78L1.39,4.22l1.41-1.41l18.38,18.38L19.78,22.61z M15.32,18.15L5.84,8.67C5.31,9.66,5,10.8,5,12 c0,3.86,3.14,7,7,7C13.2,19,14.34,18.69,15.32,18.15z M20.94,13h-2.02c-0.12,0.83-0.39,1.61-0.77,2.32l1.47,1.47 C20.32,15.67,20.79,14.38,20.94,13z",
            }
        }
    }
}

pub fn upgrade_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M13,14v1H7v-1H13z M9.5,8v5h1V8H13l-3-3L7,8H9.5z",
                }
            }
        }
    }
}

pub fn upgrade_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,18v2H8v-2H16z M11,7.99V16h2V7.99h3L12,4L8,7.99H11z",
                }
            }
        }
    }
}

pub fn verified_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M18,10l-1.77-2.03l0.25-2.69l-2.63-0.6l-1.37-2.32L10,3.43L7.53,2.36L6.15,4.68L3.53,5.28l0.25,2.69L2,10l1.77,2.03 l-0.25,2.69l2.63,0.6l1.37,2.32L10,16.56l2.47,1.07l1.37-2.32l2.63-0.6l-0.25-2.69L18,10z M8.59,13.07l-2.12-2.12l0.71-0.71 l1.41,1.41l4.24-4.24l0.71,0.71L8.59,13.07z",
                }
            }
        }
    }
}

pub fn verified_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M23,12l-2.44-2.79l0.34-3.69l-3.61-0.82L15.4,1.5L12,2.96L8.6,1.5L6.71,4.69L3.1,5.5L3.44,9.2L1,12l2.44,2.79l-0.34,3.7 l3.61,0.82L8.6,22.5l3.4-1.47l3.4,1.46l1.89-3.19l3.61-0.82l-0.34-3.69L23,12z M10.09,16.72l-3.8-3.81l1.48-1.48l2.32,2.33 l5.85-5.87l1.48,1.48L10.09,16.72z",
                }
            }
        }
    }
}

pub fn verified_user_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm-2 16l-4-4 1.41-1.41L10 14.17l6.59-6.59L18 9l-8 8z",
            }
        }
    }
}

pub fn vertical_split_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M3 15h8v-2H3v2zm0 4h8v-2H3v2zm0-8h8V9H3v2zm0-6v2h8V5H3zm10 0h8v14h-8V5z",
            }
        }
    }
}

pub fn view_agenda_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    y: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v3C3,8.33,3.67,9,4.5,9h11C16.33,9,17,8.33,17,7.5v-3C17,3.67,16.33,3,15.5,3z",
                    }
                    path {
                        d: "M15.5,11h-11C3.67,11,3,11.67,3,12.5v3C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-3C17,11.67,16.33,11,15.5,11z",
                    }
                }
            }
        }
    }
}

pub fn view_agenda_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            enable_background: "new 0 0 24 24",
            g {
                rect {
                    y: "0",
                    fill: "none",
                    width: "24",
                    height: "24",
                    x: "0",
                }
            }
            g {
                g {
                    path {
                        d: "M19,13H5c-1.1,0-2,0.9-2,2v4c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v-4C21,13.9,20.1,13,19,13z",
                    }
                    path {
                        d: "M19,3H5C3.9,3,3,3.9,3,5v4c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z",
                    }
                }
            }
        }
    }
}

pub fn view_array_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M3,5h2v10H3V5z M15,5v10h2V5H15z M6,5h8v10H6V5z",
            }
        }
    }
}

pub fn view_array_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21,5h-3v14h3V5z M17,5H7v14h10V5z M6,5H3v14h3V5z",
            }
        }
    }
}

pub fn view_carousel_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M2,6.5h3v7H2V6.5z M15,6.5v7h3v-7H15z M6,5h8v10H6V5z",
            }
        }
    }
}

pub fn view_carousel_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M2,7h4v10H2V7z M7,19h10V5H7V19z M18,7h4v10h-4V7z",
            }
        }
    }
}

pub fn view_column_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M13,15h4V5h-4V15z M8,5v10h4V5H8z M7,15V5H3v10H7z",
            }
        }
    }
}

pub fn view_column_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M14.67,5v14H9.33V5H14.67z M15.67,19H21V5h-5.33V19z M8.33,19V5H3v14H8.33z",
                }
            }
        }
    }
}

pub fn view_comfy_alt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-9C18,4.67,17.33,4,16.5,4z M9,14 H6v-3h3V14z M9,9H6V6h3V9z M14,14h-3v-3h3V14z M14,9h-3V6h3V9z",
                    }
                }
            }
        }
    }
}

pub fn view_comfy_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M11,17H7v-4h4V17z M11,11H7V7h4V11 z M17,17h-4v-4h4V17z M17,11h-4V7h4V11z",
                    }
                }
            }
        }
    }
}

pub fn view_compact_alt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-9C18,4.67,17.33,4,16.5,4z M9.5,13.5h-3v-3h3V13.5z M9.5,9.5h-3v-3h3V9.5z M13.5,13.5h-3v-3h3V13.5z M13.5,9.5h-3v-3h3V9.5z",
                    }
                }
            }
        }
    }
}

pub fn view_compact_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M11.5,16.5h-4v-4h4V16.5z M11.5,11.5h-4v-4h4V11.5z M16.5,16.5h-4v-4h4V16.5z M16.5,11.5h-4v-4h4V11.5z",
                    }
                }
            }
        }
    }
}

pub fn view_cozy_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-9C18,4.67,17.33,4,16.5,4z M9.25,13.75h-3v-3h3V13.75z M9.25,9.25h-3v-3h3V9.25z M13.75,13.75h-3v-3h3V13.75z M13.75,9.25h-3v-3h3V9.25z",
                    }
                }
            }
        }
    }
}

pub fn view_cozy_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M11.25,16.75h-4v-4h4V16.75z M11.25,11.25h-4v-4h4V11.25z M16.75,16.75h-4v-4h4V16.75z M16.75,11.25h-4v-4h4V11.25z",
                    }
                }
            }
        }
    }
}

pub fn view_day_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M2 21h19v-3H2v3zM20 8H3c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1h17c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1zM2 3v3h19V3H2z",
            }
        }
    }
}

pub fn view_headline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M4 15h16v-2H4v2zm0 4h16v-2H4v2zm0-8h16V9H4v2zm0-6v2h16V5H4z",
            }
        }
    }
}

pub fn view_in_ar_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M18.25 7.6l-5.5-3.18c-.46-.27-1.04-.27-1.5 0L5.75 7.6c-.46.27-.75.76-.75 1.3v6.35c0 .54.29 1.03.75 1.3l5.5 3.18c.46.27 1.04.27 1.5 0l5.5-3.18c.46-.27.75-.76.75-1.3V8.9c0-.54-.29-1.03-.75-1.3zM7 14.96v-4.62l4 2.32v4.61l-4-2.31zm5-4.03L8 8.61l4-2.31 4 2.31-4 2.32zm1 6.34v-4.61l4-2.32v4.62l-4 2.31zM7 2H3.5C2.67 2 2 2.67 2 3.5V7h2V4h3V2zm10 0h3.5c.83 0 1.5.67 1.5 1.5V7h-2V4h-3V2zM7 22H3.5c-.83 0-1.5-.67-1.5-1.5V17h2v3h3v2zm10 0h3.5c.83 0 1.5-.67 1.5-1.5V17h-2v3h-3v2z",
            }
        }
    }
}

pub fn view_kanban_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M7.5,14H6V6h1.5V14z M10.75,10h-1.5V6h1.5V10z M14,12h-1.5V6H14V12z",
                    }
                }
            }
        }
    }
}

pub fn view_kanban_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M9,17H7V7h2V17z M13,12h-2V7h2V12z M17,15h-2V7h2V15z",
                }
            }
        }
    }
}

pub fn view_list_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M6,12.33V15H3v-2.67H6z M6,8.67H3v2.67h3V8.67z M6,5H3v2.67h3V5z M7,8.67v2.67h10V8.67H7z M7,15h10v-2.67H7V15z M17,7.67V5 H7v2.67H17z",
            }
        }
    }
}

pub fn view_list_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3,14h4v-4H3V14z M3,19h4v-4H3V19z M3,9h4V5H3V9z M8,14h13v-4H8V14z M8,19h13v-4H8V19z M8,5v4h13V5H8z",
            }
        }
    }
}

pub fn view_module_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M8,15v-4.5h4V15H8z M13,9.5h4V5h-4V9.5z M8,5v4.5h4V5H8z M7,9.5V5H3v4.5H7z M13,10.5V15h4v-4.5H13z M7,10.5H3V15h4V10.5z",
            }
        }
    }
}

pub fn view_module_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                path {
                    d: "M14.67,5v6.5H9.33V5H14.67z M15.67,11.5H21V5h-5.33V11.5z M14.67,19v-6.5H9.33V19H14.67z M15.67,12.5V19H21v-6.5H15.67z M8.33,12.5H3V19h5.33V12.5z M8.33,11.5V5H3v6.5H8.33z",
                }
            }
        }
    }
}

pub fn view_quilt_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M8,15v-4.5h4V15H8z M8,5v4.5h9V5H8z M7,15V5H3v10H7z M13,10.5V15h4v-4.5H13z",
            }
        }
    }
}

pub fn view_quilt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                    d: "M21,5v6.5H9.33V5H21z M14.67,19v-6.5H9.33V19H14.67z M15.67,12.5V19H21v-6.5H15.67z M8.33,19V5H3v14H8.33z",
                }
            }
        }
    }
}

pub fn view_sidebar_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,7.33h-3V4h3V7.33z M17,12.67h-3V16h3V12.67z M17,8.33h-3v3.33h3V8.33z M13,4H3v12h10V4z",
                }
            }
        }
    }
}

pub fn view_sidebar_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,20H2V4h14V20z M18,8h4V4h-4V8z M18,20h4v-4h-4V20z M18,14h4v-4h-4V14z",
                }
            }
        }
    }
}

pub fn view_stream_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M4.5,5h11C16.33,5,17,5.6,17,6.34v1.57c0,0.74-0.67,1.34-1.5,1.34h-11C3.67,9.25,3,8.65,3,7.91V6.34C3,5.6,3.67,5,4.5,5z M4.5,15h11c0.83,0,1.5-0.6,1.5-1.34v-1.57c0-0.74-0.67-1.34-1.5-1.34h-11c-0.83,0-1.5,0.6-1.5,1.34v1.57C3,14.4,3.67,15,4.5,15z",
            }
        }
    }
}

pub fn view_stream_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3,17v-2c0-1.1,0.9-2,2-2h14c1.1,0,2,0.9,2,2v2c0,1.1-0.9,2-2,2H5C3.9,19,3,18.1,3,17z M3,7v2c0,1.1,0.9,2,2,2h14 c1.1,0,2-0.9,2-2V7c0-1.1-0.9-2-2-2H5C3.9,5,3,5.9,3,7z",
            }
        }
    }
}

pub fn view_timeline_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M15.5,3h-11C3.67,3,3,3.67,3,4.5v11C3,16.33,3.67,17,4.5,17h11c0.83,0,1.5-0.67,1.5-1.5v-11C17,3.67,16.33,3,15.5,3z M9,14 H5v-1.5h4V14z M12,10.75H8v-1.5h4V10.75z M15,7.5h-4V6h4V7.5z",
                }
            }
        }
    }
}

pub fn view_timeline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M12,17H6v-2h6V17z M15,13H9v-2h6V13 z M18,9h-6V7h6V9z",
                }
            }
        }
    }
}

pub fn view_week_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M4.83,16H3.5C2.67,16,2,15.33,2,14.5v-9C2,4.67,2.67,4,3.5,4h1.33c0.83,0,1.5,0.67,1.5,1.5v9C6.33,15.33,5.66,16,4.83,16z M12.17,14.5v-9c0-0.83-0.67-1.5-1.5-1.5H9.33c-0.83,0-1.5,0.67-1.5,1.5v9c0,0.83,0.67,1.5,1.5,1.5h1.33 C11.5,16,12.17,15.33,12.17,14.5z M18,14.5v-9C18,4.67,17.33,4,16.5,4h-1.33c-0.83,0-1.5,0.67-1.5,1.5v9c0,0.83,0.67,1.5,1.5,1.5 h1.33C17.33,16,18,15.33,18,14.5z",
            }
        }
    }
}

pub fn view_week_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5.33,20H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h1.33c1.1,0,2,0.9,2,2v12C7.33,19.1,6.44,20,5.33,20z M22,18V6 c0-1.1-0.9-2-2-2h-1.33c-1.1,0-2,0.9-2,2v12c0,1.1,0.9,2,2,2H20C21.11,20,22,19.1,22,18z M14.67,18V6c0-1.1-0.9-2-2-2h-1.33 c-1.1,0-2,0.9-2,2v12c0,1.1,0.9,2,2,2h1.33C13.77,20,14.67,19.1,14.67,18z",
            }
        }
    }
}

pub fn visibility_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z",
            }
        }
    }
}

pub fn visibility_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z",
            }
        }
    }
}

pub fn voice_over_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M12.99 9.18c0-.06.01-.12.01-.18 0-2.21-1.79-4-4-4-.06 0-.12.01-.18.01l4.17 4.17zm-6.1-3.56L4.27 3 3 4.27l2.62 2.62C5.23 7.5 5 8.22 5 9c0 2.21 1.79 4 4 4 .78 0 1.5-.23 2.11-.62L19.73 21 21 19.73l-8.62-8.62-5.49-5.49zM9 15c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4zm7.76-9.64l-1.68 1.69c.84 1.18.84 2.71 0 3.89l1.68 1.69c2.02-2.02 2.02-5.07 0-7.27zM20.07 2l-1.63 1.63c2.77 3.02 2.77 7.56 0 10.74L20.07 16c3.9-3.89 3.91-9.95 0-14z",
            }
        }
    }
}

pub fn watch_later_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        path {
                            d: "M12,2C6.5,2,2,6.5,2,12s4.5,10,10,10s10-4.5,10-10S17.5,2,12,2z M16.2,16.2L11,13V7h1.5v5.2l4.5,2.7L16.2,16.2z",
                        }
                    }
                }
            }
        }
    }
}

pub fn webhook_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.5,3.5C9.12,3.5,8,4.62,8,6c0,1.17,0.81,2.15,1.89,2.43L7,12.92c0,0.03,0,0.05,0,0.08c0,0.55-0.45,1-1,1s-1-0.45-1-1 c0-0.43,0.27-0.8,0.66-0.94l2.05-3.19C6.96,8.14,6.5,7.12,6.5,6c0-2.21,1.79-4,4-4l0,0c2.21,0,4,1.79,4,4H13 C13,4.62,11.88,3.5,10.5,3.5z M8.5,12.25h4.84C13.51,12.09,13.75,12,14,12c0.55,0,1,0.45,1,1s-0.45,1-1,1 c-0.25,0-0.49-0.09-0.66-0.25l-3.41,0C9.58,15.6,7.95,17,6,17c-2.21,0-4-1.79-4-4c0-1.86,1.27-3.43,3-3.87l0,1.58 C4.12,11.1,3.5,11.98,3.5,13c0,1.38,1.12,2.5,2.5,2.5s2.5-1.12,2.5-2.5V12.25z M14,10.5c1.38,0,2.5,1.12,2.5,2.5 c0,1.38-1.12,2.5-2.5,2.5c-0.56,0-1.08-0.19-1.5-0.5l-1.97,0c0.69,1.2,1.98,2,3.46,2c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4l0,0 c-0.36,0-0.71,0.05-1.04,0.14l-1.47-2.95c0.01-0.06,0.02-0.12,0.02-0.18c0-0.55-0.45-1-1-1s-1,0.45-1,1c0,0.39,0.23,0.73,0.56,0.9 l2.17,4.33C12.68,10.78,13.31,10.5,14,10.5z",
                }
            }
        }
    }
}

pub fn webhook_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,15l5.88,0c0.27-0.31,0.67-0.5,1.12-0.5c0.83,0,1.5,0.67,1.5,1.5c0,0.83-0.67,1.5-1.5,1.5c-0.44,0-0.84-0.19-1.12-0.5 l-3.98,0c-0.46,2.28-2.48,4-4.9,4c-2.76,0-5-2.24-5-5c0-2.42,1.72-4.44,4-4.9l0,2.07C4.84,13.58,4,14.7,4,16c0,1.65,1.35,3,3,3 s3-1.35,3-3V15z M12.5,4c1.65,0,3,1.35,3,3h2c0-2.76-2.24-5-5-5l0,0c-2.76,0-5,2.24-5,5c0,1.43,0.6,2.71,1.55,3.62l-2.35,3.9 C6.02,14.66,5.5,15.27,5.5,16c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5c0-0.16-0.02-0.31-0.07-0.45l3.38-5.63 C10.49,9.61,9.5,8.42,9.5,7C9.5,5.35,10.85,4,12.5,4z M17,13c-0.64,0-1.23,0.2-1.72,0.54l-3.05-5.07C11.53,8.35,11,7.74,11,7 c0-0.83,0.67-1.5,1.5-1.5S14,6.17,14,7c0,0.15-0.02,0.29-0.06,0.43l2.19,3.65C16.41,11.03,16.7,11,17,11l0,0c2.76,0,5,2.24,5,5 c0,2.76-2.24,5-5,5c-1.85,0-3.47-1.01-4.33-2.5l2.67,0C15.82,18.82,16.39,19,17,19c1.65,0,3-1.35,3-3S18.65,13,17,13z",
                }
            }
        }
    }
}

pub fn width_full_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-9C18,4.67,17.33,4,16.5,4z M3.5,5.5h1v9h-1V5.5z M16.5,14.5h-1v-9h1V14.5z",
                }
            }
        }
    }
}

pub fn width_full_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M4,6h1v12H4V6z M20,18h-1V6h1V18z",
                }
            }
        }
    }
}

pub fn width_normal_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-9C18,4.67,17.33,4,16.5,4z M3.5,5.5H7v9H3.5V5.5z M16.5,14.5H13v-9h3.5V14.5z",
                }
            }
        }
    }
}

pub fn width_normal_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M4,6h4v12H4V6z M20,18h-4V6h4V18z",
                }
            }
        }
    }
}

pub fn width_wide_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.5,4h-13C2.67,4,2,4.67,2,5.5v9C2,15.33,2.67,16,3.5,16h13c0.83,0,1.5-0.67,1.5-1.5v-9C18,4.67,17.33,4,16.5,4z M3.5,5.5H5v9H3.5V5.5z M16.5,14.5H15v-9h1.5V14.5z",
                }
            }
        }
    }
}

pub fn width_wide_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6C22,4.9,21.1,4,20,4z M4,6h2v12H4V6z M20,18h-2V6h2V18z",
                }
            }
        }
    }
}

pub fn wifi_protected_setup_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                            d: "M15,4H9v6l1.8-1.8c1.22,0.91,2.01,2.35,2.01,3.99c0,0.75-0.18,1.45-0.47,2.09c1.49-0.98,2.47-2.67,2.47-4.59 c0-1.52-0.62-2.89-1.61-3.89L15,4z",
                        }
                    }
                    g {
                        path {
                            d: "M7.19,7.84c0-0.75,0.18-1.45,0.47-2.09c-1.49,0.98-2.47,2.67-2.47,4.59c0,1.52,0.62,2.89,1.61,3.89L5,16h6v-5.97 l-1.8,1.8C7.99,10.91,7.19,9.47,7.19,7.84z",
                        }
                    }
                }
            }
        }
    }
}

pub fn wifi_protected_setup_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        path {
                            d: "M16.71,5.29L19,3h-8v8l2.3-2.3c1.97,1.46,3.25,3.78,3.25,6.42c0,1.31-0.32,2.54-0.88,3.63c2.33-1.52,3.88-4.14,3.88-7.13 C19.55,9.1,18.44,6.85,16.71,5.29z",
                        }
                    }
                    g {
                        path {
                            d: "M7.46,8.88c0-1.31,0.32-2.54,0.88-3.63C6,6.77,4.46,9.39,4.46,12.38c0,2.52,1.1,4.77,2.84,6.33L5,21h8v-8l-2.3,2.3 C8.74,13.84,7.46,11.52,7.46,8.88z",
                        }
                    }
                }
            }
        }
    }
}

pub fn work_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M20 6h-4V4c0-1.11-.89-2-2-2h-4c-1.11 0-2 .89-2 2v2H4c-1.11 0-1.99.89-1.99 2L2 19c0 1.11.89 2 2 2h16c1.11 0 2-.89 2-2V8c0-1.11-.89-2-2-2zm-6 0h-4V4h4v2z",
            }
        }
    }
}

pub fn work_history_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.5,5H13V3.5C13,2.67,12.33,2,11.5,2h-3C7.67,2,7,2.67,7,3.5V5H3.5C2.67,5,2,5.67,2,6.5v9C2,16.33,2.67,17,3.5,17h5.38 c-0.24-0.62-0.38-1.29-0.38-2c0-3.04,2.46-5.5,5.5-5.5c1.58,0,3,0.67,4,1.73V6.5C18,5.67,17.33,5,16.5,5z M11.5,5h-3V3.5h3V5z",
                    }
                    path {
                        d: "M14,11c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4s4-1.79,4-4C18,12.79,16.21,11,14,11z M15.15,16.85l-1.65-1.65V13h1v1.79 l1.35,1.35L15.15,16.85z",
                    }
                }
            }
        }
    }
}

pub fn work_history_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M18,11c1.49,0,2.87,0.47,4,1.26V8c0-1.11-0.89-2-2-2h-4V4c0-1.11-0.89-2-2-2h-4C8.89,2,8,2.89,8,4v2H4 C2.89,6,2.01,6.89,2.01,8L2,19c0,1.11,0.89,2,2,2h7.68C11.25,20.09,11,19.08,11,18C11,14.13,14.13,11,18,11z M10,4h4v2h-4V4z",
                    }
                    path {
                        d: "M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M19.65,20.35l-2.15-2.15V15h1v2.79l1.85,1.85 L19.65,20.35z",
                    }
                }
            }
        }
    }
}

pub fn work_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M23 21.74l-1.46-1.46L7.21 5.95 3.25 1.99 1.99 3.25l2.7 2.7h-.64c-1.11 0-1.99.89-1.99 2l-.01 11c0 1.11.89 2 2 2h15.64L21.74 23 23 21.74zM22 7.95c.05-1.11-.84-2-1.95-1.95h-4V3.95c0-1.11-.89-2-2-1.95h-4c-1.11-.05-2 .84-2 1.95v.32l13.95 14V7.95zM14.05 6H10V3.95h4.05V6z",
            }
        }
    }
}

pub fn work_outline_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                fill_rule: "evenodd",
                d: "M14 6V4h-4v2h4zM4 8v11h16V8H4zm16-2c1.11 0 2 .89 2 2v11c0 1.11-.89 2-2 2H4c-1.11 0-2-.89-2-2l.01-11c0-1.11.88-2 1.99-2h4V4c0-1.11.89-2 2-2h4c1.11 0 2 .89 2 2v2h4z",
            }
        }
    }
}

pub fn wysiwyg_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,3H5C3.89,3,3,3.9,3,5v14c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.11,3,19,3z M19,19H5V7h14V19z M17,12H7v-2 h10V12z M13,16H7v-2h6V16z",
                }
            }
        }
    }
}

pub fn youtube_searched_for_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M0 0h24v24H0V0zm0 0h24v24H0V0z",
            }
            path {
                d: "M17.01 14h-.8l-.27-.27c.98-1.14 1.57-2.61 1.57-4.23 0-3.59-2.91-6.5-6.5-6.5s-6.5 3-6.5 6.5H2l3.84 4 4.16-4H6.51C6.51 7 8.53 5 11.01 5s4.5 2.01 4.5 4.5c0 2.48-2.02 4.5-4.5 4.5-.65 0-1.26-.14-1.82-.38L7.71 15.1c.97.57 2.09.9 3.3.9 1.61 0 3.08-.59 4.22-1.57l.27.27v.79l5.01 4.99L22 19l-4.99-5z",
            }
        }
    }
}

pub fn zoom_in_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z",
            }
            path {
                d: "M12 10h-2v2H9v-2H7V9h2V7h1v2h2v1z",
            }
        }
    }
}

pub fn zoom_out_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
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
                d: "M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14zM7 9h5v1H7z",
            }
        }
    }
}

