use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, Function, VisualizationData, Visualizable};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let x = ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("x"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::Copy
    });
    let y = ResourceAccessPoint::Owner(Owner {
        hash: 2,
        name: String::from("y"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::None
    });

    let println_func = ResourceAccessPoint::Function(Function {
        hash: 3,
        name: String::from("println!()"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };

    vd.append_external_event(ExternalEvent::Duplicate{from: None, to: Some(x.clone())}, &(2 as usize));
    vd.append_external_event(ExternalEvent::Duplicate{from: Some(x.clone()), to: Some(y.clone())}, &(3 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(x.clone()), to: Some(println_func.clone())}, &(5 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(y.clone()), to: Some(println_func.clone())}, &(5 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: y.clone() }, &(6 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: x.clone() }, &(6 as usize));

    svg_generation::render_svg(&"04_01_10".to_owned(), &"int_copy_2".to_owned(), &vd);
}
