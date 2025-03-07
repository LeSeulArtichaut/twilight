# Changelog

Changelog for `twilight-standby`.

## [0.5.1] - 2021-07-23

### Changes

`#![deny(unsafe_code)]` has been added, ensuring no unsafe code exists in the
crate ([#1042] - [@zeylahellyer]).

[#1042]: https://github.com/twilight-rs/twilight/pull/1042

## [0.5.0] - 2021-06-13

This major version bump of the Standby crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.4.1] - 2021-06-12

### Changes

Support the new events added in model: `IntegrationCreate, IntegrationDelete,
IntegrationUpdate, StageInstanceCreate`, `StageInstanceDelete`,
`StageInstanceUpdate` ([#845], [#914] - [@7596ff]).

[#845]: https://github.com/twilight-rs/twilight/pull/845
[#914]: https://github.com/twilight-rs/twilight/pull/914

## [0.4.0] - 2021-05-12

### Upgrade Path

The MSRV is now Rust 1.49.

Errors are no longer enums and don't expose their concrete underlying error
source. You can access the underlying error via the implemented
`std::error::Error::source` method or the `into_parts` or `into_source` methods
on each error struct, which will return a boxed `std::error::Error`. To access
the reason for the error use the `kind` or `into_parts` method on error structs;
the returned error type is an enum with variants for each potential reason the
error occurred.

`tokio` is now a required runtime dependency.

### Enhancements

The `futures-channel` dependency has been removed in favor of `tokio` due to the
dependency of it on other Twilight crates ([#785] - [@Gelbpunkt]).

[#785]: https://github.com/twilight-rs/twilight/pull/785

## [0.3.0] - 2021-01-08

This major version bump of the Standby crate is done to match all of the other
crates in the ecosystem receiving a major version bump. There are no changes.

## [0.2.2] - 2021-01-05

### Enhancements

Upgrade `dashmap` from version 3 to 4.0 ([#666] - [@vivian]).

[#666]: https://github.com/twilight-rs/twilight/pull/666

## [0.2.1] - 2020-11-29

### Misc.

Use the renamed
`twilight_model::gateway::payload::identify::IdentityInfo::compress`
model field ([#624] - [@chamburr]).

## [0.2.0] - 2020-10-30

This major version bump of Standby is done to match all of the other crates in
the ecosystem receiving a major version bump. There are no changes.

## [0.2.0-beta.0] - 2020-10-10

This major version bump of Standby is done to match all of the other crates in
the ecosystem receiving a major version bump. There are no changes.

## [0.1.1] - 2020-09-25

### Fixes

- Fix typo in documentation link ([#523] - [@nickelc])

## [0.1.0] - 2020-09-13

Initial release.

[@7596ff]: https://github.com/7596ff
[@chamburr]: https://github.com/chamburr
[@Gelbpunkt]: https://github.com/Gelbpunkt
[@nickelc]: https://github.com/nickelc
[@vivian]: https://github.com/vivian
[@zeylahellyer]: https://github.com/zeylahellyer

[#624]: https://github.com/twilight-rs/twilight/pull/624
[#523]: https://github.com/twilight-rs/twilight/pull/523

[0.5.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.5.1
[0.5.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.5.0
[0.4.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.4.1
[0.4.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-0.4.0
[0.3.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.3.0
[0.2.2]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.2.2
[0.2.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.2.1
[0.2.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.2.0
[0.2.0-beta.0]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.2.0-beta.0
[0.1.1]: https://github.com/twilight-rs/twilight/releases/tag/standby-v0.1.1
[0.1.0]: https://github.com/twilight-rs/twilight/releases/tag/v0.1.0
