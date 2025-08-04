use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use itertools::Itertools;
use serde::de::IgnoredAny;
use waybar_cffi::{
    InitInfo, Module,
    gtk::{
        EventBox, Image,
        gdk_pixbuf::Pixbuf,
        gio::{Cancellable, MemoryInputStream},
        glib::{Bytes, ControlFlow, Propagation, timeout_add_local},
        prelude::*,
    },
    waybar_module,
};

const UPD_INTERVAL: Duration = Duration::from_millis(200);
const N: usize = 5;
const CATS: [&[u8]; N] = [
    include_bytes!("res/cat/dark_cat_0.png"),
    include_bytes!("res/cat/dark_cat_1.png"),
    include_bytes!("res/cat/dark_cat_2.png"),
    include_bytes!("res/cat/dark_cat_3.png"),
    include_bytes!("res/cat/dark_cat_4.png"),
];

struct Waycat;
waybar_module!(Waycat);

impl Module for Waycat {
    type Config = IgnoredAny;

    fn init(info: &InitInfo, _config: Self::Config) -> Self {
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

        let event_box = EventBox::new();
        let image = Image::new();
        event_box.add(&image);
        let active = Arc::new(AtomicBool::new(false));
        toggle(&cats, &active, &image);
        event_box.connect_button_press_event(move |_event_box, _event| {
            toggle(&cats, &active, &image);
            Propagation::Proceed
        });
        container.add(&event_box);

        Self
    }
}

fn update(cats: &[Pixbuf], active: &AtomicBool, image: &Image) -> ControlFlow {
    let now_active = active.load(Ordering::Relaxed);
    let n = if now_active {
        let clock = image.frame_clock().unwrap();
        let time = clock.frame_time() as usize;
        (time / UPD_INTERVAL.as_micros() as usize) % N
    } else {
        0
    };
    image.set_from_pixbuf(Some(&cats[n]));
    if now_active {
        ControlFlow::Continue
    } else {
        ControlFlow::Break
    }
}

fn toggle(cats: &[Pixbuf; N], active: &Arc<AtomicBool>, image: &Image) {
    let now_active = !active.fetch_not(Ordering::Relaxed);
    if !now_active {
        return;
    }
    let active = active.clone();
    let cats = cats.clone();
    let image = image.clone();
    timeout_add_local(UPD_INTERVAL, move || update(&cats, &active, &image));
}
