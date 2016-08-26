// Copyright 2016 immi Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod circular_progress_bar;
pub mod image;
pub mod image9;
pub mod image_button;
pub mod image9_button;
pub mod label;
pub mod progress_bar;

/// Whether the cursor clicked on the widget.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum Interaction {
    /// The cursor clicked.
    Clicked,
    /// The cursor didn't click.
    None,
}

impl Interaction {
    #[inline]
    pub fn clicked(&self) -> bool {
        match self {
            &Interaction::Clicked => true,
            _ => false,
        }
    }
}
