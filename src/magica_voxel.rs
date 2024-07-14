// write an iterator of points to a voxel file.

use std::{io::Write, iter};

use crate::voxel::Vec3;

trait SerializableChunk {
    fn write_u32(e: u32, w: &mut impl Write) {
        w.write(&e.to_le_bytes());
    }
    fn get_id() -> [u8; 4];
    fn get_content_size_bytes(&self) -> u32;
    fn get_children_size_bytes(&self) -> u32;

    fn write_id(w: &mut impl Write) {
        w.write(&Self::get_id());
    }

    fn write_content_size(&self, w: &mut impl Write) {
        Self::write_u32(self.get_content_size_bytes(), w);
    }
    fn write_children_size(&self, w: &mut impl Write) {
        Self::write_u32(self.get_children_size_bytes(), w);
    }
    fn write_content(&self, w: &mut impl Write);
    fn write_children(&self, w: &mut impl Write);

    fn write(&self, w: &mut impl Write) {
        Self::write_id(w);
        self.write_content_size(w);
        self.write_children_size(w);
        self.write_content(w);
        self.write_children(w);
    }
}

struct SizeChunk {
    size_x: u32,
    size_y: u32,
    size_z: u32,
}
const_assert_eq!(std::mem::size_of::<SizeChunk>(), 12);

impl SerializableChunk for SizeChunk {
    fn get_id() -> [u8; 4] {
        [b'S', b'I', b'Z', b'E']
    }

    fn get_content_size_bytes(&self) -> u32 {
        std::mem::size_of::<Self>() as u32
    }

    fn get_children_size_bytes(&self) -> u32 {
        0
    }

    fn write_content(&self, w: &mut impl Write) {
        Self::write_u32(self.size_x, w);
        Self::write_u32(self.size_y, w);
        Self::write_u32(self.size_z, w);
    }

    fn write_children(&self, w: &mut impl Write) {
        // No children.
    }
}

struct XYZI {
    x: u32,
    y: u32,
    z: u32,
    i: u32,
}
const_assert_eq!(std::mem::size_of::<XYZI>(), 16);
struct XYZIChunk<'a, I>
where
    &'a I: IntoIterator<Item = &'a XYZI>,
{
    num_voxels: u32,
    voxels: &'a I,
}
impl<'a, I> SerializableChunk for XYZIChunk<'a, I>
where
    &'a I: IntoIterator<Item = &'a XYZI>,
{
    fn get_id() -> [u8; 4] {
        [b'X', b'Y', b'Z', b'I']
    }

    fn get_content_size_bytes(&self) -> u32 {
        self.num_voxels * std::mem::size_of::<XYZI>() as u32
    }

    fn get_children_size_bytes(&self) -> u32 {
        0
    }

    fn write_content(&self, w: &mut impl Write) {
        let mut count = 0;
        for xyzi in self.voxels {
            count += 1;
            Self::write_u32(xyzi.x, w);
            Self::write_u32(xyzi.y, w);
            Self::write_u32(xyzi.z, w);
            Self::write_u32(xyzi.i, w);
        }
        if count != self.num_voxels {
            panic!("Expected {} voxels, found {}", self.num_voxels, count);
        }
    }

    fn write_children(&self, _: &mut impl Write) {
        // No children.
    }
}
// https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox.txt
pub fn write_to_vox<'a, I, F>((x_len, y_len, z_len): (u32, u32, u32), voxels: I, path: F)
where
    I: IntoIterator<Item = &'a Vec3>,
    F: ToString,
{
    // let size_chunk = {
    //     let mut size_chunk: Vec<u8> = Vec::new();
    //     // chunk id
    //     size_chunk.extend("SIZE".bytes());
    //     // num bytes of chunk content
    //     size_chunk.extend((12 as u32).to_le_bytes());
    //     // num bytes of children chunks
    //     size_chunk.extend((0 as u32).to_le_bytes());

    //     size_chunk.extend((x_len as u32).to_le_bytes());
    //     size_chunk.extend((y_len as u32).to_le_bytes());
    //     size_chunk.extend((z_len as u32).to_le_bytes());

    //     size_chunk
    // };

    let xyzi_chunk_header = {
        let mut header: Vec<u8> = Vec::new();
        header.extend("XYZI".bytes());
        header.extend(
            (4 + 4 * voxels.into_iter().map(|arr| arr.len()).sum::<usize>() as u32).to_le_bytes(),
        );
        header.extend((0 as u32).to_le_bytes());

        // numVoxels
        header.extend((voxels.iter().map(|arr| arr.len()).sum::<usize>() as u32).to_le_bytes());
        header
    };

    let xyzi_chunk = xyzi_chunk_header
        .into_iter()
        .chain(voxels.iter().enumerate().flat_map(|(i, voxel_group)| {
            let mut xyzi_chunk: Vec<u8> = Vec::new();

            for &(x, y, z) in voxel_group.iter() {
                xyzi_chunk.extend((x as u8).to_le_bytes());
                xyzi_chunk.extend((y as u8).to_le_bytes());
                xyzi_chunk.extend((z as u8).to_le_bytes());
                xyzi_chunk.extend((((i + 1) * 50) as u8).to_le_bytes());
            }
            xyzi_chunk
        }))
        .collect::<Vec<_>>();

    let main_chunk = {
        let mut main_chunk: Vec<u8> = Vec::new();
        main_chunk.extend("MAIN".bytes());
        main_chunk.extend((0 as u32).to_le_bytes());
        main_chunk.extend(((size_chunk.len() + xyzi_chunk.len()) as u32).to_le_bytes());

        main_chunk.extend(size_chunk);
        main_chunk.extend(xyzi_chunk);
        main_chunk
    };

    let mut vox_bytes: Vec<u8> = Vec::new();

    // Header
    vox_bytes.extend("VOX ".bytes());
    vox_bytes.extend((150 as u32).to_le_bytes());

    vox_bytes.extend(main_chunk);

    let mut file = File::create(file_path).unwrap();
    // Write a slice of bytes to the file
    file.write_all(&vox_bytes).unwrap();
}
