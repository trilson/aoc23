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
        let valid_parts = &input[1..input.len() - 1];
        let mut pt = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        for section in valid_parts.split(",") {
            let x: Vec<&str> = section.split("=").collect();
            let val = x[1].parse::<i64>().expect("Not a valid value");
            match x[0] {
                "x" => pt.x = val,
                "m" => pt.m = val,
                "a" => pt.a = val,
                "s" => pt.s = val,
                _ => panic!("Unexpected section"),
            };
        }
        pt
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
        let mut pass_constraint = constraint.clone();
        let mut fail_constraint = constraint.clone();

        if let Some(pred) = self.pred {
            match pred.0 {
                'x' => apply_constraint(
                    pred,
                    &mut pass_constraint.min_x,
                    &mut pass_constraint.max_x,
                    &mut fail_constraint.min_x,
                    &mut fail_constraint.max_x,
                ),
                'm' => apply_constraint(
                    pred,
                    &mut pass_constraint.min_m,
                    &mut pass_constraint.max_m,
                    &mut fail_constraint.min_m,
                    &mut fail_constraint.max_m,
                ),
                'a' => apply_constraint(
                    pred,
                    &mut pass_constraint.min_a,
                    &mut pass_constraint.max_a,
                    &mut fail_constraint.min_a,
                    &mut fail_constraint.max_a,
                ),
                's' => apply_constraint(
                    pred,
                    &mut pass_constraint.min_s,
                    &mut pass_constraint.max_s,
                    &mut fail_constraint.min_s,
                    &mut fail_constraint.max_s,
                ),
                _ => {}
            }
            (pass_constraint, fail_constraint)
        } else {
            (pass_constraint, fail_constraint)
        }
    }
}

impl From<&str> for Rule {
    fn from(r: &str) -> Self {
        let q_a: Vec<_> = r.split(":").collect();
        if q_a.len() == 2 {
            let rule_result = match q_a[1] {
                "R" => RuleResult::Reject,
                "A" => RuleResult::Accept,
                next => RuleResult::Next(next.to_string()),
            };
            let condition = q_a[0];
            let threshold = condition[2..condition.len()].parse::<i64>().unwrap_or(0);
            let mut chars = condition.chars();
            let part_type = chars.next().expect("Not a valid part type");
            let comp_type = chars.next().expect("Not a valid comparison type");

            Rule {
                rule_result,
                pred: Some((part_type, comp_type, threshold)),
            }
        } else {
            Rule {
                rule_result: match q_a[0] {
                    "R" => RuleResult::Reject,
                    "A" => RuleResult::Accept,
                    other => RuleResult::Next(other.to_string()),
                },
                pred: None,
            }
        }
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
        let parts: Vec<_> = r.split("{").collect();
        let mut rule_str = parts[1].to_string();
        rule_str.pop();

        WorkFlow {
            name: parts[0].to_string(),
            rules: rule_str.split(",").map(|r| Rule::from(r)).collect(),
        }
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
            if !current.1.is_valid() {
                continue;
            }
            match current.0 {
                RuleResult::Accept => {
                    let perm = current.1.permutations();
                    perm_count += perm;
                    continue;
                }
                RuleResult::Reject => {
                    continue;
                }
                RuleResult::Next(next) => {
                    let wf = self.workflows.get(&next).expect("No workflow found");
                    let next_states = wf.solve_pt2(current.1);
                    next_states.iter().for_each(|ns| search.push(ns.clone()));
                }
            }
        }
        perm_count
    }
}

fn apply_constraint(
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
    min_x: i64,
    max_x: i64,

    min_m: i64,
    max_m: i64,

    min_a: i64,
    max_a: i64,

    min_s: i64,
    max_s: i64,
}

impl Constraint {
    fn new() -> Self {
        Constraint {
            min_x: 1,
            max_x: 4000,
            min_m: 1,
            max_m: 4000,
            min_a: 1,
            max_a: 4000,
            min_s: 1,
            max_s: 4000,
        }
    }

    fn is_valid(&self) -> bool {
        self.min_x < self.max_x
            && self.min_m < self.max_m
            && self.min_a < self.max_a
            && self.min_s < self.max_s
    }

    fn permutations(&self) -> i64 {
        (1 + self.max_x - self.min_x)
            * (1 + self.max_m - self.min_m)
            * (1 + self.max_a - self.min_a)
            * (1 + self.max_s - self.min_s)
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