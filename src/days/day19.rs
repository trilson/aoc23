use std::collections::HashMap;

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

impl Part {
    fn sum(&self) -> i64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Rule {
    rule_result: RuleResult,
    pred: Option<(char, char, i64)>,
}

impl Rule {
    fn accept(&self, part: &Part) -> Option<&RuleResult> {
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

#[derive(Clone, Debug)]
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
    fn accept(&self, part: &Part) -> &RuleResult {
        for rule in &self.rules {
            println!("Applying rule: {rule:?}");
            if let Some(result) = rule.accept(&part) {
                println!("Result: {result:?}");
                return result;
            } else {
                println!("Rule does not pass. Skip.");
                continue;
            }
        }
        panic!("We shouldn't get here");
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

    fn accept(&self, part: &Part) -> Option<i64> {
        println!("Processing part: {:?}", part);
        let mut wf = "in";
        while let Some(workflow) = self.workflows.get(wf) {
            println!("Entering workflow: {wf}");
            match workflow.accept(part) {
                RuleResult::Accept => return Some(part.sum()),
                RuleResult::Reject => return None,
                RuleResult::Next(v) => wf = v,
            }
        }
        None
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

    let sol1: i64 = parts.iter().filter_map(|s| wf_runner.accept(s)).sum();
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

// fn less_than(&)
#[test]
fn run_me() {
    println!("{:?}", solve());
}
