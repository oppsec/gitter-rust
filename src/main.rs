use crate::interface::ui::get_banner;
use crate::modules::rustfinder::get_username;

pub mod interface;
pub mod modules;

fn main() {
    // |
    get_banner();
    get_username();
}
