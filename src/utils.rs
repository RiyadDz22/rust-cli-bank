use std::io::stdin;

pub fn take_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub fn str_to_float(input: &str) -> Result<f32, String> {
    input
        .trim()
        .parse::<f32>()
        .map_err(|_| "❌ Please enter a valid number.".to_string())
}
