use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

trait Copier{
   fn new(&self, bs: u64, on_copy_block: &'static dyn Fn(&Self)) -> Self; //bs = block size = amount of bytes to copy before updating a progress
   fn get_bytes_copied(&self) -> u64; //bytes
   fn get_bytes_total(&self) -> u64; //bytes
   fn copy(&self, src: &str, dst: &str) -> bool;
}


pub struct FSCopier{
   bs: u64,
   total_bytes: u64,
   copied_bytes: u64,
   on_copy_block: &'static dyn Fn(&FSCopier)
}

impl FSCopier{
   fn get_dst_path(_src: &str,_dst: &str) -> String {
       let mut dst_path = _dst.clone().to_owned(); 
       let src = _src.clone().to_owned();
       if _dst.chars().last().unwrap() == '/' {
         let src_fname = Path::new(&src).file_name().unwrap().to_os_string().into_string().unwrap();
         //dst_path = &Path::new(dst_path).join(src_fname.clone()).into_os_string().into_string().unwrap(); //WAT?!! join returns PathBuf? WTF is PathBuf?!
         //Go to hell with urs PathBuf!
         dst_path.push_str(&src_fname.clone());
      }
      dst_path
   }
}

impl Copier for FSCopier{
   fn new(&self, _bs: u64, _on_copy_block: &'static dyn Fn(&FSCopier)) -> FSCopier{
      FSCopier{
        bs: _bs,
        total_bytes: 0,
        copied_bytes: 0,
        on_copy_block: _on_copy_block
      }
   }
   
   fn get_bytes_copied(&self) -> u64{
        self.copied_bytes
   }
   
   fn get_bytes_total(&self) -> u64{
        self.total_bytes
   }

   fn copy(&self, src: &str, dst: &str) -> bool{
        let src_path = Path::new(src);
        let src_file = match File::open(&src_path) {
            Err(reason) => panic!("Failed to open source file: {}", reason),
            Ok(f) => f,
        };
        
        let dst_path = Path::new(&FSCopier::get_dst_path(src, dst));
        let mut dst_file = File::create(dst_path);
        
        self.total_bytes = src_file.metadata().unwrap().len();
        
        let mut buffer = Vec::with_capacity(self.bs as usize);
        while self.copied_bytes<self.total_bytes{
            let r_sz = src_file.read(&mut buffer)?;
            dst_file.write(&buffer);
            self.copied_bytes+=r_sz as u64;
            (self.on_copy_block)(self);
        }
        true
   }
}
