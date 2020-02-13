use std::collections::{HashMap, HashSet};
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
    fn append_event(&mut self, ro : &ResourceOwner, event: Event, line_number: &usize);
    // add an ExternalEvent to the Visualizable data structure
    fn append_external_event(&mut self, line_number : usize, external_event: ExternalEvent); 

    // if resource_owner with hash is mutable
    fn is_mut(&self, hash: &u64 ) -> bool;

    fn calc_state(&self, previous_state : & State, event : & Event, event_line: usize, hash: &u64) -> State;

    fn get_states(&self, hash: &u64) -> Vec::<(usize, usize, State)>;

    
    // SVG left panel generation facilities, MIGHT NOT NEED IMPL
    // // return a timeline for a single resource owner 
    // fn svg_dot_info(&self, hash : &u64) -> Timeline;
    // // return all timelines
    // fn svg_dot_info_map(&self) -> HashMap<u64, Vec<SvgLineInfo>>;
    // // return svg_arrows := {Move, Borrow, Return}
    // fn svg_arrows_info(&self) -> &Vec<(usize, ExternalEvent)>;
}

// A ResourceOwner is either a Variable or a Function that 
// have ownership to a memory object, during some stage of
// a the program execution.
#[derive(Clone)]
pub struct ResourceOwner {
    pub hash: u64,
    pub name: String,
    // whether the variable itself is mutable
    pub is_mut: bool,
    // pub is_fun: bool,
    // pub lifetime_trait: LifetimeTrait,
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
        from: Option<ResourceOwner>
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

#[derive(Clone)]
pub enum LifetimeTrait {
    Copy,
    Move,
    None,
}


// A State is a description of a ResourceOwner IMMEDIATELY AFTER a specific line.
// We think of this as what read/write access we have to its resource.
#[derive(Clone)]
pub enum _State {
    // The viable is no longer in the scope after this line.
    OutOfScope {
        scope_terminate_at_line: usize
    },
    // The resource is transferred on this lResourceOwnerine or before this line,
    // thus it is impossible to access this variable anymore.
    Moved {
        to: ResourceOwner,
        move_at_line: usize
    },
    // The resource can be fully accessed right after this line. 
    ReadableAndWritable,
    // The resource can be read, but cannot be written to right after this line.
    // This also means that it is not possible to create a mutable reference
    // on the next line.
    ReadableOnly {
        borrowed_to : HashSet<ResourceOwner>,
    },
    // when mutably borrowed
    NotReadable,
    // should not appear for visualization in a correct program
    Invalid,
}

// A State is a description of a ResourceOwner IMMEDIATELY AFTER a specific line.
// We think of this as what read/write access we have to its resource.
#[derive(Clone)]
pub enum State {
    // The viable is no longer in the scope after this line.
    OutOfScope,
    // The resource is transferred on this lResourceOwnerine or before this line,
    // thus it is impossible to access this variable anymore.
    ResourceMoved {
        to: ResourceOwner,
        move_at_line: usize
    },
    // The resource can be fully accessed right after this line; whether it's mutable is up to its def
    FullPriviledge,
    // The resource can be read, but cannot be written to right after this line.
    // This also means that it is not possible to create a mutable reference
    // on the next line.
    FractionalPriviledge {
        borrowed_to : HashSet<ResourceOwner>,
    },
    // temporarily no read or write access right to the resource, but eventually 
    // the priviledge will come back. Most frequently occurs when mutably borrowed
    NoPriviledge,
    // should not appear for visualization in a correct program
    Invalid,
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
    pub timelines: HashMap<u64, Timeline>,
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

    // if the RO is declared mutable
    fn is_mut(&self, hash: &u64 ) -> bool {
        self.timelines[hash].resource_owner.is_mut
    }

    fn calc_state(&self, previous_state: & State, event: & Event, event_line : usize, hash: &u64) -> State {
        match(previous_state) {
            State::Invalid | State::OutOfScope => State::Invalid,      // any event happened on an already OutOfScope RO is invalid
            State::FullPriviledge => {
                match (event) {
                    // not dealing with duplicate, cuz thats a use
                    Event::Acquire{from : _} => {
                        if (self.is_mut(hash)) {State::FullPriviledge} else {State::Invalid}
                    }
                    _ => State::Invalid,
                }
            }
            _ => State::Invalid,
        }
    }

    fn get_states(&self, hash: &u64) -> Vec::<(usize, usize, State)> {

        let mut states = Vec::<(usize, usize, State)>::new();
        let mut start_line_number = std::usize::MAX;
        let mut prev_state = State::OutOfScope;
        for (line_number, event) in self.timelines[hash].history.iter() {
            if (start_line_number == std::usize::MAX) {
                start_line_number = *line_number;
            }
            prev_state = self.calc_state(&prev_state, &event, *line_number, hash);
            states.push((start_line_number, *line_number, prev_state.clone()));
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
    fn append_event(&mut self, ro: &ResourceOwner, event: Event, line_number: &usize) {
        let hash = &ro.hash;
        let var_name = &ro.name;
        // if this event belongs to a new ResourceOwner hash,
        // create a new Timeline first, thenResourceOwner bind it to the corresponding hash.
        match self.timelines.get(hash) {
            None => {
                let timeline = Timeline {
                    resource_owner: ro.clone(),
                    history: Vec::new(),
                };
                self.timelines.insert(*hash, timeline);
            },
            _ => {}
        }

        // append the event to the end of the timeline of the corresponding hash
        match self.timelines.get_mut(hash) {
            Some(timeline) => {
                timeline.history.push((*line_number, event));
            },
            _ => {
                panic!("Timeline disappeared right after creation or when we could index it. This is impossible.");
            }
        }
    }

    // TODO IMPLEMENT
    fn append_external_event(&mut self, line_number : usize, external_event: ExternalEvent){
        self.external_events.push((line_number, external_event));
    } 

    // // return all states for all Resource Owner
    // fn svg_line_info_all(&self) -> HashMap<u64, Vec<SvgLineInfo>>{
    //     HashMap::new()
    // }
    // // return a timeline for a single resource owner 
    // fn svg_dot_info(&self, hash : &u64) -> Timeline{
    //     let tl = Timeline{
    //         resource_owner : ResourceOwner {
    //             hash : 0,
    //             name : String::from("x"),
    //             is_mut : false,
    //             // is_fun : false,
    //         },
    //         history : Vec::<(usize, Event)>::new(),

    //     };
    //     tl
    // }

    // // return all timelines
    // fn svg_dot_info_map(&self) -> HashMap<u64, Vec<SvgLineInfo>>{
    //     HashMap::new()
    // }

    // // return svg_arrows := {Move, Borrow, Return}
    // fn svg_arrows_info(&self) -> &Vec<(usize, ExternalEvent)> {
    //     &self.external_events
    // }
}