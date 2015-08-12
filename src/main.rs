mod vec2d;
mod elem;
mod map;
mod solver;

struct Scenario {
    map: &'static str,
    cars: Vec<elem::Car>,
}

struct Symbology {
    input: char,
    extra: char,
    help: &'static str,
}

fn main() {
    println!("SYMBOLOGY:");
    for sym in vec![
        Symbology{input: 'x', extra: ' ', help: "road"},
        Symbology{input: 'r', extra: ' ', help: "red cube"},
        Symbology{input: 'g', extra: ' ', help: "green cube"},
        Symbology{input: 'b', extra: ' ', help: "blue cube"},
        Symbology{input: 'y', extra: ' ', help: "yellow cube"},
        Symbology{input: 'R', extra: ' ', help: "red house"},
        Symbology{input: 'G', extra: ' ', help: "green house"},
        Symbology{input: 'B', extra: ' ', help: "blue house"},
        Symbology{input: 'Y', extra: ' ', help: "yellow house"},
        Symbology{input: 'v', extra: 'p', help: "pink down button"},
        Symbology{input: '^', extra: 'p', help: "pink up button"},
        Symbology{input: '~', extra: 'p', help: "pink open brige"},
        Symbology{input: '#', extra: 'p', help: "pink closed button"},
        Symbology{input: 'v', extra: 'v', help: "violet down button"},
        Symbology{input: '^', extra: 'v', help: "violet up button"},
        Symbology{input: '~', extra: 'v', help: "violet open brige"},
        Symbology{input: '#', extra: 'v', help: "violet closed button"},
        Symbology{input: 'v', extra: 'o', help: "orange down button"},
        Symbology{input: '^', extra: 'o', help: "orange up button"},
        Symbology{input: '~', extra: 'o', help: "orange open brige"},
        Symbology{input: '#', extra: 'o', help: "orange closed button"},
        Symbology{input: 'v', extra: 'c', help: "cream down button"},
        Symbology{input: '^', extra: 'c', help: "cream up button"},
        Symbology{input: '~', extra: 'c', help: "cream open brige"},
        Symbology{input: '#', extra: 'c', help: "cream closed button"},
    ] {
        println!("{}{} => {}: {}",
                    sym.input,
                    sym.extra,
                    elem::Elem::from_char(sym.input, sym.extra),
                    sym.help,
                );
    }
    println!("\n\n");

    let scenarii = vec![
        Scenario {
            map: "\
                x  r--R\n\
                |  |   \n\
                x--x   \
            ",
            cars: vec![
                elem::Car::new(0, 0, elem::Color::Red),
            ],
        },
        Scenario {
            map: "\
                x--B--x     x--B--x\n\
                |     |     |     |\n\
                x     x--B  x     x\n\
                |           |     |\n\
                x--x--x--x--x     x\n\
                .                 |\n\
                x--b--b--b--x--x--x\
            ",
            cars: vec![
                elem::Car::new(0, 3, elem::Color::Blue),
            ],
        },
        Scenario {
            map: "\
                R--x--x   \n\
                .     |   \n\
                .  x--x--x\n\
                .  |  |  |\n\
                .  r--x  x\n\
                .        |\n\
                x--x--x--x\n\
            ",
            cars: vec![
                elem::Car::new(0, 3, elem::Color::Red),
            ],
        },
        Scenario {
            map: "x--r--R  B--b--x--x",
            cars: vec![
                elem::Car::new(0, 0, elem::Color::Red),
                elem::Car::new(6, 0, elem::Color::Blue),
            ],
        },
        Scenario {
            map: "\
                x--r--x--x--x\n\
                .  |     |   \n\
                B--x--x  b   \n\
                .  |  |  |   \n\
                R--x  x--x   \n\
            ",
            cars: vec![
                elem::Car::new(0, 0, elem::Color::Red),
                elem::Car::new(4, 0, elem::Color::Blue),
            ],
        },
        Scenario {
            map: "\
                .     R      \n\
                .     |      \n\
                .     r      \n\
                .     |      \n\
                x--x--x--x--x\n\
                .     |      \n\
                .     b      \n\
                .     |      \n\
                .     B      \n\
            ",
            cars: vec![
                elem::Car::new(0, 2, elem::Color::Red),
                elem::Car::new(4, 2, elem::Color::Blue),
            ],
        },
        Scenario {
            map: "\
                .  x--R--x--G--x   \n\
                .  |     |     |   \n\
                .  x--x--r--x--x   \n\
                .  |     |     |   \n\
                .  x--x--g--x--x   \n\
                .  |     |     |   \n\
                .  x     x     x   \n\
                .  |     |     |   \n\
                .  x     x     x   \n\
                .  |     |     |   \n\
                x--x--x--x--x--x--x\n\
            ",
            cars: vec![
                elem::Car::new(0, 5, elem::Color::Red),
                elem::Car::new(6, 5, elem::Color::Green),
            ],
        },
        Scenario {
            map: "\
                .  x--x--r  x--x--x   \n\
                .  |     |  |     |   \n\
                R--x     x--x     x--x\n\
                .  |     |  |     |   \n\
                .  x--x--x  y--x--x   \n\
                .     |        |      \n\
                .  x--x--Y  x--x--x   \n\
                .  |     |  |     |   \n\
                .  x     x--x     x   \n\
                .  |     |  |     |   \n\
                .  x--x--x  x--x--x   \n\
                .     |               \n\
                .     x
            ",
            cars: vec![
                elem::Car::new(2, 6, elem::Color::Red),
                elem::Car::new(7, 1, elem::Color::Yellow),
            ],
        },
        Scenario {
            map: "\
             x--x--x--x--x--x--x--x--x\n\
             |           |           |\n\
             x     b--B--x--x--b     x\n\
             |     |     |     |     |\n\
             x     x     x     x     x\n\
             |     |     |     |     |\n\
             x--B--x--x--x--x--x--x--b\n\
             |     |     |     |     |\n\
             x     x     x     x     x\n\
             |     |     |     |     |\n\
             x     x--x--x--B--x     x\n\
             |           |           |\n\
             x--x--x--x--x--x--x--x--x\
            ",
            cars: vec![
                elem::Car::new(4, 3, elem::Color::Blue),
            ],
        },
        Scenario {
            map: "\
            x--r--R      \n\
            .            \n\
            x--b--x--B   \n\
            .            \n\
            x--y--x--x--Y\
            ",
            cars: vec![
                elem::Car::new(0, 0, elem::Color::Red),
                elem::Car::new(0, 1, elem::Color::Blue),
                elem::Car::new(0, 2, elem::Color::Yellow),
            ],
        },
        Scenario {
            map: "\
            x--x--x--x--x--x--x--x\n\
            |           |        |\n\
            x           x        x\n\
            |           |        |\n\
            x--R--x--x--r--x--R--x\n\
            |     |     |        |\n\
            x     x     x        x\n\
            |     |     |        |\n\
            x     x     x--x--R--x\n\
            |     |     |        |\n\
            r--x--r     r        x\n\
            |     |     |        |\n\
            x     x     x--x--R--x\n\
            |     |     |        |\n\
            x     x     x        x\n\
            |     |     |        |\n\
            x--x--x--x--x--x--x--x\n\
            .           |         \n\
            .           x         \
            ",
            cars: vec![
                elem::Car::new(4, 9, elem::Color::Red),
            ],
        },
        Scenario {
            map: "\
            R--x--Y--x--B--x--x--x--x--x--x\n\
            .                             |\n\
            .                             x\n\
            .                             |\n\
            x--x--x--r--x--y--x--b--x--x--x\
            ",
            cars: vec![
                elem::Car::new(0, 2, elem::Color::White),
            ],
        },
        Scenario {
            map: "\
            .        x--x--~v-x--x         \n\
            .        |           |         \n\
            x--r--^v-x--x--#v-x--x--x--x--R\
            ",
            cars: vec![
                elem::Car::new(0, 1, elem::Color::Red),
            ],
        },
        Scenario {
            map: "\
            .           R            \n\
            .           |            \n\
            .           x            \n\
            .           |            \n\
            .           ~v           \n\
            .           |            \n\
            x--x--x--x--x--x--x--x--x\n\
            |           |           |\n\
            x           x           x\n\
            |           |           |\n\
            ^v          ~v          r\n\
            |           |           |\n\
            x           x           x\n\
            |           |           |\n\
            x--x--x--x--x--x--x--x--x\n\
            .           |            \n\
            .           x            \
            ",
            cars: vec![
                elem::Car::new(4, 8, elem::Color::Red),
            ],
        },
        Scenario {
            map: "\
            x--r--^p-x--R\n\
            .            \n\
            x--b--#p-x--B\n\
            ",
            cars: vec![
                elem::Car::new(0, 1, elem::Color::Blue),
                elem::Car::new(0, 0, elem::Color::Red),
            ],
        },
        Scenario {
            map: "\
            x--r--^p-x--^o-vp-x--#p-~o-x--x--R\
            ",
            cars: vec![
                elem::Car::new(0, 0, elem::Color::Red),
            ],
        },
        Scenario {
            map: "\
            x--vo-x--x--x     x--x--x--x--x \n\
            |     |     |     |           | \n\
            x     y     x--#o-x           x \n\
            |     |     |     |           | \n\
            x--^o-x--x--x     x--x--G--x--x \n\
            |     |     |     |     |     | \n\
            ~o    ~o    ~o    ~v    ~o    ~v\n\
            |     |     |     |     |     | \n\
            x--^o-x--^o-x     x     x--vv-x \n\
            |     |     |     |     |     | \n\
            x     x     x--~o-x     ^o    g \n\
            |     |     |     |     |     | \n\
            x--Y--^o-x--x     x--x--r--x--x \n\
            |     |     |     |     |     | \n\
            #v    ~o    ~v    ~v    #o    #v \n\
            |     |     |     |     |     | \n\
            x--^v-x--x--x     x--x--x--x--x \n\
            |     |     |     |           | \n\
            x     x     x--~o-x           x \n\
            |     |     |     |           | \n\
            x--x--x--vv-x     x--x--R--x--x \
            ",
            cars: vec![
                elem::Car::new(2, 9, elem::Color::White),
            ],
        },
    ];

    for scenario in scenarii {
        let m = map::Map::from_str(scenario.map);
        let map = m.clone();
        println!("INPUT:\n{}", m);
        let mut s = solver::Solver::new(m, scenario.cars.to_vec());
        if let Option::Some(solution) = s.solve() {
            println!("SOLUTION FOUND:");
            map.output_solution(&solution, &scenario.cars);
            for (i, car) in scenario.cars.iter().enumerate() {
                let car_moves: Vec<Option<elem::Dir>> = solution.iter().map(|moves| moves[i]).collect();
                let car_moves = group(car_moves);
                println!("{} ({}, {}): {}",
                    car, car.coord.0, car.coord.1,
                    car_moves.into_iter()
                              .filter(|&(dir, _)| dir.is_some())
                              .map(|(dir, n)| format!("{}{}", n, dir.unwrap()))
                              .fold("".to_string(), |s, d| format!("{}{} ", s, d)),
                );
            }
        } else {
            println!("NO SOLUTION FOUND");
        }
        println!("\n\n\n");
    }
}

fn group<T: Eq + Copy>(v: Vec<T>) -> Vec<(T, usize)> {
    let mut r = vec![];
    if v.len() == 0 {
        return r;
    }

    let mut last = v.get(0).unwrap();
    let mut n = 1;
    for x in v.iter().skip(1) {
        if *x != *last {
            r.push((*last, n));
            n = 0;
            last = x;
        }
        n += 1;
    }
    r.push((*last, n));
    r
}
