pub trait Draw {
    fn draw(&self);
}


pub struct Button {
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing button")
    }
}


pub struct TextField {
}

impl Draw for TextField {
    fn draw(&self) {
        println!("drawing textfield")
    }
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}



fn main() {
    let scr = Screen { components: vec![
        Box::new(Button{}),
        Box::new(TextField{}),
        ] };
    scr.run();
}
