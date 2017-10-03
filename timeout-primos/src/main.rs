extern crate futures;
extern crate futures_cpupool;
extern crate tokio_timer;

use std::time::Duration;

use futures::Future;
use futures_cpupool::CpuPool;
use tokio_timer::Timer;

const PRIMO: u64 = 15485867;

fn main() {

    let timeout = Timer::default()
        .sleep(Duration::from_millis(500))
        .then(|_| Err(()));

    let primo = CpuPool::new_num_cpus()
        .spawn_fn(|| {
            Ok(eh_primo(PRIMO)) //Tente 157
        });

    match timeout
        .select(primo)
        .map(|(ok, _)| ok)
        .wait() {
            Ok(true) => println!("Primo"),
            Ok(false) => println!("Composto"),
            Err(_) => println!("Timeout"),
        }
}

fn eh_primo(numero: u64) -> bool {
    for i in 2..numero {
        if numero % i == 0 { return false }
    }
    true
}