use std::ops::Deref;

// TODO: create OctreeD0 through OctreeD31 (hardcoded types)
type OctreeD0 = OctreeNode<PointData>;
type OctreeD1 = OctreeNode<Option<Box<OctreeD0>>>;
type OctreeD2 = OctreeNode<Option<Box<OctreeD1>>>;
type OctreeD3 = OctreeNode<Option<Box<OctreeD2>>>;
type OctreeD4 = OctreeNode<Option<Box<OctreeD3>>>;
type OctreeD5 = OctreeNode<Option<Box<OctreeD4>>>;
type OctreeD6 = OctreeNode<Option<Box<OctreeD5>>>;
type OctreeD7 = OctreeNode<Option<Box<OctreeD6>>>;
type OctreeD8 = OctreeNode<Option<Box<OctreeD7>>>;
type OctreeD9 = OctreeNode<Option<Box<OctreeD8>>>;
type OctreeD10 = OctreeNode<Option<Box<OctreeD9>>>;
type OctreeD11 = OctreeNode<Option<Box<OctreeD10>>>;
type OctreeD12 = OctreeNode<Option<Box<OctreeD11>>>;
type OctreeD13 = OctreeNode<Option<Box<OctreeD12>>>;
type OctreeD14 = OctreeNode<Option<Box<OctreeD13>>>;
type OctreeD15 = OctreeNode<Option<Box<OctreeD14>>>;
type OctreeD16 = OctreeNode<Option<Box<OctreeD15>>>;
type OctreeD17 = OctreeNode<Option<Box<OctreeD16>>>;
type OctreeD18 = OctreeNode<Option<Box<OctreeD17>>>;
type OctreeD19 = OctreeNode<Option<Box<OctreeD18>>>;
type OctreeD20 = OctreeNode<Option<Box<OctreeD19>>>;
type OctreeD21 = OctreeNode<Option<Box<OctreeD20>>>;
type OctreeD22 = OctreeNode<Option<Box<OctreeD21>>>;
type OctreeD23 = OctreeNode<Option<Box<OctreeD22>>>;
type OctreeD24 = OctreeNode<Option<Box<OctreeD23>>>;
type OctreeD25 = OctreeNode<Option<Box<OctreeD24>>>;
type OctreeD26 = OctreeNode<Option<Box<OctreeD25>>>;
type OctreeD27 = OctreeNode<Option<Box<OctreeD26>>>;
type OctreeD28 = OctreeNode<Option<Box<OctreeD27>>>;
type OctreeD29 = OctreeNode<Option<Box<OctreeD28>>>;
type OctreeD30 = OctreeNode<Option<Box<OctreeD29>>>;
type OctreeD31 = OctreeNode<Option<Box<OctreeD30>>>;

enum Octree {
    OctreeD0(OctreeD0),
    OctreeD1(OctreeD1),
    OctreeD2(OctreeD2),
    OctreeD3(OctreeD3),
    OctreeD4(OctreeD4),
    OctreeD5(OctreeD5),
    OctreeD6(OctreeD6),
    OctreeD7(OctreeD7),
    OctreeD8(OctreeD8),
    OctreeD9(OctreeD9),
    OctreeD10(OctreeD10),
    OctreeD11(OctreeD11),
    OctreeD12(OctreeD12),
    OctreeD13(OctreeD13),
    OctreeD14(OctreeD14),
    OctreeD15(OctreeD15),
    OctreeD16(OctreeD16),
    OctreeD17(OctreeD17),
    OctreeD18(OctreeD18),
    OctreeD19(OctreeD19),
    OctreeD20(OctreeD20),
    OctreeD21(OctreeD21),
    OctreeD22(OctreeD22),
    OctreeD23(OctreeD23),
    OctreeD24(OctreeD24),
    OctreeD25(OctreeD25),
    OctreeD26(OctreeD26),
    OctreeD27(OctreeD27),
    OctreeD28(OctreeD28),
    OctreeD29(OctreeD29),
    OctreeD30(OctreeD30),
    OctreeD31(OctreeD31),
}
type Depth = u32;

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
    const fn remove_top_bit(self) -> MaskUInt<{ B - 1 }> {
        // NOTE: We design the constructors such that self.0 is guaranteed to be less than 2 ^ Depth.
        assert!(self.0 < (1 << B));
        MaskUInt::<{ B - 1 }>(self.0 & !(1 << B))
    }
    const fn get_top_bit(&self) -> Depth {
        // This invariant should hold given the design of the struct.
        assert!(self.0 >> B <= 1);
        self.0 >> B
    }
}

type PointData = u32;
type Coord<const B: Depth> = MaskUInt<B>;

struct OctreePoint<const B: Depth> {
    x: Coord<B>,
    y: Coord<B>,
    z: Coord<B>,
}

impl<const B: Depth> OctreePoint<B> {
    fn decrease_depth(self) -> OctreePoint<{ B - 1 }> {
        OctreePoint::<{ B - 1 }> {
            x: self.x.remove_top_bit(),
            y: self.y.remove_top_bit(),
            z: self.z.remove_top_bit(),
        }
    }
}

impl<const B: Depth> Deref for MaskUInt<B> {
    type Target = Depth;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/// For Octrees of depth 1, N will be an array of elements that correspond to each point (e.g., colors)
/// For Octrees of depth > 1, N will be simply another OctreeNode.
struct OctreeNode<E>([E; 8]);
trait Nested<const D: Depth> {
    fn contains_inner(&self, point: OctreePoint<D>) -> bool;

    fn contains(&self, point: impl TryInto<OctreePoint<D>>) -> bool {
        let Ok(coord) = point.try_into() else {
            return false;
        };
        self.contains_inner(coord)
    }
    fn get_idx(point: &OctreePoint<D>) -> Depth {
        (point.x.get_top_bit() << 2) | (point.y.get_top_bit() << 1) | (point.z.get_top_bit() << 0)
    }
}
impl Nested<0> for OctreeNode<PointData> {
    fn contains_inner(&self, point: OctreePoint<0>) -> bool {
        let idx = Self::get_idx(&point);
        todo!("Need to determine what counts as an empty leaf")
        // children.0[idx as usize] != 0
    }
}

impl<T: Nested<{ D - 1 }>, const D: Depth> Nested<D> for OctreeNode<Option<Box<T>>> {
    fn contains_inner(&self, point: OctreePoint<D>) -> bool {
        let idx = Self::get_idx(&point);
        if let Some(ref child) = self.0[idx as usize] {
            return child.contains_inner(point.decrease_depth());
        }
        false
    }
}

impl<E: Default + Copy> OctreeNode<E> {
    fn new() -> Self {
        Self([E::default(); 8])
    }
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
