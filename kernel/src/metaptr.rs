// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Google LLC 2024.

//! Defines the MetaPtr type

use core::fmt::{Formatter, LowerHex, UpperHex};
use core::ops::AddAssign;

/// A pointer with target specific metadata.
/// This should be used any time the kernel wishes to grant authority to the user, or any time
/// the user should be required to prove validity of a pointer.
#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct MetaPtr {
    ptr: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum MetaPermissions {
    Any,
    Read,
    Write,
    ReadWrite,
    Execute,
}

impl From<MetaPtr> for usize {
    #[inline]
    fn from(from: MetaPtr) -> Self {
        from.ptr
    }
}

impl From<usize> for MetaPtr {
    #[inline]
    fn from(from: usize) -> Self {
        Self { ptr: from }
    }
}

impl UpperHex for MetaPtr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        UpperHex::fmt(&self.ptr, f)
    }
}

impl LowerHex for MetaPtr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        LowerHex::fmt(&self.ptr, f)
    }
}

impl AddAssign<usize> for MetaPtr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.ptr.add_assign(rhs)
    }
}

impl MetaPtr {
    pub fn as_ptr(&self) -> *const () {
        self.ptr as *const ()
    }

    /// Convert to a raw pointer, checking that metadata allows a particular set of permissions over
    /// a given number of bytes.
    /// If the metadata does not allow for this, returns null.
    /// If no such metadata exists, this succeeds.
    #[inline]
    pub fn as_ptr_checked(&self, _length: usize, _perms: MetaPermissions) -> *const () {
        self.ptr as *const ()
    }

    #[inline]
    pub fn new_with_metadata(
        ptr: *const (),
        _base: usize,
        _length: usize,
        _perms: MetaPermissions,
    ) -> Self {
        Self { ptr: ptr as usize }
    }

    #[inline]
    pub fn map_or<U, F>(&self, default: U, f: F) -> U
    where
        F: FnOnce(&Self) -> U,
    {
        if self.ptr == 0usize {
            default
        } else {
            f(self)
        }
    }

    #[inline]
    pub fn map_or_else<U, D, F>(&self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(&Self) -> U,
    {
        let addr: usize = (*self).into();

        if addr == 0 {
            default()
        } else {
            f(self)
        }
    }
}
