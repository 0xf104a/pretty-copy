use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub trait Copier {
    fn new(bs: u64, on_copy_block: &'static dyn Fn(&Self)) -> Self; //bs = block size = amount of bytes to copy before updating a progress
    fn get_bytes_copied(&self) -> u64; //bytes
    fn get_bytes_total(&self) -> u64; //bytes
    fn copy(&mut self, src: &str, dst: &str) -> Result<(), &str>;
}

pub struct FSCopier {
    bs: u64,
    total_bytes: u64,
    copied_bytes: u64,
    on_copy_block: &'static dyn Fn(&FSCopier),
}

impl FSCopier {
    fn get_dst_path(_src: &str, _dst: &str) -> String {
        let mut dst_path = _dst.clone().to_owned();
        let src = _src.clone().to_owned();
        if _dst.chars().last().unwrap() == '/' {
            let src_fname = Path::new(&src)
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();
            //dst_path = &Path::new(dst_path).join(src_fname.clone()).into_os_string().into_string().unwrap(); //WAT?!! join returns PathBuf? WTF is PathBuf?!
            //Go to hell with urs PathBuf!
            dst_path.push_str(&src_fname.clone());
        }
        dst_path
    }
}

impl Copier for FSCopier {
    fn new(_bs: u64, _on_copy_block: &'static dyn Fn(&FSCopier)) -> FSCopier {
        FSCopier {
            bs: _bs,
            total_bytes: 0,
            copied_bytes: 0,
            on_copy_block: _on_copy_block,
        }
    }

    fn get_bytes_copied(&self) -> u64 {
        self.copied_bytes
    }

    fn get_bytes_total(&self) -> u64 {
        self.total_bytes
    }

    fn copy(&mut self, src: &str, dst: &str) -> Result<(), &str> {
        let src_path = Path::new(src);
        let mut src_file =
            File::open(&src_path).expect(&format!("Failed to open source file: {}", src));
        let tmp = FSCopier::get_dst_path(src, dst);
        let dst_path = Path::new(&tmp);
        let mut dst_file =
            File::create(dst_path).expect(&format!("Failed to create destination file {}", dst));

        self.total_bytes = src_file.metadata().unwrap().len();

        let mut buffer = Vec::with_capacity(self.bs as usize);
        while self.copied_bytes < self.total_bytes {
            let r_sz: usize = src_file
                .read(&mut buffer)
                .expect(&format!("Failed at reading from source file {}", src));
            dst_file
                .write_all(&buffer)
                .expect(&format!("Failed at writing to destination file {}", dst));
            self.copied_bytes += r_sz as u64;
            println!("r_sz={x}", x=r_sz);
            (self.on_copy_block)(self);
        }
        Ok(())
    }
}
