fn main() {
    encrypt('X','D');
}

fn toNum(character: char) -> u8 {
    let mut x = character as u8;
    x = x - 65; // gets in range of 0-25
    return x as u8;
}

fn toChar(number: u8) -> char {
    let x: u8 = number + 65; // gets back into ascii value
    let character: char = x as char;
    
    return character;
}

fn encrypt(x: char, key: char) { // can only take upcase letters
    let x: u8 = toNum(x);
    let key: u8 = toNum(key) + 1;
    let intermediate = (x + key) % 26;
    let value: char = toChar(intermediate);
    println!("Expected: B; Result {}", value);
}
