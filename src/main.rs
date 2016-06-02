#![allow(dead_code)]
extern crate rayon;

use std::fmt::{Display, Debug};
use std::cmp::Ordering;

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

#[test]
fn binary_search_test() {
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    assert!(recursive_binsearch(&sorted, 5));
}

// fn recursive_binsearch<T: Ord>(list: &[T], target: T) -> bool {
//     if list.is_empty() {
//         return false;
//     }
//     let guess = list.len() / 2;
//     if target == list[guess] {
//         true
//     } else if list[guess] > target {
//         recursive_binsearch(&list[..guess], target)
//     } else {
//         recursive_binsearch(&list[guess+1..], target)
//     }
// }

fn recursive_binsearch<T: Ord>(list: &[T], target: T) -> bool {
    if list.is_empty() {
        return false;
    }
    let guess = list.len() / 2;
    match target.cmp(&list[guess]) {
        Ordering::Less => recursive_binsearch(&list[..guess], target),
        Ordering::Greater => recursive_binsearch(&list[guess + 1..], target),
        Ordering::Equal => true,
    }
}

#[test]
fn recursive_binsearch_test() {
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    assert!(recursive_binsearch(&sorted, 4));
    assert!(!recursive_binsearch(&sorted, -3));
    assert!(!recursive_binsearch(&sorted, 14));
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
    merge(&mut list[0..len]);
}

fn merge<T: Clone + Ord>(list: &mut [T]) {
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

fn par_mergesort<T: Clone + Ord + Send + Debug>(list: &mut [T]) -> Vec<T> {
    let (len, mid) = (list.len(), list.len() / 2);
    if len <= 1 {
        return list.to_owned();
    }

    let (left, right) = list.split_at_mut(mid);
    println!("splitting {:?} -- {:?}", left, right);
    // rayon::join(|| par_mergesort(left), || par_mergesort(right));
    par_mergesort(left);
    par_mergesort(right);

    merge_two(left, right)
}

fn merge_two<T: Clone + Ord + Debug>(left: &mut [T], right: &mut [T]) -> Vec<T> {
    println!("merging {:?} -- {:?}", left, right);
    let (mut i, mut j) = (0, 0);
    let mut b = Vec::with_capacity(left.len() + right.len());
    while i < left.len() && j < right.len() {
        println!("{:?} - {:?}", left[i], right[j]);
        if left[i] < right[j] {
            println!("less {:?} {:?}", left[i], right[j]);
            b.push(left[i].clone());
            i += 1;
        } else {
            println!("greater {:?} {:?}", left[i], right[j]);
            b.push(right[j].clone());
            j += 1;
        }
    }
    println!("cmp {:?}", b);
    if left.len() > 0 {
        b.extend_from_slice(&left[i..left.len()]);
    }
    if right.len() > 0 {
        b.extend_from_slice(&right[j..right.len()]);
    }

    b
}

#[test]
fn par_mergesort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    let result = par_mergesort(&mut list);
    assert_eq!(*result, *sorted);
}

fn quicksort<T: Ord>(list: &mut [T]) {
    if list.len() <= 1 {
        return;
    }
    let pivot = partition(list);
    let (lo, hi) = list.split_at_mut(pivot);
    quicksort(lo);
    quicksort(hi);
}

fn par_quicksort<T: Ord + Send>(list: &mut [T]) {
    if list.len() <= 1 {
        return;
    }
    let pivot = partition(list);
    let (lo, hi) = list.split_at_mut(pivot);
    rayon::join(|| par_quicksort(lo), || par_quicksort(hi));
}

fn partition<T: Ord>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

#[test]
fn quicksort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    quicksort(&mut list);
    assert_eq!(*list, *sorted);
}

#[test]
fn par_quicksort_test() {
    let mut list = vec![8, 6, 4, 9, 3, 4, 5, 10];
    let sorted = vec![3, 4, 4, 5, 6, 8, 9, 10];
    par_quicksort(&mut list);
    assert_eq!(*list, *sorted);
}
