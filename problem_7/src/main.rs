fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn nth_prime(n: usize) -> u64 {
    let mut count = 0;
    let mut candidate = 1;
    
    while count < n {
        candidate += 1;
        if is_prime(candidate) {
            count += 1;
        }
    }
    
    candidate
}

fn main() {
    let n = 10001;
    let prime_number = nth_prime(n);
    println!("The {}th prime number is {}", n, prime_number);
}

