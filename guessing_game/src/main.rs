use std::io; // importing standard lib
use rand::Rng; // this import, not being standard, must be in the cargo.toml
use std::cmp::Ordering;


fn main() { //main function mandatory, always with no parameters
    println!("Guess the number!"); // print line

    let secret_number = rand::thread_rng().gen_range(1..101); //ccreates rand number (notice no mut)

    //println!("The secret number is: {}", secret_number);

    loop{
    println!("Please input your guess."); //print line (! is macro)

    let mut guess = String::new(); //variable (let) mutable (mut) String (:: means is associated function of string type)

    io::stdin() //caling stdin
        .read_line(&mut guess) //calls read line method from object std::io::Stdin // & means is a reference (does not copy the variable), also add mut because references are not mutable by default
        .expect("Failed to read line"); //handling error

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //handles input that is not a number
        };

    println!("You guessed: {}", guess);

    //he cmp method compares two values and can be called on anything that can be compared. 
    //It takes a reference to whatever you want to compare with: here it’s comparing the guess to the secret_number. 
    //Then it returns a variant of the Ordering enum we brought into scope with the use statement. 
    match guess.cmp(&secret_number) { //match guess variable with reference (&) of secret number
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal =>  {
            println!("You win!");
            break;
        }
    }
}
}