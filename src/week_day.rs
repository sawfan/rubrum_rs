use WeekDay::*;

pub enum WeekDay {
  Saturday,
  Thursday,
  Tuesday,
  Sunday,
  Friday,
  Wednesday,
  Monday,
}

impl WeekDay {
  pub fn roman_name(&self) -> &str {
    match self {
      Saturday  => "dies Saturni",
      Sunday    => "dies Solis",
      Monday    => "dies Lunae",
      Tuesday   => "dies Martis",
      Thursday  => "dies Jovis",
      Wednesday => "dies Mercurii",
      Friday    => "dies Veneris",
    }
  }
}
