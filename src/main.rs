use std::fs::File;
use poseidon::Poseidon;

use halo2curves::bn256::Fr;

fn main() {
    // Define the parameters for the Poseidon hasher
    // let number_of_full_rounds = 8;
    // let number_of_half_rounds = 56;

    // Initialize a mutable hasher with constant capacity parameters
    // and number of rounds arguments. This will also generate matrices
    // and constants according to the specification
    let mut hasher = Poseidon::<Fr, 9, 8>::new(8, 63);

    let json = serde_json::to_string_pretty(&hasher.clone().spec).unwrap();
    println!("Serialized JSON: {}", json);
    let mut file = File::create("config.json").expect("Unable to create file");
    use std::io::Write;
    file.write_all(json.as_bytes()).expect("Unable to write data");

    hasher.update(&[Fr::one()]);
    let result = hasher.squeeze();
    let array: [u8; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
    ];
    Fr::from_repr(array).unwrap();
    println!("hash result is {:?}", result);
}
