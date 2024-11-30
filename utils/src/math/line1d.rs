#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line1D {
    start: i64,
    end: i64,
}

impl Line1D {
    pub fn new<T, U>((start, end): (T, U)) -> Self
    where
        T: Into<i64>,
        U: Into<i64>,
    {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }

    pub fn contains(&self, point: impl Into<i64>) -> bool {
        let point = point.into();
        self.start <= point && point <= self.end
    }

    pub fn contains_line(&self, line: &Self) -> bool {
        self.contains(line.start) && self.contains(line.end)
    }

    /// Including the case where the lines share a point
    pub fn overlaps(&self, line: &Self) -> bool {
        self.contains(line.start) || self.contains(line.end)
    }

    pub fn line_from_overlap(&self, line: &Self) -> Option<Self> {
        if !self.overlaps(line) {
            return None;
        }

        let start = self.start.max(line.start);
        let end = self.end.min(line.end);
        Some(Self::new((start, end)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let line = Line1D::new((-5, 10));
        assert!(line.contains(-5));
        assert!(line.contains(0));
        assert!(line.contains(10));
        assert!(!line.contains(-6));
        assert!(!line.contains(11));
    }

    #[test]
    fn test_contains_line() {
        let line = Line1D::new((-5, 10));
        assert!(line.contains_line(&Line1D::new((-5, 10))));
        assert!(line.contains_line(&Line1D::new((-5, 5))));
        assert!(line.contains_line(&Line1D::new((5, 10))));
        assert!(!line.contains_line(&Line1D::new((-6, 10))));
        assert!(!line.contains_line(&Line1D::new((-5, 11))));
    }

    #[test]
    fn test_overlaps() {
        let line = Line1D::new((-5, 10));
        assert!(line.overlaps(&Line1D::new((-5, 10))));
        assert!(line.overlaps(&Line1D::new((-5, 5))));
        assert!(line.overlaps(&Line1D::new((5, 10))));
        assert!(line.overlaps(&Line1D::new((-6, 10))));
        assert!(line.overlaps(&Line1D::new((-5, 11))));
        assert!(!line.overlaps(&Line1D::new((-7, -6))));
        assert!(!line.overlaps(&Line1D::new((11, 14))));
    }

    #[test]
    fn test_line_from_overlap() {
        let line = Line1D::new((-5, 10));
        assert_eq!(
            line.line_from_overlap(&Line1D::new((-5, 10))),
            Some(Line1D::new((-5, 10)))
        );
        assert_eq!(
            line.line_from_overlap(&Line1D::new((-5, 5))),
            Some(Line1D::new((-5, 5)))
        );
        assert_eq!(
            line.line_from_overlap(&Line1D::new((5, 10))),
            Some(Line1D::new((5, 10)))
        );
        assert_eq!(
            line.line_from_overlap(&Line1D::new((-6, 10))),
            Some(Line1D::new((-5, 10)))
        );
        assert_eq!(
            line.line_from_overlap(&Line1D::new((-5, 11))),
            Some(Line1D::new((-5, 10)))
        );
        assert_eq!(
            line.line_from_overlap(&Line1D::new((-3, 11))),
            Some(Line1D::new((-3, 10)))
        );
        assert_eq!(line.line_from_overlap(&Line1D::new((-7, -6))), None);
        assert_eq!(line.line_from_overlap(&Line1D::new((11, 14))), None);
    }
}
