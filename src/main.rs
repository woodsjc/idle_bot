use chrono;
use rdev::{display_size, simulate, Button, EventType, SimulateError};
use std::{thread, time};
use xdotool::{command::options::SearchOption, mouse, option_vec, OptionVec};

struct WizardWindow {
    window_x: i32,
    window_y: i32,
    size_x: i32,
    size_y: i32,
}

fn main() {
    let (w, h) = display_size().unwrap();
    println!("w:{w}, h:{h}");
    let wizard_window = search();

    for _ in 0..=100 {
        wizard_window.click_loop();
        thread::sleep(time::Duration::from_secs(69));
    }
    println!("Finished click loop");
}

impl WizardWindow {
    fn click_loop(&self) {
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
            "{}: mouse location: (x:{orig_x}, y:{orig_y}), focused window pid:{}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            window.trim()
        );

        // 1925, 29
        // 1024 768
        send(&EventType::MouseMove {
            //x: 2850.0,
            x: self.window_x as f64 + 925.0,
            //y: 720.0,
            y: self.window_y as f64 + 691.0,
        });
        for _ in 0..15 {
            left_click();
        }

        return_to_origin(orig_x, orig_y);
        xdotool::window::focus_window(&window, xdotool::OptionVec::new());
        println!(
            "{}: Success!",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
        );
    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
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

fn search() -> WizardWindow {
    let pid = xdotool::window::search("Wizard", option_vec![SearchOption::Name,]);
    let pid = String::from_utf8(pid.stdout).unwrap();
    let pid = pid.trim();
    let geometry = xdotool::window::get_window_geometry(pid, option_vec![]);
    let geometry = String::from_utf8(geometry.stdout).unwrap();
    let mut nums = vec![];
    let mut num = String::new();

    for c in geometry.chars() {
        if c.is_numeric() {
            num.push(c);
        } else if !num.is_empty() {
            nums.push(num.parse::<i32>().unwrap());
            num.clear();
        }
    }

    println!("Wizard's Wheel pid: {pid}\ngeometry: {:?}", geometry);
    println!(
        "Position ({}, {}), Size {}x{}",
        nums[1], nums[2], nums[4], nums[5]
    );

    WizardWindow {
        window_x: nums[1],
        window_y: nums[2],
        size_x: nums[4],
        size_y: nums[5],
    }
}
