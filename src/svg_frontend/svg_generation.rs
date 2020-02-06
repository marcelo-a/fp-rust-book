extern crate handlebars;

use crate::data::VisualizationData;
use crate::svg_frontend::{right_panel, left_panel, utils};
use std::collections::BTreeMap;
use handlebars::Handlebars;

pub fn render_svg(example_name: &String, visualization_data: &VisualizationData) {
    let path = "examples/".to_owned() + example_name;
    let mut right_panel_string = String::new();
    // let mut left_panel_string = String::new();

    let svg_template = utils::read_file_to_string("src/svg_frontend/template.svg")
        .unwrap_or("Reading template.svg failed.".to_owned());

    let mut handlebars = Handlebars::new();
    // We want to preserve the inputs `as is`, and want to make no changes based on html escape.
    handlebars.register_escape_fn(handlebars::no_escape);
    let full_svg_template = svg_template;
    // register the template. The template string will be verified and compiled.
    assert!(handlebars
        .register_template_string("full_svg_template", full_svg_template)
        .is_ok());

    let css_string = utils::read_file_to_string("src/svg_frontend/svg_style.css")
        .unwrap_or("Reading svg_style.css failed.".to_owned());

    if let Ok(lines) = utils::read_lines(path.to_owned() + "/annotated_source.rs") {
        right_panel_string = right_panel::render_source_code(lines);
    }

    
    // data for right panel
    let mut svg_data = BTreeMap::new();
    svg_data.insert("visualization_name".to_owned(), example_name);
    svg_data.insert("css".to_owned(), &css_string);
    svg_data.insert("code".to_owned(), &right_panel_string);
    
    // data for left panel
    let events_string = left_panel::render_events(visualization_data);
    svg_data.insert("events".to_owned(), &events_string);

    let final_svg_content = handlebars.render("full_svg_template", &svg_data).unwrap();

    println!("{}", final_svg_content);
    utils::create_and_write_to_file(final_svg_content, path.to_owned() + "/rendering.svg");
}
