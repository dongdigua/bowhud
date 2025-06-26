mod arrow;
mod vec3;

use arrow::Arrow;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{DrawingArea};
use gtk::gdk::{self, prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell, KeyboardMode};

use std::collections::HashMap;

const FOV: f64 = 70.0;
const PIXEL: i32 = 1080;

#[derive(Debug, Copy, Clone)]
struct CrosshairData {
    h: f64,
    w: f64
}

fn main() {
    let mut arrow = Arrow::new(1.0,0.0,0.0, 3.0);
    let mut res = HashMap::new();
    for _ in 0..31 { // 1.5s
        arrow.tick();
        let (x, y) = (arrow.pos.x, arrow.pos.y);
        let h = block2screen(FOV, PIXEL as f64, x, y);
        let w = block2screen(FOV, PIXEL as f64, x, 1.0);

        let dec = ((x / 10.0).round() as i32) * 10;
        res.entry(dec)
            .and_modify(|best: &mut (f64, CrosshairData)| {
                if (x - dec as f64).abs() < (best.0 - dec as f64).abs() {
                    *best = (x, CrosshairData { h, w });
                }
            })
            .or_insert((x, CrosshairData { h, w }));
    }
    res.remove(&0);
    res.remove(&10); // 十米之内，又准又快
    println!("{:#?}", res);

    let app = gtk::Application::new(
        Some("com.github.dongdigua.wayhud"),
        Default::default(),
    );

    app.connect_activate(move |app| build_ui(app, res.clone()));
    app.run();
}

fn build_ui(application: &gtk::Application, data: HashMap<i32, (f64, CrosshairData)>) {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(".background{background-color: transparent;}");
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = gtk::ApplicationWindow::new(application);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    let anchors = [
        (Edge::Top, true),
        (Edge::Bottom, true),
    ];
    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    // https://github.com/ErikReider/SwayOSD/blob/ce1f34d80a7f8b4393a5551ea0535bd8beabb28c/src/server/osd_window.rs#L60
    window.connect_map(|window| {
		if let Some(surface) = window.surface() {
			let region = gtk::cairo::Region::create();
			surface.set_input_region(&region);
		}
	});

    let overlay = DrawingArea::builder()
        .content_width(PIXEL)
        .content_height(PIXEL)
        .build();

    overlay.set_draw_func(move |_area, ctx, width, height| {
        ctx.set_source_rgba(0.0, 0.0, 1.0, 0.5);
        ctx.set_line_width(1.0);

        let (cx, cy) = (width as f64 / 2.0, height as f64 / 2.0);

        for (_, (_, v)) in data.iter() {
            ctx.move_to(cx - v.w/2.0, cy - v.h);
            ctx.line_to(cx + v.w/2.0, cy - v.h);
            ctx.move_to(cx, cy);
            ctx.line_to(cx, cy - v.h);
        }

        ctx.stroke().unwrap();
    });

    window.set_child(Some(&overlay));
    window.present();
}

fn block2screen(fov: f64, screen: f64, depth: f64, block: f64) -> f64 {
    let rad = fov / 180.0 * std::f64::consts::PI;
    let screenh = (rad/2.0).tan() * depth * 2.0;
    (block / screenh * screen).round()
}

