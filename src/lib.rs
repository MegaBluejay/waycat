use std::time::Duration;

use itertools::Itertools;
use serde::de::IgnoredAny;
use waybar_cffi::{
    Module,
    gtk::{
        Image,
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream},
        glib::{Bytes, ControlFlow, timeout_add_local},
        prelude::*,
    },
    waybar_module,
};

struct Waycat;
waybar_module!(Waycat);

impl Module for Waycat {
    type Config = IgnoredAny;

    fn init(info: &waybar_cffi::InitInfo, _config: Self::Config) -> Self {
        let container = info.get_root_widget();

        let cats: [Pixbuf; N] = CATS
            .into_iter()
            .map(|data| {
                Pixbuf::from_stream(
                    &MemoryInputStream::from_bytes(&Bytes::from(data)),
                    Cancellable::NONE,
                )
                .unwrap()
            })
            .collect_array()
            .unwrap();

        let image = Image::new();
        container.add(&image);

        const UPD_INTERVAL: Duration = Duration::from_millis(200);

        timeout_add_local(UPD_INTERVAL, move || {
            let clock = image.frame_clock().unwrap();
            let time = clock.frame_time() as usize;
            image.set_from_pixbuf(Some(&cats[(time / UPD_INTERVAL.as_micros() as usize) % N]));
            ControlFlow::Continue
        });

        Self
    }
}

const N: usize = 5;
const CATS: [&[u8]; N] = [
    include_bytes!("res/cat/dark_cat_0.png"),
    include_bytes!("res/cat/dark_cat_1.png"),
    include_bytes!("res/cat/dark_cat_2.png"),
    include_bytes!("res/cat/dark_cat_3.png"),
    include_bytes!("res/cat/dark_cat_4.png"),
];
