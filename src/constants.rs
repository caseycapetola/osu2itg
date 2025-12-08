pub struct OsuNoteTypeV2;
impl OsuNoteTypeV2 {
    pub const TAP: i32 = 0b1;
    pub const SLIDER: i32 = 0b10;
    pub const SPINNER: i32 = 0b1000;
}

pub struct OsuModeV2;
impl OsuModeV2 {
    pub const STANDARD: i32 = 0;
    pub const _TAIKO: i32 = 1;
    pub const _CATCH: i32 = 2;
    pub const _MANIA: i32 = 3;
}

pub struct Foot {
  pub state: i8,
}
impl Foot {
    pub const LEFT: i8 = 0;
    pub const RIGHT: i8 = 1;

    pub fn new(state: i8) -> Self {
        Foot { state }
    }

    pub fn switch_foot(&mut self) {
        match self.state {
            Foot::LEFT => self.state = Foot::RIGHT,
            Foot::RIGHT => self.state = Foot::LEFT,
            _ => panic!("Invalid foot value"),
        }
    }
}

pub struct SM5NoteType;
impl SM5NoteType {
  pub const LSTEP: &'static str = "1000";
  pub const DSTEP: &'static str = "0100";
  pub const USTEP: &'static str = "0010";
  pub const RSTEP: &'static str = "0001";
  pub const LHOLD: &'static str = "2000";
  pub const DHOLD: &'static str = "0200";
  pub const UHOLD: &'static str = "0020";
  pub const RHOLD: &'static str = "0002";
  pub const _LRELEASE: &'static str = "3000";
  pub const _DRELEASE: &'static str = "0300";
  pub const _URELEASE: &'static str = "0030";
  pub const _RRELEASE: &'static str = "0003";
  pub const LSTEP_DRELEASE: &'static str = "1300";
  pub const LSTEP_URELEASE: &'static str = "1030";
  pub const LSTEP_RRELEASE: &'static str = "1003";
  pub const DSTEP_LRELEASE: &'static str = "3100";
  pub const DSTEP_URELEASE: &'static str = "0130";
  pub const DSTEP_RRELEASE: &'static str = "0103";
  pub const USTEP_LRELEASE: &'static str = "3010";
  pub const USTEP_DRELEASE: &'static str = "0310";
  pub const USTEP_RRELEASE: &'static str = "0013";
  pub const RSTEP_LRELEASE: &'static str = "3001";
  pub const RSTEP_DRELEASE: &'static str = "0301";
  pub const RSTEP_URELEASE: &'static str = "0031";
  pub const LHOLD_DRELEASE: &'static str = "2300";
  pub const LHOLD_URELEASE: &'static str = "2030";
  pub const LHOLD_RRELEASE: &'static str = "2003";
  pub const DHOLD_LRELEASE: &'static str = "3200";
  pub const DHOLD_URELEASE: &'static str = "0230";
  pub const DHOLD_RRELEASE: &'static str = "0203";
  pub const UHOLD_LRELEASE: &'static str = "3020";
  pub const UHOLD_DRELEASE: &'static str = "0320";
  pub const UHOLD_RRELEASE: &'static str = "0023";
  pub const RHOLD_LRELEASE: &'static str = "3002";
  pub const RHOLD_DRELEASE: &'static str = "0302";
  pub const RHOLD_URELEASE: &'static str = "0032";
  pub const QUADSTEP: &'static str = "1111";
}

pub struct TimingPointFields;
impl TimingPointFields {
    pub const TIME: usize = 0;
    pub const BEAT_LENGTH: usize = 1;
    pub const METER: usize = 2;
    pub const SAMPLE_SET: usize = 3;
    pub const SAMPLE_INDEX: usize = 4;
    pub const VOLUME: usize = 5;
    pub const UNINHERITED: usize = 6;
    pub const EFFECTS: usize = 7;
}

pub struct OsuFields;
impl OsuFields {
    pub const GENERAL: usize = 0;
    pub const EDITOR: usize = 1;
    pub const METADATA: usize = 2;
    pub const DIFFICULTY: usize = 3;
    pub const EVENTS: usize = 4;
    pub const TIMING_POINTS: usize = 5;
    pub const COLOURS: usize = 6;
    pub const HIT_OBJECTS: usize = 7;
}