// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn main() {
//     let dimensions: (u32, u32) = (20, 40);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(dimensions)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
// #[derive(Debug)]
// struct Rectangle {
//     height: u32,
//     width: u32
// }

// fn main() {
//     let rect = Rectangle {
//         height: 20,
//         width: 40
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect)
//     );

//     dbg!(rect);
// }

// fn area(rect: &Rectangle) -> u32 {
//     rect.height * rect.width
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
