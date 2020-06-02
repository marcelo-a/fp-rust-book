# Notes about rust lifetime
Move happens when there is no Copy trait implemented, and Copy happens otherwise.
&T has Copy trait, &mut T has move trait.

Exception: reborrow
Note that this exception only applies in situations where the parameter is of the form &mut T; if it is a generic taken by value (e.g. fn<T>(x: T)) then a mutable reference you pass in will not be reborrowed automatically. You can still use the &mut *x incantation to manually reborrow it, however.

https://chrismorgan.info/blog/rust-ownership-the-hard-way/
