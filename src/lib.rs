mod hist_rotors;

use hist_rotors::is_unique_wiring;

struct Rotor {
    wiring: String,
}

impl Rotor {
    pub fn new(wiring: &str) -> Self {
        let wiring = is_unique_wiring(wiring);

        Self {
            wiring: wiring.to_string(),
        }
    }
}

struct Enigma {
    rotors: Vec<Rotor>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {}
}
