# emoji-clock-2

- [docs.rs](https://docs.rs/emoji-clock-2/)
- [crates.io](https://crates.io/crates/emoji-clock-2)

```rust
use time::Time; // feature time
use emoji_clock_2::{Clock, Rounding};
let clock = Clock::new(Time::from_hms(12, 29, 00).unwrap()).with_rounding(Rounding::Floor);
assert_eq!("🕛", clock.to_string());
```

```rust
use chrono::NaiveTime; // feature chrono
use emoji_clock_2::{Clock, Meridiem};
let am_clock = Clock::new(NaiveTime::from_hms_opt(9, 15, 00).unwrap()).with_meridiem(Meridiem::default());
assert_eq!("🕤🌞", am_clock.to_string());
let pm_clock = Clock::new(NaiveTime::from_hms_opt(21, 44, 00).unwrap()).with_meridiem(Meridiem{ am: '🌞', pm: '🌙' });
assert_eq!("🕤🌙", pm_clock.to_string());
```

This library is inspired by [emoji-clock](https://docs.rs/emoji-clock/).

## Differences from [emoji-clock](https://docs.rs/emoji-clock/)
- [time](https://docs.rs/time/) and [chrono](https://docs.rs/chrono/) compatibility: supports both chrono and time libraries through features. By default, none are enabled, giving you the freedom to choose.
- 30-Minute precision: more detailed time representation with double the granularity.
- Rounding options: round, floor or ceil.
- `no_std`
