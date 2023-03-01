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
    //"x:595 y:301 screen:0 window:23068675\n"

    let vals = String::from_utf8(loc_text.stdout)
        .unwrap()
        .split(" ")
        .map(|s| s.split(":").last().unwrap().parse::<i32>())
        .collect::<Vec<_>>();
    let (orig_x, orig_y) = (vals[0].clone().unwrap(), vals[1].clone().unwrap());

    println!(
        "{}: mouse location: (x:{orig_x}, y:{orig_y})",
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
    //return to origin
    send(&EventType::MouseMove {
        x: x as f64,
        y: y as f64,
    });
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));
}

fn left_click() {
    send(&EventType::ButtonPress(Button::Left));
    send(&EventType::ButtonRelease(Button::Left));
}
