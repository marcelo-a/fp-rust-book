extern crate handlebars;

use crate::data::VisualizationData;
use crate::svg_frontend::{left_panel, right_panel, utils};
use handlebars::Handlebars;
use serde::Serialize;
use std::cmp;

#[derive(Serialize)]
struct SvgData {
    visualization_name: String,
    css: String,
    code: String,
    diagram: String,
    divider_x_pos: i64,
    divider_y_endpoint: i32,
    width: i32,
    height: i32,
}

pub fn render_svg(listing_id: &String, description: &String, visualization_data: &VisualizationData) {
    let example_dir_path = format!("examples/book_{}_{}/", listing_id, description);
    let book_image_file_path = format!("rustBook/src/img/vis_{}.svg", listing_id);
    
    let mut left_panel_string = String::new();
    let mut num_lines = 0;
    let mut max_x = 0;

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

    let css_string = utils::read_file_to_string("src/svg_frontend/book_svg_style.css")
        .unwrap_or("Reading book_svg_style.css failed.".to_owned());

    // data for left panel
    if let Ok(lines) = utils::read_lines(example_dir_path.to_owned() + "annotated_source.rs") {
        let (output, line_of_code) = left_panel::render_left_panel(lines);
        left_panel_string = output;
        num_lines = line_of_code;
    }
        
    // set divider position
    if let Ok(lines) = utils::read_lines(example_dir_path.to_owned() + "source.rs") {
        max_x = left_panel::set_divider_pos(lines);
    }

    // data for right panel
    let (right_panel_string, max_width) = right_panel::render_right_panel(visualization_data, max_x);
        
    let svg_data = SvgData {
        visualization_name: description.to_owned(),
        css: css_string,
        code: left_panel_string,
        diagram: right_panel_string,
        divider_x_pos: max_x,
        divider_y_endpoint: num_lines * 20 + 80,
        width: cmp::max(max_width, 750),
        height: (num_lines * 20 + 80) + 50,
    };

    let final_svg_content = handlebars.render("full_svg_template", &svg_data).unwrap();

    println!("{}", final_svg_content);
    utils::create_and_write_to_file(&final_svg_content, example_dir_path + "rendering.svg");
    utils::create_and_write_to_file(&final_svg_content, book_image_file_path);
}
