use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceOwner, Variable, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;

fn main() {
    // Variables
    let s = Some(ResourceOwner::Variable(Variable {
        hash: 5,
        name: String::from("s"),
        is_mut: false,
        is_ref: true,
        lifetime_trait: LifetimeTrait::Move,
    }));
    // Functions
    let len_func = Some(ResourceOwner::Function(Function {
        hash: 2,
        name: String::from("len()"),
    }));
    let calculate_length = Some(ResourceOwner::Function(Function {
        hash: 3,
        name: String::from("calculate_length()"),
    }));

    // Data
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    
    // s is a reference copied from god knows where
    vd.append_external_event(ExternalEvent::Duplicate{ from: None, to: s.clone() }, &(2 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: s.clone(), to: len_func.clone()}, &(3 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : s.unwrap().clone() },  &(4 as usize));

    //rendering image
    svg_generation::render_svg(&"04_02_03".to_owned(), &"safely_out_of_scope".to_owned(), &vd);
}
