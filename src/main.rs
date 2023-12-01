use jisard::Neith;


// This is the Json-Wizard or Jisard for short.
mod jisard;
// The column representation
mod column;
// The table representation
mod table;
// The general data representation
mod data;

fn main() {
    let test = Neith::from_neithdb_file("test.neithdb");
    println!("NeitData: {:?}", test);
}
