use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, VisualizationData, Visualizable};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let s = ResourceOwner::Variable(Variable {
        hash: 1,
        name: String::from("s"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::None
    });
    let from_func = ResourceOwner::Function(Function {
        hash: 2,
        name: String::from("String::from()"),
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    //
    // hash s : 1
    //
    vd.append_external_event(ExternalEvent::Move{from: Some(from_func.to_owned()), to: Some(s.clone())}, &(2 as usize));

    svg_generation::render_svg(&"04_01_04".to_owned(), &"reference".to_owned(), &vd);
}
