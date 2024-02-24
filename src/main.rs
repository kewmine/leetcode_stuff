mod problems;


fn main() {
    let str1 = String::from("AAAAAAAAAA");
    let str2 = String::from("AAA");

    let output = gcd_of_strings(str1, str2);

    println!("{}", output);
}


pub fn gcd_of_strings(str1: String, str2: String) -> String {

    // find the shorter string
    let short_string = if str1.len() < str2.len() { &str1 } else if str1.len() > str2.len() { &str2 } else { &str1 };

    let mut possible_gcd = String::new();
    let mut gcd = String::new();
    // gcd cannot be longer than shorter string
    for i in short_string.chars() {
        // push string characters one by one
        possible_gcd.push(i);

        // expand the created gcd to the length of the string
        let mut test_string1 = String::new();
        while test_string1.len() < str1.len() {
            test_string1 += possible_gcd.repeat(1).as_str();
        }

        // if str1 matches do the same for str2
        if test_string1 == str1 {
            let mut test_string2 = String::new();
            while test_string2.len() < str2.len() {
                test_string2 += possible_gcd.repeat(1).as_str();
            }
            // return gcd if str2 also matches
            if test_string2 == str2 {
                gcd = possible_gcd.clone();
            }
        }
    }

    gcd
}