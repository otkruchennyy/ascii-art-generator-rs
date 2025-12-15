use std::path::Path;

pub fn img_path_exists(path: &str) -> bool {
    let len = path.len();
    
    (len >= 4 && &path[len-4..] == ".png") ||
    (len >= 4 && &path[len-4..] == ".jpg") ||
    (len >= 5 && &path[len-5..] == ".jpeg")
}

pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}