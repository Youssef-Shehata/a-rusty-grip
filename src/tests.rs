use crate::grep::grep_test;
#[cfg(test)]
pub mod exact {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(grep_test("wd", "d"), true);
    }

    #[test]
    fn case2() {
        assert!(grep_test("ass", "ass"));
    }
}
#[cfg(test)]

pub mod symbols {
    use super::*;
    #[test]
    fn case3() {
        assert!(grep_test("2", "\\d"));
    }
    #[test]
    fn case4() {
        assert!(grep_test("012", "\\d\\d\\d"));
    }
}
#[cfg(test)]
pub mod groups {
    use super::*;

    #[test]
    fn case5() {
        assert_ne!(grep_test("w29d", "[sa]"), true);
    }
    #[test]
    fn case6() {
        assert!(grep_test("oopspp", "[so]"));
    }
    #[test]
    fn case7() {
        assert!(grep_test("019248apapopopiw23", "[^nmbv]"));
    }
    #[test]
    fn case8() {
        assert!(grep_test("qwe", "[sw]"));
    }
}
#[cfg(test)]
pub mod beginning_of_line {
    use super::*;
    #[test]
    fn case12() {
        assert_eq!(grep_test("opac", "^opa"), true);
    }
    #[test]
    fn case13() {
        assert_eq!(grep_test("opac", "^o"), true);
    }
    #[test]
    fn case14() {
        assert_eq!(grep_test("a", "^a"), true);
    }
    #[test]
    fn case18() {
        assert_ne!(grep_test("da", "^das"), true);
    }
    #[test]
    fn case19() {
        assert_ne!(grep_test("ad", "^d"), true);
    }
    #[test]
    fn case20() {
        assert_ne!(grep_test("1p", "^1 "), true);
    }
    #[test]
    fn case21() {
        assert_ne!(grep_test("daas", "^aas"), true);
    }
    #[test]
    fn case22() {
        assert_ne!(grep_test("slog", "^log"), true);
    }
}

#[cfg(test)]
pub mod end_of_line {
    use super::*;

    #[test]
    fn case23() {
        assert_ne!(grep_test("man ", "man$"), true);
    }
    #[test]
    fn case24() {
        assert!(grep_test("o", "o$"));
    }
    #[test]
    fn case25() {
        assert!(grep_test("mad man", "man$"));
    }
    #[test]
    fn case26() {
        assert!(grep_test("qwe  ", "  $"));
    }
}
#[cfg(test)]
pub mod plus {
    use super::*;

    #[test]
    fn case27() {
        assert_eq!(grep_test("man ", "ma+n"), true);
    }
    #[test]
    fn case28() {
        assert_eq!(grep_test("maan ", "ma+n"), true);
    }
    #[test]
    fn case29() {
        assert_ne!(grep_test("mn ", "ma+n"), true);
    }
    #[test]
    fn case30() {
        assert_eq!(grep_test("aan ", "a+n"), true);
    }
    #[test]
    fn case31() {
        assert_eq!(grep_test("maa ", "ma+"), true);
    }
}
pub mod question_mark {
    use super::*;

    #[test]
    fn case32() {
        assert_eq!(grep_test("mn ", "ma?n"), true);
    }
    #[test]
    fn case33() {
        assert_ne!(grep_test("maan ", "ma?n"), true);
    }
    #[test]
    fn case34() {
        assert_eq!(grep_test("mn ", "ma?"), true);
    }
    #[test]
    fn case35() {
        assert_eq!(grep_test("n", "a?n"), true);
    }
    #[test]
    fn case36() {
        assert_eq!(grep_test("maaasa ", "a?"), true);
    }
}

pub mod wild_card {
    use super::*;

    #[test]
    fn case37() {
        assert_eq!(grep_test("mapn", "ma.n"), true);
    }
    #[test]
    fn case38() {
        assert_ne!(grep_test("mn ", "m.n"), true);
    }
    #[test]
    fn case39() {
        assert_eq!(grep_test("ma ", "ma."), true);
    }
}
#[cfg(test)]
pub mod combinations {

    use super::*;
    #[test]
    fn case9() {
        assert!(grep_test("d2d apple", "\\w\\d\\w apple"));
    }
    #[test]
    fn case10() {
        assert!(grep_test("22w a", "\\d\\dw [sa]"));
    }
    #[test]
    fn case11() {
        assert_eq!(grep_test("opac", "[^c]"), true);
    }
    #[test]
    fn case15() {
        assert_ne!(grep_test("dsx", "d[pw]x"), true);
    }
    #[test]
    fn case16() {
        assert_ne!(grep_test("12 ds 21", "12 ds [^2]1"), true);
    }
    #[test]
    fn case17() {
        assert_ne!(grep_test("22w ", "\\d\\dw [^sa]"), true);
    }
    #[test]
    fn comb1() {
        assert_eq!(grep_test("22w w   ", "\\d\\dw [^sa].+"), true);
    }
    #[test]
    fn comb2() {
        assert_eq!(grep_test("22w", "\\d\\dws?"), true);
    }
    #[test]
    fn comb3() {
        assert_eq!(grep_test("s22w", "\\w?\\d\\d."), true);
    }
}
mod star {

    use super::*;
    #[test]
    fn star1() {
        assert_eq!(grep_test("w2w", "w\\d*w"), true);
    }
    #[test]
    fn star3() {
        assert_eq!(grep_test("w222w", "w\\d*w"), true);
    }
    #[test]
    fn star4() {
        assert_eq!(grep_test("ww", "w\\d*w"), true);
    }
    #[test]
    fn star5() {
        assert_eq!(grep_test("w", "\\d*w"), true);
    }
    #[test]
    fn star6() {
        assert_eq!(grep_test("2w", "\\d*w"), true);
    }
    #[test]
    fn star7() {
        assert_eq!(grep_test("222w", "\\d*w"), true);
    }
    #[test]
    fn star8() {
        assert_eq!(grep_test("w222", "w\\d*"), true);
    }
    #[test]
    fn star9() {
        assert_eq!(grep_test("w", "w\\d*"), true);
    }
}
