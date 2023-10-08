use crossterm::style::Color;

pub enum Type {
    None,
    Number,
}

impl Type {
    pub fn to_color(&self) -> Color {
        match self {
            Type::Number => Color::Rgb {
                r: 180,
                g: 126,
                b: 141,
            },
            _ => Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}
