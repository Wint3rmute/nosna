use crate::Samples;
use macroquad::prelude::*;

fn draw_my_cool_thingy(data: &Vec<f32>) {
    let x_step = screen_width() / data.len() as f32;
    for i in 0..data.len() - 1 {
        screen_width();
        screen_height();

        draw_line(
            x_step * i as f32,
            data[i] * screen_height() / 4.0 + screen_height() / 2.0,
            x_step * (i as f32 + 1.0),
            data[i + 1] * screen_height() / 4.0 + screen_height() / 2.0,
            2.0,
            WHITE,
        );
    }
}

pub async fn ui_loop(samples: Samples) {
    loop {
        clear_background(BLACK);

        draw_my_cool_thingy(&samples.read().unwrap());

        next_frame().await
    }
}
