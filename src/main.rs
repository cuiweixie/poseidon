use std::fs::File;
use poseidon::Poseidon;

use halo2curves::bn256::Fr;

fn main() {
    // Define the parameters for the Poseidon hasher
    let number_of_full_rounds = 2;
    let number_of_half_rounds = 1;
    let number_of_inputs_0 = 2;
    let number_of_inputs_1 = 3;

    // Initialize a mutable hasher with constant capacity parameters
    // and number of rounds arguments. This will also generate matrices
    // and constants according to the specification
    let mut hasher = Poseidon::<Fr, 2, 1>::new(number_of_full_rounds, number_of_half_rounds);

    let json = serde_json::to_string_pretty(&hasher.clone().spec).unwrap();
    println!("Serialized JSON: {}", json);
    let mut file = File::create("config.json").expect("Unable to create file");
    use std::io::Write;
    file.write_all(json.as_bytes()).expect("Unable to write data");
    // In sake of the example we generate some dummy scalar inputs
    let inputs = (0..number_of_inputs_0)
        .map(|_| Fr::one())
        .collect::<Vec<Fr>>();

    // Feed inputs to the Absorption line
    hasher.update(&inputs[..]);

    println!("state final: {:?}", hasher.get_state());

    // Yield your challenge with squeeze function
    let challenge_alpha = hasher.squeeze();

    println!("challenge_alpha: {:?}", challenge_alpha);
}
