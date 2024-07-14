mod magica_voxel;
mod voxel;

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::Write,
};
#[macro_use]
extern crate static_assertions;

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
    fn cmp(&self, other: &Self) -> Ordering {
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

fn dijkstra(
    start: (usize, usize),
    end: (usize, usize),
    height_map: &Vec<Vec<i32>>,
) -> Vec<(usize, usize)> {
    let start = (start.0 as i32, start.1 as i32);
    let end = (end.0 as i32, end.1 as i32);

    assert!(!height_map.is_empty());
    assert!(!height_map[0].is_empty());

    let x_len = height_map.len() as i32;
    let y_len = height_map[0].len() as i32;
    // TODO: Use an actual 2d array instead of vec of vecs.
    // TODO: Handle the terrible casting issue between usize and i32
    //       - this is only a problem due to finding the neighbors in a non-infinite height map.

    // Maps a point to the point that it came from
    let mut visited = HashMap::new();

    let mut frontier = BinaryHeap::new();

    frontier.push(HeapState::new(0, start));

    assert!(visited.insert(start, start).is_none());

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

        for neighbor in neighbors.into_iter().filter_map(|(x, y)| {
            if x >= 0 && y >= 0 && x < x_len && y < y_len {
                Some((x, y))
            } else {
                None
            }
        }) {
            if visited.contains_key(&neighbor) {
                continue;
            }
            let neighbor_cost = curr.cost
                + 1
                + (height_map[neighbor.0 as usize][neighbor.1 as usize]
                    - height_map[curr.position.0 as usize][curr.position.1 as usize])
                    .checked_abs()
                    .unwrap() as usize;

            frontier.push(HeapState::new(neighbor_cost, neighbor));
            visited.insert(neighbor, curr.position);
        }
    }
    assert!(visited.contains_key(&end));

    // Backtrack via the visited map to get the path from end to start - then reverse it.
    let mut reverse_path = Vec::new();

    let mut curr = end;
    while curr != start {
        reverse_path.push((curr.0 as usize, curr.1 as usize));
        curr = *visited.get(&curr).unwrap();
    }
    reverse_path.push((start.0 as usize, start.1 as usize));

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

    // println!("{:?}", voxels);
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_valid_manhattan_path(
        start: (usize, usize),
        end: (usize, usize),
        path: &[(usize, usize)],
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
            if !visited.insert(*point) {
                return Err(format!("Path contained a duplicate point: {:?}", point));
            }
            match ((point.0 as i32 - prev.0 as i32).abs(), (point.1 as i32 - prev.1 as i32).abs()) {
                (1, 1) => return Err(format!("Path is not 4-connected between these points: from: {:?}, to: {:?}", prev, point)),
                (0, 0) => unreachable!("If `path` contains two of the same points, it should be caught by the set insertion above!"),
                _ => (),
            };
            prev = *point;
        }
        // If the first and last elements equal the start and end, respectively,
        // and all the points in-between are 4-connected, then the path is valid!
        Ok(())
    }

    #[test]
    fn test_basic() {
        let start = (0, 0);
        let end = (9, 9);
        let height_map = vec![vec![0; 10]; 10];
        let path = dijkstra(start, end, &height_map);
        // dbg!(&path);

        test_valid_manhattan_path(start, end, &path).unwrap();
    }

    #[test]
    fn from_height_map_256_256() {
        let mut height_map_path = PathBuf::from(format!(
            "{}/{}",
            env!("CARGO_MANIFEST_DIR"),
            "resources/test"
        ));
        height_map_path.push("height_map_256_256.txt");
        let height_map_file = File::open(height_map_path).expect("no such file");

        let buf = BufReader::new(height_map_file);
        // The file is assumed to be single-space separated integers with no trailing spaces.
        // Each row is separated by a new line, and there is no newline at the end.
        let height_map: Vec<Vec<i32>> = buf
            .lines()
            .map(|line| {
                line.unwrap()
                    .split(' ')
                    // max(254) so that we can hard-code +1 to the path we generate
                    .map(|s| s.parse::<i32>().unwrap().min(254))
                    .collect()
            })
            .collect();

        assert!(height_map.len() == 256 && height_map[0].len() == 256);

        let start = (0, 0);
        let end = (255, 255);
        let path = dijkstra(start, end, &height_map);

        // dbg!(&path);

        test_valid_manhattan_path(start, end, &path).unwrap();

        let path_3d = path
            .into_iter()
            .map(|(x, y)| (x, y, height_map[x][y] as usize + 1))
            .collect::<Vec<_>>();
        let height_map_3d = height_map
            .into_iter()
            .enumerate()
            .flat_map(|(x, col_points)| {
                col_points
                    .into_iter()
                    .enumerate()
                    .map(|(y, z)| (x, y, z as usize))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        magica_voxel::write_to_vox(
            (256, 256, 256),
            &[&path_3d[..], &height_map_3d[..]],
            // &[&path_3d[..]],
            "output.vox".into(),
        );
    }
}
