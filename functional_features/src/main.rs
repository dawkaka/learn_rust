use std::thread;
#[derive(Debug, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, preference: Option<ShirtColor>) -> ShirtColor {
        preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };
    let user_pref_1 = Some(ShirtColor::Red);
    let user_pref_2: Option<ShirtColor> = None;
    let shirt_1 = inventory.giveaway(user_pref_1);
    let shirt_2 = inventory.giveaway(user_pref_2);
    println!("user preference {:?}: user got {:?}", user_pref_1, shirt_1);
    println!(
        "user 2 preference {:?}: user 2 got {:?}",
        user_pref_2, shirt_2
    );

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
