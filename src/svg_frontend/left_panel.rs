extern crate handlebars;

use crate::data::{VisualizationData, Visualizable, Event, ResourceOwner};
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::collections::HashMap;

pub fn render_left_panel(visualization_data : &VisualizationData) -> String {
    /* Template creation */
    let mut handlebars = Handlebars::new();
    // We want to preserve the inputs `as is`, and want to make no changes based on html escape.
    handlebars.register_escape_fn(handlebars::no_escape);
    let resource_owner_template = 
    "        <text class=\"code\" x=\"{{X_VAL}}\" y=\"80\" data-hash=\"{{HASH}}\" text-anchor=\"middle\">{{NAME}}</text>";

    // register the template. The template string will be verified and compiled.
    assert!(
        handlebars.register_template_string("resource_owner_template", resource_owner_template).is_ok()
    );
    
    // hash -> (resource_owner name, pos)
    let mut resource_owners_layout = HashMap::new();

    let mut x : i64 = -180;
    let x_space = 30;               // for every new ResourceOwner, move 30 px to the left
    for(hash, _) in visualization_data.timelines.iter(){
        let name = match visualization_data.get_name_from_hash(hash){
            Some(_name) => _name,
            None => panic!("no matching resource owner for hash {}", hash),
        };
        resource_owners_layout.insert(hash, (name, x));
        x = x - x_space;
    }

    // render resource owner labels
    let mut output = String::from("<g id=\"resource_owners\">\n");
    for(hash, (name, x_val)) in resource_owners_layout.iter(){
        let mut data = BTreeMap::new();
        data.insert("X_VAL".to_string(), x_val.to_string());
        data.insert("HASH".to_string(), hash.to_string());
        data.insert("NAME".to_string(), name.clone());
        output.push_str(&handlebars.render("resource_owner_template", &data).unwrap());
        output.push_str("\n");
    }
    output.push_str("        </g>\n");

    // render event dots
    let dot_template =
        "        <use xlink:href=\"#eventDot\" data-hash=\"{{HASH}}\" x=\"{{DOT_X}}\" y=\"{{DOT_Y}}\"/>";
    let arrow_template = 
        "        <polyline strokeWidth=\"3\" stroke=\"beige\" points=\"{{X1}},{{Y1}} {{X2}},{{Y2}} \" marker-end=\"url(#arrowHead)\"/>";
    assert!(
        handlebars.register_template_string("dot_template", dot_template).is_ok()
    );
    assert!(
        handlebars.register_template_string("arrow_template", arrow_template).is_ok()
    );
    let timelines = &visualization_data.timelines;
    for (hash, timeline) in timelines {
        let ro = & timeline.resource_owner;
        for (line_number, event) in timeline.history.iter(){
            // dots
            let mut data = BTreeMap::new();
            let ro1_x_pos = resource_owners_layout[hash].1;
            let ro1_y_pos = event_y_pos(line_number);
            data.insert("HASH".to_string(), *hash as i64);
            data.insert("DOT_X".to_string(), ro1_x_pos);
            data.insert("DOT_Y".to_string(), ro1_y_pos);
            output.push_str(&handlebars.render("dot_template", &data).unwrap());
            output.push_str("\n");

            // arrows
            match event{
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
                        output.push_str(&handlebars.render("arrow_template", &data).unwrap());
                        output.push_str("\n");
                    }
                    
                },
                _ => (),
            };
        } 
    }
    let vertical_line_template = "    <line class=\"{{LINE_CLASS}}\" data-hash=\"{{HASH}}\" x1=\"{{X1}}\" x2=\"{{X2}}\" y1=\"{{Y1}}\" y2=\"{{Y2}}\"/>";

    output
}

fn event_y_pos(line_number : &usize) -> i64{
    (60 + 20 * line_number) as i64
}

fn insert_dot_data(data : & mut Vec<BTreeMap<String, String>>, maybe_ro : Option<ResourceOwner>){
    if let Some(ro) = maybe_ro {
        //data.insert("data");
    }
}