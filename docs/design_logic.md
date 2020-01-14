# Overview
The *rrt* library attempts to address the following visualization process:

- Take rust code, analyze its lifetime (either by hand or use another tool to assist this process);
- Serialize this information into a rust struct [Lifetime](src/lib.rs);
- Convert this struct into a frontend representation of choice (currently we are aiming to support interactive svg).
- publish such svgs by embedding them into webpages and then host them online.

## General Pipeline
(Annotated Lifetime information) + (Annotated Source Code)

---Serialization to *rrt* Library-->

(*rrt* `Lifetime` objects) + (Annotated Source Code)

---Renderer-->

(interactive svg files)

At the end stage, we would like to have the svg files to take the form of a vertical split side-by-side view of **lifetime** (left panel) and **source code** (right panel).

We wish to be able to have pointer actions on both sides and incur changes to the visualization to the other side, which dictates that both panel must be rendered as a whole. 

Example interaction include:

- user click a variable name in the right panel, and the left panel renders to a lifetime of just that one variable (with relavent other variable if necessary). 
- user hovers over a lifetime `Event` in the left panel, relavent variables will appear highlighted on the right panel.


## Design note on Jan 13, 2020
In order to enable this process, I looked for svg generation, and found that the main-stream one like [resvg](https://github.com/RazrFalcon/resvg) has no support on dynamic svg which is core to our visualization plan. So I wish to use a more general templating library written in Rust. We are looking at [handlebars-rust](https://github.com/sunng87/handlebars-rust), [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark), or a more lightweight jinja-like [tera](https://github.com/Keats/tera).

On top of that, a serialization library could also make sense, like [Serde](https://github.com/serde-rs/serde), specifically, there are xml serialization support from some projectect, including [xml5ever](https://github.com/servo/html5ever/tree/master/xml5ever). 

We will have to evaluate the trade off of using one another, before crafting the frontend that generates the svg.