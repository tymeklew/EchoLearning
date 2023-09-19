const CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@£$%^*()";
use rand::Rng;

pub fn generate_secret() -> String {
    let mut output = String::new();
    for _ in 0..40 {
        let rand = rand::thread_rng().gen_range(0..CHARACTERS.len() - 1);
        let char: char = CHARACTERS.chars().nth(rand).unwrap();
        output += &char.to_string();
    }
    return output;
}
