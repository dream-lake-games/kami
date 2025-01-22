use crate::prelude::*;

lazy_static::lazy_static! {
    pub static ref LEVEL_DEFNS: Vec<LevelDefn> = vec![
        LevelDefn::new(
            "CAKES IN THE FOREST",
            "6dab9440-c210-11ef-ab00-79b1690c4bfe",
            vec![],
            LevelTierCutoff::new(0, 100, 200),
        ),
    ];
}

pub fn get_level_defn<S: AsRef<str>>(lid: S) -> LevelDefn {
    LEVEL_DEFNS
        .iter()
        .find(|level_defn| level_defn.lid == lid.as_ref())
        .cloned()
        .unwrap()
}
