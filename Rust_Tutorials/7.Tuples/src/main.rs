#[allow(unused_variables)]
fn main() {
    println!("---------Tuples---------");

    // let some_tuple = (2, 3.4, "a", "b".to_string(), 'c', (1.1, true));
    // println!("My data is {} {}", some_tuple.0, some_tuple.1);
    // println!("My full tuple is {:?}", some_tuple);
    // let some_val = some_tuple.5 .1;
    // let some_val = (some_tuple.5).1;
    // let some_val = (some_tuple.5).2; // Error

    let some_color = get_some_rgb();
    println!("Green is {}", some_color.1);

    let (my_red, my_green, my_blue) = some_color;
    println!("r {} g {} b {}", my_red, my_green, my_blue);

    // rgba
    let some_other_color: (u8, u8, u8, u8) = (0, 100, 150, 255);
    println!("{:?}", some_other_color);

    let empty_tuple = ();

    match some_color.2 {
        0..=200 => println!("0 to 200"),
        _ => (),
    }

    some_function();
}

fn some_function() {
    ()
}

fn get_some_rgb() -> (u8, u8, u8) {
    // Some logic...
    (200, 100, 20)
}