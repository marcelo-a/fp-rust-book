use std::collections::{HashSet, BTreeMap};
use std::vec::Vec;
/** 
 * Basic Data Structure Needed by Lifetime Visualization
 */


// Top level Api that the Timeline object supports
pub trait Visualizable {
    // returns None if the hash does not exist
    fn get_name_from_hash(&self, hash: &u64) -> Option<String>;
    // returns Noneappend_event if the hash does not exist
    fn get_state(&self, hash: &u64, line_number: &usize) -> Option<State>;
    // add an event to the Visualizable data structure
    fn append_event(&mut self, resource_owner: &ResourceOwner, event: Event, line_number: &usize);
    // add an ExternalEvent to the Visualizable data structure
    fn append_external_event(&mut self, line_number: usize, external_event: ExternalEvent); 

    // if resource_owner with hash is mutable
    fn is_mut(&self, hash: &u64 ) -> bool;

    fn calc_state(&self, previous_state: & State, event: & Event, event_line: usize, hash: &u64) -> State;

    fn get_states(&self, hash: &u64) -> Vec::<(usize, usize, State)>;
}

// A ResourceOwner is either a Variable or a Function that 
// have ownership to a memory object, during some stage of
// a the program execution.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ResourceOwner {
    pub hash: u64,
    pub name: String,
    // whether the variable itself is mutable
    pub is_mut: bool,
    pub is_ref: bool,
    // pub is_fun: bool,
    pub lifetime_trait: LifetimeTrait,
}


pub enum ExternalEvent{
    // let binding
    Acquire{
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    // copy / clone
    Duplicate{
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    Move{
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    Borrow{
        is_mut: bool,
        from: Option<ResourceOwner>,
        to: Option<ResourceOwner>,
    },
    Return{
        // return the resource to "to"
        is_mut: bool,
        from: Option<ResourceOwner>, 
        to: Option<ResourceOwner>,
    },
    GoOutOfScope,
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
    MutableBorrow{
        from: ResourceOwner
    },
    MutableReturn{
        to: Option<ResourceOwner>
    },
    MutableReacquire {
        from: Option<ResourceOwner>
    },
    StaticLend {
        to: Option<ResourceOwner>
    },
    StaticBorrow{
        from: ResourceOwner
    },
    StaticReturn{
        to: Option<ResourceOwner>
    },
    StaticReacquire {
        from: Option<ResourceOwner>
    },
    // this happens when a variable is returned this line,
    // or if this variable's scope ends at this line.
    GoOutOfScope,
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
    ResourceReturned {
        to: Option<ResourceOwner>,
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            State::OutOfScope => write!(f, "OutOfScope"),
            State::ResourceMoved { move_to: _, move_at_line: _ } => write!(f, "ResourceMoved"),
            State::FullPrivilege => write!(f, "FullPrivilege"),
            State::PartialPrivilege { borrow_count: _, borrow_to: _ } => write!(f, "PartialPrivilege"),
            State::RevokedPrivilege { to: _, borrow_to: _ } => write!(f, "RevokedPrivilege"),
            State::Invalid => write!(f, "Invalid"),
            State::ResourceReturned { to: _ } => write!(f, "Resource Returned"),
        }
    }
}

// a vector of ownership transfer history of a specific variable, 
// in a sorted order by line number.
pub struct Timeline {
    pub resource_owner: ResourceOwner,  
    // line number in usize
    pub history: Vec<(usize, Event)>,
}

// VisualizationData supplies all the information we need in the frontend, 
// from rendering a PNG to px roducing an interactive HTML guide. 
// The internal data is simple: a map from variable hash to its Timeline.
pub struct VisualizationData {
    pub timelines: BTreeMap<u64, Timeline>,
    // line_number, event
    // for svg arrows
    pub external_events: Vec<(usize, ExternalEvent)>,
}

// fulfills the promise that we can support all the methods that a 
// frontend would need. 
impl Visualizable for VisualizationData {
    fn get_name_from_hash(&self, hash: &u64) -> Option<String> {
        match self.timelines.get(hash) {
            Some(timeline) => Some(timeline.resource_owner.name.to_owned()),
            _ => None
        }
    }

    // if the ResourceOwner is declared mutable
    fn is_mut(&self, hash: &u64 ) -> bool {
        self.timelines[hash].resource_owner.is_mut
    }

    fn calc_state(&self, previous_state: & State, event: & Event, event_line: usize, hash: &u64) -> State {
        match (previous_state, event) {
            (State::Invalid, _) => State::Invalid,

            (State::OutOfScope, Event::Acquire{ from: _ })  => State::FullPrivilege,

            // (State::OutOfScope, Event::StaticBorrow{ from: ro })  => State::PartialPrivilege{  },
            (State::OutOfScope, Event::StaticBorrow{ from: _ }) => State::Invalid,

            (State::FullPrivilege, Event::Move{to: to_ro}) => State::ResourceMoved{move_to: to_ro.to_owned(), move_at_line: event_line},
            
            (State::FullPrivilege, Event::MutableLend{ to: to_ro }) => 
                if self.is_mut(hash) { State::FullPrivilege } else { State::Invalid },
            
            (State::FullPrivilege, Event::StaticLend{to: to_ro}) => 
                State::PartialPrivilege {
                    borrow_count: 1,
                    borrow_to: [(to_ro.to_owned().unwrap())].iter().cloned().collect() // we assume there is no borrow_to:None
                },
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

    // TODO: state solving not complete
    fn get_state(&self, hash: &u64, line_number: &usize) -> Option<State> {
        match self.timelines.get(hash) {
            Some(timeline) => {
                // example return value
                Some(State::OutOfScope)
            },
            _ => None
        }
    }


    // add event using (internal) events
    fn append_event(&mut self, resource_owner: &ResourceOwner, event: Event, line_number: &usize) {
        let hash = &resource_owner.hash;
        // if this event belongs to a new ResourceOwner hash,
        // create a new Timeline first, thenResourceOwner bind it to the corresponding hash.
        match self.timelines.get(hash) {
            None => {
                let timeline = Timeline {
                    resource_owner: resource_owner.clone(),
                    history: Vec::new(),
                };
                self.timelines.insert(*hash, timeline);
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

    // TODO IMPLEMENT
    fn append_external_event(&mut self, line_number: usize, external_event: ExternalEvent){
        self.external_events.push(
            (line_number, external_event)
        );
    } 
}
