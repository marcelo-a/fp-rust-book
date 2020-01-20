fn foo(mut x: i32){

    let  z =  &   & mut x;
    **z = 2;
    println!("{}", **z);

}

fn bar(str1: String) {
    let mut str2 = &str1;
    print!("{}", str2);
}
fn main(){
    foo(1);
    bar(String::from("hey"));
}