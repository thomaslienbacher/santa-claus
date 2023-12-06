use mcmf::{GraphBuilder, Vertex, Cost, Capacity};
use rs_graph::traits::refs::IndexGraphRef;

struct Present {
    name: &'static str,
    quantity: i32,
}

fn P(name: &'static str, quantity: i32) -> Present {
    Present {
        name,
        quantity,
    }
}

struct Child {
    name: &'static str,
    wishlist: Vec<&'static str>,
}

fn C(name: &'static str, wishlist: Vec<&'static str>) -> Child {
    Child {
        name,
        wishlist,
    }
}


fn main() {
    use mcmf::{GraphBuilder, Vertex, Cost, Capacity};
    let mut g = GraphBuilder::new();

    let presents = vec![P("p1", 7), P("p2", 7), P("p3", 7), P("p4", 7)];
    //let children = vec![C("c1", vec!["p1", "p2"]), C("c2", vec!["p1", "p2", "p3", "p4"])];
    let children = vec![C("c1", vec!["p1", "p2"]), C("c2", vec!["p1", "p2", "p3", "p4"])];

    for p in &presents {
        g.add_edge(Vertex::Source, p.name, Capacity(p.quantity), Cost(0));

        for c in &children {
            if c.wishlist.contains(&p.name) {
                g.add_edge(p.name, c.name, Capacity(1), Cost(0));
            }
        }
    }
    for c in &children {
        g.add_edge(c.name, Vertex::Sink, Capacity(3), Cost(0));
    }

    let (_, paths) = g.mcmf();
    for p in &paths {
        for f in &p.flows {
            println!("{:?} -> {:?} with {:#?} ({})", f.a, f.b, f.amount, p.amount());
        }
    }

    // create tikz
    println!("tikz code: \n");

    println!(r"\node[state] (start) {{$S$}};");

    println!(r"\node[state] (p1) [below of = start] {{$p_1$}};");
    for i in 1..presents.len() {
        println!(r"\node[state] (p{}) [right of = p{}] {{$p_{}$}};", i + 1, i, i + 1);
    }

    println!(r"\node[state] (c1) [below of = p1] {{$c_1$}};");
    for i in 1..children.len() {
        println!(r"\node[state] (c{}) [right of = c{}] {{$c_{}$}};", i + 1, i, i + 1);
    }
    println!(r"\node[state] (end) [below of = c1] {{$E$}};");

    for p in &paths {
        for f in &p.flows {
            let a = match f.a {
                Vertex::Source => { "start" }
                Vertex::Sink => { "end" }
                Vertex::Node(t) => { t }
            };

            let b = match f.b {
                Vertex::Source => { "start" }
                Vertex::Sink => { "end" }
                Vertex::Node(t) => { t }
            };


            println!(r"\path[->,very thick,blue] ({}) edge [] node [right] {{{}}} ({});", a, f.amount, b);
        }
    }
}
