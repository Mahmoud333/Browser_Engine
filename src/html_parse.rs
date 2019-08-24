//2
/* HTML has its unique parsing algorithm unlike most parsers for most programming languages and file format
 * The HTML parsing algorithm does not reject invalid input, instead it uses very specific error handling instructions
 * This way different web browsers will be able to agree how website would look like
 * Also bec. non conforming HTML has been supported since early days of web its now used in large amount of web pages that exist
 */
use crate::dom::{AttrMap, ElementData, Node, NodeType};

use std::iter::Peekable;
use std::str::Chars;

// NOTE 1- Data Structure
//Has lifetime of 'a, 
pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>, // A peekable list of charachters
    node_q: Vec<String>, 
    //Peakable data type is an iterator type it iterates over option types and tells whether the option has some or none 
    //In this case bec. we are iterating over characters, Its like a character stream 
    //Our Node Q is like adding our characters into strings and then passing them into a vector
}

// NOTE 2- Implement Methods For The DataStructure
impl<'a> HtmlParser<'a> {
    pub fn new(full_html: &str) -> HtmlParser {
        HtmlParser {
            chars: full_html.chars().peekable(), //take string, convert it to peekable iterator of characthers
            node_q: Vec::new(), //empty vector
        }
    }

    //Main entry point for our HTML parser
    //HTML can accept inavalid structure and invalid syntax 
    //Rather than throw error on invalid syntax, we want them to correct that invalid syntax
    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.chars.peek().is_some() {  //Check if we have some data, will return true if Option is some and false if its not 
            self.consume_while(char::is_whitespace);    //Check if char is whitespace, if its consume/remove it
            if self.chars.peek().map_or(false, |c| *c == '<' ) { //if c is opening tag
                self.chars.next();          //move forward
                
                if self.chars.peek().map_or(false, |c| *c == '/') { //If its backslash
                    self.chars.next();      //move forward
                    self.consume_while(char::is_whitespace);  //Check if char is whitespace, if its consume/remove it

                    //Check if we have valid tagname inside HTML as long its bacis word its valid tag name
                    let close_tag_name = self.consume_while(is_valid_tag_name);

                    self.consume_while(|x| x != '>');
                    self.chars.next();

                    self.node_q.push(close_tag_name);
                    break;
                
                } else if self.chars.peek().map_or(false, |c| *c == '!') { //means we looking at comment
                    self.chars.next();
                    nodes.push(self.parse_comment_node());
                
                } else {    //if we dont have appropriate looking HTML for a comment or for a normal element
                    let mut node = self.parse_node();
                    //Check what length of our node are
                    let insert_index = nodes.len();

                    //Match on node type
                    match node.node_type {
                        //Element type
                        NodeType::Element(ref e) => if self.node_q.len() > 0 {
                            let assumed_tag = self.node_q.remove(0); //remove front of our node q

                            if e.tag_name != assumed_tag {      //check if they aren't the same
                                nodes.append(&mut node.children); //append node children to our nodes
                                self.node_q.insert(0, assumed_tag); //insert our assumed tag into our node q
                            }
                        },
                        _ => {},
                    }

                    nodes.insert(insert_index, node);
                } 
            } else {    //if dont have '>', we're not running into an HTML element instead we're running into block of text
                nodes.push(self.parse_text_node());
            }
        }

        nodes
    }

    fn parse_node(&mut self) -> Node {
        let tagname = self.consume_while(is_valid_tag_name); //check tag name valid
        let attributes = self.parse_attributes();

        //Create new element data
        let elem = ElementData::new(tagname, attributes);   //Main
        let children = self.parse_nodes();                  //Children
        Node::new(NodeType::Element(elem), children)        //Return it
    }

    fn parse_text_node(&mut self) -> Node {
        let mut text_content = String::new();   //Empty String

        //Iterator thorugh all characters & peek inside checking if character is NOT EQUAL to '<' Open HTML tag
        while self.chars.peek().map_or(false, |c| *c != '<' ) {
            let whitespace = self.consume_while(char::is_whitespace);   //Consume all whitespace
            if whitespace.len() > 0 {       //if white space more than 0
                text_content.push(' ');     //take & push 1 single space
                //"I   Love   You   Man!" -> "I Love You Man!" | will delete spaces & make it just 1
            }
            //Check where x != whitespace & != to opening HTML tag, oush that content into our string 
            let text_part = self.consume_while(|x| !x.is_whitespace() && x != '<');
            text_content.push_str(&text_part);
        }
        //Create/Return Text type node
        Node::new(NodeType::Text(text_content), Vec::new())
    }

    fn parse_comment_node(&mut self) -> Node {
        let mut comment_content = String::new();

        //Checking if we have acutal HTML comment that looks correct
        //like so <!-- This is a comment -->

        if self.chars.peek().map_or(false, |c| *c == '-') { //if followed by 1st dash
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '-') {//if followed by 2nd dash
                self.chars.next();
            } else {
                self.consume_while(|c| c != '>'); //consume all string until hit closing tag
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            }
        } else {    //missing dashes, exit immediatley
            self.consume_while(|c| c != '>');
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }

        //If we have an error, like we have opening tag then immediatley have closing tag
        if self.chars.peek().map_or(false, |c| *c == '>') {
            self.chars.next();
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }

        //If after opening tag we have 1 dash & then have closing tag
        //Take everything after that and slam it into our string as well
        if self.chars.peek().map_or(false, |c| *c == '-') {
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '>' ) {
                self.chars.next();
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            } else {    //if not
                //Put dash 
                comment_content.push('-');
            }
        }

        //Check different error cases for our comments, all else statements create proper error tag
        //with the elements that are missing from the if statements
        while self.chars.peek().is_some() {
            comment_content.push_str(&self.consume_while(|c| c != '<' && c != '-'));
            if self.chars.peek().map_or(false, |c| *c == '<') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '-') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.consume_while(|c| c != '>');

                            return Node::new(NodeType::Comment(String::from("")), Vec::new());
                        } else {
                            comment_content.push_str("<!-");
                        }
                    } else if self.chars.peek().map_or(false, |c| *c == ' ') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.chars.next();
                            if self.chars.peek().map_or(false, |c| *c == '-') {
                                self.chars.next();
                                if self.chars.peek().map_or(false, |c| *c == '-') {
                                    self.chars.next();
                                    if self.chars.peek().map_or(false, |c| *c == '>') {
                                        self.chars.next();
                                        return Node::new(
                                            NodeType::Comment(String::from("")),
                                            Vec::new(),
                                        );
                                    } else {
                                        comment_content.push_str("<! --");
                                    }
                                } else {
                                    comment_content.push_str("<! -");
                                }
                            } else {
                                comment_content.push_str("<! ");
                            }
                        }
                    } else {
                        comment_content.push_str("<!");
                    }
                } else {
                    comment_content.push('<');
                }
            } else if self.chars.peek().map_or(false, |c| *c == '-') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '-') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '>') {
                        self.chars.next();
                        break;
                    } else {
                        comment_content.push_str("--");
                    }
                } else {
                    comment_content.push('-');
                }
            }
        }

        Node::new(NodeType::Comment(comment_content), Vec::new())
    }

    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = AttrMap::new();

        //make sure our charachter is not a closing tag ">"
        while self.chars.peek().map_or(false, |c| *c != '>') {
            self.consume_while(char::is_whitespace);
            //When our attribute name is valid, lowercase it  
            let name = self.consume_while(|c| is_valid_attr_name(c)).to_lowercase();
            self.consume_while(char::is_whitespace);

            //Like class = Header,
            let value = if self.chars.peek().map_or(false, |c| *c == '=') {
                self.chars.next();
                self.consume_while(char::is_whitespace);
                let s = self.parse_attr_value(); //parse that header word
                self.consume_while(|c| !c.is_whitespace() && c != '>'); //consume until closing tag
                self.consume_while(char::is_whitespace);
                s
            } else {
                "".to_string()
            };
            attributes.insert(name, value); 
            //insert name & value into our AttrMap which was our custom HashMap<String, String>
        }
        //Keep iterating through our characters
        self.chars.next();

        attributes
    }

    fn parse_attr_value(&mut self) -> String {
        self.consume_while(char::is_whitespace);

        let result = match self.chars.peek() {
            Some(&c) if c == '"' || c == '\'' => {
                self.chars.next();  //iterate by 1 
                let ret = self.consume_while(|x| x != c); //Consume all of the text until we get another " or '
                self.chars.next();
                ret //for result
            }
            _ => self.consume_while(is_valid_attr_value),
        };
        result
    }

    //Takes condition which is a function 
    fn consume_while<F>(&mut self, condition: F) -> String 
    where 
        F: Fn(char) -> bool, //Must take character and output bool
    {
        let mut result = String::new();

        //Loop through characters & 'map or', we want to check if this condition was false
        //& if its not then return default values, in this case its white-space character
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            //Take result & push it on to our characters. then we go to next character and unwrap it
            result.push(self.chars.next().unwrap());
        }

        result
    }
}

// NOTE 3- Helper Methods
//Helper functions to parse HTML characters

fn is_valid_tag_name(ch: char) -> bool {
    ch.is_digit(36)
}

fn is_valid_attr_name(c: char) -> bool {
    !is_excluded_name(c) && !is_control(c)
}

//looking at hexadecimal characters to see if we have non ASCII characters
fn is_control(ch: char) -> bool {
    match ch {
        '\u{007F}' => true,
        c if c >= '\u{0000}' && c <= '\u{001F}' => true,
        c if c >= '\u{0080}' && c <= '\u{009F}' => true,
        _ => false,
    }
}

fn is_excluded_name(c: char) -> bool {
    match c {
        ' ' | '"' | '\'' | '>' | '/' | '=' => true,
        _ => false,
    }
}

fn is_valid_attr_value(c: char) -> bool {
    match c {
        ' ' | '"' | '\'' | '=' | '<' | '>' | '`' => false,
        _ => true,
    }
}