pub trait Output {
    fn as_str(&self) -> &str;
}

impl Output for String {
    fn as_str(&self) -> &str {
        self.as_str()
    }
}
