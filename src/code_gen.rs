use rand::Rng;


pub fn generate() -> String{
	let mut rng = rand::thread_rng();
	let letters = vec!['R', 'G', 'B', 'Y', 'O', 'C', 'I', 'V'];
	let mut code = Vec::new();

	while code.len() < 4 { //this is to generate a code where all the colors are unique
		let x = rng.gen_range(0.. letters.len());
		if !code.iter().any(|&i| i==letters[x]) {
			code.push(letters[x]);
		}
		
	}

	return code.iter().collect();
}