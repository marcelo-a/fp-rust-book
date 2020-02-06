extern crate handlebars;

use crate::data::{VisualizationData, Visualizable, ExternalEvent, ResourceOwner};
use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io;

pub fn render_events(visualization_data : &VisualizationData) -> String {
    /* Template creation */
    let mut handlebars = Handlebars::new();
    // We want to preserve the inputs `as is`, and want to make no changes based on html escape.
    handlebars.register_escape_fn(handlebars::no_escape);
    let resource_owner_template = 
    "        <text class=\"code\" x=\"{{X_VAL}}\" y=\"80\" data-hash=\"{{HASH}}\" text-anchor=\"middle\">{{NAME}}</text>";
    
    //let line_template =
    //    "              <polyline strokeWidth=\"3\" stroke=\"beige\" points=\"{X1},{Y1} -{X2},{Y2} \" marker-end=\"url(#arrowHead)\"/>";
    // register the template. The template string will be verified and compiled.
    assert!(handlebars
        .register_template_string("resource_owner_template", resource_owner_template)
        .is_ok());
    
    // hash -> (resource_owner name, pos)
    let mut resource_owners_layout = HashMap::new();

    let mut x = -40;
    let x_space = 30;
    for(hash, _) in visualization_data.timelines.iter(){
        let name = match visualization_data.get_name_from_hash(hash){
            Some(_name) => _name,
            None => panic!("no matching resource owner for hash {}", hash),
        };
        resource_owners_layout.insert(hash, (name, x));
        x = x - x_space;
    }

    // render resource owner
    let mut output = String::from("        <g id=\"resource_owners\">\n");
    for(hash, (name, x_val)) in resource_owners_layout.iter(){
        let mut data = BTreeMap::new();
        data.insert("X_VAL".to_string(), x_val.to_string());
        data.insert("HASH".to_string(), hash.to_string());
        data.insert("NAME".to_string(), name.clone());
        output.push_str(&handlebars.render("resource_owner_template", &data).unwrap());
        output.push_str("\n");
    }
    output.push_str("        </g>\n");

    // render events
    let dot_template =
        "        <use xlink:href=\"#eventDot\" data-hash=\"{{HASH}}\" x=\"{{DOT_X}}\" y=\"{{DOT_Y}}\"/>";
    let arrow_template = 
        "        <polyline strokeWidth=\"3\" stroke=\"beige\" points=\"{{X1}},{{Y1}} {{X2}},{{Y2}} \" marker-end=\"url(#arrowHead)\"/>";
    assert!(handlebars
        .register_template_string("dot_template", dot_template)
        .is_ok());
    assert!(handlebars
        .register_template_string("arrow_template", arrow_template)
        .is_ok());
    let external_events = &visualization_data.external_events;
    for (line_number, external_event) in external_events{
        //let mut data = BTreeMap::new();
        // match  external_event {
        //     ExternalEvent::Acquire{from, to} => {
        //         insert_dot_data(external_event);
        //         insert_dot_data(to);
        //         match (from, to) {
        //             (Some(ro_from), Some(ro_to)) => {insert_arrow_data(ro_from, ro_to)},
        //             (_, _) => {},
        //         }
        //     }
                
        // }
    }
   
    // for(line_number, external_event) in arrows_info.iter(){
        
    // }


    output

}

fn insert_dot_data(data : & mut Vec<BTreeMap<String, String>>, maybe_ro : Option<ResourceOwner>){
    if let Some(ro) = maybe_ro {
        //data.insert("data");
    }
}