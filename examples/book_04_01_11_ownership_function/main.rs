use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    // Variables
    let s = ResourceOwner::Variable(Variable {
        hash: 1,
        name: String::from("s"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    let x = ResourceOwner::Variable(Variable {
        hash: 2,
        name: String::from("x"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Copy,
    });
    let some_string = ResourceOwner::Variable(Variable {
        hash: 3,
        name: String::from("some_string"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move,
    });
    let some_integer = ResourceOwner::Variable(Variable {
        hash: 4,
        name: String::from("some_integer"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Copy,
    });
    // Functions
    let from_func = ResourceOwner::Function(Function {
        hash: 5,
        name: String::from("String::from()"),
    });
    let takes_ownership = ResourceOwner::Function(Function {
        hash: 6,
        name: String::from("takes_ownership()"),
    });
    let makes_copy = ResourceOwner::Function(Function {
        hash: 7,
        name: String::from("makes_copy()"),
    });
    let println_func = ResourceOwner::Function(Function {
        hash: 8,
        name: String::from("println!()"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };

    vd.append_external_event(ExternalEvent::Move{from: Some(from_func.clone()), to: Some(s.clone())}, &(2 as usize));
    vd.append_external_event(ExternalEvent::Move{from: Some(s.clone()), to: Some(takes_ownership.clone())}, &(4 as usize));
    vd.append_external_event(ExternalEvent::Duplicate{from: None, to: Some(x.clone())}, &(7 as usize));
    vd.append_external_event(ExternalEvent::Duplicate{from: Some(x.clone()), to: Some(makes_copy.clone()) }, &(9 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: s }, &(13 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: x }, &(13 as usize));

    vd.append_external_event(ExternalEvent::Duplicate{from: None, to: Some(some_string.clone()) }, &(16 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(some_string.clone()), to: Some(println_func.clone()) }, &(17 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: some_string }, &(18 as usize));

    vd.append_external_event(ExternalEvent::Duplicate{from: None, to: Some(some_integer.clone()) }, &(21 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(some_integer.clone()), to: Some(println_func.clone()) }, &(22 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: some_integer }, &(23 as usize));

    //rendering image
    svg_generation::render_svg(&"04_01_11".to_owned(), &"ownership_function".to_owned(), &vd);
}
