#![no_std]
#![deny(missing_docs)]

//! # Examples
//!
//! ```
//! #[cfg(feature = "time")]
//! {
//!		use emoji_clock_2::{Clock, TimeRounding};
//!		use time::Time;
//!		let clock = Clock::Dial(Time::MIDNIGHT, TimeRounding::Round);
//!		assert_eq!("🕛", clock.to_string());
//! }
//! ```
//!
//! ```
//! #[cfg(feature = "chrono")]
//! {
//! 	use emoji_clock_2::{Clock, TimeRounding};
//! 	use chrono::NaiveTime;
//! 	let am_clock = Clock::DialMeridiem(NaiveTime::from_hms_opt(9, 25, 00).unwrap(), TimeRounding::Round);
//! 	assert_eq!("🕤🌞", am_clock.to_string());
//! 	let pm_clock = Clock::DialMeridiem(NaiveTime::from_hms_opt(21, 25, 00).unwrap(), TimeRounding::Round);
//! 	assert_eq!("🕤🌝", pm_clock.to_string());
//! }
//! ```

/// The common set of methods for time
pub trait TimeLike {
	/// Returns the hour number from 0 to 23
	fn hour(&self) -> u8;
	/// Returns the minute number from 0 to 59
	fn minute(&self) -> u8;
}

/// Strategies for rounding time to 30-minute precision
pub enum TimeRounding {
	/// Round to the nearest time
	/// - 01:45 - 02:14 : 02:00
	/// - 02:15 - 02:44 : 02:30
	Round,
	/// Round down to the nearest time
	/// - 02:00 - 02:29 : 02:00
	/// - 02:30 - 02:59 : 02:30
	Floor,
	/// Round up to the nearest time
	/// - 01:31 - 02:00 : 02:00
	/// - 02:01 - 02:30 : 02:30
	Ceil,
}

/// Renders a clock in emoji
/// # Examples
///
/// ```
/// #[cfg(feature = "time")]
/// {
///		use emoji_clock_2::{Clock, TimeRounding};
///		use time::Time;
///		let clock = Clock::Dial(Time::MIDNIGHT, TimeRounding::Round);
///		assert_eq!("🕛", clock.to_string());
/// }
/// ```
///
/// ```
/// #[cfg(feature = "chrono")]
/// {
/// 	use emoji_clock_2::{Clock, TimeRounding};
/// 	use chrono::NaiveTime;
/// 	let am_clock = Clock::DialMeridiem(NaiveTime::from_hms_opt(9, 25, 00).unwrap(), TimeRounding::Round);
/// 	assert_eq!("🕤🌞", am_clock.to_string());
/// 	let pm_clock = Clock::DialMeridiem(NaiveTime::from_hms_opt(21, 25, 00).unwrap(), TimeRounding::Round);
/// 	assert_eq!("🕤🌝", pm_clock.to_string());
/// }
/// ```
#[non_exhaustive]
pub enum Clock<T>
where
	T: TimeLike,
{
	/// Dial with 12 hour time
	Dial(T, TimeRounding),
	/// Dial with 12 hour time and AM/PM indication (🌞/🌝)
	DialMeridiem(T, TimeRounding),
}

impl<T> core::fmt::Display for Clock<T>
where
	T: TimeLike,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let (time, rounding, meridiem) = match self {
			Self::Dial(time, rounding) => (time, rounding, false),
			Self::DialMeridiem(time, rounding) => (time, rounding, true),
		};
		let (hour, half_hour) = rounding.round(time);
		let clock = clock_emoji(hour, half_hour);
		#[allow(clippy::zero_prefixed_literal)]
		let meridiem = match meridiem {
			false => "",
			true => match hour {
				01..=12 => "🌞",
				13..=23 | 00 => "🌝",
				_ => return Err(core::fmt::Error),
			},
		};
		write!(f, "{clock}{meridiem}")
	}
}

/// Returns the emoji clock with the correct hour hand and minute hand, accurate to 30-minute precision
///
/// # Panics
///
/// Panics if the resulting character is not valid
#[must_use]
pub fn clock_emoji(hour: u8, half_hour: bool) -> char {
	char::from_u32(128336 + (u32::from(hour) + 11) % 12 + if half_hour { 12 } else { 0 }).unwrap()
}

impl TimeRounding {
	/// Rounds time to 30-min precision
	fn round<Time>(&self, time: &Time) -> (u8, bool)
	where
		Time: TimeLike,
	{
		let (hour, minute) = (time.hour(), time.minute());
		assert!(hour <= 23, "Hour greater than 23");
		#[allow(clippy::zero_prefixed_literal)]
		match self {
			Self::Floor => match minute {
				00..=29 => (hour, false),
				30..=59 => (hour, true),
				_ => panic!("Minute greater than 59"),
			},
			Self::Ceil => match minute {
				0 => (hour, false),
				01..=30 => (hour, true),
				31..=59 => (hour + 1, false),
				_ => panic!("Minute greater than 59"),
			},
			Self::Round => match minute {
				00..=14 => (hour, false),
				15..=44 => (hour, true),
				45..=59 => (hour + 1, false),
				_ => panic!("Minute greater than 59"),
			},
		}
	}
}

#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "time")]
mod time;

#[cfg(test)]
mod tests;
