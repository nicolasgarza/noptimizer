use crate::plan::*;
use std::sync::Arc;

pub fn join_commute(node: Arc<RelNode>) -> Option<Arc<RelNode>> {
    if let Some(a) = Join::try_from_relnode(node) {
        return Some(join(a.right().clone(), a.left().clone(), a.cond().clone()).into());
    }
    None
}

pub fn join_assoc(node: Arc<RelNode>) -> Option<Arc<RelNode>> {
    if let Some(a) = Join::try_from_relnode(node) {
        if let Some(b) = Join::try_from_relnode(a.left()) {
            return Some(
                join(
                    b.left().clone(),
                    join(b.right().clone(), a.right().clone(), a.cond().clone()),
                    b.cond().clone(),
                )
                .into(),
            );
        }
    }

    None
}
