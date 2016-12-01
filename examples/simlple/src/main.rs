extern crate build;

fn main(){
  let details = build::build_as_json();
  println!("{}", details);
}
