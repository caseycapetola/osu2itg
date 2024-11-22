use std::path::PathBuf;

// enum FileFields {
//     OsuAudioFilename,
//     SM5AudioFilename,
//     OsuTitle,
//     SM5Title,

// }
pub trait Deserialize {
    fn deserialize(&self) -> String;
}

pub struct OsuAudioFilename {
    pub name: PathBuf,
}

impl Deserialize for OsuAudioFilename {
    fn deserialize(&self) -> String {
        return format!("AudioFilename: {}", self.name.display());
    }
}

pub struct SM5AudioFilename {
    name: PathBuf
}

impl Deserialize for SM5AudioFilename {
    fn deserialize(&self) -> String {
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

impl Deserialize for OsuTitle {
    fn deserialize(&self) -> String {
        return format!("Title:{}", self.name.display());
    }
}

pub struct SM5Title {
    name: PathBuf
}

impl Deserialize for SM5Title {
    fn deserialize(&self) -> String {
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

// let sm5 = file.write(SM5AudioFilename::from(OsuAudioFilename).do_thing();