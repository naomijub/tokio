const PRIMO: u64 = 15485867;

fn main() {
    if is_prime(PRIMO) {
        println!("{} é primo", PRIMO)
    }
}

fn is_prime(numero: u64) -> bool {
    for i in 2..numero {
        if numero % i == 0 { return false }
    }
    true
}