use poseidon::Poseidon;

use halo2curves::bn256::Fr;

fn main() {
    // Define the parameters for the Poseidon hasher
    let number_of_full_rounds = 2;
    let number_of_half_rounds = 1;
    let number_of_inputs_0 = 1;
    let number_of_inputs_1 = 3;

    // Initialize a mutable hasher with constant capacity parameters
    // and number of rounds arguments. This will also generate matrices
    // and constants according to the specification
    let mut hasher = Poseidon::<Fr, 2, 1>::new(number_of_full_rounds, number_of_half_rounds);

    let json = serde_json::to_string(&hasher.clone().spec).unwrap();
    println!("Serialized JSON: {}", json);
    // In sake of the example we generate some dummy scalar inputs
    let inputs =  (0..number_of_inputs_0)
        .map(|_| Fr::one())
        .collect::<Vec<Fr>>();

    // Feed inputs to the Absorption line
    let r = hasher.update_exact(&[Fr::one()]);
    println!("r: {:?}", r);

    println!("state[0]: {:?}", hasher.get_state());

    // Yield your challenge with squeeze function
    let challenge_alpha = hasher.squeeze();

    // Then again ...
    let inputs = (0..number_of_inputs_1)
        .map(|_| Fr::one())
        .collect::<Vec<Fr>>();
    hasher.update(&inputs[..]);
    let challenge_beta = hasher.squeeze();

    // Print the challenges
    println!("Challenge Alpha: {:?}", challenge_alpha);
    println!("Challenge Beta: {:?}", challenge_beta);
}