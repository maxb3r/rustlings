// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}

//Whenever we want to return a value  from a fn in Rust, it being considered as an expression, this means that we have to remove the semicolon since it's not a statement anymore