use rrt_lib::data::{ExternalEvent, LifetimeTrait, ResourceAccessPoint, Owner, MutRef, StaticRef, Function, VisualizationData, Visualizable};
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
    let r1 = ResourceAccessPoint::StaticRef(StaticRef {
        hash: 2,
        my_owner_hash: Some(1),
        name: String::from("r1"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::None,
    });
    let r2 = ResourceAccessPoint::StaticRef(StaticRef {
        hash: 3,
        my_owner_hash: Some(1),
        name: String::from("r2"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::None,
    });
    let r3 = ResourceAccessPoint::MutRef(MutRef {
        hash: 4,
        my_owner_hash: Some(1),
        name: String::from("r3"),
        is_mut: false,
        lifetime_trait: LifetimeTrait::None,
    });
    let string_ctor = Some(ResourceAccessPoint::Function(Function {
        hash: 5,
        name: String::from("String::from()"),
    }));

    let print_func = Some(ResourceAccessPoint::Function(Function {
        hash: 6,
        name: String::from("println!()"),
    }));
    let mut vd = VisualizationData {
        timelines: BTreeMap::new(),
        external_events: Vec::new(),
    };
    //
    // hash s : 1
    //      r1 : 2
    //      r2 : 3
    //      r3 : 4
    // functions: 1
    // Event 1: give s the resource and make it the owner
    vd.append_external_event(ExternalEvent::Move{from: string_ctor.clone(), to: Some(s.clone())}, &(2 as usize));
    // Mark event: "r1" borrows immutable reference from "s"
    // Events 2-3: lend s resource to r1 and r1 borrow resource from s
    vd.append_external_event(ExternalEvent::StaticBorrow{from: Some(s.clone()), to: Some(r1.clone())}, &(4 as usize));
    // Events 4-5: lend s resource to r2 and r2 borrow resource from s
    vd.append_external_event(ExternalEvent::StaticBorrow{from: Some(s.clone()), to: Some(r2.clone())}, &(5 as usize));
    // Event 6-8: r1 and r2 return resource priviledges to s
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(r1.clone()), to: print_func.clone()}, &(6 as usize));
    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(r2.clone()), to: print_func.clone()}, &(6 as usize));
    vd.append_external_event(ExternalEvent::StaticReturn{from: Some(r1.clone()), to: Some(s.clone())}, &(6 as usize));
    vd.append_external_event(ExternalEvent::StaticReturn{from: Some(r2.clone()), to: Some(s.clone())}, &(6 as usize));
    // Events 9-10: mutable lend s resource to r3 and r3 borrow resource from s
    vd.append_external_event(ExternalEvent::MutableBorrow{from: Some(s.clone()), to: Some(r3.clone())}, &(9 as usize));

    vd.append_external_event(ExternalEvent::PassByStaticReference{from: Some(r3.clone()), to: print_func.clone()}, &(10 as usize));
    // Event 11-12: r3 return resource priviledges to s
    vd.append_external_event(ExternalEvent::MutableReturn{from: Some(r3.clone()), to: Some(s.clone())}, &(10 as usize));
    //all variables go out of scope at end of function
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : s.clone() },  &(12 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : r1.clone() },  &(12 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : r2.clone() },  &(12 as usize));
    vd.append_external_event(ExternalEvent::GoOutOfScope{ ro : r3.clone() },  &(12 as usize));

    //rendering image
    svg_generation::render_svg(&"04_02_09".to_owned(), &"shared_and_unique_borrow".to_owned(), &vd);
}
