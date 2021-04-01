use crate::version::Version;

pub struct IndexedRubyVersion {
    pub version: Version,
    pub url: String,
    pub sha1: String,
    pub sha256: String,
    pub sha521: String,
}

pub fn list(base_url: &reqwest::Url) -> Result<Vec<IndexedRubyVersion>, reqwest::Error> {
    let value =
        reqwest::blocking::get(format!("{}/index.txt", base_url.as_str()).as_str())?.text()?;
    let re = regex::Regex::new(r"(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)").unwrap();
    let mut versions = vec![];
    for (index, line) in value.split('\n').enumerate() {
        if line.is_empty() || index == 0 {
            continue;
        }
        let cap = re.captures(line).unwrap();
        if cap
            .get(1)
            .map_or("".to_string(), |m| m.as_str().to_string())
            .starts_with("ruby-0")
        {
            continue;
        }
        versions.push(IndexedRubyVersion {
            version: match Version::parse(
                cap.get(1)
                    .map_or("".to_string(), |m| m.as_str().to_string()),
            ) {
                Ok(version) => version,
                Err(_) => continue,
            },
            url: cap
                .get(2)
                .map_or("".to_string(), |m| m.as_str().to_string()),
            sha1: cap
                .get(3)
                .map_or("".to_string(), |m| m.as_str().to_string()),
            sha256: cap
                .get(4)
                .map_or("".to_string(), |m| m.as_str().to_string()),
            sha521: cap
                .get(5)
                .map_or("".to_string(), |m| m.as_str().to_string()),
        })
    }
    Ok(versions)
}
