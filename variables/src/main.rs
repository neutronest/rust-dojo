fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);

    const THREE_HOURS_IN_SECONDS_MULTIFY_X: u32 = 60 * 60 * 3;
    println!(
        "THREE_HOURS_IN_SECONDS_MULTIFY_X: {}",
        THREE_HOURS_IN_SECONDS_MULTIFY_X
    );

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outter scope is: {}", x);
}
