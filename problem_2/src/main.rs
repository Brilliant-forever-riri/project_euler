fn main() {
    let mut sum = 0;
    let mut a = 1;
    let mut b = 2;

    while b <= 4000000 {
        if b % 2 == 0 {
            sum += b;
        }

        let next = a + b;
        a = b;
        b = next;
    }

    println!("The sum of even-valued terms is: {}", sum);
}
