mod primitives;
mod sky;

fn main() {
    let mut the_sky = sky::Sky::new();
    let tile = primitives::Rectangle::new(358.0, -88.0, 348.0, -84.0, Option::Some(4));
    the_sky.add_tile(tile);
    let tiles = the_sky.tiles();
    for t in tiles {
        println!("{:?}", t);
    }
}
