
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Color {
    pub fn as_list(&self) -> [f32; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
    pub fn black() -> Self {
        Color {
            red: 0f32,
            green: 0f32,
            blue: 0f32,
            alpha: 1f32,
        }
    }
}
