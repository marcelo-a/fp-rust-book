extern crate handlebars;

use crate::data::{VisualizationData, Visualizable, Event, ExternalEvent, State, ResourceOwner};
use handlebars::Handlebars;
use std::collections::HashMap;
use serde::Serialize;
use std::cmp;

struct TimelineColumnData {
    name: String,
    x_val: i64,
    title: String,
}

#[derive(Serialize)]
struct LeftPanelData {
    labels: String,
    dots: String,
    timelines: String,
    arrows: String,
}
#[derive(Serialize)]
struct ResourceOwnerLabelData {
    x_val: i64,
    hash: String,
    name: String,
    title: String,
}

#[derive(Serialize)]
struct EventDotData {
    hash: i64,
    dot_x: i64,
    dot_y: i64,
    title: String
}

#[derive(Serialize)]
struct ArrowData {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    title: String
}

#[derive(Serialize)]
struct FunctionLogoData {
    x: i64,
    y: i64,
    title: String
}

#[derive(Serialize, Clone)]
struct VerticalLineData {
    line_class: String,
    hash: u64,
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
    title: String,
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
    // let vertical_lines_string = render_vertical_lines_string(visualization_data, &resource_owners_layout, &registry);
    let arrows_string = render_arrows_string_external_events_version(visualization_data, &resource_owners_layout, &registry);
    let left_panel_data = LeftPanelData {
        labels: labels_string,
        dots: dots_string,
        timelines: timelines_string,
        arrows: arrows_string
    };

    registry.render("left_panel_template", &left_panel_data).unwrap()
}

fn prepare_registry(registry: &mut Handlebars) {
    // We want to preserve the inputs `as is`, and want to make no changes based on html escape.
    registry.register_escape_fn(handlebars::no_escape);

    let left_panel_template =
        "    <g id=\"labels\">\n{{ labels }}    </g>\n\n    \
        <g id=\"timelines\">\n{{ timelines }}    </g>\n\n    \
        <g id=\"arrows\">\n{{ arrows }}    </g>\n\n    \
        <g id=\"events\">\n{{ dots }}    </g>";

    let label_template =
        "        <text class=\"code\" x=\"{{x_val}}\" y=\"80\" data-hash=\"{{hash}}\"><title>{{title}}</title>{{name}}</text>\n";
    let dot_template =
        "        <use xlink:href=\"#eventDot\" data-hash=\"{{hash}}\" x=\"{{dot_x}}\" y=\"{{dot_y}}\"><title>{{title}}</title></use>\n";
    let function_logo_template =
        "        <text x=\"{{x}}\" y=\"{{y}}\" font-size = \"20\" font-style = \"italic\" class=\"heavy\" ><title>{{title}}</title>f</text>";
    let arrow_template =
        "        <polyline stroke-width=\"2.5\" stroke=\"beige\" points=\"{{x2}},{{y2}} {{x1}},{{y1}} \" marker-end=\"url(#arrowHead)\"><title>{{title}}</title></polyline>\n";
    let vertical_line_template =
        "        <line data-hash=\"{{hash}}\" class=\"{{line_class}}\" x1=\"{{x1}}\" x2=\"{{x2}}\" y1=\"{{y1}}\" y2=\"{{y2}}\"><title>{{title}}</title></line>\n";
    
    let hollow_line_internal_template =
        "        <line stroke=\"#232323\" stroke-width=\"3px\" x1=\"{{x1}}\" x2=\"{{x2}}\" y1=\"{{y1}}\" y2=\"{{y2}}\"><title>{{title}}</title></line>\n";
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
    assert!(
        registry.register_template_string("hollow_line_internal_template", hollow_line_internal_template).is_ok()
    );
    assert!(
        registry.register_template_string("function_logo_template", function_logo_template).is_ok()
    );
}

// Returns: a hashmap from the hash of the ResourceOwner to its Column information
fn compute_column_layout<'a>(visualization_data: &'a VisualizationData) -> HashMap<&'a u64, TimelineColumnData> {
    let mut resource_owners_layout = HashMap::new();
    let mut x : i64 = -10;                   // Right-most Column x-offset.
    // let x_space = 30;                   // for every new ResourceOwner, move 30 px to the left
    for (hash, timeline) in visualization_data.timelines.iter().rev() {
        // only put variable in the column layout
        if let ResourceOwner::Variable(_) = timeline.resource_owner {
            let name = match visualization_data.get_name_from_hash(hash) {
                Some(_name) => _name,
                None => panic!("no matching resource owner for hash {}", hash),
            };
            let x_space = cmp::max(70, (&(name.len() as i64)-1)*10);
            x = x - x_space;
            let title = match visualization_data.is_mut(hash) {
                true => String::from("mutable"),
                false => String::from("immutable"),
            };
            resource_owners_layout.insert(hash, TimelineColumnData
                { 
                    name: name.clone(), 
                    x_val: x, 
                    title: name.clone() + ", " + &title,
                });
        }
        
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
            name: column_data.name.clone(),
            title: column_data.title.clone(),
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
        if let ResourceOwner::Variable(_) = timeline.resource_owner {
            for (line_number, event) in timeline.history.iter() {
                let data = EventDotData {
                    hash: *hash as i64,
                    dot_x: resource_owners_layout[hash].x_val,
                    dot_y: get_y_axis_pos(line_number),
                    title: event.to_string()
                };
                output.push_str(&registry.render("dot_template", &data).unwrap());
            }
        }   
    }
    output
}

// render events involving two RO using an arrow
// NOTE currently using render_arrows_string_external_events_version instead of this one
fn render_arrows_string(
    visualization_data: &VisualizationData,
    resource_owners_layout: &HashMap<&u64, TimelineColumnData>,
    registry: &Handlebars
) -> String {
    let timelines = &visualization_data.timelines;

    let mut output = String::new();
    for (hash, timeline) in timelines {
        if let ResourceOwner::Variable(_) = timeline.resource_owner {

            let _ = timelines[hash].resource_owner.to_owned();
            
            for (line_number, event) in timeline.history.iter() {
                let ro1_x_pos = resource_owners_layout[hash].x_val;
                let ro1_y_pos = get_y_axis_pos(line_number);

                // render arrow only if ro2 give resource to ro1
                // i.e. ro2 points to ro1
                let some_ro2 : Option<ResourceOwner> = match event {
                    Event::Acquire { from : from_ro } => from_ro.to_owned(),
                    Event::MutableBorrow { from : from_ro } => Some(from_ro.to_owned()),
                    Event::StaticBorrow { from : from_ro } => Some(from_ro.to_owned()),
                    Event::StaticReacquire { from : from_ro } => from_ro.to_owned(),
                    Event::MutableReacquire { from: from_ro } => from_ro.to_owned(),
                    _ => None,
                    };
                let title: String = match event {
                    Event::Acquire { from : Some(from_ro) } => format!("{}{}", String::from("acquire resource from: "), from_ro.name()),
                    Event::MutableBorrow { from : from_ro } => format!("{}{}", String::from("dynamic borrow from: "), from_ro.name()),
                    Event::StaticBorrow { from : from_ro } => format!("{}{}", String::from("static borrow from: "), from_ro.name()),
                    Event::StaticReacquire { from : Some(from_ro) } => format!("{}{}", String::from("static return from: "), from_ro.name()),
                    Event::MutableReacquire { from: Some(from_ro) } => format!("{}{}", String::from("dynamic return from: "), from_ro.name()),
                    _ => String::from(""),
                };
                if let Some(ro2) = some_ro2 {
                    let mut data = ArrowData {
                        x1: ro1_x_pos,
                        y1: ro1_y_pos,
                        x2: resource_owners_layout[&ro2.hash()].x_val,
                        y2: ro1_y_pos,
                        title: title,
                    };
                    // adjust arrow head pos
                    if data.x1 < data.x2 {
                        data.x1 = data.x1 + 10;
                    }
                    else {
                        data.x1 = data.x1 - 10;
                    }
                    output.push_str(&registry.render("arrow_template", &data).unwrap());
                }
            }
        }
    }
    output
}

// render arrows that support function
fn render_arrows_string_external_events_version(
    visualization_data: &VisualizationData,
    resource_owners_layout: &HashMap<&u64, TimelineColumnData>,
    registry: &Handlebars
) -> String {
    let mut output = String::new();
    for (line_number, external_event) in &visualization_data.external_events {
        let mut title = String::from("");
        let (from, to) = match external_event {
            ExternalEvent::Duplicate{ from: from_ro, to: to_ro } => {
                title = String::from("Duplicate");
                (from_ro, to_ro)
            },
            ExternalEvent::Move{ from: from_ro, to: to_ro } => {
                title = String::from("Move");
                (from_ro, to_ro)
            },
            ExternalEvent::StaticBorrow{ from: from_ro, to: to_ro } => {
                title = String::from("Static borrow");
                (from_ro, to_ro)
            },
            ExternalEvent::StaticReturn{ from: from_ro, to: to_ro } => {
                title = String::from("Return statically borrowed resource");
                (from_ro, to_ro)
            },
            ExternalEvent::MutableBorrow{ from: from_ro, to: to_ro } => {
                title = String::from("Mutable borrow");
                (from_ro, to_ro)
            },
            ExternalEvent::MutableReturn{ from: from_ro, to: to_ro } => {
                title = String::from("Return mutably borrowed resource");
                (from_ro, to_ro)
            },
            ExternalEvent::PassByMutableReference{ from: from_ro, to: to_ro } => {
                title = String::from("Pass by mutable reference");
                (from_ro, to_ro)
            }
            ExternalEvent::PassByStaticReference{ from: from_ro, to: to_ro } => {
                title = String::from("Pass by static reference");
                (from_ro, to_ro)
            }
            _ => (&None, &None),
        };
        // complete title
        if let Some(some_from) = from {
            let from_string = match some_from {
                    ResourceOwner::Variable(var) => var.name.to_owned(),
                    ResourceOwner::Function(func) => "the return value of ".to_owned() + &func.name,
            };
            title = format!("{} from {}", title, from_string);
        };
        if let Some(some_to) = to {
            let to_string = match some_to {
                ResourceOwner::Variable(var) => var.name.to_owned(),
                ResourceOwner::Function(func) => "the parameter of ".to_owned() + &func.name,
            };
            title = format!("{} to {}", title, to_string);
        };
        // calc arrow data and function logo data
        let mut data = ArrowData {
            x1: 0,
            y1: get_y_axis_pos(line_number),
            x2: 0,
            y2: get_y_axis_pos(line_number),
            title: title
        };
        let arrow_length = 20;
        match (from, to, external_event) {
            
            // TODO change this to or pattern
            (Some(ResourceOwner::Variable(variable)), 
             Some(ResourceOwner::Function(function)), 
             ExternalEvent::PassByStaticReference{..}) => {
                // get variable's position
                let function_data = FunctionLogoData {
                x: resource_owners_layout[&variable.hash].x_val,
                y: get_y_axis_pos(line_number),
                title: function.name.to_owned(),
                };
                output.push_str(&registry.render("function_logo_template", &function_data).unwrap());
            },
            (Some(ResourceOwner::Variable(variable)), 
             Some(ResourceOwner::Function(function)), 
             ExternalEvent::PassByMutableReference{..}) => {
                // get variable's position
                let function_data = FunctionLogoData {
                x: resource_owners_layout[&variable.hash].x_val,
                y: get_y_axis_pos(line_number),
                title: function.name.to_owned(),
                };
                output.push_str(&registry.render("function_logo_template", &function_data).unwrap());
            },
            (Some(ResourceOwner::Function(from_function)), Some(ResourceOwner::Variable(to_variable)), _) => {
                // ro1 (to_variable) <- ro2 (from_function)
                data.x1 = resource_owners_layout[&to_variable.hash].x_val + 3; // adjust arrow head pos
                data.x2 = data.x1 + arrow_length;
                let function_data = FunctionLogoData {
                    x: data.x2 + 3,
                    y: data.y2 + 5,
                    title: from_function.name.to_owned(),
                };
                output.push_str(&registry.render("function_logo_template", &function_data).unwrap());
            },
            (Some(ResourceOwner::Variable(from_variable)), Some(ResourceOwner::Function(to_function)), _) => {
                //  ro1 (to_function) <- ro2 (from_variable)
                data.x2 = resource_owners_layout[&from_variable.hash].x_val - 5;
                data.x1 = data.x2 - arrow_length;
                let function_data = FunctionLogoData {
                    // adjust Function logo pos
                    x: data.x1 - 10,  
                    y: data.y1 + 5,
                    title: to_function.name.to_owned(),
                };
                output.push_str(&registry.render("function_logo_template", &function_data).unwrap());
            },
            (Some(ResourceOwner::Variable(from_variable)), Some(ResourceOwner::Variable(to_variable)), _) => {
                data.x1 = resource_owners_layout[&to_variable.hash].x_val;
                data.x2 = resource_owners_layout[&from_variable.hash].x_val;
            },
            _ => (), // don't support other cases for now
        }
        // draw arrow only if data.x1 is not dafault value
        if data.x1 != 0 {
            // adjust arrow head pos
            if data.x1 < data.x2 {
                data.x1 = data.x1 + 10;
            }
            else {
                data.x1 = data.x1 - 10;
            }
            output.push_str(&registry.render("arrow_template", &data).unwrap()); 
        }
           
    }
    output
}

// render timelines (states) for ROs using vertical lines
fn render_timelines_string(
    visualization_data: &VisualizationData,
    resource_owners_layout: &HashMap<&u64, TimelineColumnData>,
    registry: &Handlebars
) -> String {
    let timelines = &visualization_data.timelines;

    let mut output = String::new();
    for (hash, timeline) in timelines{
        if let ResourceOwner::Variable(_) = timeline.resource_owner {
            let ro = timeline.resource_owner.to_owned();
            // verticle state lines
            let states = visualization_data.get_states(hash);
            for (line_start, line_end, state) in states.iter() {
            println!("{} {} {} {}", resource_owners_layout[hash].name, line_start, line_end, state);
            let mut data = VerticalLineData {
                line_class: String::new(),
                hash: *hash,
                x1: resource_owners_layout[hash].x_val,
                y1: get_y_axis_pos(line_start),
                x2: resource_owners_layout[hash].x_val,
                y2: get_y_axis_pos(line_end),
                title: String::new()
            };
            match (state, ro.is_mut()) {
                (State::FullPrivilege, true) => {
                    data.line_class = String::from("solid");
                    if ro.is_ref() {
                        data.title = String::from("has read and write privilege to the reference itself")
                    }else{
                        data.title = String::from("has read and write privilege to the real data")
                    }
                    output.push_str(&registry.render("vertical_line_template", &data).unwrap());
                },
                (State::FullPrivilege, false) => {
                    data.line_class = String::from("solid");
                    if ro.is_ref() {
                        data.title = String::from("has read only privilege to the reference itself")
                    }else{
                        data.title = String::from("has read only privilege to the real data")
                    }
                    output.push_str(&registry.render("vertical_line_template", &data).unwrap());
                    let mut hollow_internal_line_data = data.clone();
                    hollow_internal_line_data.y1 += 5;
                    hollow_internal_line_data.y2 -= 5;
                    if ro.is_ref() {
                        hollow_internal_line_data.title = String::from("has read only privilege to the reference itself")
                    }else{
                        hollow_internal_line_data.title = String::from("has read only privilege to the real data")
                    }
                    output.push_str(&registry.render("hollow_line_internal_template", &hollow_internal_line_data).unwrap());
                },
                (State::PartialPrivilege{ borrow_count: _, borrow_to: _ }, _) => {
                    data.line_class = String::from("solid");
                    if ro.is_ref() {
                        data.title = String::from("has read only privilege to the reference itself")
                    }else{
                        data.title = String::from("has read only privilege to the real data")
                    }
                    output.push_str(&registry.render("vertical_line_template", &data).unwrap());
                    let mut hollow_internal_line_data = data.clone();
                    hollow_internal_line_data.y1 += 5;
                    hollow_internal_line_data.y2 -= 5;
                    if ro.is_ref() {
                        hollow_internal_line_data.title = String::from("has read only privilege to the reference itself")
                    }else{
                        hollow_internal_line_data.title = String::from("has read only privilege to the real data")
                    }
                    output.push_str(&registry.render("hollow_line_internal_template", &hollow_internal_line_data).unwrap());
                },
                (State::ResourceMoved{ move_to: _, move_at_line: _ }, true) => {
                    data.line_class = String::from("extend");
                    if ro.is_ref() {
                        data.title = String::from("its value(reference) has been moved")
                    }else{
                        data.title = String::from("its value(data) has been moved")
                    }
                    output.push_str(&registry.render("vertical_line_template", &data).unwrap());
                }
                 // do nothing when the case is (RevokedPrivilege, false), (OutofScope, _), (ResouceMoved, false)
                 _ => (),
            }
        }
        }
    }
    output
}


fn get_y_axis_pos(line_number : &usize) -> i64 {
    (60 + 20 * line_number) as i64
}
