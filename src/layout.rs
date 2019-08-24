/* Our layout module will take our style tree & translate it into a bunch of rectangles in 2Dimensional space
 * The layout modules input is the style tree itself & then the output is yet another tree which will be our layout tree 
 * layout and HTML and CSS is all about boxes a box is rectangular section of a web page it has a width a height a position on the page
 * The rectangle is called the content area bec. its where the boxs content is drawn, content may be (Text, Image, Video or other boxes)
 * The box may have (padding, borders, margins) to serround its content area and the CSS spec has a diagram showing how all of these layers fit together
 * descciption: https://www.w3.org/TR/CSS2/box.html#box-dimensions
 */

use std::fmt; //bec. will add Debugging

use crate::css::{Unit, Value}; //Unit enum (have all unit types), Value enum (have diff. values that comes from our CSS)
use crate::style::{Display, StyledNode}; //Display enum that have (block, inline, inlineblock, None), styledNode the main structure from style tree


//The Main Structure for our layout tree - like node wrapper
#[derive(Clone)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    box_type: BoxType,
    pub styled_node: &'a StyledNode<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

#[derive(Clone, Copy, Default)]
pub struct Dimensions {
    pub content: Rectangle,
    padding: EdgeSizes,
    pub border: EdgeSizes,
    margin: EdgeSizes,
    current: Rectangle, //refer to current layer box that we're looking at
}

#[derive(Clone, Copy, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32, 
    //The position of the content area relative to the original document
}

#[derive(Clone, Copy, Default)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    //Are our surrounding edges of our layout box
}

#[derive(Clone)]
pub enum BoxType {
    Block,
    Inline,
    InlineBlock,
    Anonymous,
}

//NOTE: LayoutBox Methods
impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType, styled_node: &'a StyledNode) -> LayoutBox<'a> {
        LayoutBox {
            box_type: box_type,
            styled_node: styled_node,
            dimensions: Default::default(), //default value for dimensions, which sets all values to 0.0
            children: Vec::new(),
        }
    }

    //ANCHOR Layout
    fn layout(&mut self, b_box: Dimensions) {
        match self.box_type {
            BoxType::Block => self.layout_block(b_box),
            BoxType::Inline => self.layout_block(b_box),
            BoxType::InlineBlock => self.layout_inline_block(b_box),
            BoxType::Anonymous => {}
        }
    }

    fn layout_inline_block(&mut self, b_box: Dimensions) {
        //each one will calculate its respective thing in pixels 
        self.calculate_inline_width(b_box); //calculate width in pixels
        self.calculate_inline_position(b_box); //calculate position in pixels
        self.layout_children();  //call it in our children
        self.calculate_height(); //calculate height in pixels
    }

    //ANCHOR Calculate
    fn calculate_inline_width(&mut self, b_box: Dimensions) {
        let s = self.styled_node;       //get Style node
        let d = &mut self.dimensions;    //get Dimensions

        //Set width of our layout box
        d.content.width = get_absolute_num(s, b_box, "width").unwrap_or(0.0); //unwrap it from none, but if none then default it to 0.0
        //Put selector that corresponds with element we're looking at then put the default value
        //Will try to read from CSS "margin-left" if it gets value it will set that value if it didn't we will set the default (0.0)
        d.margin.left = s.num_or("margin-left", 0.0);
        d.margin.right = s.num_or("margin-right", 0.0);
        d.padding.left = s.num_or("padding-left", 0.0);
        d.padding.right = s.num_or("padding-right", 0.0);
        d.border.left = s.num_or("border-left", 0.0);
        d.border.right = s.num_or("border-right", 0.0);
    }

    //Will position current box below any previous boxes in a container by updating the height
    //& if have any boxs that are to left or to right it will also update the width
    fn calculate_inline_position(&mut self, b_box: Dimensions) {
        let style = self.styled_node;       //get Style node
        let d = &mut self.dimensions;        //get Dimensions

        //Put selector that corresponds with element we're looking at then put the default value
        //Will try to read from CSS "margin-top" if it gets value it will set that value if it didn't we will set the default (0.0)
        d.margin.top = style.num_or("margin-top", 0.0);
        d.margin.bottom = style.num_or("margin-bottom", 0.0);
        d.border.top = style.num_or("border-top-width", 0.0);
        d.border.bottom = style.num_or("border-bottom-width", 0.0);
        d.padding.top = style.num_or("padding-top", 0.0);
        d.padding.bottom = style.num_or("padding-top", 0.0);

        d.content.x = b_box.content.x + b_box.current.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y = b_box.content.height + b_box.content.y + d.margin.top + d.border.top + d.padding.top;
    }

    //ANCHOR Layout
    //Calls all the functions to layout the current box with our dimentions here being parent box
    //so the bounding box that might be surrounding this box
    fn layout_block(&mut self, b_box: Dimensions) {
        self.calculate_width(b_box);
        self.calculate_position(b_box);
        self.layout_children();
        self.calculate_height();
    } 

    //ANCHOR Calculate
    //Update current layout boxes with dimensions, pass parent bounding box
    fn calculate_width(&mut self, b_box: Dimensions) {
        let style = self.styled_node;
        let d = &mut self.dimensions;

        //get absoule number from CSS by calculating based on where parent is & where child is
        let width = get_absolute_num(style, b_box, "width").unwrap_or(0.0); //Unwrap the option value & if it was None then make it 0.0
        //get margin left - which is styled by value
        let margin_l = style.value("margin-left");
        let margin_r = style.value("margin-right");

        //bec. they r refrences to references we need to double deconstruct them
        let margin_l_num = match margin_l {
            Some(m) => match **m {
                Value::Other(ref s) => s.parse().unwrap_or(0.0), //parse string into an number & if can't unwrap or parsed make it default 0.0
                _ => 0.0,
            },
            None => 0.0,
        };
        let margin_r_num = match margin_r {
            Some(m) => match **m {
                Value::Other(ref s) => s.parse().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        };

        //These are all for our child box, based on parent box & we're calculating them for child box by fulling them out of our style tree
        d.border.left = style.num_or("border-left-width", 0.0);
        d.border.right = style.num_or("border-right-width", 0.0);
        d.padding.left = style.num_or("padding-left", 0.0);
        d.padding.right = style.num_or("padding-right", 0.0); 

        //add all number to total
        let total = width + margin_l_num + margin_r_num + d.border.left + d.border.right + d.padding.left + d.padding.right;
        //underflow which is parent's box content width - total, EXAMPLE: if u have two boxs next to one another inside larger box 
        //if u calculate total which is width of entire parent box & remove width of one of these smaller boxs then 
        //u will be able to figure out where the child box will be
        let underflow = b_box.content.width - total;

        match (width, margin_l, margin_r) {
            (0.0, _, _) => {    //means our width is auto
                if underflow >= 0.0 {
                    d.content.width = underflow;
                    d.margin.right = margin_r_num;
                } else {
                    d.margin.right = margin_r_num + underflow;
                    d.content.width = width;
                }
                d.margin.left = margin_l_num;
            }
            (w, None, Some(_)) if w != 0.0 => { //means our margin left is auto, 
                d.margin.left = underflow;
                d.margin.right = margin_r_num;
                d.content.width = w;
            }
            (w, Some(_), None) if w != 0.0 => { //means our margin right is auto
                d.margin.right = underflow;
                d.margin.left = margin_l_num;
                d.content.width = w;
            }
            (w, None, None) if w != 0.0 => { //means both right & left are auto
                d.margin.left = underflow / 2.0;
                d.margin.right = underflow / 2.0;
                d.content.width = w;
            }
            (_, _, _) => {  //our values are over constrained 
                d.margin.right = margin_r_num + underflow;
                d.margin.left = margin_l_num;
                d.content.width = width;
            }
        }
    }

    //ANCHOR Calculate
    //method allow us to position current box below the previous boxs in container by updating height
    //its sort of inverse to the one we did above called calculate inline position
    fn calculate_position(&mut self, b_box: Dimensions) {
        let style = self.styled_node;
        let d = &mut self.dimensions;

        //Get these values or else set them to 0.0
        d.margin.top = style.num_or("margin-top", 0.0);
        d.margin.bottom = style.num_or("margin-bottom", 0.0);
        d.border.top = style.num_or("margin-top-width", 0.0);
        d.border.bottom = style.num_or("margin-bottom-width", 0.0);
        d.padding.top = style.num_or("padding-top", 0.0);
        d.padding.bottom = style.num_or("padding-top", 0.0);

        d.content.x = b_box.content.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y = b_box.content.height + b_box.content.y + d.margin.top + d.border.top + d.padding.top;    
    }

    //Use this method to find the style nodes height value if it does exist
    fn calculate_height(&mut self) {
        self.styled_node.value("height").map_or((), |h| match **h {
            Value::Length(n, _) => self.dimensions.content.height = n, //check if Value::Length comes back with f32
            _ => {}
        })
    }

    //ANCHOR Layout
    //Will lay out current child nodes & adjust them based on height
    fn layout_children(&mut self) {
        let d = &mut self.dimensions;
        let mut max_child_height = 0.0;

        let mut prevBoxType = BoxType::Block; //by default to block

        for child in &mut self.children {
            match prevBoxType {
                BoxType::InlineBlock => match child.box_type {
                    BoxType::Block => {
                        d.content.height += max_child_height;
                        d.current.x = 0.0;
                    }
                    _ => {},
                },
                _ => {}
            }

            child.layout(*d);
            let new_height = child.dimensions.margin_box().height;

            if new_height > max_child_height {
                max_child_height = new_height;
            }

            match child.box_type {
                BoxType::Block => d.content.height += child.dimensions.margin_box().height,
                BoxType::InlineBlock => {
                    d.current.x += child.dimensions.margin_box().width;

                    if d.current.x > d.content.width {
                        d.content.height += max_child_height;
                        d.current.x = 0.0;
                        child.layout(*d); // will relayout our child
                        d.current.x += child.dimensions.margin_box().width;
                    }
                }
                _ => {}
            }

            //Will clone our child and make it next previous box
            prevBoxType = child.box_type.clone();
        }
    }
}

//Debug for layout box
impl<'a> fmt::Debug for LayoutBox<'a> { 
    fn fmt(&self, f: &mut fmt::Formatter)  -> fmt::Result  {
        write!(f, "type:\n  {:?}\n{:?}\n", self.box_type, self.dimensions)
    }
}

//NOTE: Dimensions
impl Dimensions {
    //Will update the content size to include the paddings
    fn padding_box(&self) -> Rectangle {
        self.content.expanded(self.padding)
    }

    //Will update the content size to include our borders
    pub fn border_box(&self) -> Rectangle {
        self.padding_box().expanded(self.border)
    }

    //Will update content size to include our margins
    fn margin_box(&self) -> Rectangle {
        self.border_box().expanded(self.margin)
    }
}

//Debug for Dimensions
impl fmt::Debug for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            "content:\n  {:?}\npadding:\n  {:?}\nborder:\n  {:?}\nmargin:\n  {:?}",
            self.content,
            self.padding,
            self.border,
            self.margin
        )
    }
} 

//NOTE: Rectange
impl Rectangle {
    //Allow us to expand a rectange with given set of dimentions, e: edges sizes we want to epand by
    fn expanded(&self, e: EdgeSizes) -> Rectangle {
        Rectangle {
            x: self.x - e.left,
            y: self.y - e.top,
            width: self.width + e.left + e.right,
            height: self.height + e.top + e.bottom,
        }
    }
}

//Debug for Rectangle
impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, w: {}, h: {}", self.x, self.y, self.width, self.height)
    }
}

//Debug for EdgeSizes
impl fmt::Debug for EdgeSizes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "l: {}, r: {}, top: {}, bottom: {}", self.left, self.right, self.top, self.bottom)
    }
}

//Debug for BoxType
impl fmt::Debug for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_type = match *self {
            BoxType::Block => "block",
            BoxType::Inline => "inline",
            BoxType::InlineBlock => "inline-block",
            BoxType::Anonymous => "anonymous",
        };

        write!(f, "{}", display_type)
    }
}

//NOTE: Helper Function
//Get absolute number, s_node: style node, b_box: border box (the current box), prop: the selector we passing through  
fn get_absolute_num(s_node: &StyledNode, b_box: Dimensions, prop: &str) -> Option<f32> {
    match s_node.value(prop) {
        Some(ref v) => match ***v { //Dereference it 3 times
            Value::Length(l, ref u) => match *u {
                Unit::Px => Some(l),                                    //Pixel
                Unit::Pct => Some(l * b_box.content.width / 100.0),     //Percent
                _ => panic!("Unimplemented css lenght unit"),
            } ,
            _ => None,
        }
        None => None,
    }
}

//This is like an entry point to our layout tree, the root is the root of the styletree & containing block is the window or the viewport
//Takes root stylenode and containing block which is dimensions
pub fn layout_tree<'a> (
    root: &'a StyledNode<'a>,
    mut containing_block: Dimensions,
) -> LayoutBox<'a> {
    containing_block.content.height = 0.0; //expect it to start with 0

    let mut root_box = build_layout_tree(root);
    root_box.layout(containing_block);
    return root_box;
}

//Will recursively build our layout tree given our style tree
//the node we're given is the current style node thats being laidout 
fn build_layout_tree<'a>(node: &'a StyledNode) -> LayoutBox<'a> {
    let mut layout_node = LayoutBox::new(
        match node.get_display() {
            Display::Block => BoxType::Block,
            Display::Inline => BoxType::Inline,
            Display::InlineBlock => BoxType::InlineBlock,
            Display::None => BoxType::Anonymous,
        },
        node,
    );

    for child in &node.children {
        match child.get_display() {
            Display::Block => layout_node.children.push(build_layout_tree(child)),
            Display::Inline => layout_node.children.push(build_layout_tree(child)),
            Display::InlineBlock => layout_node.children.push(build_layout_tree(child)),
            Display::None => {}
        }
    }
    layout_node
}

//start with our root node, level: the area of the tree we're currently on 
//recursively pretty print the children 
pub fn pretty_print<'a>(n: &'a LayoutBox, level: usize) {
    println!("{}{:?}\n", level, n); //the passed ones

    for child in n.children.iter() {    //the children ones
        pretty_print(&child, level + 1);
    }
}