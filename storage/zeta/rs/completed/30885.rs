use std::io::stdin;
use std::ptr;

struct Cell {
    value: u64,
    index: usize,
    left: *mut Cell,
    right: *mut Cell,
}

impl Cell {
    fn new(value: u64, index: usize) -> *mut Cell {
        Box::into_raw(Box::new(Cell {
            value,
            index,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }))
    }
}

fn last_man_standing(mut head: *mut Cell, mut tail: *mut Cell) -> *mut Cell {
    let mut cnt = unsafe { (*tail).index + 1 };
    let mut curr = head;

    while cnt > 1 {
        curr = head;

        while true {
            unsafe {
                let left = (*curr).left;
                let right = (*curr).right;
                let v = (*curr).value;

                if !left.is_null() && (*left).value <= v {
                    // eat left
                    (*curr).value += (*left).value;
                    // remove left
                    let left_left = (*left).left;
                    if left == head {
                        head = curr;
                    }

                    (*curr).left = left_left;
                    if !left_left.is_null() {
                        (*left_left).right = curr;
                    }
                    cnt -= 1;
                }
                if !right.is_null() && (*right).value <= v {
                    // eat right
                    (*curr).value += (*right).value;
                    // remove right
                    let right_right = (*right).right;
                    if ptr::eq(right, tail) {
                        tail = curr;
                    }
                    (*curr).right = right_right;
                    if !right_right.is_null() {
                        (*right_right).left = curr;
                    }
                    cnt -= 1;
                }

                if ptr::eq(curr, tail) {
                    break;
                }
                curr = (*curr).right;
            }
        }
    }
    head
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let n: usize = line.trim().parse().unwrap();

    line.clear();
    stdin().read_line(&mut line).unwrap();

    let a: Vec<u64> = line
        .split(' ')
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    let mut ptrs: Vec<*mut Cell> = Vec::new();
    let head = Cell::new(a[0], 0);
    let tail = Cell::new(a[n - 1], n - 1);
    ptrs.push(head);
    let mut curr = head;

    for (i, v) in a.into_iter().enumerate().skip(1).take(n - 2) {
        let cell = Cell::new(v, i);
        unsafe {
            (*cell).left = curr;
            (*curr).right = cell;
            curr = cell;
        }
        ptrs.push(cell);
    }
    unsafe {
        (*tail).left = curr;
        (*curr).right = tail;
    }

    ptrs.push(tail);

    let last = last_man_standing(head, tail);
    unsafe {
        println!("{} {}", (*last).value, (*last).index + 1);
    }

    unsafe {
        for ele in ptrs {
            Box::from_raw(ele);
        }
    }
}
