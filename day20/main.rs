use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize
}

impl Node {
    fn new(x: usize, y: usize) -> Node {
        Node { x: x, y: y }
    }
}

#[derive(Eq)]
struct NodeWithDistance {
    id: usize,
    dist: usize
}

impl Ord for NodeWithDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for NodeWithDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl PartialEq for NodeWithDistance {
    fn eq(&self, other: &Self) -> bool {
        other.dist == self.dist
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    a: usize,
    b: usize
}

impl Edge {
    fn new(a: usize, b: usize) -> Edge {
        Edge { a: a, b: b }
    }
}

#[derive(Debug)]
struct Portal {
    name: String,
    i1: usize,
    x1: usize,
    y1: usize,
    i2: usize,
    x2: usize,
    y2: usize
}

impl Portal {
    fn new(name: &str, i: usize, x: usize, y: usize) -> Portal {
        Portal { name: name.to_owned(), i1: i, x1: x, y1: y, i2: 0, x2: 0, y2: 0 }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut map = Vec::new();
    let mut width = 0;
    let mut node_count = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '\n' && width == 0 {
            width = i;
        }
        if c != '\n' {
            map.push(c);
            if c == '.' {
                node_count += 1;
            }
        }
    }

    let mut nodes: Vec<Node> = Vec::with_capacity(node_count);
    let mut edges = Vec::with_capacity(node_count * 2);
    let mut portals: Vec<Portal> = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let height = map.len() / width;
    for (i, cell) in map.iter().enumerate() {
        if *cell == '.' {
            let x = i % width;
            let y = i / width;
            // add new edges
            if x > 0 {
                if let Some(index) = nodes.iter().position(|n| n.x == x - 1 && n.y == y) {
                    edges.push(Edge::new(nodes.len(), index));
                    edges.push(Edge::new(index, nodes.len()));
                }
            }
            if y > 0 {
                if let Some(index) = nodes.iter().position(|n| n.x == x && n.y == y - 1) {
                    edges.push(Edge::new(nodes.len(), index));
                    edges.push(Edge::new(index, nodes.len()));
                }
            }
            // find any portals
            for dir in 0..4 {
                let (cell_a, cell_b) = match dir {
                    0 => { // top
                        if i >= width * 2 {
                            (map[i - width * 2], map[i - width])
                        } else {
                            break;
                        }
                    },
                    1 => { // bottom
                        if i < map.len() - width * 2 {
                            (map[i + width], map[i + width * 2])
                        } else {
                            break;
                        }
                    },
                    2 => { // left
                        if i % width > 1 {
                            (map[i - 2], map[i - 1])
                        } else {
                            break;
                        }
                    },
                    3 => { //right
                        if i % width < width - 2 {
                            (map[i + 1], map[i + 2])
                        } else {
                            break;
                        }
                    }
                    _ => panic!("invalid direction")
                };

                if 'A' <= cell_a && cell_a <= 'Z' && 'A' <= cell_b && cell_b <= 'Z' {
                    if cell_a == 'A' && cell_b == 'A' {
                        start = nodes.len();
                    } else if cell_a == 'Z' && cell_b == 'Z' {
                        end = nodes.len();
                    } else {
                        let mut name = String::with_capacity(2);
                        name.push(cell_a);
                        name.push(cell_b);
                        if let Some(id) = portals.iter().position(|p| p.name == name) {
                            portals[id].i2 = nodes.len();
                            portals[id].x2 = x;
                            portals[id].y2 = y;
                            if portals[id].x1 == 2 || portals[id].x1 == width - 3 ||
                                    portals[id].y1 == 2 || portals[id].y1 == height - 3 {
                                let tmp_i = portals[id].i2;
                                let tmp_x = portals[id].x2;
                                let tmp_y = portals[id].y2;
                                portals[id].i2 = portals[id].i1;
                                portals[id].x2 = portals[id].x1;
                                portals[id].y2 = portals[id].y1;
                                portals[id].i1 = tmp_i;
                                portals[id].x1 = tmp_x;
                                portals[id].y1 = tmp_y;
                            }
                            assert!(portals[id].x2 == 2 || portals[id].x2 == width - 3 ||
                                portals[id].y2 == 2 || portals[id].y2 == height - 3, "Invalid {:?}", portals[id]);
                        } else {
                            portals.push(Portal::new(&name, nodes.len(), x, y));
                        }
                    }
                }
            }
            // add new node
            nodes.push(Node::new(x, y));
        }
    }

    let mut edges_part1 = edges.clone();
    for portal in &portals {
        edges_part1.push(Edge::new(portal.i1, portal.i2));
        edges_part1.push(Edge::new(portal.i2, portal.i1));
    }

    let distance = dijkstra(nodes.len(), start, &edges_part1);
    println!("Part 1: {}", distance[end].unwrap());

    // for part 2, everything needs to be repeated portals.len() + 1 times
    let mut edges_part2 = Vec::with_capacity((edges.len() + portals.len()) * (portals.len() + 1));
    for edge in &edges {
        for i in 0..portals.len() + 1 {
            edges_part2.push(Edge::new(edge.a + i * nodes.len(), edge.b + i * nodes.len()));
        }
    }
    for portal in &portals {
        for i in 0..portals.len() {
            edges_part2.push(Edge::new(portal.i1 + i * nodes.len(), portal.i2 + (i + 1) * nodes.len()));
            edges_part2.push(Edge::new(portal.i2 + (i + 1) * nodes.len(), portal.i1 + i * nodes.len()));
        }
    }

    let distance = dijkstra(nodes.len() * (portals.len() + 1), start, &edges_part2);
    println!("Part 2: {:?}", distance[end]);
}

fn dijkstra(node_count: usize, start_id: usize, edges: &Vec<Edge>) -> Vec<Option<usize>> {
    let mut heap: BinaryHeap<NodeWithDistance> = BinaryHeap::with_capacity(node_count);
    heap.push(NodeWithDistance { id: start_id, dist: 0 });
    let mut distance: Vec<Option<usize>> = vec![None; node_count];
    distance[start_id] = Some(0);
    let mut predecessor: Vec<Option<usize>> = vec![None; node_count];
    while let Some(NodeWithDistance { id, dist }) = heap.pop() {
        if distance[id] == None || dist > distance[id].unwrap() { continue; }
        let new_dist = dist + 1;
        for v in edges.iter().filter_map(|e| if e.a == id { Some(e.b) } else { None }) {
            if distance[v] == None || distance[v].unwrap() > new_dist {
                distance[v] = Some(new_dist);
                predecessor[v] = Some(id);
                heap.push(NodeWithDistance { id: v, dist: new_dist });
            }
        }
    }
    distance
}