extern crate ring;
extern crate bip39;
extern crate bit_vec;
extern crate bitreader;

#[macro_use]
extern crate structopt;

use bip39::Language;
use bip39::MnemonicType;
use bit_vec::BitVec;
use ring::digest::{self, digest};
use bitreader::BitReader;
use std::io::{stdin,stdout,Write};
use structopt::StructOpt;

/// Calculate checksum BIP39 words
#[derive(StructOpt, Debug)]
#[structopt(name = "dice-demon")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    
    /// Number of words to calculate for, must be either 12, 15, 18, 21, or 24
    #[structopt(short = "w", long = "numwords", default_value = "24")]
    numwords: usize,
}

fn main() {
  
    // get cli input
    let opt = Opt::from_args();
   
    // set num words
    let numwords = opt.numwords;
    let target_mnemonic;
    match numwords {
        12 => target_mnemonic = MnemonicType::Type12Words, 
        15 => target_mnemonic = MnemonicType::Type15Words, 
        18 => target_mnemonic = MnemonicType::Type18Words, 
        21 => target_mnemonic = MnemonicType::Type21Words, 
        24 => target_mnemonic = MnemonicType::Type24Words, 
        _ => {
            println!("Invalid number of words!");
            return;
        }
    }

    let mut buffer = String::new();
    let mut wordvec: Vec<String> = vec![];
    
    let lang = Language::English;
    let word_map = lang.get_wordmap(); 
    let word_list = lang.get_wordlist();

    // Retrieve the words
    println!("Enter word the first {} words then the checksum word will be computed.", (numwords -1));
    for i in 0..(numwords-1) {
        if opt.debug {
            let debug_word = "abandon";
            wordvec.push(debug_word.to_owned());
            continue;
        }

        print!("{}.\t",(i+1));

        let _=stdout().flush();
        stdin().read_line(&mut buffer).expect("Did not enter a correct string");    
        buffer.pop(); // remove the trailing new line

        let _ = match word_map.get(&buffer) {
            Some(_) => {},
            None => {
                println!("Entered non-bip39 word! ({})", &buffer);
                return;
            }
        };
        
        wordvec.push(buffer.clone());
        buffer.clear();
    }

    assert_eq!(numwords, target_mnemonic.word_count());

    // get word bits from entered words
    let added_entropy= target_mnemonic.entropy_bits() - (numwords-1) *11;     
    let mut to_validate: BitVec = BitVec::new();
    for word in wordvec {
        let n = match word_map.get(&word) {
            Some(n) => n,
            None => panic!("bad non-bip39 made it through!")
        };
        for i in 0..11 {
            let bit = bit_from_u16_as_u11(*n, i);
            to_validate.push(bit);
        }
    }

    // create the entropy (all 0s) - TODO add custom entropy bits to CLI
    for _ in 0..added_entropy{
        let bit = false; //used fixed bits
        to_validate.push(bit);
    }
    let entropy = to_validate.to_bytes();
    let hash = sha256(entropy.as_ref());
    let entropy_hash_to_validate_bits = BitVec::from_bytes(hash.as_ref());
    
    // add entropy bytes to the word bits
    &to_validate.extend(entropy_hash_to_validate_bits.into_iter().take(target_mnemonic.checksum_bits()));
    let word_bytes = &to_validate.to_bytes();
    let mut reader = BitReader::new(word_bytes);

    // read all the words back out (TODO only read the final word out?)
    let mut words: Vec<&str> = Vec::new();
    for _ in 0..numwords {
        let n = reader.read_u16(11);
        words.push(word_list[n.unwrap() as usize].as_ref());
    }

    // print the final checksum word
    println!("{}.\t{}",numwords, words[numwords-1])
}

fn sha256(input: &[u8]) -> Vec<u8> {
    static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
    let hash = digest(DIGEST_ALG, input);
    hash.as_ref().to_vec()
}

fn bit_from_u16_as_u11(input: u16, position: u16) -> bool {
    if position < 11 {
        input & (1 << (10 - position)) != 0
    } else { false }
}
