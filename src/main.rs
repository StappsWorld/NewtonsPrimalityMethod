fn main() {

    for i in 0..100000{
        println!("{}", primeNumbers(i))
    }

}

fn primeNumbers(i:i128) -> i128{
    let mut sum = 0;
    let mut x = 0;
    while x < i {
        if isPrime(x) {
            sum += 1;
        }
        x += 1;
    }

    return sum;
}

fn isPrime(n:i128) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0{
        return false;
    }

    let mut i = 5;

    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    return true;
}