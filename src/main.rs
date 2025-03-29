mod game_utils;

fn main() {
    let mut my_stack = game_utils::my_stack::MyStack::new();
    my_stack.push((1, 2));
    my_stack.push((2, 3));
    my_stack.push((4, 5));

    let element = my_stack.peek();
    println!("{:?}", element);

    while !my_stack.is_empty() {
        let (a, b) = my_stack.pop().unwrap();
        println!("{} {}", a, b);
    }

}

