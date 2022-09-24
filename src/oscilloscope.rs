use crate::Samples;
use macroquad::prelude::*;

fn draw_my_cool_thingy(samples: &Samples) {
    let samples_len = samples.read().unwrap().len();
    let x_step = screen_width() / samples_len as f32;
    for i in 0..samples_len - 1 {
        screen_width();
        screen_height();

        let data = samples.read().unwrap();

        draw_line(
            x_step * i as f32,
            data[i] * screen_height() / 2.0 + screen_height() / 2.0,
            x_step * (i as f32 + 1.0),
            data[i + 1] * screen_height() / 2.0 + screen_height() / 2.0,
            2.0,
            WHITE,
        );
    }
}

pub async fn ui_loop(samples: Samples) {
    loop {
        clear_background(BLACK);

        draw_my_cool_thingy(&samples);

        next_frame().await
    }
}
