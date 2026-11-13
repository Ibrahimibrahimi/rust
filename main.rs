// use std::io; // for I/O
fn main() {
    // positional formatting
    println!("Hi ibrahim you are {1} you have {0} dh", "0", "student");

    // name based formatting
    println!(
        "Hi my name is {name} i have {age} years",
        name = "ibrahim",
        age = 11
    );

    // number formatting
    println!("Binary : {:b} {:x} {:o}", 11, 11, 11);

    // centered align

    println!("Center  aligned : {:^10} ll", 15);
    println!("Left aligned : {:<10} ll", 15);
    println!("Right aligned : {:>10} ll", 15);

    // formatting tips
    println!("price : {:12.2}", 15.12364); // zfill with 12 spaces + round 2
    // NOTE  use "+" to show the sign of the number
    println!("price : {:+12.2}", 15.12364);

    // padding with "0"
    println!("price : {:l^4}", 11);

    // custom padding char 'll'
    println!("price :{:l>11}", 11);

    // custom padding char + centered
    println!("{:-^22}", "hi");

    // string format function
    println!("{}", format!("Hi im {} ", "ibrahim"));

    // ------- Input ----------------
    //let mut input = String::new();
    //io::stdin().read_line(&mut input).unwrap();
    // println!("You typed:  {} !", input);
    // input.clear();

    //// ======= Example =========
    //println!("Enter your name : ");
    //std::io::stdin().read_line(&mut input).unwrap();

    // variables
    let name: String = "ibrahim".to_string();
    println!("Name : {}", name);

    // cast from number => string
    let myage: i16 = 11;
    let stringed: String = myage.to_string();
    println!("Stringned : {}", stringed);

    // Or simply
    let stringedsimple: String = format!("{}", myage);
    println!("{}", stringedsimple);

    // parse ; from string to integer
    let salarydata: String = "15000".to_string();

    // NOTE: ff
    let salarydataparsed: i32 = salarydata.parse().unwrap();
    println!("{} => {}", salarydata, salarydataparsed);
    println!("End");
}
