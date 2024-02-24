#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {

    // put digits into an array
    let array:Vec<char> = x.to_string()
        .chars()
        .collect();

    // check if length of array is odd
    let is_odd = if array.len() % 2 == 0 { false } else { true };
    // how many digits to check
    let checks = if is_odd {array.len()-1/2} else {array.len()/2};

    for i in 0..checks {
        if array[i] != array[array.len()-i-1] { return false }
    }

    true
}