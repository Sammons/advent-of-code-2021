// 3 bits version
// 3 bits type id

// when type id = 4
// 5 bit chunks, leading with 1 until last chunk which leads with 0

// otherwise type id is operator
// bit 1 is length type
// 0 = 15 bits are the length of all sub packets
// 1 = 11 bits are the number of sub

struct Problem {
    nums: Vec<Num>,
}

#[derive(Debug, Clone)]
enum Num {
    Value(i32),
    Pair((Box<Num>, Box<Num>)),
}

enum ReductionResult {
    AddToLeftRight(i32),
    AddToLeftRightLater(i32),
    AddToRightLeft(i32),
    AddToRightLeftLater(i32),
    Success,
}

impl Num {
    fn is_value(&self) -> bool {
        match self {
            Num::Value(_) => true,
            _ => false,
        }
    }
    fn unwrap_value(&self) -> i32 {
        match self {
            Num::Value(v) => v.clone(),
            _ => {
                println!("{:?}", self);
                panic!("Unwrapped non-value Num!")
            }
        }
    }
    fn is_pair(&self) -> bool {
        match self {
            Num::Pair(_) => true,
            _ => false,
        }
    }
    fn unwrap_pair(&self) -> (&Num, &Num) {
        match self {
            Num::Pair(tuple) => (&tuple.0, &tuple.1),
            _ => panic!("Unwrapped non-pair Num!"),
        }
    }
    fn both_values(&self) -> bool {
        match self {
            Num::Pair(tuple) => tuple.0.is_value() && tuple.1.is_value(),
            _ => false,
        }
    }
    // fn unwrap_pair_mut(&mut self) -> (&mut Num, &mut Num) {
    //     match self {
    //         Num::Pair(tuple) => (tuple.0.as_mut(), tuple.1.as_mut()),
    //         _ => panic!("Unwrapped non-pair Num!"),
    //     }
    // }
    fn next_right_value(i: &mut usize, s: &str) -> Num {
        let slice_range = (*i)..(*i) + 1;
        let c = &s[slice_range];
        // always consumes a char
        if c == "[" {
            Num::from_string(i, s)
        } else {
            *i += 1;
            let v: i32 = c
                .parse()
                .map_err(|e| {
                    println!("{} -> {}", c, e);
                    e
                })
                .unwrap();
            Num::Value(v)
        }
    }
    fn next_left_value(i: &mut usize, s: &str) -> Num {
        let slice_range = (*i)..(*i) + 1;
        let c = &s[slice_range];
        if c == "[" {
            Num::from_string(i, s)
        } else {
            *i += 1;
            let v: i32 = c.parse().unwrap();
            Num::Value(v)
        }
    }
    fn from_string(i: &mut usize, s: &str) -> Num {
        let slice_range = (*i)..(*i) + 1;
        let c = &s[slice_range];
        if c == "[" {
            *i += 1; // move past [
            let left = Num::next_left_value(i, s);
            *i += 1; // move past ,
            let right = Num::next_right_value(i, s);
            *i += 1; // move past ]
            Num::Pair((Box::new(left), Box::new(right)))
        } else {
            panic!("Cannot parse string not starting with [")
        }
    }
    fn add_to_left_most_value(&mut self, v: i32) {
        let mut cur = self;
        loop {
            match cur {
                Num::Pair((left, _)) => {
                    cur = left.as_mut();
                }
                Num::Value(value) => {
                    // println!("{} + {}", *value, v);
                    *cur = Num::Value(*value + v);
                    break;
                }
            };
        }
    }
    fn apply_split(v: i32) -> Num {
        if v < 10 {
            Num::Value(v)
        } else {
            // println!("Split {}", v);
            let div_2: f32 = v as f32 / 2.0f32;
            let left_div_2_floor = div_2.floor() as i32;
            let right_div_2_ceil = div_2.ceil() as i32;
            Num::Pair((
                Box::new(Num::Value(left_div_2_floor)),
                Box::new(Num::Value(right_div_2_ceil)),
            ))
        }
    }
    fn add_to_right_most_value(&mut self, v: i32) {
        let mut cur = self;
        loop {
            match cur {
                Num::Pair((_, right)) => {
                    cur = right.as_mut();
                }
                Num::Value(value) => {
                    // println!("{} + {}", *value, v);
                    *cur = Num::Value(*value + v);
                    break;
                }
            };
        }
    }
    fn is_sploding_pair(&self, depth: usize) -> bool {
        self.is_pair() && self.both_values() && depth >= 4
    }
    fn splode_recurse<'a, 'b>(&'a mut self, depth: usize) -> Option<ReductionResult>
    where
        'a: 'b,
    {
        if self.is_value() {
            return None;
        } else if let Num::Pair((left, right)) = self {
            // self with right-hand pair ready to explode
            // need to mutate self.right to be 0
            // need to add self.right.right to...
            //   first parent where this descent was the left-hand
            //   parent.left-most-pair.left
            // need to add self.right.left to self.left.right-most-pair.right
            //
            // self with left-hand pair ready to explode
            // need to mutate self.left to be 0
            // need to add self.left.right to self.right.left-most-pair.left
            // need to add self.left.left to...
            //   first parent where this descent was the right-hand
            //   parent.left-most-pair.right
            if left.is_sploding_pair(depth + 1) {
                let left_pair = left.unwrap_pair();
                let lefts_right_value = left_pair.1.unwrap_value();
                let lefts_left_value = left_pair.0.unwrap_value();
                right.add_to_left_most_value(lefts_right_value);
                *left = Box::new(Num::Value(0));
                // reverse delegate: lefts left value
                Some(ReductionResult::AddToLeftRight(lefts_left_value))
            } else if right.is_sploding_pair(depth + 1) {
                let right_pair = right.unwrap_pair();
                let rights_right_value = right_pair.1.unwrap_value();
                let rights_left_value = right_pair.0.unwrap_value();
                left.add_to_right_most_value(rights_left_value);
                *right = Box::new(Num::Value(0));
                // reverse delegate: rights right value
                Some(ReductionResult::AddToRightLeft(rights_right_value))
            } else {
                let s1 = left.splode_recurse(depth + 1);
                if s1.is_some() {
                    return match s1.unwrap() {
                        ReductionResult::AddToLeftRight(v) => {
                            Some(ReductionResult::AddToLeftRight(v))
                        },
                        ReductionResult::AddToLeftRightLater(v) => {
                            Some(ReductionResult::AddToLeftRight(v))
                        },
                        ReductionResult::AddToRightLeft(v) => {
                            right.add_to_left_most_value(v.clone());
                            Some(ReductionResult::Success)
                        },
                        ReductionResult::AddToRightLeftLater(v) => {
                            Some(ReductionResult::AddToRightLeft(v))
                        },
                        success => Some(success),
                    }
                }
                let s2 = right.splode_recurse(depth + 1);
                if s2.is_some() {
                    return match s2.unwrap() {
                        ReductionResult::AddToLeftRight(v) => {
                            left.add_to_right_most_value(v);
                            Some(ReductionResult::Success)
                        },
                        ReductionResult::AddToLeftRightLater(v) => {
                            Some(ReductionResult::AddToLeftRight(v))
                        },
                        ReductionResult::AddToRightLeft(v) => {
                            Some(ReductionResult::AddToRightLeft(v))
                        },
                        ReductionResult::AddToRightLeftLater(v) => {
                            Some(ReductionResult::AddToRightLeft(v))
                        },
                        success => Some(success),
                    }
                }
                None
            }
        } else {
            None
        }
    }
    fn apply_splits(&mut self) -> bool {
        match self {
            Num::Value(v) => {
                if *v < 10 {
                    false
                } else {
                    *self = Num::apply_split(v.clone());
                    true
                }
            }
            Num::Pair(pair) => {
                let l = pair.0.apply_splits();
                if l {
                    l
                } else {
                    pair.1.apply_splits()
                }
            }
        }
    }
    fn reduce(&mut self) -> bool {
        // let mut siblings: Vec<Descent> = vec![];
        let mut some_action = false;
        // println!("start {}", self.to_string_with_depth(0));
        loop {
            let mut this_iter_some_action = false;
            while self.splode_recurse(0).is_some() {
                // println!("      {}", self.to_string_with_depth(0));
                this_iter_some_action = true;
            }
            if self.apply_splits() {
                // println!("split");
                this_iter_some_action = true
            }
            some_action = some_action || this_iter_some_action;
            if !this_iter_some_action {
                return some_action;
            }
        }
    }
    fn get_magnitude(&self) -> i64 {
        match self {
            Num::Value(v) => v.clone() as i64,
            Num::Pair((left, right)) => {
                left.get_magnitude() * 3 + right.get_magnitude() * 2
            },
        }
    }
    fn to_string(&self) -> String {
        let mut s = "".to_string();
        match self {
            Num::Value(v) => s += &format!("{}", v),
            Num::Pair((l, r)) => {
                s += "";
                s += &l.to_string();
                s += ",";
                s += &r.to_string();
                s += "]";
            }
        }
        s
    }
    fn to_string_with_depth(&self, depth: usize) -> String {
        let mut s = "".to_string();
        match self {
            Num::Value(v) => s += &format!("{}", v),
            Num::Pair((l, r)) => {
                s += &format!("[{}d ", depth);
                s += &l.to_string_with_depth(depth + 1);
                s += ",";
                s += &r.to_string_with_depth(depth + 1);
                s += "]";
            }
        }
        s
    }
    fn add(&self, rhs: &Num) -> Num {
        Num::Pair((Box::new(self.clone()), Box::new(rhs.clone())))
    }
    // fn reduce(&self) -> Num {

    // }
}

impl Problem {
    fn from_file(path: &str) -> Result<Problem, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let lines: Vec<String> = file_str
            .split("\n")
            .map(|line| line.trim().to_string())
            .collect();
        let nums = lines
            .iter()
            .map(|l| {
                let mut i = 0;
                let n = Num::from_string(&mut i, l.trim());
                // println!("{:?}", n);
                n
            })
            .collect();
        Ok(Problem { nums })
    }
}

fn main() {
    let input_res = Problem::from_file("./src/input.txt");
    match input_res {
        Ok(mut input) => {
            let mut cur = input.nums[0].clone();
            let mut largest_mag = 0;
            for n in input.nums.iter().skip(1) {
                // println!("   {}", cur.to_string());
                // println!("+  {}", n.to_string());
                cur = cur.add(n);
                cur.reduce();
                // println!("=  {}", cur.to_string());
                // println!("");
            }
            println!("final sum {}", cur.to_string());
            println!("magnitude {}", cur.get_magnitude());
            let mut max = 0;
            for left in 0..input.nums.len() {
                for right in 0..input.nums.len() {
                    if left == right {
                        continue
                    }
                    let l = &input.nums[left];
                    let r = &input.nums[right];
                    let mut c = l.add(r);
                    c.reduce();
                    let mag = c.get_magnitude();
                    if mag > max {
                        max = mag 
                    }
                }
            }
            println!("best mag {}", max);
            // let packets = input.
            // let result = packets[0].evaluate();
            // println!("{:?}", input.nums);
        }
        Err(e) => println!("{}", e),
    }
}
