use std::collections::BTreeMap;
use handlebars::Handlebars;
use std::env;
use std::fs;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn render_source_code(path: String) -> String {
    // initialize coordinate y.
    let mut x = 0;
    let mut y = 80;
    
    // initialize output.
    let output = String::from("        <g id=\"code\">\n");

    // register the template. The template string will be verified and compiled.
    let code_line_temp = "                <text class=\"code\" x=\"{{X_VAL}}\" y=\"{{Y_VAL}}\"> {{LINE}} </text>\n";
    assert!(handlebars.register_template_string("code_line_temp", code_line_temp).is_ok());

    // file hosts must exist in given path.
    if let Ok(lines) = read_lines(path + "/annotated_source.rs") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // create the handlebars registry
                let mut handlebars = Handlebars::new();
                let mut data = BTreeMap::new();
                data.insert("X_VAL".to_string(), x.to_string());
                data.insert("Y_VAL".to_string(), Y.to_string());
                data.insert("LINE".to_string(), ip);
                output.push_str(handlebars.render("code_line_temp", &data).unwrap());
                y = y + 20;
            }
        }
    }
  
}

fn main() {
    path = "..\\example\\move_with_no_borrow";
    let str1 = render_source_code(path);
    println("{}", str1);
}