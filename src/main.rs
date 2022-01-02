use double_barrel::DoubleBarrel;

fn main() {
    let filename = String::from("a.out");
    let mut db = DoubleBarrel::new(filename).unwrap();
    db.start();
}

