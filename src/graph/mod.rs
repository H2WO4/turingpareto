use crate::list;

mod gif3d;
mod html2d;
mod png3d;

pub fn handle(level: &str, animated: bool) {
    if list::LEVELS_2D.contains(&level) {
        if animated {
            eprintln!("Cannot generate animated graph of a component level!")
        }
        html2d::generate(level);
    } else if list::LEVELS_3D.contains(&level) {
        if animated {
            gif3d::generate(level);
        } else {
            png3d::generate(level);
        }
    } else {
        eprintln!("Unknown level ID!")
    }
}
