use open::that_in_background as open_in_background;

pub fn open_url(url: &str) {
    open_in_background(url).join().ok();
}