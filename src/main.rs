fn main() {
    println!("Hello, world!");
    another_function(5, 'T'); 
    println!("{}", plus_one(58)); 
}


fn another_function(x: i32, unit_label: char) { 
    println!("The value of x is {x}{unit_label}");
} 


// expressions vs statements 


fn plus_one(x: i32) -> i32 { 
    x + 1 
}