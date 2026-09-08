use alloc::string::String;
use alloc::vec::Vec;


pub struct BootEntry{
    pub name: String,
    pub path : String,
}


pub fn parse_config(raw: &str) -> Vec<BootEntry> {
    raw.lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty() && !l.starts_with("#"))
    .filter_map(|line| {
        let mut parts = line.splitn(2, '|');
        let name = parts.next()?.trim();
        let path = parts.next()?.trim();
        if name.is_empty() || path.is_empty(){
            return None;
        }
        Some(BootEntry{
            name:String::from(name),
            path:String::from(path),
        })
    })
    .collect()
}