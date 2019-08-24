//Style Tree
//Style tree is much like a dom tree except each node in this tree includes a pointer to a dom node plus its CSS properties
//If u take a look at mozilla gecko engine, it takes a dom tree and then it produces whats called a frame tree which then used to build 
//A view tree then chrome's webkit takes in a dom tree and outputs what is called a render tree
//It has a few other trees which are called layer trees and widget trees
//In our implementation each node in the Dom tree has exactly 1 node in the style tree 

use std::collections::HashMap;
use std::{fmt, str};            //fmt trait and str trait

use crate::dom::{ElementData, Node, NodeType};
use crate::css::{Selector, Stylesheet, Value};

type PropertyMap<'a> = HashMap<&'a str, &'a Value>;

//Each of our style nodes will corespond with a node (dom Node)
//We have styles which is our PropertyMap which contains 1- Slice of String, 2- Value | which is enum of Color Length or other
//We have our children field which is vector of our stylednode
//we r using these lifetime modifiers to make sure everything lives long enough

pub struct StyledNode<'a> {
    node: &'a Node,
    styles: PropertyMap<'a>,
    pub children: Vec<StyledNode<'a>>, 
}

pub enum Display {
    Block,
    Inline,
    InlineBlock,
    None,
    //Correspond with a different style of styled node,
    //Block style node corrresponds with a element that will naturally look like a block like page itself would be a block
    //Inline style node is inline and inside of an object so text or maybe like a button or something
    //Inline Block style node would be something that sits inside our block but also is a block itself
    //None will have to styling at all
}

//NOTE: Functions
impl <'a> StyledNode <'a> {
    pub fn new(node: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
        //Will recursivelly create our style tree without any of the style rules 
        //& then apply the style rules afterward
        let mut style_children = Vec::new();

        for child in &node.children {   //every child inside the passed node
            match child.node_type {
                NodeType::Element(_) => style_children.push(StyledNode::new(&child, stylesheet))
            }
        }

        StyledNode {
            node,   //Passed Node
            styles: match node.node_type {
                NodeType::Element(ref e) => StyledNode::get_styles(e, stylesheet),
                _ => PropertyMap::new(),
            },
            children: style_children,
        }
    }

    //Return syle of the current node that we're looking at
    fn get_styles(element: &'a ElementData, stylesheet: &'a Stylesheet) -> PropertyMap<'a> {
        let mut styles = PropertyMap::new();

        for rule in &stylesheet.rules { //for rule in stylesheet.rules
            for selector in &rule.selectors { //for selector in rule.selector
                if selector_matches(element, &selector) {
                    for declar in &rule.declarations {  //iterate through them and add them to our propertymap
                        styles.insert(&declar.property, &declar.value);
                    }
                    break;
                }
            }
        }
        styles
    }

    //Reason we have && (Double refrence) is bec. the get function, if we were to use 1 reference we would have miss match
    //specific reason: bec. our PropertyMap which has both reference to value & reference to string, & bec. we want
    //to get that value out we need to say that its a reference to the reference to value
    pub fn value(&self, name: &str) -> Option<&&Value> {
        self.styles.get(name)
    }

    //Return the value of display property of the current node, tell us what type of display our node is
    pub fn get_display(&self) -> Display {
        match self.value("display") {   //match on what we get from value function
            Some(s) => match **s { //if get some with value inside of it, then ** (double the reference) s
                Value::Other(ref v) => match v.as_ref() { //check if value other, match on info inside other as reference
                    "block" => Display::Block,
                    "none" => Display::None,
                    "inline-block" => Display::InlineBlock,
                    _ => Display::Inline, //anything else
                },
                _ => Display::Inline, //anything else
            },
            None => Display::Inline, //anything else
        }
    }

    //Applies to properties that have numbers in them
    //return style property for the current node or a default value
    //our name is the property name of the return value
    //default is the value we gonna default to if we get back none
    pub fn num_or(&self, name: &str, default: f32) -> f32 {
        match self.value(name) { //get back option of reference reference value
            Some(v) => match **v {  //Option has a Some, take value thats inside of it then double dereference that value and match on it
                Value::Length(n, _) => n,   //if got value length then take out the f32 inside of that
                _ => default,               //return default
            }
            None => default,                //return default
        }
    }
}

impl<'a> fmt::Debug for StyledNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.node, self.styles)
    }
}

//NOTE: Helper functions
//Make sure our selector matches a Dom node
//Element: Element data for the Dom node that we want to match
//Selector: the selector we want to match to that Dom node
fn selector_matches(element: &ElementData, selector: &Selector) -> bool {
    for simple in selector.simple { // Vector of Simple Selector
        let mut selector_match = true;

        //ANCHOR Checks selector part of our stylesheet
        match simple.tag_name {
            Some(ref t) => if *t != element.tag_name { //if some, check that tag name isn't equal to the element tag name
                continue;
            },
            None => {}
        };
        
        //ANCHOR Checks for id in our node in stylesheet
        match element.get_id() { //make sure they are not equivalent
            Some(i) => match simple.id {
                Some(ref id) => if *i != *id {
                    continue;
                },
                None => {}
            },
            None => match simple.id {
                Some(_) => {
                    continue;
                },
                _ => {}
            },
        }

        //ANCHOR See if that class in in our node in stylesheet 
        //Get all classes for our styled element
        let element_classes = element.get_classes(); //return HashSet with Ref. to slice of string

        for class in &simple.classes {
            selector_match &= element_classes.contains::<str>(class);
            //try to see if element classes contains the class we iterating through
            //if it does change our boolen selector match
        }

        if selector_match { //if true return true
            return true
        }
    }
    false  //else return false 
}


pub fn pretty_print(node: &StyledNode, indent_size: usize) {
    //get our indent size by iterating through indent_size & map it to create whitespaces & collect it to string
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    //print out indent & node with a debug flag
    println!("{}{:?}", indent, node);

    //Iterate through all of the node for our styled node & apply our pretty_print func to all of these children node
    for child in node.children.iter() {
        pretty_print(&child, indent_size + 2);
    }
}