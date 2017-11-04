extern crate pdf;

use std::env::args;
use std::time::SystemTime;

use pdf::file::File;
use pdf::print_err;
use pdf::object::*;

fn main() {
    let path = args().nth(1).expect("no file given");
    println!("read: {}", path);
    let now = SystemTime::now();
    let file = File::<Vec<u8>>::open(&path).unwrap_or_else(|e| print_err(e));
    
    let num_pages = file.get_root().pages.count;
    let mut pages = file.pages();
    for i in 0..num_pages {
        let p = file.get_page(i).unwrap();
        assert_eq!(p as *const Page, pages.next().unwrap() as *const Page); 
    }
    assert!(pages.next().is_none());

    let images: Vec<_> = file.pages()
        .filter_map(|page| page.resources.as_ref())
        .filter_map(|res| res.xobject.as_ref())
        .flat_map(|xo| xo)
        .filter_map(|(_, o)| match *o { XObject::Image(ref im) => Some(im), _ => None })
        .collect();
    
    if let Ok(elapsed) = now.elapsed() {
        println!("Time: {}s", elapsed.as_secs() as f64
                 + elapsed.subsec_nanos() as f64 * 1e-9);
    }
}
