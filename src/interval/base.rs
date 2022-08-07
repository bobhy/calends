use super::bound;
use chrono::NaiveDate;
use std::ops::Bound;
/// Base interval
///
/// Used to coalesce both recurring and non-recurring intervals into one interface.
pub trait BaseInterval {
    fn start(&self) -> Bound<NaiveDate>;
    fn end(&self) -> Bound<NaiveDate>;

    /// Start date in the form of an option
    ///
    /// Unbounded gives us [None]
    fn start_date(&self) -> Option<NaiveDate> {
        match self.start() {
            Bound::Included(d) => Some(d),
            Bound::Excluded(d) => d.succ_opt(),
            Bound::Unbounded => None,
        }
    }

    /// End date in the form of an option
    ///
    /// Unbounded gives us [None]
    fn end_date(&self) -> Option<NaiveDate> {
        match self.end() {
            Bound::Included(d) => Some(d),
            Bound::Excluded(d) => d.pred_opt(),
            Bound::Unbounded => None,
        }
    }

    /// Determine whether a date falls within the current interval
    ///
    fn within(&self, date: NaiveDate) -> bool {
        bound::within(date, &self.start(), &self.end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Interval {
        pub start: Bound<NaiveDate>,
        pub end: Bound<NaiveDate>,
    }

    impl BaseInterval for Interval {
        fn start(&self) -> Bound<NaiveDate> {
            self.start
        }

        fn end(&self) -> Bound<NaiveDate> {
            self.end
        }
    }

    #[test]
    fn test_within() {
        let i1 = Interval {
            start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
            end: Bound::Included(NaiveDate::from_ymd(2022, 12, 31)),
        };

        assert_eq!(i1.within(NaiveDate::from_ymd(2022, 5, 18)), true);
        assert_eq!(i1.within(NaiveDate::from_ymd(2023, 5, 18)), false);
    }

    #[test]
    fn test_start_date() {
        let i1 = Interval {
            start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
            end: Bound::Included(NaiveDate::from_ymd(2022, 12, 31)),
        };

        assert_eq!(i1.start_date(), NaiveDate::from_ymd_opt(2022, 1, 1));
    }

    #[test]
    fn test_end_date() {
        let i1 = Interval {
            start: Bound::Included(NaiveDate::from_ymd(2022, 1, 1)),
            end: Bound::Included(NaiveDate::from_ymd(2022, 12, 31)),
        };

        assert_eq!(i1.end_date(), NaiveDate::from_ymd_opt(2022, 12, 31));
    }
}
