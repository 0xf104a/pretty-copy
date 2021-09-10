mod copier;

use copier::Copier;

fn print_progress(cp: &copier::FSCopier){
   println!("{x}", x=cp.get_bytes_copied());
}

fn main() {
    let mut c = copier::FSCopier::new(1024, &print_progress);
    c.copy("test","test2");
}
