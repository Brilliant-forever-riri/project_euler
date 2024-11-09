fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    for num in 2..((limit as f64).sqrt() as usize + 1) {
        if is_prime[num] {
            for multiple in (num * num..limit).step_by(num) {
                is_prime[multiple] = false;
            }
        }
    }
    is_prime
}

fn sum_of_primes_below(limit: usize) -> u64 {
    let is_prime = sieve_of_eratosthenes(limit);
    let mut sum = 0;

    for (num, prime) in is_prime.iter().enumerate() {
        if *prime {
            sum += num as u64;
        }
    }
    sum
}

fn main() {
    let limit = 2000000;
    let sum = sum_of_primes_below(limit);
    println!("The sum of all primes below {} is {}", limit, sum);
}
