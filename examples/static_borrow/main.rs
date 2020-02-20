use rrt_lib::data::{Event, ExternalEvent, ResourceOwner, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
// visualization of simple_lifetime/example.rs
fn main() {
    let s = ResourceOwner {
        hash: 1,
        name: String::from("s"),
        is_mut: true,
    };
    let x = ResourceOwner {
        hash: 2,
        name: String::from("x"),
        is_mut: false,
    };
    let y = ResourceOwner {
        hash: 3,
        name: String::from("y"),
        is_mut: false,
    };
    let z = ResourceOwner {
        hash: 4,
        name: String::from("z"),
        is_mut: true,
    };
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };

    //
    // hash s : 1
    //      x : 2
    //      y : 3
    //      z : 4
    // functions: 0
    vd.append_event(&s, Event::Acquire { from: None }, &(2 as usize));
    // Mark event: "x" borrows immutable reference from "s"
    vd.append_event(&s, Event::StaticLend { to: Some(x.clone()) }, &(4 as usize));
    vd.append_event(&x, Event::StaticBorrow { from: s.clone()/*Some(s.clone())*/ }, &(4 as usize));
    // Mark event: "y" borrows immutable reference from "s"
    vd.append_event(&s, Event::StaticLend { to: Some(y.clone()) }, &(5 as usize));
    vd.append_event(&y, Event::StaticBorrow { from: s.clone()/*Some(s.clone())*/ }, &(5 as usize));
    // "x" and "y" return reference and go out of scope
    vd.append_event(&x, Event::StaticReturn{ to: Some(s.clone()) }, &(7 as usize));
    vd.append_event(&y, Event::StaticReturn{ to: Some(s.clone()) }, &(7 as usize));
    vd.append_event(&s, Event::StaticReacquire{ from: Some(z.clone()) }, &(7 as usize));
    // Mark event: "z" borrows mutable reference form "s"
    vd.append_event(&s, Event::MutableLend { to: Some(z.clone()) }, &(10 as usize));
    vd.append_event(&z, Event::MutableBorrow { from: s.clone()/*Some(s.clone())*/ }, &(10 as usize));
    // "z" goes out of scope
    vd.append_event(&z, Event::MutableReturn { to: Some(s.clone()) }, &(11 as usize));
    vd.append_event(&s, Event::StaticReacquire{ from: Some(z.clone()) }, &(11 as usize));
    //value of s is returned and goes out of scope
    vd.append_event(&s, Event::GoOutOfScope, &(12 as usize));
    vd.append_event(&x, Event::GoOutOfScope, &(12 as usize));
    vd.append_event(&y, Event::GoOutOfScope, &(12 as usize));
    vd.append_event(&z, Event::GoOutOfScope, &(12 as usize));

    //rendering image
    svg_generation::render_svg(&"static_borrow".to_owned(), &vd);
}
