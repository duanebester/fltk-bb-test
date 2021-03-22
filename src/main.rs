use fltk::{app, button::*, frame::*, window::*};

fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 200, "");
    let mut button = Button::new(160, 210, 80, 40, "Click me!");
    button.set_color(Color::from_u32(0xffebee)); // You can use one of the provided colors in the fltk enums
    button.set_frame(FrameType::BorderFrame);
    button.set_down_frame(FrameType::FlatBox);
    button.clear_visible_focus();
    button.set_label_font(Font::Helvetica);
    window.end();
    window.show();
    button.set_callback(move || frame.set_label("Hello World!"));
    app.run().unwrap();
}