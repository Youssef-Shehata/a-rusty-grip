use crate::grep::grep;
#[cfg(test)]
pub mod exact {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(grep("wd", "d"), true);
    }

    #[test]
    fn case2() {
        assert!(grep("ass", "ass"));
    }
}
#[cfg(test)]

pub mod symbols {
    use super::*;
    #[test]
    fn case3() {
        assert!(grep("2", "\\d"));
    }
    #[test]
    fn case4() {
        assert!(grep("012", "\\d\\d\\d"));
    }
}
#[cfg(test)]
pub mod groups {
    use super::*;

    #[test]
    fn case5() {
        assert_ne!(grep("w29d", "[sa]"), true);
    }
    #[test]
    fn case6() {
        assert!(grep("oopspp", "[so]"));
    }
    #[test]
    fn case7() {
        assert!(grep("019248apapopopiw23", "[^nmbv]"));
    }
    #[test]
    fn case8() {
        assert!(grep("qwe", "[sw]"));
    }
}
#[cfg(test)]
pub mod beginning_of_line {
    use super::*;
    #[test]
    fn case12() {
        assert_eq!(grep("opac", "^opa"), true);
    }
    #[test]
    fn case13() {
        assert_eq!(grep("opac", "^o"), true);
    }
    #[test]
    fn case14() {
        assert_eq!(grep("a", "^a"), true);
    }
    #[test]
    fn case18() {
        assert_ne!(grep("da", "^das"), true);
    }
    #[test]
    fn case19() {
        assert_ne!(grep("ad", "^d"), true);
    }
    #[test]
    fn case20() {
        assert_ne!(grep("1p", "^1 "), true);
    }
    #[test]
    fn case21() {
        assert_ne!(grep("daas", "^aas"), true);
    }
    #[test]
    fn case22() {
        assert_ne!(grep("slog", "^log"), true);
    }
}

#[cfg(test)]
pub mod end_of_line {
    use super::*;

    #[test]
    fn case23() {
        assert_ne!(grep("man ", "man$"), true);
    }
    #[test]
    fn case24() {
        assert!(grep("o", "o$"));
    }
    #[test]
    fn case25() {
        assert!(grep("mad man", "man$"));
    }
    #[test]
    fn case26() {
        assert!(grep("qwe  ", "  $"));
    }
}
#[cfg(test)]
pub mod plus {
    use super::*;

    #[test]
    fn case27() {
        assert_eq!(grep("man ", "ma+n"), true);
    }
    #[test]
    fn case28() {
        assert_eq!(grep("maan ", "ma+n"), true);
    }
    #[test]
    fn case29() {
        assert_ne!(grep("mn ", "ma+n"), true);
    }
    #[test]
    fn case30() {
        assert_eq!(grep("aan ", "a+n"), true);
    }
    #[test]
    fn case31() {
        assert_eq!(grep("maa ", "ma+"), true);
    }
}
pub mod question_mark {
    use super::*;

    #[test]
    fn case32() {
        assert_eq!(grep("mn ", "ma?n"), true);
    }
    #[test]
    fn case33() {
        assert_ne!(grep("maan ", "ma?n"), true);
    }
    #[test]
    fn case34() {
        assert_eq!(grep("mn ", "ma?"), true);
    }
    #[test]
    fn case35() {
        assert_eq!(grep("n", "a?n"), true);
    }
    #[test]
    fn case36() {
        assert_eq!(grep("maaasa ", "a?"), true);
    }
}

pub mod wild_card {
    use super::*;

    #[test]
    fn case37() {
        assert_eq!(grep("mapn", "ma.n"), true);
    }
    #[test]
    fn case38() {
        assert_ne!(grep("mn ", "m.n"), true);
    }
    #[test]
    fn case39() {
        assert_eq!(grep("ma ", "ma."), true);
    }
}
#[cfg(test)]
pub mod combinations {

    use super::*;
    #[test]
    fn case9() {
        assert!(grep("d2d apple", "\\w\\d\\w apple"));
    }
    #[test]
    fn case10() {
        assert!(grep("22w a", "\\d\\dw [sa]"));
    }
    #[test]
    fn case11() {
        assert_eq!(grep("opac", "[^c]"), true);
    }
    #[test]
    fn case15() {
        assert_ne!(grep("dsx", "d[pw]x"), true);
    }
    #[test]
    fn case16() {
        assert_ne!(grep("12 ds 21", "12 ds [^2]1"), true);
    }
    #[test]
    fn case17() {
        assert_ne!(grep("22w ", "\\d\\dw [^sa]"), true);
    }
    #[test]
    fn comb1() {
        assert_eq!(grep("22w w   ", "\\d\\dw [^sa].+"), true);
    }
    #[test]
    fn comb2() {
        assert_eq!(grep("22w", "\\d\\dws?"), true);
    }
    #[test]
    fn comb3() {
        assert_eq!(grep("s22w", "\\w?\\d\\d."), true);
    }
}
mod star {

    use super::*;
    #[test]
    fn star1() {
        assert_eq!(grep("w2w", "w\\d*w"), true);
    }
    #[test]
    fn star3() {
        assert_eq!(grep("w222w", "w\\d*w"), true);
    }
    #[test]
    fn star4() {
        assert_eq!(grep("ww", "w\\d*w"), true);
    }
    #[test]
    fn star5() {
        assert_eq!(grep("w", "\\d*w"), true);
    }
    #[test]
    fn star6() {
        assert_eq!(grep("2w", "\\d*w"), true);
    }
    #[test]
    fn star7() {
        assert_eq!(grep("222w", "\\d*w"), true);
    }
    #[test]
    fn star8() {
        assert_eq!(grep("w222", "w\\d*"), true);
    }
    #[test]
    fn star9() {
        assert_eq!(grep("w", "w\\d*"), true);
    }
}
