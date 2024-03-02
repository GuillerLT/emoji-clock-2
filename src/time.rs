//! [time] implementations

use crate::TimeLike;

impl TimeLike for time::Time {
	fn hour(&self) -> u8 {
		time::Time::hour(*self)
	}
	fn minute(&self) -> u8 {
		time::Time::minute(*self)
	}
}

impl TimeLike for time::PrimitiveDateTime {
	fn hour(&self) -> u8 {
		time::PrimitiveDateTime::hour(*self)
	}
	fn minute(&self) -> u8 {
		time::PrimitiveDateTime::minute(*self)
	}
}

impl TimeLike for time::OffsetDateTime {
	fn hour(&self) -> u8 {
		time::OffsetDateTime::hour(*self)
	}
	fn minute(&self) -> u8 {
		time::OffsetDateTime::minute(*self)
	}
}
