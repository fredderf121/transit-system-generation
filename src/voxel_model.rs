use typenum::Unsigned;
use typenum::B1;
use typenum::U0;
use typenum::{Diff, ToInt, UInt};

use std::error::Error;
use std::ops::Sub;

type Depth = u32;
/// Dimension and Coordinate System
/// The SVO's coordinate system is at the origin and covers a cube region.
/// It is divided into 8 octants, where importantly, the origin (0, 0, 0), is placed
/// in the first octant (+, +, +).
struct SparseVoxelOctree {
    children: Box<dyn ChildrenNodeTrait>,
}

struct MaskUInt<const B: Depth>(Depth);

impl<const B: Depth> TryFrom<Depth> for MaskUInt<B> {
    type Error = String;

    fn try_from(value: Depth) -> Result<Self, Self::Error> {
        if value < 1 << B {
            Ok(Self(value))
        } else {
            Err("Too large".into())
        }
    }
}

impl<const B: Depth> MaskUInt<B> {
    fn remove_top_bit(self) -> MaskUInt<{ B - 1 }> {
        // NOTE: We design the constructors such that self.0 is guaranteed to be less than 2 ^ Depth.
        assert!(self.0 < (1 << B));
        MaskUInt::<{ B - 1 }>(self.0 & !(1 << B))
    }
    fn get_top_bit(&self) -> Depth {
        // This invariant should hold given the design of the struct.
        assert!(self.0 >> B <= 1);
        self.0 >> B
    }
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
type Coord<const B: Depth> = MaskUInt<B>;

/// For Octrees of depth 1, N will be an array of elements that correspond to each point (e.g., colors)
/// For Octrees of depth > 1, N will be simply another OctreeNode.
struct OctreeNode<E>([E; 8]);

impl<E: Default + Copy> OctreeNode<E> {
    fn new() -> Self {
        Self([E::default(); 8])
    }

    // fn exists(x: Coord<D>, y: Coord<D>, z: Coord<D>) -> bool {
    //     false
    // }
}
trait OctreeDepth<const D: Depth> {
    type ChildrenNode;
    fn contains<OC>(children_node: &Self::ChildrenNode, octree_coord: OC) -> bool
    where
        OC: TryInto<OctreeCoord<D>>;
}

struct OctreeCoord<const B: Depth> {
    x: Coord<B>,
    y: Coord<B>,
    z: Coord<B>,
}
impl<const D: Depth> TryFrom<u32> for OctreeCoord<D> {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl OctreeDepth<{ <Self as ToInt<Depth>>::INT }> for U0 {
    type ChildrenNode = OctreeNode<PointData>;

    fn contains<TryIntoOctreeCoord>(
        children: &Self::ChildrenNode,
        octree_coord: TryIntoOctreeCoord,
    ) -> bool
    where
        TryIntoOctreeCoord: TryInto<OctreeCoord<{ <Self as ToInt<Depth>>::INT }>>,
    {
        let Ok(coord) = octree_coord.try_into() else {
            return false;
        };

        let idx = (coord.x.get_top_bit() << 2)
            | (coord.y.get_top_bit() << 1)
            | (coord.z.get_top_bit() << 0);
        todo!("Need to determine what counts as an empty leaf")
        // children.0[idx as usize] != 0
    }
}

impl<U, B> OctreeDepth<{ <Self as ToInt<Depth>>::INT }> for UInt<U, B>
where
    // You must be able to subtract 1 from the depth (excludes U0 basis case)
    Self: Sub<B1> + ToInt<Depth>,
    // Depth - 1 must also be a valid OctreeDepth (inductive)
    Diff<Self, B1>: OctreeDepth<{ <Self as ToInt<Depth>>::INT }>,
    OctreeNode<
        Option<Box<<Diff<Self, B1> as OctreeDepth<{ <Self as ToInt<Depth>>::INT }>>::ChildrenNode>>,
    >: Sized,
{
    type ChildrenNode = OctreeNode<
        Option<Box<<Diff<Self, B1> as OctreeDepth<{ <Self as ToInt<Depth>>::INT }>>::ChildrenNode>>,
    >;

    fn contains<OC>(children: &Self::ChildrenNode, octree_coord: OC) -> bool
    where
        OC: TryInto<OctreeCoord<{ <Self as ToInt<Depth>>::INT }>>,
    {
        todo!()
    }

    // fn contains(children: &Self::ChildrenNode, coord: OctreeCoord) -> bool {
    //     if !is_in_bounds(&coord, Self::DEPTH) {
    //         return false;
    //     }
    //     let idx = (((coord.x >> Self::DEPTH) & 1) << 2)
    //         | (((coord.y >> Self::DEPTH) & 1) << 1)
    //         | (((coord.z >> Self::DEPTH) & 1) << 0);
    //     let inner_coord = OctreeCoord {
    //         // TODO: SET the DEPTH-th bit to 0
    //         x: coord.x
    //     }
    //     return children.0[idx as usize] != 0;
    // }
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
