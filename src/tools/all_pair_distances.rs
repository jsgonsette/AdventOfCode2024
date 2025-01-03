pub type GraphWeight = u32;

/// Given a graph of `num_nodes`, implicitly given by the function `fn_adjacency`, returns
/// a matrix giving all pair distances between the nodes.
///
/// Given some node index, function *fn_adjacency* returns the neighbor node indexes and the edge weights.
///
/// This function uses the [Floyd–Warshall algorithm](https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm)
/// algorithm
pub fn compute_all_pair_distances<F, I> (num_nodes: usize, fn_adjacency: F) -> Vec<Vec<u32>>
where
    F: Fn(usize) -> I,
    I: Iterator<Item = (usize, GraphWeight)>, {

    let mut distances = vec![vec![GraphWeight::MAX; num_nodes]; num_nodes];

    // First iteration where all 1-step neighbor distances are established
    for ni in 0..num_nodes {
        distances [ni][ni] = 0;

        for (adj_node, weight) in fn_adjacency(ni) {
            distances [ni][adj_node] = weight;
        }
    }

    for nk in 0..num_nodes {
        for ni in 0..num_nodes {
            for nj in 0..num_nodes {
                distances[ni][nj] =
                    distances[ni][nj].min (distances[ni][nk].saturating_add(distances[nk][nj]))
            }
        }
    }

    distances
}
