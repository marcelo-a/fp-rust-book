use rrt_lib::data::{Event, LifetimeTrait, ResourceOwner, Variable, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
// visualization of shadowing
// Ch 3.3
fn main() {
    let x = ResourceOwner::Variable(Variable {
        hash: 1,
        name: String::from("x"),
        is_mut: false,
        is_ref: false,
        is_func: false,
        lifetime_trait: LifetimeTrait::None,
    });
    let plus_one = ResourceOwner::Function(Function {
        hash: 2,
        name: String::from("plus_one()"),
    });
    let plus_one_x = ResourceOwner::Variable(Variable {
        hash: 3,
        name: String::from("x"),
        is_mut: false,
        is_ref: false,
        is_func: false,
        lifetime_trait: LifetimeTrait::Copy,
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };

    //
    // hash x : 1
    //      plus_one : 2
    // functions: 1
    vd.append_event(&plus_one, Event::Acquire { from: None }, &(2 as usize));
    // Mark event: "x" acquire value x+1
    vd.append_event(&x, Event::Acquire { from: Some(plus_one.clone()) }, &(2 as usize));
    vd.append_event(&plus_one, Event::Move { to: Some(x.clone()) }, &(2 as usize));
    vd.append_event(&plus_one, Event::GoOutOfScope, &(2 as usize));
    //x goes out of scope
    vd.append_event(&x, Event::GoOutOfScope, &(5 as usize));
    //plus_one function call
    vd.append_event(&plus_one_x, Event::Acquire { from: None }, &(7 as usize));
    vd.append_event(&plus_one_x, Event::GoOutOfScope, &(9 as usize));
    
    // vd.append_event()
    //rendering image
    svg_generation::render_svg(&"book_03_03_09_functions".to_owned(), &vd);
}
