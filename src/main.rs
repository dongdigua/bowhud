mod arrow;
mod vec3;
use arrow::Arrow;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{DrawingArea, gdk};
use gtk::glib::{self, ControlFlow};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use crossbeam_channel::{bounded, Receiver};

use std::io::Write;
use std::collections::HashMap;

// net/minecraft/client/network/AbstractClientPlayerEntity.java
const FOV_EFFECT_SCALE: f64 = 1.0 - 0.15;

#[derive(Debug, Copy, Clone)]
struct CrosshairData {
    h: f64,
    w: f64
}

fn main() {
    let mut pixel = 1080;
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        pixel = args[1].parse().unwrap();
    }

    let (tx0, rx0) = bounded(1);
    let (tx1, rx1) = bounded(1); // 天才

    std::thread::spawn(move || {
        let mut fov: f64 = 70.0;
        let mut height: f64 = 0.0;
        let mut speed: f64 = 3.0;
        loop {
            let mut arrow = Arrow::new(1.0,0.0,0.0, speed);
            let mut res = HashMap::new();
            for _ in 0..31 { // outside 1s will have no accuracy at all
                arrow.tick();
                let (x, y) = (arrow.pos.x, arrow.pos.y);
                let h = block2screen(fov, pixel as f64, x, y);
                let w = block2screen(fov, pixel as f64, (x*x + height*height).sqrt(), 1.0);

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
            res.remove(&90);

            tx0.send(res.clone()).unwrap();

            let mut buffer = String::new();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut buffer).unwrap();

            if buffer.starts_with("fov") {
                if let Ok(ffov) = buffer.strip_prefix("fov").unwrap().trim().parse::<f64>() {
                    fov = ffov;
                }
            } else if buffer.starts_with("@") {
                if let Some(weapon) = buffer.strip_prefix("@") {
                    match weapon.trim() {
                        "bow" => speed = 3.0,
                        "crossbow" => speed = 3.15,
                        _ => println!("?"),
                    }
                }
            } else if let Ok(hheight) = buffer.trim().parse::<f64>() {
                height = hheight;
            } else if buffer == "eff" {
                fov *= FOV_EFFECT_SCALE;
            }
            tx1.send(true).unwrap(); // new trajectory, update display
        }});

    let app = gtk::Application::new(
        Some("com.github.dongdigua.wayhud"),
        gtk::gio::ApplicationFlags::HANDLES_COMMAND_LINE
    );

    app.connect_command_line(|app, _cmdline| {
        // ignore this
        app.activate();
        0
    });
    app.connect_activate(move |app| build_ui(app, pixel, rx0.clone(), rx1.clone()));
    app.run();
}

fn build_ui(application: &gtk::Application, pixel: i32, rx0: Receiver<HashMap<i32, (f64, CrosshairData)>>, rx1: Receiver<bool>) {
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
        .content_width(pixel)
        .content_height(pixel)
        .build();

    overlay.set_draw_func(move |_area, ctx, width, height| {
        if let Ok(data) = rx0.try_recv() {
            ctx.set_source_rgba(1.0, 1.0, 1.0, 0.7);
            ctx.set_line_width(1.0);

            let (cx, cy) = (width as f64 / 2.0, height as f64 / 2.0);

            for (_, (_, v)) in data.iter() {
                ctx.move_to(cx - v.w/2.0, cy - v.h);
                ctx.line_to(cx + v.w/2.0, cy - v.h);
                // ctx.move_to(cx, cy);
                // ctx.line_to(cx, cy - v.h);
            }

            ctx.stroke().unwrap();
        }
    });

    window.set_child(Some(&overlay));
    window.present();

    glib::source::timeout_add_local(std::time::Duration::from_millis(500), move || {
        if let Ok(_) = rx1.try_recv() {
            overlay.clone().queue_draw();
        }
        ControlFlow::Continue
    });
}

fn block2screen(fov: f64, screen: f64, depth: f64, block: f64) -> f64 {
    let rad = fov / 180.0 * std::f64::consts::PI;
    let screenh = (rad/2.0).tan() * depth * 2.0;
    (block / screenh * screen).round()
}

