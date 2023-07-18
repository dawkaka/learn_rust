use Crates_More::mix;
use Crates_More::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let ms = mix(red, yellow);
    println!("{:?}", ms)
}
