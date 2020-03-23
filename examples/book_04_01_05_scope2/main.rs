use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, VisualizationData, Visualizable};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    let s = ResourceOwner::Variable(Variable {
        hash: 4,
        name: String::from("s"),
        is_mut: false,
        is_ref: false,
        lifetime_trait: LifetimeTrait::None
    });
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    //
    // hash s : 1
    //
    vd.append_external_event(ExternalEvent::Move{from: None, to: Some(s.clone())}, &(3 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro: s.clone() }, &(6 as usize));
    // same as 04_01_02
    svg_generation::render_svg(&"04_01_05".to_owned(), &"scope2".to_owned(), &vd);
}
