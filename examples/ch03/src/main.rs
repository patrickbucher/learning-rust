fn main() {
    let salary = 6543; // decimal
    let color = 0xfe3abc; // hexadecimal
    let perms = 0o755; // octal
    let mask = 0b0100_0011; //binary
    let space = b' ';
    let ellipsis = '…';
    println!("{salary} {color} {perms} {mask} {space} {ellipsis}");

    let fibs: [u8; 6] = [1, 1, 2, 3, 5, 8];
    for f in fibs {
        println!("{f}");
    }
    let ones = [1; 5];
    for i in ones {
        println!("{i}");
    }

    let next_fib = if fibs.len() > 0 {
        fibs[fibs.len() - 2] + fibs[fibs.len() - 1]
    } else {
        1
    };
    println!("{next_fib}");

    let mut x = 0;
    'outer: loop {
        loop {
            x += 1;
            if x > 10 {
                break 'outer;
            }
            x += 1;
        }
        x -= 1;
    }
    println!("{x}");

    println!("100°F is {}°C", to_celsius(100.0));
    println!("38°C is {}°F", to_fahrenheit(38.0));

    println!("the 30th Fibonacci number is {}", fib(30));
}

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}

fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
