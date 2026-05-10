
mod pocket;

use crate::pocket::{doberman, doberman::tail, doberman::teeth, save_to_pocket};

fn main() {
    doberman::bark_at_the_moon();
    pocket::save_to_pocket();
    teeth::show_teeth();
    save_to_pocket();
    tail::shake_tail();
}
