use std::fmt;
use ::map;
use ::elem;

#[derive(Clone)]
pub struct State {
    from: Vec<Option<elem::Dir>>,
    cars: Vec<elem::Car>,
    map: map::Map,
}

impl fmt::Display for State {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.map.fmt(fmt)
    }
}

pub struct Solver {
    states: Vec<State>,
    dirs: Vec<Vec<Option<elem::Dir>>>,
}

impl Solver {
    pub fn new(m: map::Map, cars: Vec<elem::Car>) -> Solver {
        if let Some(car) = cars.iter().find(|car| car.coord.0 >= m.width || car.coord.1 >= m.height) {
            panic!("car {} is misplaced ({}, {})", car, car.coord.0, car.coord.1);
        }
        Solver {
            dirs: gen_dirs(cars.len()),
            states: vec![State{
                from: vec![],
                cars: cars,
                map: m,
            }],
        }
    }

    pub fn solve(&mut self) -> Option<Vec<Vec<Option<elem::Dir>>>> {
        let dirs = self.dirs.to_vec(); // avoid borrow
        loop {
            //DEBUG:println!("state:\n{}", self.states.last().unwrap().map);
            for moves in dirs.iter() {
                //DEBUG:println!("trying {:?}", moves);
                // Try to move. Update states on success.
                // Return false on failure.
                if !self.push(moves) {
                    continue
                }
                if self.is_solved() {
                    return Some(vec![moves.to_vec()]);
                }
                // Solve recursively.
                if let Some(res) = self.solve() {
                    // If a solution has been found, return.
                    let mut s = vec![moves.to_vec()];
                    s.extend(res.into_iter());
                    return Some(s);
                }
                // Test other moves.
                self.pop();
            }
            return None
        }
    }

    fn is_solved(&self) -> bool {
        !self.states.last().unwrap().map.iter().any(|item| -> bool {
            match item.typ {
                elem::Type::House(_) => true,
                _ => false,
            }
        })
    }

    fn push(&mut self, moves: &Vec<Option<elem::Dir>>) -> bool {
        // Check that if moves[i] == Some then last state's moves[i] != None
        if !self.states.last().unwrap().from.iter().zip(moves.iter()).all(|(last_dir, dir)| -> bool {
            if last_dir.is_none() {
                return dir.is_none();
            }
            return true;
        }) {
            return false;
        }
        // Copy the last state
        let mut state = self.states.last().unwrap().clone();
        state.from = moves.to_vec();
        // Move the cars
        for (dir, car) in moves.into_iter().zip(state.cars.iter_mut()) {
            if !state.map.move_car(car, *dir) {
                return false;
            }
        }

        state.from = moves.to_vec();
        self.states.push(state);
        true
    }

    fn pop(&mut self) {
        //DEBUG:println!("rollback");
        self.states.pop();
    }
}

/// Generate all possible directions given a number of cars.
/// e.g. gen_dirs(1) => [[Up], [Right], [Down], [Left]]
fn gen_dirs(n: usize) -> Vec<Vec<Option<elem::Dir>>> {
    gen_dirs_inner(n).into_iter().filter(|dirs| !dirs.iter().all(|d| d.is_none())).collect()
}

fn gen_dirs_inner(n: usize) -> Vec<Vec<Option<elem::Dir>>> {
    let dirs = vec![
        Some(elem::Dir::Left),
        Some(elem::Dir::Up),
        Some(elem::Dir::Right),
        Some(elem::Dir::Down),
        None,
    ];

    let mut v: Vec<Vec<_>> = vec![];
    if n == 1 {
        for dir in dirs.into_iter() {
            v.push(vec![dir]);
        }
    } else {
        let dirs_n1 = gen_dirs_inner(n-1);
        for dir_n1 in dirs_n1 {
            for dir in dirs.iter() {
                let mut c = dir_n1.to_vec();
                c.push(*dir);
                v.push(c);
            }
        }
    }
    v
}
