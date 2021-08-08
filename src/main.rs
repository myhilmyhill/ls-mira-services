mod lscmd;
use crate::lscmd::lscmd;

fn main() {
    lscmd("http://192.168.0.200:40772/api/services", |d| {
        println!("{:#?}", d);
    }).unwrap();
}
