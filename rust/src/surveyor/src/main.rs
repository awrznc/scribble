fn main() {
    let surveyor_object = surveyor::Writer::new("./target/", "sample");
    surveyor_object.write_all().expect("error")
}
