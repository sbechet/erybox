use std::time;
use std::time::{Duration, Instant};

use eframe::egui;

mod ekn;
use ekn::ui::EknUi;

mod sidbox;
use sidbox::*;

struct MyApp {
    bpm: u64,
    audio: Audio,
    ekn_ui: EknUi,
    now: Instant,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            bpm: 60,
            audio: Audio::new(),
            ekn_ui: EknUi::default(), //new(3, 8, 40, true),
            now: time::Instant::now(),
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let d = Duration::from_millis(1000*60/(self.bpm*self.ekn_ui.ekn.get_steps() as u64));
        if self.now.elapsed() > d/2 {
            if self.audio.is_playing {
                self.audio.pause();
            }
        }
        if self.now.elapsed() > d {
            self.now = time::Instant::now();
            self.ekn_ui.ekn.next_step();
            if self.ekn_ui.ekn.is_event() {
                // println!("play");
                self.audio.play();
                ctx.request_repaint_after(d/2);
            } else {
                // println!("pause");
                self.audio.pause();
                ctx.request_repaint_after(d);
            }
         } else {
            let dur_delta = d - self.now.elapsed();
            // println!("inter-refresh, next wait:{:?}", dur_delta);
            ctx.request_repaint_after(dur_delta);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Euclidian Rythms Generator");
            ui.add(egui::Slider::new(&mut self.bpm, 1..=120).text("BPM"));
            self.ekn_ui.ekn_ui(ui);
        });

    }
}

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();
    let mut myapp = MyApp::default();
    myapp.ekn_ui.debug = true;

    eframe::run_native(
        "Euclidian Rythms Application",
        options,
        Box::new(|_cc| Box::new(myapp)),
    );

}
