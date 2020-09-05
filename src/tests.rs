use super::*;
use std::f64::consts::PI;
use std::{thread, time};

fn led_matrix() -> LedMatrix {
    let mut options = LedMatrixOptions::new();
    options.set_hardware_mapping("adafruit-hat-pwm");
    options.set_chain_length(2);
    options.set_hardware_pulsing(false);
    LedMatrix::new(Some(options)).unwrap()
}

#[test]
fn matrix_create() {
    let _matrix = led_matrix();
}

#[test]
fn canvas_size() {
    let matrix = led_matrix();
    let canvas = matrix.canvas();
    assert_eq!(canvas.size(), (64, 32));
}

#[test]
fn draw_line() {
    let matrix = led_matrix();
    let mut canvas = matrix.canvas();
    let (width, height) = canvas.size();
    let mut color = LedColor {
        red: 127,
        green: 0,
        blue: 0,
    };

    canvas.clear();
    for x in 0..width {
        color.blue = 255 - 3 * x as u8;
        canvas.draw_line(x, 0, width - 1 - x, height - 1, &color);
        thread::sleep(time::Duration::new(0, 10_000_000));
    }
}

#[test]
fn draw_circle() {
    let matrix = led_matrix();
    let mut canvas = matrix.canvas();
    let (width, height) = canvas.size();
    let mut color = LedColor {
        red: 127,
        green: 0,
        blue: 0,
    };
    let (x, y) = (width / 2, height / 2);

    canvas.clear();
    for r in 0..(width / 2) {
        color.green = color.red;
        color.red = color.blue;
        color.blue = (r * r) as u8;
        canvas.draw_circle(x, y, r as u32, &color);
        thread::sleep(time::Duration::new(0, 10_000_000));
    }
}

#[test]
fn draw_text() {
    let matrix = led_matrix();
    let mut canvas = matrix.canvas();
    let font = LedFont::new(Path::new("/usr/share/fonts/misc/10x20.bdf")).unwrap();
    let color = LedColor {
        red: 0,
        green: 127,
        blue: 0,
    };
    let (width, height) = canvas.size();
    let text_width = 10 * 9;
    let baseline = height / 2;

    canvas = matrix.offscreen_canvas();
    for x in 0..(2 * width) {
        let x = x % (10 * 9);
        canvas.clear();
        canvas.draw_text(&font, "Mah boy! ", x, baseline, &color, 0, false);
        canvas.draw_text(
            &font,
            "Mah boy! ",
            x - text_width,
            baseline,
            &color,
            0,
            false,
        );
        canvas.draw_text(
            &font,
            "Mah boy! ",
            x + text_width,
            baseline,
            &color,
            0,
            false,
        );
        canvas = matrix.swap(canvas);
        thread::sleep(time::Duration::new(0, 5_0000_000));
    }
}

#[test]
fn gradient() {
    let matrix = led_matrix();
    let mut canvas = matrix.canvas();
    let mut color = LedColor {
        red: 0,
        green: 0,
        blue: 0,
    };
    let period = 400;
    let duration = time::Duration::new(0, 100_000_000);
    let sleep_duration = duration / period;

    for t in 0..period {
        let t = t as f64;
        color.red = ((PI * t / period as f64).sin() * 255.) as u8;
        color.green = ((2. * PI * t / period as f64).cos() * 255.) as u8;
        color.blue = ((3. * PI * t / period as f64 + 0.3).cos() * 255.) as u8;
        canvas.fill(&color);
        thread::sleep(sleep_duration);
    }
}

#[test]
fn canvas_swap() {
    let matrix = led_matrix();
    let mut canvas = matrix.offscreen_canvas();
    let mut color = LedColor {
        red: 10,
        green: 0,
        blue: 0,
    };

    canvas.fill(&color);
    canvas = matrix.swap(canvas);
    thread::sleep(time::Duration::new(0, 100_000_000));
    color.red = 0;
    color.green = 10;
    canvas.fill(&color);
    canvas = matrix.swap(canvas);
    thread::sleep(time::Duration::new(0, 100_000_000));
    color.green = 0;
    color.blue = 10;
    canvas.fill(&color);
    canvas = matrix.swap(canvas);
    thread::sleep(time::Duration::new(0, 100_000_000));
}
