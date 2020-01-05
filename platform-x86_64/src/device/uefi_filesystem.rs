use alloc::{
    alloc::Layout,
    vec::Vec,
    rc::Rc,
    string::String
};

use core::{
    borrow::Borrow,
    cell::RefCell
};

use uefi::{
    data_types::Align,
    prelude::*,
    proto::media::{
        file::{File, FileInfo, FileMode, FileAttribute, FileType},
        fs::SimpleFileSystem,
    }
};

use crate::{
    X8664Platform
};

#[derive(Clone)]
pub struct UEFIFilesystem {
    system_table: Rc<RefCell<SystemTable<Boot>>>
}

// trust me I'm an expert
unsafe impl core::marker::Send for UEFIFilesystem {}

impl UEFIFilesystem {
    pub fn new(system_table: &Rc<RefCell<SystemTable<Boot>>>) -> Self {
        Self { system_table: system_table.clone() }
    }

    fn filesystem_protocol(&self) -> Result<&mut SimpleFileSystem, <X8664Platform as kernel::Platform>::Error> {
        let system_table: &RefCell<SystemTable<Boot>> = self.system_table.borrow();
        let system_table = system_table.borrow();
        let boot_services = system_table.boot_services();
        let (status, fs) = boot_services.locate_protocol::<SimpleFileSystem>()??;
        if status.is_warning() { 
            log::warn!("UEFI warning locating protocol: {:?}", status); 
        }
        let fs = unsafe { &mut *fs.get() };

        Ok(fs)
    }
}

impl kernel::Device<X8664Platform> for UEFIFilesystem {
    fn poll(&mut self) {}
    fn as_filesystem(&mut self) -> Option<&mut dyn kernel::Filesystem<X8664Platform>> { Some(self) }
}

impl kernel::Filesystem<X8664Platform> for UEFIFilesystem {
    fn list(&mut self) -> Result<Vec<String>, <X8664Platform as kernel::Platform>::Error> {
        let fs = self.filesystem_protocol()?;
        
        let (_status, mut root) = fs.open_volume()??;

        let layout = Layout::from_size_align(104, <FileInfo as Align>::alignment())
            .unwrap()
            .pad_to_align();
        let mut buffer = uefi::exts::allocate_buffer(layout);

        let mut files = Vec::new();

        while let (_, Some(info)) = root.read_entry(&mut buffer)?? {
            let file_name = String::from_utf16(info.file_name().to_u16_slice()).unwrap();
            files.push(file_name);
        }

        Ok(files)
    }

    fn read(&mut self, path: &str) -> Result<Vec<u8>, <X8664Platform as kernel::Platform>::Error> {
        let fs = self.filesystem_protocol()?;
        
        let (_status, mut root) = fs.open_volume()??;
        let (_status, mut file) = root.open(path, FileMode::Read, FileAttribute::empty())??;

        let (_status, file_type) = file.into_type()??;
        match file_type {
            FileType::Dir(_) => unimplemented!(),
            FileType::Regular(mut file) => {
                let (_status, info) = file.get_boxed_info::<FileInfo>()??;
                let file_size = info.file_size() as usize;
                let mut buffer = Vec::new();
                buffer.resize(file_size, 0u8);

                let (_status, _) = file.read(&mut buffer)??;

                Ok(buffer)
            }
        }
    }
}
