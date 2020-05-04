use super::*;
use crate::*;

pub type XParser<'a> =
  mapper::config::Parser<'a, XAppIdentifier, XKeySymbol, XModifier, XExecution, XIntoDomain>;
