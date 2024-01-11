use sha2::{Digest, Sha256};

fn calculate_merkle_root(hashes: &mut [Vec<u8>]) -> Vec<u8> {
    if hashes.len() == 1 {
        return hashes[0].clone();
    }

    let mut new_hashes = vec![];
    for i in (0..hashes.len()).step_by(2) {
        let mut hasher = Sha256::new();
        hasher.update(&hashes[i]);
        if i + 1 < hashes.len() {
            hasher.update(&hashes[i + 1]);
        }
        new_hashes.push(hasher.finalize().to_vec());
    }

    calculate_merkle_root(&mut new_hashes)
}



fn main() {
    let og_address1 = b"D1zbwtqesJg4j47BAqLZ4UEfqgLNmn8Rnbqz42uzCuqm";
    let og_address2 = b"64hXByruLythJg4L46eus47FxNHMzFBU2ntadRE9yPpr";

    let mut hashes = vec![Sha256::digest(og_address1).to_vec(), Sha256::digest(og_address2).to_vec()];
    let merkle_root = calculate_merkle_root(&mut hashes);

    let merkle_root_hex: String = merkle_root.iter().map(|byte| format!("{:02x}", byte)).collect();
    println!("Merkle OG: {}", merkle_root_hex);


    let whitelist_address1 = b"3cnWESFiTamZSCgiCcdUmwHwkkro5CsVtSfEaJdu8Ba6";
    let whitelist_address2 = b"BwpFU4Ma6SQk6ZMpsBV2VuoPYkoQXtDQYg3idNobLDKv";

    let mut hashesWL = vec![Sha256::digest(whitelist_address1).to_vec(), Sha256::digest(whitelist_address2).to_vec()];
    let merkle_root_wl = calculate_merkle_root(&mut hashesWL);

    let merkle_root_wl_hex: String = merkle_root_wl.iter().map(|byte| format!("{:02x}", byte)).collect();
    println!("Merkle Whitelist: {}", merkle_root_wl_hex);
}