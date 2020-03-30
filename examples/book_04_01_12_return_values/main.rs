use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    // Variables
    let s1 = ResourceOwner::Variable(Variable {
        hash: 1,
        name: String::from("s1"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    let s2 = ResourceOwner::Variable(Variable {
        hash: 2,
        name: String::from("s2"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    let s3 = ResourceOwner::Variable(Variable {
        hash: 3,
        name: String::from("s3"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    let some_string = ResourceOwner::Variable(Variable {
        hash: 4,
        name: String::from("some_string"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    let a_string = ResourceOwner::Variable(Variable {
        hash: 5,
        name: String::from("a_string"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    // Functions
    let gives_ownership = ResourceOwner::Function(Function {
        hash: 6,
        name: String::from("gives_ownership()"),
    });
    let takes_and_gives_back = ResourceOwner::Function(Function {
        hash: 7,
        name: String::from("takes_and_gives_back()"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    
    // s1 gets resource from gives_ownership()
    vd.append_external_event(ExternalEvent::Move{from: Some(gives_ownership.clone()), to: Some(s1.clone())}, &(2 as usize));
    // some_string gets resource from String::from()
    vd.append_external_event(ExternalEvent::Move{from: None, to: Some(some_string.clone())}, &(17 as usize));
    // move out as return value, does not render because to=None
    vd.append_external_event(ExternalEvent::Move{from: Some(some_string.clone()), to: None}, &(19 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : some_string.clone() }, &(22 as usize));

    // s2 gets resource from String::from()
    vd.append_external_event(ExternalEvent::Move{from: None, to: Some(s2.clone())}, &(5 as usize));
    // s2 moves resource to takes_and_gives_back()
    vd.append_external_event(ExternalEvent::Move{from: Some(s2.clone()), to: Some(takes_and_gives_back.clone())}, &(7 as usize));

    // a_string gets resource from parameter
    vd.append_external_event(ExternalEvent::Move{from: None, to: Some(a_string.clone())}, &(25 as usize));
    vd.append_external_event(ExternalEvent::Move{from: Some(a_string.clone()), to: None}, &(28 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: a_string }, &(29 as usize));


    // s3 gets resource from takes_and_gives_back()
    vd.append_external_event(ExternalEvent::Move{from: Some(takes_and_gives_back.clone()), to: Some(s3.clone())}, &(7 as usize));
    
    // s1, s2, s3 go out of scope
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: s1 }, &(10 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: s2 }, &(10 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: s3 }, &(10 as usize));

    //rendering image
    svg_generation::render_svg(&"04_01_12".to_owned(), &"return_values".to_owned(), &vd);
}
