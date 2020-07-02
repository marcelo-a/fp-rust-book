use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    // Variables
    let s1 = Some(ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("s1"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Move,
    }));
    let len = Some(ResourceAccessPoint::Owner(Owner {
        hash: 2,
        name: String::from("len"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Copy,
    }));
    // Functions
    let calculate_length = Some(ResourceAccessPoint::Function(Function {
        hash: 3,
        name: String::from("calculate_length()"),
    }));
    let string_ctor = Some(ResourceAccessPoint::Function(Function {
        hash: 6,
        name: String::from("String::from()"),
    }));
    // Data
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    
    // s1 gets resource from String::from
    vd.append_external_event(ExternalEvent::Move{from: string_ctor.clone(), to: s1.clone()}, &(2 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: s1.clone(), to: calculate_length.clone()}, &(4 as usize));

    vd.append_external_event(ExternalEvent::Duplicate{from: calculate_length.clone(), to: len.clone()}, &(4 as usize));

    // vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : s1.unwrap().clone() },  &(4 as usize));
    // vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : len.unwrap().clone() },  &(4 as usize));    
    
    //rendering image
    svg_generation::render_svg(&"04_02_02".to_owned(), &"acquire_from_function".to_owned(), &vd);
}
