use crate::prelude::*;

lazy_static::lazy_static! {
    pub static ref LEVEL_DEFNS: Vec<LevelDefn> = vec![
        // FIRST ROW OF SIX
        LevelDefn::new(
            "FIRST FLIGHT",
            "6dab9440-c210-11ef-ab00-79b1690c4bfe",
            vec![
                LevelIntroMessage::new("CHEFS NEEDED", "The mountains have been overtaken by wild cakes! Brave volunteers needed to address the threat."),
                LevelIntroMessage::new("IDEAL CANDIDATE", "Skills in flying and dive-bombing requested."),
                LevelIntroMessage::new("ADDED BONUS", "Cake!"),
                LevelIntroMessage::new("YOUR MISSION", "Collect acorns, land on the cake."),
                LevelIntroMessage::new("CONTROLS", "Hold space (or left mouse) to launch. Hold or tap again to adjust flight."),
            ],
            LevelTierCutoff::new(100, 250, 450),
        ),
        LevelDefn::new(
            "FIRST CHOICE",
            "cdadbba0-c210-11ef-a5df-bd1eec6b98a9",
            vec![
                LevelIntroMessage::new("USE YOUR BRAIN", "Not all cakes are created equal."),
                LevelIntroMessage::new("HIGH VALUE TARGETS", "Cakes with more layers of frosting are worth more points."),
            ],
            LevelTierCutoff::new(300, 550, 750),
        ),
        LevelDefn::new(
            "OVER AND UNDER",
            "cf4e7350-c210-11ef-a5df-9fddf15971d9",
            vec![
                LevelIntroMessage::new("LIMITS", "Holding launch too long does bad things. Don't do that."),
            ],
            LevelTierCutoff::new(450, 850, 1250),
        ),
        LevelDefn::new(
            "SLIIIIIDE",
            "d1400a20-c210-11ef-a5df-2de2024860bb",
            vec![
                LevelIntroMessage::new("SLIPPERY DIPPERY", "Some ground is slippery. Use this to crush your enemies (cake)."),
            ],
            LevelTierCutoff::new(100, 200, 400),
        ),
        LevelDefn::new(
            "SPONGECAKE? YUCK",
            "d6e903a0-c210-11ef-a5df-b14a282515ea",
            vec![
                LevelIntroMessage::new("AVOID", "Some cakes are friends. Do NOT land on spotted cake."),

            ],
            LevelTierCutoff::new(0, 250, 550),
        ),
        LevelDefn::new(
            "FIRST LOOP",
            "d90776d0-c210-11ef-a5df-5b73bfd4e193",
            vec![],
            LevelTierCutoff::new(200, 400, 600),
        ),
        // SECOND ROW OF SIXZ
        LevelDefn::new(
            "SCATTERED NUTS",
            "c796e400-c210-11ef-a5df-1d1d99b5bc99",
            vec![],
            LevelTierCutoff::new(500, 800, 1100),
        ),
        LevelDefn::new(
            "SOFT LANDING",
            "c9db07a0-c210-11ef-a5df-9b2d1b25ffd8",
            vec![],
            LevelTierCutoff::new(400, 650, 900),
        ),
        LevelDefn::new(
            "SPELUNK",
            "cbea3890-c210-11ef-a5df-ab0d33c4076d",
            vec![],
            LevelTierCutoff::new(100, 250, 400),
        ),
        LevelDefn::new(
            "CAKES IN THE FOREST",
            "cdaa5f20-c210-11ef-a5df-3169b19bccd5",
            vec![],
            LevelTierCutoff::new(0, 100, 200),
        ),
        LevelDefn::new(
            "CAKES IN THE FOREST",
            "cf5046f0-c210-11ef-a5df-21b8dbce8d91",
            vec![],
            LevelTierCutoff::new(0, 100, 200),
        ),
        LevelDefn::new(
            "CAKES IN THE FOREST",
            "d148e2a0-c210-11ef-a5df-3114ad4182c0",
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
