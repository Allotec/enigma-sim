pub mod rotor;

use lazy_static::lazy_static;
use rotor::Rotor;

#[derive(Debug)]
struct Enigma {
    main_rotors: Vec<Rotor>,
    entry_rotor: Option<Rotor>,
    reflect_rotor: Option<Rotor>,
}

impl Enigma {
    pub fn new(
        main_rotors: Vec<Rotor>,
        entry_rotor: Option<Rotor>,
        reflect_rotor: Option<Rotor>,
    ) -> Self {
        Self {
            entry_rotor,
            main_rotors,
            reflect_rotor,
        }
    }
}

lazy_static! {
    // pub static ref MilEnigma: Enigma = Enigma::new(
    //     rotor::german_rail::
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {}
}
