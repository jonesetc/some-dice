//! An independent implementation of the [AnyDice][0] language, parser, interpreter, and sampler.
//!
//! All reasonable efforts are taken to maintain compatibility, but there could be unintended differences.
//! It is also possible that the AnyDice reference implementation changes.
//! Feel free to open an [issue][2] on GitHub about any incompatibilities.
//!
//! More information for what these types represent can be found in the AnyDice [documentation][1].
//!
//! [0]: https://anydice.com/
//! [1]: https://anydice.com/docs/
//! [2]: https://github.com/jonesetc/some-dice/issues

pub mod ast;
#[cfg(feature = "interpret")]
pub mod interpret;
#[cfg(any(feature = "interpret", feature = "sample"))]
pub mod outcome;
#[cfg(feature = "parse")]
pub mod parse;
#[cfg(feature = "sample")]
pub mod sample;
