// example of an Acquire Event that has 2 'from' ResourceOwners
fn  acqure_from_two() {
    let y = String::from("y");
    let z = String::from("z");
    let x = {
        if 17 == 17 {
            y
        }
        else {
            z
        }
    };
    println!("{}", x);
    // println!("{}", y);       // Uncomment this line incurs error[E0382]: borrow of moved value: `y`
    // println!("{}", z);       // Uncomment this line incurs error[E0382]: borrow of moved value: `z`
}

fn main() {
    acqure_from_two();
}