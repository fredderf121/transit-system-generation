// write an iterator of points to a voxel file.

use std::iter;

use crate::voxel::Vec3;

type ByteIter<'a> = Box<dyn Iterator<Item = &'a u8>>;
trait SerializableChunk<'a> {
    fn get_id_bytes(&self) -> ByteIter<'a>;
    fn get_content_size_bytes(&self) -> ByteIter<'a>;
    fn get_children_chunk_size_bytes(&self) -> ByteIter<'a>;
    fn get_content_bytes(&self) -> ByteIter<'a>;
    fn get_children_bytes(&self) -> ByteIter<'a>;

    fn iter(&self) -> ByteIter<'a> {
        Box::new(
            [
                self.get_id_bytes(),
                self.get_content_size_bytes(),
                self.get_children_chunk_size_bytes(),
                self.get_content_bytes(),
                self.get_children_bytes(),
            ]
            .into_iter()
            .flat_map(|i| i),
        )
    }
}

struct SizeChunk<'a> {
    a: u8,
}
impl<'a> SerializableChunk<'a> for SizeChunk<'a> {
    fn get_id_bytes(&self) -> ByteIter<'a> {
        Box::new("SIZE".as_bytes().iter())
    }

    fn get_content_size_bytes(&self) -> ByteIter<'a> {
        Box::new(iter::once(&self.a))
    }

    fn get_children_chunk_size_bytes(&self) -> ByteIter<'a> {
        todo!()
    }

    fn get_content_bytes(&self) -> ByteIter<'a> {
        todo!()
    }

    fn get_children_bytes(&self) -> ByteIter<'a> {
        todo!()
    }
}

// https://github.com/ephtracy/voxel-model/blob/master/MagicaVoxel-file-format-vox.txt
pub fn write_to_vox<'a, I, F>((x_len, y_len, z_len): (u32, u32, u32), voxels: I, path: F)
where
    I: IntoIterator<Item = &'a Vec3>,
    F: ToString,
{
    let size_chunk = {
        let mut size_chunk: Vec<u8> = Vec::new();
        // chunk id
        size_chunk.extend("SIZE".bytes());
        // num bytes of chunk content
        size_chunk.extend((12 as u32).to_le_bytes());
        // num bytes of children chunks
        size_chunk.extend((0 as u32).to_le_bytes());

        size_chunk.extend((x_len as u32).to_le_bytes());
        size_chunk.extend((y_len as u32).to_le_bytes());
        size_chunk.extend((z_len as u32).to_le_bytes());

        size_chunk
    };

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
