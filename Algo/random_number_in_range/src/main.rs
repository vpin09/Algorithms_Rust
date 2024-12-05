use rand::*;
fn main() {
    let mut rng=rand::thread_rng();
    println!("Integer : {}", rng.gen_range(0..10) );
    println!("float : {}", rng.gen_range(0.0..10.0) );
}
   
