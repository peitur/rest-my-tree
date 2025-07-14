

use rest_my_tree::types;
use rest_my_tree::fileoperation;
use rest_my_tree::checksum;

fn main() {
    let alg = rest_my_tree::types::ChecksumAlgorithm::SHA256;

    let tst = rest_my_tree::types::FileInfo::new( String::from("test"), String::from("/tmp/test") );

    println!("Hello, world! {:?}", tst );
}
