use universe::universe::Universe;

fn main() {
    let universe = Universe::new(800, 600);
    println!("area: {}", universe.get_area());
}
