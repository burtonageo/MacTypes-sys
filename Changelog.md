# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/) 
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased
### Changed
- Made `StrLength` always inline, and not have `extern "C"` linkage because
  it is used as an internal/inline interface (and is declared as a macro when
  compiled as `C` in the header.

### Added
- Added `Float32Point` struct.

## [1.0.4] - 2016-10-19
### Changed
- Made kVariableLengthArray a variable of type ItemCount, and kUnknownType
  an OSType.
- Shortened the various `developStage` variables to `UInt8`, from `UInt32`.

### Added
- Added documentation link to Cargo.toml and Readme.md.

## [1.0.3] - 2016-09-20
### Changed
- Do not use any features in the libc crate.
- Float32/Float64 now alias to f32/f64 instead of c_float/c_double.

## [1.0.2] - 2016-09-14
### Changed
- Remove the WinApi dependency.
- Use rust-native fixed sized int types instead of libc's stdint types.

### Fixed

- Bugfix: Mark Float80 and Float96 as #[repr(C)]

## [1.0.1] - 2016-09-08
### Added
- Added crate-level documentation comment.

### Fixed
- Fixed typos in Readme.md

## [1.0.0] - 2016-09-06
### Added
- First release.

