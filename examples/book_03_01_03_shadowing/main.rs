use rrt_lib::data::{Event, LifetimeTrait, ResourceOwner, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
// visualization of shadowing
// Ch 3.1 "The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again"
fn main() {
    let x = vd.create_resource_owner (
        ResourceOwner(Variable {
            hash: 1,
            name: String::from("x"),
            is_mut: false,
            is_ref: false,
            lifetime_trait: LifetimeTrait::None,
        })
    );
    let x1 = vd.create_resource_owner ( 
        ResourceOwner(Variable {
            hash: 2,
            name: String::from("x"),
            is_mut: false,
            is_ref: false,
            lifetime_trait: LifetimeTrait::Copy,
        })
    );
    let x2 = vd.create_resource_owner ( 
        ResourceOwner(Variable {
            hash: 3,
            name: String::from("x"),
            is_mut: false,
            is_ref: false,
            lifetime_trait: LifetimeTrait::Copy,
        })
    );  

    //
    // hash x : 1
    //      x1 : 2
    //      x2 : 3
    // functions: 0
    vd.append_event(&x, Event::Acquire { from: None }, &(2 as usize));
    // Mark event: "x" acquire value x+1
    vd.append_acq_event(variable=&x1, from=&x, line_num=(4 as usize));
    vd.append_event(&x1, Event::Acquire { from: Some(x) }, &(4 as usize));
    vd.append_event(&x1, Event::Acquire { from: x.hash }, &(4 as usize));
    vd.append_event(&x, Event::GoOutOfScope, &(4 as usize));
    // Mark event: "x" acquire value x*2
    vd.append_event(&x2, Event::Acquire { from: Some(x1)) }, &(6 as usize));
    vd.append_event(&x1, Event::GoOutOfScope, &(6 as usize));
    //x goes out of scope
    vd.append_event(&x2, Event::GoOutOfScope, &(9 as usize));

    //rendering image
    svg_generation::render_svg(&"book_03_01_03_shadowing".to_owned(), &vd);
}
