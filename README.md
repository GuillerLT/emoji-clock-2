# emoji-clock-2

This library is inspired by [emoji-clock](https://docs.rs/emoji-clock/).

- crates.io

## Differences from [emoji-clock](https://docs.rs/emoji-clock/)
- [time](https://docs.rs/time/) and [chrono](https://docs.rs/chrono/) compatibility: supports both chrono and time libraries through features. By default, none are enabled, giving you the freedom to choose.
- `no_std`
- 30-Minute precision: more detailed time representation with double the granularity.
- Rounding options: round, floor or ceil.
