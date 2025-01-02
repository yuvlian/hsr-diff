use serde::Deserialize;

#[derive(Deserialize)]
pub struct DiffMap {
    pub diff_map: Vec<Diff>,
}

#[derive(Deserialize)]
pub struct Diff {
    // pub patch_file_md5: String,
    pub patch_file_name: String,
    // pub patch_file_size: u64,
    // pub source_file_md5: String,
    pub source_file_name: String,
    // pub source_file_size: u64,
    // pub target_file_md5: String,
    pub target_file_name: String,
    // pub target_file_size: u64,
}
