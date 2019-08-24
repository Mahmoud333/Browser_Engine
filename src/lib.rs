#[macro_use] //so we can use macros inside of the library
extern crate gfx;
extern crate gfx_text;
extern crate gfx_window_glutin;
extern crate glutin;

pub mod render;
pub mod command;
pub mod dom;    //Help us to parse the Dom or the document object model
pub mod html_parse;
pub mod css;
pub mod style;
pub mod layout;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

