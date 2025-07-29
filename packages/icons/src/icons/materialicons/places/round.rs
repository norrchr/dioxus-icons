use dioxus::prelude::*;
use crate::IconProps;
pub fn ac_unit_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 11h-3.17l2.54-2.54c.39-.39.39-1.02 0-1.41-.39-.39-1.03-.39-1.42 0L15 11h-2V9l3.95-3.95c.39-.39.39-1.03 0-1.42-.39-.39-1.02-.39-1.41 0L13 6.17V3c0-.55-.45-1-1-1s-1 .45-1 1v3.17L8.46 3.63c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.03 0 1.42L11 9v2H9L5.05 7.05c-.39-.39-1.03-.39-1.42 0-.39.39-.39 1.02 0 1.41L6.17 11H3c-.55 0-1 .45-1 1s.45 1 1 1h3.17l-2.54 2.54c-.39.39-.39 1.02 0 1.41.39.39 1.03.39 1.42 0L9 13h2v2l-3.95 3.95c-.39.39-.39 1.03 0 1.42.39.39 1.02.39 1.41 0L11 17.83V21c0 .55.45 1 1 1s1-.45 1-1v-3.17l2.54 2.54c.39.39 1.02.39 1.41 0 .39-.39.39-1.03 0-1.42L13 15v-2h2l3.95 3.95c.39.39 1.03.39 1.42 0 .39-.39.39-1.02 0-1.41L17.83 13H21c.55 0 1-.45 1-1s-.45-1-1-1z",
            }
        }
    }
}

pub fn airport_shuttle_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22.41 10.41l-4.83-4.83c-.37-.37-.88-.58-1.41-.58H3c-1.1 0-2 .89-2 2v7c0 1.1.9 2 2 2 0 1.66 1.34 3 3 3s3-1.34 3-3h6c0 1.66 1.34 3 3 3s3-1.34 3-3c1.1 0 2-.9 2-2v-2.17c0-.53-.21-1.04-.59-1.42zM3 10V8c0-.55.45-1 1-1h3v4H4c-.55 0-1-.45-1-1zm3 7.25c-.69 0-1.25-.56-1.25-1.25s.56-1.25 1.25-1.25 1.25.56 1.25 1.25-.56 1.25-1.25 1.25zM13 11H9V7h4v4zm5 6.25c-.69 0-1.25-.56-1.25-1.25s.56-1.25 1.25-1.25 1.25.56 1.25 1.25-.56 1.25-1.25 1.25zM15 11V7h1l4 4h-5z",
            }
        }
    }
}

pub fn all_inclusive_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20.22 6.86c-2-.6-4.06-.04-5.39 1.29L12 10.66 10.48 12h.01L7.8 14.39c-.81.81-1.95 1.15-3.12.92-1.25-.25-2.28-1.25-2.57-2.49-.52-2.23 1.16-4.2 3.29-4.2.91 0 1.76.35 2.44 1.03l.47.41c.38.34.95.34 1.33 0 .45-.4.45-1.1 0-1.5l-.42-.36C8.2 7.18 6.84 6.62 5.4 6.62 2.42 6.62 0 9.04 0 12s2.42 5.38 5.4 5.38c1.44 0 2.8-.56 3.77-1.53l2.83-2.5.01.01L13.52 12h-.01l2.69-2.39c.81-.81 1.95-1.15 3.12-.92 1.25.25 2.28 1.25 2.57 2.49.52 2.23-1.16 4.2-3.29 4.2-.9 0-1.76-.35-2.44-1.03l-.48-.42c-.38-.34-.95-.34-1.33 0-.45.4-.45 1.1 0 1.5l.42.37c1.02 1.01 2.37 1.57 3.82 1.57 3.27 0 5.86-2.9 5.33-6.25-.3-1.99-1.77-3.69-3.7-4.26z",
            }
        }
    }
}

pub fn apartment_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,9V5c0-0.55-0.45-1-1-1H8C7.45,4,7,4.45,7,5v2H4C3.45,7,3,7.45,3,8v7c0,0.55,0.45,1,1,1h5.5c0.28,0,0.5-0.22,0.5-0.5V13 h1v2.5c0,0.28,0.22,0.5,0.5,0.5H16c0.55,0,1-0.45,1-1v-5c0-0.55-0.45-1-1-1H14z M6,14H5v-1h1V14z M6,12H5v-1h1V12z M6,10H5V9h1V10z M10,11H9v-1h1V11z M10,9H9V8h1V9z M10,7H9V6h1V7z M12,11h-1v-1h1V11z M12,9h-1V8h1V9z M12,7h-1V6h1V7z M15,14h-1v-1h1V14z M15,12 h-1v-1h1V12z",
                }
            }
        }
    }
}

pub fn apartment_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,11V5c0-1.1-0.9-2-2-2H9C7.9,3,7,3.9,7,5v2H5C3.9,7,3,7.9,3,9v10c0,1.1,0.9,2,2,2h5c0.55,0,1-0.45,1-1v-3h2v3 c0,0.55,0.45,1,1,1h5c1.1,0,2-0.9,2-2v-6c0-1.1-0.9-2-2-2H17z M7,19H5v-2h2V19z M7,15H5v-2h2V15z M7,11H5V9h2V11z M11,15H9v-2h2V15 z M11,11H9V9h2V11z M11,7H9V5h2V7z M15,15h-2v-2h2V15z M15,11h-2V9h2V11z M15,7h-2V5h2V7z M19,19h-2v-2h2V19z M19,15h-2v-2h2V15z",
                }
            }
        }
    }
}

pub fn baby_changing_station_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14,9L14,9c0,0.55-0.45,1-1,1h-1.58c-0.28,0-0.55-0.06-0.8-0.17l-2.3-1.01L7,12.75V21c0,0.55-0.45,1-1,1H4 c-0.55,0-1-0.45-1-1v-8.67c0-0.22,0.04-0.44,0.11-0.65l1.48-4.32C4.96,6.25,6.22,5.69,7.3,6.18l4.15,1.83L13,8 C13.55,8,14,8.45,14,9z M8,1C6.9,1,6,1.9,6,3s0.9,2,2,2s2-0.9,2-2S9.1,1,8,1z M10,19h10c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H10 c-0.55,0-1,0.45-1,1v0C9,18.55,9.45,19,10,19z M19.5,16c0.83,0,1.5-0.67,1.5-1.5c0-0.83-0.67-1.5-1.5-1.5S18,13.67,18,14.5 C18,15.33,18.67,16,19.5,16z M13,12c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1v1c0,1.1,0.9,2,2,2h2 c1.1,0,2-0.9,2-2v-2c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v1h-2V12z",
                }
            }
        }
    }
}

pub fn backpack_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                g {
                    g {
                        path {
                            d: "M20,8v12c0,1.1-0.9,2-2,2H6c-1.1,0-2-0.9-2-2V8c0-1.86,1.28-3.41,3-3.86V3.5C7,2.67,7.67,2,8.5,2h0 C9.33,2,10,2.67,10,3.5V4h4V3.5C14,2.67,14.67,2,15.5,2h0C16.33,2,17,2.67,17,3.5v0.64C18.72,4.59,20,6.14,20,8z M6,13L6,13 c0,0.55,0.45,1,1,1h9v1c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1H7C6.45,12,6,12.45,6,13z",
                        }
                    }
                }
            }
        }
    }
}

pub fn balcony_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M16,12.09L16,8c0-3.38-2.79-6.1-6.18-6C6.53,2.1,4,4.97,4,8.25v3.83c-0.58,0.21-1,0.76-1,1.41v3.02c0,0.83,0.67,1.5,1.5,1.5 h11c0.83,0,1.5-0.67,1.5-1.5V13.5C17,12.85,16.58,12.29,16,12.09z M6.12,16.5H4.5v-3h1.62V16.5z M9.25,16.5H7.62v-3h1.62V16.5z M5.5,12V8c0-2.23,1.62-4.08,3.75-4.44l0,8.44H5.5z M10.75,3.56C12.88,3.92,14.5,5.77,14.5,8v4h-3.75V3.56z M12.38,16.5h-1.62v-3 h1.62V16.5z M15.5,16.5h-1.62v-3h1.62V16.5z M8.5,9.25C8.5,9.66,8.16,10,7.75,10S7,9.66,7,9.25C7,8.84,7.34,8.5,7.75,8.5 S8.5,8.84,8.5,9.25z M13,9.25C13,9.66,12.66,10,12.25,10S11.5,9.66,11.5,9.25c0-0.41,0.34-0.75,0.75-0.75S13,8.84,13,9.25z",
            }
        }
    }
}

pub fn balcony_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20,14.27V10c0-4.42-3.58-8-8-8s-8,3.58-8,8v4.27C3.4,14.61,3,15.26,3,16v4c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v-4 C21,15.26,20.6,14.61,20,14.27z M7,20H5v-4h2V20z M11,20H9v-4h2V20z M11,14H6v-4c0-2.97,2.16-5.44,5-5.92V14z M13,4.08 c2.84,0.48,5,2.94,5,5.92v4h-5V4.08z M15,20h-2v-4h2V20z M19,20h-2v-4h2V20z M8,11c0-0.55,0.45-1,1-1s1,0.45,1,1s-0.45,1-1,1 S8,11.55,8,11z M16,11c0,0.55-0.45,1-1,1s-1-0.45-1-1s0.45-1,1-1S16,10.45,16,11z",
            }
        }
    }
}

pub fn bathtub_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        cy: "6.5",
                        r: "1.5",
                        cx: "6.5",
                    }
                    path {
                        d: "M16.5,11H16V5.12C16,3.95,15.05,3,13.88,3c-0.56,0-1.1,0.22-1.5,0.62l-0.83,0.83c-0.17-0.08-0.37-0.12-0.57-0.12 c-0.28,0-0.54,0.08-0.76,0.23l1.93,1.93c0.14-0.22,0.23-0.48,0.23-0.76c0-0.2-0.05-0.39-0.12-0.57l0.83-0.83 C13.3,4.12,13.58,4,13.88,4C14.5,4,15,4.5,15,5.12V11H9.1c-0.2-0.14-0.38-0.3-0.55-0.48L7.62,9.49C7.49,9.35,7.33,9.23,7.16,9.15 C6.95,9.05,6.72,9,6.49,9C5.67,9,5,9.67,5,10.5V11H3.5C3.22,11,3,11.22,3,11.5V15c0,0.55,0.45,1,1,1v0.5C4,16.78,4.22,17,4.5,17 h11c0.28,0,0.5-0.22,0.5-0.5V16c0.55,0,1-0.45,1-1v-3.5C17,11.22,16.78,11,16.5,11z",
                    }
                }
            }
        }
    }
}

pub fn bathtub_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    g {
                        circle {
                            r: "2",
                            cx: "7",
                            cy: "7",
                        }
                    }
                    g {
                        path {
                            d: "M21,13h-1V4.83C20,3.27,18.73,2,17.17,2c-0.75,0-1.47,0.3-2,0.83l-1.25,1.25C13.76,4.03,13.59,4,13.41,4 c-0.4,0-0.77,0.12-1.08,0.32l2.76,2.76c0.2-0.31,0.32-0.68,0.32-1.08c0-0.18-0.03-0.34-0.07-0.51l1.25-1.25 C16.74,4.09,16.95,4,17.17,4C17.63,4,18,4.37,18,4.83V13h-6.85c-0.3-0.21-0.57-0.45-0.82-0.72l-1.4-1.55 c-0.19-0.21-0.43-0.38-0.69-0.5C7.93,10.08,7.59,10,7.24,10C6,10.01,5,11.01,5,12.25V13H3c-0.55,0-1,0.45-1,1v5c0,1.1,0.9,2,2,2 c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1c1.1,0,2-0.9,2-2v-5C22,13.45,21.55,13,21,13z",
                        }
                    }
                }
            }
        }
    }
}

pub fn beach_access_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13.13 14.56l1.43-1.43 5.73 5.73c.39.39.39 1.03 0 1.43-.39.39-1.03.39-1.43 0l-5.73-5.73zm4.29-5.73l1.27-1.27c.89-.89.77-2.43-.31-3.08-3.89-2.38-9.03-1.89-12.4 1.47 3.93-1.3 8.31-.25 11.44 2.88zM5.95 5.98c-3.36 3.37-3.85 8.51-1.48 12.4.66 1.08 2.19 1.21 3.08.31l1.27-1.27C5.7 14.29 4.65 9.91 5.95 5.98zm.02-.02l-.01.01c-.38 3.01 1.17 6.88 4.3 10.02l5.73-5.73c-3.13-3.13-7.01-4.68-10.02-4.3z",
            }
        }
    }
}

pub fn bento_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                    d: "M16,11V5h4c1.1,0,2,0.9,2,2v4H16z M20,19c1.1,0,2-0.9,2-2v-4h-6v6H20z M14,5v14H4c-1.1,0-2-0.9-2-2V7c0-1.1,0.9-2,2-2H14z M9.5,12c0-0.83-0.67-1.5-1.5-1.5S6.5,11.17,6.5,12s0.67,1.5,1.5,1.5S9.5,12.83,9.5,12z",
                }
            }
        }
    }
}

pub fn bungalow_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                transform: "matrix(-1 -1.224647e-16 1.224647e-16 -1 20 20)",
                height: "20",
                fill: "none",
                width: "20",
            }
            path {
                d: "M10,13c0.41,0,0.75,0.34,0.75,0.75V17H13c0.55,0,1-0.45,1-1v-3.77l0.33,0.53c0.22,0.35,0.68,0.46,1.03,0.24 s0.46-0.68,0.24-1.03l-4.97-7.95c-0.29-0.47-0.98-0.47-1.27,0L4.4,11.96C4.18,12.32,4.28,12.78,4.64,13s0.81,0.11,1.03-0.24L6,12.23 V16c0,0.55,0.45,1,1,1h2.25v-3.25C9.25,13.34,9.59,13,10,13z M10,11.5c-0.41,0-0.75-0.34-0.75-0.75C9.25,10.34,9.59,10,10,10 s0.75,0.34,0.75,0.75C10.75,11.16,10.41,11.5,10,11.5z",
            }
        }
    }
}

pub fn bungalow_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                display: "none",
                width: "24",
            }
            path {
                d: "M12,16c0.55,0,1,0.45,1,1v4h3c0.55,0,1-0.45,1-1v-5.21l0.57,0.92c0.29,0.47,0.91,0.61,1.38,0.32 c0.47-0.29,0.61-0.91,0.32-1.38L12.85,4.36c-0.39-0.63-1.31-0.63-1.7,0L4.73,14.65c-0.29,0.47-0.15,1.09,0.32,1.38 c0.47,0.29,1.08,0.15,1.38-0.32L7,14.8V20c0,0.55,0.45,1,1,1h3v-4C11,16.45,11.45,16,12,16z M13,13c0,0.55-0.45,1-1,1s-1-0.45-1-1 s0.45-1,1-1S13,12.45,13,13z",
            }
        }
    }
}

pub fn business_center_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13 16h-2c-.55 0-1-.45-1-1H3.01v4c0 1.1.9 2 2 2H19c1.1 0 2-.9 2-2v-4h-7c0 .55-.45 1-1 1zm7-9h-4c0-2.21-1.79-4-4-4S8 4.79 8 7H4c-1.1 0-2 .9-2 2v3c0 1.11.89 2 2 2h6v-1c0-.55.45-1 1-1h2c.55 0 1 .45 1 1v1h6c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2zM10 7c0-1.1.9-2 2-2s2 .9 2 2H9.99 10z",
            }
        }
    }
}

pub fn cabin_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M4.25,3.45c0.31-0.64,0.94-1.2,2-1.2c0.24-0.01,0.52-0.08,0.66-0.34C7.04,1.66,7.3,1.5,7.58,1.5c0.54,0,0.91,0.57,0.68,1.05 c-0.31,0.64-0.94,1.2-2,1.2C6.01,3.76,5.73,3.83,5.59,4.09C5.46,4.34,5.2,4.5,4.92,4.5C4.38,4.5,4.01,3.93,4.25,3.45z M17.79,10.4 c-0.25,0.33-0.72,0.39-1.05,0.14L16,9.97V16c0,0.55-0.45,1-1,1H5c-0.55,0-1-0.45-1-1V9.97l-0.74,0.57c-0.33,0.25-0.8,0.19-1.05-0.14 c-0.25-0.33-0.19-0.8,0.14-1.05L4,8.08V6.25C4,5.84,4.34,5.5,4.75,5.5S5.5,5.84,5.5,6.25v0.69l3.89-2.97c0.36-0.27,0.86-0.27,1.21,0 l7.05,5.38C17.98,9.6,18.05,10.07,17.79,10.4z M8.54,6.5h2.91L10,5.39L8.54,6.5z M14.5,14h-9v1.5h9V14z M14.5,11h-9v1.5h9V11z M14.5,8.82L13.42,8H6.58L5.5,8.82V9.5h9V8.82z",
            }
        }
    }
}

pub fn cabin_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M4.37,3.55C4.89,2.62,5.87,2,7,2c0.38,0,0.72-0.22,0.89-0.53C8.04,1.16,8.39,1,8.73,1c0.74,0,1.26,0.8,0.9,1.45 C9.11,3.38,8.13,4,7,4C6.62,4,6.28,4.22,6.11,4.53C5.96,4.84,5.61,5,5.27,5C4.53,5,4.01,4.2,4.37,3.55z M22.39,12.19 c-0.34,0.44-0.96,0.52-1.4,0.19L20,11.62V20c0,0.55-0.45,1-1,1H5c-0.55,0-1-0.45-1-1v-8.38l-0.99,0.76 c-0.44,0.34-1.07,0.25-1.4-0.19c-0.33-0.44-0.25-1.07,0.19-1.4L4,9.11V7c0-0.55,0.45-1,1-1s1,0.45,1,1v0.58l5.39-4.12 c0.36-0.27,0.86-0.27,1.21,0l9.6,7.33C22.64,11.13,22.73,11.76,22.39,12.19z M10.06,7h3.89L12,5.52L10.06,7z M6,10.1V11h12v-0.9 L16.56,9H7.44L6,10.1z M6,13v2h12v-2H6z M18,19v-2H6v2H18z",
            }
        }
    }
}

pub fn carpenter_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19.73,14.23L7.71,2.21c-0.39-0.39-1.02-0.39-1.41,0L3.7,4.8C3.36,5.14,3.31,5.68,3.59,6.08l7.65,10.98 c-0.78,0.78-0.78,2.05,0,2.83l1.41,1.41c0.78,0.78,2.05,0.78,2.83,0l4.24-4.24C20.51,16.28,20.51,15.01,19.73,14.23z M14.07,19.88 l-1.41-1.41l4.24-4.24l1.41,1.41L14.07,19.88z",
            }
        }
    }
}

pub fn casino_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM7.5 18c-.83 0-1.5-.67-1.5-1.5S6.67 15 7.5 15s1.5.67 1.5 1.5S8.33 18 7.5 18zm0-9C6.67 9 6 8.33 6 7.5S6.67 6 7.5 6 9 6.67 9 7.5 8.33 9 7.5 9zm4.5 4.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm4.5 4.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm0-9c-.83 0-1.5-.67-1.5-1.5S15.67 6 16.5 6s1.5.67 1.5 1.5S17.33 9 16.5 9z",
            }
        }
    }
}

pub fn chalet_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M8.25,12C8.66,12,9,12.34,9,12.75V16h2c0.55,0,1-0.45,1-1v-2.62l0.41,0.41c0.29,0.29,0.77,0.3,1.06,0 c0.29-0.29,0.3-0.77,0-1.06l-4.5-4.53c-0.39-0.39-1.03-0.39-1.42,0l-4.53,4.53c-0.29,0.29-0.29,0.77,0,1.06 c0.29,0.29,0.77,0.29,1.06,0l0.35-0.35l0.04,2.57c0.01,0.55,0.45,0.98,1,0.98H7.5v-3.25C7.5,12.34,7.84,12,8.25,12z M15,6h0.9 l1.07-1.07c0.1-0.1,0.26-0.1,0.35,0c0.1,0.1,0.1,0.26,0,0.35L16.61,6h1.14C17.89,6,18,6.11,18,6.25S17.89,6.5,17.75,6.5h-1.14 l0.72,0.72c0.1,0.1,0.1,0.26,0,0.35c-0.1,0.1-0.26,0.1-0.35,0L15.9,6.5c0,0-0.9,0-0.9,0v0.9l1.07,1.07c0.1,0.1,0.1,0.26,0,0.35 c-0.1,0.1-0.26,0.1-0.35,0L15,8.11v1.14c0,0.14-0.11,0.25-0.25,0.25S14.5,9.39,14.5,9.25V8.11l-0.72,0.72c-0.1,0.1-0.26,0.1-0.35,0 c-0.1-0.1-0.1-0.26,0-0.35L14.5,7.4V6.5c0,0-0.9,0-0.9,0l-1.07,1.07c-0.1,0.1-0.26,0.1-0.35,0c-0.1-0.1-0.1-0.26,0-0.35l0.72-0.72 h-1.14c-0.14,0-0.25-0.11-0.25-0.25S11.61,6,11.75,6h1.14l-0.72-0.72c-0.1-0.1-0.1-0.26,0-0.35c0.1-0.1,0.26-0.1,0.35,0L13.6,6 c0,0,0.9,0,0.9,0V5.1l-1.07-1.07c-0.1-0.1-0.1-0.26,0-0.35c0.1-0.1,0.26-0.1,0.35,0l0.72,0.72V3.25C14.5,3.11,14.61,3,14.75,3 S15,3.11,15,3.25v1.14l0.72-0.72c0.1-0.1,0.26-0.1,0.35,0c0.1,0.1,0.1,0.26,0,0.35L15,5.1C15,5.1,15,6,15,6z",
            }
        }
    }
}

pub fn chalet_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M10,15c-0.55,0-1,0.45-1,1v4H6c-0.55,0-1-0.45-1-1v-3.67l-0.38,0.38c-0.39,0.39-1.02,0.39-1.41,0l0,0 c-0.39-0.39-0.39-1.02,0-1.41l6.09-6.09c0.39-0.39,1.02-0.39,1.41,0l6.09,6.09c0.39,0.39,0.39,1.02,0,1.41v0 c-0.39,0.39-1.02,0.39-1.41,0L15,15.33V19c0,0.55-0.45,1-1,1h-3v-4C11,15.45,10.55,15,10,15z M17.5,7.5v0.89l-1.08,1.08 c-0.18,0.18-0.21,0.48-0.05,0.69c0.19,0.23,0.53,0.24,0.74,0.04l0.39-0.39v0.69c0,0.28,0.22,0.5,0.5,0.5h0c0.28,0,0.5-0.22,0.5-0.5 V9.81l0.39,0.39c0.21,0.21,0.55,0.19,0.74-0.04c0.17-0.2,0.14-0.5-0.05-0.69L18.5,8.39c0,0,0-0.89,0-0.89h0.89l1.08,1.08 c0.18,0.18,0.48,0.21,0.69,0.05c0.23-0.19,0.24-0.53,0.04-0.74L20.81,7.5h0.69C21.78,7.5,22,7.28,22,7v0c0-0.28-0.22-0.5-0.5-0.5 l-0.69,0l0.39-0.39c0.21-0.21,0.19-0.55-0.04-0.74c-0.2-0.17-0.5-0.14-0.69,0.05L19.39,6.5c0,0-0.89,0-0.89,0V5.61l1.08-1.08 c0.18-0.18,0.21-0.48,0.05-0.69c-0.19-0.23-0.53-0.24-0.74-0.04L18.5,4.19V3.5C18.5,3.22,18.28,3,18,3h0c-0.28,0-0.5,0.22-0.5,0.5 v0.69L17.11,3.8c-0.21-0.21-0.55-0.19-0.74,0.04c-0.17,0.2-0.14,0.5,0.05,0.69l1.08,1.08c0,0,0,0.89,0,0.89h-0.89l-1.08-1.08 c-0.18-0.18-0.48-0.21-0.69-0.05c-0.23,0.19-0.24,0.53-0.04,0.74l0.39,0.39H14.5C14.22,6.5,14,6.72,14,7v0c0,0.28,0.22,0.5,0.5,0.5 l0.69,0L14.8,7.89c-0.21,0.21-0.19,0.55,0.04,0.74c0.2,0.17,0.5,0.14,0.69-0.05l1.08-1.08C16.61,7.5,17.5,7.5,17.5,7.5z",
            }
        }
    }
}

pub fn charging_station_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17,1H7C5.9,1,5,1.9,5,3v18c0,1.1,0.9,2,2,2h10c1.1,0,2-0.9,2-2V3C19,1.9,18.1,1,17,1z M17,18H7V6h10V18z M12.5,11V9.12 c0-0.53-0.71-0.7-0.95-0.22l-1.69,3.38C9.7,12.61,9.94,13,10.31,13h1.19v1.88c0,0.53,0.71,0.7,0.95,0.22l1.69-3.38 C14.3,11.39,14.06,11,13.69,11H12.5z",
                }
            }
        }
    }
}

pub fn checkroom_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M21.6,18.2L13,11.75v-0.91c1.65-0.49,2.8-2.17,2.43-4.05c-0.26-1.31-1.3-2.4-2.61-2.7c-1.76-0.4-3.37,0.53-4.02,1.98 C8.5,6.74,8.98,7.5,9.71,7.5h0c0.39,0,0.75-0.22,0.9-0.57C10.84,6.38,11.37,6,12,6c0.83,0,1.5,0.67,1.5,1.5 c0,0.84-0.69,1.52-1.53,1.5C11.43,8.99,11,9.45,11,9.99l0,1.76L2.4,18.2C1.63,18.78,2.04,20,3,20h9h9 C21.96,20,22.37,18.78,21.6,18.2z M6,18l6-4.5l6,4.5H6z",
                }
            }
        }
    }
}

pub fn child_care_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                cy: "10.5",
                cx: "14.5",
                r: "1.25",
            }
            circle {
                cx: "9.5",
                cy: "10.5",
                r: "1.25",
            }
            path {
                d: "M16.1 14H7.9c-.19 0-.32.2-.23.37C8.5 15.94 10.13 17 12 17s3.5-1.06 4.33-2.63c.08-.17-.05-.37-.23-.37zm6.84-2.66c-.25-1.51-1.36-2.74-2.81-3.17-.53-1.12-1.28-2.1-2.19-2.91C16.36 3.85 14.28 3 12 3s-4.36.85-5.94 2.26c-.92.81-1.67 1.8-2.19 2.91-1.45.43-2.56 1.65-2.81 3.17-.04.21-.06.43-.06.66 0 .23.02.45.06.66.25 1.51 1.36 2.74 2.81 3.17.52 1.11 1.27 2.09 2.17 2.89C7.62 20.14 9.71 21 12 21s4.38-.86 5.97-2.28c.9-.8 1.65-1.79 2.17-2.89 1.44-.43 2.55-1.65 2.8-3.17.04-.21.06-.43.06-.66 0-.23-.02-.45-.06-.66zM19 14c-.1 0-.19-.02-.29-.03-.2.67-.49 1.29-.86 1.86C16.6 17.74 14.45 19 12 19s-4.6-1.26-5.85-3.17c-.37-.57-.66-1.19-.86-1.86-.1.01-.19.03-.29.03-1.1 0-2-.9-2-2s.9-2 2-2c.1 0 .19.02.29.03.2-.67.49-1.29.86-1.86C7.4 6.26 9.55 5 12 5s4.6 1.26 5.85 3.17c.37.57.66 1.19.86 1.86.1-.01.19-.03.29-.03 1.1 0 2 .9 2 2s-.9 2-2 2z",
            }
        }
    }
}

pub fn child_friendly_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13 3.08V10h8c0-4.03-2.98-7.37-6.86-7.92-.6-.09-1.14.39-1.14 1zm6.32 12.81C20.37 14.54 21 12.84 21 11H6.44l-.68-1.43C5.6 9.22 5.24 9 4.86 9H3c-.55 0-1 .45-1 1s.45 1 1 1h1.22s1.89 4.07 2.12 4.42c-1.33.71-2.14 2.27-1.74 3.94.3 1.26 1.34 2.27 2.6 2.55 2.1.46 3.98-.96 4.25-2.91h2.08c.27 1.94 2.14 3.36 4.22 2.92 1.27-.27 2.31-1.27 2.63-2.53.35-1.39-.14-2.68-1.06-3.5zM8 20c-.83 0-1.5-.67-1.5-1.5S7.17 17 8 17s1.5.67 1.5 1.5S8.83 20 8 20zm9 0c-.83 0-1.5-.67-1.5-1.5S16.17 17 17 17s1.5.67 1.5 1.5S17.83 20 17 20z",
            }
        }
    }
}

pub fn corporate_fare_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12,7V5c0-1.1-0.9-2-2-2H4C2.9,3,2,3.9,2,5v14c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V9c0-1.1-0.9-2-2-2H12z M10,19H4v-2h6V19z M10,15H4v-2h6V15z M10,11H4V9h6V11z M10,7H4V5h6V7z M20,19h-8V9h8V19z M18,11h-4v2h4V11z M18,15h-4v2h4V15z",
            }
        }
    }
}

pub fn cottage_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M4.75,5.5C4.34,5.5,4,5.84,4,6.25v1.83L2.35,9.35C2.02,9.6,1.95,10.07,2.2,10.4c0.25,0.33,0.72,0.39,1.05,0.14L4,9.97V16 c0,0.55,0.45,1,1,1h4.25v-3.75c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75V17H15c0.55,0,1-0.45,1-1V9.97l0.74,0.57 c0.33,0.25,0.8,0.19,1.05-0.14c0.25-0.33,0.19-0.8-0.14-1.05l-7.05-5.38c-0.36-0.27-0.86-0.27-1.21,0L5.5,6.94V6.25 C5.5,5.84,5.16,5.5,4.75,5.5z M4.92,4.5c-0.54,0-0.91-0.57-0.68-1.05c0.31-0.64,0.94-1.2,2-1.2c0.24-0.01,0.52-0.08,0.66-0.34 C7.04,1.66,7.3,1.5,7.58,1.5c0.54,0,0.91,0.57,0.68,1.05c-0.31,0.64-0.94,1.2-2,1.2C6.01,3.76,5.73,3.83,5.59,4.09 C5.46,4.34,5.2,4.5,4.92,4.5L4.92,4.5z",
            }
        }
    }
}

pub fn cottage_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22.39,12.19c0.34-0.44,0.25-1.07-0.19-1.4l-9.6-7.33c-0.36-0.27-0.86-0.27-1.21,0L6,7.58V7c0-0.55-0.45-1-1-1S4,6.45,4,7 v2.11l-2.21,1.68c-0.44,0.33-0.52,0.96-0.19,1.4c0.34,0.44,0.96,0.52,1.4,0.19L4,11.62V20c0,0.55,0.45,1,1,1h6v-5c0-0.55,0.45-1,1-1 s1,0.45,1,1v5h6c0.55,0,1-0.45,1-1v-8.38l0.99,0.76C21.43,12.72,22.06,12.63,22.39,12.19z M5.27,5c-0.74,0-1.26-0.8-0.9-1.45 C4.89,2.62,5.87,2,7,2c0.38,0,0.72-0.22,0.89-0.53C8.04,1.16,8.39,1,8.73,1c0.74,0,1.26,0.8,0.9,1.45C9.11,3.38,8.13,4,7,4 C6.62,4,6.28,4.22,6.11,4.53C5.96,4.84,5.61,5,5.27,5z",
            }
        }
    }
}

pub fn countertops_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18,10l0-2.83c0-1.62-1.22-3.08-2.84-3.17c-1.21-0.06-2.27,0.59-2.8,1.57C12.01,6.22,12.53,7,13.27,7h0.01 c0.34,0,0.68-0.16,0.84-0.46C14.28,6.22,14.62,6,15,6c0.55,0,1,0.45,1,1v3H8c1.1,0,2-0.9,2-2V5c0-0.55-0.45-1-1-1H5 C4.45,4,4,4.45,4,5v3c0,1.1,0.9,2,2,2H3c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1v7c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1v-7h1 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H18z M13,18h-2v-6h2V18z",
            }
        }
    }
}

pub fn crib_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M15.1,15.1c-0.27-0.27-0.71-0.3-1-0.05c-0.34,0.28-0.7,0.52-1.09,0.72l0-2.27h1.5c0.83,0,1.5-0.67,1.5-1.5V9.5 C16,8.67,15.33,8,14.5,8H10V5.5C10,4.67,9.33,4,8.5,4H7C5.34,4,4,5.34,4,7v5c0,0.83,0.67,1.5,1.5,1.5H7v2.27 c-0.39-0.2-0.76-0.45-1.09-0.72c-0.32-0.26-0.79-0.21-1.06,0.11c-0.26,0.32-0.19,0.81,0.13,1.07C6.35,17.34,8.1,18,10,18 c1.91,0,3.67-0.67,5.04-1.79C15.39,15.93,15.41,15.41,15.1,15.1z M11.5,16.33c-0.48,0.11-0.98,0.17-1.5,0.17s-1.02-0.06-1.5-0.17 V13.5h3V16.33z",
            }
        }
    }
}

pub fn crib_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.32,18.32c-0.36-0.36-0.92-0.4-1.31-0.08c-0.32,0.25-0.65,0.48-1,0.69V16h2c1.1,0,2-0.9,2-2v-3c0-1.1-0.9-2-2-2h-6V6 c0-1.1-0.9-2-2-2H8C5.79,4,4,5.79,4,8v6c0,1.1,0.9,2,2,2h2v2.93c-0.35-0.2-0.69-0.43-1-0.69c-0.39-0.32-0.96-0.27-1.31,0.08 c-0.42,0.42-0.39,1.12,0.08,1.5C7.47,21.18,9.64,22,12,22c2.36,0,4.53-0.82,6.24-2.18C18.71,19.44,18.74,18.74,18.32,18.32z M14,19.75C13.36,19.91,12.69,20,12,20c-0.69,0-1.36-0.09-2-0.25V16h4V19.75z",
            }
        }
    }
}

pub fn desk_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M2,6v8.25C2,14.66,2.34,15,2.75,15h0c0.41,0,0.75-0.34,0.75-0.75V6.5H12v7.75c0,0.41,0.34,0.75,0.75,0.75h0 c0.41,0,0.75-0.34,0.75-0.75V13.5h3v0.75c0,0.41,0.34,0.75,0.75,0.75l0,0c0.41,0,0.75-0.34,0.75-0.75V6c0-0.55-0.45-1-1-1H3 C2.45,5,2,5.45,2,6z M13.5,6.5h3v2h-3V6.5z M13.5,12v-2h3v2H13.5z",
                }
            }
        }
    }
}

pub fn desk_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M2,7v10c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V8h10v9c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-1h4v1c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1V7c0-0.55-0.45-1-1-1H3C2.45,6,2,6.45,2,7z M20,8v2h-4V8H20z M16,14v-2h4v2H16z",
                }
            }
        }
    }
}

pub fn do_not_step_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M2.1,3.51c-0.39,0.39-0.39,1.02,0,1.41l7.19,7.19c0.18,0.2,0.18,0.5-0.01,0.7c-0.1,0.1-0.23,0.15-0.35,0.15 s-0.26-0.05-0.35-0.15L6.87,11.1c-0.11,0.4-0.26,0.78-0.45,1.12l1.4,1.4c0.2,0.2,0.2,0.51,0,0.71c-0.1,0.1-0.23,0.15-0.35,0.15 s-0.26-0.05-0.35-0.15l-1.27-1.27c-0.24,0.29-0.5,0.56-0.77,0.8l1.28,1.28c0.2,0.2,0.2,0.51,0,0.71C6.26,15.95,6.13,16,6,16 s-0.26-0.05-0.35-0.15l-1.38-1.38c-0.71,0.47-1.43,0.81-2.02,1.04C1.49,15.81,1,16.55,1,17.37L1,18c0,1.1,0.9,2,2,2h6.67 c0.53,0,1.04-0.21,1.41-0.59l2.74-2.74l5.23,5.23c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41L3.51,3.51 C3.12,3.12,2.49,3.12,2.1,3.51L2.1,3.51z M18.51,15.68l-1.41-1.41l4.48-4.48l0,0c0.78,0.78,0.78,2.05,0,2.83L18.51,15.68z M20.88,9.08l-4.48,4.48L9.3,6.47l3.09-3.07c0.78-0.78,2.04-0.77,2.82,0L20.88,9.08z",
                }
            }
        }
    }
}

pub fn do_not_touch_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M13,10.17l-2.5-2.5V2.25C10.5,1.56,11.06,1,11.75,1C12.44,1,13,1.56,13,2.25V10.17z M20,5.32c0-0.65-0.47-1.25-1.12-1.32 c-0.75-0.08-1.38,0.51-1.38,1.24v5.25c0,0.28-0.22,0.5-0.5,0.5h0c-0.28,0-0.5-0.22-0.5-0.5l0-7.18c0-0.65-0.47-1.25-1.12-1.32 C14.63,1.93,14,2.52,14,3.25v7.92l6,6L20,5.32z M9.5,4.25C9.5,3.56,8.94,3,8.25,3c-0.67,0-1.2,0.53-1.24,1.18L9.5,6.67V4.25z M17,22c0.62,0,1.18-0.19,1.65-0.52l-0.02-0.02l0.44,0.44c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41L3.51,3.51 c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l4.92,4.92L7,9.83v4.3l-2.6-1.48c-0.17-0.09-0.34-0.14-0.54-0.14 c-0.26,0-0.5,0.09-0.7,0.26L2,13.88l0,0l6.8,7.18c0.57,0.6,1.35,0.94,2.18,0.94L17,22L17,22z",
                }
            }
        }
    }
}

pub fn dry_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M1.94,11.79C1.34,12.36,1,13.14,1,13.97V20c0,1.66,1.34,3,3,3l13.68,0c0.65,0,1.25-0.47,1.32-1.12 c0.08-0.75-0.51-1.38-1.24-1.38H12.5c-0.28,0-0.5-0.22-0.5-0.5v0c0-0.28,0.22-0.5,0.5-0.5l7.18,0c0.65,0,1.25-0.47,1.32-1.12 c0.08-0.75-0.51-1.38-1.24-1.38H12.5c-0.28,0-0.5-0.22-0.5-0.5l0,0c0-0.28,0.22-0.5,0.5-0.5l8.18,0c0.65,0,1.25-0.47,1.32-1.12 c0.08-0.75-0.51-1.38-1.24-1.38H12.5c-0.28,0-0.5-0.22-0.5-0.5v0c0-0.28,0.22-0.5,0.5-0.5l6.18,0c0.65,0,1.25-0.47,1.32-1.12 c0.08-0.75-0.51-1.38-1.24-1.38H8.86l1.49-2.61c0.09-0.16,0.14-0.33,0.14-0.53c0-0.26-0.09-0.5-0.26-0.7L9.81,5.71 C9.43,5.32,8.8,5.3,8.4,5.68L1.94,11.79z M16.99,8.07c0,0.52-0.42,0.93-0.93,0.93c-0.52,0-0.93-0.42-0.93-0.93l0,0 c0.03-0.67-0.22-1.33-0.71-1.86l-0.07-0.06c-0.9-0.89-1.38-2.03-1.34-3.22l0,0C13,2.42,13.42,2,13.93,2c0.51,0,0.93,0.42,0.93,0.93 l0,0c-0.03,0.67,0.22,1.33,0.71,1.86l0.07,0.07C16.55,5.74,17.03,6.88,16.99,8.07L16.99,8.07z M21,8.07C21,8.58,20.58,9,20.06,9 s-0.93-0.42-0.93-0.93l0,0c0.03-0.67-0.22-1.33-0.71-1.86l-0.07-0.06c-0.9-0.89-1.38-2.03-1.34-3.22l0,0 c0-0.51,0.42-0.93,0.93-0.93s0.93,0.42,0.93,0.93l0,0c-0.03,0.67,0.22,1.33,0.71,1.86l0.07,0.07C20.55,5.74,21.03,6.88,21,8.07 L21,8.07z",
                }
            }
        }
    }
}

pub fn elevator_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M8.5,6c0.69,0,1.25,0.56,1.25,1.25 c0,0.69-0.56,1.25-1.25,1.25S7.25,7.94,7.25,7.25C7.25,6.56,7.81,6,8.5,6z M11,13c0,0.55-0.45,1-1,1v3c0,0.55-0.45,1-1,1H8 c-0.55,0-1-0.45-1-1v-3c-0.55,0-1-0.45-1-1v-1.5c0-1.1,0.9-2,2-2h1c1.1,0,2,0.9,2,2V13z M17.52,13.76l-1.6,2.56 c-0.2,0.31-0.65,0.31-0.85,0l-1.6-2.56C13.27,13.43,13.51,13,13.9,13h3.2C17.49,13,17.73,13.43,17.52,13.76z M17.1,11h-3.2 c-0.39,0-0.63-0.43-0.42-0.77l1.6-2.56c0.2-0.31,0.65-0.31,0.85,0l1.6,2.56C17.73,10.57,17.49,11,17.1,11z",
                }
            }
        }
    }
}

pub fn escalator_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                g {
                    path {
                        d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2l0-14C21,3.9,20.1,3,19,3z M17,9h-1.7l-4.71,8.49 C10.41,17.8,10.07,18,9.71,18H7c-0.83,0-1.5-0.67-1.5-1.5S6.17,15,7,15h1.7l4.71-8.49C13.59,6.2,13.93,6,14.29,6H17 c0.83,0,1.5,0.67,1.5,1.5S17.83,9,17,9z",
                    }
                }
            }
        }
    }
}

pub fn escalator_warning_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M6.5,2c1.1,0,2,0.9,2,2s-0.9,2-2,2s-2-0.9-2-2S5.4,2,6.5,2z M15.5,9.5c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5 S17.83,8,17,8S15.5,8.67,15.5,9.5z M18.5,12h-2.84c-0.58,0.01-1.14,0.32-1.45,0.86l-0.92,1.32L9.72,8C9.35,7.37,8.69,7.01,8.01,7H5 C3.9,7,3,7.9,3,9v5c0,0.55,0.45,1,1,1h0.5v6c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-9.39l2.24,3.89c0.18,0.31,0.51,0.5,0.87,0.5 h1.1c0.33,0,0.63-0.16,0.82-0.43L15,14.9V21c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1v-4h0c0.55,0,1-0.45,1-1v-2.5 C20,12.68,19.33,12,18.5,12z",
                }
            }
        }
    }
}

pub fn family_restroom_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,4c0-1.11,0.89-2,2-2s2,0.89,2,2s-0.89,2-2,2S16,5.11,16,4z M20,21v-5h1.11c0.68,0,1.16-0.67,0.95-1.32l-2.1-6.31 C19.68,7.55,18.92,7,18.06,7h-0.12c-0.86,0-1.63,0.55-1.9,1.37l-0.86,2.58C16.26,11.55,17,12.68,17,14v8h2C19.55,22,20,21.55,20,21 z M12.5,11.5c0.83,0,1.5-0.67,1.5-1.5s-0.67-1.5-1.5-1.5S11,9.17,11,10S11.67,11.5,12.5,11.5z M5.5,6c1.11,0,2-0.89,2-2 s-0.89-2-2-2s-2,0.89-2,2S4.39,6,5.5,6z M7.5,21v-6H8c0.55,0,1-0.45,1-1V9c0-1.1-0.9-2-2-2H4C2.9,7,2,7.9,2,9v5c0,0.55,0.45,1,1,1 h0.5v6c0,0.55,0.45,1,1,1h2C7.05,22,7.5,21.55,7.5,21z M10,14v3c0,0.55,0.45,1,1,1h0v3c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-3h0 c0.55,0,1-0.45,1-1v-3c0-0.82-0.68-1.5-1.5-1.5h-2C10.68,12.5,10,13.18,10,14",
                }
            }
        }
    }
}

pub fn fence_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M21,11L21,11c0-0.55-0.45-1-1-1h-1V7l-2.29-2.29c-0.39-0.39-1.02-0.39-1.41,0L14,6l-1.29-1.29c-0.39-0.39-1.02-0.39-1.41,0 L10,6L8.71,4.71c-0.39-0.39-1.02-0.39-1.41,0L5,7v3H4c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1v2H4c-0.55,0-1,0.45-1,1v0 c0,0.55,0.45,1,1,1h1v3c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1v-3h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1v-2h1 C20.55,12,21,11.55,21,11z M16,6.83l1,1V10h-2V7.83l0.41-0.41L16,6.83z M12,6.83l0.59,0.59L13,7.83V10h-2V7.83l0.41-0.41L12,6.83z M11,14v-2h2v2H11z M13,16v2h-2v-2H13z M7,7.83l1-1l0.59,0.59L9,7.83V10H7V7.83z M7,12h2v2H7V12z M7,16h2v2H7V16z M17,18h-2v-2h2 V18z M17,14h-2v-2h2V14z",
                }
            }
        }
    }
}

pub fn fire_extinguisher_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M7,19h10v1c0,1.1-0.9,2-2,2H9c-1.1,0-2-0.9-2-2V19z M7,18h10v-5H7V18z M17,4.23v3.54c0,0.63-0.58,1.11-1.21,0.98 l-1.94-0.41c0,0.02,0,0.01-0.01,0.03c1.54,0.62,2.71,1.98,3.06,3.63H7.1c0.34-1.66,1.52-3.02,3.07-3.64 c-0.33-0.26-0.6-0.58-0.8-0.95L5.49,6.6C5.2,6.54,5,6.29,5,6v0c0-0.29,0.2-0.54,0.49-0.6l3.88-0.81C9.87,3.65,10.86,3,12,3 c0.7,0,1.34,0.25,1.85,0.66l1.94-0.41C16.42,3.12,17,3.6,17,4.23z M13,6c-0.03-0.59-0.45-1-1-1s-1,0.45-1,1s0.45,1,1,1 S13,6.55,13,6z",
                }
            }
        }
    }
}

pub fn fitness_center_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20.57 14.86l.72-.72c.39-.39.39-1.02 0-1.41l-.02-.02c-.39-.39-1.02-.39-1.41 0L17 15.57 8.43 7l2.86-2.86c.39-.39.39-1.02 0-1.41l-.02-.02c-.39-.39-1.02-.39-1.41 0l-.72.72-.72-.72c-.39-.39-1.03-.39-1.42 0L5.57 4.14l-.72-.72c-.39-.39-1.04-.39-1.43 0-.39.39-.39 1.04 0 1.43l.72.72L2.71 7c-.39.39-.39 1.02 0 1.41l.72.72-.72.73c-.39.39-.39 1.02 0 1.41l.02.02c.39.39 1.02.39 1.41 0L7 8.43 15.57 17l-2.86 2.86c-.39.39-.39 1.02 0 1.41l.02.02c.39.39 1.02.39 1.41 0l.72-.72.72.72c.39.39 1.02.39 1.41 0l1.43-1.43.72.72c.39.39 1.04.39 1.43 0 .39-.39.39-1.04 0-1.43l-.72-.72L21.29 17c.39-.39.39-1.02 0-1.41l-.72-.73z",
            }
        }
    }
}

pub fn food_bank_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M10.8,3.9l-6,4.5C4.3,8.78,4,9.37,4,10v9c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-9c0-0.63-0.3-1.22-0.8-1.6l-6-4.5 C12.49,3.37,11.51,3.37,10.8,3.9z M12.5,12.5c0,0.83-0.67,1.5-1.5,1.5v3.5c0,0.28-0.22,0.5-0.5,0.5h0c-0.28,0-0.5-0.22-0.5-0.5V14 c-0.83,0-1.5-0.67-1.5-1.5V10c0-0.28,0.22-0.5,0.5-0.5h0c0.28,0,0.5,0.22,0.5,0.5v2.5H10V10c0-0.28,0.22-0.5,0.5-0.5h0 c0.28,0,0.5,0.22,0.5,0.5v2.5h0.5V10c0-0.28,0.22-0.5,0.5-0.5h0c0.28,0,0.5,0.22,0.5,0.5V12.5z M14.5,18L14.5,18 c-0.28,0-0.5-0.22-0.5-0.5v-3h-0.5c-0.28,0-0.5-0.22-0.5-0.5v-2.5c0-0.88,0.57-1.63,1.36-1.89C14.67,9.5,15,9.75,15,10.09v7.41 C15,17.78,14.78,18,14.5,18z",
            }
        }
    }
}

pub fn foundation_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19,12h1.7c0.46,0,0.68-0.57,0.33-0.87L12.67,3.6c-0.38-0.34-0.96-0.34-1.34,0l-8.36,7.53C2.63,11.43,2.84,12,3.3,12H5v3H4 c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h4v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1 v-2h4v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1V12z M7,15v-4.81l4-3.6V15H7z M13,15 V6.59l4,3.6V15H13z",
            }
        }
    }
}

pub fn free_breakfast_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 3H6c-1.1 0-2 .9-2 2v8c0 2.21 1.79 4 4 4h6c2.21 0 4-1.79 4-4v-3h2c1.11 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 5h-2V5h2v3zM5 19h14c.55 0 1 .45 1 1s-.45 1-1 1H5c-.55 0-1-.45-1-1s.45-1 1-1z",
            }
        }
    }
}

pub fn gite_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17.56,8.06l-2.12-2.12c-0.28-0.28-0.66-0.44-1.06-0.44H7.5V4.75C7.5,4.34,7.16,4,6.75,4S6,4.34,6,4.75V5.5H5.62 c-0.4,0-0.78,0.16-1.06,0.44L2.44,8.06C2.16,8.34,2,8.72,2,9.12v4.38C2,14.33,2.67,15,3.5,15h8H13h3.5c0.83,0,1.5-0.67,1.5-1.5V9.12 C18,8.72,17.84,8.34,17.56,8.06z M3.5,13.5V10h8v3.5H3.5z M16.5,13.5H13V9.12l1.75-1.75l1.75,1.75V13.5z",
            }
        }
    }
}

pub fn gite_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21.41,9.41l-2.83-2.83C18.21,6.21,17.7,6,17.17,6H9V5c0-0.55-0.45-1-1-1S7,4.45,7,5v1H6.83C6.3,6,5.79,6.21,5.41,6.59 L2.59,9.41C2.21,9.79,2,10.3,2,10.83V17c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-6.17C22,10.3,21.79,9.79,21.41,9.41z M14,17H4v-5h10 V17z M20,17h-4v-6.17l2-2l2,2V17z",
            }
        }
    }
}

pub fn golf_course_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                cy: "19.5",
                cx: "19.5",
                r: "1.5",
            }
            path {
                d: "M11 18.03V8.98l4.22-2.15c.73-.37.73-1.43-.01-1.79l-4.76-2.33C9.78 2.38 9 2.86 9 3.6V19c0 .55-.45 1-1 1s-1-.45-1-1v-.73c-1.79.35-3 .99-3 1.73 0 1.1 2.69 2 6 2s6-.9 6-2c0-.99-2.16-1.81-5-1.97z",
            }
        }
    }
}

pub fn grass_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15.64,11.02c0.55-1.47,1.43-2.78,2.56-3.83c0.38-0.36,0.04-1-0.46-0.85c-3.32,0.98-5.75,4.05-5.74,7.69c0,0,0,0,0,0 C12.95,12.75,14.2,11.72,15.64,11.02z M11.42,8.85c-0.6-1.56-1.63-2.91-2.96-3.87C8.04,4.68,7.5,5.17,7.74,5.63 C8.54,7.15,9,8.88,9,10.71c0,0.21-0.03,0.41-0.04,0.61c0.43,0.24,0.83,0.52,1.22,0.82C10.39,10.96,10.83,9.85,11.42,8.85z M12,20H3 c-0.55,0-1-0.45-1-1s0.45-1,1-1h4.75c-0.57-2.19-2.04-4.02-4-5.06l0,0c-0.16-0.08-0.26-0.25-0.26-0.44c0-0.27,0.22-0.49,0.49-0.5 c0.01,0,0.02,0,0.02,0C8.42,12,12,15.58,12,20z M20.26,12.94L20.26,12.94c-1.96,1.04-3.44,2.87-4,5.06H21c0.55,0,1,0.45,1,1 s-0.45,1-1,1h-5h-2c0-0.68-0.07-1.35-0.2-2c-0.15-0.72-0.38-1.42-0.67-2.07C14.52,13.58,17.07,12,20,12c0.01,0,0.02,0,0.02,0 c0.27,0,0.49,0.23,0.49,0.5C20.52,12.69,20.41,12.85,20.26,12.94z",
            }
        }
    }
}

pub fn holiday_village_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M17.5,15.25c0,0.42-0.35,0.76-0.77,0.75C16.32,15.99,16,15.63,16,15.22V6.85c0-0.13-0.05-0.26-0.15-0.35l-1.21-1.21 C14.17,4.81,14.5,4,15.17,4c0.2,0,0.39,0.08,0.53,0.22l1.51,1.51c0.19,0.19,0.29,0.44,0.29,0.71V15.25z M11.21,8.21 c0.19,0.19,0.29,0.44,0.29,0.71V15c0,0.55-0.45,1-1,1H7.75v-3c0-0.41-0.34-0.75-0.75-0.75S6.25,12.59,6.25,13v3H3.5 c-0.55,0-1-0.45-1-1V8.91c0-0.27,0.11-0.52,0.29-0.71l3.5-3.5c0.39-0.39,1.02-0.39,1.41,0L11.21,8.21z M7.75,10 c0-0.41-0.34-0.75-0.75-0.75S6.25,9.59,6.25,10c0,0.41,0.34,0.75,0.75,0.75S7.75,10.41,7.75,10z M14.5,7.67 c0-0.27-0.11-0.52-0.29-0.71l-2.75-2.75C11.32,4.08,11.13,4,10.93,4c-0.67,0-1,0.81-0.53,1.28l2.45,2.45C12.95,7.83,13,7.95,13,8.09 v7.13c0,0.41,0.32,0.77,0.73,0.78c0.42,0.01,0.77-0.33,0.77-0.75V7.67z",
            }
        }
    }
}

pub fn holiday_village_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17,20c0.55,0,1-0.45,1-1V8.76c0-0.27-0.11-0.52-0.29-0.71l-3.76-3.76C13.76,4.11,13.51,4,13.24,4 c-0.89,0-1.34,1.08-0.71,1.71l3.32,3.32C15.95,9.12,16,9.25,16,9.38V19C16,19.55,16.45,20,17,20z M21,20c0.55,0,1-0.45,1-1V7.11 c0-0.26-0.11-0.52-0.29-0.71l-2.1-2.11C19.42,4.11,19.16,4,18.9,4C18,4,17.56,5.08,18.19,5.71l1.67,1.67C19.95,7.47,20,7.6,20,7.73 V19C20,19.55,20.45,20,21,20z M8,15c0.55,0,1,0.45,1,1v4h4c0.55,0,1-0.45,1-1v-8.59c0-0.27-0.11-0.52-0.29-0.71l-5-5 c-0.39-0.39-1.02-0.39-1.41,0l-5,5C2.11,9.89,2,10.15,2,10.41V19c0,0.55,0.45,1,1,1h4v-4C7,15.45,7.45,15,8,15z M8,13 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S8.55,13,8,13z",
            }
        }
    }
}

pub fn hot_tub_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                cy: "6",
                cx: "7",
                r: "2",
            }
            path {
                d: "M21 12h-9.85c-.31-.22-.59-.46-.82-.72l-1.4-1.55c-.19-.21-.43-.38-.69-.5-.29-.14-.62-.23-.96-.23h-.03C6.01 9 5 10.01 5 11.25V12H3c-.55 0-1 .45-1 1v7c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2v-7c0-.55-.45-1-1-1zM7 19c0 .55-.45 1-1 1s-1-.45-1-1v-4c0-.55.45-1 1-1s1 .45 1 1v4zm4 0c0 .55-.45 1-1 1s-1-.45-1-1v-4c0-.55.45-1 1-1s1 .45 1 1v4zm4 0c0 .55-.45 1-1 1s-1-.45-1-1v-4c0-.55.45-1 1-1s1 .45 1 1v4zm4 0c0 .55-.45 1-1 1s-1-.45-1-1v-4c0-.55.45-1 1-1s1 .45 1 1v4zm-3.94-9c.5 0 .93-.39.94-.89.04-1.4-.58-2.48-1.35-3.25-.65-.72-.8-1.27-.77-1.91.02-.52-.41-.95-.94-.95-.5 0-.93.4-.94.9-.03 1.29.5 2.43 1.35 3.25.61.59.78 1.27.78 1.89-.01.52.4.96.93.96zm4 0c.5 0 .93-.39.94-.89.04-1.4-.58-2.48-1.35-3.25-.65-.72-.8-1.27-.77-1.91.02-.52-.41-.95-.94-.95-.5 0-.93.4-.94.9-.03 1.29.5 2.43 1.35 3.25.61.59.78 1.27.78 1.89-.01.52.4.96.93.96z",
            }
        }
    }
}

pub fn house_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M15,8.29V5.5C15,5.22,14.78,5,14.5,5h-1C13.22,5,13,5.22,13,5.5v1.07l-2.67-2.29c-0.19-0.16-0.46-0.16-0.65,0L3,10h2v5.5 C5,15.78,5.22,16,5.5,16h3C8.78,16,9,15.78,9,15.5V12h2v3.5c0,0.28,0.22,0.5,0.5,0.5h3c0.28,0,0.5-0.22,0.5-0.5V10h2L15,8.29z M9,9c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1H9z",
                    }
                }
            }
        }
    }
}

pub fn house_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M19,9.3V5c0-0.55-0.45-1-1-1h-1c-0.55,0-1,0.45-1,1v1.6l-3.33-3c-0.38-0.34-0.96-0.34-1.34,0l-8.36,7.53 C2.63,11.43,2.84,12,3.3,12H5v7c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-5h4v5c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-7h1.7 c0.46,0,0.68-0.57,0.33-0.87L19,9.3z M10,10c0-1.1,0.9-2,2-2s2,0.9,2,2H10z",
                }
            }
        }
    }
}

pub fn houseboat_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M4.44,8.3c0.24,0.33,0.71,0.41,1.05,0.16L6,8.09V11l-0.97,0c-0.27,0-0.52-0.11-0.71-0.29l-0.51-0.51 C3.52,9.9,3.05,9.87,2.75,10.15c-0.31,0.29-0.32,0.78-0.02,1.08l0.54,0.54c0.47,0.47,1.1,0.73,1.77,0.73h9.93 c0.66,0,1.3-0.26,1.77-0.73l0.52-0.52c0.29-0.29,0.32-0.77,0.04-1.07c-0.29-0.31-0.78-0.31-1.08-0.02l-0.54,0.54 C15.49,10.89,15.23,11,14.97,11L14,11V8.09l0.51,0.37c0.33,0.24,0.8,0.17,1.05-0.16c0.25-0.33,0.17-0.8-0.16-1.05l-4.8-3.52 c-0.35-0.26-0.83-0.26-1.18,0L4.6,7.26C4.27,7.5,4.2,7.97,4.44,8.3 M10.75,11h-1.5V9.75C9.25,9.34,9.59,9,10,9h0 c0.41,0,0.75,0.34,0.75,0.75V11z M15.34,13.5c1.29,0,1.61,0.66,2.2,0.91c0.27,0.11,0.46,0.38,0.46,0.7c0,0.42-0.34,0.76-0.76,0.76 c-0.11,0-0.21-0.02-0.3-0.06c-0.67-0.29-0.93-0.8-1.61-0.8c-0.95,0-1.08,1-2.67,1c-1.58,0-1.72-1-2.67-1c-0.95,0-1.08,1-2.67,1 c-1.59,0-1.72-1-2.67-1c-0.68,0-0.94,0.51-1.62,0.8c-0.09,0.04-0.19,0.06-0.3,0.06C2.34,15.86,2,15.52,2,15.11 c0-0.31,0.19-0.58,0.46-0.7c0.59-0.25,0.92-0.91,2.21-0.91c1.58,0,1.72,1,2.67,1c0.95,0,1.08-1,2.67-1c1.58,0,1.72,1,2.67,1 C13.62,14.5,13.76,13.5,15.34,13.5z",
            }
        }
    }
}

pub fn houseboat_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22,17.83c0-0.42-0.27-0.8-0.67-0.94C20.62,16.62,20.21,16,18.67,16c-1.91,0-2.14,1-3.33,1c-1.24,0-1.39-1-3.34-1 s-2.1,1-3.34,1c-1.19,0-1.42-1-3.33-1c-1.54,0-1.95,0.62-2.66,0.88C2.27,17.03,2,17.4,2,17.83c0,0.7,0.69,1.19,1.35,0.95 c0.8-0.29,1.18-0.78,2-0.78c1.19,0,1.42,1,3.33,1c1.95,0,2.08-1,3.32-1s1.37,1,3.32,1c1.91,0,2.14-1,3.33-1c0.83,0,1.21,0.49,2,0.78 C21.31,19.02,22,18.52,22,17.83z M18.91,9.81c0.33-0.45,0.23-1.07-0.22-1.4l-6.1-4.47c-0.35-0.26-0.83-0.26-1.18,0l-6.1,4.47 c-0.45,0.33-0.54,0.95-0.22,1.4c0.33,0.45,0.95,0.54,1.4,0.22L7,9.65V13H5.74c-0.27,0-0.52-0.11-0.71-0.29l-0.66-0.66 c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l0.66,0.66C4.18,14.68,4.95,15,5.74,15h12.51c0.8,0,1.56-0.32,2.12-0.88 l0.66-0.66c0.39-0.39,0.39-1.02,0-1.41c-0.39-0.39-1.02-0.39-1.41,0l-0.66,0.66C18.78,12.89,18.52,13,18.26,13H17V9.65l0.51,0.37 C17.96,10.35,18.58,10.25,18.91,9.81z M13,13h-2v-2h2V13z",
            }
        }
    }
}

pub fn house_siding_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19,12h1.7c0.46,0,0.68-0.57,0.33-0.87L12.67,3.6c-0.38-0.34-0.96-0.34-1.34,0l-8.36,7.53C2.63,11.43,2.84,12,3.3,12H5v7 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-1h10v1c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1V12z M7.21,10h9.58L17,10.19V12H7v-1.81 L7.21,10z M14.57,8H9.43L12,5.69L14.57,8z M7,16v-2h10v2H7z",
            }
        }
    }
}

pub fn iron_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M18,6.25C18,6.66,17.66,7,17.25,7H17c-0.28,0-0.5,0.22-0.5,0.5V11c0,1.1-0.9,2-2,2H14v0.5c0,0.55-0.45,1-1,1H3 c-0.55,0-1-0.45-1-1V12c0-1.66,1.34-3,3-3h7.5V8c0-0.28-0.22-0.5-0.5-0.5H8c-0.16,0-0.31,0.08-0.4,0.2 C7.44,7.92,7.15,8.04,6.85,7.99c-0.29-0.06-0.53-0.3-0.59-0.59C6.22,7.17,6.28,6.96,6.4,6.8C6.76,6.31,7.35,6,8,6h4c1.1,0,2,0.9,2,2 v3.5h0.5c0.28,0,0.5-0.22,0.5-0.5V7.5c0-1.1,0.9-2,2-2h0.25C17.66,5.5,18,5.84,18,6.25z",
            }
        }
    }
}

pub fn iron_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M8.27,10c0.34,0,0.68-0.16,0.84-0.47C9.28,9.22,9.62,9,10,9h4c0.55,0,1,0.45,1,1v1H6c-2.21,0-4,1.79-4,4v2 c0,0.55,0.45,1,1,1h13c0.55,0,1-0.45,1-1v-1c1.66,0,3-1.34,3-3V9c0-0.55,0.45-1,1-1h0c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h0 c-1.66,0-3,1.34-3,3v4c0,0.55-0.45,1-1,1v-4c0-1.66-1.34-3-3-3h-4C8.87,7,7.89,7.62,7.37,8.55C7.01,9.2,7.53,10,8.27,10z",
            }
        }
    }
}

pub fn kitchen_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18 2.01 6 2a2 2 0 00-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.11-.9-1.99-2-1.99zM17 20H7c-.55 0-1-.45-1-1v-7.02c0-.55.45-1 1-1h10c.55 0 1 .45 1 1V19c0 .55-.45 1-1 1zm0-11H7c-.55 0-1-.45-1-1V5c0-.55.45-1 1-1h10c.55 0 1 .45 1 1v3c0 .55-.45 1-1 1zM9 5c.55 0 1 .45 1 1v1c0 .55-.45 1-1 1s-1-.45-1-1V6c0-.55.45-1 1-1zm0 7c.55 0 1 .45 1 1v3c0 .55-.45 1-1 1s-1-.45-1-1v-3c0-.55.45-1 1-1z",
            }
        }
    }
}

pub fn meeting_room_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    g {
                        path {
                            d: "M20,19h-1V5c0-0.55-0.45-1-1-1h-4c0-0.55-0.45-1-1-1H6C5.45,3,5,3.45,5,4v15H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h9 c0.55,0,1-0.45,1-1V6h3v14c0,0.55,0.45,1,1,1h2c0.55,0,1-0.45,1-1S20.55,19,20,19z M11,13L11,13c-0.55,0-1-0.45-1-1v0 c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v0C12,12.55,11.55,13,11,13z",
                        }
                    }
                }
            }
        }
    }
}

pub fn microwave_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M6.15,9.97L6.15,9.97C5.69,9.51,5.77,8.73,6.33,8.4C6.73,8.18,7.21,8,7.75,8c0.8,0,1.39,0.39,1.81,0.67 C9.87,8.88,10.07,9,10.25,9c0.13,0,0.26-0.05,0.39-0.12c0.39-0.22,0.88-0.16,1.2,0.16l0,0c0.46,0.46,0.38,1.24-0.18,1.56 c-0.39,0.23-0.87,0.4-1.41,0.4c-0.79,0-1.37-0.38-1.79-0.66C8.13,10.12,7.94,10,7.75,10c-0.13,0-0.26,0.05-0.39,0.12 C6.96,10.35,6.47,10.28,6.15,9.97z M7.75,15c0.19,0,0.38,0.12,0.71,0.34c0.42,0.28,1,0.66,1.79,0.66c0.54,0,1.02-0.17,1.41-0.4 c0.56-0.32,0.64-1.1,0.18-1.56l0,0c-0.32-0.32-0.81-0.38-1.2-0.16C10.51,13.95,10.38,14,10.25,14c-0.18,0-0.38-0.12-0.69-0.33 C9.14,13.39,8.55,13,7.75,13c-0.54,0-1.02,0.18-1.42,0.4c-0.56,0.33-0.64,1.11-0.18,1.56l0,0c0.32,0.32,0.81,0.38,1.2,0.16 C7.49,15.05,7.62,15,7.75,15z M22,6v12c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2h16C21.1,4,22,4.9,22,6z M14,6H4v12h10 V6z M19,16c0-0.55-0.45-1-1-1c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1C18.55,17,19,16.55,19,16z M19,12c0-0.55-0.45-1-1-1 c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1C18.55,13,19,12.55,19,12z M19,7h-2v2h2V7z",
            }
        }
    }
}

pub fn night_shelter_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M10.8,3.9l-6,4.5C4.3,8.78,4,9.37,4,10v9c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2v-9c0-0.63-0.3-1.22-0.8-1.6l-6-4.5 C12.49,3.37,11.51,3.37,10.8,3.9z M9.75,12.5c0.69,0,1.25,0.56,1.25,1.25S10.44,15,9.75,15S8.5,14.44,8.5,13.75S9.06,12.5,9.75,12.5 z M16.5,18L16.5,18c-0.28,0-0.5-0.22-0.5-0.5v-1H8v1C8,17.78,7.78,18,7.5,18h0C7.22,18,7,17.78,7,17.5v-6C7,11.22,7.22,11,7.5,11h0 C7.78,11,8,11.22,8,11.5v4h3.5v-3c0-0.28,0.22-0.5,0.5-0.5h3c1.1,0,2,0.9,2,2v3.5C17,17.78,16.78,18,16.5,18z",
            }
        }
    }
}

pub fn no_backpack_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                y: "0",
                fill: "none",
            }
            g {
                g {
                    path {
                        d: "M6.98,4.15c0.01,0,0.01-0.01,0.02-0.01V3.5C7,2.67,7.67,2,8.5,2S10,2.67,10,3.5V4h4V3.5C14,2.67,14.67,2,15.5,2 S17,2.67,17,3.5v0.64c1.72,0.45,3,2,3,3.86v9.17l-2.03-2.03C17.98,15.09,18,15.05,18,15v-2c0-0.55-0.45-1-1-1h-2.17L6.98,4.15z M20.49,21.9c-0.39,0.39-1.02,0.39-1.41,0l-0.14-0.14C18.65,21.91,18.34,22,18,22H6c-1.1,0-2-0.9-2-2V8 c0-0.36,0.06-0.69,0.15-1.02L2.1,4.93c-0.39-0.39-0.39-1.02,0-1.41c0.39-0.39,1.02-0.39,1.41,0l16.97,16.97 C20.88,20.88,20.88,21.51,20.49,21.9z M11.17,14l-2-2H7c-0.55,0-1,0.45-1,1c0,0.55,0.45,1,1,1H11.17z",
                    }
                }
            }
        }
    }
}

pub fn no_cell_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M8.83,6l-3.7-3.7C5.42,1.55,6.15,1,7,1l10,0.01c1.1,0,2,0.89,2,1.99v13.17l-2-2V6H8.83z M20.49,21.9 c-0.39,0.39-1.02,0.39-1.41,0l-0.2-0.2C18.58,22.45,17.85,23,17,23H7c-1.1,0-2-0.9-2-2V7.83l-2.9-2.9c-0.39-0.39-0.39-1.02,0-1.41 c0.39-0.39,1.02-0.39,1.41,0l16.97,16.97C20.88,20.88,20.88,21.51,20.49,21.9z M15.17,18L7,9.83V18H15.17z",
                }
            }
        }
    }
}

pub fn no_drinks_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20.49,20.49L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0c-0.39,0.39-0.39,1.02,0,1.41l7.54,7.54L11,14v5H7c-0.55,0-1,0.45-1,1 s0.45,1,1,1h10c0.32,0,0.59-0.16,0.78-0.4l1.3,1.3c0.39,0.39,1.02,0.39,1.41,0C20.88,21.51,20.88,20.88,20.49,20.49z M13,19v-3.17 L16.17,19H13z M7.83,5l-2-2h13.72C20.35,3,21,3.65,21,4.45c0,0.35-0.13,0.7-0.37,0.96l-5.83,6.56L9.83,7h6.74l1.78-2H7.83z",
                }
            }
        }
    }
}

pub fn no_flash_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M3.16,3.16c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l4.6,4.6L6.14,9.4H3.6C2.72,9.4,2,10.12,2,11v9.4 C2,21.28,2.72,22,3.6,22h12.8c0.75,0,1.38-0.52,1.55-1.22l1.47,1.47c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41 L3.16,3.16z M10,20c-2.21,0-4-1.79-4-4c0-1.95,1.4-3.57,3.25-3.92l1.57,1.57c-0.26-0.09-0.53-0.15-0.82-0.15 c-1.38,0-2.5,1.12-2.5,2.5c0,1.38,1.12,2.5,2.5,2.5c1.38,0,2.5-1.12,2.5-2.5c0-0.29-0.06-0.56-0.15-0.82l1.57,1.57 C13.57,18.6,11.95,20,10,20z M18,15.17L10.83,8h0.87c0.56,0,1.1,0.24,1.48,0.65l0.69,0.75h2.54c0.88,0,1.6,0.72,1.6,1.6V15.17z M20.4,5.6h0.75c0.38,0,0.62,0.41,0.44,0.74L19,11V7h-0.5C18.22,7,18,6.78,18,6.5v-4C18,2.22,18.22,2,18.5,2h2.73 c0.36,0,0.6,0.37,0.46,0.7L20.4,5.6z",
                }
            }
        }
    }
}

pub fn no_food_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16,22c0,0.55-0.45,1-1,1H2c-0.55,0-1-0.45-1-1s0.45-1,1-1h13C15.55,21,16,21.45,16,22z M22.89,6.1 C22.95,5.51,22.49,5,21.9,5H18V2c0-0.55-0.45-1-1-1s-1,0.45-1,1v3h-3.9c-0.59,0-1.05,0.51-1,1.1l0.24,2.41L18,15.17l3.62,3.62 L22.89,6.1z M21.19,22.61c0.39-0.39,0.39-1.02,0-1.41L12,12L9.01,9.01h0l-6.2-6.2c-0.39-0.39-1.02-0.39-1.41,0l0,0 C1,3.2,1,3.83,1.39,4.22l4.99,4.99c-2.56,0.54-4.76,2.08-5.28,4.63C0.99,14.45,1.49,15,2.1,15l10.07,0l2,2H2c-0.55,0-1,0.45-1,1 s0.45,1,1,1h13c0.32,0,0.59-0.16,0.78-0.4l4,4C20.17,23,20.8,23,21.19,22.61L21.19,22.61z",
                }
            }
        }
    }
}

pub fn no_meeting_room_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14 6h3v7.88l2 2V5c0-.55-.45-1-1-1h-4c0-.55-.45-1-1-1H6.12L14 10.88V6zm7.17 14.88L12 11.71V13h-2v-2h1.29L3.12 2.83c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L5 7.54V19H4c-.55 0-1 .45-1 1s.45 1 1 1h9c.55 0 1-.45 1-1v-3.46l5.75 5.75c.39.39 1.02.39 1.41 0 .4-.39.4-1.02.01-1.41z",
            }
        }
    }
}

pub fn no_photography_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10.94,8.12L7.48,4.66L8.4,3.65C8.78,3.24,9.32,3,9.88,3h4.24c0.56,0,1.1,0.24,1.47,0.65L16.83,5H20c1.1,0,2,0.9,2,2v12 c0,0.05-0.01,0.1-0.02,0.16l-5.1-5.1C16.96,13.71,17,13.36,17,13c0-2.76-2.24-5-5-5C11.64,8,11.29,8.04,10.94,8.12z M19.78,22.61 L18.17,21H4c-1.1,0-2-0.9-2-2V7c0-0.59,0.27-1.12,0.68-1.49L1.39,4.22C1,3.83,1,3.2,1.39,2.81l0,0c0.39-0.39,1.02-0.39,1.41,0 l18.38,18.38c0.39,0.39,0.39,1.02,0,1.41l0,0C20.8,23,20.17,23,19.78,22.61z M14.49,17.32l-1.5-1.5C12.67,15.92,12.35,16,12,16 c-1.66,0-3-1.34-3-3c0-0.35,0.08-0.67,0.19-0.98l-1.5-1.5C7.25,11.24,7,12.09,7,13c0,2.76,2.24,5,5,5 C12.91,18,13.76,17.75,14.49,17.32z",
                }
            }
        }
    }
}

pub fn no_stroller_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M8,20c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2S8,18.9,8,20z M14.3,4.1C13.03,3.4,11.56,3,10,3C8.77,3,7.61,3.26,6.54,3.71 l4.37,4.37L14.3,4.1z M20.49,21.9c0.39-0.39,0.39-1.02,0-1.41l-9.82-9.82L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0l0,0 c-0.39,0.39-0.39,1.02,0,1.41l7.26,7.26l-2.66,3.12c-0.55,0.65-0.09,1.65,0.76,1.65h6.66l1.17,1.17c-0.88,0.33-1.47,1.25-1.26,2.28 c0.15,0.76,0.78,1.39,1.54,1.54c1.03,0.21,1.95-0.38,2.28-1.26l1.2,1.2C19.46,22.29,20.09,22.29,20.49,21.9L20.49,21.9z M17,6.27 C17.58,5.59,17.97,5,18.65,5c0.68,0,1.22,0.52,1.33,1.21C20.08,6.66,20.48,7,20.96,7c0.55,0,1-0.45,1-1c0-0.06,0-0.11-0.01-0.16 l0-0.01C21.65,4.22,20.3,3,18.65,3c-1.66,0-2.54,1.27-3.18,2.03l-3.5,4.11L17,14.17V6.27z",
                }
            }
        }
    }
}

pub fn other_houses_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M2.2,10.4c0.25,0.33,0.72,0.39,1.05,0.14L4,9.97V16c0,0.55,0.45,1,1,1h10c0.55,0,1-0.45,1-1V9.97l0.74,0.57 c0.33,0.25,0.8,0.19,1.05-0.14c0.25-0.33,0.19-0.8-0.14-1.05l-7.05-5.38c-0.36-0.27-0.86-0.27-1.21,0L2.35,9.35 C2.02,9.6,1.95,10.07,2.2,10.4z M7,12.5c-0.41,0-0.75-0.34-0.75-0.75C6.25,11.34,6.59,11,7,11s0.75,0.34,0.75,0.75 C7.75,12.16,7.41,12.5,7,12.5z M10,12.5c-0.41,0-0.75-0.34-0.75-0.75C9.25,11.34,9.59,11,10,11s0.75,0.34,0.75,0.75 C10.75,12.16,10.41,12.5,10,12.5z M13,12.5c-0.41,0-0.75-0.34-0.75-0.75c0-0.41,0.34-0.75,0.75-0.75s0.75,0.34,0.75,0.75 C13.75,12.16,13.41,12.5,13,12.5z",
            }
        }
    }
}

pub fn other_houses_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M1.61,12.19c0.34,0.44,0.96,0.52,1.4,0.19L4,11.62V20c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1v-8.38l0.99,0.76 c0.44,0.34,1.07,0.25,1.4-0.19c0.34-0.44,0.25-1.07-0.19-1.4l-9.6-7.33c-0.36-0.27-0.86-0.27-1.21,0l-9.6,7.33 C1.36,11.13,1.27,11.76,1.61,12.19z M8,15c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C9,14.55,8.55,15,8,15z M12,15 c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1s1,0.45,1,1C13,14.55,12.55,15,12,15z M16,15c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1 s1,0.45,1,1C17,14.55,16.55,15,16,15z",
            }
        }
    }
}

pub fn pool_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M6.11 5.56C7.3 5.7 8.14 6.14 9 7l1 1-3.25 3.25c.31.12.56.27.77.39.37.23.59.36 1.15.36s.78-.13 1.15-.36c.46-.27 1.08-.64 2.19-.64s1.73.37 2.18.64c.37.22.6.36 1.15.36.55 0 .78-.13 1.15-.36.12-.07.26-.15.41-.23L10.48 5C9.22 3.74 8.04 3.2 6.3 3.05 5.6 2.99 5 3.56 5 4.26v.09c0 .63.49 1.13 1.11 1.21zm15.24 13.35c-.17-.06-.32-.15-.5-.27-.45-.27-1.07-.64-2.18-.64s-1.73.37-2.18.64c-.37.23-.6.36-1.15.36-.55 0-.78-.14-1.15-.36-.45-.27-1.07-.64-2.18-.64s-1.73.37-2.19.64c-.37.23-.59.36-1.15.36s-.78-.13-1.15-.36c-.45-.27-1.07-.64-2.18-.64s-1.73.37-2.19.64c-.18.11-.33.2-.5.27-.38.13-.65.45-.65.85v.12c0 .67.66 1.13 1.3.91.37-.13.65-.3.89-.44.37-.22.6-.35 1.15-.35.55 0 .78.13 1.15.36.45.27 1.07.64 2.18.64s1.73-.37 2.19-.64c.37-.23.59-.36 1.15-.36.55 0 .78.14 1.15.36.45.27 1.07.64 2.18.64s1.72-.37 2.18-.64c.37-.23.59-.36 1.15-.36.55 0 .78.14 1.15.36.23.14.51.31.88.44.63.22 1.3-.24 1.3-.91v-.12c0-.41-.27-.73-.65-.86zM3.11 16.35c.47-.13.81-.33 1.09-.49.37-.23.6-.36 1.15-.36.55 0 .78.14 1.15.36.45.27 1.07.64 2.18.64s1.73-.37 2.18-.64c.37-.23.59-.36 1.15-.36.55 0 .78.14 1.15.36.45.27 1.07.64 2.18.64s1.73-.37 2.18-.64c.37-.23.59-.36 1.15-.36.55 0 .78.14 1.15.36.23.14.5.3.85.43.63.23 1.31-.24 1.31-.91v-.12c0-.4-.27-.72-.64-.86-.17-.06-.32-.15-.51-.26-.45-.27-1.07-.64-2.18-.64s-1.73.37-2.18.64c-.37.23-.6.36-1.15.36s-.78-.14-1.15-.36c-.45-.27-1.07-.64-2.18-.64s-1.73.37-2.18.64c-.37.23-.59.36-1.15.36-.55 0-.78-.14-1.15-.36-.45-.27-1.07-.64-2.18-.64s-1.73.37-2.18.64c-.18.11-.33.2-.5.27-.38.13-.65.45-.65.85v.23c0 .58.55 1.02 1.11.86z",
            }
            circle {
                cy: "5.5",
                r: "2.5",
                cx: "16.5",
            }
        }
    }
}

pub fn rice_bowl_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22,12L22,12c0-5.48-4.4-9.93-9.86-10C8.52,1.95,5.29,4.03,3.43,7.14C0.1,12.69,2.98,18.27,8,20.25v0.25 C8,21.33,8.67,22,9.5,22h5c0.83,0,1.5-0.67,1.5-1.5v-0.25C19.53,18.86,22,15.69,22,12z M20,12h-4V5.08C18.39,6.47,20,9.05,20,12z M14,4.26V12h-4V4.26C10.64,4.1,11.31,4,12,4S13.36,4.1,14,4.26z M4,12c0-2.95,1.61-5.53,4-6.92V12H4z",
            }
        }
    }
}

pub fn roofing_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M13,18h-2v-2h2V18z M9,15v4c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-4c0-0.55-0.45-1-1-1h-4C9.45,14,9,14.45,9,15z M19,9.3 L19,9.3V5c0-0.55-0.45-1-1-1h-1c-0.55,0-1,0.45-1,1v1.6v0l-3.33-3c-0.38-0.34-0.96-0.34-1.34,0l-8.36,7.53 C2.63,11.43,2.84,12,3.3,12h1.31c0.25,0,0.49-0.09,0.67-0.26L12,5.69l6.71,6.05C18.9,11.91,19.14,12,19.38,12h1.31 c0.46,0,0.68-0.57,0.33-0.87L19,9.3z",
            }
        }
    }
}

pub fn room_preferences_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M21.75,17c0-0.22-0.03-0.42-0.06-0.63l0.84-0.73c0.18-0.16,0.22-0.42,0.1-0.63l-0.59-1.02c-0.12-0.21-0.37-0.3-0.59-0.22 l-1.06,0.36c-0.32-0.27-0.68-0.48-1.08-0.63l-0.22-1.09c-0.05-0.23-0.25-0.4-0.49-0.4h-1.18c-0.24,0-0.44,0.17-0.49,0.4l-0.22,1.09 c-0.4,0.15-0.76,0.36-1.08,0.63l-1.06-0.36c-0.23-0.08-0.47,0.02-0.59,0.22l-0.59,1.02c-0.12,0.21-0.08,0.47,0.1,0.63l0.84,0.73 c-0.03,0.21-0.06,0.41-0.06,0.63s0.03,0.42,0.06,0.63l-0.84,0.73c-0.18,0.16-0.22,0.42-0.1,0.63l0.59,1.02 c0.12,0.21,0.37,0.3,0.59,0.22l1.06-0.36c0.32,0.27,0.68,0.48,1.08,0.63l0.22,1.09c0.05,0.23,0.25,0.4,0.49,0.4h1.18 c0.24,0,0.44-0.17,0.49-0.4l0.22-1.09c0.4-0.15,0.76-0.36,1.08-0.63l1.06,0.36c0.23,0.08,0.47-0.02,0.59-0.22l0.59-1.02 c0.12-0.21,0.08-0.47-0.1-0.63l-0.84-0.73C21.72,17.42,21.75,17.22,21.75,17z M18,19c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2 S19.1,19,18,19z M14,11.26V6h3v4h2V5c0-0.55-0.45-1-1-1h-4c0-0.55-0.45-1-1-1H6C5.45,3,5,3.45,5,4v15H4c-0.55,0-1,0.45-1,1 s0.45,1,1,1h8.26C11.47,19.87,11,18.49,11,17C11,14.62,12.19,12.53,14,11.26z M10,12c0-0.55,0.45-1,1-1s1,0.45,1,1 c0,0.55-0.45,1-1,1S10,12.55,10,12z",
                }
            }
        }
    }
}

pub fn room_service_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3 17h18c.55 0 1 .45 1 1s-.45 1-1 1H3c-.55 0-1-.45-1-1s.45-1 1-1zm10.84-9.21c.1-.24.16-.51.16-.79 0-1.1-.9-2-2-2s-2 .9-2 2c0 .28.06.55.16.79C6.25 8.6 3.27 11.93 3 16h18c-.27-4.07-3.25-7.4-7.16-8.21z",
            }
        }
    }
}

pub fn rv_hookup_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 17h-1v-6c0-1.1-.9-2-2-2H7v-.74c0-.46-.56-.7-.89-.37L4.37 9.63c-.2.2-.2.53 0 .74l1.74 1.74c.33.33.89.1.89-.37V11h4v3H5c-.55 0-1 .45-1 1v2c0 1.1.9 2 2 2h2c0 1.66 1.34 3 3 3s3-1.34 3-3h7c.55 0 1-.45 1-1s-.45-1-1-1zm-10 3c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm7-6h-4v-3h3c.55 0 1 .45 1 1v2zm-8-8h7v.74c0 .46.56.7.89.37l1.74-1.74c.2-.2.2-.53 0-.74l-1.74-1.74c-.33-.33-.89-.1-.89.37V4h-7c-.55 0-1 .45-1 1s.45 1 1 1z",
            }
        }
    }
}

pub fn smoke_free_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20.5 13H22v3h-1.5zM18 13h1.5v3H18zm-1 1.5c0-.83-.67-1.5-1.5-1.5h-.84l2.18 2.18c.1-.21.16-.44.16-.68zm1.96-12.15H19h-.04zm-.11 2.38c.38-.38.67-.84.84-1.35.16-.5-.19-1.01-.71-1.02-.34.01-.61.25-.72.58-.18.55-.62.99-1.17 1.17-.34.11-.59.39-.59.74V5c0 .37.27.69.64.75 1.93.31 3.36 2 3.36 4.02v1.48c0 .41.34.75.75.75s.75-.34.75-.75V9.76c0-2.22-1.28-4.14-3.15-5.03zm-4.24 3.92h1.42c1.05 0 1.97.74 1.97 2.05v.55c0 .41.33.75.75.75h.01c.41 0 .75-.33.75-.75v-.89c0-1.81-1.6-3.16-3.47-3.16h-1.3c-1.02 0-1.94-.73-2.07-1.75-.12-.95.46-1.7 1.3-1.93.32-.09.54-.38.54-.72 0-.49-.46-.86-.93-.72-1.42.41-2.45 1.73-2.42 3.28.02 1.85 1.61 3.29 3.45 3.29zM4.12 5.29c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L9 13H3.5c-.83 0-1.5.67-1.5 1.5S2.67 16 3.5 16H12l6.29 6.29c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L4.12 5.29z",
            }
        }
    }
}

pub fn smoking_rooms_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15.5 16h-12c-.83 0-1.5.67-1.5 1.5S2.67 19 3.5 19h12c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5zm3.35-8.27c.62-.61 1-1.45 1-2.38 0-1.51-1-2.79-2.38-3.21-.48-.14-.97.22-.97.72 0 .33.21.62.52.71.77.23 1.33.94 1.33 1.78 0 .82-.53 1.51-1.27 1.76-.33.11-.58.39-.58.74V8c0 .37.27.69.64.75 1.93.31 3.36 2 3.36 4.02v1.48c0 .41.34.75.75.75s.75-.34.75-.75v-1.49c0-2.22-1.28-4.14-3.15-5.03zm-2.82 2.47h-1.3c-1.02 0-1.94-.73-2.07-1.75-.12-.95.46-1.7 1.3-1.93.32-.09.54-.38.54-.72 0-.49-.46-.86-.93-.72-1.42.41-2.45 1.73-2.42 3.28.03 1.84 1.62 3.29 3.46 3.29h1.42c1.05 0 1.97.74 1.97 2.05v.55c0 .41.33.75.75.75h.01c.41 0 .75-.33.75-.75v-.89c-.01-1.81-1.61-3.16-3.48-3.16zM18 16h1.5v3H18zm2.5 0H22v3h-1.5z",
            }
        }
    }
}

pub fn soap_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M14.25,6C14.66,6,15,6.34,15,6.75S14.66,7.5,14.25,7.5S13.5,7.16,13.5,6.75S13.84,6,14.25,6 M14.25,4.5 C13.01,4.5,12,5.51,12,6.75S13.01,9,14.25,9s2.25-1.01,2.25-2.25S15.49,4.5,14.25,4.5L14.25,4.5z M20,5.5c0.28,0,0.5,0.22,0.5,0.5 S20.28,6.5,20,6.5S19.5,6.28,19.5,6S19.72,5.5,20,5.5 M20,4c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S21.1,4,20,4L20,4z M18,2.5 C18,3.33,17.33,4,16.5,4C15.67,4,15,3.33,15,2.5S15.67,1,16.5,1C17.33,1,18,1.67,18,2.5z M1.94,11.79C1.34,12.35,1,13.14,1,13.97 V20c0,1.66,1.34,3,3,3l13.68,0c0.65,0,1.25-0.47,1.32-1.12c0.08-0.75-0.51-1.38-1.24-1.38H12.5c-0.28,0-0.5-0.22-0.5-0.5v0 c0-0.28,0.22-0.5,0.5-0.5l7.18,0c0.65,0,1.25-0.47,1.32-1.12c0.08-0.75-0.51-1.38-1.24-1.38H12.5c-0.28,0-0.5-0.22-0.5-0.5l0,0 c0-0.28,0.22-0.5,0.5-0.5l8.18,0c0.65,0,1.25-0.47,1.32-1.12c0.08-0.75-0.51-1.38-1.24-1.38H12.5c-0.28,0-0.5-0.22-0.5-0.5v0 c0-0.28,0.22-0.5,0.5-0.5l6.18,0c0.65,0,1.25-0.47,1.32-1.12c0.08-0.75-0.51-1.38-1.24-1.38H8.86l1.49-2.61 c0.09-0.16,0.14-0.33,0.14-0.53c0-0.26-0.09-0.5-0.26-0.7L9.81,5.71C9.43,5.32,8.8,5.3,8.4,5.68L1.94,11.79z",
                }
            }
        }
    }
}

pub fn spa_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15.49 9.63c-.16-2.42-1.03-4.79-2.64-6.76-.41-.5-1.16-.5-1.57 0-1.65 1.98-2.57 4.35-2.77 6.76 1.28.68 2.46 1.56 3.49 2.63 1.03-1.06 2.21-1.94 3.49-2.63zm-6.5 2.65c-.14-.1-.3-.19-.45-.29.15.11.31.19.45.29zm6.42-.25c-.13.09-.27.16-.4.26.13-.1.27-.17.4-.26zM12 15.45c-1.95-2.97-5.14-5.03-8.83-5.39-.64-.06-1.17.47-1.11 1.11.45 4.8 3.65 8.78 7.98 10.33.63.23 1.29.4 1.97.51.68-.12 1.33-.29 1.97-.51 4.33-1.55 7.53-5.52 7.98-10.33.06-.64-.48-1.17-1.11-1.11-3.71.36-6.9 2.42-8.85 5.39z",
            }
        }
    }
}

pub fn sports_bar_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M15.5,7.5h-1.27C14.47,7.05,14.6,6.54,14.6,6c0-1.79-1.46-3.25-3.25-3.25c-0.24,0-0.47,0.03-0.69,0.07 C10.05,2.31,9.26,2,8.4,2C6.95,2,5.7,2.89,5.17,4.15C3.91,4.55,3,5.73,3,7.12c0,1.51,1.07,2.77,2.5,3.06v1.54v3.77 C5.5,16.33,6.17,17,7,17h5.5c0.83,0,1.5-0.67,1.5-1.5h1.5c0.83,0,1.5-0.67,1.5-1.5V9C17,8.17,16.33,7.5,15.5,7.5z M10.79,6 c-0.8,0-1.55,0.31-2.12,0.88L7.28,8.27C6.96,8.59,6.51,8.78,6.02,8.75C5.2,8.7,4.54,8.02,4.5,7.2C4.46,6.27,5.2,5.5,6.12,5.5 c0,0,0,0,0,0c0.14,0,0.27-0.1,0.29-0.24C6.53,4.26,7.39,3.5,8.4,3.5c0.71,0,1.34,0.37,1.69,0.93c0.07,0.12,0.23,0.15,0.34,0.08 c0.27-0.16,0.58-0.26,0.92-0.26c0.96,0,1.75,0.79,1.75,1.75H10.79z M15.5,14H14V9h1.5V14z",
            }
        }
    }
}

pub fn sports_bar_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19,9h-1.56c0.33-0.55,0.53-1.18,0.55-1.86c0.04-1.03-0.43-1.99-1.16-2.71c-1.54-1.54-2.74-1.56-3.82-1.29 C12.2,2.45,11.16,2.02,10,2.02c-1.89,0-3.51,1.11-4.27,2.71C4.15,5.26,3,6.74,3,8.5c0,1.86,1.28,3.41,3,3.86L6,19c0,1.1,0.9,2,2,2h7 c1.1,0,2-0.9,2-2v0h2c1.1,0,2-0.9,2-2v-6C21,9.9,20.1,9,19,9z M7,10.5c-1.1,0-2-0.9-2-2c0-0.85,0.55-1.6,1.37-1.88l0.8-0.27 l0.36-0.76C8,4.62,8.94,4.02,10,4.02c0.79,0,1.39,0.35,1.74,0.65l0.78,0.65c0,0,0.64-0.32,1.47-0.32c1.1,0,2,0.9,2,2c0,0-3,0-3,0 C9.67,7,9.15,10.5,7,10.5z M19,17h-2v-6h2V17z",
            }
        }
    }
}

pub fn stairs_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                g {
                    g {
                        path {
                            d: "M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M17,8h-1.42v3.33H13v3.33h-2.58 L10.45,18H7c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h1.42v-3.33H11V9.33h2.58V6H17c0.55,0,1,0.45,1,1C18,7.55,17.55,8,17,8z",
                        }
                    }
                }
            }
        }
    }
}

pub fn storefront_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
            }
            g {
                path {
                    d: "M21.9,7.89l-1.05-3.37c-0.22-0.9-1-1.52-1.91-1.52H5.05c-0.9,0-1.69,0.63-1.9,1.52L2.1,7.89C1.64,9.86,2.95,11,3,11.06V19 c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2v-7.94C22.12,9.94,22.09,8.65,21.9,7.89z M13,5h1.96l0.54,3.52C15.59,9.23,15.11,10,14.22,10 C13.55,10,13,9.41,13,8.69V5z M6.44,8.86C6.36,9.51,5.84,10,5.23,10C4.3,10,3.88,9.03,4.04,8.36L5.05,5h1.97L6.44,8.86z M11,8.69 C11,9.41,10.45,10,9.71,10c-0.75,0-1.3-0.7-1.22-1.48L9.04,5H11V8.69z M18.77,10c-0.61,0-1.14-0.49-1.21-1.14L16.98,5l1.93-0.01 l1.05,3.37C20.12,9.03,19.71,10,18.77,10z",
                }
            }
        }
    }
}

pub fn stroller_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M18,20c0,1.1-0.9,2-2,2s-2-0.9-2-2s0.9-2,2-2S18,18.9,18,20z M6,18c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S7.1,18,6,18z M14.3,4.1C13.03,3.4,11.56,3,10,3C8.49,3,7.07,3.38,5.83,4.03C5.24,4.34,5.15,5.15,5.61,5.61l3.99,3.99L14.3,4.1z M21.94,5.83 C21.65,4.22,20.3,3,18.65,3c-1.66,0-2.54,1.27-3.18,2.03L6.71,15.31c-0.55,0.65-0.09,1.65,0.76,1.65H15c1.1,0,2-0.9,2-2V6.27 C17.58,5.59,17.97,5,18.65,5c0.68,0,1.22,0.52,1.33,1.21l0,0C20.08,6.66,20.48,7,20.96,7c0.55,0,1-0.45,1-1 C21.96,5.94,21.95,5.89,21.94,5.83L21.94,5.83z",
                }
            }
        }
    }
}

pub fn tapas_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22,10V2c0-0.55-0.45-1-1-1h-6c-0.55,0-1,0.45-1,1v8c0,1.86,1.28,3.41,3,3.86V21h-1c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1 h4c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1v-7.14C20.72,13.41,22,11.86,22,10z M20,3v3h-4V3H20z M9.86,9L8,9V8l1.86,0 c1.31,0,2.5-0.94,2.63-2.24C12.64,4.26,11.47,3,10,3H8V2c0-0.55-0.45-1-1-1h0C6.45,1,6,1.45,6,2v1L4.14,3 c-1.31,0-2.5,0.94-2.63,2.24C1.36,6.74,2.53,8,4,8h2v1L4.14,9c-1.31,0-2.5,0.94-2.63,2.24C1.36,12.74,2.53,14,4,14h2v8 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-8h2c1.47,0,2.64-1.26,2.49-2.76C12.36,9.94,11.17,9,9.86,9z",
            }
        }
    }
}

pub fn tty_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M15,4L15,4c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v0C14,4.45,14.45,4,15,4z M14,7L14,7 c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v0C13,7.45,13.45,7,14,7z M12,4L12,4c0.55,0,1,0.45,1,1v0 c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v0C11,4.45,11.45,4,12,4z M17,9L17,9c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h0 c0.55,0,1,0.45,1,1v0C18,8.55,17.55,9,17,9z M18,6L18,6c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v0 C19,5.55,18.55,6,18,6z M20,9L20,9c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v0C21,8.55,20.55,9,20,9z M21,6 L21,6c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v0C22,5.55,21.55,6,21,6z M14.62,14.38L12.1,16.9 c-2.5-1.43-4.57-3.5-6-6l2.52-2.52C8.86,8.14,8.96,7.8,8.9,7.48L8.16,3.8C8.07,3.34,7.66,3,7.18,3H3.03C2.47,3,2,3.47,2.03,4.03 C2.2,6.92,3.05,9.63,4.43,12c1.58,2.73,3.85,4.99,6.57,6.57c2.37,1.37,5.08,2.23,7.97,2.4c0.56,0.03,1.03-0.44,1.03-1v-4.15 c0-0.48-0.34-0.89-0.8-0.98l-3.67-0.73C15.2,14.04,14.86,14.14,14.62,14.38z M15,10L15,10c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h0 c-0.55,0-1-0.45-1-1v0C14,10.45,14.45,10,15,10z M12,10L12,10c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v0 C11,10.45,11.45,10,12,10z M18,12L18,12c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v0C19,11.55,18.55,12,18,12z M21,12L21,12c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v0C22,11.55,21.55,12,21,12z",
                }
            }
        }
    }
}

pub fn umbrella_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M17.12,6.28L14.5,6.92L13,5.77V3.88V3.4c0-0.26,0.22-0.48,0.5-0.48c0.23,0,0.43,0.16,0.49,0.36C14.1,3.7,14.49,4,14.94,4 c0.55,0,1-0.45,1-1c0-0.1-0.02-0.2-0.05-0.3C15.59,1.72,14.63,1,13.5,1C12.12,1,11,2.07,11,3.4v0.48v1.89L9.5,6.92L6.88,6.28 C6.5,6.19,6.16,6.55,6.28,6.92l4.77,14.39C11.2,21.77,11.6,22,12,22s0.8-0.23,0.95-0.69l4.77-14.39 C17.84,6.55,17.5,6.19,17.12,6.28z M11,14.8L9.03,8.86l0.92,0.23l0.76-0.58L11,8.29V14.8z M13,14.8V8.29l0.28,0.22l0.76,0.58 l0.92-0.23L13,14.8z",
                }
            }
        }
    }
}

pub fn vape_free_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.54,17.6c-0.29,0.29-0.77,0.29-1.06,0l-2.6-2.6H7v-2h3.88L2.4,4.52c-0.29-0.29-0.29-0.77,0-1.06s0.77-0.29,1.06,0 l13.08,13.08C16.83,16.83,16.83,17.31,16.54,17.6z M15.12,13H17c0.55,0,1,0.45,1,1c0,0.51-0.39,0.94-0.89,0.99L15.12,13z M9,13.65 c-0.19,0-0.35,0.16-0.35,0.35S8.81,14.35,9,14.35S9.35,14.19,9.35,14S9.19,13.65,9,13.65z M16,10.5C16,9.12,14.88,8,13.5,8h-1 C11.67,8,11,7.33,11,6.5S11.67,5,12.5,5C12.78,5,13,4.78,13,4.5C13,4.22,12.78,4,12.5,4C11.12,4,10,5.12,10,6.5S11.12,9,12.5,9h1 c0.83,0,1.5,0.67,1.5,1.5v1c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5V10.5z M13.5,6.5c0,0.27,0.21,0.49,0.47,0.53 C15.68,7.26,17,8.73,17,10.5v1c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5v-1c0-1.88-1.16-3.5-2.8-4.17C15.69,5.88,16,5.22,16,4.5 c0-1.17-0.8-2.15-1.89-2.42C13.8,2,13.5,2.24,13.5,2.56c0,0.23,0.16,0.43,0.38,0.49C14.53,3.22,15,3.8,15,4.5 c0,0.69-0.46,1.26-1.09,1.44C13.66,6.01,13.5,6.24,13.5,6.5z M3,14.75c1,0,2.02,0.09,3,0.25v-2c-0.98,0.16-2,0.25-3,0.25H2.75 C2.34,13.25,2,13.59,2,14s0.34,0.75,0.75,0.75H3z",
                }
            }
        }
    }
}

pub fn vape_free_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M20.49,21.9c-0.39,0.39-1.02,0.39-1.41,0l-2.9-2.9H8v-3h5.17L2.1,4.93c-0.39-0.39-0.39-1.02,0-1.41s1.02-0.39,1.41,0 l16.97,16.97C20.88,20.88,20.88,21.51,20.49,21.9z M18.83,16h1.67c0.83,0,1.5,0.67,1.5,1.5c0,0.46-0.21,0.87-0.53,1.14L18.83,16z M10.5,17c-0.28,0-0.5,0.22-0.5,0.5s0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5S10.78,17,10.5,17z M18.85,7.73c0.62-0.61,1-1.45,1-2.38 c0-1.51-1-2.79-2.38-3.21C16.99,2,16.5,2.36,16.5,2.86c0,0.33,0.21,0.62,0.52,0.71c0.77,0.23,1.33,0.94,1.33,1.78 c0,0.82-0.53,1.51-1.27,1.76C16.75,7.22,16.5,7.5,16.5,7.85V8c0,0.37,0.27,0.69,0.64,0.75c1.93,0.31,3.36,2,3.36,4.02v1.48 c0,0.41,0.34,0.75,0.75,0.75S22,14.66,22,14.25v-1.49C22,10.54,20.72,8.62,18.85,7.73z M14.48,11.65c0.04,0,0.09,0,0.13,0h1.42 c1.05,0,1.97,0.74,1.97,2.05v0.55c0,0.41,0.34,0.75,0.76,0.75c0.41,0,0.75-0.33,0.75-0.75v-0.89c-0.01-1.81-1.61-3.16-3.48-3.16 h-1.3c-1.02,0-1.94-0.73-2.07-1.75c-0.12-0.95,0.46-1.7,1.3-1.93c0.32-0.09,0.54-0.38,0.54-0.72c0-0.49-0.46-0.86-0.93-0.72 c-1.41,0.41-2.43,1.71-2.42,3.24L14.48,11.65z M3,18.5c1.33,0,2.71,0.18,4,0.5v-3c-1.29,0.32-2.67,0.5-4,0.5c-0.55,0-1,0.45-1,1 S2.45,18.5,3,18.5z",
                }
            }
        }
    }
}

pub fn vaping_rooms_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M18,14c0,0.55-0.45,1-1,1H7v-2h10C17.55,13,18,13.45,18,14z M9,13.65c-0.19,0-0.35,0.16-0.35,0.35S8.81,14.35,9,14.35 S9.35,14.19,9.35,14S9.19,13.65,9,13.65z M16,10.5C16,9.12,14.88,8,13.5,8h-1C11.67,8,11,7.33,11,6.5S11.67,5,12.5,5 C12.78,5,13,4.78,13,4.5C13,4.22,12.78,4,12.5,4C11.12,4,10,5.12,10,6.5S11.12,9,12.5,9h1c0.83,0,1.5,0.67,1.5,1.5v1 c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5V10.5z M13.5,6.5c0,0.27,0.21,0.49,0.47,0.53C15.68,7.26,17,8.73,17,10.5v1 c0,0.28,0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5v-1c0-1.88-1.16-3.5-2.8-4.17C15.69,5.88,16,5.22,16,4.5c0-1.17-0.8-2.15-1.89-2.42 C13.8,2,13.5,2.24,13.5,2.56c0,0.23,0.16,0.43,0.38,0.49C14.53,3.22,15,3.8,15,4.5c0,0.69-0.46,1.26-1.09,1.44 C13.66,6.01,13.5,6.24,13.5,6.5z M3,14.75c1,0,2.02,0.09,3,0.25v-2c-0.98,0.16-2,0.25-3,0.25H2.75C2.34,13.25,2,13.59,2,14 s0.34,0.75,0.75,0.75H3z",
                }
            }
        }
    }
}

pub fn vaping_rooms_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M22,17.5c0,0.83-0.67,1.5-1.5,1.5H8v-3h12.5C21.33,16,22,16.67,22,17.5z M10.5,17c-0.28,0-0.5,0.22-0.5,0.5 s0.22,0.5,0.5,0.5s0.5-0.22,0.5-0.5S10.78,17,10.5,17z M18.85,7.73c0.62-0.61,1-1.45,1-2.38c0-1.51-1-2.79-2.38-3.21 C16.99,2,16.5,2.36,16.5,2.86c0,0.33,0.21,0.62,0.52,0.71c0.77,0.23,1.33,0.94,1.33,1.78c0,0.82-0.53,1.51-1.27,1.76 C16.75,7.22,16.5,7.5,16.5,7.85V8c0,0.37,0.27,0.69,0.64,0.75c1.93,0.31,3.36,2,3.36,4.02v1.48c0,0.41,0.34,0.75,0.75,0.75 S22,14.66,22,14.25v-1.49C22,10.54,20.72,8.62,18.85,7.73z M18.76,15c0.41,0,0.75-0.33,0.75-0.75v-0.89 c-0.01-1.81-1.61-3.16-3.48-3.16h-1.3c-1.02,0-1.94-0.73-2.07-1.75c-0.12-0.95,0.46-1.7,1.3-1.93c0.32-0.09,0.54-0.38,0.54-0.72 c0-0.49-0.46-0.86-0.93-0.72c-1.42,0.41-2.45,1.73-2.42,3.28c0.03,1.84,1.62,3.29,3.46,3.29h1.42c1.05,0,1.97,0.74,1.97,2.05v0.55 C18,14.66,18.34,15,18.76,15z M3,18.5c1.33,0,2.71,0.18,4,0.5v-3c-1.29,0.32-2.67,0.5-4,0.5c-0.55,0-1,0.45-1,1S2.45,18.5,3,18.5z",
                }
            }
        }
    }
}

pub fn villa_icons_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M5,17H4c-0.55,0-1-0.45-1-1V7.57c0-0.41,0.25-0.78,0.64-0.93l8-3.11C12.29,3.27,13,3.76,13,4.46V8.5H7c-0.55,0-1,0.45-1,1 V16C6,16.55,5.55,17,5,17z M12.25,13c0.41,0,0.75,0.34,0.75,0.75V17h3c0.55,0,1-0.45,1-1v-6c0-0.83-0.67-1.5-1.5-1.5S14,9.17,14,10 H8.5c-0.55,0-1,0.45-1,1v5c0,0.55,0.45,1,1,1h3v-3.25C11.5,13.34,11.84,13,12.25,13z",
            }
        }
    }
}

pub fn villa_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7,21H4c-0.55,0-1-0.45-1-1V8.69C3,8.27,3.25,7.9,3.64,7.75l11-4.23C15.3,3.27,16,3.75,16,4.46V10H8c-0.55,0-1,0.45-1,1V21z M17,12h-7c-0.55,0-1,0.45-1,1v7c0,0.55,0.45,1,1,1h4v-4c0-0.55,0.45-1,1-1s1,0.45,1,1v4h4c0.55,0,1-0.45,1-1v-8c0-1.1-0.9-2-2-2 S17,10.9,17,12z",
            }
        }
    }
}

pub fn wash_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M1.94,11.79C1.34,12.35,1,13.14,1,13.97V20c0,1.66,1.34,3,3,3l13.68,0c0.65,0,1.25-0.47,1.32-1.12 c0.08-0.75-0.51-1.38-1.24-1.38H12.5c-0.28,0-0.5-0.22-0.5-0.5v0c0-0.28,0.22-0.5,0.5-0.5l7.18,0c0.65,0,1.25-0.47,1.32-1.12 c0.08-0.75-0.51-1.38-1.24-1.38H12.5c-0.28,0-0.5-0.22-0.5-0.5l0,0c0-0.28,0.22-0.5,0.5-0.5l8.18,0c0.65,0,1.25-0.47,1.32-1.12 c0.08-0.75-0.51-1.38-1.24-1.38H12.5c-0.28,0-0.5-0.22-0.5-0.5v0c0-0.28,0.22-0.5,0.5-0.5l6.18,0c0.65,0,1.25-0.47,1.32-1.12 c0.08-0.75-0.51-1.38-1.24-1.38H8.86l1.49-2.61c0.09-0.16,0.14-0.33,0.14-0.53c0-0.26-0.09-0.5-0.26-0.7L9.81,5.71 C9.43,5.32,8.8,5.3,8.4,5.68L1.94,11.79z M18.5,8C19.88,8,21,6.88,21,5.5c0-1.25-1.41-3.16-2.11-4.04c-0.2-0.25-0.57-0.25-0.77,0 C17.41,2.34,16,4.25,16,5.5C16,6.88,17.12,8,18.5,8z M13.5,9C14.33,9,15,8.33,15,7.5c0-0.56-0.67-1.49-1.11-2.04 c-0.2-0.25-0.58-0.25-0.77,0C12.67,6.01,12,6.94,12,7.5C12,8.33,12.67,9,13.5,9z",
                }
            }
        }
    }
}

pub fn water_damage_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M11.33,3.6l-8.36,7.53C2.63,11.43,2.84,12,3.3,12H5v7c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1v-7h1.7 c0.46,0,0.68-0.57,0.33-0.87L12.67,3.6C12.29,3.26,11.71,3.26,11.33,3.6z M12,16c-1.1,0-2-0.9-2-2c0-0.78,0.99-2.44,1.58-3.36 c0.2-0.31,0.64-0.31,0.84,0C13.01,11.56,14,13.22,14,14C14,15.1,13.1,16,12,16z",
            }
        }
    }
}

pub fn wheelchair_pickup_icons_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                path {
                    d: "M4.5,4c0-1.11,0.89-2,2-2s2,0.89,2,2s-0.89,2-2,2S4.5,5.11,4.5,4z M10,10.95V9c0-1.1-0.9-2-2-2H5C3.9,7,3,7.9,3,9v5 c0,0.55,0.45,1,1,1h1v6c0,0.55,0.45,1,1,1h2.5v-0.11c-1.24-1.26-2-2.99-2-4.89C6.5,14.42,7.91,12.16,10,10.95z M16.5,17 c0,1.65-1.35,3-3,3s-3-1.35-3-3c0-1.11,0.61-2.06,1.5-2.58v-2.16C9.98,12.9,8.5,14.77,8.5,17c0,2.76,2.24,5,5,5s5-2.24,5-5H16.5z M19,14h-4V9c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v6c0,0.55,0.45,1,1,1h4.46l1.92,2.88c0.31,0.46,0.93,0.58,1.39,0.28h0 c0.46-0.31,0.58-0.93,0.28-1.39l-2.21-3.32C19.65,14.17,19.33,14,19,14z",
                }
            }
        }
    }
}

