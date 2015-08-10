mod vec2d;
mod elem;
mod map;
mod solver;

struct Scenario {
    map: &'static str,
    cars: Vec<elem::Car>,
}

fn main() {
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
                println!("{} ({}, {}): {:?}", car, car.coord.0, car.coord.1, car_moves); 
            }
        } else {
            println!("NO SOLUTION FOUND");
        }
        println!("\n\n\n");
    }
}
