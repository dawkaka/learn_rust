pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

#[derive(Debug)]
struct Rectangle {
    height: usize,
    width: usize,
}

impl Rectangle {
    fn can_hold(&self, r2: &Rectangle) -> bool {
        self.height >= r2.height && self.width >= r2.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_holds_smaller() {
        let r1 = Rectangle {
            height: 8,
            width: 7,
        };
        let r2 = Rectangle {
            height: 5,
            width: 3,
        };
        assert!(r1.can_hold(&r2));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let r1 = Rectangle {
            height: 5,
            width: 3,
        };
        let r2 = Rectangle {
            height: 8,
            width: 10,
        };
        assert!(!r1.can_hold(&r2))
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
