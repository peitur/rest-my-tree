

use rest_my_tree::types;
use rest_my_tree::fileoperation;
use rest_my_tree::checksum;

fn main() {
    let alg = rest_my_tree::types::ChecksumAlgorithm::SHA256;

    let tst = rest_my_tree::types::FileInfo{ 
        name : String::from("test"),
        path : String::from("/tmp/test"),
        size: 0,
    };

    println!("Hello, world! {:?}", tst );
}
