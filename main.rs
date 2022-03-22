use std::io::BufRead;
use std::io::BufReader;
use crate::fs::File;
use std::fs;
use std::env;

//fn main() -> std::io::Result<()> {

//    for entry in fs::read_dir("/Users/mihai/Desktop/octave lab")?{
//        let entry = entry?;
//        println!("{:?}", entry.file_name());
//   }

//    Ok(())

//}
fn print_file(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn size(filepath: &str) -> Result<()> {
    let x = fs::metadata(filepath).len();
    println!("{}", x);
}

fn main(){
    print_file("/Users/mihai/Desktop/fcsb.txt");
    size("/Users/mihai/Desktop/fcsb.txt");
}