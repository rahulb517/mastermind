mod code_gen;
mod turn;

use std::io::{self};


fn main() {
	let mut correct: bool = false;
	let mut turns_left: i32 = 12;
	let secret = code_gen::generate();
	//let secret: String = "SJAM".to_string();
	println!("{}", secret);	
	println!("Welcome to Mastermind!");
	println!("To win, you must guess the secret code. You have 12 tries to do so.");
	println!("After each turn, you will recieve feedback in the form of colored pegs.");
	println!("A white peg indicates that you have selected a correct color but at an incorrect position.");
	println!("A black peg indicates that you have selected a correct color at the correct position.");
	println!("There are 8 colors to choose from: R, G, B, Y, O, C, I, and V ");



	println!("Would you like to play? (Y/N)");

	let mut choice = String::new();
	match io::stdin().read_line(&mut choice) {
		Ok(_) => {}
		Err(err) => println!("error: {}", err),
	}
	choice = choice.trim().to_string();

	while choice == "Y"{
		while correct == false && turns_left > 0 {
			let mut guess = String::new();
			println!("Enter a guess: ");
			match io::stdin().read_line(&mut guess) {
				Ok(_) => {}
				Err(err) => println!("error: {}", err),
			}
			guess = guess.trim().to_string();

			if turn::is_valid(&guess){
				correct = turn::is_correct(&secret, &guess);
				turns_left -= 1;
			}		
		}

		if correct == true{
			println!("Congrats, you won!");
		}
		else{
			println!("Oof, you lost, the correct code was {}", secret)
		}
		println!("Would you like to play again? (Y/N)");
		choice = String::new();
		match io::stdin().read_line(&mut choice) {
			Ok(_) => {}
			Err(err) => println!("error: {}", err),
		}
		choice = choice.trim().to_string();

	}
	println!("Thanks for playing!");
}
