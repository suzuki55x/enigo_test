use enigo::*;
use std::{thread, time};

fn main() {
    let mut enigo = Enigo::new();

    let start = time::Instant::now();

    loop {
        let elapsed = start.elapsed().as_secs_f64();

        // 30秒経過したら終了
        if elapsed > 30.0 {
            break;
        }
        enigo.mouse_move_relative(1, 1);

        // 1秒待機
        thread::sleep(time::Duration::from_millis(10));
    }
}
