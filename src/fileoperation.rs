
use std::fs;

use crate::types::ChecksumAlgorithm;
use crate::types::FileInfo;

impl crate::types::FileInfo {

    pub fn new( name: String, path: String ) -> crate::types::FileInfo {

        return crate::types::FileInfo {
            name : name,
            path : path,
            size: 0,
        }
    }
}