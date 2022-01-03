use std::fs::DirEntry;
use std::path::PathBuf;

fn _find_event_mouses() -> Option<Vec<PathBuf>> {
    PathBuf::from("/dev/input/by-path")
        .read_dir()
        .ok()?
        .filter_map(|x: Result<DirEntry, _>| x.ok())
        .filter(|x: &DirEntry| x.file_name().to_str().unwrap_or("").contains("event-mouse"))
        .map(|x: DirEntry| x.path())
        .collect::<Vec<PathBuf>>()
        .into()
}

pub fn find_event_mouses() -> Vec<PathBuf> {
    _find_event_mouses().unwrap_or_else(Vec::new)
}

pub fn find_event_mouse() -> Option<PathBuf> {
    find_event_mouses().into_iter().next()
}
