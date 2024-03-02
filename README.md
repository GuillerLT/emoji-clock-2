# emoji-clock-2

- [docs.rs](https://docs.rs/emoji-clock-2/)
- [crates.io](https://crates.io/crates/emoji-clock-2)

```rust
use emoji_clock_2::{Clock, TimeRounding};
use time::Time;
let clock = Clock::Dial(Time::MIDNIGHT, TimeRounding::Round);
assert_eq!("🕛", clock.to_string());
```

```rust
use emoji_clock_2::{Clock, TimeRounding};
use chrono::NaiveTime;
let am_clock = Clock::DialMeridiem(NaiveTime::from_hms_opt(9, 25, 00).unwrap(), TimeRounding::Round);
assert_eq!("🕤🌞", am_clock.to_string());
let pm_clock = Clock::DialMeridiem(NaiveTime::from_hms_opt(21, 25, 00).unwrap(), TimeRounding::Round);
assert_eq!("🕤🌝", pm_clock.to_string());
```

This library is inspired by [emoji-clock](https://docs.rs/emoji-clock/).

## Differences from [emoji-clock](https://docs.rs/emoji-clock/)
- [time](https://docs.rs/time/) and [chrono](https://docs.rs/chrono/) compatibility: supports both chrono and time libraries through features. By default, none are enabled, giving you the freedom to choose.
- `no_std`
- 30-Minute precision: more detailed time representation with double the granularity.
- Rounding options: round, floor or ceil.
