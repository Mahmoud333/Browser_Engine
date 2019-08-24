extern crate  browser_engine; //Our Browser engine
use browser_engine::{command, css, css_parser, dom, html_parse, layout, render, style}; //gonna use every single module

//all these libraries are necessary so we can read our html and css properly 
//and convert them into a string and then push them into our CSS parser and HTML parser
use std::env;   
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    //NOTE: 1- Nodes
    let nodes = get_html(); //Vector of Dom nodes
    for n in nodes.iter() {
        dom::pretty_print(n, 0);    //indent size of 0
    }

    let ref root_node = nodes[0]; //the root node is organised to be the 0 index 

    //NOTE: 2- Stylesheet
    let stylesheet = get_css();
    println!("{:?}", stylesheet);

    //NOTE: 3- Style tree
    //get style node from our style tree, pass root_node which is root of the dom node, 
    let style_tree_root = style::StyledNode::new(&root_node, &stylesheet);
    style::pretty_print(&style_tree_root, 0);   //indent size of 0

    //The size of actuall HTML that we are rendering, we have the size for the window b
    //but we need to specify we want the HTML to match that window size
    let mut viewport = layout::Dimensions::default();
    viewport.content.width = 1024.0;
    viewport.content.height = 768.0;

    //NOTE: 4- Layout tree
    let layout_tree = layout_tree::layout_tree(&style_tree_root, viewport);
    layout::pretty_print(&layout_tree, 0);  //indent size of 0

    //NOTE: 5- Display commands
    let display_commands = command::build_display_commands(&layout_tree); //return vector of display commands
    render::render_loop(&display_commands);
}

//will be the function that we use to grab our html file and convert it into a string
//then Dom node and then pass it back to our main function
fn get_html() -> Vec<dom::Node> {
    let mut path = env::current_dir().unwrap(); //current path
    path.push("example/example1.html");  //add the path we want

    let mut file_reader = match File::open(&path) { 
        Ok(f) => BufReader::new(f), //found and opened our file, put it into our new buff reader
        Err(e) => panic!("file: {}, error: {}", path.display(), e); //display file name and error
    };

    let mut html_input = String::new(); //read all our HTML into it
    file_reader.read_to_string(&mut html_input).unwrap(); //take file convert all content into string

    //call 'html_parse HtmlParser new' which is our entry point into our HTML parser, pass string for html input, 
    //then call parse_node so it will actually traverse the string and grab all of the information from it
    let nodes = html_parse::HtmlParser::new(&html_input).parse_nodes();
    nodes
}

fn get_css() -> css:Stylesheet {
    let mut path = env::current_dir().unwrap(); //current path
    path.push("example/example1.css");  //add the path we want

    let mut file_reader = match File::open(&path) { 
        Ok(f) => BufReader::new(f), //found and opened our file, put it into our new buff reader
        Err(e) => panic!("file: {}, error: {}", path.display(), e); //display file name and error
    };

    let mut css_input = String::new(); //read all our CSS into it
    file_reader.read_to_string(&mut css_input).unwrap(); //take file convert all content into string

    //call 'css_parser CssParser new' which is our entry point into our css_parser, pass string for html input, 
    //then call parse_stylesheet so it traverse the string and grab all of the information from it
    let stylesheet = css_parser::CssParser::new(&css_input).parse_stylesheet();
    stylesheet
}