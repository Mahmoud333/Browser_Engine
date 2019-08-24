//implement our Command and Rendered libraries
//walk through Our Layout tree and build a display list, this is a list of graphics operations that allow us to draw things
//For instance one for render some text, another one for draw rectange, we put these commands into our display list then we can use that list to search out items that might
//be completely covered up by later operations & remove them to eliminate wasteful rendering
//You can also modify & reuse display list incase where u know only certain items have been changed, libraries like react that use Dom diffing and take advantage of this type of behavior
//1st type of our display list is going to be our command module

use crate::css::{Color, Value};
use crate::layout::{LayoutBox, Rectangle};
use std::fmt;   //So we can implement debugging

pub type DisplayList = Vec<DisplayCommand>;

pub enum DisplayCommand {
    SolidRectangle(Color, Rectangle), 
    //Color and actual area of the rectange
    //We could add content box and put a string in that but our current css parser & current HTML parser only parses specific elements
}

//idea of this func is to traverse our entire layout tree and create our display list
pub fn build_display_commands(root: &LayoutBox) -> DisplayList {
    let mut commands = Vec::new();

    render_layout_box(&mut commands, root);
    commands
}

//reference to our LayoutBox
fn render_layout_box(commands: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(commands, layout_box);
    render_borders(commands, layout_box);

    //Iterate through our childs and call the function recursively
    for child in &layout_box.children {
        render_layout_box(commands, child);
    }
}

fn render_background(commands: &mut DisplayList, layout_box: &LayoutBox) {
    //our layout_box and then background color selector
    //our map closure will take color & push it into our commands our DisplayCommand SolidRectange with color and border_box of layout_box
    get_color(layout_box, "background-color").map(|color| {
        commands.push(DisplayCommand::SolidRectangle(    //commands is our vector display commands
            color,
            layout_box.dimensions.border_box(),
        ))
    });
}

//Input: layout_box & name which is slice of string , Output: Option Color
//This func assure we get color value for our selector & ensure we get valid color value for our selector
fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.styled_node.value(name) { //get value from selector from the styled node 
        Some(v) => match **v { //found Some value then de-refrence it 
            Value::Color(ref c) => return Some(c.clone()),  //if type color, get color out & put it inside option and clone it
            _ => return None,
        },
        None => return None,
    }
}

//Input: DisplayList, refrence to out layoutBox
fn render_borders(commands: &mut DisplayList, layout_box: &LayoutBox) {
    //create our color with match, for drawing the border color of our box
    let color = match get_color(layout_box, "border-color") { 
        Some(color) => color,
        _ => return,
    };

    let d = &layout_box.dimensions;   //d is our layer box so the actual element that we're looking at 
    let border_box = d.border_box(); //the border box which is parent box of this particular element, thats outside this element

    //push DisplayCommand SolidRectangle
    commands.push(DisplayCommand::SolidRectangle(
        color.clone(),
        Rectangle {      //instantiate a rectange
            x: border_box.x,
            y: border_box.y,
            width: d.border.left,
            height: border_box.height,
        },
    ));

    commands.push(DisplayCommand::SolidRectangle(
        color.clone(),
        Rectangle {      //instantiate a rectangle
            x: border_box.x + border_box.width - d.border.right,
            y: border_box.y,
            width: d.border.right,
            height: border_box.height,
        },
    ));

    commands.push(DisplayCommand::SolidRectangle(
        color.clone(),
        Rectangle {      //instantiate a rectange
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: d.border.top,
        },
    ));

    commands.push(DisplayCommand::SolidRectangle(
        color.clone(),
        Rectangle {      //instantiate a rectange
            x: border_box.x,
            y: border_box.y + border_box.height - d.border.bottom,
            width: border_box.width,
            height: d.border.bottom,
        },
    ));
}

//Debug 
impl fmt::Debug for DisplayCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisplayCommand::SolidRectangle(ref color, ref rectange) =>  write!(f, "{:?} {:?}", color, rectange),
        }
    }
}