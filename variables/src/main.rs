const STARTING_MISSLES:  i32 = 8;
const READY_AMOUNT:  i32 = 2;
fn main() {
    let (missles, ready) = (STARTING_MISSLES,READY_AMOUNT);
    let _ready_amount = 1;
    println!("Firing {} of my {} missles...",ready, missles);

    // missles = missles - ready;
    println!("{} missles left", missles - ready);
}
