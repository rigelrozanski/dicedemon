extern crate ring;
extern crate bip39;
extern crate bit_vec;

use bip39::Language;
use bip39::MnemonicType;
use bit_vec::BitVec;
use ring::digest::{self, digest};

fn main() {
    
    // temp code 
    let oneword = "abandon";
    let mut wordvec = vec![];
    let numwords = 23;
<<<<<<< HEAD
    let target_mnemonic = MnemonicType::Type24Words;
    assert_eq!(numwords,target_mnemonic.word_count()-1);

    let added_entropy= target_mnemonic.entropy_bits() - numwords *11;     

    for x in 0..numwords { 
=======
    for _ in 0..numwords { 
>>>>>>> 2f459266847f1cd9aa4c2b72da252a08dced5c2a
        wordvec.push(oneword);
    }

    let lang = Language::English;
    let word_map = lang.get_wordmap(); 
    let mut to_validate: BitVec = BitVec::new();
    
    for word in wordvec {
        let n = match word_map.get(word) {
            Some(n) => n,
            None => panic!("uh oh")
        };
        for i in 0..11 {
            let bit = bit_from_u16_as_u11(*n, i);
            to_validate.push(bit);
        }
    }

<<<<<<< HEAD

    for x in 0..added_entropy{
        let bit = false; //used fixed bits
        to_validate.push(bit);
    }

    println!("{}", wordvec[0]);
=======
    //println!("{}", wordvec[0]);
>>>>>>> 2f459266847f1cd9aa4c2b72da252a08dced5c2a
}

fn sha256(input: &[u8]) -> Vec<u8> {
    static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
    let hash = digest(DIGEST_ALG, input);
    hash.as_ref().to_vec()
}

fn bit_from_u16_as_u11(input: u16, position: u16) -> bool {
    if position < 11 {
        input & (1 << (10 - position)) != 0
    } else {
        false
    }
}
