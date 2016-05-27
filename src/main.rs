#![allow(dead_code)]
fn main() {
    println!("Hello, world!");
}

fn selection_sort<T: PartialOrd + Clone>(list: &mut Vec<T>) -> Vec<T>{
    for x in 0..list.len() {
        let mut min = x;
        for j in (x + 1)..list.len() {
            if list[j] < list[min] {
                min = j;
            }
        }
        list.swap(x, min);
    }
    list.to_owned()
}

#[test]
fn selecton_sort_test() {
    let mut list = vec![8,6,4,9,3,4,5,10];
    let sorted = vec![3,4,4,5,6,8,9,10];
    let result = selection_sort(&mut list);
    assert_eq!(result, sorted);
}

fn selection_sort_abstraction<T>(list: &mut Vec<T>) -> Vec<T> where T: PartialOrd + Ord + Clone {
    for i in 0..list.len() {
        let min = list[i..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0);
        list.swap(i, min + i);
    }
    list.to_owned()
}

#[test]
fn selection_sort_abstraction_test() {
    let mut list = vec![8,6,4,9,3,4,5,10];
    let sorted = vec![3,4,4,5,6,8,9,10];
    let result = selection_sort_abstraction(&mut list);
    assert_eq!(result, sorted);
}

fn insertion_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        let tmp = list[i];
        let mut pos = i;
        while pos > 0 && list[pos - 1] > tmp {
            list.swap(pos, pos - 1);
            pos += 1;
        }
        list[pos] = tmp;
    }
}

fn bubble_sort<T: PartialOrd>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        for j in (i + 1)..list.len() {
            if list[i] > list[j] {
                list.swap(i, j);
            }
        }
    }
}

fn merge_sort<T: PartialOrd + Copy>(list: &mut Vec<T>, start: usize, end: usize) {
    if start <= end {
        let mid = list.len() / 2;
        merge_sort(list, start, mid);
        merge_sort(list, mid + 1, end);
        merge(list, start, mid, end);
    }
}

fn merge<T: PartialOrd + Copy>(list: &mut Vec<T>, start: usize, mid: usize, end: usize) {

    let mut temp = Vec::with_capacity(end - start + 1);
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i <= mid && j <= end {
        if list[i] < list[j] {
            temp[k] = list[i];
            i += 1;
        } else {
            temp[k] = list[j];
            j += 1;
        }
        k += 1;
    }
    while i <= mid {
        temp[k] = list[i];
        k += 1;
        i += 1;
    }
    while j <= mid {
        temp[k] = list[j];
        k += 1;
        j += 1;
    }
    i = start;
    for i in i..end {
        list[i] = temp[i - start];
    }
}
