use std::f64;

use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Weekday, Grain};
use rustling_ontology_values::helpers::second;


pub fn rules_datetime(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {

    /* DATETIME - COMPLEX RULES */
    b.rule_2("intersect <datetime>",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, b| {
                 let res = a.value().intersect(b.value());
                 println!("intersect {:?}", res);
                 res
             }
    );
    b.rule_3("intersect by \",\"",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             b.reg(r#","#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |a, _, b| a.value().intersect(b.value())
    );
    /* END OF DATETIME - COMPLEX RULES */

    /* DATETIME - DATE - PREPOSITION + DATES */
    b.rule_2("in|on <date>",
             b.reg(r#"в|на"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::DayOfMonth)(datetime) || form!(Form::Celebration)(datetime)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    // TODO: add restrictions on datetime form
    b.rule_2("during <date>",
             b.reg(r#"(?:в )?течени[ие]"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("for <datetime>",
             b.reg(r#"в|на"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("in|for <named-month>",
             b.reg(r#"в|на"#)?,
             datetime_check!(form!(Form::Month(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("in|for <year>",
             b.reg(r#"в|на"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::Year(_))(datetime) && !datetime.latent),
             |_, a| Ok(a.value().clone().not_latent())
    );

    b.rule_2("on <season>",
             b.reg(r#"на"#)?,
             datetime_check!(form!(Form::Season)),
             |_, a| Ok(a.value().clone().not_latent())
    );
    /* END OF DATETIME - DATE - PREPOSITION + DATES */

    /* DATETIME - DATE - STANDALONE SINGLE GRAIN */

    b.rule_1_terminal("named-day",
                      b.reg(r#"понедельник[ау]?|пн\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Mon)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"вторнику?|вт?\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Tue)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"сред[еау]|ср\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Wed)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"четверг[у]?|чт\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"пятниц[ауеы]|пт\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Fri)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"суббот[ауе]|сб\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sat)
    );
    b.rule_1_terminal("named-day",
                      b.reg(r#"воскресень[еяю]|вс\.?"#)?,
                      |_| helpers::day_of_week(Weekday::Sun)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"январ[ьяюе]|янв\.?"#)?,
                      |_| helpers::month(1)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"феврал[ьяюе]|фев\.?"#)?,
                      |_| helpers::month(2)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"март[ауе]?"#)?,
                      |_| helpers::month(3)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"апрел[ьяюе]|апр\.?"#)?,
                      |_| helpers::month(4)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"ма[йюяе]"#)?,
                      |_| helpers::month(5)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"июн[ьяюе]"#)?,
                      |_| helpers::month(6)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"июл[ьяюе]"#)?,
                      |_| helpers::month(7)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"август[аеу]?|авг\.?"#)?,
                      |_| helpers::month(8)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"сентябр[ьяюе]|сент\.?"#)?,
                      |_| helpers::month(9)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"октябр[ьяюе]|окт\.?"#)?,
                      |_| helpers::month(10)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"ноябр[ьяюе]|нояб\.?"#)?,
                      |_| helpers::month(11)
    );
    b.rule_1_terminal("named-month",
                      b.reg(r#"декабр[ьяюе]|дек\.?"#)?,
                      |_| helpers::month(12)
    );
    // Quarters identified by an ordinal are similar to months
    b.rule_2("<ordinal> quarter",
             ordinal_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             |ordinal, _| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, &helpers::cycle_nth(Grain::Year, 0)?)
    );
    b.rule_3("<ordinal> quarter <year>",
             ordinal_check_by_range!(1, 4),
             cycle_check!(|cycle: &CycleValue| cycle.grain == Grain::Quarter),
             datetime_check!(form!(Form::Year(_))),
             |ordinal, _, datetime| helpers::cycle_nth_after(Grain::Quarter, ordinal.value().value - 1, datetime.value())
    );
    b.rule_2("Q1-4 <year>",
             b.reg(r#"([1234]|первый|второй|третий|четвертый) ?квартал"#)?,
             datetime_check!(form!(Form::Year(_))),
             |q, year| {
                 let n = match q.group(0).as_ref() {
                     "1" => 0,
                     "2" => 1,
                     "3" => 2,
                     "4" => 3,
                     "первый" => 0,
                     "второй" => 1,
                     "третий" => 2,
                     "четвертый" => 3,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 helpers::cycle_nth_after(Grain::Quarter, n, year.value())
             }
    );
    /* END OF DATETIME - DATE - STANDALONE SINGLE GRAIN */


    /* DATETIME - DATE - DEICTICS */

    b.rule_1_terminal("today",
                      b.reg(r#"сегодня"#)?,
                      |_| {
                          println!("today");
                          helpers::cycle_nth(Grain::Day, 0)
                      }
    );
    b.rule_1_terminal("tomorrow",
                      b.reg(r#"завтра"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 1)
    );
    b.rule_1_terminal("the day after tomorrow",
                      b.reg(r#"послезавтра"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 2)
    );
    b.rule_1_terminal("yesterday",
                      b.reg(r#"вчера"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -1)
    );
    b.rule_1_terminal("the day before yesterday",
                      b.reg(r#"позавчера"#)?,
                      |_| helpers::cycle_nth(Grain::Day, -2)
    );
    b.rule_1_terminal("end of week",
                      b.reg(r#"кон(?:ец|це) недели"#)?,
                      |_| helpers::day_of_week(Weekday::Thu)
                          ?.span_to(&helpers::day_of_week(Weekday::Sun)?, false)
    );
    b.rule_1_terminal("by the end of the week",
                      b.reg(r#"(?:к|до) конц[yа] недели"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)?
                          .span_to(&helpers::day_of_week(Weekday::Sun)?, true)
    );
    b.rule_1_terminal("end of day",
                      b.reg(r#"(?:к |в |на )?кон(?:ец|це|у) дня"#)?,
                      |_| helpers::hour(20, false)
    );
    b.rule_1_terminal("end of month",
                      b.reg(r#"кон(?:ец|це) месяца"#)?,
                      |_| {
                          let month = helpers::cycle_nth(Grain::Month, 1)?;
                          Ok(helpers::cycle_nth_after(Grain::Day, -10, &month)?
                              .span_to(&month, false)?
                              .latent()
                              .form(Form::PartOfMonth))
                      }
    );
    b.rule_1_terminal("by the end of month",
                      b.reg(r#"(?:к|до) конц[yа] месяца"#)?,
                      |_| helpers::cycle_nth(Grain::Day, 0)?
                          .span_to(&helpers::cycle_nth(Grain::Month, 0)?, true)
    );
    b.rule_1_terminal("end of year",
                      b.reg(r#"кон(?:ец|це) года"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let start = current_year.intersect(&helpers::month(10)?)?;
                          let end = current_year.intersect(&helpers::month(12)?)?;
                          start.span_to(&end, true)
                      }
    );
    b.rule_1_terminal("by the end of year",
                      b.reg(r#"(?:к|до) конц[yа] года"#)?,
                      |_| {
                          let current_year = helpers::cycle_nth(Grain::Year, 0)?;
                          let end = current_year.intersect(&helpers::month(12)?)?;
                          helpers::cycle_nth(Grain::Second, 0)?
                              .span_to(&end, true)
                      }
    );
    b.rule_2("this|next <day-of-week>",
             b.reg(r#"эт(?:а|от|им|у|ого|ой)|следующ(?:ий|ая|ей|его)"#)?,
             datetime_check!(form!(Form::DayOfWeek{..})),
             |_, a| a.value().the_nth_not_immediate(0)
    );
    b.rule_2("this <datetime>",
             b.reg(r#"эт(?:от|им|у|ого)|текущий"#)?,
             datetime_check!(|datetime: &DatetimeValue| !form!(Form::PartOfDay(_))(datetime) && !form!(Form::Meal)(datetime)),
             |_, a| {
                 Ok(a.value().the_nth(0)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    b.rule_2("next <datetime>",
             b.reg(r#"следующ(?:ий|ая|ей|его)"#)?,
             datetime_check!(|datetime: &DatetimeValue| !form!(Form::PartOfDay(_))(datetime) && !form!(Form::Meal)(datetime)),
             |_, a| {
                 Ok(a.value().the_nth(0)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    b.rule_2("last <datetime>",
             b.reg(r#"последн(?:ий|ая|ей|яя|ее)|прошл(?:ый|ая|ой|ое)|предыдущ(?:ий|ая|ей|ее)"#)?,
             datetime_check!(|datetime: &DatetimeValue| !form!(Form::PartOfDay(_))(datetime) && !form!(Form::Meal)(datetime)),
             |_, a| {
                 Ok(a.value().the_nth(-1)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    // TODO: add restrictions on datetime form (week days?)
    b.rule_2("<datetime> after next",
             datetime_check!(),
             b.reg(r#"после следующ(?:ий|ая|ей|его)"#)?,
             |a, _| {
                 Ok(a.value().the_nth_not_immediate(1)?
                     .form(a.value().form.clone())
                     .datetime_kind(a.value().datetime_kind.clone()))
             }
    );
    /* END OF DATETIME - DATE - DEICTICS */

    /* DATETIME - DATE - YEAR */

    b.rule_2("the year + integer 1000-2100",
             b.reg(r#"год"#)?,
             integer_check_by_range!(1000, 2100),
             |_, integer| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_2("integer 1000-2100 + year",
             integer_check_by_range!(1000, 2100),
             b.reg(r#"г(?:од)?\.?"#)?,
             |integer, _| {
                 helpers::year(integer.value().value as i32)
             }
    );
    b.rule_3("the year + composed 1900-2199",
             b.reg(r#"год"#)?,
             integer_check_by_range!(19, 21),
             integer_check_by_range!(10, 99),
             |_, a, b| {
                 let y = a.value().value * 100 + b.value().value;
                 Ok(helpers::year(y as i32)?.latent())
             }
    );
    b.rule_1("year as integer 1000-2100",
             integer_check_by_range!(1000, 2100),
             |integer| {
                 if integer.value().suffixed {
                     return Err(RuleError::Invalid.into())
                 } else {
                     helpers::year(integer.value().value as i32)
                 }
             }
    );
    b.rule_1_terminal("year as short integer 00-09",
              b.reg(r#"0(\d)"#)?,
             |integer| {
                 Ok(helpers::year(integer.group(1).parse()?)?.latent())
             }
    );
    b.rule_1("year as short integer 10-99",
             integer_check_by_range!(10, 99),
             |integer| {
                 Ok(helpers::year(integer.value().value as i32)?.latent())
             }
    );
    b.rule_2("year as integer composed 1900-2199",
             integer_check_by_range!(19, 21),
             integer_check_by_range!(10, 99),
             |a, b| {
                 let y = a.value().value * 100 + b.value().value;
                 Ok(helpers::year(y as i32)?.latent())
             }
    );
    b.rule_1("year as integer -1000-999",
             integer_check_by_range!(-1000, 999),
             |integer| {
                 if integer.value().suffixed {
                     return Err(RuleError::Invalid.into())
                 } else {
                     Ok(helpers::year(integer.value().value as i32)?.latent())
                 }
             }
    );
    b.rule_1("year as integer 2101-2200",
             integer_check_by_range!(2101, 2200),
             |integer| {
                 if integer.value().suffixed {
                     return Err(RuleError::Invalid.into())
                 } else {
                     Ok(helpers::year(integer.value().value as i32)?.latent())
                 }
             }
    );
    /* END OF DATETIME - DATE - YEAR */

    /* DATETIME - DATE - DATES */
    // TODO: list supported variants for dates
    /* Supported:
    -

    */

    b.rule_1("<day-of-month> (ordinal)",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |ordinal| {
                 Ok(helpers::day_of_month(ordinal.value().value as u32)?.latent())
             }
    );
    b.rule_2("<named-day> <day-of-month> (ordinal)",
             datetime_check!(form!(Form::DayOfWeek{..})),
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             |a, ordinal| {
                 a.value().intersect(&helpers::day_of_month(ordinal.value().value as u32)?)
             }
    );
    b.rule_2("<named-day> <month-day>",
             datetime_check!(form!(Form::DayOfWeek{..})),
             datetime_check!(form!(Form::MonthDay(_))),
             |dow, month_day| {
                 month_day.value().intersect(&dow.value())
             }
    );
    b.rule_2("<month-day> <year>",
             datetime_check!(form!(Form::MonthDay(_))),
             datetime_check!(form!(Form::Year(_))),
             |month_day, year| {
                 year.value().intersect(&month_day.value())
             }
    );
    b.rule_2("<day-of-month> (non ordinal) <named-month>",
             integer_check_by_range!(1, 31),
             datetime_check!(form!(Form::Month(_))),
             |integer, month| {
                 let m = month.value().form_month()?;
                 helpers::month_day(m, integer.value().value as u32)
             }
    );
    b.rule_2("<day-of-month> (ordinal) <named-month>",
             ordinal_check!(|ordinal: &OrdinalValue| 1 <= ordinal.value && ordinal.value <= 31),
             datetime_check!(form!(Form::Month(_))),
             |ordinal, month| {
                 let m = month.value().form_month()?;
                 helpers::month_day(m, ordinal.value().value as u32)
             }
    );
    /* END OF DATETIME - DATE - DATES */

    /* DATETIME - TIME - TIME OF DAY */

    b.rule_1("time-of-day (latent) (1 to 23)",
             integer_check_by_range!(1, 23),
             |integer| {
                 Ok(helpers::hour(integer.value().value as u32, integer.value().value <= 12)?.latent())
             }
    );
    b.rule_1("time-of-day (latent) (0)",
             integer_check_by_range!(0, 0),
             |_| Ok(helpers::hour(0, false)?.latent())
    );
    b.rule_1("time-of-day (latent) (half)",
             number_check!(|number: &NumberValue| {
                let hour = (number.value() - 0.5) as u32;
                hour as f64 == (number.value() - 0.5) && hour >= 1 && hour <= 23
            }),
             |number| {
                 let hour = number.value().value() as u32;
                 Ok(helpers::hour_minute(hour, 30, hour <= 12)?.latent())
             }
    );
    b.rule_1("time-of-day (latent) (quarter)",
             number_check!(|number: &NumberValue| {
                let hour = (number.value() - 0.25) as u32;
                hour as f64 == (number.value() - 0.25) && hour >= 1 && hour <= 23
            }),
             |number| {
                 let hour = number.value().value() as u32;
                 Ok(helpers::hour_minute(hour, 15, hour <= 12)?.latent())
             }
    );
    b.rule_2("at <time-of-day>",
             b.reg(r#"в"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().not_latent())
    );
    b.rule_2("<time-of-day> hours",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"час(?:ов|а)"#)?,
             |a, _| Ok(a.value().clone().not_latent())
    );
    b.rule_3("at <time-of-day> hours",
             b.reg(r#"в"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"час(?:ов|а)"#)?,
             |_, a, _| Ok(a.value().clone().not_latent())
    );
    // b.rule_2("<time-of-day> in the morning|afternoon",
    //          datetime_check!(form!(Form::TimeOfDay(_))),
    //          b.reg(r#"(утра|дня|вечера)"#)?,
    //          |a, text_match| {
    //              let mut value = a.value().clone();
    //              let day_period = if text_match.group(1) == "утра" {
    //                  helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?
    //              } else {
    //                  helpers::hour(12, false)?.span_to(&helpers::hour(0, false)?, false)?
    //              };
    //              println!("span: {:?}",  helpers::hour(0, false)?.span_to(&helpers::hour(12, false)?, false)?);
    //              Ok(a.value().intersect(&day_period)?.form(a.value().form.clone()))
    //          }
    // );
    b.rule_2("<time-of-day> morning",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"утра"#)?,
             |a, _| {
                 let time_of_day = a.value().form_time_of_day()?;
                 let full_hour = time_of_day.full_hour();
                 if full_hour > 12 {
                     return Ok(a.value().clone());
                 }

                 let period = helpers::hour(0, false)?
                     .span_to(&helpers::hour(12, false)?, false)?;
                 let fixed = a.value().intersect(&period)?.form(a.value().form.clone());
                 Ok(fixed)
             }
    );
    b.rule_2("<time-of-day> afternoon",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"дня|вечера"#)?,
             |a, _| {
                 let time_of_day = a.value().form_time_of_day()?;
                 let mut full_hour = time_of_day.full_hour();
                 if full_hour < 12 {
                     full_hour += 12;
                 }

                 let fixed = match time_of_day {
                     TimeOfDayForm::Hour { .. } => helpers::hour(full_hour, false),
                     TimeOfDayForm::HourMinute { minute, .. } => helpers::hour_minute(full_hour, minute, false),
                     TimeOfDayForm::HourMinuteSecond { minute, second, .. } => helpers::hour_minute_second(full_hour, minute, second, false),
                 }?;
                 Ok(fixed)
             }
    );
    b.rule_1_terminal("noon",
                      b.reg(r#"пол(?:удня|день)"#)?,
                      |_| helpers::hour(12, false)
    );
    b.rule_1_terminal("midnight",
                      b.reg(r#"полночь"#)?,
                      |_| helpers::hour(0, false)
    );
    b.rule_1_terminal("quarter (relative minutes)",
                      b.reg(r#"четверть"#)?,
                      |_| helpers::relative_minute_value(15)
    );
    b.rule_1_terminal("half (relative minutes)",
                      b.reg(r#"пол(?:овина)?"#)?,
                      |_| helpers::relative_minute_value(30)
    );
    b.rule_2("0 or o as 0 + number [1-9] (as relative minutes)",
             b.reg(r#"ноль"#)?,
             integer_check_by_range!(1, 9),
             |_, a| {
                 helpers::relative_minute_value_prefixed(a.value().value as i32)
             }
    );
    b.rule_1("number [1-59] (as relative minutes)",
             integer_check_by_range!(1, 59),
             |a| helpers::relative_minute_value(a.value().value as i32)
    );
    b.rule_3("0 or o as 0 number [1-9] minutes (as relative minutes)",
             b.reg(r#"ноль"#)?,
             integer_check_by_range!(1, 9),
             b.reg(r#"минут"#)?,
             |_, a, _| helpers::relative_minute_value_prefixed(a.value().value as i32)
    );
    b.rule_2("number [1-59] minutes (as relative minutes)",
             integer_check_by_range!(1, 59),
             b.reg(r#"минут"#)?,
             |a, _| helpers::relative_minute_value(a.value().value as i32)
    );
    b.rule_2("<hour-of-day> <integer> (as relative minutes)",
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             relative_minute_check!(),
             |datetime, relative_minutes| {
                     if relative_minutes.value().value < 10 && !relative_minutes.value().prefixed {
                         return Err(RuleError::Invalid.into())
                     } else {
                         Ok(helpers::hour_relative_minute(
                             datetime.value().form_time_of_day()?.full_hour(),
                             relative_minutes.value().value,
                             datetime.value().form.is_12_clock())?
                             .precision(datetime.value().precision))
                     }
             }
    );
    b.rule_5("at <hour-of-day> hours <integer> minutes",
             b.reg(r#"в"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             b.reg(r#"час(?:а|ов)(?: и)?"#)?,
             relative_minute_check!(),
             b.reg(r#"минут"#)?,
             |_, datetime, _, relative_minutes, _| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minutes.value().value,
                 datetime.value().form.is_12_clock())?
                 .precision(datetime.value().precision))
    );
    b.rule_3("relative minutes to|till|before <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"к|до"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |relative_minutes, _, datetime| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 -1 * relative_minutes.value().value,
                 datetime.value().form.is_12_clock())?
                 .precision(datetime.value().precision))
    );
    b.rule_3("relative minutes after|past <integer> (hour-of-day)",
             relative_minute_check!(),
             b.reg(r#"после"#)?,
             datetime_check!(form!(Form::TimeOfDay(TimeOfDayForm::Hour {.. }))),
             |relative_minutes, _, datetime| Ok(helpers::hour_relative_minute(
                 datetime.value().form_time_of_day()?.full_hour(),
                 relative_minutes.value().value,
                 datetime.value().form.is_12_clock())?.precision(datetime.value().precision))
    );
    /* END OF DATETIME - TIME - TIME OF DAY */

    /* DATETIME - TIME - TIME OF DAY - WRITTEN FORMS */
    b.rule_1_terminal("hh:mm",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.ч]([0-5]\d)"#)?,
                      |text_match| {
                          let hours: u32 = text_match.group(1).parse()?;
                          let minutes: u32 = text_match.group(2).parse()?;
                          helpers::hour_minute(hours, minutes, false)
                      }
    );
    b.rule_3_terminal("hh(:|h)mm - hh(:|h)mm (time-of-day interval)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.ч]([0-5]\d)"#)?,
                      b.reg(r#" ?\- ?"#)?,
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.ч]([0-5]\d)"#)?,
                      |a, _, b| {
                          let hour_start: u32 = a.group(1).parse()?;
                          let minute_start: u32 = a.group(2).parse()?;
                          let hour_end: u32 = b.group(1).parse()?;
                          let minute_end: u32 = b.group(2).parse()?;
                          let start = helpers::hour_minute(hour_start, minute_start, hour_start < 12)?;
                          let end = helpers::hour_minute(hour_end, minute_end, hour_end < 12)?;
                          start.smart_span_to(&end, false)
                      }
    );
    b.rule_1_terminal("hh:mm:ss",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))[:.]([0-5]\d)[:.]([0-5]\d)"#)?,
                      |text_match| {
                          let hours: u32 = text_match.group(1).parse()?;
                          let minutes: u32 = text_match.group(2).parse()?;
                          let seconds: u32 = text_match.group(3).parse()?;
                          helpers::hour_minute_second(hours, minutes, seconds, false)
                      }
    );
    b.rule_1_terminal("hhmm (military time-of-day)",
                      b.reg(r#"((?:[01]?\d)|(?:2[0-3]))([0-5]\d)"#)?,
                      |text_match| Ok(helpers::hour_minute(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          false
                      )?.latent())
    );
    /* END OF DATETIME - TIME - TIME OF DAY - WRITTEN FORMS */

    /* DATETIME - DATE - DATES - WRITTEN FORMS */
// Date - written form only
    b.rule_1_terminal("yyyy-mm-dd - ISO",
                      b.reg(r#"(\d{4})[-/](0?[1-9]|1[0-2])[-/](3[01]|[12]\d|0?[1-9])"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(3).parse()?)
    );

    /* Notice for accepted forms and resolution:
    Non ambiguous month-day combinations: 13<=d<=31 && 01<=m<=12
    Ambiguous month-day combinations: 01<=d<=12 && 01<=m<=12
    regexes:
    month and ambiguous day: (0?[1-9]|1[0-2])
    non ambiguous day: (1[3-9]|2\d|3[01])
    */
// Date - written form only
    b.rule_1_terminal("dd/mm/yy or dd/mm/yyyy - Non ambiguous cases - Non US standard",
                      b.reg(r#"(1[3-9]|2\d|3[01])[-/\.](0?[1-9]|1[0-2])[-/\.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("mm/dd/yy or mm/dd/yyyy - Non ambiguous cases - US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[-/\.](1[3-9]|2\d|3[01])[-/\.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("mm/dd/yy or mm/dd/yyyy - Ambiguous cases - interpret as US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[-/\.](0?[1-9]|1[0-2])[-/\.](\d{2,4})"#)?,
                      |text_match| helpers::year_month_day(
                          text_match.group(3).parse()?,
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("dd/mm - Non ambiguous cases - Non US standard",
                      b.reg(r#"(1[3-9]|2\d|3[01])[/\.](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(2).parse()?,
                          text_match.group(1).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("mm/dd - Non ambiguous cases - US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[/\.](3[01]|2\d|1[3-9])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
// Date - written form only
    b.rule_1_terminal("mm/dd - Ambiguous cases - interpret as US standard",
                      b.reg(r#"(0?[1-9]|1[0-2])[/\.](0?[1-9]|1[0-2])"#)?,
                      |text_match| helpers::month_day(
                          text_match.group(1).parse()?,
                          text_match.group(2).parse()?)
    );
    /* END OF DATETIME - DATE - DATES - WRITTEN FORMS */

    /* DATETIME - TIME - PARTS OF DAY */

    b.rule_1_terminal("now",
                      b.reg(r#"(?:прямо )?сейчас|немедленно|в этот момент|в это время"#)?,
                      |_| helpers::cycle_nth(Grain::Second, 0)
    );

    b.rule_1_terminal("morning",
                      b.reg(r#"утро"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(12, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("breakfast",
                      b.reg(r#"завтрак"#)?,
                      |_| Ok(helpers::hour(5, false)?
                          .span_to(&helpers::hour(10, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_1_terminal("early morning",
                      b.reg(r#"ранн(?:ее|им) утром?"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(9, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
// FIXME: Change PartOfDayForm?
    b.rule_1_terminal("before work",
                      b.reg(r#"до работы"#)?,
                      |_| {
                          Ok(helpers::hour(4, false)?
                              .span_to(&helpers::hour(9, false)?, false)?
                              .form(Form::PartOfDay(PartOfDayForm::Morning)))
                      }
    );
    b.rule_1_terminal("during work",
                      b.reg(r#"в (?:течении дня|рабочее время)"#)?,
                      |_| {
                          Ok(helpers::hour(9, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::None)))
                      }
    );
    b.rule_1_terminal("afternoon",
                      b.reg(r#"после полудня"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(19, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("evening",
                      b.reg(r#"вечер"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("night",
                      b.reg(r#"ночь"#)?,
                      |_| {
                          Ok(helpers::hour(00, false)?
                              .span_to(&helpers::hour(5, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    b.rule_1_terminal("last night",
                      b.reg(r#"прошл(?:ая|ой) ночью?|последн(?:яя|ей) ночью?"#)?,
                      |_| {
                          let yesterday = helpers::cycle_nth(Grain::Day, -1)?;
                          let night = helpers::hour(00, false)?
                              .span_to(&helpers::hour(5, false)?, false)?
                              .latent()
                              .form(Form::PartOfDay(PartOfDayForm::Evening));
                          yesterday.intersect(&night)
                      }
    );
    b.rule_1_terminal("lunch",
                      b.reg(r#"обед"#)?,
                      |_| {
                          Ok(helpers::hour(12, false)?
                              .span_to(&helpers::hour(14, false)?, false)?
                              .latent()
                              .form(Form::Meal))
                      }
    );
    b.rule_1_terminal("dinner",
                      b.reg(r#"ужин"#)?,
                      |_| Ok(helpers::hour(18, false)?
                          .span_to(&helpers::hour(23, false)?, false)?
                          .latent()
                          .form(Form::Meal))
    );
    b.rule_2("in|for|during the <part-of-day>",
             b.reg(r#"(?:в течении|в пределах|за)"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, datetime| Ok(datetime.value().clone().not_latent())
    );
    b.rule_2("this <part-of-day>",
             b.reg(r#"эт(?:от|им|у|ого)"#)?,
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |_, datetime| Ok(helpers::cycle_nth(Grain::Day, 0)?
                 .intersect(datetime.value())?
                 .form(datetime.value().form.clone())
                 .datetime_kind(DatetimeKind::DatetimeComplement { date_and_time: true, today: true }))
    );
    b.rule_1_terminal("tonight",
                      b.reg(r#"ночью"#)?,
                      |_| {
                          let period = helpers::hour(18, false)?.span_to(&helpers::hour(0, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                              .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Evening))
                              .datetime_kind(DatetimeKind::DatetimeComplement { date_and_time: true, today: true }))
                      }
    );
    b.rule_1_terminal("after lunch",
                      b.reg(r#"после обеда"#)?,
                      |_| {
                          let period = helpers::hour(13, false)?.span_to(&helpers::hour(17, false)?, false)?;
                          Ok(helpers::cycle_nth(Grain::Day, 0)?
                                 .intersect(&period)?
                              .form(Form::PartOfDay(PartOfDayForm::Afternoon)))
                      }
    );
    b.rule_1_terminal("after work - resolve as 'evening' but not latent",
                      b.reg(r#"после работы"#)?,
                      |_| {
                          Ok(helpers::hour(18, false)?
                              .span_to(&helpers::hour(0, false)?, false)?
                              .form(Form::PartOfDay(PartOfDayForm::Evening)))
                      }
    );
    /* END OF DATETIME - TIME - PARTS OF DAY */

    /* DATETIME - DATE - DATE + PARTS OF DAY */

    // TODO: Date ruletime - restrict combination of date/time forms
    b.rule_2("<datetime> <part-of-day>",
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::Year(_))(datetime) && excluding_form!(Form::Month(_))(datetime)),
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             |datetime, part_of_day| datetime.value().intersect(part_of_day.value())
    );
    // TODO: Date ruletime - restrict combination of date/time forms - but check correctness & support
    b.rule_2("<part-of-day> <datetime>",
             datetime_check!(|datetime: &DatetimeValue| form!(Form::PartOfDay(_))(datetime) || form!(Form::Meal)(datetime)),
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::Year(_))(datetime) && excluding_form!(Form::Month(_))(datetime)),
             |part_of_day, datetime| datetime.value().intersect(part_of_day.value())
    );
    // TODO: Date rule - check if supported and restrict date form to day
    b.rule_3("<datetime> before <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"до|к"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );

    /* END OF DATETIME - DATE - DATE + PARTS OF DAY */

    /* DATETIME - DATE-PERIOD - GRAINS AS DATE INTERVALS */

    b.rule_1_terminal("week-end - Hour grain, from Friday evening to Sunday midnight",
                      b.reg(r#"выходные(?: дни)"#)?,
                      |_| {
                          let friday = helpers::day_of_week(Weekday::Fri)?
                              .intersect(&helpers::hour(18, false)?)?;
                          let monday = helpers::day_of_week(Weekday::Mon)?
                              .intersect(&helpers::hour(0, false)?)?;
                          Ok(friday.span_to(&monday, false)?.datetime_kind(DatetimeKind::DatePeriod))
                      }
    );
    b.rule_1_terminal("season - summer",
                      b.reg(r#"летом?"#)?,
                      |_| Ok(helpers::month_day(6, 21)?
                          .span_to(&helpers::month_day(9, 23)?, false)?
                          .form(Form::Season))
    );
    b.rule_1_terminal("season - fall",
                      b.reg(r#"осенью?"#)?,
                      |_| Ok(helpers::month_day(9, 23)?
                          .span_to(&helpers::month_day(12, 21)?, false)?
                          .form(Form::Season))
    );
    b.rule_1_terminal("season - winter",
                      b.reg(r#"зим(?:а|ой|у)"#)?,
                      |_| Ok(helpers::month_day(12, 21)?
                          .span_to(&helpers::month_day(3, 20)?, false)?
                          .form(Form::Season))
    );
    // FIXME: Also add support for combined years? e.g. "winter 2014-2015"
    b.rule_2("season - winter <year>",
                      b.reg(r#"зим(?:ой|у)"#)?,
                      datetime_check!(form!(Form::Year(_))),
                      |_, year| Ok(helpers::year_month_day(year.value().form_year()?, 12, 21)?
                          .span_to(&helpers::year_month_day(year.value().form_year()? + (1 as i32), 3, 20)?, false)?
                          .form(Form::Season))
    );
    b.rule_1_terminal("season - spring",
                      b.reg(r#"весн(?:а|ой|у)"#)?,
                      |_| Ok(helpers::month_day(3, 20)?
                          .span_to(&helpers::month_day(6, 21)?, false)?
                          .form(Form::Season))
    );

    /* END OF DATETIME - DATE-PERIOD - GRAINS AS DATE INTERVALS */

    /* DATETIME - TIME - TIME OF DAY WITH PRECISION - UNSUPPORTED */

    b.rule_2("<time-of-day> approximately",
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"примерно"#)?,
             |datetime, _| Ok(datetime.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("about <time-of-day>",
             b.reg(r#"около|примерно|в районе"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().not_latent().precision(Precision::Approximate))
    );
    b.rule_2("exactly <time-of-day>",
             b.reg(r#"в точности|ровно|точно"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, datetime| Ok(datetime.value().clone().not_latent().precision(Precision::Exact))
    );
    /* END OF DATETIME - TIME - TIME OF DAY WITH PRECISION - UNSUPPORTED */


    /* Date and Time period need separate rules for the resolution to be adjusted to the right grain */
    /* DATETIME - DATE-PERIOD - FROM DATE INTERVALS */

    // TODO: split written / verbalized forms
    b.rule_4("<month> dd-dd (interval)",
             datetime_check!(form!(Form::Month(_))),
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             b.reg(r#"\-|по|до"#)?,
             b.reg(r#"(3[01]|[12]\d|0?[1-9])"#)?,
             |datetime, a, _, b| {
                 let start = datetime.value()
                     .intersect(&helpers::day_of_month(a.group(1).parse()?)?)?;
                 let end = datetime.value()
                     .intersect(&helpers::day_of_month(b.group(1).parse()?)?)?;
                 start.span_to(&end, true)
             }
    );
    // TODO: split written / verbalized forms
    b.rule_3("<datetime> - <datetime> (interval)",
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"\-|по|до"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |a, _, b| a.value().span_to(b.value(), true)
    );
    // TODO: split written / verbalized forms
    b.rule_4("from <datetime> - <datetime> (interval)",
             b.reg(r#"с"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"по|до"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // TODO: split written / verbalized forms
    b.rule_4("between <datetime> and <datetime> (interval)",
             b.reg(r#"между"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"и"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a, _, b| a.value().span_to(b.value(), true)
    );
    /* END OF DATETIME - DATE-PERIOD - FROM DATE INTERVALS */


    /* DATETIME - TIME-PERIOD - FROM TIME INTERVALS */

    // TODO: split written / verbalized forms
    b.rule_3("<time-of-day> - <time-of-day> (interval)",
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             b.reg(r#"\-|:|по|до"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |a, _, b| a.value().span_to(b.value(), false)
    );
    // TODO: split written / verbalized forms
    b.rule_4("from <time-of-day> - <time-of-day> (interval)",
             b.reg(r#"с"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"\-|по|до"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    // TODO: split written / verbalized forms
    b.rule_4("between <time-of-day> and <time-of-day> (interval)",
             b.reg(r#"мужду"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             b.reg(r#"и"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a, _, b| a.value().span_to(b.value(), false)
    );
    /* END OF DATETIME - TIME-PERIOD - FROM TIME INTERVALS */

    /* DATETIME - DATE AND TIME PERIODS */

    b.rule_2("from <datetime> (incl. <time-of-day>)",
             b.reg(r#"с"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().mark_after_start())
    );
    b.rule_2("by <time-of-day>",
             b.reg(r#"к"#)?,
             datetime_check!(|datetime: &DatetimeValue| !datetime.latent && form!(Form::TimeOfDay(_))(datetime)),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(a.value(), false)
    );
    // TODO: restrict datetime forms
    b.rule_2("by the end of <datetime>",
             b.reg(r#"к концу"#)?,
             datetime_check!(),
             |_, a| helpers::cycle_nth(Grain::Day, 0)?.span_to(a.value(), true)
    );
    // TODO: correct regex
    b.rule_2("until <time-of-day>",
             b.reg(r#"до"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().mark_before_end())
    );
    b.rule_2("until <datetime>",
             b.reg(r#"до"#)?,
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a| Ok(a.value().clone().mark_before_end_all())
    );
    b.rule_2("before <time-of-day>",
             b.reg(r#"до|перед"#)?,
             datetime_check!(form!(Form::TimeOfDay(_))),
             |_, a| Ok(a.value().clone().mark_before_start())
    );
    b.rule_2("before <datetime>",
             b.reg(r#"до"#)?,
             datetime_check!(|datetime: &DatetimeValue| excluding_form!(Form::TimeOfDay(_))(datetime)),
             |_, a| Ok(a.value().clone().mark_before_start())
    );
    // TODO: split date/time period + correct regex
    b.rule_2("after <time-of-day>",
             b.reg(r#"после"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().clone().mark_after_end())
    );
    // TODO: split date/time period + correct regex
    b.rule_2("since <time-of-day>",
             b.reg(r#"c(?: момента|о)?"#)?,
             datetime_check!(),
             |_, a| Ok(a.value().the_nth(-1)?.mark_after_start())
    );
    b.rule_2("about <datetime>",
             b.reg(r#"около|примерно|в районе"#)?,
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().precision(Precision::Approximate))
    );
    b.rule_2("exactly <datetime>",
             b.reg(r#"в точности|ровно|точно"#)?,
             datetime_check!(|datetime: &DatetimeValue|  !datetime.latent),
             |_, datetime| Ok(datetime.value().clone().precision(Precision::Exact))
    );
    /* END OF DATETIME - DATE AND TIME PERIODS - SPLIT TO DO */

    /* DATETIME - MISC. / UNSUPPORTED? */

    b.rule_2("absorption of , after named day",
             datetime_check!(form!(Form::DayOfWeek{..})),
             b.reg(r#","#)?,
             |a, _| Ok(a.value().clone())
    );
    Ok(())
}

pub fn rules_datetime_with_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("in <duration>",
             b.reg(r#"через"#)?,
             duration_check!(),
             |_, duration| {
                 println!("duration_1: {:?}", duration);
                 duration.value().in_present()
             }
    );
    // TODO: split date/time period
    b.rule_2("within <duration>",
             b.reg(r#"в течение"#)?,
             duration_check!(),
             |_, a| helpers::cycle_nth(Grain::Second, 0)?.span_to(&a.value().in_present()?, false)
    );

    b.rule_2("<duration> from now/today",
             duration_check!(),
             b.reg(r#"с этого момента"#)?,
             |a, _| {
                 a.value().in_present()
             }
    );

    b.rule_2("<duration> ago",
             duration_check!(),
             b.reg(r#"(?:тому )?назад"#)?,
             |a, _| {
                 a.value().ago()
             }
    );

    b.rule_3("<duration> after <datetime>",
             duration_check!(),
             b.reg(r#"после"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().after(datetime.value())
    );

    b.rule_3("<duration> before <datetime>",
             duration_check!(),
             b.reg(r#"до"#)?,
             datetime_check!(),
             |duration, _, datetime| duration.value().before(datetime.value())
    );
    Ok(())
}


pub fn rules_datetime_with_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    // b.rule_2("in <cycle>",
    //          b.reg(r#"через"#)?,
    //          cycle_check!(),
    //          |_, a| {
    //              let res = helpers::cycle_nth(a.value().grain, 1);
    //              res
    //          }
    // );
    b.rule_2("this <cycle>",
             b.reg(r#"эт(?:а|от|им|у|ого|ой)|текущий"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain.is_greater_than_day()),
             |_, a| helpers::cycle_nth(a.value().grain, 0)
    );
    b.rule_2("last <cycle>",
             b.reg(r#"последн(?:ий|ая|ей|яя)|прошл(?:ый|ая|ой)|предыдущ(?:ий|ая|ей)|прошедш(?:ая|ий|ей)"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain.is_greater_than_day()),
             |_, a| helpers::cycle_nth(a.value().grain, -1)
    );
    b.rule_2("next <cycle>",
             b.reg(r#"следующ(?:ий|ая|ей|его)"#)?,
             cycle_check!(|cycle: &CycleValue| cycle.grain.is_greater_than_day()),
             |m, a| helpers::cycle_nth(a.value().grain, 1)
    );
    b.rule_4("nth <datetime> after <datetime>",
             ordinal_check!(),
             datetime_check!(),
             b.reg(r#"после"#)?,
             datetime_check!(),
             |ordinal, a, _, b| {
                 a.value().the_nth_after(ordinal.value().value - 1, b.value())
             }
    );
    b.rule_3("<cycle> after <datetime>",
             cycle_check!(),
             b.reg(r#"после"#)?,
             datetime_check!(),
             |cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, 1, datetime.value())
    );
    b.rule_3("<cycle> before <datetime>",
             cycle_check!(),
             b.reg(r#"до|перед"#)?,
             datetime_check!(),
             |cycle, _, datetime| helpers::cycle_nth_after(cycle.value().grain, -1, datetime.value())
    );
    // TODO: resolution is not correct for times, i.e. rounds at grain
    b.rule_3("last n <cycle>",
             b.reg(r#"последн(?:ий|ая|ей|яя)|прошл(?:ый|ая|ой)|предыдущ(?:ий|ая|ей)"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, -1 * integer.value().value)
    );
    b.rule_3("next n <cycle>",
             b.reg(r#"следующ(?:ий|ая|ей|его)"#)?,
             integer_check_by_range!(1, 9999),
             cycle_check!(),
             |_, integer, cycle| helpers::cycle_n_not_immediate(cycle.value().grain, integer.value().value)
    );
    b.rule_4("<ordinal> <cycle> after <datetime>",
             ordinal_check_by_range!(1, 9999),
             cycle_check!(),
             b.reg(r#"после"#)?,
             datetime_check!(),
             |ordinal, cycle, _, datetime| helpers::cycle_nth_after_not_immediate(cycle.value().grain, ordinal.value().value - 1, datetime.value())
    );
    Ok(())
}


/* DATETIME - CYCLE DEFINITIONS */
pub fn rules_cycle(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (cycle)",
                      b.reg(r#"сек(?:унд[аыу]?)?"#)?,
                      |_| CycleValue::new(Grain::Second)
    );
    b.rule_1_terminal("minute (cycle)",
                      b.reg(r#"мин(?:ут[аыу])?"#)?,
                      |_| CycleValue::new(Grain::Minute)
    );
    b.rule_1_terminal("hour (cycle)",
                      b.reg(r#"ч(?:ас|аса|асов)?"#)?,
                      |_| CycleValue::new(Grain::Hour)
    );
    b.rule_1_terminal("day (cycle)",
                      b.reg(r#"д(?:ень|ней|ня)?"#)?,
                      |_| CycleValue::new(Grain::Day)
    );
    b.rule_1_terminal("week (cycle)",
                      b.reg(r#"недел[яьие]"#)?,
                      |_| CycleValue::new(Grain::Week)
    );
    b.rule_1_terminal("month (cycle)",
                      b.reg(r#"месяц(?:ев|[еуа])?"#)?,
                      |_| CycleValue::new(Grain::Month)
    );
    b.rule_1_terminal("quarter (cycle)",
                      b.reg(r#"квартал(?:а|ов)?"#)?,
                      |_| CycleValue::new(Grain::Quarter)
    );
    b.rule_1_terminal("year (cycle)",
                      b.reg(r#"года?|лет"#)?,
                      |_| CycleValue::new(Grain::Year)
    );
    Ok(())
}
