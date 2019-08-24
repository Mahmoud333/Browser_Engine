//1
/* Help us to parse the Dom or the document object model
 * Dom itself is a tree of nodes
 */

use std::collections::{HashMap, HashSet}; 
//HashMap: like dict HashMap<String, String>, HashSet: like set
use std::fmt;

#[derive(PartialEq, Eq, Clone)]
pub enum NodeType {     //Node types that we gonna deal with
    Text(String),   //Node Text with String inside of it
    Element(ElementData), //... Element with ElementData inside of it
    Comment(String),      //... comment with String inside of it
}

#[derive(PartialEq, Eq, Clone)]
pub struct Node {
    pub children: Vec<Node>,    //0 or more connected to this node, like tree pattern
    pub node_type: NodeType,    
}

#[derive(PartialEq, Eq, Clone)]
pub struct ElementData {
    pub tag_name: String,       //div
    attributes: AttrMap,    //Any number of attributes
/* If u have a div, the div is the tag name 
 * and then it could have a class or an id and those would be an attributes and we can store them at AttrMap
 */
} 

impl ElementData {
    pub fn new(tag_name: String, attributes: AttrMap) -> ElementData {
        ElementData {
            tag_name,
            attributes,
        }
    }

    ///Get Attributes Id
    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    ///Get Attributes Classes
    pub fn get_classes(&self) -> HashSet<&str> {
        //Match on the output, if found it return it as HashSet of str
        match self.attributes.get("class") {
            Some(s) => s.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}


pub type AttrMap = HashMap<String, String>; //typealias HashMap, its like Dict
//type Int = i32; //Other example

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children,
        }
    }
}

//Implement Debug for Node so we can debug & easily see it inside browser or terminal
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type) //with the debug flag
    }
}

// ALSO Implement Debug for NodeType 
impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            //With Text & Comment writing with simple write statement

            NodeType::Element(ref e) => write!(f, "{:?}", e),
            //With Element writing with Debug flag
        }
    }
}

// ALSO Implement Debug for ElementData 
impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //Empty String
        let mut attributes_string = String::new();

        //Iterate in HashMap & add every attributes with its value
        for (attr, value) in self.attributes.iter() {
            attributes_string.push_str(&format!(" {}=\"{}\"", attr, value));
        }
        write!(f, "<{},{}>", self.tag_name, attributes_string)
    }
}

//Print Node and its descendants with indentation, so it be more neater for us
fn pretty_print(n: &Node, indent_size: usize) {
    //Iterate from 0 to our indent size, map closure & return space, into vector of strings
    //Will give us approprite indent size for everything we need
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    //Match on node type and print them in different ways
    match n.node_type {
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
    }

    //Iterate through all of child nodes in our node
    //Each of them call pretty_print recursively with incremented indent_size by 2
    for child in n.children.iter() {
        pretty_print(&child, indent_size + 2);
    } 

    //Match on nodetype, if element print out indent + indent with tag name inside of this here
    //so actually it look like HTML, for rest skip this part, bec. they don't have tag names
    match n.node_type {
        NodeType::Element(ref e) => println!("{}</{}>", indent, e.tag_name),
        _ => {},
    }

}