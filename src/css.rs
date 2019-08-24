/* CSS is a series of rules it include one or more selectors which are separated by commas followed by series of declarations which are enclosed in braces
 * these selectors can be simple selector or chain of selectors joined by 'Combinators'
 * our implementation is only going to support simple selection & not going to support cascading, cascading is when we can have multiple CSS sheets
 * and they can override one another and bec. its fairy difficult function to put into our browser engine we're going to just ignore it for now
 * in our browser engine a simple selector can include a tag_name an Id prefixed by number sign and any number of class names prefixed by period or combination
 * of the above if tag name is empty or has asterix in it then it is a universal selector which will match it with any tag 
 * there are many types of selectors in CSS particular in CSS3 but we will deal with these for now
 */

use std::fmt;   //bec. we gonna implement debug for some of our data structures
use std::default::Default; //allow us to put default values inside of our data structures

// NOTE 1- Data Structures

pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>,
    //simple: to account for simple selectors
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    //Selectors can have multiple classes on them & we want them to gave single id as well as their tag name
}

pub struct Declaration {
    pub property: String,
    pub value: Value,
    //This is similar to like a HashMap u have property that u want to affect & then the value u want to set in that property
}

pub enum Value {
    Color(Color),
    Length(f32, Unit),
    Other(String),
    //In our CSS implementation our values can be Colors, Length, Other
}

pub enum Unit {
    Em, //calculated or inherited font size
    Ex, //the height of fonts x character
    Ch, //the width of a fonts of o character
    Rem,//font size of the root element
    Vh, //100 the height of a viewport
    Vw, //is the 100th the width of the viewport
    Vmin, //is 100th the smallest side  
    Vmax, //is 100th the largest side 
    Px, //Pixel
    Mm, //Millimeter
    Q,  //1/4 of a millimeter
    Cm, //centimeter
    In, //1/32 of an inch
    Pt, //point
    Pc, //11 points or pica
    Pct,//percenage
}

#[derive(Clone)]
pub struct Color {
    pub r: f32, //Red
    pub g: f32, //Green
    pub b: f32, //Blue
    pub a: f32, //Alpha
}

// NOTE 2- Implement Methods For These DataStructures

impl Stylesheet {
    pub fn new (rules: Vec<Rule>) -> Stylesheet {
        Stylesheet { rules }
    }
}
impl Default for Stylesheet {   //Implement Deafult trait so we can put default values in it
    fn default() -> Self { //these 'Self' refer to Stylesheet, we will output stylesheet object with a empty vector for the rule
        Stylesheet{ rules: Vec::new() }
    }
}
impl fmt::Debug for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_result = String::new();
        for rule in &self.rules { //self.rules is vector of rules
            if rule_result.len() > 0 {  //Add new line if its the second line or more
                rule_result.push_str("\n\n");
            }
            rule_result.push_str(&format!("{:?}", rule)); //add our rule debug to string
        }
        write!(f, "{}", rule_result)
    }
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
        Rule {
            selectors,
            declarations,
        }
    }
}
impl Default for Rule {   //Implement Deafult trait so we can put default values in it
    fn default() -> Self {
        Rule {
            selectors: Vec::new(),
            declarations: Vec::new(),
        }
    }
}
impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sel_result = String::new();
        let mut decl_result = String::new();
        let tab = "     ";

        for selector in &self.selectors {
            if sel_result.len() > 0 {   //if second line or more then add comma and space
                sel_result.push_str(", ");
            }
            sel_result.push_str(&format!("{:?}", selector)); //add our selector debug to string
        }

        for declaration in &self.declarations {
            decl_result.push_str(tab);   //add tab between each of the declarations
            decl_result.push_str(&format!("{:?}", declaration));  //add our declaration debug to string
            decl_result.push('\n');  //add new line after it
        }

        write!(f, "{} {{\n{}}}", sel_result, decl_result)
    }
}

impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinators: Vec<char>) -> Selector {
        Selector {
            simple,
            combinators,
        }
    }
}
impl Default for Selector {   //Implement Deafult trait so we can put default values in it
    fn default() -> Self {
        Selector {
            simple: Vec::new(),
            combinators: Vec::new(),
        }
    }
}
impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for sel in &self.simple {
            if result.len() > 0 {   //if second selector or more add commna and whitespace
                result.push_str(", ");
            }
            result.push_str(&format!("{:?}", sel)); //add it to the string
        }

        write!(f, "{}", result) 
    }
}

impl SimpleSelector {
    pub fn new(tag_name: Option<String>, id: Option<String>, classes: Vec<String>) -> SimpleSelector {
        SimpleSelector {
            tag_name,
            id,
            classes,
        }
    }
}
impl Default for SimpleSelector {   //Implement Deafult trait so we can put default values in it
    fn default() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
        }
    }
}
impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new(); //empty string

        match self.tag_name {
            Some(ref t) => result.push_str(t),  //add the tag name
            None => {}
        }

        match self.id {
            Some(ref s) => {    //add id with '#' infront of it
                result.push('#');
                result.push_str(s);
            },
            None => {}
        }

        for class in &self.classes { //loop through classes
            result.push('.');       //add . infront of the class
            result.push_str(class); //add the class to our string
        }

        write!(f, "{}", result)
    }
} 

impl Declaration {
    pub fn new(property: String, value: Value) -> Declaration {
        Declaration {
            property,
            value,
        }
    }
}
impl Default for Declaration {    //Implement Deafult trait so we can put default values in it
    fn default() -> Self {
        Declaration {
            property: String::from(""),
            value: Value::Other(String::from(""))
        }
    }
}
impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.property, self.value)
    }
}

impl fmt::Debug for Value { //Debug for our types 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Color(ref c) => write!(f, "{:?}", c),
            Value::Length(l, _) => write!(f, "{:?}", l),
            Value::Other(ref s) => write!(f, "{:?}", s),
        }
    }
}


impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}
impl Default for Color {  //Implement Deafult trait so we can put default values in it
    fn default() -> Self {  //return white
        Color::new(1.0, 1.0, 1.0, 1.0)
    }
}
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {}, g: {}, b: {}, a: {}", self.r, self.g, self.b, self.a)
    }
}





// NOTE 3- Helper Methods