mod player;

fn main() {
    println!("Hello, world!");
    player::play_movie("The Matrix");
    clean::clean_house();
    clean::files::clean_files();
}

mod clean {
    pub fn clean_house() {
        println!("Cleaning the house");
    }
    pub mod files {
        pub fn clean_files() {
            println!("Cleaning the files");
        }
    }
}