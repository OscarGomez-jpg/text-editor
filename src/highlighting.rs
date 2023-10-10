use crossterm::style::Color;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MiltilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

impl Type {
    pub fn to_color(self) -> Color {
        match self {
            Type::Number => Color::Rgb {
                r: 180,
                g: 126,
                b: 141,
            },
            Type::Match => Color::Rgb {
                r: 38,
                g: 139,
                b: 210,
            },
            Type::String => Color::Rgb {
                r: 205,
                g: 92,
                b: 8,
            },
            Type::Character => Color::Rgb {
                r: 108,
                g: 113,
                b: 196,
            },
            Type::Comment | Type::MiltilineComment => Color::Rgb {
                r: 245,
                g: 232,
                b: 183,
            },
            Type::PrimaryKeywords => Color::Red,
            Type::SecondaryKeywords => Color::Rgb {
                r: 255,
                g: 161,
                b: 152,
            },
            _ => Color::Rgb {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }
}
