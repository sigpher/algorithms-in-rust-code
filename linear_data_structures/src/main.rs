use linear_data_structures::Stack;

fn main() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(3);

    println!("{:?}",stack);
}
