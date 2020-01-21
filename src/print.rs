pub fn run(){

    //Format specifiers
    println!("{} world i am loving {}", "Hello", "Rust");

    //Variable based arguments
    println!("{wish} world i am loving {lang}", wish="Hello", lang="Rust");

    // Positional arguements
    println!("{0} world i am loving {1}", "Hello", "Rust");

    //Math operation
    println!("8 + 7 = {x}", x=8+7); 

    //Boolean operation 
    println!("7 < 10 = {}", 7 < 10); 
    println!("7 > 10 = {}", 7 > 10);

    //binary, hex and octal representation
    println!("15 in binary={:b}, in hex={:X}, in octal={:o}", 15,15,15);

    println!("{:?}", (7==10, "star"=="star", "sun" == "star"));

}