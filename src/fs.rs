use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileType};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::{boot, CStr16, Result};

pub fn read_file_as_string(path: &str) -> Result<String> {
    let handle = boot::get_handle_for_protocol::<SimpleFileSystem>()?;
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(handle)?;
    let mut root = fs.open_volume()?;

    let mut buf = [0u16; 256];
    let cpath = CStr16::from_str_with_buf(path, &mut buf)
        .expect("path too long for buffer");

    let file_handle = root.open(cpath, FileMode::Read, FileAttribute::empty())?;

    let mut file = match file_handle.into_type()? {
        FileType::Regular(f) => f,
        FileType::Dir(_) => panic!("expected a file, found a directory: {path}"),
    };

    let mut info_buf = [0u8; 512];
    let size = match file.get_info::<FileInfo>(&mut info_buf) {
        Ok(info) => info.file_size() as usize,
        Err(e) => {
            if let Some(needed) = e.data() {
                let mut bigger = alloc::vec![0u8; *needed];
                let info = file
                    .get_info::<FileInfo>(&mut bigger)
                    .map_err(|e| e.to_err_without_payload())?;
                info.file_size() as usize
            } else {
                return Err(e.to_err_without_payload());
            }
        }
    };

    let mut data: Vec<u8> = alloc::vec![0u8; size];
    file.read(&mut data)?;

    Ok(String::from_utf8_lossy(&data).into_owned())
}


pub fn read_file_bytes(path: &str) -> Result<Vec<u8>> {
    let handle = boot::get_handle_for_protocol::<SimpleFileSystem>()?;
    let mut fs = boot::open_protocol_exclusive::<SimpleFileSystem>(handle)?;
    let mut root = fs.open_volume()?;

    let mut buf = [0u16; 256];
    let cpath = CStr16::from_str_with_buf(path, &mut buf)
        .expect("path too long for buffer");

    let file_handle = root.open(cpath, FileMode::Read, FileAttribute::empty())?;

    let mut file = match file_handle.into_type()? {
        FileType::Regular(f) => f,
        FileType::Dir(_) => panic!("expected a file, found a directory: {path}"),
    };

    let mut info_buf = [0u8; 512];
    let size = match file.get_info::<FileInfo>(&mut info_buf) {
        Ok(info) => info.file_size() as usize,
        Err(e) => {
            if let Some(needed) = e.data() {
                let mut bigger = alloc::vec![0u8; *needed];
                let info = file
                    .get_info::<FileInfo>(&mut bigger)
                    .map_err(|e| e.to_err_without_payload())?;
                info.file_size() as usize
            } else {
                return Err(e.to_err_without_payload());
            }
        }
    };

    let mut data: Vec<u8> = alloc::vec![0u8; size];
    file.read(&mut data)?;
    Ok(data)
}