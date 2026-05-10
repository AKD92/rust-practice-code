
pub fn bark_at_the_moon() {
    println!("Barking at the moon!!");
    teeth::show_teeth();
    tail::shake_tail();
}

pub mod teeth {
    pub fn show_teeth() {
        println!("Can you see the teeth now?");
    }
}

pub mod tail {
    pub fn shake_tail() {
        println!("Saking my tail for fun!!");
        super::super::personal_call();
    }
}