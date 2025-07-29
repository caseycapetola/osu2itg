pub enum OsuNoteType {
    Tap,
    Hold,
    Spinner,
}

impl OsuNoteType {
    pub fn val(&self) -> i32 {
        match self {
            OsuNoteType::Tap => 0b1,
            OsuNoteType::Hold => 0b10,
            OsuNoteType::Spinner => 0b1000,
        }
    }
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
  pub const LRELEASE: &'static str = "3000";
  pub const DRELEASE: &'static str = "0300";
  pub const URELEASE: &'static str = "0030";
  pub const RRELEASE: &'static str = "0003";
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
}