use std::io::stdin;

mod update_heap {
    use std::cmp::Ordering;

    pub struct Task {
        pub idx: usize,
        pub remaining_time: usize,
    }

    impl Task {
        pub fn new(idx: usize, time: usize) -> Self {
            Self {
                idx,
                remaining_time: time,
            }
        }
    }

    impl PartialEq<Self> for Task {
        fn eq(&self, other: &Self) -> bool {
            self.remaining_time == other.remaining_time
        }
    }

    impl PartialOrd<Self> for Task {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.remaining_time.partial_cmp(&other.remaining_time)
        }
    }

    pub struct UpdateHeap {
        data: Vec<Task>,
    }

    impl UpdateHeap {
        pub fn new() -> Self {
            Self { data: Vec::new() }
        }

        pub fn is_empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn push(&mut self, item: Task) {
            self.data.push(item);
            let idx = self.data.len() - 1;
            self.sift_up(idx);
        }

        pub fn pop(&mut self) -> Option<Task> {
            if self.is_empty() {
                return None;
            }

            let last_idx = self.data.len() - 1;
            self.data.swap(0, last_idx);
            let ret = self.data.pop().unwrap();
            if !self.data.is_empty() {
                self.sift_down(0);
            }
            Some(ret)
        }

        fn sift_up(&mut self, mut idx: usize) {
            while idx > 0 {
                let parent = (idx - 1) / 2;
                if self.data[parent] > self.data[idx] {
                    self.data.swap(parent, idx);
                    idx = parent;
                } else {
                    break;
                }
            }
        }

        fn sift_down(&mut self, mut idx: usize) {
            while idx < self.data.len() / 2 {
                let left = 2 * idx + 1;
                let right = 2 * idx + 2;
                let mut child = left;
                if right < self.data.len() && self.data[right] < self.data[left] {
                    child = right;
                }
                if self.data[child] < self.data[idx] {
                    self.data.swap(child, idx);
                    idx = child;
                } else {
                    break;
                }
            }
        }

        pub fn update(&mut self, dt: usize) {
            self.data.iter_mut().for_each(|x| x.remaining_time -= dt);
        }
    }
}

use crate::update_heap::Task;
use update_heap::UpdateHeap;

fn get_inv_dependencies_edges(tasks: &Vec<(usize, Vec<usize>)>) -> Vec<Vec<usize>> {
    let mut inv_dependencies = vec![vec![]; tasks.len()];
    for i in 0..tasks.len() {
        for &v in tasks[i].1.iter() {
            inv_dependencies[v].push(i);
        }
    }
    inv_dependencies
}

fn get_minimum_cost_of_tasks(tasks: &Vec<(usize, Vec<usize>)>) -> usize {
    let inv_dependencies = get_inv_dependencies_edges(tasks);

    let mut indeg: Vec<usize> = vec![0; tasks.len()];
    for v in inv_dependencies.iter() {
        for &w in v.iter() {
            indeg[w] += 1;
        }
    }

    let mut heap: UpdateHeap = UpdateHeap::new();

    for (i, deg) in indeg.iter().enumerate() {
        if *deg == 0 {
            heap.push(Task::new(i, tasks[i].0));
        }
    }

    let mut total_time = 0;

    while !heap.is_empty() {
        let task = heap.pop().unwrap();

        total_time += task.remaining_time;
        heap.update(task.remaining_time);
        inv_dependencies[task.idx].iter().for_each(|&v| {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                heap.push(Task::new(v, tasks[v].0));
            }
        });
    }

    total_time
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    let n = line.trim_end().parse::<usize>().unwrap();
    line.clear();
    let tasks = (0..n)
        .map(|_| {
            stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            let cost = iter.next().unwrap().parse::<usize>().unwrap();
            let _ = iter.next();
            let depends = iter
                .map(|x| x.parse::<usize>().unwrap() - 1)
                .collect::<Vec<usize>>();
            line.clear();
            (cost, depends)
        })
        .collect::<Vec<(usize, Vec<usize>)>>();

    println!("{}", get_minimum_cost_of_tasks(&tasks));
}
