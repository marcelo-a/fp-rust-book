extern crate handlebars;

use crate::data::VisualizationData;
use crate::svg_frontend::{left_panel, right_panel, utils};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct SvgData {
    visualization_name: String,
    css: String,
    code: String,
    diagram: String,
}

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

    // data for left panel
    let left_panel_string = left_panel::render_left_panel(visualization_data);

    // data for right panel
    if let Ok(lines) = utils::read_lines(path.to_owned() + "/annotated_source.rs") {
        right_panel_string = right_panel::render_right_panel(lines);
    }

    let mut svg_data = SvgData {
        visualization_name: example_name.to_owned(),
        css: css_string,
        code: right_panel_string,
        diagram: left_panel_string
    };

    let final_svg_content = handlebars.render("full_svg_template", &svg_data).unwrap();

    println!("{}", final_svg_content);
    utils::create_and_write_to_file(final_svg_content, path.to_owned() + "/rendering.svg");
}
