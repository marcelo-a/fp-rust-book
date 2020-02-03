extern crate handlebars;

use handlebars::Handlebars;
use crate::data::VisualizationData;
use crate::svg_frontend::{right_panel, utils};

pub fn render_svg(path: &String, visualization_data: &VisualizationData) {
    let mut final_svg_content = String::new();
    let mut right_panel_string = String::new();

    if let Ok(lines) = utils::read_lines(path.to_owned() + "/annotated_source.rs") {
        right_panel_string = right_panel::render_source_code(lines);
    }

    final_svg_content += &right_panel_string;
    println!("{}", final_svg_content);
    utils::create_and_write_to_file(final_svg_content, path.to_owned() + "/rendering.svg");
}
