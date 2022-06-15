use crate::{Samples, SharedVoiceManager};
use eframe::egui;
use egui::plot::{Line, Plot, Value, Values};

pub fn run_synth_ui(voice_manager: SharedVoiceManager, samples: Samples) {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(move |_cc| {
            Box::new(SynthUI {
                voice_manager: voice_manager.clone(),
                samples: samples.clone(),
            })
        }),
    );
}

pub struct SynthUI {
    voice_manager: SharedVoiceManager,
    samples: Samples,
}

impl SynthUI {
    // pub fn new(voice: SharedVoice, samples: Samples) -> Self {
    //     Self {
    //         voice_man: voice,
    //         samples: samples,
    //     }
    // }
}

impl eframe::App for SynthUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Make some sound").clicked() {
                self.voice_manager.write().unwrap().note_on(440.0);
            }
            let line = Line::new(Values::from_ys_f32(&self.samples.read().unwrap())); //.width(2.0);
            ctx.request_repaint();
            Plot::new("my_plot")
                .view_aspect(2.0)
                .include_x(1024.0)
                .show_axes([false, false])
                .show_background(false)
                .allow_zoom(false)
                .allow_boxed_zoom(false)
                .allow_scroll(false)
                .allow_drag(false)
                .include_y(-1.0)
                .include_y(1.0)
                .show_x(false)
                .show_y(false)
                .show(ui, |plot_ui| plot_ui.line(line));
        });
    }
}