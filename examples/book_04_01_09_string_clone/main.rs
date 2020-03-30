use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, VisualizationData, Visualizable};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let s1 = ResourceOwner::Variable(Variable {
        hash: 1,
        name: String::from("s1"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move
    });
    let s2 = ResourceOwner::Variable(Variable {
        hash: 2,
        name: String::from("s2"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Move
    });
    let from_func = ResourceOwner::Function(Function {
        hash: 3,
        name: String::from("String::from()"),
    });
    let clone_func = ResourceOwner::Function(Function {
        hash: 4,
        name: String::from("clone()"),
    });
    let println_func = ResourceOwner::Function(Function {
        hash: 5,
        name: String::from("println!()"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };

    vd.append_external_event(ExternalEvent::Duplicate{from: Some(from_func.clone()), to: Some(s1.clone())}, &(2 as usize));
    vd.append_external_event(ExternalEvent::Move{from: Some(clone_func.clone()), to: Some(s2.clone())}, &(3 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(s1.clone()), to: Some(clone_func.clone())}, &(3 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(s1.clone()), to: Some(println_func.clone())}, &(5 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(s2.clone()), to: Some(println_func.clone())}, &(5 as usize));
    svg_generation::render_svg(&"04_01_09".to_owned(), &"string_clone".to_owned(), &vd);
}
