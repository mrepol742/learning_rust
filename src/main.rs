#![allow(dead_code)]
#![allow(unused)]

enum Device {
    Iphone(i8),
    Samsung(String),
}

enum Type {
    Peck(i32),
    VeryPeck(i32),
}

enum Level {
    Easy,
    Hard,
    Extreme,
    Custom (Type),
}

fn main() {
    
    use crate::Device;
    use crate::Type::{Peck, VeryPeck};
    use crate::Level::Custom;
    use crate::Level::Easy;
    
    let phone: Device = Device::Iphone(17);
    let stage: Level = Easy;
    let another_level: Level = Custom(VeryPeck(9_0));
    
    
    check(stage);
    check(another_level);
    
    match phone {
        Device::Iphone(x) => println!("Phone:: {}", x),
        Device::Samsung(x) => println!("Phone:: {}", x)
    }
}

fn check (stage: Level) {
    match stage {
        Level::Easy => println!("Level:: Easy"),
        Level::Custom(option) => {
            match option {
                Type::Peck(x) => println!("Custom:: Peck:: {}", x),
                Type::VeryPeck(x) => println!("Custom:: VeryPeck:: {}", x)
            }
        },
        _=> println!("Other:: Hard || Extreme")
    } 
}