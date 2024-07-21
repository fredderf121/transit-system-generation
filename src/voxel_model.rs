use typenum::Unsigned;
use typenum::B1;
use typenum::U0;
use typenum::{Diff, ToInt, UInt};

use std::ops::Sub;

type Depth = u32;
/// Dimension and Coordinate System
/// The SVO's coordinate system is at the origin and covers a cube region.
/// It is divided into 8 octants, where importantly, the origin (0, 0, 0), is placed
/// in the first octant (+, +, +).
struct SparseVoxelOctree {
    children: Box<dyn ChildrenNodeTrait>,
}

macro_rules! children_node {
    ($e:expr, $m:tt) => {
        match $e {
            0 => <U0 as OctreeDepth>::ChildrenNode::$m(),
            _ => todo!(),
        }
    };
}

impl SparseVoxelOctree {
    fn new(d: Depth) -> Self {
        let c = children_node!(d, new);
        Self {
            children: Box::new(c),
        }
    }
}

type PointData = u32;
type Coord = u32;

/// For Octrees of depth 1, N will be an array of elements that correspond to each point (e.g., colors)
/// For Octrees of depth > 1, N will be simply another OctreeNode.
struct OctreeNode<E, const D: Depth>([E; 8]);

impl<E: Default + Copy, const D: Depth> OctreeNode<E, D> {
    fn new() -> Self {
        Self([E::default(); 8])
    }

    fn exists(x: Coord, y: Coord, z: Coord) -> bool {
        false
    }
}
trait OctreeDepth {
    type ChildrenNode: ChildrenNodeTrait;
    const DEPTH: Depth;
    fn contains(c: &Self::ChildrenNode, c: OctreeCoord) -> bool;
}

struct OctreeCoord {
    x: Coord,
    y: Coord,
    z: Coord,
}
trait ChildrenNodeTrait {
    fn get_depth(&self) -> Depth;
}

impl<N, const D: Depth> ChildrenNodeTrait for OctreeNode<N, D> {
    fn get_depth(&self) -> Depth {
        D
    }
}

fn is_in_bounds(c: &OctreeCoord, d: Depth) -> bool {
    let max = 1 << d;
    c.x < max && c.y < max && c.z < max
}

impl OctreeDepth for U0 {
    const DEPTH: Depth = <U0 as ToInt<Depth>>::INT;
    type ChildrenNode = OctreeNode<PointData, { Self::DEPTH }>;
    fn contains(children: &Self::ChildrenNode, coord: OctreeCoord) -> bool {
        if !is_in_bounds(&coord, Self::DEPTH) {
            return false;
        }
        let idx = (((coord.x >> Self::DEPTH) & 1) << 2)
            | (((coord.y >> Self::DEPTH) & 1) << 1)
            | (((coord.z >> Self::DEPTH) & 1) << 0);
        return children.0[idx as usize] != 0;
    }
}

impl<U, B> OctreeDepth for UInt<U, B>
where
    // You must be able to subtract 1 from the depth (excludes U0 basis case)
    UInt<U, B>: Sub<B1> + ToInt<Depth>,
    // Depth - 1 must also be a valid OctreeDepth (inductive)
    Diff<UInt<U, B>, B1>: OctreeDepth,
    OctreeNode<
        Option<Box<<Diff<UInt<U, B>, B1> as OctreeDepth>::ChildrenNode>>,
        { <UInt<U, B> as ToInt<Depth>>::INT },
    >: Sized,
{
    type ChildrenNode = OctreeNode<
        Option<Box<<Diff<UInt<U, B>, B1> as OctreeDepth>::ChildrenNode>>,
        { <UInt<U, B> as ToInt<Depth>>::INT },
    >;

    fn contains(children: &Self::ChildrenNode, coord: OctreeCoord) -> bool {
        if !is_in_bounds(&coord, Self::DEPTH) {
            return false;
        }
        let idx = (((coord.x >> Self::DEPTH) & 1) << 2)
            | (((coord.y >> Self::DEPTH) & 1) << 1)
            | (((coord.z >> Self::DEPTH) & 1) << 0);
        let inner_coord = OctreeCoord {
            // TODO: SET the DEPTH-th bit to 0
            x: coord.x
        }
        return children.0[idx as usize] != 0;
    }
}

impl SparseVoxelOctree {
    /// Abs designed for SVO boundary tests, where the SVO
    /// is biased such that it can contain between [-k, k)
    /// where k is the length of an octant. This shifts
    /// [-k, -1] rightwards before negating so that the resulting value
    /// is between [0, k).
    const fn abs_svo(n: i32) -> u32 {
        (if n < 0 { -(n + 1) } else { n }) as u32
    }
    const fn is_in_bounds(&self, v: &Voxel) -> bool {
        // Get the max of the coordinates via bitwise-or, then check if it is less than the length of an octant.
        // Note the use of abs_svo due to the negative skewed coordinate system.
        (Self::abs_svo(v.x) | Self::abs_svo(v.y) | Self::abs_svo(v.z)) < self.octant_length
    }
}

struct Voxel {
    x: i32,
    y: i32,
    z: i32,
}
