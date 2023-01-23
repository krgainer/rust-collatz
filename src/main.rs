fn collatz(mut n: u128) -> (u128, u128) {
    let mut y: u128 = 0;
    let mut n_max: u128 = n;
    loop {
        if n > n_max {n_max = n;}
        if n <= 1 {break;}
        if n % 2 == 0 {
                n = n / 2;
                y += 1;
            } else {
                n = 3 * n + 1;
                y += 1;
            };
    };
    (y, n_max)
}

fn main() {
    // max_jumps is the highest number of jumps it took to reach 4,2,1. Init val, Jumps
    let mut max_jumps: (u128, u128) = (0,0);
    // highest_number is the highest number reached during collatz. Init val, highest number reached
    let mut highest_number: (u128, u128) = (0,0);
    let mut x: u128 = 1;
    while x>0 {
        let (z, i) = collatz(x);
        println!("{} took {} steps to reach 4,2,1. It reached the highest number of {}", x, z, i);
        if z > max_jumps.1 {max_jumps = (x, z);}
        if i > highest_number.1 {highest_number = (x, i);}
        x += 1;
        if x%100==0 {println!("Highest Number {:?} \nMax Jumps {:?}", highest_number, max_jumps);}
};
}
