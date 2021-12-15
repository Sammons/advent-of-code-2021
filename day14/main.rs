use std::{
    collections::{HashMap, HashSet, LinkedList},
    iter::FromFn, ops::DerefMut,
};

#[derive(Debug)]
struct HListEl<T> {
    id: usize,
    value: T,
    // prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Debug)]
struct HList<T> {
    elements: HashMap<usize, HListEl<T>>,
    counter: usize,
    first: Option<usize>,
    last: Option<usize>,
}
impl<T> HList<T> {
    fn vec(&self) -> Vec<usize> {
        let mut cur = self.first;
        let all_elements = std::iter::from_fn(move || {
            let to_return: &HListEl<T> = cur.map(|id| self.elements.get(&id))??;
            cur = to_return.next;
            Some(to_return.id)
        });
        all_elements.collect()
    }
    fn get_mut(&mut self, id: &usize) -> Option<&mut HListEl<T>> {
        self.elements.get_mut(&id)
    }
    fn get_val(&self, id: &usize) -> Option<T>
    where
        T: Clone,
    {
        self.elements.get(&id).map(|e| e.value.clone())
    }
    fn get_next_val(&self, id: &usize) -> Option<T>
    where
        T: Clone,
    {
        let next = self.next_of(id);
        next.map(|e| e.value.clone())
    }
    fn get_next_id(&self, id: &usize) -> Option<usize> {
        let next = self.next_of(id);
        next.map(|e| e.id)
    }
    fn append(&mut self, el: T) {
        self.counter += 1;
        let id = self.counter;
        let new_el = HListEl {
            id,
            value: el,
            // prev: self.last,
            next: None,
        };
        if self.last.is_some() {
            let last = self.get_mut(&self.last.unwrap());
            last.unwrap().next = Some(id);
        }
        if self.first.is_none() {
            self.first = Some(id);
        }
        self.last = Some(id);
        self.elements.insert(id, new_el);
    }
    fn insert_after(&mut self, id: usize, value: T) {
        self.counter += 1;
        let new_id = self.counter;
        let prev = self.elements.get_mut(&id).unwrap();
        let new_el = HListEl {
            id: new_id,
            value,
            // prev: Some(id),
            next: prev.next,
        };
        // configure next to point to new prev
        if prev.next.is_none() {
            self.last = Some(new_id);
        }
        // configure prev to point to new next
        prev.next = Some(new_id);
        self.elements.insert(new_id, new_el);
    }
    fn next_of(&self, id: &usize) -> Option<&HListEl<T>> {
        let next = self
            .elements
            .get(id)
            .map(|e| e.next)
            .flatten()
            .map(|next_id| self.elements.get(&next_id))??;

        Some(&next)
    }
}

struct Input {
    polymer: HList<char>,
    pairs: HashMap<(char, char), char>,
}

impl Input {
    fn from_file(path: &str) -> Result<Input, String> {
        let res = std::fs::read_to_string(path);
        let file_str = res.map_err(|e| e.to_string())?;
        let lines: Vec<String> = file_str
            .split("\n")
            .map(|line| line.trim().to_string())
            .collect();
        // let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
        let mut polymer: HList<char> = HList {
            elements: HashMap::new(),
            counter: 0,
            first: None,
            last: None,
        };
        for c in lines[0].chars() {
            polymer.append(c.to_owned());
        }
        let mut pairs: HashMap<(char, char), char> = HashMap::new();
        for line in lines.iter().skip(1) {
            if line.trim().len() == 0 {
                continue;
            } else {
                let pieces: Vec<&str> = line.trim().split(" -> ").collect();
                let left: Vec<char> = pieces[0].chars().collect();
                let right = pieces[1].chars().collect::<Vec<char>>()[0];
                pairs.insert((left[0], left[1]), right);
            }
        }
        Ok(Input { polymer, pairs })
    }
}

impl Input {
    fn poly_to_s(&self) -> String {
        self.polymer
            .vec()
            .iter()
            .flat_map(|v| self.polymer.get_val(v))
            .collect::<String>()
    }
    fn print(&self) {
        let s = self.poly_to_s();
        println!("{}", s);
    }
    fn tally(&self) {
        let ids = self.polymer.vec();
        let mut frequencies: HashMap<char, usize> = HashMap::new();
        for id in ids {
            let v = self.polymer.get_val(&id).unwrap();
            if !frequencies.contains_key(&v) {
                frequencies.insert(v, 0);
            }
            let counter = frequencies.get_mut(&v).unwrap();
            *counter += 1;
        }
        let max = frequencies.values().max().unwrap();
        let min = frequencies.values().min().unwrap();
        println!("{} - {} = {}", max, min, max - min);
        println!("{:?}", frequencies);
    }
    fn discover_recurse(
        &self,
        polymer: &str,
        depth: usize,
        global_occurrences: &mut HashMap<char, usize>,
        memo: &mut HashMap<(String, usize), Vec<(char, usize)>>
    ) {
        let mut apply_freq = |freq: &Vec<(char, usize)>| {
            for (char, count) in freq {
                match global_occurrences.get_mut(char) {
                    Some(char_count) => {
                        *char_count += count;
                    },
                    None => {
                        global_occurrences.insert(*char, *count);
                    }
                }
            }
        };
        if memo.contains_key(&(polymer.to_string(), depth)) {
           let freq = memo.get(&(polymer.to_string(), depth)).unwrap();
           apply_freq(freq);
           return;
        }
        if depth == 0 {
            return;
        }
        let mut occurrences = HashMap::new();
        for idx in 0..polymer.len() - 1 {
            let pair: &str = &polymer[idx..idx + 2];
            let c1 = pair.as_bytes()[0] as char;
            let c2 = pair.as_bytes()[1] as char;
            if self.pairs.contains_key(&(c1, c2)) {
                let insertion = self.pairs.get(&(c1, c2));
                if let Some(insert_char) = insertion {
                    match occurrences.get_mut(insert_char) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            occurrences.insert(*insert_char, 1);
                        }
                    }
                    let left = c1.to_string() + &insert_char.to_string();
                    let right = insert_char.to_string() + &c2.to_string();
                    self.discover_recurse(&left, depth - 1, &mut occurrences, memo);
                    self.discover_recurse(&right, depth - 1, &mut occurrences, memo);
                }
            }
        }
        let memo_val: Vec<(char, usize)> = occurrences.iter().map(|(c,s)| (*c, *s)).collect();
        apply_freq(&memo_val);
        memo.insert((polymer.to_string(), depth), memo_val);
    }
    fn discover(&self, depth: usize) {
        let mut occurrences = HashMap::new();
        let mut memo = HashMap::new();
        let poly_s = self.poly_to_s();
        for c in poly_s.chars() {
            match occurrences.get_mut(&c) {
                Some(count) => {
                    *count += 1;
                }
                None => {
                    occurrences.insert(c, 1);
                }
            }
        }
        self.discover_recurse(&poly_s, depth, &mut occurrences, &mut memo);
        let max = occurrences.values().max().unwrap();
        let min = occurrences.values().min().unwrap();
        println!("{} - {} = {}", max, min, max - min);
        println!("{:?}", occurrences);
    }
    fn tick(&mut self) {
        let mut cur = self.polymer.first;
        let list = std::iter::from_fn(move || {
            let cur_id = cur?;
            let next = self.polymer.get_next_id(&cur_id);
            {
                let cur_el = self.polymer.get_val(&cur_id);
                let next_el = self.polymer.get_next_val(&cur_id);
                if cur_el.is_some() && next_el.is_some() {
                    let cur_char = cur_el.unwrap();
                    let next_char = next_el.unwrap();
                    let tup = (cur_char, next_char);
                    if self.pairs.contains_key(&tup) {
                        let to_insert = self.pairs.get(&tup).unwrap();
                        self.polymer.insert_after(cur_id, to_insert.to_owned());
                    }
                }
            }
            cur = next;
            next
        });
        for _ in list {}
    }
}

fn main() {
    let input_res = Input::from_file("./src/input.txt");
    match input_res {
        Ok(mut input) => {
            input.print();
            // let n = 14;
            // for i in 0..n {
            //     input.tick();
            //     // println!("{} ->", i + 1);
            //     // input.print();
            //     println!("{}", i);
            // }
            // input.tally();
            input.discover(40);
            println!("Done");
        }
        Err(e) => println!("{}", e),
    }
}
