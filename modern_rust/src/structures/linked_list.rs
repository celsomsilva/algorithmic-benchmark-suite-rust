/*
 * ORIGINAL FILE: PONTEIRO2.PAS
 *
 * Demonstrates basic operations on a singly linked list
 * using explicit pointer manipulation.
 *
 * This Rust version preserves the same low-level, procedural
 * structure found in the original Pascal code.
 * This file contains a standalone Rust translation of a C algorithm.
 * Functions may appear unused because they are meant for comparison,
 * benchmarking, or standalone execution.
 *![allow(dead_code)]
 */
 
// In Rust, linked lists are not copied.
// Each node owns the next one, and assignments transfer ownership.
// This differs from C and Pascal pointer-based implementations.


#[derive(Debug)]
struct Node {
    data: i32,
    prox: Option<Box<Node>>,
}

fn main() {
    let mut prim: Option<Box<Node>>;

    /* first node */
    prim = Some(Box::new(Node {
        data: 2,
        prox: None,
    }));

    /* insert at beginning */
    prim = Some(Box::new(Node {
        data: 6,
        prox: prim.take(),
    }));

    prim = Some(Box::new(Node {
        data: 9,
        prox: prim.take(),
    }));

    /* print list */
    {
        let mut cursor = prim.as_deref();
        while let Some(node) = cursor {
            print!("{} ", node.data);
            cursor = node.prox.as_deref();
        }
        println!();
    }

    /* search */
    let mut cursor = prim.as_deref_mut();
    while let Some(node) = cursor {
        if node.data == 2 {
            /* insert after */
            node.prox = Some(Box::new(Node {
                data: 10,
                prox: node.prox.take(),
            }));
            break;
        }
        cursor = node.prox.as_deref_mut();
    }

    /* verification */
    println!();
    let mut cursor = prim.as_deref();
    while let Some(node) = cursor {
        print!("{} ", node.data);
        cursor = node.prox.as_deref();
    }
    println!();
}


