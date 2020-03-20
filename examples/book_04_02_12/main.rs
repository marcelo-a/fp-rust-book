use rrt_lib::data::{Event, ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;
// visualization of shadowing
// Ch 3.3
fn main() {
    let s = ResourceOwner::Variable(Variable {
        hash: 1,
        name: String::from("s"),
        is_mut: false,
        is_ref: false,
        is_func: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    let no_dangle = ResourceOwner::Function(Function {
        hash: 2,
        name: String::from("no_dangle"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    

    vd.append_event(&s, Event::Acquire { from: None }, &(2 as usize));
    
    vd.append_external_event(ExternalEvent::Move{from: Some(s.clone()), to: Some(no_dangle.clone())}, &(5 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : no_dangle.clone() },  &(5 as usize));
    vd.append_event(&s, Event::GoOutOfScope{},  &(5 as usize));
    
    //rendering image
    svg_generation::render_svg(&"book_04_02_12".to_owned(), &vd);
}
