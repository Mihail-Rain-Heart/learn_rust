static N: i32 = 3; // Число повторений

fn main() {
    let mut i = 0;
    loop {
        println!("{}", i);
        if i == N {
            break;
        }
        i += 1;
    }

    i = 0;

    while i <= N {
        println!("{}", i);
        i += 1
    }

    for i in 0..=N {
        println!("{}", i)
    }
}
