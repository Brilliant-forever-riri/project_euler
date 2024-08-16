fn main() {
    let mut n: u64 = 600851475143;
    let mut largest_prime = 0;
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            largest_prime = divisor;
            n /= divisor;
        }
        divisor += 1;
    }

    println!("The largest prime factor is: {}", largest_prime);
}

