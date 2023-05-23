use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut bimap = bimap::BiMap::<Vec<u8>, Vec<u8>>::new();
            bimap.insert(data.to_vec(), data.to_vec());
        });
    }
}