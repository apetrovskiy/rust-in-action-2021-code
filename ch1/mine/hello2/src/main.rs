fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grus Gott!";
    let japan = "1111";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
         println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
