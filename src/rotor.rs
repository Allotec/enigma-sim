use lazy_static::lazy_static;

pub const fn is_unique_wiring(wiring: &str) -> &str {
    if wiring.len() != 26 {
        panic!("Wiring must be 26 characters long");
    }
    let bytes = wiring.as_bytes();
    let mut seen = [false; 26];
    let mut i = 0;

    while i < 26 {
        let c = bytes[i];
        if !(c >= b'A' && c <= b'Z') {
            panic!("Wiring must be uppercase ASCII A-Z");
        }
        let idx = (c - b'A') as usize;
        if seen[idx] {
            panic!("Duplicate character in wiring");
        }
        seen[idx] = true;
        i += 1;
    }

    wiring
}

#[derive(Debug)]
pub struct Rotor {
    wiring: String,
    position: usize,
    notch: Vec<char>,
}

impl Rotor {
    pub fn new(wiring: &str, position: usize, notch: Vec<char>) -> Self {
        let wiring = is_unique_wiring(wiring);

        Self {
            wiring: wiring.to_string(),
            position,
            notch,
        }
    }
}

pub mod commerical {
    use super::*;

    lazy_static! {
        // 1924 Commerical Enigma A, B
        pub static ref ROTOR_I: Rotor = Rotor::new(
            is_unique_wiring("DMTWSILRUYQNKFEJCAZBPGXOHV"),
            0,
            vec!['E']
        );
        pub static ref ROTOR_II: Rotor = Rotor::new(
            is_unique_wiring("HQZGPJTMOBLNCIFDYAWVEUSRKX"),
            0,
            vec!['N']
        );
        pub static ref ROTOR_III: Rotor = Rotor::new(
            is_unique_wiring("UQNTLSZFMREHDPXKIBVYGJCWOA"),
            0,
            vec!['Y']
        );
    }
}

pub mod german_rail {
    use super::*;

    lazy_static! {
        // 7 February 1941	German Railway (Rocket)
        pub static ref ROTOR_I: Rotor = Rotor::new(is_unique_wiring("JGDQOXUSCAMIFRVTPNEWKBLZYH"), 0, vec!['E']);
        pub static ref ROTOR_II: Rotor = Rotor::new(is_unique_wiring("NTZPSFBOKMWRCJDIVLAEYUXHGQ"), 0, vec!['N']);
        pub static ref ROTOR_III: Rotor = Rotor::new(is_unique_wiring("JVIUBHTCDYAKEQZPOSGXNRMWFL"), 0, vec!['Y']);
        pub static ref ROTOR_ENTRY: Rotor = Rotor::new(is_unique_wiring("QWERTZUIOASDFGHJKPYXCVBNML"), 0, vec![]);
        pub static ref ROTOR_REFLECT: Rotor = Rotor::new(is_unique_wiring("QYHOGNECVPUZTFDJAXWMKISRBL"), 0, vec![]);
    }
}

pub mod swiss_k {
    use super::*;

    lazy_static! {
        //February 1939	Swiss K
        pub static ref ROTOR_I: Rotor = Rotor::new(is_unique_wiring("PEZUOHXSCVFMTBGLRINQJWAYDK"), 0, vec!['Y']);
        pub static ref ROTOR_II: Rotor = Rotor::new(is_unique_wiring("ZOUESYDKFWPCIQXHMVBLGNJRAT"), 0, vec!['E']);
        pub static ref ROTOR_III: Rotor = Rotor::new(is_unique_wiring("EHRVXGAOBQUSIMZFLYNWKTPDJC"), 0, vec!['N']);
        pub static ref ROTOR_ENTRY: Rotor = Rotor::new(is_unique_wiring("QWERTZUIOASDFGHJKPYXCVBNML"), 0, vec![]);
        pub static ref ROTOR_REFLECT: Rotor = Rotor::new(is_unique_wiring("IMETCGFRAYSQBZXWLHKDVUPOJN"), 0, vec![]);
    }
}

pub mod german_mil {
    use super::*;

    lazy_static! {
        // 1930 Enigma I
        pub static ref ROTOR_I: Rotor = Rotor::new(
            is_unique_wiring("EKMFLGDQVZNTOWYHXUSPAIBRCJ"),
            0,
            vec!['Q']
        );
        pub static ref ROTOR_II: Rotor = Rotor::new(
            is_unique_wiring("AJDKSIRUXBLHWTMCQGZNPYFVOE"),
            0,
            vec!['E']
        );
        pub static ref ROTOR_III: Rotor = Rotor::new(
            is_unique_wiring("BDFHJLCPRTXVZNYEIWGAKMUSQO"),
            0,
            vec!['V']
        );

        // December 1938	M3 Army
        pub static ref ROTOR_IV: Rotor = Rotor::new(
            is_unique_wiring("ESOVPZJAYQUIRHXLNFTGKDCMWB"),
            0,
            vec!['J']
        );

        pub static ref ROTOR_V: Rotor = Rotor::new(
            is_unique_wiring("VZBRGITYUPSDNHLXAWMJQOFECK"),
            0,
            vec!['Z']
        );

        // 1939	M3 & M4 Naval (FEB 1942)
        pub static ref ROTOR_VI: Rotor = Rotor::new(
            is_unique_wiring("JPGVOUMFYQBENHZRDKASXLICTW"),
            0,
            vec!['Z', 'M']
        );

        pub static ref ROTOR_VII: Rotor = Rotor::new(
            is_unique_wiring("NZJHGRCXMYSWBOUFAIVLPEKQDT"),
            0,
            vec!['Z', 'M']
        );

        pub static ref ROTOR_VIII: Rotor = Rotor::new(
            is_unique_wiring("FKQHTLXOCBJSPDZRAMEWNIUYGV"),
            0,
            vec!['Z', 'M']
        );

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet() {
        is_unique_wiring("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    #[test]
    #[should_panic]
    fn short() {
        is_unique_wiring("ABCDEF");
    }

    #[test]
    #[should_panic]
    fn duplicate() {
        is_unique_wiring("ABCDEFGHIJKLMNOPQRSTUVWXZZ");
    }
}
