use rand::seq::SliceRandom;

// All possible cube moves represented as `u8`s.
const BASE: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

// The disjoint versions of CUBE_TURNS, which vary based on what the last choice was.
// These are stored as constants to prevent having to calculate and re-calculate set differences
const NO_RL: [u8; 8] = [4, 5, 6, 7, 8, 9, 10, 11];
const NO_FB: [u8; 8] = [0, 1, 2, 3, 8, 9, 10, 11];
const NO_UD: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

const MAPPINGS: [&str; 12] = [
    "R", "R'", "L", "L'", "F", "F'", "B", "B'", "U", "U'", "D", "D'",
];

/// Used to map the u8 `id` to the name of the turn as a `String`
fn id_to_str(id: u8) -> String {
    MAPPINGS[id as usize].to_string()
}

/// Generates a sequence to shuffle the cube
pub fn shuffler(length: u64) -> Vec<String> {
    let mut sequence: Vec<String> = Vec::new();
    let mut last: u8 = *BASE.choose(&mut rand::thread_rng()).unwrap();
    sequence.push(id_to_str(last));

    // starts at one because last is initialized with the first move
    for _ in 1..length {
        last = pick_turn(last);
        sequence.push(id_to_str(last));
    }

    sequence
}

/// Generates an individual move for shuffling the cube
fn pick_turn(prev: u8) -> u8 {
    let mut rng = rand::thread_rng();

    if prev < 4 {
        return *NO_RL.choose(&mut rng).unwrap();
    } else if 3 < prev && prev < 8 {
        return *NO_FB.choose(&mut rng).unwrap();
    } else {
        return *NO_UD.choose(&mut rng).unwrap();
    }
}
