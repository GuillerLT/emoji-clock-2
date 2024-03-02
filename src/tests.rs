use crate::*;

use core::fmt::Write;

#[derive(Default, Debug)]
struct Buf {
	data: [u8; 8],
	len: usize,
}

impl core::fmt::Write for Buf {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		let s = s.as_bytes();
		let Some(data) = self.data.get_mut(self.len..(self.len + s.len())) else {
			return Err(core::fmt::Error);
		};
		data.copy_from_slice(s);
		self.len += s.len();
		Ok(())
	}
}

impl core::cmp::PartialEq<&str> for Buf {
	fn eq(&self, other: &&str) -> bool {
		self.data[..self.len].eq(other.as_bytes())
	}
}

struct MockTime {
	hour: u8,
	minute: u8,
}

impl TimeLike for MockTime {
	fn hour(&self) -> u8 {
		self.hour
	}
	fn minute(&self) -> u8 {
		self.minute
	}
}

#[test]
fn emoji() {
	let expected = [
		'🕛', '🕧', '🕐', '🕜', '🕑', '🕝', '🕒', '🕞', '🕓', '🕟', '🕔', '🕠', '🕕', '🕡', '🕖',
		'🕢', '🕗', '🕣', '🕘', '🕤', '🕙', '🕥', '🕚', '🕦',
	];
	for (hour, expected_clocks) in (0..48).zip(expected.chunks(2).cycle()) {
		for half_hour in [false, true] {
			let expected_clock = expected_clocks[usize::from(half_hour)];
			let minute = if half_hour { 30 } else { 0 };
			assert_eq!(
				clock_emoji(hour, half_hour),
				expected_clock,
				"Expected {hour:02}:{minute:02}"
			);
		}
	}
}

#[test]
fn clock() {
	let expected = [
		"🕛", "🕧", "🕐", "🕜", "🕑", "🕝", "🕒", "🕞", "🕓", "🕟", "🕔", "🕠", "🕕", "🕡", "🕖",
		"🕢", "🕗", "🕣", "🕘", "🕤", "🕙", "🕥", "🕚", "🕦",
	];
	for (hour, expected_clocks) in (0..48).zip(expected.chunks(2).cycle()) {
		for half_hour in [false, true] {
			let expected_clock = expected_clocks[usize::from(half_hour)];
			let hour = hour % 24;
			let minute = if half_hour { 30 } else { 0 };
			let clock = Clock::Dial(MockTime { hour, minute }, TimeRounding::Round);
			let mut buf = Buf::default();
			write!(&mut buf, "{clock}").unwrap();
			assert_eq!(buf, expected_clock, "Expected {hour:02}:{minute:02}");
		}
	}
}

#[test]
fn clock_meridiem() {
	let expected = [
		"🕛🌝", "🕧🌝", // AM
		"🕐🌞", "🕜🌞", "🕑🌞", "🕝🌞", "🕒🌞", "🕞🌞", "🕓🌞", "🕟🌞", "🕔🌞", "🕠🌞", "🕕🌞",
		"🕡🌞", "🕖🌞", "🕢🌞", "🕗🌞", "🕣🌞", "🕘🌞", "🕤🌞", "🕙🌞", "🕥🌞", "🕚🌞", "🕦🌞",
		"🕛🌞", "🕧🌞", // PM
		"🕐🌝", "🕜🌝", "🕑🌝", "🕝🌝", "🕒🌝", "🕞🌝", "🕓🌝", "🕟🌝", "🕔🌝", "🕠🌝", "🕕🌝",
		"🕡🌝", "🕖🌝", "🕢🌝", "🕗🌝", "🕣🌝", "🕘🌝", "🕤🌝", "🕙🌝", "🕥🌝", "🕚🌝", "🕦🌝",
	];
	for (hour, expected_clocks) in (0..48).zip(expected.chunks(2).cycle()) {
		for half_hour in [false, true] {
			let expected_clock = expected_clocks[usize::from(half_hour)];
			let hour = hour % 24;
			let minute = if half_hour { 30 } else { 0 };
			let clock = Clock::DialMeridiem(MockTime { hour, minute }, TimeRounding::Round);
			let mut buf = Buf::default();
			write!(&mut buf, "{clock}").unwrap();
			assert_eq!(buf, expected_clock, "Expected {hour:02}:{minute:02}");
		}
	}
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn rounding_floor() {
	assert_eq!(
		TimeRounding::Floor.round(&MockTime {
			hour: 01,
			minute: 00
		}),
		(1, false)
	);
	assert_eq!(
		TimeRounding::Floor.round(&MockTime {
			hour: 02,
			minute: 29
		}),
		(2, false)
	);
	assert_eq!(
		TimeRounding::Floor.round(&MockTime {
			hour: 03,
			minute: 30
		}),
		(3, true)
	);
	assert_eq!(
		TimeRounding::Floor.round(&MockTime {
			hour: 03,
			minute: 59
		}),
		(3, true)
	);
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn rounding_ceil() {
	assert_eq!(
		TimeRounding::Ceil.round(&MockTime {
			hour: 01,
			minute: 00
		}),
		(1, false)
	);
	assert_eq!(
		TimeRounding::Ceil.round(&MockTime {
			hour: 02,
			minute: 29
		}),
		(2, true)
	);
	assert_eq!(
		TimeRounding::Ceil.round(&MockTime {
			hour: 03,
			minute: 30
		}),
		(3, true)
	);
	assert_eq!(
		TimeRounding::Ceil.round(&MockTime {
			hour: 03,
			minute: 59
		}),
		(4, false)
	);
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn rounding_round() {
	assert_eq!(
		TimeRounding::Round.round(&MockTime {
			hour: 01,
			minute: 00
		}),
		(1, false)
	);
	assert_eq!(
		TimeRounding::Round.round(&MockTime {
			hour: 02,
			minute: 14
		}),
		(2, false)
	);
	assert_eq!(
		TimeRounding::Round.round(&MockTime {
			hour: 03,
			minute: 15
		}),
		(3, true)
	);
	assert_eq!(
		TimeRounding::Round.round(&MockTime {
			hour: 04,
			minute: 44
		}),
		(4, true)
	);
	assert_eq!(
		TimeRounding::Round.round(&MockTime {
			hour: 05,
			minute: 45
		}),
		(6, false)
	);
}

#[test]
#[cfg(feature = "chrono")]
fn chrono() {
	let (hour, minute) = (12, 30);
	let time = ::chrono::NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
	let clock = Clock::Dial(time, TimeRounding::Round);
	let expected_clock = "🕧";
	let mut buf = Buf::default();
	write!(&mut buf, "{clock}").unwrap();
	assert_eq!(buf, expected_clock, "Expected {hour:02}:{minute:02}");
}

#[test]
#[cfg(feature = "time")]
fn time() {
	let (hour, minute) = (12, 30);
	let time = ::time::Time::from_hms(hour, minute, 0).unwrap();
	let clock = Clock::Dial(time, TimeRounding::Round);
	let expected_clock = "🕧";
	let mut buf = Buf::default();
	write!(&mut buf, "{clock}").unwrap();
	assert_eq!(buf, expected_clock, "Expected {hour:02}:{minute:02}");
}
