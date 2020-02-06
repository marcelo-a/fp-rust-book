extern crate handlebars;

use handlebars::Handlebars;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;

pub fn render_source_code(lines: io::Lines<io::BufReader<File>>) -> String {
    /* Template creation */
    let mut handlebars = Handlebars::new();
    let line_template =
        "                <text class=\"code\" x=\"{{X_VAL}}\" y=\"{{Y_VAL}}\"> {{LINE}} </text>\n";
    // register the template. The template string will be verified and compiled.
    assert!(handlebars
        .register_template_string("code_line_template", line_template)
        .is_ok());

    /* Render the code segment of the svg to a String */
    let x = 0;
    let mut y = 80;
    let mut output = String::from("        <g id=\"code\">\n");
    for line in lines {
        if let Ok(line_string) = line {
            let mut data = BTreeMap::new();
            data.insert("X_VAL".to_string(), x.to_string());
            data.insert("Y_VAL".to_string(), y.to_string());
            data.insert("LINE".to_string(), line_string);
            output.push_str(&handlebars.render("code_line_template", &data).unwrap());
            y = y + 20;
        }
    }
    output.push_str("        </g>\n");
    output
}
