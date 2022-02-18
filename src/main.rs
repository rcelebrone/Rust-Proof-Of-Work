use sha2::{Sha256, Digest};
use std::time::SystemTime;

fn main() {

    let start = SystemTime::now();
    let nonce:&mut i32 = &mut 0;
    let data = "POC em rust do funcionamento da prova de trabalho na blockchain";
    let difficulty = 6;
    let hash:&mut String = &mut calculate_hash(data, &nonce);

    mine(data, difficulty, hash, nonce);

    let end = SystemTime::now();

    println!("Hash gerado: {}", hash);
    println!("Execuções: {}", nonce);
    println!("Duração em segundos: {:?}", end.duration_since(start));
}

fn calculate_hash(data: &str, nonce: &i32) -> String {
    
    let mut sha256 = Sha256::new();
    
    sha256.update(format!("{}-{}", data, nonce));
    
    format!("{:X}", sha256.finalize())
}

fn mine(data: &str, difficulty: usize, hash: &mut String, nonce: &mut i32) {

    let leading_zeros = "0".repeat(difficulty);

    while hash.is_empty() || hash[..difficulty] != leading_zeros {
        *nonce += 1;
        *hash = calculate_hash(data, &nonce);  
    }  
}  