use std::collections::HashMap;
use std::process::id;
use rs_graph::Builder;
use rs_graph::maxflow::{edmondskarp, EdmondsKarp};
use rs_graph::traits::DirectedEdge;
use rs_graph::vecgraph::VecGraphBuilder;
use rs_graph::vecgraph::Node;

struct Present {
    name: &'static str,
    quantity: i32,
    node: Option<Node>,
}

fn P(name: &'static str, quantity: i32) -> Present {
    Present {
        name,
        quantity,
        node: None,
    }
}

struct Child {
    name: &'static str,
    wishlist: Vec<&'static str>,
    node: Option<Node>,
}

fn C(name: &'static str, wishlist: Vec<&'static str>) -> Child {
    Child {
        name,
        wishlist,
        node: None,
    }
}

fn main() {
    let mut presents = vec![
        P("p1", 2),
        P("p2", 1),
        P("p3", 3),
        P("p4", 4),
        P("p5", 3),
    ];
    let mut children = vec![
        C("c1", vec!["p1", "p2"]),
        C("c2", vec!["p1", "p2", "p3", "p4"]),
        C("c3", vec!["p4", "p5"]),
    ];

    let mut g = VecGraphBuilder::new();
    let mut capacity_mappings = HashMap::new();
    let mut edge_mappings = HashMap::new();

    let start = g.add_node();
    let end = g.add_node();

    for p in &mut presents {
        p.node = Some(g.add_node());
    }
    for c in &mut children {
        c.node = Some(g.add_node());
    }

    for p in &presents {
        let e = g.add_edge(start, p.node.unwrap());
        capacity_mappings.insert(e, p.quantity);
        edge_mappings.insert(e, ("start", p.name, p.quantity));

        for c in &children {
            if c.wishlist.contains(&p.name) {
                let m = g.add_edge(p.node.unwrap(), c.node.unwrap());
                capacity_mappings.insert(m, 1);
                edge_mappings.insert(m, (p.name, c.name, 1));
            }
        }
    }

    for c in &mut children {
        let d = g.add_edge(c.node.unwrap(), end);
        capacity_mappings.insert(d, 3);
        edge_mappings.insert(d, (c.name, "end", 3));
    }

    let graph = g.into_graph();
    let mut ek = EdmondsKarp::new(&graph);
    ek.solve(start, end, |e| {
        let cap = capacity_mappings.get(&e);
        println!("{:?} => {:?}", e, cap);
        *cap.unwrap_or(&1)
    });

    println!("max flow: {}", ek.value());

    // create tikz
    println!("tikz code: \n");

    let y_step = 2.5;
    let x_step = 2.5;

    println!(r"\node[state] (start) at (0, 0) {{$S$}};");

    let total = (presents.len() - 1) as f64 * x_step;
    for i in 0..presents.len() {
        let x = -total * 0.5 + i as f64 * x_step;
        println!(r"\node[state] (p{}) at ({x}, {}) {{$p_{}$}};", i + 1, -y_step, i + 1);
    }

    let total = (children.len() - 1) as f64 * x_step;
    for i in 0..children.len() {
        let x = -total * 0.5 + i as f64 * x_step;
        println!(r"\node[state] (c{}) at ({x}, {}){{$c_{}$}};", i + 1, -2.0 * y_step, i + 1);
    }
    println!(r"\node[state] (end) at (0, {}) {{$E$}};", -3.0 * y_step);

    for (e, f) in ek.flow_iter() {
        let (a, b, c) = edge_mappings[&e];
        let pos = if a == "start" || b == "end" {
            0.5
        } else {
            0.3
        };
        if f == 0 {
            println!(r"\path[->] ({}) edge [] node [right, pos={pos}] {{\color{{black}} \scriptsize {}/{}}} ({});", a, f, c, b);
        } else {
            println!(r"\path[->, thick, red] ({}) edge [] node [right, pos={pos}] {{\color{{black}} \scriptsize {}/{}}} ({});", a, f, c, b);
        }
    }
}
