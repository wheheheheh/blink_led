use std::fs::{File, OpenOptions};
use std::io::Write;
use std::thread;
use std::time::Duration;

const GPIO: u8 = 110;

fn main() {
    let mut export_file = File::create("/sys/class/gpio/export")
        .expect("error open export");
    write!(export_file, "{}", GPIO).expect("error export GPIO"); 

    let direction_path = format!("/sys/class/gpio/gpio{}/direction", GPIO);
    let mut direction_file = OpenOptions::new()
        .write(true)
        .open(&direction_path)
        .expect("error open direction");
    write!(direction_file, "out").expect("error create direction");

    let value_path = format!("/sys/class/gpio/gpio{}/value", GPIO);
    let mut value_file = OpenOptions::new()
        .write(true)
        .open(&value_path)
        .expect("error open value");

    for _ in 0..10 {
        write!(value_file, "1").expect("error on GPIO");
        println!("GPIO on");
        thread::sleep(Duration::from_millis(500));

        write!(value_file, "0").expect("error off GPIO");
        println!("GPIO off");
        thread::sleep(Duration::from_millis(500));
    }

    let mut unexport_file = File::create("/sys/class/gpio/unexport")
        .expect("error open unexport");

    write!(unexport_file, "{}", GPIO).expect("error off GPIO");
}
