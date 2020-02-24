use rrt_lib::data::{Event, LifetimeTrait, ResourceOwner, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
// visualization of static_borrow/example.rs
// taken from The Book chapter 4.2
// variable oriented: display stats on variable, not the data stored in it
fn main() {
    let s = ResourceOwner {
        hash: 1,
        name: String::from("s"),
        is_mut: true,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    };
    let x = ResourceOwner {
        hash: 2,
        name: String::from("x"),
        is_mut: false,
        is_ref: true,
        lifetime_trait: LifetimeTrait::None,
    };
    let y = ResourceOwner {
        hash: 3,
        name: String::from("y"),
        is_mut: false,
        is_ref: true,
        lifetime_trait: LifetimeTrait::None,
    };
    let z = ResourceOwner {
        hash: 4,
        name: String::from("z"),
        is_mut: false,
        is_ref: true,
        lifetime_trait: LifetimeTrait::None,
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
    // functions: 1
    // Event 1: give s the resource and make it the owner
    vd.append_event(&s, Event::Acquire { from: None }, &(2 as usize));
    // Mark event: "x" borrows immutable reference from "s"
    // Events 2-3: lend s resource to x and x borrow resource from s
    vd.append_event(&s, Event::StaticLend { to: Some(x.clone()) }, &(4 as usize));
    vd.append_event(&x, Event::StaticBorrow { from: s.clone() }, &(4 as usize));
    // Events 4-5: lend s resource to y and y borrow resource from s
    vd.append_event(&s, Event::StaticLend { to: Some(y.clone()) }, &(5 as usize));
    vd.append_event(&y, Event::StaticBorrow { from: s.clone() }, &(5 as usize));
    // Event 6-8: x and y return resource priviledges to s
    vd.append_event(&x, Event::StaticReturn{ to: Some(s.clone()) }, &(6 as usize));
    vd.append_event(&y, Event::StaticReturn{ to: Some(s.clone()) }, &(6 as usize));
    vd.append_event(&s, Event::StaticReacquire{ from: Some(x.clone()) }, &(6 as usize));
    vd.append_event(&s, Event::StaticReacquire{ from: Some(y.clone()) }, &(6 as usize));
    // Events 9-10: mutable lend s resource to z and z borrow resource from s
    vd.append_event(&s, Event::MutableLend { to: Some(z.clone()) }, &(9 as usize));
    vd.append_event(&z, Event::MutableBorrow { from: s.clone() }, &(9 as usize));
    // Event 11-12: z return resource priviledges to s
    vd.append_event(&z, Event::MutableReturn { to: Some(s.clone()) }, &(10 as usize));
    vd.append_event(&s, Event::MutableReacquire{ from: Some(z.clone()) }, &(10 as usize));
    //all variables go out of scope at end of function
    vd.append_event(&s, Event::GoOutOfScope, &(12 as usize));
    vd.append_event(&x, Event::GoOutOfScope, &(12 as usize));
    vd.append_event(&y, Event::GoOutOfScope, &(12 as usize));
    vd.append_event(&z, Event::GoOutOfScope, &(12 as usize));

    //rendering image
    svg_generation::render_svg(&"03_copy_scalar_var_TODO".to_owned(), &vd);
}
