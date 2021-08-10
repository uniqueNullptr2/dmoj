use std::str::FromStr;
use std::{collections::BTreeMap, io::stdin};

struct Timeline {
    filter_t: i32,
    filter_v: i32,
    t_map: BTreeMap<i32, i32>,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            filter_t: 0,
            filter_v: 0,
            t_map: BTreeMap::new(),
        }
    }

    pub fn add_event(&mut self, event_str: &str) {
        let mut time = 0;
        let mut label = 0;
        for (i, s) in event_str.split_whitespace().enumerate() {
            if i == 0 {
                time = i32::from_str(s).unwrap();
            } else if i == 1 {
                label = i32::from_str(s).unwrap();
            }
        }
        self.t_map.insert(time, label);
    }

    fn change_time(&mut self, time_old: i32, time_new: i32) {
        let label = self.t_map.remove(&time_old).unwrap();
        self.t_map.insert(time_new, label);
    }

    fn change_label(&mut self, time: i32, label_new: i32) {
        *self.t_map.get_mut(&time).unwrap() = label_new;
    }

    fn change_filter(&mut self, filter_type: i32, filter_val: i32) {
        self.filter_t = filter_type;
        self.filter_v = filter_val;
    }

    fn filter(&self, label: i32) -> bool {
        match self.filter_t {
            1 => label > self.filter_v,
            -1 => label < self.filter_v,
            _ => true,
        }
    }

    fn search(&self, time: i32) -> i32 {
        let mut upper = self.t_map.range(time..);
        let mut lower = self.t_map.range(..time);

        let mut next = upper.next();
        let mut prev = lower.next_back();

        loop {
            match (prev, next) {
                (Some((p_time, p_label)), Some((n_time, _)))
                    if self.filter(*p_label) && time - p_time < n_time - time =>
                {
                    return *p_time;
                }
                (Some((p_time, _)), Some((n_time, n_label)))
                    if self.filter(*n_label) && n_time - time <= time - p_time =>
                {
                    return *n_time;
                }

                (Some((_, p_label)), Some(_)) if self.filter(*p_label) => {
                    next = upper.next();
                }
                (Some(_), Some((_, n_label))) if self.filter(*n_label) => {
                    prev = lower.next_back();
                }

                (Some((p_time, p_label)), None) if self.filter(*p_label) => {
                    return *p_time;
                }
                (None, Some((n_time, n_label))) if self.filter(*n_label) => {
                    return *n_time;
                }

                (Some(_), None) => {
                    prev = lower.next_back();
                }
                (None, Some(_)) => {
                    next = upper.next();
                }

                (None, None) => return -1,

                (Some(_), Some(_)) => {
                    prev = lower.next_back();
                    next = upper.next();
                }
            }
        }
    }

    pub fn execute_query(&mut self, query: &str) -> Option<i32> {
        let (q_type, param1, param2) = Timeline::parse_query(query);

        match q_type {
            "T" => {
                self.change_time(param1, param2.unwrap());
                None
            }
            "L" => {
                self.change_label(param1, param2.unwrap());
                None
            }
            "F" => {
                self.change_filter(param1, param2.unwrap());
                None
            }
            "S" => Some(self.search(param1)),
            _ => None,
        }
    }

    fn parse_query(s: &str) -> (&str, i32, Option<i32>) {
        let mut q_type = "";
        let mut param1 = 0;
        let mut param2 = None;

        for (i, ss) in s.split_whitespace().enumerate() {
            if i == 0 {
                q_type = ss;
            } else if i == 1 {
                if q_type == "F" {
                    param1 = match ss {
                        ">" => 1,
                        "<" => -1,
                        _ => 0,
                    };
                } else {
                    param1 = i32::from_str(ss).unwrap();
                }
            } else if i == 2 {
                param2 = Some(i32::from_str(ss).unwrap());
            }
        }

        (q_type, param1, param2)
    }
}

fn main() {
    let stdin = stdin();
    let mut s = String::with_capacity(100);
    stdin.read_line(&mut s).unwrap();
    let n = u32::from_str_radix(s.trim(), 10).unwrap();
    let mut timeline = Timeline::new();

    for _ in 0..n {
        s.clear();
        stdin.read_line(&mut s).unwrap();

        timeline.add_event(&s);
    }

    s.clear();
    stdin.read_line(&mut s).unwrap();
    let q = u32::from_str_radix(s.trim(), 10).unwrap();

    for _ in 0..q {
        s.clear();
        stdin.read_line(&mut s).unwrap();

        if let Some(x) = timeline.execute_query(&s) {
            println!("{}", x);
        }
    }
}
