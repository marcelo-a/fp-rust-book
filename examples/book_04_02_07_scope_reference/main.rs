use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, MutRef, Function, Visualizable, VisualizationData};
use rrt_lib::svg_frontend::svg_generation;
use std::collections::BTreeMap;


// visualization of static_borrow/example.rs
// taken from The Book chapter 4.2
// variable oriented: display stats on variable, not the data stored in it
fn main() {
    
    let s = ResourceAccessPoint::Owner(Owner {
        hash: 1,
        name: String::from("s"),
        is_mut: true,
        lifetime_trait: LifetimeTrait::Move,
    });
    let r1 = ResourceAccessPoint::MutRef(MutRef {
        hash: 2,
        name: String::from("r1"),
        is_mut: false,
        my_owner_hash: Some(1),
        lifetime_trait: LifetimeTrait::None,
    });
    let r2 = ResourceAccessPoint::MutRef(MutRef {
        hash: 3,
        name: String::from("r2"),
        is_mut: false,
        my_owner_hash: Some(1),
        lifetime_trait: LifetimeTrait::None,
    });
    let string_ctor = Some(ResourceAccessPoint::Function(Function {
        hash: 4,
        name: String::from("String::from()"),
    }));
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    vd.append_external_event(ExternalEvent::Move{from: string_ctor.clone(), to: Some(s.clone())}, &(2 as usize));
  
    vd.append_external_event(ExternalEvent::MutableBorrow{from: Some(s.clone()), to: Some(r1.clone())}, &(5 as usize));
    vd.append_external_event(ExternalEvent::MutableReturn{from: Some(r1.clone()), to: Some(s.clone())}, &(6 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : r1.clone() },  &(6 as usize));

    vd.append_external_event(ExternalEvent::MutableBorrow{from: Some(s.clone()), to: Some(r2.clone())}, &(9 as usize));

    //rendering image
    svg_generation::render_svg(&"04_02_07".to_owned(), &"scope_reference".to_owned(), &vd);
}
