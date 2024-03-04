#![no_std]
#![deny(missing_docs)]

//! # Examples
//!
//! ```
//! #[cfg(feature = "time")]
//! {
//!   use time::Time;
//!   use emoji_clock_2::{Clock, Rounding};
//!   let clock = Clock::new(Time::from_hms(12, 29, 00).unwrap()).with_rounding(Rounding::Floor);
//!   assert_eq!("🕛", clock.to_string());
//! }
//! ```
//!
//! ```
//! #[cfg(feature = "chrono")]
//! {
//!   use chrono::NaiveTime;
//!   use emoji_clock_2::{Clock, Meridiem};
//!   let am_clock = Clock::new(NaiveTime::from_hms_opt(9, 15, 00).unwrap()).with_meridiem(Meridiem::default());
//!   assert_eq!("🕤🌞", am_clock.to_string());
//!   let pm_clock = Clock::new(NaiveTime::from_hms_opt(21, 44, 00).unwrap()).with_meridiem(Meridiem{ am: '🌞', pm: '🌙' });
//!   assert_eq!("🕤🌙", pm_clock.to_string());
//! }
//! ```

/// Renders a clock in emoji
/// # Examples
///
/// ```
/// #[cfg(feature = "time")]
/// {
///   use time::Time;
///   use emoji_clock_2::{Clock, Rounding};
///   let clock = Clock::new(Time::from_hms(12, 29, 00).unwrap()).with_rounding(Rounding::Floor);
///   assert_eq!("🕛", clock.to_string());
/// }
/// ```
///
/// ```
/// #[cfg(feature = "chrono")]
/// {
///   use chrono::NaiveTime;
///   use emoji_clock_2::{Clock, Meridiem};
///   let am_clock = Clock::new(NaiveTime::from_hms_opt(9, 15, 00).unwrap()).with_meridiem(Meridiem::default());
///   assert_eq!("🕤🌞", am_clock.to_string());
///   let pm_clock = Clock::new(NaiveTime::from_hms_opt(21, 44, 00).unwrap()).with_meridiem(Meridiem{ am: '🌞', pm: '🌙' });
///   assert_eq!("🕤🌙", pm_clock.to_string());
/// }
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Clock<Time>
where
	Time: TimeLike,
{
	time: Time,
	rounding: Rounding,
	meridiem: Option<Meridiem>,
}

impl<T> core::fmt::Display for Clock<T>
where
	T: TimeLike,
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let (hour, half_hour) = self.rounding.round(&self.time);
		let clock = clock_emoji(hour, half_hour);

		if let Some(Meridiem { am, pm }) = self.meridiem {
			#[allow(clippy::zero_prefixed_literal)]
			let meridiem = match hour {
				00..=11 => am,
				12..=23 => pm,
				_ => return Err(core::fmt::Error),
			};
			write!(f, "{clock}{meridiem}")
		} else {
			write!(f, "{clock}")
		}
	}
}

impl<Time> Clock<Time>
where
	Time: TimeLike,
{
	/// Creates a new clock with round strategy and no meridiem indicators
	#[must_use]
	pub const fn new(time: Time) -> Self {
		Self {
			time,
			rounding: Rounding::Round,
			meridiem: None,
		}
	}
	/// Sets rounding strategy ([Round](crate::Rounding::Round) by default)
	#[must_use]
	pub fn with_rounding(self, rounding: Rounding) -> Self {
		Self {
			time: self.time,
			rounding,
			meridiem: self.meridiem,
		}
	}
	/// Enables meridiem (AM/PM) indicators (disabled by default)
	#[must_use]
	pub fn with_meridiem(self, meridiem: Meridiem) -> Self {
		Self {
			time: self.time,
			rounding: self.rounding,
			meridiem: Some(meridiem),
		}
	}
}

/// The common set of methods for time
pub trait TimeLike {
	/// Returns the hour number from 0 to 23
	fn hour(&self) -> u8;
	/// Returns the minute number from 0 to 59
	fn minute(&self) -> u8;
}

/// Meridiem (AM/PM) indicators
#[derive(Debug, Copy, Clone)]
pub struct Meridiem {
	/// AM indicator
	pub am: char,
	/// PM indicator
	pub pm: char,
}

impl Default for Meridiem {
	/// Default meridiem (AM/PM) indicators (🌞/🌝)
	fn default() -> Self {
		Self {
			am: '🌞', pm: '🌝'
		}
	}
}

/// Strategies for rounding time to emoji clock precision (30-minute granularity)
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Rounding {
	/// Round to the nearest emoji clock
	/// - 01:45 - 02:14 : 02:00 🕑
	/// - 02:15 - 02:44 : 02:30 🕝
	Round,
	/// Round down to the nearest emoji clock
	/// - 02:00 - 02:29 : 02:00 🕑
	/// - 02:30 - 02:59 : 02:30 🕝
	Floor,
	/// Round up to the nearest emoji clock
	/// - 01:31 - 02:00 : 02:00 🕑
	/// - 02:01 - 02:30 : 02:30 🕝
	Ceil,
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

impl Rounding {
	/// Rounds time to 30-min precision
	fn round<Time>(&self, time: &Time) -> (u8, bool)
	where
		Time: TimeLike,
	{
		let (hour, minute) = (time.hour(), time.minute());
		assert!(hour <= 23, "Hour greater than 23");
		#[allow(clippy::zero_prefixed_literal)]
		match *self {
			Self::Floor => match minute {
				00..=29 => (hour, false),
				30..=59 => (hour, true),
				_ => panic!("Minute greater than 59"),
			},
			Self::Ceil => match minute {
				0 => (hour, false),
				01..=30 => (hour, true),
				31..=59 => ((hour + 1) % 24, false),
				_ => panic!("Minute greater than 59"),
			},
			Self::Round => match minute {
				00..=14 => (hour, false),
				15..=44 => (hour, true),
				45..=59 => ((hour + 1) % 24, false),
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

impl TimeLike for (u8, u8) {
	fn hour(&self) -> u8 {
		self.0
	}
	fn minute(&self) -> u8 {
		self.1
	}
}

impl core::convert::From<(char, char)> for Meridiem {
	fn from((am, pm): (char, char)) -> Self {
		Self { am, pm }
	}
}

impl<Time> core::ops::Deref for Clock<Time>
where
	Time: TimeLike,
{
	type Target = Time;
	fn deref(&self) -> &Self::Target {
		&self.time
	}
}

impl<Time> core::ops::DerefMut for Clock<Time>
where
	Time: TimeLike,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.time
	}
}
