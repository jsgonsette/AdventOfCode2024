use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

/// Element that can be topologically sorted with the function [topo_sort]
pub trait TopoSortElement<I>  {
    type Iter: Iterator<Item = I>;

    /// Return an iterator on the predecessors of this element
    fn what_before(&self) -> Self::Iter;
}

/// Given an unsorted map of `items` of type [T], identified by values of type [I],
/// return a vector of identifiers that are topologically sorted. This means that
/// the successors of any element in this vector are ensured to appear at a higher
/// index in this vector.
pub fn topo_sort<I: Copy + Eq + Hash + Debug, T> (items: &HashMap<I, T>) -> Vec<I>
where T: TopoSortElement<I> {

    let mut visited = HashSet::<I>::new ();
    let mut dfs_queue = Vec::<(I, &T)>::new();
    let mut heap = Vec::<I>::new();

    // Process all unvisited elements of the hash map
    for (id, item_ref) in items {
        if visited.contains(id) { continue }

        // Push the next unvisited element into the DFS queue, then start processing it
        dfs_queue.push((*id, item_ref));
        while let Some((id, item_ref)) = dfs_queue.pop() {

            // Check if all the successors of the current item are visited
            let all_next_visited = item_ref.what_before().all (|nid| {
                visited.contains(&nid)
            });

            // If yes, we can add the current item onto the heap and mark it as visited
            if all_next_visited {
                heap.push(id);
                visited.insert(id);
            }
            // Otherwise, reschedule a visit of the current item after its
            // successors have been processed first
            else {
                dfs_queue.push((id, item_ref));

                for next_id in item_ref.what_before() {
                    if !visited.contains(&next_id) {
                        let next = items.get(&next_id).unwrap();
                        dfs_queue.push((next_id, next));
                    }
                };
            }
        }
    }

    heap
}
