use std::collections::{HashMap, HashSet, VecDeque};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Clone, Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BeamType {
    High,
    Low,
}

#[derive(Clone, Debug)]
struct Module {
    name: String,
    on: bool,
    module_type: ModuleType,
    targets: Vec<String>,
    conj: HashSet<String>,
}

impl Module {
    fn handle_source(&mut self, source: &str) {
        self.conj.insert(source.to_owned());
    }

    fn accept_beam(&mut self, incoming: BeamType, from: &str) -> Option<(BeamType, &Vec<String>)> {
        if self.module_type == ModuleType::Broadcaster {
            return Some((incoming, &self.targets));
        }
        if self.module_type == ModuleType::FlipFlop {
            if incoming == BeamType::Low {
                self.on = !self.on;
                if self.on {
                    return Some((BeamType::High, &self.targets));
                } else {
                    return Some((BeamType::Low, &self.targets));
                }
            }
        }
        if self.module_type == ModuleType::Conjunction {
            if incoming == BeamType::High {
                self.conj.remove(from);
            } else {
                self.conj.insert(from.to_owned());
            }
            if self.conj.is_empty() {
                return Some((BeamType::Low, &self.targets));
            } else {
                return Some((BeamType::High, &self.targets));
            }
        }
        None
    }
}
pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day20.txt");
    let mut modules: HashMap<_, _> = lines
        .iter()
        .map(|m| {
            let sp: Vec<_> = m.split(" -> ").collect();
            let targets: Vec<String> = sp[1].split(", ").map(|s| s.to_string()).collect();

            match sp[0].chars().next().expect("Not a valid module") {
                '%' => Module {
                    name: sp[0][1..].to_owned(),
                    on: false,
                    module_type: ModuleType::FlipFlop,
                    targets,
                    conj: HashSet::new(),
                },
                '&' => Module {
                    name: sp[0][1..].to_owned(),
                    on: true,
                    module_type: ModuleType::Conjunction,
                    targets,
                    conj: HashSet::new(),
                },
                _ => Module {
                    name: sp[0].to_owned(),
                    on: true,
                    module_type: ModuleType::Broadcaster,
                    targets,
                    conj: HashSet::new(),
                },
            }
        })
        .map(|s| (s.name.to_owned(), s.clone()))
        .collect();

    let updates: Vec<(String, Vec<String>)> = modules
        .values()
        .map(|module| (module.name.clone(), module.targets.clone()))
        .collect();

    for (name, targets) in updates {
        for target in targets {
            if let Some(target_module) = modules.get_mut(&target) {
                target_module.handle_source(&name);
            }
        }
    }
    let sol1 = solve_pt1(&mut modules);
    let sol2 = solve_pt2(&mut modules);
    (Solution::from(sol1), Solution::from(sol2))
}

fn solve_pt2(modules: &mut HashMap<String, Module>) -> i64 {
    0
}


fn solve_pt1(modules: &mut HashMap<String, Module>) -> i64 {
    let mut low =0;
    let mut high = 0;
    
    for _i in 0..1000 {
        let mut state = VecDeque::new();
        state.push_back((
            String::from("button"),
            String::from("broadcaster"),
            BeamType::Low,
        ));

        while let Some((from, to, beam)) = state.pop_front() {
            if beam == BeamType::Low {
                low += 1;
            } else {
                high += 1;
            }
            {
                if let Some(module) = modules.get_mut(&to) {
                    if let Some((new_data, targets)) = module.accept_beam(beam, &from) {
                        for target in targets.iter().cloned() {
                            state.push_back((to.clone(), target, new_data.clone()));
                        }
                    }
                }
            }
        }
    }
    low * high
}

#[test]
fn run_me() {
    let s = solve();
    print!("{s:?}");
}
