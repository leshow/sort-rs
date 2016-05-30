#![allow(dead_code)]

use std::fmt::Display;

fn main() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    insertion_sort(&mut list);
    println!("{:?}", list);
    assert_eq!(list, sorted);
}

fn selection_sort<T: PartialOrd>(list: &mut Vec<T>) {
    for x in 0..list.len() {
        let mut min = x;
        for j in (x + 1)..list.len() {
            if list[j] < list[min] {
                min = j;
            }
        }
        list.swap(x, min);
    }
}

#[test]
fn selecton_sort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    selection_sort(&mut list);
    assert_eq!(list, sorted);
}

fn selection_sort_abstraction<T: Ord>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        let min = list[i..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0);
        list.swap(i, min + i);
    }
}

#[test]
fn selection_sort_abstraction_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    let result = selection_sort_abstraction(&mut list);
    assert_eq!(result, sorted);
}

fn insertion_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        let tmp = list[i]; // save our item
        let mut pos = i; // and position
        // look back in array, swap each position that's bigger than us
        while pos > 0 && list[pos - 1] > tmp {
            list.swap(pos, pos - 1);
            pos -= 1;
        }
        // found the spot to insert
        list[pos] = tmp;
    }
}

#[test]
fn insertion_sort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    insertion_sort(&mut list);
    assert_eq!(list, sorted);
}

fn bubble_sort<T: Ord>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        for j in (i + 1)..list.len() {
            if list[i] > list[j] {
                list.swap(i, j);
            }
        }
    }
}

#[test]
fn bubble_sort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    bubble_sort(&mut list);
    assert_eq!(list, sorted);
}

fn merge_sort<T: Ord>(list: &mut [T]) {
    let len = list.len();
    if len <= 1 {
        return;
    }
    let mid = list.len() / 2;
    let (l1, l2) = list.split_at_mut(mid);
    merge_sort(l1);
    merge_sort(l2);
    merge(list, start, mid, end);
}

fn merge<T: Ord + Copy>(list: &mut [T], start: usize, mid: usize, end: usize) {
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

#[test]
fn merge_sort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    let length = list.len();
    let result = merge_sort(&mut list, 0, length);
    assert_eq!(result, sorted);
}
