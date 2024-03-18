pub trait Color {
    fn to_char(&self) -> char;
    fn is_normal_color(&self) -> bool;
}
