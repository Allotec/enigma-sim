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

pub mod commerical {
    use super::is_unique_wiring;

    // 1924 Commerical Enigma A, B
    const ROTOR_IC: &str = is_unique_wiring("DMTWSILRUYQNKFEJCAZBPGXOHV");
    const ROTOR_IIC: &str = is_unique_wiring("HQZGPJTMOBLNCIFDYAWVEUSRKX");
    const ROTOR_IIIC: &str = is_unique_wiring("UQNTLSZFMREHDPXKIBVYGJCWOA");
}

pub mod german_rail {
    use super::is_unique_wiring;

    // 7 February 1941	German Railway (Rocket)
    const ROTOR_I: &str = is_unique_wiring("JGDQOXUSCAMIFRVTPNEWKBLZYH");
    const ROTOR_II: &str = is_unique_wiring("NTZPSFBOKMWRCJDIVLAEYUXHGQ");
    const ROTOR_III: &str = is_unique_wiring("JVIUBHTCDYAKEQZPOSGXNRMWFL");
    const ROTOR_UKW: &str = is_unique_wiring("QYHOGNECVPUZTFDJAXWMKISRBL");
    const ROTOR_ETW: &str = is_unique_wiring("QWERTZUIOASDFGHJKPYXCVBNML");
}

pub mod swiss_k {
    use super::is_unique_wiring;

    //February 1939	Swiss K
    const ROTOR_I: &str = is_unique_wiring("PEZUOHXSCVFMTBGLRINQJWAYDK");
    const ROTOR_II: &str = is_unique_wiring("ZOUESYDKFWPCIQXHMVBLGNJRAT");
    const ROTOR_III: &str = is_unique_wiring("EHRVXGAOBQUSIMZFLYNWKTPDJC");
    const ROTOR_UKW: &str = is_unique_wiring("IMETCGFRAYSQBZXWLHKDVUPOJN");
    const ROTOR_ETW: &str = is_unique_wiring("QWERTZUIOASDFGHJKPYXCVBNML");
}

pub mod german_mil {
    use super::is_unique_wiring;

    // 1930 Enigma I
    const ROTOR_I: &str = is_unique_wiring("EKMFLGDQVZNTOWYHXUSPAIBRCJ");
    const ROTOR_II: &str = is_unique_wiring("AJDKSIRUXBLHWTMCQGZNPYFVOE");
    const ROTOR_III: &str = is_unique_wiring("BDFHJLCPRTXVZNYEIWGAKMUSQO");

    // December 1938	M3 Army
    const ROTOR_IV: &str = is_unique_wiring("ESOVPZJAYQUIRHXLNFTGKDCMWB");
    const ROTOR_V: &str = is_unique_wiring("VZBRGITYUPSDNHLXAWMJQOFECK");

    // 1939	M3 & M4 Naval (FEB 1942)
    const ROTOR_VI: &str = is_unique_wiring("JPGVOUMFYQBENHZRDKASXLICTW");
    const ROTOR_VII: &str = is_unique_wiring("NZJHGRCXMYSWBOUFAIVLPEKQDT");
    const ROTOR_VIII: &str = is_unique_wiring("FKQHTLXOCBJSPDZRAMEWNIUYGV");
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
