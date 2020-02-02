use rrt_lib::data::{ResourceOwner, VisualizationData, LifetimeTrait, Event, Visualizable};

use std::collections::HashMap;
// visualization of simple_lifetime/example.rs
fn main() {
    let x = ResourceOwner {
        hash: 1,
        name: String::from("x")
    };
    let y= ResourceOwner {
        hash: 2,
        name: String::from("y")
    };
    let z= ResourceOwner {
        hash: 3,
        name: String::from("z")
    };
    let mut vd = VisualizationData{
        timelines : HashMap::new(),
    };
    // 
    // hash x : 1
    //      y : 2
    //      z : 3
    // functions: 0
    vd.append_event(&y, Event::Acquire{from : None}, &(2 as usize));
    vd.append_event(&z, Event::Acquire{from : None}, &(3 as usize));
    vd.append_event(&y, Event::Transfer{to : Some(x.clone())}, &(4 as usize));
    vd.append_event(&x, Event::Acquire{from : Some(y.clone())}, &(4 as usize));
    vd.append_event(&x, Event::GoOutOfScope, &(9 as usize));
    vd.append_event(&z, Event::GoOutOfScope, &(9 as usize));
    vd.append_event(&y, Event::GoOutOfScope, &(9 as usize));
    println!("AAA");
}
