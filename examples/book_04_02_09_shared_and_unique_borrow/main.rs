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
        is_func: false,
        lifetime_trait: LifetimeTrait::Move,
    };
    let r1 = ResourceOwner {
        hash: 2,
        name: String::from("r1"),
        is_mut: false,
        is_ref: true,
        is_func: false,
        lifetime_trait: LifetimeTrait::None,
    };
    let r2 = ResourceOwner {
        hash: 3,
        name: String::from("r2"),
        is_mut: false,
        is_ref: true,
        is_func: false,
        lifetime_trait: LifetimeTrait::None,
    };
    let r3 = ResourceOwner {
        hash: 4,
        name: String::from("r3"),
        is_mut: false,
        is_ref: true,
        is_func: false,
        lifetime_trait: LifetimeTrait::None,
    };
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };

    //
    // hash s : 1
    //      r1 : 2
    //      r2 : 3
    //      r3 : 4
    // functions: 1
    // Event 1: give s the resource and make it the owner
    vd.append_event(&s, Event::Acquire { from: None }, &(2 as usize));
    // Mark event: "r1" borrows immutable reference from "s"
    // Events 2-3: lend s resource to r1 and r1 borrow resource from s
    vd.append_event(&s, Event::StaticLend { to: Some(r1.clone()) }, &(4 as usize));
    vd.append_event(&r1, Event::StaticBorrow { from: s.clone() }, &(4 as usize));
    // Events 4-5: lend s resource to r2 and r2 borrow resource from s
    vd.append_event(&s, Event::StaticLend { to: Some(r2.clone()) }, &(5 as usize));
    vd.append_event(&r2, Event::StaticBorrow { from: s.clone() }, &(5 as usize));
    // Event 6-8: r1 and r2 return resource priviledges to s
    vd.append_event(&r1, Event::StaticReturn{ to: Some(s.clone()) }, &(6 as usize));
    vd.append_event(&r2, Event::StaticReturn{ to: Some(s.clone()) }, &(6 as usize));
    vd.append_event(&s, Event::StaticReacquire{ from: Some(r1.clone()) }, &(6 as usize));
    vd.append_event(&s, Event::StaticReacquire{ from: Some(r2.clone()) }, &(6 as usize));
    // Events 9-10: mutable lend s resource to r3 and r3 borrow resource from s
    vd.append_event(&s, Event::MutableLend { to: Some(r3.clone()) }, &(9 as usize));
    vd.append_event(&r3, Event::MutableBorrow { from: s.clone() }, &(9 as usize));
    // Event 11-12: r3 return resource priviledges to s
    vd.append_event(&r3, Event::MutableReturn { to: Some(s.clone()) }, &(10 as usize));
    vd.append_event(&s, Event::MutableReacquire{ from: Some(r3.clone()) }, &(10 as usize));
    //all variables go out of scope at end of function
    vd.append_event(&s, Event::GoOutOfScope, &(12 as usize));
    vd.append_event(&r1, Event::GoOutOfScope, &(12 as usize));
    vd.append_event(&r2, Event::GoOutOfScope, &(12 as usize));
    vd.append_event(&r3, Event::GoOutOfScope, &(12 as usize));

    //rendering image
    svg_generation::render_svg(&"book_04_02_09_shared_and_unique_borrow".to_owned(), &vd);
}
