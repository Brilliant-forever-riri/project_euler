fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn main() {
    let mut result = 1;

    for i in 2..=20 {
        result = lcm(result, i);
    }

    println!("The smallest number that is evenly divisible by all numbers from 1 to 20 is: {}", result);
}

