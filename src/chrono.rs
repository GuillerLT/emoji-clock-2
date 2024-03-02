//! [chrono] implementations

use crate::TimeLike;

impl TimeLike for chrono::NaiveTime {
	fn hour(&self) -> u8 {
		chrono::Timelike::hour(self) as u8
	}
	fn minute(&self) -> u8 {
		chrono::Timelike::minute(self) as u8
	}
}

impl TimeLike for chrono::NaiveDateTime {
	fn hour(&self) -> u8 {
		chrono::Timelike::hour(self) as u8
	}
	fn minute(&self) -> u8 {
		chrono::Timelike::minute(self) as u8
	}
}

impl<Tz: chrono::TimeZone> TimeLike for chrono::DateTime<Tz> {
	fn hour(&self) -> u8 {
		chrono::Timelike::hour(self) as u8
	}
	fn minute(&self) -> u8 {
		chrono::Timelike::minute(self) as u8
	}
}
