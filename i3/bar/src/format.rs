#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum ThemeColor {
    White,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    BrightBlack,
    BrightGreen
}

impl ThemeColor {
    /// Gets the color associated with the enum. Basically color definitions.
    pub fn get_string(self) -> String {
        use ThemeColor::*;
        String::from(match self {
            White       => "839496",
            Black       => "002b36",
            Red         => "dc322f",
            Green       => "859900",
            Yellow      => "b58900",
            Blue        => "268bd2",
            Magenta     => "d33682",
            Cyan        => "2aa198",
            BrightBlack => "073642",
            BrightGreen => "9baf00"
        })
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum TextFormat {
    Foreground(ThemeColor),
    Background(ThemeColor),
    ColorPair(ThemeColor, ThemeColor),
    Underline(ThemeColor),
    Font(u8),
    Swap,
    Left,
    Right,
    Center,
    Multi(Vec<TextFormat>)
}

impl TextFormat {
    pub fn process_format(self) -> String {
        use TextFormat::*;

        String::from(match self {
            Foreground(color) => format!("%{{F#{}}}", color.get_string()),
            Background(color) => format!("%{{B#{}}}", color.get_string()),
            ColorPair(fg, bg) => format!("%{{F#{}}}%{{B#{}}}", fg.get_string(), bg.get_string()),
            Underline(color)  => format!("%{{U#{}}}", color.get_string()),
            Font(index)       => format!("%{{T#{}}}", index),
            Swap              => String::from("%{R}"),
            Left              => String::from("%{l}"),
            Right             => String::from("%{r}"),
            Center            => String::from("%{c}"),
            Multi(children) => {
                let mut sb = String::new();

                for child in children.iter() {
                    sb += child.clone().process_format().as_str();
                }

                sb
            },
        })
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum MouseEvent {
    LeftClick,
    MiddleClick,
    RightClick,
    ScrollUp,
    ScrollDown
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Segment {
    Compound(Vec<Segment>),
    Formatted(TextFormat, Box<Segment>),
    Action(MouseEvent, String, Box<Segment>),
    Text(String)
}

impl Segment {
    pub fn process_segment(self) -> String {
        use Segment::*;
        use MouseEvent::*;

        match self {
            Compound(children) => {
                let mut sb = String::new();

                for child in children.iter() {
                    sb += child.clone().process_segment().as_str();
                }

                sb
            },
            Formatted(fmt, box child_segment) => {
                format!("{}{}", fmt.process_format(), child_segment.process_segment())
            },
            Action(evt, cmd, box child_segment) => {
                let key = match evt {
                    LeftClick => 1,
                    MiddleClick => 2,
                    RightClick => 3,
                    ScrollUp => 4,
                    ScrollDown => 5
                };

                format!("%{{A{}:{}:}}{}%{{A}}", key, cmd, child_segment.process_segment())
            },
            Text(string) => string
        }
    }
}
