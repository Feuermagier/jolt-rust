use std::marker::PhantomData;

use joltc_sys::*;

use crate::BodyId;

/// See also: Jolt's [`Body`](https://secondhalfgames.github.io/jolt-docs/5.0.0/class_body.html) class.
pub struct Body<'interface> {
    inner: *mut JPC_Body,
    _phantom: PhantomData<&'interface ()>,
}

impl<'interface> Body<'interface> {
    pub(crate) fn new(inner: *mut JPC_Body) -> Self {
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    pub fn id(&self) -> BodyId {
        let raw = unsafe { JPC_Body_GetID(self.inner) };
        BodyId::new(raw)
    }

    pub fn as_raw(&self) -> *mut JPC_Body {
        self.inner
    }
}
