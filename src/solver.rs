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
    targets: Vec<Vec<bool>>,
    dirs: Vec<Vec<Option<elem::Dir>>>,
}

impl Solver {
    pub fn new(m: map::Map, cars: Vec<elem::Car>) -> Solver {
        if let Some(car) = cars.iter().find(|car| car.coord.0 >= m.width || car.coord.1 >= m.height) {
            panic!("car {} is misplaced ({}, {})", car, car.coord.0, car.coord.1);
        }
        Solver {
            targets: gen_onoff(m.iter().filter(|&elem| match elem.typ { elem::Type::DropOff => true, _ => false }).count()),
            dirs: gen_dirs(cars.len()),
            states: vec![State{
                from: vec![],
                cars: cars,
                map: m,
            }],
        }
    }

    pub fn solve(&mut self) -> Option<Solution> {
        // Try all combination of targets.
        for targets in self.targets.to_vec() {
            self.states.get_mut(0).unwrap().map.iter_mut()
                                      .filter(|elem| match elem.typ { elem::Type::DropOff => true, _ => false })
                                      .zip(&targets)
                                      .map(|(e, &is_on)| { if is_on { e.typ = elem::Type::DropOn; } e })
                                      .count();
            let res = self.solve_inner();
            if let Some(dirs) = res {
                return Some(Solution {
                    dirs: dirs,
                    targets: targets,
                });
            }
        }
        None
    }

    fn solve_inner(&mut self) -> Option<Vec<Vec<Option<elem::Dir>>>> {
        let dirs = self.dirs.to_vec(); // avoid borrow
        loop {
            for moves in dirs.iter() {
                // Try to move. Update states on success.
                // Return false on failure.
                if !self.push(moves) {
                    continue
                }
                if self.is_solved() {
                    return Some(vec![moves.to_vec()]);
                }
                // Solve recursively.
                if let Some(res) = self.solve_inner() {
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
        // Check the coherency of the moves
        if !state.map.check(&state.cars) {
            return false;
        }

        self.states.push(state);
        true
    }

    fn pop(&mut self) {
        self.states.pop();
    }
}

pub struct Solution {
    pub dirs: Vec<Vec<Option<elem::Dir>>>,
    pub targets: Vec<bool>,
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

fn gen_onoff(n: usize) -> Vec<Vec<bool>> {
    let mut v: Vec<Vec<_>> = vec![];
    if n == 0 {
        return vec![vec![]];
    } else {
        let onoffs_n1 = gen_onoff(n-1);
        for onoff_n1 in onoffs_n1 {
            let mut c = onoff_n1.to_vec();
            c.push(false);
            v.push(c);
            let mut c = onoff_n1.to_vec();
            c.push(true);
            v.push(c);
        }
    }
    v
}
