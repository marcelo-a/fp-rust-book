fn foo(mut v: i32){
    let x = &v;
    let y = &x;
    v += 1;
    //take(y);//err

}

fn take(v : &&i32){
}

fn the_book(){
    let mut r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    //println!("r: {}", r); //          |
    let a = 1;
    r = &a;
    println!("r: {}", r);
}   
fn main(){
    the_book();
}