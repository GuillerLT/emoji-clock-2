use crate::*;

use core::fmt::Write;

#[derive(Default)]
struct Buf {
	data: [u8; 8],
	len: usize,
}

impl core::fmt::Debug for Buf {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
		let data = &self.data[..self.len];
		if let Ok(str) = core::str::from_utf8(data) {
			write!(f, "{str:?}")
		} else {
			write!(f, "{data:?}")
		}
	}
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
				"Expected {expected_clock}({hour:02}:{minute:02})"
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
			let clock = Clock::new((hour, minute));
			let mut buf = Buf::default();
			write!(&mut buf, "{clock}").unwrap();
			assert_eq!(
				buf, expected_clock,
				"Expected {expected_clock}({hour:02}:{minute:02})"
			);
		}
	}
}

#[test]
fn clock_meridiem() {
	let expected = [
		// AM
		"🕛🌞", "🕧🌞", "🕐🌞", "🕜🌞", "🕑🌞", "🕝🌞", "🕒🌞", "🕞🌞", "🕓🌞", "🕟🌞", "🕔🌞",
		"🕠🌞", "🕕🌞", "🕡🌞", "🕖🌞", "🕢🌞", "🕗🌞", "🕣🌞", "🕘🌞", "🕤🌞", "🕙🌞", "🕥🌞",
		"🕚🌞", "🕦🌞", // PM
		"🕛🌝", "🕧🌝", "🕐🌝", "🕜🌝", "🕑🌝", "🕝🌝", "🕒🌝", "🕞🌝", "🕓🌝", "🕟🌝", "🕔🌝",
		"🕠🌝", "🕕🌝", "🕡🌝", "🕖🌝", "🕢🌝", "🕗🌝", "🕣🌝", "🕘🌝", "🕤🌝", "🕙🌝", "🕥🌝",
		"🕚🌝", "🕦🌝",
	];
	for (hour, expected_clocks) in (0..48).zip(expected.chunks(2).cycle()) {
		for half_hour in [false, true] {
			let expected_clock = expected_clocks[usize::from(half_hour)];
			let hour = hour % 24;
			let minute = if half_hour { 30 } else { 0 };
			let clock = Clock::new((hour, minute)).with_meridiem(Meridiem::default());
			let mut buf = Buf::default();
			write!(&mut buf, "{clock}").unwrap();
			assert_eq!(
				buf, expected_clock,
				"Expected {expected_clock}({hour:02}:{minute:02})"
			);
		}
	}
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn rounding_floor() {
	assert_eq!(Rounding::Floor.round(&(01, 00)), (01, false));
	assert_eq!(Rounding::Floor.round(&(02, 29)), (02, false));
	assert_eq!(Rounding::Floor.round(&(03, 30)), (03, true));
	assert_eq!(Rounding::Floor.round(&(03, 59)), (03, true));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn rounding_ceil() {
	assert_eq!(Rounding::Ceil.round(&(01, 00)), (01, false));
	assert_eq!(Rounding::Ceil.round(&(02, 29)), (02, true));
	assert_eq!(Rounding::Ceil.round(&(03, 30)), (03, true));
	assert_eq!(Rounding::Ceil.round(&(03, 59)), (04, false));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn rounding_round() {
	assert_eq!(Rounding::Round.round(&(01, 00)), (01, false));
	assert_eq!(Rounding::Round.round(&(02, 14)), (02, false));
	assert_eq!(Rounding::Round.round(&(03, 15)), (03, true));
	assert_eq!(Rounding::Round.round(&(04, 44)), (04, true));
	assert_eq!(Rounding::Round.round(&(05, 45)), (06, false));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn rounding_wrap() {
	assert_eq!((00, false), Rounding::Round.round(&(23, 45)));
	assert_eq!((00, false), Rounding::Ceil.round(&(23, 45)));
}

#[test]
#[allow(clippy::zero_prefixed_literal)]
fn custom_meridiem() {
	let expected = ["🕛a", "🕛p"];
	let minute = 00;
	for hour in [00, 12] {
		let expected_clock = expected[usize::from(hour >= 12)];
		let clock = Clock::new((hour, minute)).with_meridiem(Meridiem { am: 'a', pm: 'p' });
		let mut buf = Buf::default();
		write!(&mut buf, "{clock}").unwrap();
		assert_eq!(
			buf,
			expected[usize::from(hour >= 12)],
			"Expected {expected_clock}({hour:02}:{minute:02})"
		);
	}
}

#[test]
#[cfg(feature = "chrono")]
fn chrono() {
	let (hour, minute) = (12, 30);
	let time = ::chrono::NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
	let clock = Clock::new(time);
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
	let clock = Clock::new(time);
	let expected_clock = "🕧";
	let mut buf = Buf::default();
	write!(&mut buf, "{clock}").unwrap();
	assert_eq!(buf, expected_clock, "Expected {hour:02}:{minute:02}");
}
