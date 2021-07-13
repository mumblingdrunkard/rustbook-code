#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("(width1, height1) is: ({}, {})", width1, height1);
    println!(
        "The area of the rectangle is {}\n",
        area_two_parameters(width1, height1)
    );

    let rect1 = (30, 70);

    println!("rect1 is: {:?}", rect1);
    println!(
        "The area of the rectangle is {}\n",
        area_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 90,
    };

    println!("rect2 is: {:#?}", rect2);
    println!(
        "The area of the rectangle is {}\n",
        area_rectangle(&rect2)
    );
}

fn area_two_parameters(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
