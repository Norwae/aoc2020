use std::time::Instant;
use mod_exp::mod_exp;

const MODULUS: usize = 20201227;

fn disc_log_table() -> Box<[u32; MODULUS]> { // brute force to the max!
    let mut array = box [0u32;MODULUS]; // finally, an excuse to break out this syntax
    let mut curr = 1;
    for i in 1..MODULUS {
        curr = (curr * 7) % MODULUS;
        array[curr] = i as u32;
    }

    array
}

pub fn solve() {
    let start = Instant::now();
    let table = disc_log_table();
    println!("Building table took {:?}", Instant::now() - start);
    let pk_card = 2069194u32;
    let pk_door = 16426071u32;
    let loop_size_card = table[pk_card as usize];
    let loop_size_door = table[pk_door as usize];
    println!("Loop size card: {}", loop_size_card);
    println!("Loop size door: {}", loop_size_door);
    println!("Encryption key: {}", mod_exp(pk_door as u64, loop_size_card as u64, MODULUS as u64));
}