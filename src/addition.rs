use chrono::{Datelike, NaiveDate};

use crate::util;

/// Add a month duration to the current date
///
/// This function adds one month, it does not add 30 days or 31 days
///
/// # Examples
///
/// ```
/// # use chrono::NaiveDate;
/// # use dateutil::addition;
///
/// let n1 = addition::add_months_duration(NaiveDate::from_ymd(2022, 1, 1), 1);
/// assert_eq!(n1, NaiveDate::from_ymd(2022, 2, 1));
///
/// let n2 = addition::add_months_duration(NaiveDate::from_ymd(2022, 2, 3), 2);
/// assert_eq!(n2, NaiveDate::from_ymd(2022, 4, 3));
/// ```
///
/// The behavior for end of month works as follows:
///
/// ```
/// # use chrono::NaiveDate;
/// # use dateutil::addition;
///
/// assert_eq!(
///   addition::add_months_duration(NaiveDate::from_ymd(2022, 2, 28), 1),
///   NaiveDate::from_ymd(2022, 3, 31)
/// );
/// assert_eq!(
///   addition::add_months_duration(NaiveDate::from_ymd(2022, 3, 31), 1),
///   NaiveDate::from_ymd(2022, 4, 30)
/// );
/// ```
///
/// This also works across year boundaries:
///
/// ```
/// # use chrono::NaiveDate;
/// # use dateutil::addition;
///
/// let n4 = addition::add_months_duration(NaiveDate::from_ymd(2022, 2, 28), 11);
/// assert_eq!(n4, NaiveDate::from_ymd(2023, 1, 31));
/// ```
///
///
#[inline]
pub fn add_months_duration(date: NaiveDate, months_to_add: u32) -> NaiveDate {
  let mut month = date.month();
  let mut year = date.year();
  let month_delta = month + months_to_add;

  if month_delta > 12 {
    year += 1;
    month = month_delta - 12;
  } else {
    month = month_delta;
  }

  let date_end_of_month = util::month_end(date.year(), date.month());
  let day = if date_end_of_month.day() == date.day() {
    // if the current date is the last date of the month, the next month will need to be the
    // last date as well
    util::month_end(year, month).day()
  } else {
    // get the maximum of the month and clamp it to that, we cannot exceed the end of the current
    // month
    std::cmp::min(date.day(), util::month_end(year, month).day())
  };
  NaiveDate::from_ymd(year, month, day)
}

/// Add months duration to the current date
///
/// This function adds one month, it does not add 30 days or 31 days
///
/// Given the nature of the gregorian calendar we must clamp the dates so that when we add a month
/// we do not exceed the calendar days of the next month. An example of this is adding a month to
/// 1/31, in this instance we cannot use 2/31 as a valid date. 2/28 is the highest date available
/// to us.
///
/// # Examples
///
/// ```
/// # use chrono::NaiveDate;
/// # use dateutil::addition;
///
/// let n1 = addition::add_month_duration(NaiveDate::from_ymd(2022, 1, 1));
/// let n2 = addition::add_month_duration(NaiveDate::from_ymd(2022, 2, 3));
///
/// assert_eq!(n1, NaiveDate::from_ymd(2022, 2, 1));
/// assert_eq!(n2, NaiveDate::from_ymd(2022, 3, 3));
///
/// ```
#[inline]
pub fn add_month_duration(date: NaiveDate) -> NaiveDate {
  add_months_duration(date, 1)
}

/// Add a quarter to the date supplied
///
/// A quarter refers to one-fourth of a year and is typically expressed as Q1 for the first
/// quarter, etc., and can be expressed with the year, such as Q1 2021 (or Q121).
///
/// If the current date falls in the last quarter of the year, this will shift to the first quarter
/// of the next year.
///
/// # Examples
///
/// ```ignore
/// # use chrono::NaiveDate;
/// # use dateutil::addition;
///
/// assert_eq!(addition::add_quarter_duration(NaiveDate::from_ymd(2022, 1, 1)), NaiveDate::from_ymd(2022, 4, 1));
/// assert_eq!(addition::add_quarter_duration(NaiveDate::from_ymd(2022, 11, 3)), NaiveDate::from_ymd(2023, 2, 3));
///
/// ```
#[inline]
pub fn add_quarter_duration(date: NaiveDate) -> NaiveDate {
  if date.month() >= 10 {
    NaiveDate::from_ymd(date.year() + 1, 1, date.day())
  } else {
    NaiveDate::from_ymd(date.year(), date.month() + 3, date.day())
  }
}

/// Adds a year to the current date
///
/// # Examples
///
/// ```
/// # use chrono::NaiveDate;
/// # use dateutil::addition;
///
/// let n1 = addition::add_year_duration(NaiveDate::from_ymd(2022, 1, 1));
/// let n2 = addition::add_year_duration(NaiveDate::from_ymd(1584, 2, 3));
///
/// assert_eq!(n1, NaiveDate::from_ymd(2023, 1, 1));
/// assert_eq!(n2, NaiveDate::from_ymd(1585, 2, 3));
///
/// ```
#[inline]
pub fn add_year_duration(date: NaiveDate) -> NaiveDate {
  NaiveDate::from_ymd(date.year() + 1, date.month(), date.day())
}

/// Add a week
///
/// Simple enough
#[inline]
pub fn add_week_duration(date: NaiveDate) -> NaiveDate {
  date + chrono::Duration::weeks(1)
}

/// Add a biweek
///
/// Adds two weeks
#[inline]
pub fn add_biweek_duration(date: NaiveDate) -> NaiveDate {
  date + chrono::Duration::weeks(2)
}

/// Add a day
#[inline]
pub fn add_day(date: NaiveDate) -> NaiveDate {
  date + chrono::Duration::days(1)
}
