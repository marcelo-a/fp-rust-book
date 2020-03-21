 use std::collections::{HashSet, BTreeMap, HashMap};
use std::vec::Vec;
/**
 * Basic Data Structure Needed by Lifetime Visualization
 */


// Top level Api that the Timeline object supports
pub trait Visualizable {
    // returns None if the hash does not exist
    fn get_name_from_hash(&self, hash: &u64) -> Option<String>;
    // returns None if the hash does not exist
    fn get_state(&self, hash: &u64, line_number: &usize) -> Option<State>;
    // add an event to the Visualizable data structure
    fn append_event(&mut self, resource_owner: &ResourceOwner, event: Event, line_number: &usize);
    // add an event to the Visualizable data structure
    fn append_external_event(&mut self, event: ExternalEvent, line_number: &usize) ;
    // if resource_owner with hash is mutable
    fn is_mut(&self, hash: &u64 ) -> bool;

    fn calc_state(&self, previous_state: & State, event: & Event, event_line: usize, hash: &u64) -> State;

    fn get_states(&self, hash: &u64) -> Vec::<(usize, usize, State)>;
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
pub struct Variable {
    pub hash: u64,
    pub name: String,
    // whether the variable itself is mutable
    pub is_mut: bool,
    pub is_ref: bool,
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

    // get the name filed
    pub fn name(&self) -> &String {
        match self {
            ResourceOwner::Variable(Variable{name, ..}) => name,
            ResourceOwner::Function(Function{name, ..}) => name,
        }
    }

    // get the is_mut filed, if any
    pub fn is_mut(&self) -> bool {
        match self {
            ResourceOwner::Variable(Variable{is_mut, ..}) => is_mut.to_owned(),
            ResourceOwner::Function(_) => false,
        }
    }

    pub fn is_ref(&self) -> bool {
        match self {
            ResourceOwner::Variable(Variable{is_ref, ..}) => is_ref.to_owned(),
            ResourceOwner::Function(_) => false,
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


impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut from_ro = None;
        let mut to_ro = None;
        let mut display = match self {
            Event::Acquire{ from } => { from_ro = from.to_owned(); "Acquiring resource" },
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
            Event::GoOutOfScope => { "Goes out of scope"}
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

            (State::FullPrivilege, Event::MutableReturn{ to: to_ro }) =>
                State::RevokedPrivilege {
                    to: to_ro.to_owned(),
                    borrow_to: None
                },

            (State::FullPrivilege, Event::MutableReacquire{ from: ro }) =>
                State::ResourceReturned { to: ro.to_owned() },

            // (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::MutableReacquire{ from: ro }) =>
            //         State::ResourceReturned { to: ro.to_owned() },

            (State::FullPrivilege, Event::StaticLend{ to: to_ro }) =>
                State::PartialPrivilege {
                    borrow_count: 1,
                    borrow_to: [(to_ro.to_owned().unwrap())].iter().cloned().collect() // we assume there is no borrow_to:None
                },

            (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::MutableLend{ to: to_ro }) => State::Invalid,

            (State::PartialPrivilege{ borrow_count: current, borrow_to: _ }, Event::StaticLend{ to: to_ro }) =>
                State::PartialPrivilege {
                    borrow_count: current+1,
                    borrow_to: [(to_ro.to_owned().unwrap())].iter().cloned().collect()
                },

            (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::StaticReturn{ to: to_ro }) => State::RevokedPrivilege {
                to: None,
                borrow_to: to_ro.to_owned()
                },

            (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::StaticReacquire{ from: ro }) =>
                State::ResourceReturned{
                    to: ro.to_owned()
                },

            (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, Event::GoOutOfScope) => State::OutOfScope,

            (State::RevokedPrivilege{ to: _, borrow_to: _ }, Event::MutableReacquire{ from: ro }) => State::ResourceReturned{ to: ro.to_owned() },

            (State::ResourceReturned{ to: _ }, _) => State::FullPrivilege,

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


    // add event using (internal) events
    fn append_event(&mut self, resource_owner: &ResourceOwner, event: Event, line_number: &usize) {
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

    fn append_external_event(&mut self, event: ExternalEvent, line_number: &usize) {
        self.external_events.push((*line_number, event.clone()));
        
        // append_event if resource_owner is not null
        fn maybe_append_event(vd: &mut VisualizationData, resource_owner: &Option<ResourceOwner>, event: Event, line_number : &usize) {
            if let Some(ro) = resource_owner {
                vd.append_event(&ro, event, line_number)
            };
        }

        match event {
            // eg let ro_to = String::from("");
            ExternalEvent::Move{from: from_ro, to: to_ro} => {
                maybe_append_event(self, &to_ro, Event::Acquire{from : from_ro.to_owned()}, line_number);
                maybe_append_event(self, &from_ro, Event::Move{to : to_ro.to_owned()}, line_number);
            }
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
            ExternalEvent::GoOutOfScope{ro} => {
                maybe_append_event(self, &Some(ro), Event::GoOutOfScope, line_number);         
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