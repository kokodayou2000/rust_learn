use alloc::borrow::Cow;

#[derive(Debug,thiserror::Error)]
#[error(
    " {} , {} ",
    segment,
    illegal
)]
#[allow(missing_copy_implementations)]
pub struct InvalidPart {
    segment: String,
    illegal: String,
}

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Default,Hash)]
pub struct PathParts<'a> {
    pub(super) raw: Cow<'a, str>,
}
impl<'a> PathParts<'a> {
    
}






