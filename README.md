# Rust-Resource-Timeline
*rust-resource-timeline* (rrt) is a Rust [Lifetime and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) visualization library.

## Usage
Currently, this repo is not in any state to be used, but you can read the [library data](src/lib.rs) to get a sense of the data structure that corresponds to the design.

Update: we are starting to support some build. 
Primary examples:

Use: 
* Please use `cargo install --git https://github.com/gab-umich/mdBook.git mdbook` to install a specifically fine tuned version of the mdbook command before proceeding.
* `cargo run --example <name_of_example>` to build an SVG related to a certain piece of code.
	* for instance, `cargo run --example scope` will find an example called `scope` in `./Cargo.toml`, and trigger the running of [examples/book_04_01_02/main.rs](examples/book_04_01_02/main.rs). This will in-turn call dependencies of its execution: the [main.rs](examples/book_04_01_02/main.rs) will take in the [examples/book_04_01_02/annotated_source.rs](examples/book_04_01_02/annotated_source.rs), and compute the [rendering.svg](examples/book_04_01_02/rendering.svg) by calling functions from [src/lib.rs](src/lib.rs)
* To write your own example: we keep all the example visualizations in `./examples/`. , create a folder and create these files (don't change their filenames!) in it:
    1. source.rs (currently not used): the code that we want to visualize
    2. annotated_source.rs: every *resource owner* is enclosed by an html tag with a data hash to distinguish them from shadowing
    3. main.rs: this takes annotated_source.rs as input and generates the following files:
      		* rendering_code.svg: the code visualization
     		* rendering_timeline.svg: the timeline part
  		. The output can be found in the example directory (for debug purposes), and under `rustBook/src/img/vis_$EG_NAME$_(code|timeline).svg`. The combined visualization is `rustBook/src/img/vis_$EG_NAME$.svg`, where $EG_NAME$ is determined in the `render_svg` function.

	remember to make the new example an entry in `./Cargo.toml` for `cargo run --example $EXAMPLE` to work!

## Design Philosophy
I will discuss both current design process and choices in the document here: [docs/design_logic.md](docs/design_logic.md)
