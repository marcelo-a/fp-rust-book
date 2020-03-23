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
    let len = ResourceOwner::Variable(Variable {
        hash: 2,
        name: String::from("len"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::Copy,
    });
    let s = ResourceOwner::Variable(Variable {
        hash: 4,
        name: String::from("s"),
        is_mut: false,
        is_ref: true,
        lifetime_trait: LifetimeTrait::Move,
    });
    // Functions
    let calculate_length = ResourceOwner::Function(Function {
        hash: 3,
        name: String::from("calculate_length"),
    });
    let string_ctor = ResourceOwner::Function(Function {
        hash: 5,
        name: String::from("String::from"),
    });
    // Data
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    
    // s1 gets resource from String::from
    vd.append_external_event(ExternalEvent::Move{from: Some(string_ctor.clone()), to: Some(s1.clone())}, &(2 as usize));

    // move out as return value, does not render because to=None
    vd.append_external_event(ExternalEvent::StaticBorrow{from: Some(s1.clone()), to: Some(calculate_length.clone())}, &(4 as usize));
    vd.append_external_event(ExternalEvent::StaticReturn{from: Some(calculate_length.clone()), to: Some(s1.clone())}, &(4 as usize));

    vd.append_external_event(ExternalEvent::StaticBorrow{from: Some(calculate_length.clone()), to: Some(s.clone())}, &(9 as usize));
    vd.append_external_event(ExternalEvent::Duplicate{from: Some(s.clone()), to: None}, &(10 as usize));
    vd.append_external_event(ExternalEvent::StaticReturn{from: Some(s.clone()), to: Some(calculate_length.clone())}, &(11 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : s.clone() },  &(11 as usize));

    vd.append_external_event(ExternalEvent::Duplicate{from: Some(calculate_length.clone()), to: Some(len.clone())}, &(4 as usize));

    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : s1.clone() },  &(7 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : len.clone() },  &(7 as usize));
    
    //rendering image
    svg_generation::render_svg(&"book_04_02_01".to_owned(), &vd);
}
