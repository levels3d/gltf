
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use byteorder::{LE, ReadBytesExt};

use Error;
use GlbError;

use std::io;

/// The contents of a .glb file.
#[derive(Clone, Debug)]
pub struct Glb<'a> {
    /// The header section of the `.glb` file.
    pub header: Header,
    /// The JSON section of the `.glb` file.
    pub json: &'a [u8],
    /// The optional BIN section of the `.glb` file.
    pub bin: Option<&'a [u8]>,
}

/// The header section of a .glb file.
#[derive(Copy, Clone, Debug)]
pub struct Header {
    /// Must be `b"glTF"`.
    pub magic: [u8; 4],
    /// Must be `2`.
    pub version: u32,
    /// Must match the length of the parent .glb file.
    pub length: u32,
}

/// Chunk header with no data read yet.
#[derive(Copy, Clone, Debug)]
struct ChunkHeader {
    /// The length of the chunk data in byte excluding the header.
    length: u32,
    /// Chunk type.
    ty: [u8; 4],
}

impl Header {
    fn from_reader<R: io::Read>(mut reader: R) -> Result<Self, GlbError> {
        use GlbError::IoError;
        let mut magic = [0; 4];
        reader.read_exact(&mut magic).map_err(IoError)?;
        // We only validate magic as we don't care for version and length of
        // contents, the caller does.  Let them decide what to do next with
        // regard to version and length.
        if &magic == b"glTF" {
            Ok(Self {
                magic,
                version: reader.read_u32::<LE>().map_err(IoError)?,
                length: reader.read_u32::<LE>().map_err(IoError)?,
            })
        } else {
            Err(GlbError::Magic(magic))
        }
    }
}

impl ChunkHeader {
    fn from_reader<R: io::Read>(mut reader: R) -> Result<Self, GlbError> {
        use GlbError::IoError;
        let length = reader.read_u32::<LE>().map_err(IoError)?;
        let mut ty = [0; 4];
        reader.read_exact(&mut ty).map_err(IoError)?;
        Ok(Self { length, ty })
    }
}

impl<'a> Glb<'a> {
    /// Splits loaded GLB into its three chunks.
    ///
    /// * Mandatory GLB header.
    /// * Mandatory JSON chunk.
    /// * Optional BIN chunk.
    pub fn from_slice(mut data: &'a [u8]) -> Result<Self, Error> {
        let header = Header::from_reader(&mut data)
            .and_then(|header| if header.length as usize <= data.len() {
                Ok(header)
            } else {
                Err(GlbError::Length {
                    length: header.length,
                    length_read: data.len(),
                })
            })
            .map_err(Error::Glb)?;
        match header.version {
            2 => Self::from_v2(data)
                .map(|(json, bin)| Glb { header, json, bin })
                .map_err(Error::Glb),
            x => Err(Error::Glb(GlbError::Version(x)))
        }
    }

    /// Does the loading job for you.  Provided buf will be cleared before new
    /// data will be written.  When error happens, if only header was read, buf
    /// will not be mutated, otherwise, buf will be empty.
    pub fn from_reader<R: io::Read>(mut reader: R,
                                    buf: &'a mut Vec<u8>) -> Result<Self, Error> {
        let header = Header::from_reader(&mut reader).map_err(Error::Glb)?;
        match header.version {
            2 => {
                buf.clear();
                buf.reserve(header.length as usize);
                // SAFETY: We are doing unsafe operation on a user-supplied
                // container!  Make sure not to expose user to uninitialized
                // data if an error happens during reading.
                //
                // It is guaranteed by reserve's implementation that the reserve
                // call will make buf's capacity _at least_ header.length.
                //
                // We do not read contents of the Vec unless it is fully
                // initialized.
                unsafe { buf.set_len(header.length as usize) };
                if let Err(e) = reader.read(buf)
                    .map_err(GlbError::IoError)
                    .and_then(|len| if len == header.length as usize {
                        Ok(())
                    } else {
                        Err(GlbError::Length {
                            length: header.length,
                            length_read: len,
                        })
                    })
                {
                    // SAFETY: It is safe to not run destructors because u8 has
                    // none.
                    unsafe { buf.set_len(0) };
                    Err(Error::Glb(e))
                } else {
                    Self::from_v2(buf)
                       .map(|(json, bin)| Glb { header, json, bin })
                       .map_err(Error::Glb)
                }
            }
            x => Err(Error::Glb(GlbError::Version(x)))
        }
    }

    /// Loads GLB for glTF 2.
    fn from_v2(mut data: &'a [u8]) -> Result<(&'a [u8], Option<&'a [u8]>), GlbError> {
        use GlbError::{ChunkLength, ChunkType};
        let (json, mut data) = ChunkHeader::from_reader(&mut data)
            .and_then(|json_h| if &json_h.ty == b"JSON" {
                Ok(json_h)
            } else {
                Err(ChunkType(json_h.ty))
            })
            .and_then(|json_h| if json_h.length as usize <= data.len() {
                Ok(json_h)
            } else {
                Err(ChunkLength {
                    ty: json_h.ty,
                    length: json_h.length,
                    length_read: data.len(),
                })
            })
            // PANIC: We have verified that json_h.length is no greater than
            // that of data.len().
            .map(|json_h| data.split_at(json_h.length as usize))?;

        let bin = if data.len() > 0 {
            ChunkHeader::from_reader(&mut data)
                .and_then(|bin_h| if &bin_h.ty == b"BIN\0" {
                    Ok(bin_h)
                } else {
                    Err(ChunkType(bin_h.ty))
                })
                .and_then(|bin_h| if bin_h.length as usize <= data.len() {
                    Ok(bin_h)
                } else {
                    Err(ChunkLength {
                        ty: bin_h.ty,
                        length: bin_h.length,
                        length_read: data.len(),
                    })
                })
                // PANIC: we have verified that bin_h.length is no greater than
                // that of data.len().
                .map(|bin_h| data.split_at(bin_h.length as usize))
                .map(|(x, _)| Some(x))?
        } else {
            None
        };
        Ok((json, bin))
    }
}