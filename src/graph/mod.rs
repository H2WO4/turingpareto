use crate::list;

mod html2d;
mod html3d;

pub fn handle(level: &str) {
    if list::LEVELS_2D.contains(&level) {
        html2d::generate(level);
    } else if list::LEVELS_3D.contains(&level) {
        html3d::generate(level);
    } else {
        eprintln!("Unknown level ID!")
    }
}
