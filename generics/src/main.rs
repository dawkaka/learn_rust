fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['a', 'b', 'c', 'd'];
    let largest_n = largest(&number_list);
    let largest_c = largest(&char_list);
    println!(
        "The largest number and char are '{}' and '{}' respectively",
        largest_n, largest_c
    );

    let p_n = Point { x: 10, y: 1.10 };
    let p_c = Point { x: "hello", y: 'c' };
    let mx = p_n.mixup(p_c);
    println!("mixed points {:?}", mx);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for n in list {
        if n > largest {
            largest = n
        }
    }
    largest
}
#[derive(Debug)]
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, p: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: p.y }
    }
}
