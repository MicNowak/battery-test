use battery::Manager;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant, SystemTime};
use std::{fs::OpenOptions, thread};
use sysinfo::System;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut system = System::new();
    system.refresh_cpu();

    let battery_manager = battery::Manager::new()?;

    let system_name = match System::name() {
        Some(name) => name,
        None => "unknown".to_string(),
    };
    let file_name = format!(
        "{}-{}.csv",
        system_name.replace('/', " ").replace('\\', " "),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
    );
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;

    if let Err(e) = writeln!(
        file,
        "time,idx,vendor,model,state,percentage,time_to_full,time_to_empty"
    ) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let time = Instant::now();

    let num_threads = system.cpus().len();
    for _ in 1..num_threads {
        thread::spawn(worker);
    }

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
    loop {
        interval.tick().await;
        save_battery_data(&battery_manager, &mut file, time.elapsed())?;
    }
}

fn worker() {
    let mut _x = 0;
    loop {
        _x -= 1;
        _x += 1;
    }
}

fn save_battery_data(
    battery_manager: &Manager,
    file: &mut File,
    time: Duration,
) -> Result<(), battery::Error> {
    let time = sec_to_string(time.as_secs_f32());
    for (idx, maybe_battery) in battery_manager.batteries()?.enumerate() {
        let mut csv = vec![];
        csv.push(time.clone());
        let battery = maybe_battery?;
        csv.push(idx.to_string());
        match battery.vendor() {
            Some(vendor) => csv.push(vendor.to_string()),
            None => csv.push("".to_string()),
        }
        match battery.model() {
            Some(model) => csv.push(model.to_string()),
            None => csv.push("".to_string()),
        }
        csv.push(battery.state().to_string());
        let percentage = (battery.energy() / battery.energy_full()).value;
        csv.push(percentage.to_string());
        match battery.time_to_full() {
            Some(time_to_full) => csv.push(sec_to_string(time_to_full.value)),
            None => csv.push("".to_string()),
        }
        match battery.time_to_empty() {
            Some(time_to_empty) => csv.push(sec_to_string(time_to_empty.value)),
            None => csv.push("".to_string()),
        }
        let csv = csv.join(",");

        if let Err(e) = writeln!(file, "{}", csv) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    Ok(())
}

fn sec_to_string(seconds: f32) -> String {
    let mut minutes = seconds / 60.0;
    let hours = minutes / 60.0;
    let seconds = seconds % 60.0;
    minutes %= 60.0;
    format!("{}:{}:{}", hours.floor(), minutes.floor(), seconds.floor())
}
