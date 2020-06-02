use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, MutRef, Owner, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    // Variables
    let s = Some(ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("s"),
        is_mut: true,
        lifetime_trait: LifetimeTrait::Move,
    }));
    let some_string = Some(ResourceAccessPoint::MutRef(MutRef {
        hash: 2,
        name: String::from("some_string"),
        is_mut: true,
        my_owner_hash: Some(1),
        lifetime_trait: LifetimeTrait::Move,
    }));
    let change = Some(ResourceAccessPoint::Function(Function {
        hash: 3,
        name: String::from("change()"),
    }));
    let string_from = Some(ResourceAccessPoint::Function(Function {
        hash: 4,
        name: String::from("String::from()"),
    }));
    let push_str = Some(ResourceAccessPoint::Function(Function {
        hash: 5,
        name: String::from("push_str()"),
    }));
    // Data
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    
    // s is a reference copied from god knows where
    vd.append_external_event(ExternalEvent::Duplicate{ from: string_from.clone(), to: s.clone() }, &(2 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{ from: s.clone(), to: change.clone() }, &(4 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : s.unwrap().clone() },  &(5 as usize));

    vd.append_external_event(ExternalEvent::Duplicate{ from: None, to: some_string.clone() }, &(7 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{ from: some_string.clone(), to: push_str.clone() }, &(8 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : some_string.unwrap().clone() },  &(9 as usize));

    //rendering image
    svg_generation::render_svg(&"04_02_05".to_owned(), &"mut_ref".to_owned(), &vd);
}
