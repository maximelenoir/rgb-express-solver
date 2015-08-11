use std::fmt;
use std::result;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    White,
    Orange,
    Violet,
    Pink,
    Cream,
}

impl Color {
    pub fn colorize(&self, s: &str) -> String {
        format!("{}{}\x1b[0m", match *self {
           Color::Red     => "\x1b[38;5;9m",
           Color::Green   => "\x1b[38;5;10m",
           Color::Blue    => "\x1b[38;5;4m",
           Color::Yellow  => "\x1b[38;5;11m",
           Color::White   => "\x1b[38;5;234m",
           Color::Orange  => "\x1b[38;5;202m",
           Color::Violet  => "\x1b[38;5;5m",
           Color::Pink    => "\x1b[38;5;207m",
           Color::Cream   => "\x1b[38;5;223m",
        }, s)
    }

    pub fn from_char(c: char) -> Option<Color> {
        match c {
            'r' => Some(Color::Red),
            'g' => Some(Color::Green),
            'b' => Some(Color::Blue),
            'y' => Some(Color::Yellow),
            'w' => Some(Color::White),
            'o' => Some(Color::Orange),
            'v' => Some(Color::Violet),
            'p' => Some(Color::Pink),
            'c' => Some(Color::Cream),
            _   => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Type {
    Empty,
    Road,
    PushedButton(Color),
    ArmedButton(Color),
    OpenBridge(Color),
    ClosedBridge(Color),
    Cube(Color),
    House(Color),
    FullHouse(Color),
}

#[derive(Clone, Debug)]
pub struct Car {
    pub color: Color,
    pub coord: (usize, usize),
    pub cubes: Vec<Type>,
}

impl Car {
    pub fn new(x: usize, y: usize, color: Color) -> Car {
        Car {
            color: color,
            coord: (x, y),
            cubes: vec![],
        }
    }

    pub fn roll(&mut self, dir: Dir) {
        match dir {
            Dir::Left  => self.coord.0 -= 1,
            Dir::Up    => self.coord.1 -= 1,
            Dir::Right => self.coord.0 += 1,
            Dir::Down  => self.coord.1 += 1,
        }
    }
}

impl fmt::Display for Car {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(fmt, "{}", self.color.colorize("🚚"))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Elem {
    pub conn: [bool; 4],
    pub typ: Type,
    pub occupied: bool,
}

impl Elem {
    pub fn from_char(c: char, x: char) -> Elem {
        Elem {
            conn: [false; 4],
            typ: match c {
                'x' => Type::Road,
                'r' => Type::Cube(Color::Red),
                'R' => Type::House(Color::Red),
                'g' => Type::Cube(Color::Green),
                'G' => Type::House(Color::Green),
                'b' => Type::Cube(Color::Blue),
                'B' => Type::House(Color::Blue),
                'y' => Type::Cube(Color::Yellow),
                'Y' => Type::House(Color::Yellow),
                'v' => Type::PushedButton(Color::from_char(x).unwrap()),
                '^' => Type::ArmedButton(Color::from_char(x).unwrap()),
                '~' => Type::OpenBridge(Color::from_char(x).unwrap()),
                '#' => Type::ClosedBridge(Color::from_char(x).unwrap()),
                _   => Type::Empty,
            },
            occupied: false,
        }
    }
    pub fn connect(&mut self, dir: Dir) {
        self.conn[dir as usize] = true;
    }
    pub fn disconnect(&mut self, dir: Dir) {
        self.conn[dir as usize] = false;
    }
    pub fn connected(&self, dir: Dir) -> bool {
        self.conn[dir as usize]
    }
}

impl fmt::Display for Elem {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(fmt, "{}", match self.typ {
            Type::Empty           => " ".to_string(),
            Type::Road            => "x".to_string(),
            Type::PushedButton(c) => c.colorize("🔳"),
            Type::ArmedButton(c)  => c.colorize("🔲"),
            Type::OpenBridge(c)   => c.colorize("≋"),
            Type::ClosedBridge(c) => c.colorize("⌒"),
            Type::Cube(c)         => c.colorize("📦"),
            Type::House(c)        => c.colorize("🏫"),
            Type::FullHouse(c)    => c.colorize("🏫"),
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Dir {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn rev(&self) -> Dir {
        match *self {
            Dir::Up    => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down  => Dir::Up,
            Dir::Left  => Dir::Right,
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}", match *self { 
            Dir::Up    => "↑", 
            Dir::Right => "→",
            Dir::Down  => "↓",
            Dir::Left  => "←",
        })
    }
}

impl Default for Elem {
    fn default() -> Elem {
        Elem {
            conn: [false; 4],
            typ: Type::Empty,
            occupied: false,
        }
    }
}

