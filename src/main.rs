use std::io::{stdout, Write};
use libc::{c_int, signal, SIGUSR1, SIGUSR2};
use std::time::{Duration, Instant};
use std::thread::sleep;
static mut SIGNAL_RECEIVED:bool = false;
static mut START:bool = false;
extern "C" fn handle_signal(_:c_int)
{
    unsafe {
        SIGNAL_RECEIVED = true;
    }
}
extern "C" fn handle_start(_:c_int)
{
    unsafe {
        START = true;
    }
}
fn main()
{
    unsafe {
        signal(SIGUSR1, handle_signal as usize);
    }
    unsafe {
        signal(SIGUSR2, handle_start as usize);
    }
    loop
    {
        sleep(Duration::from_millis(1));
        unsafe {
            if START
            {
                break;
            }
        }
    }
    let watch = Instant::now();
    let mut n;
    loop
    {
        n = watch.elapsed().as_nanos();
        unsafe {
            print!("\x1B[2K\x1B[1G{:02}:{:02}:{:02}.{:09}{}",
                   n / (3_600_000_000_000),
                   (n / (60_000_000_000)) % 60,
                   (n / (1_000_000_000)) % 60,
                   n % 1_000_000_000,
                   if SIGNAL_RECEIVED
                   {
                       SIGNAL_RECEIVED = false;
                       "\n"
                   }
                   else
                   {
                       ""
                   });
        }
        stdout().flush().unwrap();
        sleep(Duration::from_millis(1));
    }
}