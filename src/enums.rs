use std::f64::consts::PI;

// automatic discriminator (Triangle = 1, Rectangle = 2, Circle = 3)
#[derive(Debug)]
enum Shapes {
    Triangle {base: f64, height: f64}, // struct example
    Rectangle (f64, f64), // tuple example
    Base // unit example
}

impl Shapes {
    fn area(&self) -> Option<f64> {
        match *self {
            Shapes::Triangle{base, height} => Some((base * height)/2f64),
            Shapes::Rectangle(width, height) => Some(width * height),
            Shapes::Base => None,
        }
    }
}

pub fn enums_demo() {
    println!("Enums demo begins here...");

    // instantiate enums
    let triangle = Shapes::Triangle{base: 2f64, height: 3f64};
    let rectangle = Shapes::Rectangle(2f64, 3f64);
    let base = Shapes::Base;

    // print enum variable
    println!("Triangle = {:?}", triangle);
    println!("Rectangle = {:?}", rectangle);
    println!("Base = {:?}", base);

    // print area
    print_area_with_match("Triangle", &triangle);
    print_area_with_if("Rectangle", &rectangle);
    print_area_with_match("Base", &base);
}

fn print_area_with_match (name: &str, shape: &Shapes) {
    let area = shape.area();
    // use Option to match values in safe manner in the absence of null keyword
    match area {
        Some(val) => println!("{} area = {} sq.units", name, val),
        None => println!("{} area not found", name)
    }
}

fn print_area_with_if (name: &str, shape: &Shapes) {
    let area = shape.area();
    // use let else to return default if value doesn't exist
    let Some(_val) = area else {
        println!("{} area not found", name);
        return;
    };

    // can use if let instead of match when there is only one case to check
    if let Some(val) = area {
        println!("{} area = {} sq.units", name, val);
    }
}
