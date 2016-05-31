#![allow(dead_code)]

use std::fmt::{Display, Debug};

fn main() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    mergesort(&mut list);
    println!("{:?}", list);
    println!("{:?}", sorted);
}

fn binary_search<T: Ord>(list: &Vec<T>, target: T) -> bool {
    let (mut min, max) = (0, list.len() - 1);
    let mut guess;
    while min <= max {
        guess = (min + max) / 2;
        if list[guess] == target {
            return true;
        } else if list[guess] < target {
            min = guess + 1;
        } else {
            min = guess - 1;
        }
    }
    return false;
}

fn recursive_binary_search<T: Ord>(list: &[T], target: T) -> bool {
    if list.len() < 1 {
        return false;
    }
    let guess = list.len() / 2;
    if target == list[guess] {
        return true;
    } else if list[guess] > target {
        return recursive_binary_search(&list[0..guess], target);
    } else {
        return recursive_binary_search(&list[guess..list.len()], target);
    }
}

fn selection_sort<T: PartialOrd>(list: &mut Vec<T>) {
    for x in 0..list.len() {
        let mut min = x;
        for j in (x + 1)..list.len() {
            if list[j] < list[min] {
                min = j;
            }
        }
        if x != min {
            list.swap(x, min);
        }
    }
}

#[test]
fn selecton_sort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    selection_sort(&mut list);
    assert_eq!(*list, *sorted);
}

fn selection_sort_abstraction<T: Ord>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        let min = list[i..]
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| v)
            .map(|(idx, _)| idx)
            .unwrap_or(0);
        if min != 0 {
            list.swap(i, min + i);
        }
    }
}

#[test]
fn selection_sort_abstraction_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    selection_sort_abstraction(&mut list);
    assert_eq!(*list, *sorted);
}

fn insertion_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        let tmp = list[i]; // save our item
        let mut pos = i; // and position
        // look back in array, swap each position that's bigger than us
        while pos > 0 && list[pos - 1] > tmp {
            // list.swap(pos, pos - 1);
            list[pos] = list[pos - 1];
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
    assert_eq!(*list, *sorted);
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
    assert_eq!(*list, *sorted);
}

// original implementation
// fn merge_sort<T: Ord + Copy>(list: &mut Vec<T>, start: usize, end: usize) {
//     if start >= end {
//         return;
//     }
//     let mid = (start + end) / 2;
//     merge_sort(list, start, mid);
//     merge_sort(list, mid + 1, end);
//     merge(list, start, mid, end);
// }
//
// fn merge<T: Ord + Copy>(list: &mut Vec<T>, start: usize, mid: usize, end: usize) {
//     let len = start + end - 1;
//     if len <= 1 {
//         return;
//     }
//     let mut temp = Vec::with_capacity(len);
//     let (mut i, mut j) = (start, mid + 1);
//     while i <= mid && j <= end {
//         if list[i] < list[j] {
//             temp.push(list[i]);
//             i += 1;
//         } else {
//             temp.push(list[j]);
//             j += 1;
//         }
//     }
//     while i <= mid {
//         temp.push(list[i]);
//         i += 1;
//     }
//     while j <= mid {
//         temp.push(list[j]);
//         j += 1;
//     }
//     i = start;
//     for i in i..end {
//         list[i] = temp[i - start];
//     }
// }


fn mergesort<T: Clone + Ord>(list: &mut [T]) {
    let (len, mid) = (list.len(), list.len() / 2);
    if len <= 1 {
        return;
    }
    mergesort(&mut list[0..mid]);
    mergesort(&mut list[mid..len]);
    merge_two(&mut list[0..len]);
}

fn merge_two<T: Clone + Ord>(list: &mut [T]) {
    let (alen, mid) = (list.len(), list.len() / 2);
    if alen <= 1 {
        return;
    }
    let (mut i, mut j) = (0, mid);
    let mut b = Vec::with_capacity(alen);
    while i < mid && j < alen {
        if list[i] < list[j] {
            b.push(list[i].clone());
            i += 1;
        } else {
            b.push(list[j].clone());
            j += 1;
        }
    }
    b.extend_from_slice(&list[i..mid]);
    b.extend_from_slice(&list[j..alen]);

    let (mut n, mut m, blen) = (0, 0, b.len());
    while n < alen && m < blen {
        list[n] = b[m].clone();
        n += 1;
        m += 1;
    }
}

#[test]
fn mergesort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    mergesort(&mut list);
    assert_eq!(*list, *sorted);
}

fn quicksort<T: Ord>(list: &mut [T]) {
    let (start, end) = (0, list.len());
    if end > start {
        let pivot = partition(list);
        quicksort(&mut list[start..(pivot - 1)]);
        quicksort(&mut list[(pivot + 1)..end])
    }
}
fn partition<T: Ord>(list: &mut [T]) -> usize {
    let len = list.len();
    let pivot = len / 2;

    list.swap(pivot, len - 1);

    let mut idx = 0;
    for i in 0..len - 1 {
        if &list[i] < &list[len - 1] {
            list.swap(i, idx);
            idx += 1;
        }
    }

    list.swap(idx, len - 1);
    idx
}

#[test]
fn quicksort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    quicksort(&mut list);
    assert_eq!(*list, *sorted);
}
