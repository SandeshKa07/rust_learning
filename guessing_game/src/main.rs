use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game!");
    println!("Enter a number = ");

    //upper bound 101 generates number till 100.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {

        let mut guess = String::new();

	io::stdin().read_line(&mut guess)
	    .expect("Failed to read the input, crashing !!");

	println!("User guessed number = {}", guess);

	let guess : u32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => continue,
		};

	match guess.cmp(&secret_number) {
	    Ordering::Less => println!("Too less!!"),
    	    Ordering::Greater => println!("Too big"),
	    Ordering::Equal => {
		println!("Hooray!! Correct guess");
		break;
	    }
	}
    }
}
