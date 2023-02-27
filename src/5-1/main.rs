struct Color(u16, u16, u16);

struct Values {
    value1: u32,
    value2: u32,
    value3: u32,
    value4: u32,
}

fn main() {
    let v1 = Values {
        value1: 1,
        value2: 2,
        value3: 3,
        value4: 4,
    };

    let v2 = Values { value1: 100, ..v1 };

    println!(
        "value1: {}, value2: {}, value3: {}, value4: {}",
        v1.value1, v1.value2, v1.value3, v1.value4
    );
    println!(
        "value1: {}, value2: {}, value3: {}, value4: {}",
        v2.value1, v2.value2, v2.value3, v2.value4
    );

    let color = Color(10, 100, 200);
    println!("1st: {}, 2nd: {}, 3rd: {}", color.0, color.1, color.2);
    let Color(r, g, b) = color;
    println!("1st: {}, 2nd: {}, 3rd: {}", r, g, b);
    println!("1st: {}, 2nd: {}, 3rd: {}", color.0, color.1, color.2);
}
