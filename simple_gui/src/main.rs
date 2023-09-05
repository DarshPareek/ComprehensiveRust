// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label.chars().count()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let _ = buffer.write_str(&*self.label);
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.label.chars().count()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let bsepl : &str = "|  ";
        let bsepr : &str = "  |";
        let _ = buffer.write_str(bsepl);
        let _ = buffer.write_str(&*self.label.label);
        let _ = buffer.write_str(bsepr);
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        let mut ts : usize = 0;
        for i in 0..self.widgets.len(){
            ts += self.widgets[i].width();
        }
        ts
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let del : &str = "\n";
        let wsep : &str = "\n==========\n";
        let _ = buffer.write_str(wsep);
        let _ = buffer.write_str(&*self.title);
        let _ = buffer.write_str(wsep);
        let _ = buffer.write_str(del);
        for i in 0..self.widgets.len(){
            self.widgets[i].draw_into(buffer);
            let _ = buffer.write_str(del);
        }
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}
