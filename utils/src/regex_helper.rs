use std::str::FromStr;

use itertools::Itertools;
use lazy_regex::regex;

pub trait RegexHelper {
    /// shortcut for find_iter followed by a map to str
    fn find_iter_str<'a, 'b: 'a>(&'a self, s: &'b str) -> impl Iterator<Item = &'b str> + 'a;

    fn find_iter_parse<'a, 'b: 'a, T: FromStr>(
        &'a self,
        s: &'b str,
    ) -> impl Iterator<Item = T> + 'a;

    /// Find all matches and return them into a tuple
    /// Panics if the number of matches does not match the tuple size
    fn find_into_tuple<'a, 'b: 'a, T>(&'a self, s: &'b str) -> T
    where
        T: itertools::traits::HomogeneousTuple<Item = &'b str>;

    fn find_parse_into_tuple<'a, 'b: 'a, T, I>(&'a self, s: &'b str) -> T
    where
        I: FromStr,
        T: itertools::traits::HomogeneousTuple<Item = I>;

    /// Finds the first match and extracts its captures into a tuple
    /// Panics if the number of captures does not match the tuple size
    fn capture_into_tuple<'a, 'b: 'a, T>(&'a self, s: &'b str) -> T
    where
        T: itertools::traits::HomogeneousTuple<Item = &'b str>;

    fn capture_parse_into_tuple<'a, 'b: 'a, T, I>(&'a self, s: &'b str) -> T
    where
        I: FromStr,
        T: itertools::traits::HomogeneousTuple<Item = I>;
}

pub fn extract_numbers<T>(s: &str) -> impl Iterator<Item = T> + '_
where
    T: num::Integer + FromStr + Clone,
{
    regex!(r"-?\d+").find_iter(s).map(|s| {
        s.as_str()
            .parse::<T>()
            .unwrap_or_else(|_| panic!("Should always be able to parse this regex into a integer"))
    })
}

impl RegexHelper for regex::Regex {
    fn find_iter_str<'a, 'b: 'a>(&'a self, s: &'b str) -> impl Iterator<Item = &'b str> + 'a {
        self.find_iter(s).map(|m| m.as_str())
    }

    fn find_iter_parse<'a, 'b: 'a, I: FromStr>(
        &'a self,
        s: &'b str,
    ) -> impl Iterator<Item = I> + 'a {
        self.find_iter_str(s).map(|s| s.parse().unwrap_or_else(|_| panic!("Failed to parse {}", s)))
    }

    fn find_into_tuple<'a, 'b: 'a, T>(&'a self, s: &'b str) -> T
    where
        T: itertools::traits::HomogeneousTuple<Item = &'b str>,
    {
        self.find_iter_str(s).collect_tuple().expect("Number of matches does not equal tuple size")
    }

    fn find_parse_into_tuple<'a, 'b: 'a, T, I>(&'a self, s: &'b str) -> T
    where
        I: FromStr,
        T: itertools::traits::HomogeneousTuple<Item = I>,
    {
        self.find_iter_parse(s).collect_tuple().unwrap()
    }

    fn capture_into_tuple<'a, 'b: 'a, T>(&'a self, s: &'b str) -> T
    where
        T: itertools::traits::HomogeneousTuple<Item = &'b str>,
    {
        self.captures(s)
            .unwrap_or_else(|| panic!("No captures for {s} in {}", self.as_str()))
            .iter()
            .skip(1) // skip whole match, only return captures
            .enumerate()
            .map(|(i, c)| c.unwrap_or_else(|| panic!("No match for group {i}")).as_str())
            .collect_tuple()
            .expect("Num captures should match tuple size")
    }

    fn capture_parse_into_tuple<'a, 'b: 'a, T, I>(&'a self, s: &'b str) -> T
    where
        T: itertools::traits::HomogeneousTuple<Item = I>,
        I: FromStr,
    {
        self.captures(s)
            .unwrap_or_else(|| panic!("No captures for {s} in {}", self.as_str()))
            .iter()
            .skip(1) // skip whole match, only return captures
            .enumerate()
            .map(|(i, c)| c.unwrap_or_else(|| panic!("No match for group {i}")).as_str())
            .map(|s| s.parse::<I>().unwrap_or_else(|_| panic!("Failed to parse {}", s)))
            .collect_tuple()
            .expect("Num captures should match tuple size")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_iter() {
        itertools::assert_equal(regex!(r"\d+").find_iter_str("123 456 789"), ["123", "456", "789"]);
        itertools::assert_equal([123, 456, 789], regex!(r"\d+").find_iter_parse("123 456 789"));
    }

    #[test]
    fn numbers() {
        assert_eq!(vec![123, 456, 789], extract_numbers("123 456 789").collect_vec());
        assert_eq!(vec![-123, 456, -789], extract_numbers("-123   456 -789").collect_vec());
    }

    #[test]
    #[should_panic]
    fn capture_into_tuple_panic() {
        let (_, _, _) = regex!(r"(\w+) (\w+)").capture_into_tuple("a b");
    }

    #[test]
    fn capture_into_tuple() {
        // Example regex straight from the regex docs
        let re = regex!(r"'([^']+)'\s+\((\d{4})\)");
        let (movie, year) = re.capture_into_tuple("Not my favorite movie: 'Citizen Kane' (1941).");
        assert_eq!(movie, "Citizen Kane");
        assert_eq!(year, "1941");

        assert_eq!(
            (123_usize, 456_usize),
            regex!(r"(\d+)\s*(\d+)").capture_parse_into_tuple("  123 456")
        );
    }

    #[test]
    fn find_into_tuple() {
        assert_eq!(("hello", "world", "bye"), regex!(r"\w+").find_into_tuple("hello world !!! bye"));
        assert_eq!((1, -1, 22), regex!(r"[+-]?\d+").find_parse_into_tuple("hjkhkjh1 hhjk -1hjhhhh [[22]]"));
    }
}
