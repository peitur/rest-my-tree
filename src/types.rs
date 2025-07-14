


#[derive(Debug)]
pub enum ChecksumAlgorithm {
    SHA1,
    SHA256
}

#[derive(Debug)]
pub struct ChecksumData {
    algorithm: ChecksumAlgorithm, 
    data: String
}

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
}

