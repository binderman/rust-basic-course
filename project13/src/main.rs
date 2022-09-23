#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum Core {
    I3,
    I5,
    I7,
}

fn old_main() {
    let core1 = Core::I3;

    match core1 {
        Core::I3 => println!("Meh..."),
        Core::I5 => println!("Ok."),
        Core::I7 => println!("Uhu!"),
    }

    let core2 = Core::I5;
    let core3 = Core::I7;

    println!("{:?}", core2);
    println!("{:?}", core3);
}


#[derive(Debug)]
enum AppleDevice {
    IPhone(String),
    IPad(String),
    AppleWatch(String),
    AppleTV,
}

fn old_old_main() {
    let iphone = AppleDevice::IPhone(String::from("X"));
    let ipad = AppleDevice::IPad(String::from("Pro"));
    let watch = AppleDevice::AppleWatch(String::from("Series 5"));
    let tv = AppleDevice::AppleTV;

    println!("{:?}", iphone);
    println!("{:?}", ipad);
    println!("{:?}", watch);
    println!("{:?}", tv);
}

#[derive(Debug)]
enum SpriteAction {
    Delete,
    Move { x: i32, y: i32 },
    Scale { x: f32, y: f32 },
    Rotate { degrees: f32 },
    Color { r: u8, g: u8, b: u8 },
}

impl SpriteAction {
    fn execute(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    let action1 = SpriteAction::Move { x: 10, y: 20 };
    let action2 = SpriteAction::Scale { x: 0.5, y: 0.5 };
    let action3 = SpriteAction::Rotate { degrees: 45.0 };
    let action4 = SpriteAction::Color { r: 255, g: 255, b: 255 };
    let action5 = SpriteAction::Delete;
    action1.execute();
}



