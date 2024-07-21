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

impl SparseVoxelOctree {
    fn new(d: Depth) -> Self {
        let c = match d {
            0 => Box::new(<U0 as OctreeDepth>::ChildrenNode::new(0)),
            _ => todo!(),
        };
        Self { children: c }
    }
}

type PointData = u32;

/// For Octrees of depth 1, N will be an array of elements that correspond to each point (e.g., colors)
/// For Octrees of depth > 1, N will be simply another OctreeNode.
struct OctreeNode<N, const D: Depth>([N; 8]);

impl<N, const D: Depth> OctreeNode<N, D> {
    fn new(e: N) -> Self {
        Self([e; 8])
    }
}
trait OctreeDepth {
    type ChildrenNode: ChildrenNodeTrait;
}

trait ChildrenNodeTrait {
    fn get_depth(&self) -> Depth;
}

impl<N, const D: Depth> ChildrenNodeTrait for OctreeNode<N, D> {
    fn get_depth(&self) -> Depth {
        D
    }
}

impl OctreeDepth for U0 {
    type ChildrenNode = OctreeNode<PointData, { <U0 as ToInt<Depth>>::INT }>;
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
