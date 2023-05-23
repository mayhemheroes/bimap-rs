use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: (&[u8], &[u8])| {
            // pull data out of tuple
            let (data1, data2) = data;

            // convert data to strings
            let key = std::str::from_utf8(data1).unwrap();
            let value = std::str::from_utf8(data2).unwrap();

            // create bimap
            let mut bimap = bimap::BiMap::<&str, &str>::new();

            // insert data into bimap
            bimap.insert(&key, &value);

            // get data from bimap
            let _ = bimap.get_by_right(&value);
        });
    }
}