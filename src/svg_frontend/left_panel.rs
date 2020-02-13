extern crate handlebars;

use crate::data::{VisualizationData, Visualizable, Event, State, ResourceOwner};
use handlebars::Handlebars;
use std::collections::HashMap;
use serde::Serialize;

struct TimelineColumnData {
    name: String,
    x_val: i64
}

#[derive(Serialize)]
struct LeftPanelData {
    labels: String,
    dots: String,
    timelines: String,
    vertical_lines: String,
}
#[derive(Serialize)]
struct ResourceOwnerLabelData {
    x_val: i64,
    hash: String,
    name: String
}

#[derive(Serialize)]
struct EventDotData {
    hash: i64,
    dot_x: i64,
    dot_y: i64
}

#[derive(Serialize)]
struct ArrowData {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64
}

#[derive(Serialize)]
struct VerticalLineData {
    line_class: String,
    hash: u64,
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64
}

pub fn render_left_panel(visualization_data : &VisualizationData) -> String {
    /* Template creation */
    let mut registry = Handlebars::new();
    prepare_registry(&mut registry);

    // hash -> TimelineColumnData
    let resource_owners_layout = compute_column_layout(visualization_data);

    // render resource owner labels
    let labels_string = render_labels_string(&resource_owners_layout, &registry);
    let dots_string = render_dots_string(visualization_data, &resource_owners_layout, &registry);
    let timelines_string = render_timelines_string(visualization_data, &resource_owners_layout, &registry);
    let vertical_lines_string = render_vertical_lines_string(visualization_data, &resource_owners_layout, &registry);

    let left_panel_data = LeftPanelData {
        labels: labels_string,
        dots: dots_string,
        timelines: timelines_string,
        vertical_lines: "".to_string() // vertical_lines_string,
    };

    registry.render("left_panel_template", &left_panel_data).unwrap()
}

fn prepare_registry(registry: &mut Handlebars) {
    // We want to preserve the inputs `as is`, and want to make no changes based on html escape.
    registry.register_escape_fn(handlebars::no_escape);

    let left_panel_template = 
        "    <g id=\"labels\">\n{{ labels }}    </g>\n\n    \
        <g id=\"events\">\n{{ dots }}    </g>\n\n    \
        <g id=\"timelines\">\n{{ timelines }}    </g>\n\n    \
        <g id=\"vertical_lines\">\n{{ vertical_lines }}    </g>\n\n";
    let label_template = 
        "        <text class=\"code\" x=\"{{x_val}}\" y=\"80\" data-hash=\"{{hash}}\" text-anchor=\"middle\">{{name}}</text>\n";
    let dot_template =
        "        <use xlink:href=\"#eventDot\" data-hash=\"{{hash}}\" x=\"{{dot_x}}\" y=\"{{dot_y}}\"/>\n";
    let arrow_template = 
        "        <polyline strokeWidth=\"3\" stroke=\"beige\" points=\"{{x1}},{{y1}} {{x2}},{{y2}} \" marker-end=\"url(#arrowHead)\"/>\n";
    let vertical_line_template =
        "        <line class=\"{{line_class}}\" data-hash=\"{{hash}}\" x1=\"{{x1}}\" x2=\"{{x2}}\" y1=\"{{y1}}\" y2=\"{{y2}}\"/>\n";
    assert!(
        registry.register_template_string("left_panel_template", left_panel_template).is_ok()
    );
    assert!(
        registry.register_template_string("label_template", label_template).is_ok()
    );
    assert!(
        registry.register_template_string("dot_template", dot_template).is_ok()
    );
    assert!(
        registry.register_template_string("arrow_template", arrow_template).is_ok()
    );
    assert!(
        registry.register_template_string("vertical_line_template", vertical_line_template).is_ok()
    );
}

fn compute_column_layout<'a>(visualization_data: &'a VisualizationData) -> HashMap<&'a u64, TimelineColumnData> {
    let mut resource_owners_layout = HashMap::new();
    let mut x : i64 = -180;
    let x_space = 30;               // for every new ResourceOwner, move 30 px to the left
    for (hash, _) in visualization_data.timelines.iter().rev() {
        let name = match visualization_data.get_name_from_hash(hash) {
            Some(_name) => _name,
            None => panic!("no matching resource owner for hash {}", hash),
        };
        resource_owners_layout.insert(hash, TimelineColumnData{ name, x_val: x });
        x = x - x_space;
    }
    resource_owners_layout
}

fn render_labels_string(
    resource_owners_layout: &HashMap<&u64, TimelineColumnData>,
    registry: &Handlebars
) -> String {
    let mut output = String::new();
    for (hash, column_data) in resource_owners_layout.iter() {
        let data = ResourceOwnerLabelData {
            x_val: column_data.x_val,
            hash: hash.to_string(),
            name: column_data.name.clone()
        };
        output.push_str(&registry.render("label_template", &data).unwrap());
    }
    output
}

fn render_dots_string(
    visualization_data: &VisualizationData,
    resource_owners_layout: &HashMap<&u64, TimelineColumnData>,
    registry: &Handlebars
) -> String {
    let timelines = &visualization_data.timelines;
    let mut output = String::new();
    for (hash, timeline) in timelines {
        for (line_number, _) in timeline.history.iter() {
            let data = EventDotData {
                hash: *hash as i64,
                dot_x: resource_owners_layout[hash].x_val,
                dot_y: get_y_axis_pos(line_number)
            };
            output.push_str(&registry.render("dot_template", &data).unwrap());
        }
    }
    output
}

fn render_timelines_string(
    visualization_data: &VisualizationData,
    resource_owners_layout: &HashMap<&u64, TimelineColumnData>,
    registry: &Handlebars
) -> String {
    let timelines = &visualization_data.timelines;
    
    let mut output = String::new();
    for (hash, timeline) in timelines {
        let ro = timelines[hash].resource_owner.to_owned();
        for (line_number, event) in timeline.history.iter() {
            let ro1_x_pos = resource_owners_layout[hash].x_val;
            let ro1_y_pos = get_y_axis_pos(line_number);

            // arrows
            match event {
                Event::Move { to : to_ro2 } => {
                    if let Some(ro2) = to_ro2 {
                        let data = ArrowData {
                            x1: ro1_x_pos,
                            y1: ro1_y_pos,
                            x2: resource_owners_layout[&ro2.hash].x_val,
                            y2: get_y_axis_pos(line_number)
                        };
                        output.push_str(&registry.render("arrow_template", &data).unwrap());
                    }
                    
                },
                _ => (),
            };
        } 

        // verticle state lines
        let states = visualization_data.get_states(hash);
        for (line_start, line_end, state) in states.iter() {
            println!("{} {} {} {}", resource_owners_layout[hash].name, line_start, line_end, state);
            match (state, ro.is_mut) {
                (State::FullPriviledge, true) => {
                    let data = VerticalLineData {
                        line_class: String::from("solid"),
                        hash: *hash,
                        x1: resource_owners_layout[hash].x_val,
                        y1: get_y_axis_pos(line_start),
                        x2: resource_owners_layout[hash].x_val,
                        y2: get_y_axis_pos(line_end)
                    };
                    output.push_str(&registry.render("verticle_line_template", &data).unwrap());
                },
                _ => (),
            }
        }
    }
    output
}

fn render_vertical_lines_string(
    visualization_data: &VisualizationData,
    resource_owners_layout: &HashMap<&u64, TimelineColumnData>,
    registry: &Handlebars
) -> String {
    let timelines = &visualization_data.timelines;
    let mut output = String::new();
    
    for (hash, timeline) in timelines {
        let ro_x_pos = resource_owners_layout[hash].x_val;
        for (start_line, end_line, state) in visualization_data.get_states(hash).iter(){
            let data = VerticalLineData {
                x1: ro_x_pos,
                y1: get_y_axis_pos(start_line),
                x2: ro_x_pos,
                y2: get_y_axis_pos(end_line),
                hash: hash.to_owned(),
                line_class: get_vertical_line_class(state),
                
            };
            output.push_str(&registry.render("vertical_line_template", &data).unwrap());
        }
    }
    output
}

fn get_y_axis_pos(line_number : &usize) -> i64 {
    (60 + 20 * line_number) as i64
}

fn get_vertical_line_class(state : &State) -> String {
    // TODO fix this
    match state {
        State::FullPriviledge => "solid".to_string(),
        State::FractionalPriviledge{borrow_to : _} => "solid".to_string(),
        _ => "".to_string(),
    }
}
