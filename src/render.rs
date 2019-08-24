use gfx;
use gfx_window_glutin;
use glutin;
use gfx_text;

use gfx::Factory;   //help us to build our vertice factory
use gfx::traits::FactoryExt; //give us few helper functions
use gfx::Device;    //allow us to gain access to the window and stuff

use crate::layout;
use crate::command::DisplayCommand;


//Types
pub type DepthFormat = gfx::format::DepthStencil; //binds to our GFK format depthStencil
pub type ColorFormat = gfx::format::Rgba8;

const SCREEN_WIDTH: usize = 1024;
const SCREEN_HEIGHT: usize = 768;
const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0]; //This is white (R G B A)
//use clear color to make sure any of elements that do not appear will end up as white
//& any of the backgrounds that we're not rendering will be white
//& if we want to rerender our picture it will show white


//This macro Allow us to create two objects,
gfx_defines! {
    vertex Vertex { //create vertex that have position field & color field
        pos: [f32; 2] = "a_Pos", //bind it to-from solid.glslv
        color: [f32; 3] = "a_Color", //bind it to-from solid.glslv
    }

    pipeline pipe { //have video buffer, output
        vbuf: gfx::VertexBuffer<Vertex> = (), //points to vertex buffer with vertex inside of it
        out: gfx::RenderTarget<ColorFormat> = "Target0", //the output points to render target color format color zero
    }
}


#[derive(Copy, Clone)]
struct RenderText<'a> {
    text: &'a str,
    position: [i32; 2], //x and y
    color: [f32; 4], //R G B A
}

//Output vector new bec. we r not actually rendering text with our HTML document, its like placeholder function
fn render_texts(command_list: &[DisplayCommand]) -> Vec<RenderText> {
    Vec::new()
}

//Output tuple of vector vertex and vector u16
fn render_commands(command_list: &[DisplayCommand]) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut index_data = Vec::new();
    let mut rect_num: u16 = 0;

    for command in command_list {   //iterate through our command list
        match *command {    //match at the de-referred command
            DisplayCommand::SolidRectangle(ref color, ref rect) => { //if DisplayCommand::SolidRectangle pull out Color and Rect
                let colors = [color.r, color.g, color.b];

                let mut v = render_rectange(&colors, rect);
                vertices.append(&mut v);

                let index_base: u16 = rect_num * 4;
                index_data.append(&mut vec![
                    index_base,     //0
                    index_base + 1, //1
                    index_base + 2, //2
                    index_base + 2, //2
                    index_base + 3, //3
                    index_base,     //0
                ]);
                rect_num += 1;
            }
        }
    }
    return (vertices, index_data)
}

fn render_rectange(c: &[f32; 3], rect: &layout::Rectangle) -> Vec<Vertex> { //c = color
    let (x, y, h, w) = transform_rectange(rect); //will allow us to transform HTML coordinates to OpenGl coordinates
    let vertices = vec![    //Create our vector of vertices
        Vertex {    //Bottom Right Corner
            pos: [x + w, y],
            color: *c,
        },
        Vertex {    //Bottom Left Corner
            pos: [x, y],
            color: *c,
        },
        Vertex {    //Top Right Corner
            pos: [x, y + h],
            color: *c,
        },
        Vertex {    //Top Left Corner
            pos: [x + w, y + h],
            color: *c,
        },
    ];
    vertices
}

//Takes layout rectange and outputs 4 f32 inside a tuple
fn transform_rectange(rect: &layout::Rectangle) -> (f32, f32, f32, f32) {
    let w = rect.width / SCREEN_WIDTH as f32 * 2.0;
    let h = rect.height / SCREEN_HEIGHT as f32 * 2.0;
    let x = rect.x / SCREEN_WIDTH as f32 * 2.0 - 1.0;
    let y = -(rect.y / SCREEN_HEIGHT as f32 * 2.0 - 1.0 + h);

    (x, y, h, w)
}

//Function that gets called many times a second to render a screen properly
pub fn render_loop(command_list: &[DisplayCommand]) {
    //window builder with title "Browser" with dimensions SCREEN_WIDTH & SCREEN_HEIGHT
    let builder = glutin::WindowBuilder::new()
        .with_title(String::from("Browser"))
        .with_dimensions(glutin::dpi::LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64)); //SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        //.with_vsync(); my 2.0 became commented

    
    let events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new();

    //Initialize our window by passing the builder inside this func here 
    //it gives us: glutin window, GFX device GL device, factory factory and main_color, _main_depth
    //gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder); //was 1.0
    //became
    let (window, mut device, mut factory, main_color, _main_depth) = match gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop) {
        Ok((window, mut device, mut factory, main_color, _main_depth)) => (window, device, factory, main_color, _main_depth), //result,
        Err(e) => panic!("Problem error in render.rs: {:?}", e),
    };

    //from factory create GFX encoder 
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory   //pipe in our 2 shaders files
        .create_pipeline_simple(
            include_bytes!("../shaders/solid.glslv"),
            include_bytes!("../shaders/solid.glslf"),
            pipe::new(),
        )
        .unwrap();

    
    let (vertices, index_data) = render_commands(command_list); //render our commands
    let texts = render_texts(command_list); //render our text

    //these create a video buffer for us 
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, &index_data[..]);

    //pipe data
    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };
    
    //we want to create gfx test renderer
    //Became 2.0 
    let mut test_renderer = gfx_text::new(factory).unwrap();
    //Was 1.0
    //let mut test_renderer = gfx_text::new(factory).build().unwrap();


    //Became 2.0
    let mut continue_running = true;
    while continue_running { 
        events_loop.poll_events(|event| {   //iterate through events
            use glutin::{Event, WindowEvent, KeyboardInput, VirtualKeyCode};
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    //if user: 1- if they close the window or 2- hits escape key, then the actual program ends it
                    WindowEvent::CloseRequested |
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => continue_running = false,
                    _ => (),
                }
            }
        });

    //WAS 1.0
    //'main: loop { //create loop with name 'main'
        //for event in window.poll_events { //iterate through events
            //match event {
            //    //if got keyboardInput with VirtualKeyCode escape OR glutin Event Closed then break outside of this loop
            //    //if user hits escape key or if they close the window then the acutal program ends it
            //    glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => break 'main,
            //    _ => {}
            //}
        //}

        //rendering our text by iterating through our text then adding them
        //it granted we have no text to add aside from the title
        for text in &texts {
            test_renderer.add(text.text, text.position, text.color);
        }

        //clear out our gfx encoder and pass in our white color and data.out
        //Just incase if we have data on the window
        encoder.clear(&data.out, CLEAR_COLOR);

        //Draw our data
        encoder.draw(&slice, &pso, &data);
        test_renderer.draw(&mut encoder, &data.out); //if had text we would use test_renderer to draw that text

        //CleanUp
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}

impl gfx_core::factory::Factory<_> for  gfx_device_gl::factory::Factory {

}