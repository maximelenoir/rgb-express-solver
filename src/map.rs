use ::vec2d;
use ::elem;
use ::solver;
use std::fmt;
use std::result;
use std::collections::HashMap;

pub type Map = vec2d::Vec2D<elem::Elem>;

impl Map {
    pub fn from_str(map: &str) -> Map { // FIXME: Option<Map, Err>
        let size = map.len();
        let width = map.find('\n').unwrap_or(size);
        let height = size / width;

        // FIXME ------------v
        let rows = (height + 2) / 2;
        let cols = (width + 2) / 3;
        let mut m = vec2d::Vec2D::new(cols, rows);
        // Collect stuff first
        for (r, row) in map.split('\n')
                            .take_while(|row| row.len() > 0)
                            .enumerate()
                            .filter(|&(r, _)| { r % 2 == 0 })
                            .map(|(r, row)| { (r / 2, row) }) {
            let mut row = row.to_string();
            row.push(' ');
            let extra_it = row.chars()
                              .enumerate()
                              .filter(|&(c, _)| { c % 3 == 1 })
                              .map(|(_, col)| { col });
            for ((c, x), extra) in row.chars()
                               .enumerate()
                               .filter(|&(c, _)| { c % 3 == 0 })
                               .map(|(c, col)| { (c / 3, col) })
                               .zip(extra_it) {
                m[(c, r)] = elem::Elem::from_char(x, extra);
            }
        }
        // Connect
        for (r, row) in map.split('\n')
                            .take_while(|row| row.len() > 0)
                            .enumerate() {
            for (c, x) in row.chars().enumerate() {
                if r % 2 == 0 && c % 3 == 2 && x == '-' {
                    m[(c / 3    , r / 2)].connect(elem::Dir::Right);
                    m[(c / 3 + 1, r / 2)].connect(elem::Dir::Left);
                } else if r % 2 == 1 && c % 3 == 0 && x == '|' {
                    m[(c / 3, r / 2    )].connect(elem::Dir::Down);
                    m[(c / 3, r / 2 + 1)].connect(elem::Dir::Up);
                }
            }
        }
        m
    }

    pub fn move_car(&mut self, car: &mut elem::Car, dir: Option<elem::Dir>) -> bool {
        if dir.is_none() {
            return true;
        }
        let dir = dir.unwrap();
        // Check there is a road to move to.
        if !self[car.coord].connected(dir) {
            return false;
        }

        // Disconnect the road, move the car.
        self[car.coord].disconnect(dir);
        self[car.coord].occupied = false;
        car.roll(dir);
        if self[car.coord].occupied {
            return false;
        }
        self[car.coord].disconnect(dir.rev());
        self[car.coord].occupied = true;


        // Update state with game dynamics.
        match self[car.coord].typ {
            elem::Type::Empty => return false, // should not happen
            elem::Type::Road => return true,
            elem::Type::DropOff => return true,
            elem::Type::DropOn => {
                self[car.coord].typ = elem::Type::DropOff;
                if let Some(&e @ elem::Type::Cube(..)) = car.cubes.last() {
                    // Drop the box on the ground
                    self[car.coord].typ = e;
                    car.cubes.pop();
                }
                return true;
            },
            elem::Type::PushedButton(_) => return true,
            elem::Type::ArmedButton(c) => {
                for x in self.iter_mut() {
                    x.typ = match x.typ {
                        elem::Type::ArmedButton(cc) if c == cc  =>  elem::Type::PushedButton(c),
                        elem::Type::PushedButton(cc) if c == cc => elem::Type::ArmedButton(c),
                        elem::Type::OpenBridge(cc) if c == cc   => elem::Type::ClosedBridge(c),
                        elem::Type::ClosedBridge(cc) if c == cc => elem::Type::OpenBridge(c),
                        _ => x.typ,
                    }
                }
                return true;
            }
            elem::Type::OpenBridge(_) => return false,
            elem::Type::ClosedBridge(_) => return true,
            elem::Type::FullHouse(_) => return false,
            elem::Type::House(_) if car.cubes.is_empty() => return false,
            elem::Type::House(c) => {
                if let Some(&elem::Type::Cube(cc)) = car.cubes.last() {
                    if cc != c {
                        return false;
                    } else {
                        // Yield the cube to the house.
                        self[car.coord].typ = elem::Type::FullHouse(c);
                        car.cubes.pop();
                        return true;
                    }
                }
                return false;
            },
            elem::Type::Cube(..) if car.cubes.len() < 3 => {
                // Steal the cube
                car.cubes.push(self[car.coord].typ);
                self[car.coord].typ = elem::Type::Road;
                return true;
            }
            elem::Type::Cube(_) => return false,
        }
    }

    pub fn check(&self, cars: &Vec<elem::Car>) -> bool {
        for car in cars {
            // Check wether a bridge opened AFTER a car moved.
            if let elem::Type::OpenBridge(_) = self[car.coord].typ {
                return false;
            }
        }
        return true;
    }

    pub fn output_solution(&mut self, solution: &solver::Solution, cars: &Vec<elem::Car>) {
        let mut cars = cars.clone();

        self.iter_mut().filter(|e| {
            match e.typ {
                elem::Type::DropOn | elem::Type::DropOff => true,
                _ => false,
            }
        }).zip(&solution.targets).map(|(e, &is_on)| { if is_on { e.typ = elem::Type::DropOn; } e }).count();

        let mut rights: HashMap<(usize, usize), elem::Color> = HashMap::new();
        let mut downs: HashMap<(usize, usize), elem::Color> = HashMap::new();
        for (i, car) in cars.iter_mut().enumerate() {
            for dir in solution.dirs.iter().map(|moves| moves[i]) {
                if dir.is_none() {
                    continue;
                }
                let dir = dir.unwrap();
                let old_coord = car.coord;
                car.roll(dir);
                match dir {
                    elem::Dir::Up => downs.insert(car.coord, car.color),
                    elem::Dir::Right => rights.insert(old_coord, car.color),
                    elem::Dir::Down => downs.insert(old_coord, car.color),
                    elem::Dir::Left => rights.insert(car.coord, car.color),
                };
            }
        }

        for (r, row) in self.undl.chunks(self.width).enumerate() {
            for (c, col) in row.iter().enumerate() {
                let mut conn = "  ".to_string();
                if col.connected(elem::Dir::Right) {
                    if let Some(color) = rights.get(&(c, r)) {
                        conn = color.colorize("--");
                    } else {
                        conn = "--".to_string();
                    }
                }
                print!("{}{}", col, conn);
            }
            println!("");
            for (c, col) in row.iter().enumerate() {
                let mut conn = " ".to_string();
                if col.connected(elem::Dir::Down) {
                    if let Some(color) = downs.get(&(c, r)) {
                        conn = color.colorize("|");
                    } else {
                        conn = "|".to_string();
                    }
                }
                print!("{}  ", conn);
            }
            println!("");
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        for (r, row) in self.undl.chunks(self.width).enumerate() {
            if r > 0 {
                try!(write!(fmt, "\n"));
            }
            for x in row {
                let mut conn = "  ";
                if x.connected(elem::Dir::Right) {
                    conn = "--";
                }
                try!(write!(fmt, "{}{}", x, conn));
            }
            try!(write!(fmt, "\n"));
            for x in row {
                let mut conn = " ";
                if x.connected(elem::Dir::Down) {
                    conn = "|";
                }
                try!(write!(fmt, "{}  ", conn));
            }
        }
        result::Result::Ok(())
    }
}
