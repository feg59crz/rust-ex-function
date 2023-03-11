fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = five();

    println!("The value of y is: {y}");

    let z = plus_one(y);
    
    println!("The value of z is: {z}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
