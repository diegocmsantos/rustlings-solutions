// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.

fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("{:?}", my_option);
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    println!("This Vec is empty, see? {:?}", my_empty_vec.clear());

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    (value_a, value_b) = (value_b, value_a);
    println!("value a: {value_a}; value b: {value_b}");
}