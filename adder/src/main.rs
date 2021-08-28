use add_one;
// use rand;

// adder_workspace % cargo run -p adder
fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );

    println!(
        "Hello, world! {} plus two is {}!",
        num,
        add_two::add_two(num)
    );
}
