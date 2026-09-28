fn main() {
    let mut count = 0;

    for x in 0..100 {
        if !(x >= 53 || x < 29) {
            println!("{:}", x);
            count += 1;
        }
    }

    println!("count: {:}", count)
}