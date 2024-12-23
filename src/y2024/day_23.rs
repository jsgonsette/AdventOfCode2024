use std::collections::{HashMap, HashSet};
use anyhow::*;
use itertools::Itertools;
use crate::{Solution};

const TEST: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";


fn split (content: &str) -> Vec<&str> {
    content.lines().collect()
}

/// Name of a computer in the network
type ComputerName = [char; 2];

/// Connection between a pair of computers
type Connection = (ComputerName, ComputerName);

/// A small clique with 3 computers
type Clique3 = (ComputerName, ComputerName, ComputerName);

/// A clique of variable size
type Clique = HashSet<ComputerName>;


/// Load the list of connections from the puzzle file content
fn load_connections (content: &[&str]) -> Result<Vec<Connection>> {
    content.iter ().map (|&row| {
        let raw = row.as_bytes();
        if raw.len() != 5 { bail!("Invalid connection description") };
        let name_left = [raw [0] as char, raw [1] as char];
        let name_right = [raw [3] as char, raw [4] as char];
        Ok ((name_left, name_right))
    }).collect  ()
}

/// A graph on the form of an adjacency list for each node
type Graph = HashMap<ComputerName, HashSet<ComputerName>>;

/// Build the graph from the list of network connections
fn make_graph (connections: Vec<Connection>) -> Graph {
    let mut graph = Graph::new();

    for connection in connections {
        graph.entry(connection.0).or_insert_with(HashSet::new).insert(connection.1);
        graph.entry(connection.1).or_insert_with(HashSet::new).insert(connection.0);
    }

    graph
}

/// Check if a pair of computers are connected
fn are_connected (graph: &Graph, node_1: &ComputerName, node_2: &ComputerName) -> bool {
    graph.get(node_1).unwrap().contains(node_2)
}

/// Sort a clique of 3 elements `(a, b, c)` by computer name
fn sort ((a, b, c): Clique3) -> Clique3 {
    let mut arr = [a, b, c];
    arr.sort();
    (arr[0], arr[1], arr[2])
}

/// Check if at least one computer in the clique `(a, b, c)` starts with `t`
fn start_with_t ((a, b, c): &Clique3) -> bool {
    a [0] == 't' || b[0] == 't' || c[0] == 't'
}

/// Get the list of 3-element cliques in the graph involving the provided `node`,
/// removing those where the letter `t` does not appear (part 1).
/// The function returns a vector of such 3-clique, where each element is sorted by name
fn get_all_3_cliques(graph: &Graph, node: &ComputerName) -> Vec<Clique3> {

    // - Iterate on all the possible pair of neighbors (a, b) attached to `node`
    // - skip pairs where a and b are not connected
    // - make a tuple (node, a, b) sorted by name
    // - filter those that do not contain the letter `t`
    let neighbors = graph.get(node).unwrap();
    let cliques: Vec<_> = neighbors
        .iter()
        .tuple_combinations::<(&ComputerName, &ComputerName)>()
        .filter(|&(first, second)| are_connected (graph, first, second))
        .map(|(first, second)| sort ((*node, *first, *second)))
        .filter(start_with_t)
        .collect()
    ;

    cliques
}

fn expand_clique (graph: &Graph, processed: &HashSet::<ComputerName>, mut clique: Clique, from: &ComputerName) -> Clique {

    for n in graph.get (from).unwrap().iter() {
        if processed.contains(n) { continue }

        let neighbors = graph.get(n).unwrap();
        let can_expand = clique.iter ().all(| clique_node | neighbors.contains(clique_node) );
        if can_expand {
            clique.insert(*n);
        }
    }
    clique
}

fn find_max_clique (graph: &Graph) -> Clique {

    let mut max_clique = Clique::new();                 // Track the biggest clique
    let mut current_clique = Clique::new();             //
    let mut processed = HashSet::<ComputerName>::new(); // All the nodes processed so far

    // Iterate on each node ...
    for (node, neighbors) in graph.iter() {

        // mark it as processed and skip it immediately if its neighborhood is not big enough
        processed.insert(*node);
        if 1 + neighbors.len() < max_clique.len() { continue }

        // Find pairs of computers (edge) around which we try to build a bigger clique
        for n in neighbors.iter() {
            if processed.contains(n) { continue }

            current_clique.insert(*n);
            current_clique.insert(*node);
            current_clique = expand_clique(graph, &processed, current_clique, node);

            // Save this clique if it contains more elements
            if current_clique.len() > max_clique.len() { max_clique = current_clique.clone(); }
            current_clique.clear();
        }
    }

    max_clique
}

/// Solve first part of the puzzle
fn part_a (content: &[&str]) -> Result<usize> {

    // Load the connections from the puzzle file content and make a graph out of it
    let connections = load_connections (content)?;
    let graph = make_graph(connections);

    // The set of identified 3-element cliques
    let mut all_cliques = HashSet::<Clique3>::new();

    // For each node in the graph
    for (node, _) in graph.iter() {

        // find the 3-element cliques where it is involved
        let cliques = get_all_3_cliques(&graph, node);
        all_cliques.extend(cliques);
    }

    Ok(all_cliques.len())
}

/// Solve second part of the puzzle
fn part_b (content: &[&str]) -> Result<String> {

    // Load the connections from the puzzle file content and make a graph out of it
    let connections = load_connections (content)?;
    let graph = make_graph(connections);

    // Find the clique containing the highest number of computers
    let max_clique = find_max_clique(&graph);

    // Sort the computers in the clique by name
    let mut max_clique: Vec<ComputerName> = max_clique.into_iter().collect();
    max_clique.sort();

    // Build the password from those names
    let password = max_clique.into_iter().map (|name| format!("{}{}", name [0], name[1])).join(",");
    Ok(password)
}

pub fn day_23 (content: &[&str]) -> Result <(Solution, Solution)> {

    debug_assert!(part_a (&split(TEST)).unwrap_or_default() == 7);
    debug_assert!(part_b (&split(TEST)).unwrap_or_default() == "co,de,ka,ta");

    let ra = part_a(content)?;
    let rb = part_b(content)?;

    Ok((Solution::Unsigned(ra), Solution::Text(rb)))
}