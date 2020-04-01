use std::collections::{HashSet, BTreeMap};
use std::vec::Vec;
use std::fmt::{Formatter, Result, Display};
use crate::data::Event::*;
use crate::hover_messages;
/**
 * Basic Data Structure Needed by Lifetime Visualization
 */


// Top level Api that the Timeline object supports
pub trait Visualizable {
    // returns None if the hash does not exist
    fn get_name_from_hash(&self, hash: &u64) -> Option<String>;
    
    // returns None if the hash does not exist
    fn get_state(&self, hash: &u64, line_number: &usize) -> Option<State>;
    
    // for querying states of a resource owner using its hash
    //                                         start line, end line, state
    fn get_states(&self, hash: &u64) -> Vec::<(usize,      usize,    State)>;

    // WARNING do not call this when making visualization!! 
    // use append_external_event instead
    fn _append_event(&mut self, resource_owner: &ResourceOwner, event: Event, line_number: &usize);
    
    // add an event to the Visualizable data structure
    fn append_external_event(&mut self, event: ExternalEvent, line_number: &usize) ;
    
    // if resource_owner with hash is mutable
    fn is_mut(&self, hash: &u64 ) -> bool;

    fn calc_state(&self, previous_state: & State, event: & Event, event_line: usize, hash: &u64) -> State;
}

// A ResourceOwner is either a Variable or a Function that
// have ownership to a memory object, during some stage of
// a the program execution.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Function {
    pub hash: u64,
    pub name: String,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum RefKind {
    MutRef, // a mutable reference, eg let a = &mut b
    StaticRef, // a static reference, eg let a = & b
    NotRef // not a reference
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Variable {
    pub hash: u64,
    pub name: String,
    // whether the variable itself is mutable
    pub is_mut: bool,
    // Some(hash_of_original) or None, if it's not a reference; NOTE that this is different from is_mut
    pub ref_kind: RefKind,
    pub lifetime_trait: LifetimeTrait,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum ResourceOwner {
    Variable(Variable),
    Function(Function),
}

impl ResourceOwner {
    // get the attribute hash
    pub fn hash(&self) -> &u64 {
        match self {
            ResourceOwner::Variable(Variable{hash, ..}) => hash,
            ResourceOwner::Function(Function{hash, ..}) => hash,
        }
    }

    // get the name field
    pub fn name(&self) -> &String {
        match self {
            ResourceOwner::Variable(Variable{name, ..}) => name,
            ResourceOwner::Function(Function{name, ..}) => name,
        }
    }

    // get the is_mut field, if any
    pub fn is_mut(&self) -> bool {
        match self {
            ResourceOwner::Variable(Variable{is_mut, ..}) => is_mut.to_owned(),
            ResourceOwner::Function(_) => false,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            ResourceOwner::Variable(Variable{ref_kind: RefKind::MutRef, ..}) => true,
            ResourceOwner::Variable(Variable{ref_kind: RefKind::StaticRef, ..}) => true,
            _ => false,
        }
    }

    pub fn ref_kind(&self) -> Option<RefKind> {
        match self {
            ResourceOwner::Variable(Variable{ref_kind: rk, ..}) => Some(rk.to_owned()),
            _ => None,
        }
    }
}

// impl std::string::ToString for ResourceOwner {
//     fn to_string(&self) -> String {
//         self.name.to_owned()
//     }
// }

/* let binding is either Duplicate (let _ = 1;)
or move (let a = String::from("");) */
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum ExternalEvent{
    // copy / clone
    Duplicate {
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    Move {
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    StaticBorrow {
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    MutableBorrow {
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    StaticReturn {
        // return the resource to "to"
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    MutableReturn {
        // return the resource to "to"
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    // a use of the variable, happens when var pass by reference
    // its really borrow and return but happens on the same line,
    // use this event instead of borrow and return for more concise visualization 
    PassByStaticReference {
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>, // must be a function
    },
    PassByMutableReference {
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>, // must be a function
    },
    GoOutOfScope {
        ro: ResourceOwner
    },
}


// ASSUMPTION: a reference must return resource before borrow;
//
// An Event describes the acquisition or release of a
// resource ownership by a variable on any given line.
// There are six types of them.
pub enum Event {
    // this happens when a variable is initiated, it should obtain
    // its resource from either another variable or from a
    // contructor.
    //
    // E.g. in the case of
    //      let x = Vec::new();
    // x obtained the resource from global resource allocator,
    // the Acquire Event's "from" variable is None.
    // in the case of
    //      let y = x;
    // y obtained its value from x, which means that the Acquire
    // Event's "from" variable is x.
    // TODO do we need mut/static_acquire for get_state?
    Acquire {
        from: Option<ResourceOwner>
    },
    // this happens when a ResourceOwner implements copy trait or
    // explicitly calls .clone() function
    // to another ResourceOwner, or a function.
    //
    // e.g.
    // 1. x: i32 = y + 15;              here y duplicate to + op, and x acquire from +
    //                                  at this point, we treat it as y duplicates to None
    // 2. x: MyStruct = y.clone();      here y duplicates to x.
    Duplicate {
        to: Option<ResourceOwner>
    },
    // this happens when a ResourceOwner transfer the ownership of its resource
    // to another ResourceOwner, or if it is no longer used after this line.
    // Typically, this happens at one of the following two cases:
    //
    // 1. variable is not used after this line.
    // 2. variable's resource has the move trait, and it transfered
    //    its ownership on this line. This includes tranfering its
    //    ownership to a function as well.
    Move {
        to: Option<ResourceOwner>
    },
    MutableLend {
        to: Option<ResourceOwner>
    },
    MutableBorrow {
        from: ResourceOwner
    },
    MutableReturn {
        to: Option<ResourceOwner>
    },
    MutableReacquire {
        from: Option<ResourceOwner>
    },
    StaticLend {
        to: Option<ResourceOwner>
    },
    StaticBorrow {
        from: ResourceOwner
    },
    StaticReturn {
        to: Option<ResourceOwner>
    },
    StaticReacquire {
        from: Option<ResourceOwner>
    },
    // this happens when a owner is returned this line,
    // or if this owner's scope ends at this line. The data must be dropped. 
    OwnerGoOutOfScope,
    // this happens when a vairable that is not an owner goes out of scope. 
    // The data is not dropped in this case
    RefGoOutOfScope

}

// Trait of a resource owner that might impact the way lifetime visualization
// behaves
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum LifetimeTrait {
    Copy,
    Move,
    None,
}

// A State is a description of a ResourceOwner IMMEDIATELY AFTER a specific line.
// We think of this as what read/write access we have to its resource.
#[derive(Clone)]
pub enum State {
    // The viable is no longer in the scope after this line.
    OutOfScope,
    // The resource is transferred on this line or before this line,
    // thus it is impossible to access this variable anymore.
    ResourceMoved {
        move_to: Option<ResourceOwner>,
        move_at_line: usize
    },
    // This ResourceOwner is the unique object that holds the ownership to the underlying resource.
    FullPrivilege,
    // More than one ResourceOwner has access to the underlying resource
    // This means that it is not possible to create a mutable reference
    // on the next line.
    // About borrow_count: this value is at least one at any time.
    //      When the first static reference of this ResourceOwner is created,
    //          this value is set to 1;
    //      When a new static reference is borrowed from this variable, increment by 1;
    //      When a static reference goes out of scope, decrement this value by 1;
    //      When a decrement happens while the borrow_count is 1, the state becomes
    //          FullPrivilege once again.
    PartialPrivilege {
        borrow_count: u32,
        borrow_to: HashSet<ResourceOwner>
    },
    // temporarily no read or write access right to the resource, but eventually
    // the privilege will come back. Occurs when mutably borrowed
    RevokedPrivilege {
        to: Option<ResourceOwner>,
        borrow_to: Option<ResourceOwner>,
    },
    // should not appear for visualization in a correct program
    Invalid,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result {
        match self {
            State::OutOfScope => write!(f, "OutOfScope"),
            State::ResourceMoved { move_to: _, move_at_line: _ } => write!(f, "ResourceMoved"),
            State::FullPrivilege => write!(f, "FullPrivilege"),
            State::PartialPrivilege { borrow_count: _, borrow_to: _ } => write!(f, "PartialPrivilege"),
            State::RevokedPrivilege { to: _, borrow_to: _ } => write!(f, "RevokedPrivilege"),
            State::Invalid => write!(f, "Invalid"),
        }
    }
}


fn safe_message(
    message_functor: fn(&String, &String) -> String,
    my_name: &String,
    some_target: &Option<ResourceOwner>
) -> String {
    if let Some(target) = some_target {
        message_functor(my_name, target.name())
    }
    else {
        message_functor(my_name, &"another value".to_owned())
    }
}


impl State {
    pub fn print_message_with_name(&self, my_name: &String) -> String {
        match self {
            State::OutOfScope => {
                hover_messages::state_out_of_scope(my_name)
            }
            State::ResourceMoved{ move_to , move_at_line: _ } => {
                safe_message(hover_messages::state_resource_moved, my_name, move_to)
            }
            State::FullPrivilege => {
                hover_messages::state_full_priviledge(my_name)
            }
            State::PartialPrivilege { borrow_count: _, borrow_to: _ } => {
                hover_messages::state_partial_priviledge(my_name)
            }
            State::RevokedPrivilege { to: _, borrow_to } => {
                safe_message(hover_messages::state_resource_revoked, my_name, borrow_to)
            }
            State::Invalid => {
                hover_messages::state_invalid(my_name)
            }
        }
    }
}


// provide string output for usages like format!("{}", eventA)
impl Display for Event {
    fn fmt(&self, f: &mut Formatter) -> Result {       
        let mut from_ro = None;
        let mut to_ro = None;
        let mut display = match self {
            Event::Acquire{ from } => { from_ro = from.to_owned(); "" },
            Event::Duplicate{ to } => { to_ro = to.to_owned(); "Copying resource" },
            Event::Move{ to } => { to_ro = to.to_owned(); "Moving resource" },
            Event::MutableLend{ to } => { to_ro = to.to_owned(); "Mutable lend" },
            Event::MutableBorrow{ from } => { from_ro = Some(from.to_owned()); "Fully borrows resource" },
            Event::MutableReturn{ to } => { to_ro = to.to_owned(); "Fully returns resource"},
            Event::MutableReacquire{ from } => { from_ro = from.to_owned(); "Fully reacquires resource" },
            Event::StaticLend{ to } => { to_ro = to.to_owned(); "Partially lends resource" },
            Event::StaticBorrow{ from } => { from_ro = Some(from.to_owned()); "Partially borrows resource" },
            Event::StaticReturn{ to } => { to_ro = to.to_owned(); "Partially returns resource"},
            Event::StaticReacquire{ from } => { from_ro = from.to_owned(); "Partially reacquires resource" },
            Event::OwnerGoOutOfScope => { "Goes out of Scope as an owner of resource" },
            Event::RefGoOutOfScope => { "Goes out of Scope as an reference to resource" },
        }.to_string();

        if let Some(from_ro) = from_ro {
            display = format!("{} from {}", display, &(from_ro.name()));
        };
        if let Some(to_ro) = to_ro {
            display = format!("{} to {}", display, &(to_ro.name()));
        };
        write!(f, "{}", display)
    }
}


impl Event {
    pub fn print_message_with_name(&self, my_name: &String) -> String {
        match self {
            // no arrow involved
            OwnerGoOutOfScope => { 
                hover_messages::event_dot_owner_go_out_out_scope(my_name)
            }
            RefGoOutOfScope => {
                hover_messages::event_dot_ref_go_out_out_scope(my_name)
            }
            // arrow going out
            Duplicate{ to } => {
                safe_message(hover_messages::event_dot_copy_to, my_name, to)
            }
            Move{ to } => {
                safe_message(hover_messages::event_dot_move_to, my_name, to)
            }
            StaticLend{ to } => {
                safe_message(hover_messages::event_dot_static_lend, my_name, to)
            }
            MutableLend{ to } => {
                safe_message(hover_messages::event_dot_mut_lend, my_name, to)
            }
            StaticReturn{ to } => {
                safe_message(hover_messages::event_dot_static_return, my_name, to)
            }
            MutableReturn{ to } => {
                safe_message(hover_messages::event_dot_mut_return, my_name, to)
            }
            // arrow going in
            Acquire{ from } => {
                safe_message(hover_messages::event_dot_acquire, my_name, from)
            }
            MutableBorrow{ from } => {
                hover_messages::event_dot_mut_borrow(my_name, from.name())
            }
            StaticBorrow{ from } => {
                hover_messages::event_dot_static_borrow(my_name, from.name())
            }
            StaticReacquire{ from } => {
                safe_message(hover_messages::event_dot_static_reacquire, my_name, from)
            }
            MutableReacquire{ from } => {
                safe_message(hover_messages::event_dot_mut_reacquire, my_name, from)
            }
        }
    }
}

// a vector of ownership transfer history of a specific variable,
// in a sorted order by line number.
pub struct Timeline {
    pub resource_owner: ResourceOwner,    // a reference of a Variable or a (TODO) Reference, 
                                // since Functions don't have a timeline 
    // line number in usize
    pub history: Vec<(usize, Event)>,
}

// VisualizationData supplies all the information we need in the frontend,
// from rendering a PNG to px roducing an interactive HTML guide.
// The internal data is simple: a map from variable hash to its Timeline.
pub struct VisualizationData {
    // When displaying all timelines in the frontend of choice, one should
    // consider picking a hash function that gives the BTreeMap a sensible order.
    //      timelines: an orderred map from a Variable's hash to 
    //      the Variable's Timeline.
    pub timelines: BTreeMap<u64, Timeline>,
    
    pub external_events: Vec<(usize, ExternalEvent)>,
}

// fulfills the promise that we can support all the methods that a
// frontend would need.
impl Visualizable for VisualizationData {
    fn get_name_from_hash(&self, hash: &u64) -> Option<String> {
        match self.timelines.get(hash) {
            Some(timeline) => Some(timeline.resource_owner.name().to_owned()),
            _ => None
        }
    }

    // if the ResourceOwner is declared mutable
    fn is_mut(&self, hash: &u64 ) -> bool {
        self.timelines[hash].resource_owner.is_mut()
    }


    
    // a Function does not have a State, so we assume previous_state is always for Variables
    fn calc_state(&self, previous_state: & State, event: & Event, event_line: usize, hash: &u64) -> State {
        /* a Variable cannot borrow or return resource from Functions, 
        but can 'lend' or 'reaquire' to Functions (pass itself by reference and take it back); */
        fn event_invalid(event: & Event) -> bool {
            match event {
                Event::StaticBorrow{ from: ResourceOwner::Function(_) } => true,
                Event::MutableBorrow{ from: ResourceOwner::Function(_) } => true,
                Event::StaticReturn{ to: Some(ResourceOwner::Function(_)) } => true,
                Event::MutableReturn{ to: Some(ResourceOwner::Function(_)) } => true,
                _ => false,
            }
        }
        if event_invalid(event) { return State::Invalid; }

        match (previous_state, event) {
            (State::Invalid, _) => State::Invalid,

            (State::OutOfScope, Event::Acquire{ from: _ })  => State::FullPrivilege,

            (State::OutOfScope, Event::StaticBorrow{ from: ro })  =>
                State::PartialPrivilege {
                    borrow_count: 1,
                    borrow_to: [ro.to_owned()].iter().cloned().collect()
                },

            (State::OutOfScope, Event::MutableBorrow{ from: ro }) =>
                State::FullPrivilege,

            (State::FullPrivilege, Event::Move{to: to_ro}) => State::ResourceMoved{ move_to: to_ro.to_owned(), move_at_line: event_line },

            (State::FullPrivilege, Event::MutableLend{ to: to_ro }) =>
                if self.is_mut(hash) { State::RevokedPrivilege{ to: None, borrow_to: to_ro.to_owned() } } else { State::Invalid },
            
            // happends when a mutable reference returns, invalid otherwise
            (State::FullPrivilege, Event::MutableReturn{ to: to_ro }) =>
                State::OutOfScope,
        
            // this looks impossible?
            // (State::FullPrivilege, Event::MutableReacquire{ from: ro }) =>
            //     State::ResourceReturned { to: ro.to_owned() },

            // (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::MutableReacquire{ from: ro }) =>
            //         State::ResourceReturned { to: ro.to_owned() },

            (State::FullPrivilege, Event::StaticLend{ to: to_ro }) =>
                State::PartialPrivilege {
                    borrow_count: 1,
                    borrow_to: [(to_ro.to_owned().unwrap())].iter().cloned().collect() // we assume there is no borrow_to:None
                },

            (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::MutableLend{ to: to_ro }) => State::Invalid,

            (State::PartialPrivilege{ borrow_count: current, borrow_to: borrow_to }, Event::StaticLend{ to: to_ro }) => {
                let mut new_borrow_to = borrow_to.clone();
                // Assume can not lend to None
                new_borrow_to.insert(to_ro.to_owned().unwrap());
                State::PartialPrivilege {
                    borrow_count: current+1,
                    borrow_to: new_borrow_to,
                }
            }
                

            // self statically borrowed resource, and it returns; TODO what about references to self?
            (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::StaticReturn{ to: to_ro }) => State::OutOfScope,

            (State::PartialPrivilege{ borrow_count, borrow_to }, Event::StaticReacquire{ from: ro }) => {
                let new_borrow_count = borrow_count - 1;
                // check if it resumes to full privilege    
                if borrow_count - 1 == 0 {
                        State::FullPrivilege 
                    } else {
                        let mut new_borrow_to = borrow_to.clone();
                        // TODO ro.unwrap() should not panic, because Reacquire{from: None} is not possible
                        // TODO change to Reaquire{from: ResourceOwner}
                        assert_eq!(new_borrow_to.remove(&ro.to_owned().unwrap()), true); // borrow_to must contain ro
                        State::PartialPrivilege{
                            borrow_count: new_borrow_count,
                            borrow_to: new_borrow_to,
                        }
                    }
                }

            (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::OwnerGoOutOfScope) => State::OutOfScope,

            (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::RefGoOutOfScope) => State::OutOfScope,

            (State::RevokedPrivilege{ to: _, borrow_to: _ }, Event::MutableReacquire{ from: ro }) => State::FullPrivilege,

            (_, Event::Duplicate { to: ro }) =>
                (*previous_state).clone(),

             // State::FullPrivilege,

            (_, _) => State::Invalid,
        }
    }

    fn get_states(&self, hash: &u64) -> Vec::<(usize, usize, State)> {
        let mut states = Vec::<(usize, usize, State)>::new();
        let mut previous_line_number: usize = 1;
        let mut prev_state = State::OutOfScope;
        for (line_number, event) in self.timelines[hash].history.iter() {
            states.push(
                (previous_line_number, *line_number, prev_state.clone())
            );
            prev_state = self.calc_state(&prev_state, &event, *line_number, hash);
            previous_line_number = *line_number;
        }
        states
    }

    fn get_state(&self, hash: &u64, line_number: &usize) -> Option<State> {
        match self.timelines.get(hash) {
            Some(timeline) => {
                // example return value
                Some(State::OutOfScope)
            },
            _ => None
        }
    }


    // WARNING do not call this when making visualization!! 
    // use append_external_event instead
     fn _append_event(&mut self, resource_owner: &ResourceOwner, event: Event, line_number: &usize) {
        let hash = &resource_owner.hash();
        // if this event belongs to a new ResourceOwner hash,
        // create a new Timeline first, thenResourceOwner bind it to the corresponding hash.
        match self.timelines.get(hash) {
            None => {
                let timeline = Timeline {
                    resource_owner: resource_owner.clone(),
                    history: Vec::new(),
                };
                self.timelines.insert(**hash, timeline);
            },
            _ => {}
        }

        // append the event to the end of the timeline of the corresponding hash
        match self.timelines.get_mut(hash) {
            Some(timeline) => {
                timeline.history.push(
                    (*line_number, event)
                );
            },
            _ => {
                panic!("Timeline disappeared right after creation or when we could index it. This is impossible.");
            }
        }
    }


    // store them in external_events, and call append_events
    // default way to record events
    fn append_external_event(&mut self, event: ExternalEvent, line_number: &usize) {
        self.external_events.push((*line_number, event.clone()));
        
        // append_event if resource_owner is not null
        fn maybe_append_event(vd: &mut VisualizationData, resource_owner: &Option<ResourceOwner>, event: Event, line_number : &usize) {
            if let Some(ro) = resource_owner {
                vd._append_event(&ro, event, line_number)
            };
        }

        match event {
            // eg let ro_to = String::from("");
            ExternalEvent::Move{from: from_ro, to: to_ro} => {
                maybe_append_event(self, &to_ro, Event::Acquire{from : from_ro.to_owned()}, line_number);
                maybe_append_event(self, &from_ro, Event::Move{to : to_ro.to_owned()}, line_number);
            }
            // eg let ro_to = 5;
            ExternalEvent::Duplicate{from: from_ro, to: to_ro} => {
                maybe_append_event(self, &to_ro, Event::Acquire{from : from_ro.to_owned()}, line_number);
                maybe_append_event(self, &from_ro, Event::Duplicate{to : to_ro.to_owned()}, line_number);
            }
            ExternalEvent::StaticBorrow{from: from_ro, to: to_ro} => {
                maybe_append_event(self, &from_ro, Event::StaticLend{to : to_ro.to_owned()}, line_number);
                if let Some(some_from_ro) = from_ro {
                    maybe_append_event(self, &to_ro, Event::StaticBorrow{from : some_from_ro.to_owned()}, line_number);
                }
            }
            ExternalEvent::StaticReturn{from: from_ro, to: to_ro} => {
                maybe_append_event(self, &to_ro, Event::StaticReacquire{from : from_ro.to_owned()}, line_number);
                maybe_append_event(self, &from_ro, Event::StaticReturn{to : to_ro.to_owned()}, line_number);
            }
            ExternalEvent::MutableBorrow{from: from_ro, to: to_ro} => {
                maybe_append_event(self, &from_ro, Event::MutableLend{to : to_ro.to_owned()}, line_number);
                if let Some(some_from_ro) = from_ro {
                    maybe_append_event(self, &to_ro, Event::MutableBorrow{from : some_from_ro.to_owned()}, line_number);
                }
            }
            ExternalEvent::MutableReturn{from: from_ro, to: to_ro} => {
                maybe_append_event(self, &to_ro, Event::MutableReacquire{from : from_ro.to_owned()}, line_number);
                maybe_append_event(self, &from_ro, Event::MutableReturn{to : to_ro.to_owned()}, line_number);
            }
            // TODO do we really need to add these events, since pass by ref does not change the state?
            ExternalEvent::PassByStaticReference{from: from_ro, to: to_ro} => {
                maybe_append_event(self, &from_ro.to_owned(), Event::StaticLend{to : to_ro.to_owned()}, line_number);
                if let Some(some_from_ro) = from_ro.to_owned() {
                    maybe_append_event(self, &to_ro.to_owned(), Event::StaticBorrow{from : some_from_ro.to_owned()}, line_number);
                } else {
                    panic!("Must pass a function to PassByStaticReference.to!");
                }
                maybe_append_event(self, &from_ro, Event::StaticReacquire{from : to_ro.to_owned()}, line_number);
                maybe_append_event(self, &to_ro, Event::StaticReturn{to : from_ro.to_owned()}, line_number);
            }
            ExternalEvent::PassByMutableReference{from: from_ro, to: to_ro} => {
                maybe_append_event(self, &from_ro, Event::MutableLend{to : to_ro.to_owned()}, line_number);
                if let Some(some_from_ro) = from_ro.to_owned() {
                    maybe_append_event(self, &to_ro, Event::MutableBorrow{from : some_from_ro.to_owned()}, line_number);
                } else {
                    panic!("Must pass a function to PassByMutableReference.to!");
                }
                maybe_append_event(self, &from_ro, Event::MutableReacquire{from : to_ro.to_owned()}, line_number);
                maybe_append_event(self, &to_ro, Event::MutableReturn{to : from_ro.to_owned()}, line_number);
            }
            ExternalEvent::GoOutOfScope{ro} => {
                if ro.is_ref() {
                    maybe_append_event(self, &Some(ro), Event::RefGoOutOfScope, line_number);
                } else {
                    maybe_append_event(self, &Some(ro), Event::OwnerGoOutOfScope, line_number);
                }
            }
        }
    }
}

/* TODO use this function to create a single copy of resource owner in resource_owner_map,
 and use hash to refer to it */ 
// impl VisualizationData {
//     fn create_resource_owner(&mut self, ro: ResourceOwner) -> &ResourceOwner {
//         self.resource_owner_map.entry(ro.get_hash()).or_insert(ro);
//         self.resource_owner_map.get(ro.get_hash())
//     }
// }