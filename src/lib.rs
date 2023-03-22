// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Numeric traits for generic mathematics
//!
//! This version of the crate only exists to re-export compatible
//! items from num-traits 0.2.  Please consider updating!

#![doc(html_root_url = "https://docs.rs/num-traits/0.1")]

extern crate num_traits;

pub use bounds::Bounded;
#[cfg(feature = "floats")]
pub use float::{Float, FloatConst};
// pub use real::Real; // NOTE: Don't do this, it breaks `use num_traits::*;`.
pub use cast::{cast, AsPrimitive, FromPrimitive, NumCast, ToPrimitive};
pub use identities::{one, zero, One, Zero};
pub use int::PrimInt;
pub use ops::checked::*;
pub use ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedShl, CheckedShr, CheckedSub};
pub use ops::saturating::Saturating;
pub use ops::wrapping::*;
pub use ops::wrapping::{WrappingAdd, WrappingMul, WrappingSub};
pub use pow::{checked_pow, pow};
pub use sign::{abs, abs_sub, signum, Signed, Unsigned};

// Re-exports from num-traits 0.2!

pub use num_traits::clamp;
#[cfg(feature = "floats")]
pub use num_traits::{FloatErrorKind, ParseFloatError};
pub use num_traits::{Num, NumOps, NumRef, RefNum};
pub use num_traits::{NumAssign, NumAssignOps, NumAssignRef};

// Note: the module structure is explicitly re-created, rather than re-exporting en masse,
// so we won't expose any items that may be added later in the new version.

pub mod identities {
    pub use num_traits::identities::{one, zero, One, Zero};
}

pub mod sign {
    pub use num_traits::sign::{abs, abs_sub, signum, Signed, Unsigned};
}

pub mod ops {
    pub mod saturating {
        pub use num_traits::ops::saturating::Saturating;
    }

    pub mod checked {
        pub use num_traits::ops::checked::{
            CheckedAdd, CheckedDiv, CheckedMul, CheckedShl, CheckedShr, CheckedSub,
        };
    }

    pub mod wrapping {
        pub use num_traits::ops::wrapping::{WrappingAdd, WrappingMul, WrappingSub};
    }
}

pub mod bounds {
    pub use num_traits::bounds::Bounded;
}

#[cfg(feature = "floats")]
pub mod float {
    pub use num_traits::float::{Float, FloatConst};
}

pub mod real {
    pub use num_traits::real::Real;
}

pub mod cast {
    pub use num_traits::cast::{cast, AsPrimitive, FromPrimitive, NumCast, ToPrimitive};
}

pub mod int {
    pub use num_traits::int::PrimInt;
}

pub mod pow {
    pub use num_traits::pow::{checked_pow, pow};
}
