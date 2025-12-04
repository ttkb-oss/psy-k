// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::default::Default;
use std::fmt::{Display, Formatter, Result};

/// The format used to display code.
#[derive(Default)]
pub enum CodeFormat {
    #[default]
    None,
    Hex,
    Disassembly,
}

/// Options for displaying [LIB](super::LIB) and [OBJ](super::OBJ) data.
#[derive(Default)]
pub struct Options {
    /// The code format to emit
    pub code_format: CodeFormat,

    /// Whether or not to recurse into each module of a [LIB](super::LIB)
    pub recursive: bool,
}

/// Display something with options.
pub trait DisplayWithOptions: Display {
    fn fmt_with_options(&self, f: &mut Formatter<'_>, _options: &Options) -> Result {
        self.fmt(f)
    }
}

pub struct PsyXDisplayable<'a, P: DisplayWithOptions> {
    p: &'a P,
    options: Options,
}

impl<'a, P> PsyXDisplayable<'a, P>
where
    P: DisplayWithOptions,
{
    pub fn wrap(p: &'a P, options: Options) -> PsyXDisplayable<'a, P> {
        Self { p, options }
    }
}

impl<P> Display for PsyXDisplayable<'_, P>
where
    P: DisplayWithOptions,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.p.fmt_with_options(f, &self.options)
    }
}
