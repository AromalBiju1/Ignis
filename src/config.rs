use alloc::string::String;
use alloc::vec::Vec;

pub struct BootEntry {
    pub name: String,
    pub path: String,
}

pub struct BootConfig {
    pub timeout_secs: u32,
    pub default: usize,
    pub entries: Vec<BootEntry>,
}

pub fn parse_config(raw: &str) -> BootConfig {
    let mut timeout_secs: u32 = 0; // 0 = no timeout, wait forever
    let mut default: usize = 0;
    let mut entries: Vec<BootEntry> = Vec::new();

    for line in raw.lines().map(|l| l.trim()) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(rest) = line.strip_prefix("timeout=") {
            if let Ok(v) = rest.trim().parse::<u32>() {
                timeout_secs = v;
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("default=") {
            if let Ok(v) = rest.trim().parse::<usize>() {
                default = v;
            }
            continue;
        }

        // otherwise treat as a "name|path" entry
        let mut parts = line.splitn(2, '|');
        if let (Some(name), Some(path)) = (parts.next(), parts.next()) {
            let name = name.trim();
            let path = path.trim();
            if !name.is_empty() && !path.is_empty() {
                entries.push(BootEntry {
                    name: String::from(name),
                    path: String::from(path),
                });
            }
        }
    }

    // clamp default to a valid index once we know entry count
    if default >= entries.len() && !entries.is_empty() {
        default = 0;
    }

    BootConfig {
        timeout_secs,
        default,
        entries,
    }
}