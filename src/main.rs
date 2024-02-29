use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("☀️📦 Guessing Game!");
    
    // generate rand code
    let secret: u32 = rand::thread_rng().gen_range(1..100);
    
    println!( "📦 secret memory addr: {:p}", &secret);
    
    loop {
        // get input
        println!("Enter a number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input invalid.");
        
        // save as u32
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🔴 must be a positive num.");
                continue
                },
        };
        
        // compare it
        match input.cmp(&secret)  {
            Ordering::Less => { println!("🔻 {input} is too low."); continue },
            Ordering::Equal => { println!("🟢 You got it! 💐"); break },
            Ordering::Greater => { println!("🔺 {input} is too high."); continue },
        };
    }
    
}