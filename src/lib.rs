/** 
 * Basic Data Structure Needed by LifeTime Visualization
 */

// A ResourceOwner is either a Variable or a Function that 
// have ownership to a memory object, during some stage of
// a the program execution. 
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
    //      x = Vec::new();
    // x obtained the resource from global resource allocator,
    // the Acquire Event's "from" variable is None. 
    // in the case of 
    //      y = x;
    // y obtained its value from x, which means that the Acquire
    // Event's "from" variable is x. 
    Acquire {
        from: Option<ResourceOwner>
    },
    // this happens when a variable goes out of scope or its
    // resource will be no longer available after this line.
    // Typically, this happens at one of the following three cases:
    // 
    // 1. variable is not used after this line. 
    // 2. variable's resource has the move trait, and it transfered
    //    its ownership on this line. This include tranfering its
    //    ownership to a function as well. 
    // 3. variable is returned on this line. 
    Release {
        to: Option<ResourceOwner>
    },
    MutableBorrow {
        from: Option<ResourceOwner>
    },
    MutableLend {
        to: Option<ResourceOwner>
    },
    StaticBorrow {
        from: Option<ResourceOwner>
    },
    StaticLend {
        to: Option<ResourceOwner>
    },
}

// a vector of ownership transfer history of a specific variable, 
// in a sorted order by line number.
pub struct Timeline {
    // line number in usize
    history: Vec<(usize, Event)>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
