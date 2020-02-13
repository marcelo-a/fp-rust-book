extern crate handlebars;

use crate::data::{VisualizationData, Visualizable, Event, ResourceOwner};
use handlebars::Handlebars;
use std::collections::{BTreeMap, HashMap};
use serde::Serialize;


#[derive(Serialize)]
struct LeftPanelData {
    labels: String,
    dots: String,
    timelines: String,
}

#[derive(Serialize)]
struct ResourceOwnerLabelData {
    x_val: String,
    hash: String,
    name: String,
}

#[derive(Serialize)]
struct EventDotData {
    hash: String,
    dot_x: String,
    dot_y: String,
}

pub fn render_left_panel(visualization_data : &VisualizationData) -> String {
    /* Template creation */
    let mut handlebars = Handlebars::new();
    // We want to preserve the inputs `as is`, and want to make no changes based on html escape.
    handlebars.register_escape_fn(handlebars::no_escape);

    let left_panel_template = 
        "    <g id=\"labels\">\n{{ labels }}    </g>\n\n    \
        <g id=\"events\">\n{{ dots }}    </g>\n\n    \
        <g id=\"timelines\">\n{{ timelines }}    </g>\n\n";
    let label_template = 
        "        <text class=\"code\" x=\"{{x_val}}\" y=\"80\" data-hash=\"{{hash}}\" text-anchor=\"middle\">{{name}}</text>\n";
    let dot_template =
        "        <use xlink:href=\"#eventDot\" data-hash=\"{{hash}}\" x=\"{{dot_x}}\" y=\"{{dot_y}}\"/>\n";
    let arrow_template = 
        "        <polyline strokeWidth=\"3\" stroke=\"beige\" points=\"{{X1}},{{Y1}} {{X2}},{{Y2}} \" marker-end=\"url(#arrowHead)\"/>\n";
    let vertical_line_template =
        "        <line class=\"{{line_class}}\" data-hash=\"{{hash}}\" x1=\"{{X1}}\" x2=\"{{X2}}\" y1=\"{{Y1}}\" y2=\"{{Y2}}\"/>\n";
    assert!(
        handlebars.register_template_string("left_panel_template", left_panel_template).is_ok()
    );
    assert!(
        handlebars.register_template_string("label_template", label_template).is_ok()
    );
    assert!(
        handlebars.register_template_string("dot_template", dot_template).is_ok()
    );
    assert!(
        handlebars.register_template_string("arrow_template", arrow_template).is_ok()
    );

    // hash -> (resource_owner name, pos)
    let mut resource_owners_layout = HashMap::new();
    let mut x : i64 = -180;
    let x_space = 30;               // for every new ResourceOwner, move 30 px to the left
    for(hash, _) in visualization_data.timelines.iter().rev() {
        let name = match visualization_data.get_name_from_hash(hash) {
            Some(_name) => _name,
            None => panic!("no matching resource owner for hash {}", hash),
        };
        resource_owners_layout.insert(hash, (name, x));
        x = x - x_space;
    }

    // render resource owner labels
    let labels_string = render_labels_string(&resource_owners_layout, &handlebars);
    let dots_string = render_dots_string(visualization_data, &resource_owners_layout, &handlebars);
    let timelines_string = render_timelines_string(visualization_data, &resource_owners_layout, &handlebars);

    let left_panel_data = LeftPanelData {
        labels: labels_string,
        dots: dots_string,
        timelines: timelines_string,
    };

    handlebars.render("left_panel_template", &left_panel_data).unwrap()
}

fn render_labels_string(
    resource_owners_layout: &HashMap<&u64, (String, i64)>,
    registry: &Handlebars
) -> String {
    let mut output = String::new();
    for(hash, (name, x_val)) in resource_owners_layout.iter() {
        let data = ResourceOwnerLabelData {
            x_val: x_val.to_string(),
            hash: hash.to_string(),
            name: name.clone()
        };
        output.push_str(&registry.render("label_template", &data).unwrap());
    }
    output
}

fn render_dots_string(
    visualization_data: &VisualizationData,
    resource_owners_layout: &HashMap<&u64, (String, i64)>,
    registry: &Handlebars
) -> String {
    let timelines = &visualization_data.timelines;
    let mut output = String::new();
    for (hash, timeline) in timelines {
        for (line_number, _) in timeline.history.iter() {
            let mut data = BTreeMap::new();
            let ro1_x_pos = resource_owners_layout[hash].1;
            let ro1_y_pos = event_y_pos(line_number);
            data.insert("hash".to_string(), *hash as i64);
            data.insert("dot_x".to_string(), ro1_x_pos);
            data.insert("dot_y".to_string(), ro1_y_pos);
            output.push_str(&registry.render("dot_template", &data).unwrap());
        }
    }
    output
}

fn render_timelines_string(
    visualization_data: &VisualizationData,
    resource_owners_layout: &HashMap<&u64, (String, i64)>,
    registry: &Handlebars
) -> String {
    let timelines = &visualization_data.timelines;
    let mut output = String::new();
    for (hash, timeline) in timelines {
        let ro = &timeline.resource_owner;
        for (line_number, event) in timeline.history.iter() {
            let ro1_x_pos = resource_owners_layout[hash].1;
            let ro1_y_pos = event_y_pos(line_number);

            // arrows
            match event {
                Event::Move { to : to_ro2 } => {
                    if let Some(ro2) = to_ro2 {
                        let mut data = BTreeMap::new();
                        let ro2_hash = & ro2.hash;
                        let ro2_x_pos = resource_owners_layout[ro2_hash].1;
                        let ro2_y_pos = event_y_pos(line_number);
                        data.insert("X2".to_string(), ro2_x_pos);
                        data.insert("Y2".to_string(), ro2_y_pos);
                        data.insert("X1".to_string(), ro1_x_pos);
                        data.insert("Y1".to_string(), ro1_y_pos);
                        output.push_str(&registry.render("arrow_template", &data).unwrap());
                    }
                    
                },
                _ => (),
            };
        } 
    }
    output
}

fn event_y_pos(line_number : &usize) -> i64 {
    (60 + 20 * line_number) as i64
}

fn insert_dot_data(data : & mut Vec<BTreeMap<String, String>>, maybe_ro : Option<ResourceOwner>){
    if let Some(ro) = maybe_ro {
        //data.insert("data");
    }
}