use std::io;

fn main() {
    println!("Hello, world!");
    println!("Guess the chicken number");
    let mut guess = String::new();
    let apple = 5;
    let pear = 94.273756375612756435672734526476532763542657342645732546724256732136887178139297397217913297312809148090834289030974097432987249842968324678278468726386423798;

    println!("Chicken number {}", pear);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to get the answer...");

    println!("The input is: {}, ah", { guess });
}
