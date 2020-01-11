use std::collections::HashMap;

/** 
 * Basic Data Structure Needed by LifeTime Visualization
 */

// Top level Api that the Timeline object supports
trait Visualizable {
    // returns None if the hash does not exist
    fn get_name_from_hash(&self, hash: &u64) -> Option<String>;
    // returns None if the hash does not exist
    fn get_state(&self, hash: &u64, line_number: &usize) -> Option<State>;
    // add an event to the Visualizable data structure
    fn append_event(&mut self, ro : &ResourceOwner, event: Event, line_number: &usize);
}

// A ResourceOwner is either a Variable or a Function that 
// have ownership to a memory object, during some stage of
// a the program execution.
#[derive(Clone)]
 pub struct ResourceOwner {
    hash: u64,
    name: String,
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
    Acquire {
        from: Option<ResourceOwner>
    },
    // this happens when a ResourceOwner transfer the ownership of its resource
    // to another ResourceOwner, or if it is no longer used after this line.
    // Typically, this happens at one of the following two cases:
    // 
    // 1. variable is not used after this line. 
    // 2. variable's resource has the move trait, and it transfered
    //    its ownership on this line. This includes tranfering its
    //    ownership to a function as well. 
    Transfer {
        to: Option<ResourceOwner>
    },
    MutableLend {
        to: Option<ResourceOwner>
    },
    MutableReacquire {
        from: Option<ResourceOwner>
    },
    StaticReacquire {
        from: Option<ResourceOwner>
    },
    StaticLend {
        to: Option<ResourceOwner>
    },
    // this happens when a variable is returned this line,
    // or if this variable's scope ends at this line.
    GoOutOfScope,
}

// A State is a description of a ResourceOwner IMMEDIATELY AFTER a specific line.
// We think of this as what read/write access we have to its resource.
pub enum State {
    // The viable is no longer in the scope after this line.
    OutOfScope {
        scope_termination_line: usize
    },
    // The resource is transferred on this line or before this line,
    // thus it is impossible to access this variable anymore.
    Transfered {
        to: ResourceOwner,
        transfer_line: usize
    },
    // The resource can be fully accessed right after this line. 
    ReadableAndWritable,
    // The resource can be read, but cannot be written to right after this line.
    // This also means that it is not possible to create a mutable reference
    // on the next line.
    ReadableOnly,
}


// a vector of ownership transfer history of a specific variable, 
// in a sorted order by line number.
pub struct Timeline {
    variable_name: String,
    // line number in usize
    history: Vec<(usize, Event)>,
}

// VisualizationData supplies all the information we need in the frontend, 
// from rendering a PNG to producing an interactive HTML guide. 
// The internal data is simple: a map from variable hash to its Timeline.
pub struct VisualizationData {
    timelines: HashMap<u64, Timeline>,
}

// fulfills the promise that we can support all the methods that a 
// frontend would need. 
impl Visualizable for VisualizationData {
    fn get_name_from_hash(&self, hash: &u64) -> Option<String> {
        match self.timelines.get(hash) {
            Some(timeline) => Some(timeline.variable_name.to_owned()),
            _ => None
        }
    }

    fn get_state(&self, hash: &u64, line_number: &usize) -> Option<State> {
        match self.timelines.get(hash) {
            Some(timeline) => {
                // example return value
                Some(State::OutOfScope {scope_termination_line: 0})
            },
            _ => None
        }
    }

    fn append_event(&mut self, ro : &ResourceOwner, event: Event, line_number: &usize) {
        let hash = &ro.hash;
        let var_name = &ro.name;
        // if this event belongs to a new ResourceOwner hash,
        // create a new Timeline first, then bind it to the corresponding hash.
        match self.timelines.get(hash) {
            None => {
                let timeline = Timeline {
                    variable_name: String::from(var_name),
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
}

fn main() {
    let x = ResourceOwner {
        hash: 1,
        name: String::from("x")
    };
    let y= ResourceOwner {
        hash: 2,
        name: String::from("y")
    };
    let z= ResourceOwner {
        hash: 3,
        name: String::from("z")
    };
    let mut vd = VisualizationData{
        timelines : HashMap::new(),
    };
    // 
    // hash y : 2
    //      x : 1
    //      z : 3
    // functions: 0
    vd.append_event(&y, Event::Acquire{from : None}, &(2 as usize));
    vd.append_event(&z, Event::Acquire{from : None}, &(3 as usize));
    vd.append_event(&y, Event::Transfer{to : Some(x.clone())}, &(4 as usize));
    vd.append_event(&x, Event::Acquire{from : Some(y.clone())}, &(4 as usize));
    vd.append_event(&y, Event::GoOutOfScope, &(9 as usize));
    vd.append_event(&y, Event::GoOutOfScope, &(9 as usize));
    
}