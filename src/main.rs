use chrono;
use rdev::{display_size, simulate, Button, EventType, SimulateError};
use std::{thread, time};
use xdotool::mouse;

fn main() {
    let (w, h) = display_size().unwrap();
    println!("w:{w}, h:{h}");

    for _ in 0..=100 {
        click_loop();
        thread::sleep(time::Duration::from_secs(69));
    }
    println!("Finished click loop");
}

fn click_loop() {
    let loc_text = mouse::get_mouse_location();
    let window: &str = "";
    let window = xdotool::window::get_window_focus(window);
    let window = String::from_utf8(window.stdout).unwrap();

    let vals = String::from_utf8(loc_text.stdout)
        .unwrap()
        .split(" ")
        .map(|s| s.split(":").last().unwrap().parse::<i32>())
        .collect::<Vec<_>>();
    let (orig_x, orig_y) = (vals[0].clone().unwrap(), vals[1].clone().unwrap());

    println!(
        "{}: mouse location: (x:{orig_x}, y:{orig_y}), focused window pid:{window}",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
    );

    send(&EventType::MouseMove {
        x: 2850.0,
        y: 720.0,
    });
    for _ in 0..15 {
        left_click();
    }

    return_to_origin(orig_x, orig_y);
    xdotool::window::focus_window(&window, xdotool::OptionVec::new());
    println!("Success!");
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(25);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    thread::sleep(delay);
}

fn return_to_origin(x: i32, y: i32) {
    send(&EventType::MouseMove {
        x: x as f64,
        y: y as f64,
    });
}

fn left_click() {
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));
}
