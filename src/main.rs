use std::io::{stdout, Write};
use libc::{c_int, signal, SIGUSR1};
use std::time::{Duration, Instant};
use std::thread::sleep;
static mut SIGNAL_RECEIVED:bool = false;
extern "C" fn handle_signal(_:c_int)
{
    unsafe {
        SIGNAL_RECEIVED = true;
    }
}
fn main()
{
    unsafe {
        signal(SIGUSR1, handle_signal as usize);
    }
    loop
    {
        sleep(Duration::from_millis(1));
        unsafe {
            if SIGNAL_RECEIVED
            {
                SIGNAL_RECEIVED = false;
                break;
            }
        }
    }
    let mut n;
    let watch = Instant::now();
    loop
    {
        n = watch.elapsed().as_millis();
        unsafe {
            if SIGNAL_RECEIVED
            {
                SIGNAL_RECEIVED = false;
                println!("\x1B[1G{:02}:{:02}:{:02}.{:03}", n / (3_600_000), (n / (60_000)) % 60, (n / (1_000)) % 60, n % 1_000);
            }
            else
            {
                print!("\x1B[1G{:02}:{:02}:{:02}.{:03}", n / (3_600_000), (n / (60_000)) % 60, (n / (1_000)) % 60, n % 1_000);
            }
        }
        stdout().flush().unwrap();
        sleep(Duration::from_millis(1));
    }
}