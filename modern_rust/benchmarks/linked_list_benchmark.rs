use std::time::Instant;

#[derive(Debug)]
struct Node {
    data: i32,
    prox: Option<Box<Node>>,
}

fn build_list() -> Option<Box<Node>> {
    let mut prim: Option<Box<Node>>;
    let p: Option<Box<Node>>;
    let aux: Option<Box<Node>>;

    prim = Some(Box::new(Node {
        data: 2,
        prox: None,
    }));

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

    prim
}

fn traverse(mut node: Option<&Box<Node>>) {
    while let Some(n) = node {
        let _ = n.data;
        node = n.prox.as_ref();
    }
}

fn main() {
    let iterations = 100_000;

    let start = Instant::now();

    for _ in 0..iterations {
        let list = build_list();
        traverse(list.as_ref());
    }

    let duration = start.elapsed();

    println!("Linked List Benchmark");
    println!("Nodes        : 3");
    println!("Iterations   : {}", iterations);
    println!("Elapsed time : {:?}", duration);
}

