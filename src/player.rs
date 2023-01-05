pub fn play_movie(name: &str) {
    println!("Playing {}", name);
    play_audio(name);
}
fn play_audio(name: &str) {
    println!("Playing {}'s Audio", name);
}