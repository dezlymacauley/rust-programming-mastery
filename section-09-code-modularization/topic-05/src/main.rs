mod file_1;

struct Rectangle {
    length: i32,
    width: i32
}

fn main() {
    let rect1: Rectangle = Rectangle {
        length: 5,
        width: 10
    };

    let area_rect1 = file_1::rect_area(&rect1.length, &rect1.width);
}
