use rand::Rng;

fn operation(x: i128) -> i128 {
    x*2
}

fn main() {
    let solution = 4; // the number we want to reach
    let mut starting_number: i128 = rand::thread_rng().gen_range(0..100); // get a random number to start
    let mut attempts = 0; // number of attempts to find the solution
    while attempts < 1000 { // limit the number of attempts
        let test_case = operation(starting_number); // run the code
        if test_case == solution { // check if the result is the solution
            break; // break out of the loop
        }
        let difference = solution-test_case; // calculate the difference between the result and the solution
        starting_number = starting_number + difference; // add the difference to the starting number
        attempts += 1; // increase the number of attempts
    }
    println!("{starting_number} and {attempts} attempts");
}
