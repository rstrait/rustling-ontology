use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::helpers;
use rustling_ontology_moment::{Grain, PeriodComp, Period};

pub fn rules_duration(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1_terminal("second (unit-of-duration)",
                      b.reg(r#"сек(?:унд[аыу]?)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Second))
    );
    b.rule_1_terminal("minute (unit-of-duration)",
                      b.reg(r#"мин(?:ут[аыу]?)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Minute))
    );
    b.rule_1_terminal("hour (unit-of-duration)",
                      b.reg(r#"ч(?:ас|аса|асов)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Hour))
    );
    b.rule_1_terminal("day (unit-of-duration)",
                      b.reg(r#"д(?:ень|ней|ня)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Day))
    );
    b.rule_1_terminal("week (unit-of-duration)",
                      b.reg(r#"недел[яьие]"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Week))
    );
    b.rule_1_terminal("month (unit-of-duration)",
                      b.reg(r#"месяц(?:ев|[еуа])?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Month))
    );
    b.rule_1_terminal("quarter (unit-of-duration)",
                      b.reg(r#"квартал(?:а|ов)?"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Quarter))
    );
    b.rule_1_terminal("year (unit-of-duration)",
                      b.reg(r#"года?|лет"#)?,
                      |_| Ok(UnitOfDurationValue::new(Grain::Year))
    );
    b.rule_1_terminal("quarter of an hour",
                      b.reg(r#"1/4 ?ч(?:аса|асов)?|четверть часа"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(15).into()))
    );
    b.rule_1_terminal("half an hour",
                      b.reg(r#"1/2 ?ч(?:аса|асов)?|пол[ -]?часа"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(30).into()))
    );
    b.rule_1_terminal("three-quarters of an hour",
                      b.reg(r#"3/4 ?ч(?:аса|асов)?"#)?,
                      |_| Ok(DurationValue::new(PeriodComp::minutes(45).into()))
    );
    b.rule_2("<integer> <unit-of-duration>",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_3("more <integer> <unit-of-duration>",
             b.reg(r#"еще"#)?,
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             |_, integer, uod| Ok(DurationValue::new(PeriodComp::new(uod.value().grain, integer.value().value).into()))
    );
    b.rule_2_terminal("number.number hours",
                      b.reg(r#"(\d+)\.(\d+)"#)?,
                      b.reg(r#"ч(?:ас|аса|асов)?"#)?,
                      |text_match, _| {
                          Ok(DurationValue::new(
                              PeriodComp::minutes(
                                  helpers::decimal_hour_in_minute(text_match.group(1), text_match.group(2))?
                              ).into()
                          )
                          )
                      }
    );
    b.rule_2("<integer> and a half hours",
             integer_check_by_range!(0),
             b.reg(r#"и пол[ -]часа"#)?,
             |integer, _| Ok(DurationValue::new(PeriodComp::minutes(integer.value().value * 60 + 30).into()))
    );
    b.rule_3("<integer> <unit-of-duration> and a half",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"[си] половин(?:а|ой)"#)?,
             |integer, uod, _| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> <unit-of-duration> and a quarter",
             integer_check_by_range!(0),
             unit_of_duration_check!(),
             b.reg(r#"[си] четвертью?"#)?,
             |integer, uod, _| {
                 let quarter_period: Period = uod.value().grain.quarter_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(quarter_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> and a half <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"[си] половин(?:а|ой)"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| {
                 let half_period: Period = uod.value().grain.half_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(half_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<integer> and a quarter <unit-of-duration>",
             integer_check_by_range!(0),
             b.reg(r#"[си] четвертью?"#)?,
             unit_of_duration_check!(),
             |integer, _, uod| {
                 let quarter_period: Period = uod.value().grain.quarter_period().map(|a| a.into()).unwrap_or_else(|| Period::default());
                 Ok(DurationValue::new(quarter_period + PeriodComp::new(uod.value().grain, integer.value().value)))
             }
    );
    b.rule_3("<number> h <number>",
             integer_check_by_range!(0),
             b.reg(r#"ч(?:аса|асов)?"#)?,
             integer_check_by_range!(0,59),
             |hour, _, minute| {
                 let hour_period = Period::from(PeriodComp::new(Grain::Hour, hour.value().clone().value));
                 let minute_period = Period::from(PeriodComp::new(Grain::Minute, minute.value().clone().value));
                 Ok(DurationValue::new(hour_period + minute_period))
             }
    );
    b.rule_2("a <unit-of-duration>",
             b.reg(r#"од(?:ин|н[ау])"#)?,
             unit_of_duration_check!(),
             |_, unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, 1).into()))
    );
    b.rule_1("<unit-of-duration>",
             unit_of_duration_check!(),
             |unit| Ok(DurationValue::new(PeriodComp::new(unit.value().grain, 1).into()))
    );
    b.rule_2("for <duration>",
             b.reg(r#"на"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("during <duration>",
             b.reg(r#"в течении"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().prefixed())
    );
    b.rule_2("after <duration>",
             b.reg(r#"после"#)?,
             duration_check!(),
             |_, duration| Ok(duration
                 .value()
                 .in_present()?
                 .mark_after_start())
    );
    b.rule_3("<duration> and <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             b.reg(r#"[cи]"#)?,
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, _, b| Ok(a.value() + b.value())
    );

    b.rule_2("<duration> <duration>",
             duration_check!(|duration: &DurationValue| !duration.suffixed),
             duration_check!(|duration: &DurationValue| !duration.prefixed),
             |a, b| Ok(a.value() + b.value())
    );

    b.rule_2("about <duration>",
             b.reg(r#"около|примерно|в районе"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Approximate))
    );

    b.rule_2("exactly <duration>",
             b.reg(r#"в точности|ровно|точно"#)?,
             duration_check!(),
             |_, duration| Ok(duration.value().clone().precision(Precision::Exact))
    );

    b.rule_2("<duration> exactly",
             duration_check!(),
             b.reg(r#"в точности"#)?,
             |duration, _| Ok(duration.value().clone().precision(Precision::Exact))
    );
    Ok(())
}