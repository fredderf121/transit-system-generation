use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Copy, Clone, Eq)]
struct HeapState {
    cost: usize,
    position: (i32, i32),
}
impl HeapState {
    fn new(cost: usize, position: (i32, i32)) -> Self {
        Self { cost, position }
    }
}
impl PartialEq for HeapState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for HeapState {
    fn cmp(&self, other: &Self) -> Ordering{
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for HeapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    // Maps a point to the point that it came from
    let mut visited = HashMap::new();

    let mut frontier = BinaryHeap::new();

    frontier.push(HeapState::new(0, start));

assert!( visited.insert(start, start).is_none());

    while let Some(curr) = frontier.pop() {
        if curr.position == end {
            break;
        }
        let neighbors = [
            (curr.position.0 - 1, curr.position.1),
            (curr.position.0 + 1, curr.position.1),
            (curr.position.0, curr.position.1 + 1),
            (curr.position.0, curr.position.1 - 1),
        ];
        for neighbor in neighbors {
            if visited.contains_key(&neighbor) {
                continue;
            }

            frontier.push(HeapState::new(curr.cost + 1, neighbor));
            visited.insert(neighbor, curr.position);
        }
    }
    assert!(visited.contains_key(&end));

    // Backtrack via the visited map to get the path from end to start - then reverse it.
    let mut reverse_path = Vec::new();

    let mut curr = end;
    while curr != start {
        reverse_path.push(curr);
        curr = *visited.get(&curr).unwrap();
    }
    reverse_path.push(start);

    reverse_path.reverse();
    reverse_path
}

fn main() {
    // Stations are just 2d coordinates.
    let _station_coords = vec![(0, 0), (100, 0)];

    // Transit lines are an ordered list of stations, identified by their index in station_coords.
    // There may be multiple lines.
    let _transit_lines = vec![vec![(0, 1)]];

    // The path connecting two stations consists of a series of straight lines.
    // This is represented by a series of points that, when connected, connect the two stations together.
    // TODO: Instead of using coordinates, use indices/references to the station_coords array.
    let transit_line_paths = vec![vec![vec![((0, 0), (100, 100))]]];

    // The voxels consist of a list of coordinates that correspond to every adjacent point in station_paths.
    let mut voxels = Vec::new();

    for transit_line_path in transit_line_paths {
        let mut transit_line_voxels = Vec::new();

        for station_path in transit_line_path {
            let mut station_path_voxels = Vec::new();
            for path_start_end in station_path {
                let mut path_voxels = Vec::new();
                for x in (path_start_end.0 .0)..(path_start_end.1 .0) {
                    path_voxels.push((x, path_start_end.0 .1));
                }
                for y in (path_start_end.0 .1)..=(path_start_end.1 .1) {
                    path_voxels.push((path_start_end.1 .0, y));
                }
                station_path_voxels.push(path_voxels);
            }
            transit_line_voxels.push(station_path_voxels);
        }
        voxels.push(transit_line_voxels);
    }

    println!("{:?}", voxels);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_valid_manhattan_path(
        start: (i32, i32),
        end: (i32, i32),
        path: Vec<(i32, i32)>,
    ) -> Result<(), String> {
        let mut visited = HashSet::new();
        if path.len() == 0 {
            return Err("Path length was zero".to_owned());
        }
        if *path.first().unwrap() != start {
            return Err(format!(
                "Path did not start at 'start'. start: {:?}, path.first(): {:?}",
                start,
                path.first().unwrap()
            ));
        }

        if *path.last().unwrap() != end {
            return Err(format!(
                "Path did not end at 'end'. end: {:?}, path.last(): {:?}",
                end,
                path.last().unwrap()
            ));
        }
        let mut prev = start;
        visited.insert(prev);
        for point in path.into_iter().skip(1) {
            if !visited.insert(point) {
                return Err(format!("Path contained a duplicate point: {:?}", point));
            }
            match ((point.0 - prev.0).abs(), (point.1 - prev.1).abs()) {
                (1, 1) => return Err(format!("Path is not 4-connected between these points: from: {:?}, to: {:?}", prev, point)),
                (0, 0) => unreachable!("If `path` contains two of the same points, it should be caught by the set insertion above!"),
                _ => (),
            };
            prev = point;
        }
        // If the first and last elements equal the start and end, respectively,
        // and all the points in-between are 4-connected, then the path is valid!
        Ok(())
    }

    #[test]
    fn test_basic() {
        let start = (0, 0);
        let end = (10, 10);
        let path = dijkstra(start, end);
        dbg!(&path);

        test_valid_manhattan_path(start, end, path).unwrap();
    }
}
