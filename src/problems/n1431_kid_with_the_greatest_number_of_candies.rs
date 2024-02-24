pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {

    let mut result = vec![];

    // find which kid initially has most candies
    let mut most_candies = candies[0];
    for i in 0..candies.len() {
        if most_candies < candies[i] { most_candies = candies[i] }
    }

    println!("{most_candies}");

    // check if each element after adding extras will be greater than max or not
    for i in 0..candies.len() {
        if candies[i]+extra_candies >= most_candies {result.push(true)}
        else {result.push(false)}
    }

    result
}
