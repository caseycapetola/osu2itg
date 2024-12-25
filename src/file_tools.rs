use std::path::PathBuf;

pub trait Serialize {
    fn serialize(&self) -> String;
}

pub struct OsuAudioFilename {
    pub name: PathBuf,
}

impl Serialize for OsuAudioFilename {
    fn serialize(&self) -> String {
        return format!("AudioFilename: {}", self.name.display());
    }
}

pub struct SM5AudioFilename {
    name: PathBuf
}

impl Serialize for SM5AudioFilename {
    fn serialize(&self) -> String {
        return format!("#MUSIC:{}", self.name.display());
    }
}

impl From<OsuAudioFilename> for SM5AudioFilename {
    fn from(value: OsuAudioFilename) -> Self {
        return SM5AudioFilename {
            name: format!("{};\n", value.name.display()).into()
        }
    }
}

pub struct OsuTitle {
    pub name: PathBuf,
}

impl Serialize for OsuTitle {
    fn serialize(&self) -> String {
        return format!("TitleUnicode:{}", self.name.display());
    }
}

pub struct SM5Title {
    name: PathBuf
}

impl Serialize for SM5Title {
    fn serialize(&self) -> String {
        return format!("#TITLE:{}", self.name.display());
    }
}

impl From<OsuTitle> for SM5Title {
    fn from(value: OsuTitle) -> Self {
        return SM5Title {
            name: format!("{};\n", value.name.display()).into()
        }
    }
}

pub struct OsuPreviewTime {
    pub time: u32,
}

impl Serialize for OsuPreviewTime {
    fn serialize(&self) -> String {
        return format!("PreviewTime: {}", self.time.to_string());
    }
}

pub struct SM5PreviewTime {
    time: f64
}

impl Serialize for SM5PreviewTime {
    fn serialize(&self) -> String {
        return format!("#SAMPLESTART:{};\n", self.time.to_string());
    }
}

impl From<OsuPreviewTime> for SM5PreviewTime {
    fn from(value: OsuPreviewTime) -> Self {
        return SM5PreviewTime {
            time: value.time as f64 / 1000.0
        }
    }
}

pub struct OsuArtist {
    pub name: PathBuf,
}

impl Serialize for OsuArtist {
    fn serialize(&self) -> String {
        return format!("ArtistUnicode:{}", self.name.display());
    }
}

pub struct SM5Artist {
    name: PathBuf
}

impl Serialize for SM5Artist {
    fn serialize(&self) -> String {
        return format!("#ARTIST:{};\n", self.name.display());
    }
}

impl From<OsuArtist> for SM5Artist {
    fn from(value: OsuArtist) -> Self {
        return SM5Artist {
            name: format!("{}", value.name.display()).into()
        }
    }
}

pub struct OsuVersion {
    pub version: PathBuf,
}

impl Serialize for OsuVersion {
    fn serialize(&self) -> String {
        return format!("Version:{}", self.version.display());
    }
}

pub struct SM5Version {
    version: PathBuf
}

impl Serialize for SM5Version {
    fn serialize(&self) -> String {
        return format!("#SUBTITLE:{};\n", self.version.display());
    }
}

impl From<OsuVersion> for SM5Version {
    fn from(value: OsuVersion) -> Self {
        return SM5Version {
            version: format!("{}", value.version.display()).into()
        }
    }
}

// let sm5 = file.write(SM5AudioFilename::from(OsuAudioFilename).do_thing();