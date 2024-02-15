# Changelog

## [v0.11.1](https://github.com/pywr/pywr-schema/compare/v0.11.0...v0.11.1) (2024-02-15)

## [v0.11.0](https://github.com/pywr/pywr-schema/compare/v0.10.0...v0.11.0) (2024-01-03)

### Features

* Implemented NegativeMaxParameter and NegativeMinParameter (#22)
([6ff0865](https://github.com/pywr/pywr-schema/commit/6ff0865dc704ecb592b07ee86e36739de1fc756a)),
closes [#22](https://github.com/pywr/pywr-schema/issues/22)

## [v0.10.0](https://github.com/pywr/pywr-schema/compare/v0.9.0...v0.10.0) (2023-12-12)

### Features

* Add strum EnumVariantNames macro to CoreNode enum (#20)
([cf2d00e](https://github.com/pywr/pywr-schema/commit/cf2d00e5b5ea14c0787c0353ff12d5ef044fa498)),
closes [#20](https://github.com/pywr/pywr-schema/issues/20)

## [v0.9.0](https://github.com/pywr/pywr-schema/compare/v0.8.0...v0.9.0) (2023-11-25)

### Features

* Add RbfProfileParameter. (#16)
([1575d33](https://github.com/pywr/pywr-schema/commit/1575d334afa12da70bdcde71aa7ec15e9b37c425)),
closes [#16](https://github.com/pywr/pywr-schema/issues/16)

### Fixes

* Make RbfProfileParameter public in parameters module. (#18)
([7bbf013](https://github.com/pywr/pywr-schema/commit/7bbf013bd5add6f7567aafa3bf86d28c5164efcf)),
closes [#18](https://github.com/pywr/pywr-schema/issues/18)

## [v0.8.0](https://github.com/pywr/pywr-schema/compare/v0.7.0...v0.8.0) (2023-10-13)

### Features

* Add PywrParameter derive
([a21d02c](https://github.com/pywr/pywr-schema/commit/a21d02cf0416f957a61bed9c8541799b656bfa78))
* Use proc-macro to generate some Node methods.
([a4efb91](https://github.com/pywr/pywr-schema/commit/a4efb91f7058770ad6953304c531508619374bce))
* Improve resource path discovery.
([ff831c2](https://github.com/pywr/pywr-schema/commit/ff831c23b135c16bb6521a3689cbf724bb502bf9))
* Add additional parameters.
([abc1c75](https://github.com/pywr/pywr-schema/commit/abc1c757d0e11292e1cf4b23d0abcee6501f732c))

### Fixes

* Skip serializing None for most fields.
([28e3a33](https://github.com/pywr/pywr-schema/commit/28e3a3376154acd51fe2fe6059f507c9e62f90bd))

## [v0.7.0](https://github.com/pywr/pywr-schema/compare/v0.6.0...v0.7.0) (2023-09-16)

### Features

* Added DivisionParameter schema (#12)
([329cd89](https://github.com/pywr/pywr-schema/commit/329cd898cf596d28873c51fd55ff4cbe0e5d5e09)),
closes [#12](https://github.com/pywr/pywr-schema/issues/12)
* Added MinParameter 
([b182fa3](https://github.com/pywr/pywr-schema/commit/b182fa3981057148fb8555fff03dccd708d5052d))

## [v0.6.0](https://github.com/pywr/pywr-schema/compare/v0.5.3...v0.6.0) (2023-05-10)

### Features

* Derive Clone for the whole schema. (#9)
([4829f76](https://github.com/pywr/pywr-schema/commit/4829f76b09a32dbf6ab965334dff04df0644acc1)),
closes [#9](https://github.com/pywr/pywr-schema/issues/9)
* Derive Clone for the whole schema.
([8c0c117](https://github.com/pywr/pywr-schema/commit/8c0c117d7d7897d51964b690913b5081311c55a0))

### [v0.5.3](https://github.com/pywr/pywr-schema/compare/v0.5.2...v0.5.3) (2023-03-06)

#### Fixes

* Fix DelayNode schema. (#7)
([330dde9](https://github.com/pywr/pywr-schema/commit/330dde97c62342b5be33e54974e511b24a376c07)),
closes [#7](https://github.com/pywr/pywr-schema/issues/7)

### [v0.5.2](https://github.com/pywr/pywr-schema/compare/v0.5.1...v0.5.2) (2023-03-06)

#### Fixes

* Add missing nsteps attribute of PiecewiseLink. (#6)
([35c128d](https://github.com/pywr/pywr-schema/commit/35c128df4f001de685d877026365353d995bbe63)),
closes [#6](https://github.com/pywr/pywr-schema/issues/6)

### [v0.5.1](https://github.com/pywr/pywr-schema/compare/v0.5.0...v0.5.1) (2023-03-03)

#### Fixes

* River split node factors should not be optional.
([d499097](https://github.com/pywr/pywr-schema/commit/d499097e0212513f79686f61586cf30642e6ecdf))
* Remove unused imports.
([41fb951](https://github.com/pywr/pywr-schema/commit/41fb95169f2e8878e85301c94418321b5bff3959))

## [v0.5.0](https://github.com/pywr/pywr-schema/compare/v0.4.0...v0.5.0) (2023-03-03)

### Features

* add schemas for remaining nodes (#1)
([6662df9](https://github.com/pywr/pywr-schema/commit/6662df9bc62117fa09ab4ca32a7d772b4437a383)),
closes [#1](https://github.com/pywr/pywr-schema/issues/1)
* Add interp_day to MonthlyProfileParameter. (#3)
([5aafa46](https://github.com/pywr/pywr-schema/commit/5aafa4633a6d696818eb694d19ae128e7d41c301)),
closes [#3](https://github.com/pywr/pywr-schema/issues/3)

### Fixes

* Add missing index_parameter to parameters() method in IndexedArrayParameter.
(#2)
([710d8e0](https://github.com/pywr/pywr-schema/commit/710d8e0e2211fbc117e070766520798da6daaedd)),
closes [#2](https://github.com/pywr/pywr-schema/issues/2)

## [v0.4.0](https://github.com/pywr/pywr-schema/compare/v0.3.0...v0.4.0) (2023-01-31)

### Features

* Add methods for finding parameters.
([6ab6a41](https://github.com/pywr/pywr-schema/commit/6ab6a41ac1427b391d2dedc8518d4246f9e74ae4))

## [v0.3.0](https://github.com/pywr/pywr-schema/compare/v0.2.0...v0.3.0) (2023-01-20)

### Features

* Add additional tests and swap enum order in ParameterValue.
([878dbc7](https://github.com/pywr/pywr-schema/commit/878dbc795150dc497fd095d59572d51c6a2da452))

## v0.2.0 (2022-12-21)

### Features

* Add additional nodes and parameters.
([cf0b8df](https://github.com/pywr/pywr-schema/commit/cf0b8df4c0bff260fbbd28f72a272b5bd8ed0c99))
