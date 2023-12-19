use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::{utils::files::lines_from_file, Solution, SolutionPair};

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl From<&str> for Part {
    fn from(input: &str) -> Self {
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        input[1..input.len() - 1].split(',').for_each(|section| {
            let mut parts = section.split('=');
            let key = parts.next().unwrap();
            let val = parts.next().unwrap().parse().expect("Invalid value");
            match key {
                "x" => part.x = val,
                "m" => part.m = val,
                "a" => part.a = val,
                "s" => part.s = val,
                _ => panic!("Unexpected section"),
            }
        });
        part
    }
}

#[derive(Debug)]
struct Rule {
    rule_result: RuleResult,
    pred: Option<(char, char, i64)>,
}

impl Rule {
    fn solve_pt1(&self, part: &Part) -> Option<&RuleResult> {
        if let Some(pred) = self.pred {
            let prop = match pred.0 {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => panic!("Not a valid part type"),
            };

            let pass = match pred.1 {
                '>' => prop > pred.2,
                '<' => prop < pred.2,
                _ => false,
            };

            if pass {
                Some(&self.rule_result)
            } else {
                None
            }
        } else {
            Some(&self.rule_result)
        }
    }

    fn solve_pt2(&self, constraint: Constraint) -> (Constraint, Constraint) {
        let mut pc = constraint.clone();
        let mut fc = constraint.clone();

        if let Some(p) = self.pred {
            match p.0 {
                'x' => apply(p, &mut pc.lx, &mut pc.mx, &mut fc.lx, &mut fc.mx),
                'm' => apply(p, &mut pc.lm, &mut pc.mm, &mut fc.lm, &mut fc.mm),
                'a' => apply(p, &mut pc.la, &mut pc.ma, &mut fc.la, &mut fc.ma),
                's' => apply(p, &mut pc.ls, &mut pc.ms, &mut fc.ls, &mut fc.ms),
                _ => {}
            }
            (pc, fc)
        } else {
            (pc, fc)
        }
    }
}

impl From<&str> for Rule {
    fn from(r: &str) -> Self {
        let parts: Vec<_> = r.split(':').collect();
        let rule_result = match parts.last().expect("No result found") {
            &"R" => RuleResult::Reject,
            &"A" => RuleResult::Accept,
            next => RuleResult::Next(next.to_string()),
        };

        let pred = if parts.len() == 2 {
            let mut chars = parts[0].chars();
            Some((
                chars.next().expect("Invalid part type"),
                chars.next().expect("Invalid comparison type"),
                parts[0][2..].parse().expect("Invalid threshold"),
            ))
        } else {
            None
        };
        Rule { rule_result, pred }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RuleResult {
    Accept,
    Reject,
    Next(String),
}

struct WorkFlow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for WorkFlow {
    fn from(r: &str) -> Self {
        let parts: Vec<_> = r.split('{').collect();
        let name = parts[0].to_string();
        let rules = parts[1]
            .trim_end_matches('}')
            .split(',')
            .map(Rule::from)
            .collect();

        WorkFlow { name, rules }
    }
}

impl WorkFlow {
    fn solve_pt1(&self, part: &Part) -> &RuleResult {
        for rule in &self.rules {
            if let Some(result) = rule.solve_pt1(&part) {
                return result;
            } else {
                continue;
            }
        }
        panic!("We shouldn't get here");
    }

    fn solve_pt2(&self, constraint: Constraint) -> Vec<(RuleResult, Constraint)> {
        let mut res = Vec::new();
        let mut constr = constraint;

        for rule in &self.rules {
            let results = rule.solve_pt2(constr);
            res.push((rule.rule_result.clone(), results.0));
            constr = results.1;
        }
        res
    }
}

struct WorkFlowRunner {
    workflows: HashMap<String, WorkFlow>,
}

impl WorkFlowRunner {
    fn new(workflows: Vec<WorkFlow>) -> Self {
        WorkFlowRunner {
            workflows: workflows
                .into_iter()
                .map(|item| (item.name.clone(), item))
                .collect(),
        }
    }

    fn solve_pt1(&self, part: &Part) -> Option<i64> {
        let mut wf = "in";
        while let Some(workflow) = self.workflows.get(wf) {
            match workflow.solve_pt1(part) {
                RuleResult::Accept => return Some(part.x + part.m + part.a + part.s),
                RuleResult::Reject => return None,
                RuleResult::Next(v) => wf = v,
            }
        }
        None
    }

    fn solve_pt2(&self) -> i64 {
        let mut search = Vec::new();
        search.push((RuleResult::Next("in".to_string()), Constraint::new()));

        let mut perm_count = 0;
        while let Some(current) = search.pop() {
            match current.0 {
                RuleResult::Accept => {
                    perm_count += current.1.permutations();
                }
                RuleResult::Next(next) => {
                    self.workflows
                        .get(&next)
                        .expect("No workflow found")
                        .solve_pt2(current.1)
                        .iter()
                        .for_each(|ns| search.push(ns.clone()));
                }
                RuleResult::Reject => {
                    continue;
                }
            }
        }
        perm_count
    }
}

fn apply(
    pred: (char, char, i64),
    pass_min: &mut i64,
    pass_max: &mut i64,
    fail_min: &mut i64,
    fail_max: &mut i64,
) {
    if pred.1 == '>' {
        *pass_min = max(pred.2 + 1, *pass_min);
        *fail_max = min(pred.2, *fail_max);
    } else {
        *pass_max = min(pred.2 - 1, *pass_max);
        *fail_min = max(pred.2, *fail_min);
    }
}

#[derive(Copy, Clone, Debug)]
struct Constraint {
    lx: i64,
    mx: i64,

    lm: i64,
    mm: i64,

    la: i64,
    ma: i64,

    ls: i64,
    ms: i64,
}

impl Constraint {
    fn new() -> Self {
        Constraint {
            lx: 1,
            mx: 4000,
            lm: 1,
            mm: 4000,
            la: 1,
            ma: 4000,
            ls: 1,
            ms: 4000,
        }
    }

    fn permutations(&self) -> i64 {
        (1 + self.mx - self.lx)
            * (1 + self.mm - self.lm)
            * (1 + self.ma - self.la)
            * (1 + self.ms - self.ls)
    }
}

pub fn solve() -> SolutionPair {
    let lines = lines_from_file("input/day19.txt");
    let split: Vec<Vec<String>> = lines.split(|l| l.is_empty()).map(|s| s.to_vec()).collect();

    let workflows: Vec<WorkFlow> = split[0]
        .iter()
        .map(|s| WorkFlow::from(s.as_str()))
        .collect();
    let wf_runner = WorkFlowRunner::new(workflows);
    let parts: Vec<Part> = split[1].iter().map(|p| Part::from(p.as_str())).collect();

    let sol1: i64 = parts.iter().filter_map(|s| wf_runner.solve_pt1(s)).sum();
    let sol2: i64 = wf_runner.solve_pt2();

    (Solution::from(sol1), Solution::from(sol2))
}
