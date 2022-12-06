const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 1;

fn main() {

    let state: (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    let (missile, ready)  = state;

    println!("Firing {} of my {} missiles", missile, ready);
    //missiles = missiles - ready;
    println!("{} missiles remaining", missile - ready);
}
