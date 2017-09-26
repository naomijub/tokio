extern crate futures;
extern crate futures_cpupool;

use futures::Future;
use futures_cpupool::CpuPool;

const PRIMO: u64 = 15485867;

fn main() {
    let pool = CpuPool::new_num_cpus();

    let future_primo = pool.spawn_fn(|| {
        let primo = eh_primo(PRIMO);

        let res: Result<bool, ()> = Ok(primo);
        res
    });
}

fn eh_primo(numero: u64) -> bool {
    for i in 2..numero {
        if numero % i == 0 { return false }
    }
    true
}