use dioxus::prelude::*;
use crate::IconProps;
pub fn account_tree_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22 11V3h-7v3H9V3H2v8h7V8h2v10h4v3h7v-8h-7v3h-2V8h2v3z",
            }
        }
    }
}

pub fn adb_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5 16c0 3.87 3.13 7 7 7s7-3.13 7-7v-4H5v4zM16.12 4.37l2.1-2.1-.82-.83-2.3 2.31C14.16 3.28 13.12 3 12 3s-2.16.28-3.09.75L6.6 1.44l-.82.83 2.1 2.1C6.14 5.64 5 7.68 5 10v1h14v-1c0-2.32-1.14-4.36-2.88-5.63zM9 9c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1zm6 0c-.55 0-1-.45-1-1s.45-1 1-1 1 .45 1 1-.45 1-1 1z",
            }
        }
    }
}

pub fn add_call_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M20 15.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM21 6h-3V3h-2v3h-3v2h3v3h2V8h3z",
            }
        }
    }
}

pub fn airline_seat_flat_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22 11v2H9V7h9c2.21 0 4 1.79 4 4zM2 14v2h6v2h8v-2h6v-2H2zm5.14-1.9c1.16-1.19 1.14-3.08-.04-4.24-1.19-1.16-3.08-1.14-4.24.04-1.16 1.19-1.14 3.08.04 4.24 1.19 1.16 3.08 1.14 4.24-.04z",
            }
        }
    }
}

pub fn airline_seat_flat_angled_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22.25 14.29l-.69 1.89L9.2 11.71l2.08-5.66 8.56 3.09c2.1.76 3.18 3.06 2.41 5.15zM1.5 12.14L8 14.48V19h8v-1.63L20.52 19l.69-1.89-19.02-6.86-.69 1.89zm5.8-1.94c1.49-.72 2.12-2.51 1.41-4C7.99 4.71 6.2 4.08 4.7 4.8c-1.49.71-2.12 2.5-1.4 4 .71 1.49 2.5 2.12 4 1.4z",
            }
        }
    }
}

pub fn airline_seat_individual_suite_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7 13c1.65 0 3-1.35 3-3S8.65 7 7 7s-3 1.35-3 3 1.35 3 3 3zm12-6h-8v7H3V7H1v10h22v-6c0-2.21-1.79-4-4-4z",
            }
        }
    }
}

pub fn airline_seat_legroom_extra_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M4 12V3H2v9c0 2.76 2.24 5 5 5h6v-2H7c-1.66 0-3-1.34-3-3zm18.83 5.24c-.38-.72-1.29-.97-2.03-.63l-1.09.5-3.41-6.98c-.34-.68-1.03-1.12-1.79-1.12L11 9V3H5v8c0 1.66 1.34 3 3 3h7l3.41 7 3.72-1.7c.77-.36 1.1-1.3.7-2.06z",
            }
        }
    }
}

pub fn airline_seat_legroom_normal_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5 12V3H3v9c0 2.76 2.24 5 5 5h6v-2H8c-1.66 0-3-1.34-3-3zm15.5 6H19v-7c0-1.1-.9-2-2-2h-5V3H6v8c0 1.65 1.35 3 3 3h7v7h4.5c.83 0 1.5-.67 1.5-1.5s-.67-1.5-1.5-1.5z",
            }
        }
    }
}

pub fn airline_seat_legroom_reduced_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M19.97 19.2c.18.96-.55 1.8-1.47 1.8H14v-3l1-4H9c-1.65 0-3-1.35-3-3V3h6v6h5c1.1 0 2 .9 2 2l-2 7h1.44c.73 0 1.39.49 1.53 1.2zM5 12V3H3v9c0 2.76 2.24 5 5 5h4v-2H8c-1.66 0-3-1.34-3-3z",
            }
        }
    }
}

pub fn airline_seat_recline_extra_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5.35 5.64c-.9-.64-1.12-1.88-.49-2.79.63-.9 1.88-1.12 2.79-.49.9.64 1.12 1.88.49 2.79-.64.9-1.88 1.12-2.79.49zM16 19H8.93c-1.48 0-2.74-1.08-2.96-2.54L4 7H2l1.99 9.76C4.37 19.2 6.47 21 8.94 21H16v-2zm.23-4h-4.88l-1.03-4.1c1.58.89 3.28 1.54 5.15 1.22V9.99c-1.63.31-3.44-.27-4.69-1.25L9.14 7.47c-.23-.18-.49-.3-.76-.38-.32-.09-.66-.12-.99-.06h-.02c-1.23.22-2.05 1.39-1.84 2.61l1.35 5.92C7.16 16.98 8.39 18 9.83 18h6.85l3.82 3 1.5-1.5-5.77-4.5z",
            }
        }
    }
}

pub fn airline_seat_recline_normal_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M7.59 5.41c-.78-.78-.78-2.05 0-2.83.78-.78 2.05-.78 2.83 0 .78.78.78 2.05 0 2.83-.79.79-2.05.79-2.83 0zM6 16V7H4v9c0 2.76 2.24 5 5 5h6v-2H9c-1.66 0-3-1.34-3-3zm14 4.07L14.93 15H11.5v-3.68c1.4 1.15 3.6 2.16 5.5 2.16v-2.16c-1.66.02-3.61-.87-4.67-2.04l-1.4-1.55c-.19-.21-.43-.38-.69-.5-.29-.14-.62-.23-.96-.23h-.03C8.01 7 7 8.01 7 9.25V15c0 1.66 1.34 3 3 3h5.07l3.5 3.5L20 20.07z",
            }
        }
    }
}

pub fn bluetooth_audio_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14.24 12.01l2.32 2.32c.28-.72.44-1.51.44-2.33 0-.82-.16-1.59-.43-2.31l-2.33 2.32zm5.29-5.3l-1.26 1.26c.63 1.21.98 2.57.98 4.02s-.36 2.82-.98 4.02l1.2 1.2c.97-1.54 1.54-3.36 1.54-5.31-.01-1.89-.55-3.67-1.48-5.19zm-3.82 1L10 2H9v7.59L4.41 5 3 6.41 8.59 12 3 17.59 4.41 19 9 14.41V22h1l5.71-5.71-4.3-4.29 4.3-4.29zM11 5.83l1.88 1.88L11 9.59V5.83zm1.88 10.46L11 18.17v-3.76l1.88 1.88z",
            }
        }
    }
}

pub fn confirmation_number_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    g {
                        path {
                            d: "M22,10V6c0-1.11-0.9-2-2-2H4C2.9,4,2.01,4.89,2.01,6v4C3.11,10,4,10.9,4,12s-0.89,2-2,2v4c0,1.1,0.9,2,2,2h16 c1.1,0,2-0.9,2-2v-4c-1.1,0-2-0.9-2-2S20.9,10,22,10z M13,17.5h-2v-2h2V17.5z M13,13h-2v-2h2V13z M13,8.5h-2v-2h2V8.5z",
                        }
                    }
                }
            }
        }
    }
}

pub fn directions_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M12.83 10h.67V7.5L17 11l-1.59 1.59L18.83 16l2.59-2.59c.78-.78.78-2.05 0-2.83l-7.99-8c-.78-.78-2.05-.78-2.83 0L8 5.17 12.83 10zM2.81 2.81 1.39 4.22 5.17 8l-2.59 2.59c-.78.78-.78 2.05 0 2.83l8.01 8c.78.78 2.05.78 2.83 0L16 18.83l3.78 3.78 1.41-1.41L2.81 2.81zM10 15H8v-4c0-.05.02-.09.03-.14L10 12.83V15z",
                    }
                }
            }
        }
    }
}

pub fn disc_full_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 16h2v-2h-2v2zm0-9v5h2V7h-2zM10 4c-4.42 0-8 3.58-8 8s3.58 8 8 8 8-3.58 8-8-3.58-8-8-8zm0 10c-1.1 0-2-.9-2-2s.9-2 2-2 2 .9 2 2-.9 2-2 2z",
            }
        }
    }
}

pub fn do_disturb_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    g {
                        path {
                            d: "M12,2C6.5,2,2,6.5,2,12s4.5,10,10,10s10-4.5,10-10S17.5,2,12,2z M4,12c0-4.4,3.6-8,8-8c1.8,0,3.5,0.6,4.9,1.7L5.7,16.9 C4.6,15.5,4,13.8,4,12z M12,20c-1.8,0-3.5-0.6-4.9-1.7L18.3,7.1C19.4,8.5,20,10.2,20,12C20,16.4,16.4,20,12,20z",
                        }
                    }
                }
            }
        }
    }
}

pub fn do_disturb_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                            d: "M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.42,0-8-3.58-8-8 c0-1.85,0.63-3.55,1.69-4.9L16.9,18.31C15.55,19.37,13.85,20,12,20z M18.31,16.9L7.1,5.69C8.45,4.63,10.15,4,12,4 c4.42,0,8,3.58,8,8C20,13.85,19.37,15.55,18.31,16.9z",
                        }
                    }
                }
            }
        }
    }
}

pub fn do_disturb_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M17,11v2h-1.17l4.51,4.51C21.39,15.93,22,14.04,22,12c0-5.52-4.48-10-10-10C9.96,2,8.07,2.61,6.49,3.66L13.83,11H17z M1.39,4.22l2.27,2.27C2.61,8.07,2,9.96,2,12c0,5.52,4.48,10,10,10c2.04,0,3.93-0.61,5.51-1.66l2.27,2.27l1.41-1.41L2.81,2.81 L1.39,4.22z M7,11h1.17l2,2H7V11z",
                }
            }
        }
    }
}

pub fn do_disturb_on_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                        d: "M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M17,13H7v-2h10V13z",
                    }
                }
            }
        }
    }
}

pub fn do_not_disturb_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8 0-1.85.63-3.55 1.69-4.9L16.9 18.31C15.55 19.37 13.85 20 12 20zm6.31-3.1L7.1 5.69C8.45 4.63 10.15 4 12 4c4.42 0 8 3.58 8 8 0 1.85-.63 3.55-1.69 4.9z",
            }
        }
    }
}

pub fn do_not_disturb_alt_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M-618-1464H782v3600H-618zM0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10 10-4.5 10-10S17.5 2 12 2zM4 12c0-4.4 3.6-8 8-8 1.8 0 3.5.6 4.9 1.7L5.7 16.9C4.6 15.5 4 13.8 4 12zm8 8c-1.8 0-3.5-.6-4.9-1.7L18.3 7.1C19.4 8.5 20 10.2 20 12c0 4.4-3.6 8-8 8z",
            }
        }
    }
}

pub fn do_not_disturb_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 11v2h-1.46l4.68 4.68C21.34 16.07 22 14.11 22 12c0-5.52-4.48-10-10-10-2.11 0-4.07.66-5.68 1.78L13.54 11H17zM2.27 2.27L1 3.54l2.78 2.78C2.66 7.93 2 9.89 2 12c0 5.52 4.48 10 10 10 2.11 0 4.07-.66 5.68-1.78L20.46 23l1.27-1.27L11 11 2.27 2.27zM7 13v-2h1.46l2 2H7z",
            }
        }
    }
}

pub fn do_not_disturb_on_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 11H7v-2h10v2z",
            }
        }
    }
}

pub fn drive_eta_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.92 5.01C18.72 4.42 18.16 4 17.5 4h-11c-.66 0-1.21.42-1.42 1.01L3 11v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.5 15c-.83 0-1.5-.67-1.5-1.5S5.67 12 6.5 12s1.5.67 1.5 1.5S7.33 15 6.5 15zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 10l1.5-4.5h11L19 10H5z",
            }
        }
    }
}

pub fn enhanced_encryption_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M18 8h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zM8.9 6c0-1.71 1.39-3.1 3.1-3.1s3.1 1.39 3.1 3.1v2H8.9V6zM16 16h-3v3h-2v-3H8v-2h3v-3h2v3h3v2z",
            }
        }
    }
}

pub fn event_available_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M16.53 11.06L15.47 10l-4.88 4.88-2.12-2.12-1.06 1.06L10.59 17l5.94-5.94zM19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11z",
            }
        }
    }
}

pub fn event_busy_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M9.31 17l2.44-2.44L14.19 17l1.06-1.06-2.44-2.44 2.44-2.44L14.19 10l-2.44 2.44L9.31 10l-1.06 1.06 2.44 2.44-2.44 2.44L9.31 17zM19 3h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11z",
            }
        }
    }
}

pub fn event_note_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 10H7v2h10v-2zm2-7h-1V1h-2v2H8V1H6v2H5c-1.11 0-1.99.9-1.99 2L3 19c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V8h14v11zm-5-5H7v2h7v-2z",
            }
        }
    }
}

pub fn folder_special_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2zm-2.06 11L15 15.28 12.06 17l.78-3.33-2.59-2.24 3.41-.29L15 8l1.34 3.14 3.41.29-2.59 2.24.78 3.33z",
            }
        }
    }
}

pub fn imagesearch_roller_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 2v6H6V6H4v4h10v5h2v8h-6v-8h2v-3H2V4h4V2",
            }
        }
    }
}

pub fn live_tv_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 6h-7.59l3.29-3.29L16 2l-4 4-4-4-.71.71L10.59 6H3c-1.1 0-2 .89-2 2v12c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V8c0-1.11-.9-2-2-2zm0 14H3V8h18v12zM9 10v8l7-4z",
            }
        }
    }
}

pub fn mms_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM5 14l3.5-4.5 2.5 3.01L14.5 8l4.5 6H5z",
            }
        }
    }
}

pub fn more_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22 3H7c-.69 0-1.23.35-1.59.88L0 12l5.41 8.11c.36.53.97.89 1.66.89H22c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 13.5c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm5 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zm5 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5z",
            }
        }
    }
}

pub fn network_check_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M15.9 5c-.17 0-.32.09-.41.23l-.07.15-5.18 11.65c-.16.29-.26.61-.26.96 0 1.11.9 2.01 2.01 2.01.96 0 1.77-.68 1.96-1.59l.01-.03L16.4 5.5c0-.28-.22-.5-.5-.5zM1 9l2 2c2.88-2.88 6.79-4.08 10.53-3.62l1.19-2.68C9.89 3.84 4.74 5.27 1 9zm20 2l2-2c-1.64-1.64-3.55-2.82-5.59-3.57l-.53 2.82c1.5.62 2.9 1.53 4.12 2.75zm-4 4l2-2c-.8-.8-1.7-1.42-2.66-1.89l-.55 2.92c.42.27.83.59 1.21.97zM5 13l2 2c1.13-1.13 2.56-1.79 4.03-2l1.28-2.88c-2.63-.08-5.3.87-7.31 2.88z",
            }
        }
    }
}

pub fn network_locked_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
            width: props.width.unwrap_or("24".to_string()),
            height: props.height.unwrap_or("24".to_string()),
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            stroke_linecap: props.stroke_linecap,
            stroke_linejoin: props.stroke_linejoin,
            path {
                d: "M0 0h24v24H0zm0 0h24v24H0z",
                fill: "none",
            }
            path {
                d: "M19.5 10c.17 0 .33.03.5.05V1L1 20h13v-3c0-.89.39-1.68 1-2.23v-.27c0-2.48 2.02-4.5 4.5-4.5zm2.5 6v-1.5c0-1.38-1.12-2.5-2.5-2.5S17 13.12 17 14.5V16c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h5c.55 0 1-.45 1-1v-4c0-.55-.45-1-1-1zm-1 0h-3v-1.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5V16z",
            }
        }
    }
}

pub fn no_encryption_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M0 0h24v24H0zm0 0h24v24H0zm0 0h24v24H0V0zm0 0h24v24H0V0z",
            }
            path {
                d: "M21 21.78L4.22 5 3 6.22l2.04 2.04C4.42 8.6 4 9.25 4 10v10c0 1.1.9 2 2 2h12c.23 0 .45-.05.66-.12L19.78 23 21 21.78zM8.9 6c0-1.71 1.39-3.1 3.1-3.1s3.1 1.39 3.1 3.1v2H9.66L20 18.34V10c0-1.1-.9-2-2-2h-1V6c0-2.76-2.24-5-5-5-2.56 0-4.64 1.93-4.94 4.4L8.9 7.24V6z",
            }
        }
    }
}

pub fn no_encryption_gmailerrorred_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M8.9,6c0-1.71,1.39-3.1,3.1-3.1s3.1,1.39,3.1,3.1v2h-4.27L20,17.17V10c0-1.1-0.9-2-2-2h-1V6c0-2.76-2.24-5-5-5 C9.79,1,7.93,2.45,7.27,4.44L8.9,6.07V6z M2.1,2.1L0.69,3.51L5.3,8.13C4.55,8.42,4,9.15,4,10v10c0,1.1,0.9,2,2,2h12 c0.34,0,0.65-0.09,0.93-0.24l1.56,1.56l1.41-1.41L2.1,2.1z M12,17c-1.1,0-2-0.9-2-2c0-0.59,0.27-1.12,0.68-1.49l2.81,2.81 C13.12,16.73,12.59,17,12,17z",
                }
            }
        }
    }
}

pub fn ondemand_video_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 3H3c-1.11 0-2 .89-2 2v12c0 1.1.89 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.11-.9-2-2-2zm0 14H3V5h18v12zm-5-6l-7 4V7z",
            }
        }
    }
}

pub fn personal_video_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M21 3H3c-1.11 0-2 .89-2 2v12c0 1.1.89 2 2 2h5v2h8v-2h5c1.1 0 1.99-.9 1.99-2L23 5c0-1.11-.9-2-2-2zm0 14H3V5h18v12z",
            }
        }
    }
}

pub fn phone_bluetooth_speaker_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M14.71 9.5L17 7.21V11h.5l2.85-2.85L18.21 6l2.15-2.15L17.5 1H17v3.79L14.71 2.5l-.71.71L16.79 6 14 8.79l.71.71zM18 2.91l.94.94-.94.94V2.91zm0 4.3l.94.94-.94.94V7.21zm2 8.29c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1z",
            }
        }
    }
}

pub fn phone_callback_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M6.62 10.79c1.44 2.83 3.76 5.14 6.59 6.59l2.2-2.2c.27-.27.67-.36 1.02-.24 1.12.37 2.33.57 3.57.57.55 0 1 .45 1 1V20c0 .55-.45 1-1 1-9.39 0-17-7.61-17-17 0-.55.45-1 1-1h3.5c.55 0 1 .45 1 1 0 1.25.2 2.45.57 3.57.11.35.03.74-.25 1.02l-2.2 2.2zm13.54-7.1l-.71-.7L13 9.29V5h-1v6h6v-1h-4.15z",
            }
        }
    }
}

pub fn phone_forwarded_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18 11l5-5-5-5v3h-4v4h4v3zm2 4.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1z",
            }
        }
    }
}

pub fn phone_in_talk_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 15.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM19 12h2c0-4.97-4.03-9-9-9v2c3.87 0 7 3.13 7 7zm-4 0h2c0-2.76-2.24-5-5-5v2c1.66 0 3 1.34 3 3z",
            }
        }
    }
}

pub fn phone_locked_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
            g {
                path {
                    d: "M17,13.18L17,16c0,0.59-0.51,1.04-1.1,1C9.04,16.46,3.54,10.96,3,4.1C2.96,3.51,3.41,3,4,3l2.82,0 C7.29,3,7.7,3.33,7.8,3.78L8.3,6.01c0.07,0.33-0.03,0.68-0.27,0.92L5.98,8.98c1.15,2.11,2.89,3.85,5,5l2.08-2 c0.24-0.24,0.59-0.34,0.92-0.27l2.23,0.49C16.67,12.3,17,12.71,17,13.18z M17,5.5v3C17,8.78,16.78,9,16.5,9h-4 C12.22,9,12,8.78,12,8.5v-3C12,5.22,12.22,5,12.5,5H13V4c0-0.82,0.67-1.5,1.5-1.5S16,3.18,16,4v1h0.5C16.78,5,17,5.22,17,5.5z M13.75,5h1.5V4c0-0.41-0.34-0.75-0.75-0.75c-0.41,0-0.75,0.34-0.75,0.75V5z",
                }
            }
        }
    }
}

pub fn phone_locked_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                        d: "M20,5V4c0-1.1-0.9-2-2-2s-2,0.9-2,2v1h-1v5h6V5H20z M19,5h-2V4c0-0.55,0.45-1,1-1s1,0.45,1,1V5z",
                    }
                    path {
                        d: "M15.63,14.4l-2.52,2.5c-2.5-1.43-4.57-3.5-6-6l2.5-2.52c0.23-0.24,0.33-0.57,0.27-0.9L9.13,3.8C9.04,3.34,8.63,3,8.15,3 L4,3C3.44,3,2.97,3.47,3,4.03C3.17,6.92,4.05,9.63,5.43,12c1.58,2.73,3.85,4.99,6.57,6.57c2.37,1.37,5.08,2.26,7.97,2.43 c0.56,0.03,1.03-0.44,1.03-1l0-4.15c0-0.48-0.34-0.89-0.8-0.98l-3.67-0.73C16.2,14.07,15.86,14.17,15.63,14.4z",
                    }
                }
            }
        }
    }
}

pub fn phone_missed_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M6.5 5.5L12 11l7-7-1-1-6 6-4.5-4.5H11V3H5v6h1.5V5.5zm17.21 11.17C20.66 13.78 16.54 12 12 12 7.46 12 3.34 13.78.29 16.67c-.18.18-.29.43-.29.71s.11.53.29.71l2.48 2.48c.18.18.43.29.71.29.27 0 .52-.11.7-.28.79-.74 1.69-1.36 2.66-1.85.33-.16.56-.5.56-.9v-3.1c1.45-.48 3-.73 4.6-.73 1.6 0 3.15.25 4.6.72v3.1c0 .39.23.74.56.9.98.49 1.87 1.12 2.67 1.85.18.18.43.28.7.28.28 0 .53-.11.71-.29l2.48-2.48c.18-.18.29-.43.29-.71s-.12-.52-.3-.7z",
            }
        }
    }
}

pub fn phone_paused_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 3h-2v7h2V3zm3 12.5c-1.25 0-2.45-.2-3.57-.57-.35-.11-.74-.03-1.02.24l-2.2 2.2c-2.83-1.44-5.15-3.75-6.59-6.59l2.2-2.21c.28-.26.36-.65.25-1C8.7 6.45 8.5 5.25 8.5 4c0-.55-.45-1-1-1H4c-.55 0-1 .45-1 1 0 9.39 7.61 17 17 17 .55 0 1-.45 1-1v-3.5c0-.55-.45-1-1-1zM19 3v7h2V3h-2z",
            }
        }
    }
}

pub fn power_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M16.01 7L16 3h-2v4h-4V3H8v4h-.01C7 6.99 6 7.99 6 8.99v5.49L9.5 18v3h5v-3l3.5-3.51v-5.5c0-1-1-2-1.99-1.99z",
            }
        }
    }
}

pub fn power_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18 14.49V9c0-1-1.01-2.01-2-2V3h-2v4h-4V3H8v2.48l9.51 9.5.49-.49zm-1.76 1.77L7.2 7.2l-.01.01L3.98 4 2.71 5.25l3.36 3.36C6.04 8.74 6 8.87 6 9v5.48L9.5 18v3h5v-3l.48-.48L19.45 22l1.26-1.28-4.47-4.46z",
            }
        }
    }
}

pub fn priority_high_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                cy: "19",
                cx: "12",
                r: "2",
            }
            path {
                d: "M10 3h4v12h-4z",
            }
        }
    }
}

pub fn running_with_errors_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
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
                d: "M18,8v7h-1.5V8H18z M16.5,16.5V18H18v-1.5H16.5z M10,3.5c-3.58,0-6.5,2.92-6.5,6.5s2.92,6.5,6.5,6.5 c2.01,0,3.81-0.92,5-2.35l0,2.1c-1.37,1.1-3.11,1.75-5,1.75c-4.42,0-8-3.58-8-8s3.58-8,8-8l0,0c2.46,0,4.67,1.11,6.13,2.87L10,11 V3.5z",
            }
        }
    }
}

pub fn running_with_errors_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22,10v8h-2v-8H22z M20,20v2h2v-2H20z M18,17.29C16.53,18.95,14.39,20,12,20c-4.41,0-8-3.59-8-8c0-4.41,3.59-8,8-8v9 l7.55-7.55C17.72,3.34,15.02,2,12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10c2.25,0,4.33-0.74,6-2V17.29z",
            }
        }
    }
}

pub fn sd_card_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18 2h-8L4.02 8 4 20c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-6 6h-2V4h2v4zm3 0h-2V4h2v4zm3 0h-2V4h2v4z",
            }
        }
    }
}

pub fn sd_card_alert_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                            d: "M18,2h-8L4.02,8L4,20c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2z M13,17h-2v-2h2V17z M13,13h-2V8h2V13z",
                        }
                    }
                }
            }
        }
    }
}

pub fn sim_card_alert_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18 2h-8L4.02 8 4 20c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-5 15h-2v-2h2v2zm0-4h-2V8h2v5z",
            }
        }
    }
}

pub fn sms_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM9 11H7V9h2v2zm4 0h-2V9h2v2zm4 0h-2V9h2v2z",
            }
        }
    }
}

pub fn sms_failed_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M20 2H4c-1.1 0-1.99.9-1.99 2L2 22l4-4h14c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-7 12h-2v-2h2v2zm0-4h-2V6h2v4z",
            }
        }
    }
}

pub fn support_agent_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M10,6C9.32,6,6.12,6.51,6.01,9.88c1.72-0.4,3.06-1.77,3.4-3.51c0.53,1.15,1.96,2.8,4.43,2.6C13.39,7.26,11.85,6,10,6z",
                    }
                    circle {
                        cy: "10.75",
                        cx: "7.5",
                        r: ".75",
                    }
                    circle {
                        cx: "12.5",
                        cy: "10.75",
                        r: ".75",
                    }
                    path {
                        d: "M16,10c0-3.31-2.69-6-6-6s-6,2.69-6,6c-0.55,0-1,0.45-1,1v2c0,0.55,0.45,1,1,1h1v-4c0-2.76,2.24-5,5-5s5,2.24,5,5v5H9v1h6 c0.55,0,1-0.45,1-1v-1c0.55,0,1-0.45,1-1v-2C17,10.45,16.55,10,16,10z",
                    }
                }
            }
        }
    }
}

pub fn support_agent_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M21,12.22C21,6.73,16.74,3,12,3c-4.69,0-9,3.65-9,9.28C2.4,12.62,2,13.26,2,14v2c0,1.1,0.9,2,2,2h1v-6.1 c0-3.87,3.13-7,7-7s7,3.13,7,7V19h-8v2h8c1.1,0,2-0.9,2-2v-1.22c0.59-0.31,1-0.92,1-1.64v-2.3C22,13.14,21.59,12.53,21,12.22z",
                    }
                    circle {
                        cx: "9",
                        cy: "13",
                        r: "1",
                    }
                    circle {
                        r: "1",
                        cy: "13",
                        cx: "15",
                    }
                    path {
                        d: "M18,11.03C17.52,8.18,15.04,6,12.05,6c-3.03,0-6.29,2.51-6.03,6.45c2.47-1.01,4.33-3.21,4.86-5.89 C12.19,9.19,14.88,11,18,11.03z",
                    }
                }
            }
        }
    }
}

pub fn sync_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M12 4V1L8 5l4 4V6c3.31 0 6 2.69 6 6 0 1.01-.25 1.97-.7 2.8l1.46 1.46C19.54 15.03 20 13.57 20 12c0-4.42-3.58-8-8-8zm0 14c-3.31 0-6-2.69-6-6 0-1.01.25-1.97.7-2.8L5.24 7.74C4.46 8.97 4 10.43 4 12c0 4.42 3.58 8 8 8v3l4-4-4-4v3z",
            }
        }
    }
}

pub fn sync_disabled_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M0 0h24v24H0zm0 0h24v24H0z",
            }
            path {
                d: "M10 6.35V4.26c-.8.21-1.55.54-2.23.96l1.46 1.46c.25-.12.5-.24.77-.33zm-7.14-.94l2.36 2.36C4.45 8.99 4 10.44 4 12c0 2.21.91 4.2 2.36 5.64L4 20h6v-6l-2.24 2.24C6.68 15.15 6 13.66 6 12c0-1 .25-1.94.68-2.77l8.08 8.08c-.25.13-.5.25-.77.34v2.09c.8-.21 1.55-.54 2.23-.96l2.36 2.36 1.27-1.27L4.14 4.14 2.86 5.41zM20 4h-6v6l2.24-2.24C17.32 8.85 18 10.34 18 12c0 1-.25 1.94-.68 2.77l1.46 1.46C19.55 15.01 20 13.56 20 12c0-2.21-.91-4.2-2.36-5.64L20 4z",
            }
        }
    }
}

pub fn sync_lock_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M16.5,14H16v-1c0-0.82-0.67-1.5-1.5-1.5S13,12.18,13,13v1h-0.5c-0.28,0-0.5,0.22-0.5,0.5v3c0,0.28,0.22,0.5,0.5,0.5h4 c0.28,0,0.5-0.22,0.5-0.5v-3C17,14.22,16.78,14,16.5,14z M13.75,13c0-0.41,0.34-0.75,0.75-0.75c0.41,0,0.75,0.34,0.75,0.75v1h-1.5 V13z M8.5,4.2v1.58c-1.74,0.62-3,2.27-3,4.22c0,1.32,0.59,2.5,1.5,3.32V12h1.5v4h-4v-1.5h1.55C4.8,13.4,4,11.8,4,10 C4,7.21,5.91,4.86,8.5,4.2z M14.5,10c-0.08-1.32-0.59-2.5-1.5-3.32V8h-1.5V4h4v1.5h-1.55C15.2,6.6,16,8.2,16,10H14.5z",
                    }
                }
            }
        }
    }
}

pub fn sync_lock_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M10,4.26v2.09C7.67,7.18,6,9.39,6,12c0,1.77,0.78,3.34,2,4.44V14h2v6H4v-2h2.73C5.06,16.54,4,14.4,4,12 C4,8.27,6.55,5.15,10,4.26z M20,4h-6v6h2V7.56c1.22,1.1,2,2.67,2,4.44h2c0-2.4-1.06-4.54-2.73-6H20V4z M20,17v-1c0-1.1-0.9-2-2-2 s-2,0.9-2,2v1c-0.55,0-1,0.45-1,1v3c0,0.55,0.45,1,1,1h4c0.55,0,1-0.45,1-1v-3C21,17.45,20.55,17,20,17z M19,17h-2v-1 c0-0.55,0.45-1,1-1s1,0.45,1,1V17z",
                }
            }
        }
    }
}

pub fn sync_problem_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M3 12c0 2.21.91 4.2 2.36 5.64L3 20h6v-6l-2.24 2.24C5.68 15.15 5 13.66 5 12c0-2.61 1.67-4.83 4-5.65V4.26C5.55 5.15 3 8.27 3 12zm8 5h2v-2h-2v2zM21 4h-6v6l2.24-2.24C18.32 8.85 19 10.34 19 12c0 2.61-1.67 4.83-4 5.65v2.09c3.45-.89 6-4.01 6-7.74 0-2.21-.91-4.2-2.36-5.64L21 4zm-10 9h2V7h-2v6z",
            }
        }
    }
}

pub fn system_update_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M17 1.01L7 1c-1.1 0-2 .9-2 2v18c0 1.1.9 2 2 2h10c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99zM17 19H7V5h10v14zm-1-6h-3V8h-2v5H8l4 4 4-4z",
            }
        }
    }
}

pub fn tap_and_play_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M2 16v2c2.76 0 5 2.24 5 5h2c0-3.87-3.13-7-7-7zm0 4v3h3c0-1.66-1.34-3-3-3zm0-8v2c4.97 0 9 4.03 9 9h2c0-6.08-4.92-11-11-11zM17 1.01L7 1c-1.1 0-2 .9-2 2v7.37c.69.16 1.36.37 2 .64V5h10v13h-3.03c.52 1.25.84 2.59.95 4H17c1.1 0 2-.9 2-2V3c0-1.1-.9-1.99-2-1.99z",
            }
        }
    }
}

pub fn time_to_leave_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M18.92 5.01C18.72 4.42 18.16 4 17.5 4h-11c-.66 0-1.21.42-1.42 1.01L3 11v8c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-1h12v1c0 .55.45 1 1 1h1c.55 0 1-.45 1-1v-8l-2.08-5.99zM6.5 15c-.83 0-1.5-.67-1.5-1.5S5.67 12 6.5 12s1.5.67 1.5 1.5S7.33 15 6.5 15zm11 0c-.83 0-1.5-.67-1.5-1.5s.67-1.5 1.5-1.5 1.5.67 1.5 1.5-.67 1.5-1.5 1.5zM5 10l1.5-4.5h11L19 10H5z",
            }
        }
    }
}

pub fn tv_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M1 3.54l1.53 1.53C1.65 5.28 1 6.06 1 7v12c0 1.1.9 2 2 2h15.46l2 2 1.26-1.27L2.27 2.27 1 3.54zM3 19V7h1.46l12 12H3zM21 5h-7.58l3.29-3.3L16 1l-4 4-4-4-.7.7L10.58 5H7.52l2 2H21v11.48l1.65 1.65c.22-.32.35-.71.35-1.13V7c0-1.11-.89-2-2-2z",
            }
        }
    }
}

pub fn vibration_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M0 15h2V9H0v6zm3 2h2V7H3v10zm19-8v6h2V9h-2zm-3 8h2V7h-2v10zM16.5 3h-9C6.67 3 6 3.67 6 4.5v15c0 .83.67 1.5 1.5 1.5h9c.83 0 1.5-.67 1.5-1.5v-15c0-.83-.67-1.5-1.5-1.5zM16 19H8V5h8v14z",
            }
        }
    }
}

pub fn video_chat_20px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 20 20".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                    d: "M16.5,2h-13C2.67,2,2,2.67,2,3.5V18l3-3h11.5c0.83,0,1.5-0.67,1.5-1.5v-10C18,2.67,17.33,2,16.5,2z M14.5,11l-2-1.8V11 c0,0.55-0.45,1-1,1h-5c-0.55,0-1-0.45-1-1V6c0-0.55,0.45-1,1-1h5c0.55,0,1,0.45,1,1v1.8l2-1.8V11z",
                }
            }
        }
    }
}

pub fn video_chat_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
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
                    d: "M20,2H4C2.9,2,2.01,2.9,2.01,4L2,22l4-4h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M17,13l-2-1.99V13c0,0.55-0.45,1-1,1H8 c-0.55,0-1-0.45-1-1V7c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1v1.99L17,7V13z",
                }
            }
        }
    }
}

pub fn voice_chat_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
            x: props.x,
            y: props.y,
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
                        d: "M20,2H4C2.9,2,2,2.9,2,4v18l4-4h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,16H6l-2,2V4h16V16z",
                    }
                    rect {
                        height: "10",
                        y: "5",
                        width: "1.5",
                        x: "11.25",
                    }
                    rect {
                        x: "8.5",
                        width: "1.5",
                        y: "7",
                        height: "6",
                    }
                    rect {
                        y: "9",
                        width: "1.5",
                        height: "2",
                        x: "6",
                    }
                    rect {
                        x: "14",
                        y: "7",
                        width: "1.5",
                        height: "6",
                    }
                    rect {
                        x: "16.5",
                        width: "1.5",
                        height: "2",
                        y: "9",
                    }
                }
            }
        }
    }
}

pub fn vpn_lock_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M22 4v-.5C22 2.12 20.88 1 19.5 1S17 2.12 17 3.5V4c-.55 0-1 .45-1 1v4c0 .55.45 1 1 1h5c.55 0 1-.45 1-1V5c0-.55-.45-1-1-1zm-.8 0h-3.4v-.5c0-.94.76-1.7 1.7-1.7s1.7.76 1.7 1.7V4zm-2.28 8c.04.33.08.66.08 1 0 2.08-.8 3.97-2.1 5.39-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H7v-2h2c.55 0 1-.45 1-1V8h2c1.1 0 2-.9 2-2V3.46c-.95-.3-1.95-.46-3-.46C5.48 3 1 7.48 1 13s4.48 10 10 10 10-4.48 10-10c0-.34-.02-.67-.05-1h-2.03zM10 20.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L8 16v1c0 1.1.9 2 2 2v1.93z",
            }
        }
    }
}

pub fn wc_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M5.5 22v-7.5H4V9c0-1.1.9-2 2-2h3c1.1 0 2 .9 2 2v5.5H9.5V22h-4zM18 22v-6h3l-2.54-7.63C18.18 7.55 17.42 7 16.56 7h-.12c-.86 0-1.63.55-1.9 1.37L12 16h3v6h3zM7.5 6c1.11 0 2-.89 2-2s-.89-2-2-2-2 .89-2 2 .89 2 2 2zm9 0c1.11 0 2-.89 2-2s-.89-2-2-2-2 .89-2 2 .89 2 2 2z",
            }
        }
    }
}

pub fn wifi_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M1 9l2 2c4.97-4.97 13.03-4.97 18 0l2-2C16.93 2.93 7.08 2.93 1 9zm8 8l3 3 3-3c-1.65-1.66-4.34-1.66-6 0zm-4-4l2 2c2.76-2.76 7.24-2.76 10 0l2-2C15.14 9.14 8.87 9.14 5 13z",
            }
        }
    }
}

pub fn wifi_off_24px(props: IconProps) -> Element {
    rsx! {
        svg {
            id: props.id,
            class: props.class,
            style: props.style,
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg".to_string()),
            view_box: props.view_box.unwrap_or("0 0 24 24".to_string()),
            fill: props.fill,
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
                d: "M24 .01c0-.01 0-.01 0 0L0 0v24h24V.01zM0 0h24v24H0V0zm0 0h24v24H0V0z",
            }
            path {
                d: "M22.99 9C19.15 5.16 13.8 3.76 8.84 4.78l2.52 2.52c3.47-.17 6.99 1.05 9.63 3.7l2-2zm-4 4c-1.29-1.29-2.84-2.13-4.49-2.56l3.53 3.53.96-.97zM2 3.05L5.07 6.1C3.6 6.82 2.22 7.78 1 9l1.99 2c1.24-1.24 2.67-2.16 4.2-2.77l2.24 2.24C7.81 10.89 6.27 11.73 5 13v.01L6.99 15c1.36-1.36 3.14-2.04 4.92-2.06L18.98 20l1.27-1.26L3.29 1.79 2 3.05zM9 17l3 3 3-3c-1.65-1.66-4.34-1.66-6 0z",
            }
        }
    }
}

