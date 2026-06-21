use crate::list;

mod html2d;
mod html3d;

pub fn handle(level: &str, log: bool) {
    if list::LEVELS_2D.contains(&level) {
        html2d::generate(level, log);
    } else if list::LEVELS_3D.contains(&level) {
        html3d::generate(level, log);
    } else {
        eprintln!("Unknown level ID!")
    }
}
