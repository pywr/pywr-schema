# Changelog

All notable changes to this project will be documented in this file.

## [0.13.0] - 2024-04-25

### Features

- Separate out the network from the model. ([#62](https://github.com/orhun/git-cliff/issues/62))

## [0.12.0] - 2024-04-23

### Miscellaneous Tasks

- Bump thiserror from 1.0.58 to 1.0.59 ([#59](https://github.com/orhun/git-cliff/issues/59))
- Release v0.12.0 ([#61](https://github.com/orhun/git-cliff/issues/61))

### Refactor

- Use chrono instead of time for consistency with pywr-next. ([#60](https://github.com/orhun/git-cliff/issues/60))

## [0.11.2] - 2024-04-19

### Bug Fixes

- Remove parameter attribute from ControlCurveIndexParameter definition ([#54](https://github.com/orhun/git-cliff/issues/54))

### Miscellaneous Tasks

- Bump time from 0.3.15 to 0.3.34 ([#38](https://github.com/orhun/git-cliff/issues/38))
- Bump serde_json from 1.0.109 to 1.0.113 ([#37](https://github.com/orhun/git-cliff/issues/37))
- Bump serde from 1.0.193 to 1.0.196 ([#35](https://github.com/orhun/git-cliff/issues/35))
- Bump syn from 2.0.48 to 2.0.49 ([#34](https://github.com/orhun/git-cliff/issues/34))
- Bump clap from 4.5.0 to 4.5.1 ([#36](https://github.com/orhun/git-cliff/issues/36))
- Bump syn from 2.0.49 to 2.0.51 ([#39](https://github.com/orhun/git-cliff/issues/39))
- Bump serde from 1.0.196 to 1.0.197 ([#41](https://github.com/orhun/git-cliff/issues/41))
- Bump serde_json from 1.0.113 to 1.0.114 ([#40](https://github.com/orhun/git-cliff/issues/40))
- Bump clap from 4.5.1 to 4.5.3 ([#47](https://github.com/orhun/git-cliff/issues/47))
- Bump thiserror from 1.0.57 to 1.0.58 ([#46](https://github.com/orhun/git-cliff/issues/46))
- Bump strum_macros from 0.26.1 to 0.26.2 ([#43](https://github.com/orhun/git-cliff/issues/43))
- Bump strum from 0.26.1 to 0.26.2 ([#45](https://github.com/orhun/git-cliff/issues/45))
- Bump syn from 2.0.51 to 2.0.53 ([#48](https://github.com/orhun/git-cliff/issues/48))
- Bump serde_json from 1.0.114 to 1.0.115 ([#51](https://github.com/orhun/git-cliff/issues/51))
- Bump clap from 4.5.3 to 4.5.4 ([#52](https://github.com/orhun/git-cliff/issues/52))
- Bump syn from 2.0.53 to 2.0.58 ([#53](https://github.com/orhun/git-cliff/issues/53))
- Bump syn from 2.0.58 to 2.0.59 ([#57](https://github.com/orhun/git-cliff/issues/57))
- Bump time from 0.3.34 to 0.3.36 ([#55](https://github.com/orhun/git-cliff/issues/55))
- Bump quote from 1.0.35 to 1.0.36 ([#56](https://github.com/orhun/git-cliff/issues/56))
- Release v0.11.2 ([#58](https://github.com/orhun/git-cliff/issues/58))

## [0.11.1] - 2024-02-15

### Miscellaneous Tasks

- Update strum to v0.26 ([#24](https://github.com/orhun/git-cliff/issues/24))
- Bump actions/checkout from 2 to 4 ([#26](https://github.com/orhun/git-cliff/issues/26))
- Bump quote from 1.0.33 to 1.0.35 ([#27](https://github.com/orhun/git-cliff/issues/27))
- Bump serde from 1.0.145 to 1.0.193 ([#28](https://github.com/orhun/git-cliff/issues/28))
- Bump clap from 4.0.12 to 4.5.0 ([#29](https://github.com/orhun/git-cliff/issues/29))
- Bump thiserror from 1.0.48 to 1.0.55 ([#30](https://github.com/orhun/git-cliff/issues/30))
- Bump serde_json from 1.0.86 to 1.0.109 ([#31](https://github.com/orhun/git-cliff/issues/31))
- Bump thiserror from 1.0.48 to 1.0.57 ([#32](https://github.com/orhun/git-cliff/issues/32))
- Release v0.11.1 ([#33](https://github.com/orhun/git-cliff/issues/33))

### Ci

- Add dependabot config. ([#25](https://github.com/orhun/git-cliff/issues/25))

## [0.11.0] - 2024-01-03

### Features

- Implemented NegativeMaxParameter and NegativeMinParameter ([#22](https://github.com/orhun/git-cliff/issues/22))

### Miscellaneous Tasks

- Release v0.11.0 ([#23](https://github.com/orhun/git-cliff/issues/23))

## [0.10.0] - 2023-12-12

### Features

- Add strum EnumVariantNames macro to CoreNode enum ([#20](https://github.com/orhun/git-cliff/issues/20))

### Miscellaneous Tasks

- Release v0.10.0 ([#21](https://github.com/orhun/git-cliff/issues/21))

## [0.9.0] - 2023-11-25

### Bug Fixes

- Make RbfProfileParameter public in parameters module. ([#18](https://github.com/orhun/git-cliff/issues/18))

### Features

- Add RbfProfileParameter. ([#16](https://github.com/orhun/git-cliff/issues/16))

### Miscellaneous Tasks

- Release v0.9.0 ([#19](https://github.com/orhun/git-cliff/issues/19))

## [0.8.0] - 2023-10-13

### Bug Fixes

- Skip serializing None for most fields.

### Features

- Add additional parameters.
- Improve resource path discovery.
- Use proc-macro to generate some Node methods.
- Add PywrParameter derive

### Miscellaneous Tasks

- Add additional comments to pywr-schema-macros
- Release v0.8.0 ([#15](https://github.com/orhun/git-cliff/issues/15))

## [0.7.0] - 2023-09-16

### Features

- Added MinParameter 
- Added DivisionParameter schema ([#12](https://github.com/orhun/git-cliff/issues/12))

### Miscellaneous Tasks

- Release v0.7.0 ([#13](https://github.com/orhun/git-cliff/issues/13))

## [0.6.0] - 2023-05-10

### Features

- Derive Clone for the whole schema.
- Derive Clone for the whole schema. ([#9](https://github.com/orhun/git-cliff/issues/9))

### Miscellaneous Tasks

- Release v0.6.0 ([#10](https://github.com/orhun/git-cliff/issues/10))

## [0.5.3] - 2023-03-06

### Bug Fixes

- Fix DelayNode schema. ([#7](https://github.com/orhun/git-cliff/issues/7))

### Miscellaneous Tasks

- Setup CI. ([#8](https://github.com/orhun/git-cliff/issues/8))
- Release v0.5.3

## [0.5.2] - 2023-03-06

### Bug Fixes

- Add missing nsteps attribute of PiecewiseLink. ([#6](https://github.com/orhun/git-cliff/issues/6))

### Miscellaneous Tasks

- Release v0.5.2

## [0.5.1] - 2023-03-03

### Bug Fixes

- Remove unused imports.
- River split node factors should not be optional.

### Miscellaneous Tasks

- Release v0.5.1

## [0.5.0] - 2023-03-03

### Bug Fixes

- Add missing index_parameter to parameters() method in IndexedArrayParameter. ([#2](https://github.com/orhun/git-cliff/issues/2))

### Features

- Add interp_day to MonthlyProfileParameter. ([#3](https://github.com/orhun/git-cliff/issues/3))
- Add schemas for remaining nodes ([#1](https://github.com/orhun/git-cliff/issues/1))

## [0.4.0] - 2023-01-31

### Features

- Add methods for finding parameters.

### Miscellaneous Tasks

- Release v0.4.0

## [0.3.0] - 2023-01-20

### Features

- Add additional tests and swap enum order in ParameterValue.

### Miscellaneous Tasks

- Release v0.3.0

## [0.2.0] - 2022-12-21

### Features

- Add additional nodes and parameters.

<!-- generated by git-cliff -->
