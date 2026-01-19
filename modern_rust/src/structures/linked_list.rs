/*
 * ORIGINAL FILE: PONTEIRO2.PAS
 *
 * Demonstrates basic operations on a singly linked list
 * using explicit pointer manipulation.
 *
 * This Rust version preserves the same low-level, procedural
 * structure found in the original Pascal code.
 */

#[derive(Debug)]
struct Node {
    data: i32,
    prox: Option<Box<Node>>,
}

fn main() {
    let mut prim: Option<Box<Node>>;
    let mut p: Option<Box<Node>>;
    let mut aux: Option<Box<Node>>;
    let mut aux2: Option<Box<Node>>;

    /* first node */
    prim = Some(Box::new(Node {
        data: 2,
        prox: None,
    }));

    /* insert at beginning */
    p = Some(Box::new(Node {
        data: 6,
        prox: prim,
    }));
    prim = p;

    aux = Some(Box::new(Node {
        data: 9,
        prox: prim,
    }));
    prim = aux;

    /* print list (until last node) */
    {
        let mut cursor = prim.as_deref();
        while let Some(node) = cursor {
            if node.prox.is_none() {
                break;
            }
            print!("{} ", node.data);
            cursor = node.prox.as_deref();
        }
        println!();
    }

    /* restore head */
    prim = aux;

    /* search */
    let mut cursor = prim.as_deref_mut();
    while let Some(node) = cursor {
        if node.data == 2 {
            break;
        }
        print!("{} ", node.data);
        cursor = node.prox.as_deref_mut();
    }
    println!();

    /* insert after the search */
    aux2 = Some(Box::new(Node {
        data: 10,
        prox: None,
    }));

    if let Some(node) = cursor {
        node.prox = aux2;
    }

    /* erase (reset traversal pointer) */
    aux = p;

    /* verification */
    println!();
    let mut cursor = aux.as_deref();
    while let Some(node) = cursor {
        print!("{} ", node.data);
        cursor = node.prox.as_deref();
    }
    println!();
}

