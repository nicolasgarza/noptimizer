use crate::plan::RelNodeType;

pub enum RelNodeMatcher {
    Match {
        typ: RelNodeType,
        children: Vec<RelNodeMatcher>,
    },
    Any,
}

impl RelNodeMatcher {
    pub fn match()
}
