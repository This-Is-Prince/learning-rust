
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let r= &x;

    let x = x+1;
    {
        let x= x+2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    println!("The value of r is: {r}");
}
