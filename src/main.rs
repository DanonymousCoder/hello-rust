fn main() {
    cool_function();
}
fn cool_function() {
    let mut even_numbers: Vec<i32> = Vec::new();
    let max_value = 300;

    for num in 1..=max_value {
        if num % 2 == 0 {
            println!("{}", num);
            even_numbers.push(num);
        }
    }

    println!("Even numbers sequence: {:?}", even_numbers);
}


