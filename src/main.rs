extern crate rand;
use rand::*;
use rand::distributions::{Bernoulli, Distribution};


fn main() {
    let mut rng = rand::thread_rng();

    println!("rand::random::<u8>() -> {}", rand::random::<u8>());
    println!("rand::random::<i8>() -> {}", rand::random::<i8>());
    println!("rand::random::<u64>() -> {}", rand::random::<u64>());
    println!("rand::random::<f32>() -> {}", rand::random::<f32>());
    println!("rand::random::<f64>() -> {}", rand::random::<f64>());

    println!("Random u8: {}", rng.gen::<u8>());
    println!("Random u16: {}", rng.gen::<i8>());
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
    
    println!(" from 1 to 3: {}", rng.gen_range(1..3));
    
    let bernoulli = Bernoulli::new(0.3).unwrap();
    println!("bernoulli -> {}", bernoulli.sample(&mut rng));

}
