use rand::Rng;

fn operation(x: f64) -> f64 {
    x*x+2.*x+8.
}

fn main() {
    let mut difference;
    let solution = 0.;
    let mut starting_number: f64 = rand::thread_rng().gen();
    let mut attempts = 0;
    while attempts < 1000 {
        let test_case = operation(starting_number);
        if test_case == solution {
            break;
        }
        difference = test_case-solution;
        starting_number += difference;
        attempts += 1;
        println!("{}", test_case);
    }
    println!("{starting_number} and {attempts} attempts");
}
