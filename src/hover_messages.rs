/* Event Dot messages: shows up when someone hovers over a dot */


/* The Event dot does not connect to any arrows, we typically follow the following format:
   ... happens
 */

// 1   0 （0 is initialized by let 0 = &1)
//     |
//     *   the star event
// 
// example: 
// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()      // a REF_GO_OUT_OF_SCOPE event
// } // Here, s goes out of scope. But because it does not have ownership of what
//   // it refers to, nothing happens.
pub fn event_dot_ref_go_out_out_scope(my_name: &String) -> String {
    format!(
        "{0} goes out of scope. The data is not dropped because {0} is not the owner.",
        my_name
    )
}

// 0
// |
// *   the star event
//
pub fn event_dot_owner_go_out_out_scope(my_name: &String) -> String {
    format!(
        "{0} goes out of scope. The value is dropped.",
        my_name
    )
}

//     0
//     
// f1  *   the star event
// example: 
// fn calculate_length(s: &String) -> usize { // here s is initialized to some value
//    /* something happens */
// }
pub fn event_dot_init_param(my_name: &String) -> String {
    format!(
        "{0} is initialized by some value passed as argument to the function",
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
pub fn event_dot_copy_to(my_name: &String, target_name: &String) -> String {
    format!(
        "a copy of the value in {0} is made and bound to {1} ({0} is unchanged)",
        my_name,
        target_name
    )
}

// 1   0
//     |
// o<--*   the star event
// |
pub fn event_dot_move_to(my_name: &String, target_name: &String) -> String {
    format!(
        "{0} moves its value to {1} and loses ownership of the data",
        my_name,
        target_name
    )
}

// 1   0
//     |
// o<--*   the star event (&)
// |   |
pub fn event_dot_static_lend(my_name: &String, target_name: &String) -> String {
    format!(
        "{0} statically lends its data to {1} and becomes read-only",
        my_name,
        target_name
    )
}

// 1   0
//     |
// o<--*   the star event (&mut)
// |
pub fn event_dot_mut_lend(my_name: &String, target_name: &String) -> String {
    format!(
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
pub fn event_dot_static_return(my_name: &String, target_name: &String) -> String {
    format!(
        "returns borrowed data to {1} ({0}'s lifetime ends here)",
        my_name,
        target_name
    )
}

// 0   1
//     |
// o<--o
// |
// *-->o   the star event (&mut)
pub fn event_dot_mut_return(my_name: &String, target_name: &String) -> String {
    format!(
        "returns borrowed data to {1} ({0}'s lifetime ends here)",
        my_name,
        target_name
    )
}

/* The Event dot is the destination of an arrow, we typically follow the following format: 
   ... from <Resource Owner 1>
 */

// 1   0        1   0
// |            |   
// o-->*   or   o-->*     the star event
// |   |            |
pub fn event_dot_acquire(my_name: &String, target_name: &String) -> String {
    format!(
        "{0} is initialized by move from {1}",
        my_name,
        target_name
    )
}

// 0   1
//     |
// *<--o   the star event (&mut)
// |   |
pub fn event_dot_mut_borrow(my_name: &String, target_name: &String) -> String {
    format!(
        "mutably borrows data from {1} ({0} gains read and write access to data)",
        my_name,
        target_name
    )
}

// 0   1
//     |
// *<--o   the star event (&)
// |   |
pub fn event_dot_static_borrow(my_name: &String, target_name: &String) -> String {
    format!(
        "statically borrows data from {1} ({0} gains read only access to data)",
        my_name,
        target_name
    )
}

// 1   0
//     |
// o<--o
// |   |
// o-->*   the star event (&)
pub fn event_dot_static_reacquire(my_name: &String, target_name: &String) -> String {
    format!(
        "{1} no longer borrows from {0}",
        my_name,
        target_name
    )
}

// 1   0
//     |
// o<--o
// |
// o-->*   the star event (&mut)
pub fn event_dot_mut_reacquire(my_name: &String, target_name: &String) -> String {
    format!(
        "{1} is no longer a mutable referece of {0}, so we may read and write the data",
        my_name,
        target_name
    )
}



/* Arrow messages: shows up when someone hovers over an arrow */

// 1   0
//     |
// o<--o
// |
pub fn arrow_move_val_to_val(from_name: &String, to_name: &String) -> String {
    format!(
        "{0} moves the data to {1} ({0}'s lifetime ends here, {1}'s lifetime begins here)",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<--o
// |   |
pub fn arrow_copy_val_to_val(from_name: &String, to_name: &String) -> String {
    format!(
        "{0} makes a copy of data and assign {1} to be the owner ({0}'s lifetime continues here, {1}'s lifetime begins here)",
        from_name,
        to_name
    )
}

// f1  0
//     |
// o<--o
// |
pub fn arrow_move_val_to_func(from_name: &String, to_name: &String) -> String {
    format!(
        "{0}'s data moved to function {1} ({0}'s lifetime ends here)",
        from_name,
        to_name
    )
}

// f1  0
//     |
// o<--o
// |   |
pub fn arrow_copy_val_to_func(from_name: &String, to_name: &String) -> String {
    format!(
        "{0} makes a copy of data pass it to function {1} ({0}'s lifetime continues here)",
        from_name,
        to_name
    )
}

// 1  f0
//     |
// o<--o
// |
pub fn arrow_move_func_to_val(from_name: &String, to_name: &String) -> String {
    format!(
        "{0} allocate resource and let {1} become the data's owner",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<--o
// |   |
pub fn arrow_static_lend_val_to_val(from_name: &String, to_name: &String) -> String {
    format!(
        "{0} statically lends its data to {1}; {1} can only read from the data",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<->o
//     |
pub fn arrow_static_lend_val_to_func(from_name: &String, to_name: &String) -> String {
    format!(
        "{0} passes a static reference to {1} for {1} to read from the data",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<--o
// |
pub fn arrow_mut_lend_val_to_val(from_name: &String, to_name: &String) -> String {
    format!(
        "{0} mutably lends its data to {1}; {1} can now read and write to the data",
        from_name,
        to_name
    )
}

// 1   0
//     |
// o<->o
//     |
pub fn arrow_mut_lend_val_to_func(from_name: &String, to_name: &String) -> String {
    format!(
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
pub fn arrow_static_return(from_name: &String, to_name: &String) -> String {
    format!(
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
pub fn arrow_mut_return(from_name: &String, to_name: &String) -> String {
    format!(
        "{0}'s mutable borrow returned to {1}",
        from_name,
        to_name
    )
}



/* State messages: shows up on the vertical lines for every value/reference */

// The viable is no longer in the scope after this line.
pub fn state_out_of_scope(my_name: &String) -> String {
    format!(
        "{0} is out of scope, cannot access data from here",
        my_name
    )
}

// The resource is transferred on this line or before this line due to move,
// thus it is impossible to access this variable anymore. This is an invisible line in the timeline.
pub fn state_resource_moved(my_name: &String, to_name: &String) -> String {
    format!(
        "{0}'s value is moved to {1}, ownership is no longer with {0}",
        my_name,
        to_name
    )
}

// temporarily no read or write access right to the resource, but eventually
// the privilege will come back. Occurs when mutably borrowed. This is an invisible line in the timeline.
pub fn state_resource_revoked(my_name: &String, to_name: &String) -> String {
    format!(
        "{0}'s value is mutably borrowed by {1}, cannot access data until {1} returns",
        my_name,
        to_name
    )
}

// This ResourceOwner is the unique object that holds the ownership to the underlying resource.
pub fn state_full_privilege(my_name: &String) -> String {
    format!(
        "{0} is the unique owner of the data with read and write privileges",
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
pub fn state_partial_privilege(my_name: &String) -> String {
    format!(
        "{}'s data is being shared by one or more variables",
        my_name
    )
}

// should not appear for visualization in a correct program
pub fn state_invalid(my_name: &String) -> String {
    format!(
        "something is wrong with the timeline of {0}",
        my_name
    )
}