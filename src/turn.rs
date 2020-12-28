pub fn is_valid(s:&str) -> bool {
	if s.len() != 4{
		println!("Your input is invalid, please try again");
		return false;
	}

	if s != s.to_uppercase(){
		println!("Your input is invalid, please try again");
		return false;
	}

	let chars: Vec<char> = s.chars().collect();
	for c in chars.into_iter(){
		if char::is_numeric(c){
			println!("Your input is invalid, please try again");
			return false;
		}
	}

	return true;
}

pub fn is_correct(secret:&str, guess:&str) -> bool {
	let mut secret_chars: Vec<char> = secret.chars().collect();
	let mut guess_chars: Vec<char> = guess.chars().collect();

	let mut black = 0;
	let mut white = 0;

	for it in secret_chars.iter().zip(guess_chars.iter_mut()) {
		let (s, g) = it;
		if *s == *g {
			black += 1;
		}
	}

	guess_chars.sort();
	secret_chars.sort();


	for it in secret_chars.iter().zip(guess_chars.iter_mut()) {
		let (s, g) = it;
		if *s == *g {
			white += 1;
		}
	}
	white -= black;
	println!("{}b_{}w", black, white);

	if black == 4{
		return true;
	}
	
	return false;
}