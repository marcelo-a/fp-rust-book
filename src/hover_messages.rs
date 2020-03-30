use std::fmt::{Formatter, Result};

/* Event Dot messages: shows up when someone hovers over a dot */


/* The Event dot does not connect to any arrows, we typically follows the following format:
   ... happens
 */

// 1   0 （0 is initiallized by let 0 = &1)
//     |
//     *   the star event
// 
// example: 
// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()      // a REF_GO_OUT_OF_SCOPE event
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, nothing happens.
pub fn event_dot_ref_go_out_out_scope(f: &mut Formatter, my_name: &String) -> Result {
    write!(
        f,
        "{0} goes out of scope. The data is not dropped because {0} is not the owner.",
        my_name
    )
}

// 0
// |
// *   the star event
//
pub fn event_dot_owner_go_out_out_scope(f: &mut Formatter, my_name: &String) -> Result {
    write!(
        f,
        "{0} goes out of scope. The data is dropped.",
        my_name
    )
}


/* The Event dot is the source of an arrow, we typically follow the following format: 
   ... to <Resource Owner 1>
 */

// 1   0
//     |
// o<--*   the star event
// |   |
pub fn event_dot_copy_to(f: &mut Formatter, my_name: &String, target_name: &String) -> Result {
    write!(
        f,
        "copies it's value to {1} ({0} keeps ownership)",
        my_name,
        target_name
    )
}

// 1   0
//     |
// o<--*   the star event
// |
pub fn event_dot_move_to(f: &mut Formatter, my_name: &String, target_name: &String) -> Result {
    write!(
        f,
        "moves it's value to {1} ({0} lost ownership)",
        my_name,
        target_name
    )
}

// 1   0
//     |
// o<--*   the star event (&)
// |   |
pub fn event_dot_static_lend(f: &mut Formatter, my_name: &String, target_name: &String) -> Result {
    write!(
        f,
        "statically lends its data to {1} ({0} becomes read-only)",
        my_name,
        target_name
    )
}

// 1   0
//     |
// o<--*   the star event (&mut)
// |
pub fn event_dot_mut_lend(f: &mut Formatter, my_name: &String, target_name: &String) -> Result {
    write!(
        f,
        "mutably lends its data to {1} ({0} becomes un-readable until the end of {1}'s lifetime)",
        my_name,
        target_name
    )
}

// 0   1
//     |
// o<--o
// |   |
// *-->o   the star event (&)
const EVENT_DOT_STATIC_RETURN: &'static str = "returns borrowed data to {1} ({0}'s lifetime ends here)";

// 0   1
//     |
// o<--o
// |
// *-->o   the star event (&mut)
const EVENT_DOT_MUT_RETURN: &'static str = "returns borrowed data to {1} ({0}'s lifetime ends here)";


/* The Event dot is the destination of an arrow, we typically follow the following format: 
   ... from <Resource Owner 1>
 */

// 1   0        1   0
// |            |   
// o-->*   or   o-->*     the star event
// |   |            |
const EVENT_DOT_ACQUIRE: &'static str = "obtains data from {1} ({0}'s lifetime begins from here)";

// 0   1
//     |
// *<--o   the star event (&mut)
// |   |
const EVENT_DOT_MUT_BORROW: &'static str = "mutably borrows data from {1} ({0} gains read and write access to data)";

// 0   1
//     |
// *<--o   the star event (&)
// |   |
const EVENT_DOT_STATIC_BORROW: &'static str = "statically borrows data from {1} ({0} gains read only access to data)";

// 1   0
//     |
// o<--o
// |   |
// o-->*   the star event (&)
const EVENT_DOT_STATIC_REACQUIRE: &'static str = "{1} no longer borrows from {0}";

// 1   0
//     |
// o<--o
// |
// o-->*   the star event (&mut)
const EVENT_DOT_MUT_REACQUIRE: &'static str = "{0} no longer has any reference, so we may read and write the data";



/* Arrow messages: shows up when someone hovers over an arrow */

// 1   0
//     |
// o<--o
// |
pub fn arrow_move_val_to_val(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0} moves the data to {1} ({0}'s lifetime ends here, {1}'s lifetime begins here)",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<--o
// |   |
pub fn arrow_copy_val_to_val(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0} makes a copy of data and assign {1} to be the owner ({0}'s lifetime continues here, {1}'s lifetime begins here)",
        from_name,
        to_name
    )
}

// f1  0
//     |
// o<--o
// |
pub fn arrow_move_val_to_func(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0}'s data moved to function {1} ({0}'s lifetime ends here)",
        from_name,
        to_name
    )
}

// f1  0
//     |
// o<--o
// |   |
pub fn arrow_copy_val_to_func(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0} makes a copy of data pass it to function {1} ({0}'s lifetime continues here)",
        from_name,
        to_name
    )
}

// 1  f0
//     |
// o<--o
// |
pub fn arrow_move_func_to_val(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0} allocate resource and let {1} become the data's owner",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<--o
// |   |
pub fn arrow_static_lend_val_to_val(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0} statically lends its data to {1}; {1} can only read from the data",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<->o
//     |
pub fn arrow_static_lend_val_to_func(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0} passes a static reference to {1} for {1} to read from the data",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<--o
// |
pub fn arrow_mut_lend_val_to_val(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0} mutably lends its data to {1}; {1} can now read and write to the data",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<->o
//     |
pub fn arrow_mut_lend_val_to_func(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0} passes a mutable reference to {1} for {1} to read or write the data",
        from_name,
        to_name
    )
}

// 0   1
//     |
// o<--o
// |   |
// o-->o   this arrow (&)
pub fn arrow_static_return(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0}'s static borrow returned to {1}",
        from_name,
        to_name
    )
}

// 0   1
//     |
// o<--o
// |
// o-->o   the star event (&mut)
pub fn arrow_mut_return(f: &mut Formatter, from_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0}'s mutable borrow returned to {1}",
        from_name,
        to_name
    )
}



/* State messages: shows up on the vertical lines for every value/reference */

// The viable is no longer in the scope after this line.
pub fn state_out_of_scope(f: &mut Formatter, my_name: &String) -> Result {
    write!(
        f,
        "{0} is out of scope, cannot access data from here",
        my_name
    )
}

// The resource is transferred on this line or before this line due to move,
// thus it is impossible to access this variable anymore. This is an invisible line in the timeline.
pub fn state_resource_moved(f: &mut Formatter, my_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0}'s resource is moved to {1}, ownership is no longer with {0}",
        my_name,
        to_name
    )
}

// temporarily no read or write access right to the resource, but eventually
// the privilege will come back. Occurs when mutably borrowed. This is an invisible line in the timeline.
pub fn state_resource_revoked(f: &mut Formatter, my_name: &String, to_name: &String) -> Result {
    write!(
        f,
        "{0}'s resource is mutably borrowed by {1}, cannot access data until {1} returns",
        my_name,
        to_name
    )
}

// This ResourceOwner is the unique object that holds the ownership to the underlying resource.
pub fn state_full_priviledge(f: &mut Formatter, my_name: &String) -> Result {
    write!(
        f,
        "{0} is the unique value that can access the data in memory",
        my_name
    )
}

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
pub fn state_partial_priviledge(f: &mut Formatter, my_name: &String) -> Result {
    write!(
        f,
        "some values are statically referencing to {0}",
        my_name
    )
}

// should not appear for visualization in a correct program
pub fn state_invalid(f: &mut Formatter, my_name: &String) -> Result {
    write!(
        f,
        "something is wrong with the timeline of {0}",
        my_name
    )
}