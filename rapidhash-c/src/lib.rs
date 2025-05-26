mod bindings;

pub use bindings::*;

#[cfg(test)]
mod tests {
    use rand::RngCore;
    use super::*;
    
    #[test]
    #[ignore]  // proves a point, doesn't need to fail every CI run
    fn trivial_collision_attack() {
        const SECRETS: [u64; 3] = [
            0x2d358dccaa6c78a5,
            0x8bb84b93962eacc9,
            0x4b33a62ed433d4a3,
        ];
        
        fn random_slice() -> Vec<u8> {
            let mut data = vec![0; 32];
            let rng = &mut rand::rng();
            rng.fill_bytes(data.as_mut_slice());
            
            let offset = data.len() - 16;
            let a = &mut data[offset .. offset + 8];
            a.copy_from_slice(&SECRETS[1].to_le_bytes());
            
            data
        }
        
        assert_ne!(random_slice(), random_slice());
        let a = rapidhashcc_v2_2(&random_slice(), 0);
        let b = rapidhashcc_v2_2(&random_slice(), 0);
        assert_ne!(a, b, "Trivial collision attack: hashes are equal.");
    }
}