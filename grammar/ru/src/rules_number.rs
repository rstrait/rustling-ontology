use std::f64;

use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;

pub fn rules_numbers(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_3("intersect (with and)",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             b.reg(r#"и"#)?,
             number_check!(),
             |a, _, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_2("intersect",
             number_check!(|number: &NumberValue| number.grain().unwrap_or(0) > 1),
             number_check!(),
             |a, b| helpers::compose_numbers(&a.value(), &b.value()));
    b.rule_1_terminal("integer 0",
                      b.reg(r#"ноль"#)?,
                      |_| IntegerValue::new_with_grain(0, 1)
    );
    b.rule_1_terminal("integer 1",
                      b.reg(r#"од(?:ин|на|ной|ного|ному)"#)?,
                      |_| IntegerValue::new_with_grain(1, 1)
    );
    b.rule_1_terminal("integer 2",
                      b.reg(r#"дв(?:а|е|ух|умя|ум|оим)"#)?,
                      |_| IntegerValue::new_with_grain(2, 1)
    );
    b.rule_1_terminal("integer 3",
                      b.reg(r#"тр(?:и|[ёе][хм]|емя|оим)"#)?,
                      |_| IntegerValue::new_with_grain(3, 1)
    );
    b.rule_1_terminal("integer 4",
                      b.reg(r#"чет(?:ыре|ыр[ёе][мх]|верым)"#)?,
                      |_| IntegerValue::new_with_grain(4, 1)
    );
    b.rule_1_terminal("integer 5",
                      b.reg(r#"пят(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(5, 1)
    );
    b.rule_1_terminal("integer 6",
                      b.reg(r#"шест(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(6, 1)
    );
    b.rule_1_terminal("integer 7",
                      b.reg(r#"сем(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(7, 1)
    );
    b.rule_1_terminal("integer 8",
                      b.reg(r#"восем(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(8, 1)
    );
    b.rule_1_terminal("integer 9",
                      b.reg(r#"девят(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(9, 1)
    );
    b.rule_1_terminal("integer 10",
                      b.reg(r#"десят(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(10, 1)
    );
    b.rule_1_terminal("integer 11",
                      b.reg(r#"одиннадцат(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(11, 1)
    );
    b.rule_1_terminal("integer 12",
                      b.reg(r#"двенадцат(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(12, 1)
    );
    b.rule_1_terminal("integer 13",
                      b.reg(r#"тринадцат(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(13, 1)
    );
    b.rule_1_terminal("integer 14",
                      b.reg(r#"четырнадцат(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(14, 1)
    );
    b.rule_1_terminal("integer 15",
                      b.reg(r#"пятнадцат(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(15, 1)
    );
    b.rule_1_terminal("integer 16",
                      b.reg(r#"шестнадцат(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(16, 1)
    );
    b.rule_1_terminal("integer 17",
                      b.reg(r#"семнадцат(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(17, 1)
    );
    b.rule_1_terminal("integer 18",
                      b.reg(r#"восемнадцат(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(18, 1)
    );
    b.rule_1_terminal("integer 19",
                      b.reg(r#"девятнадцат(?:[ьи]|ерых|ью|ерым|ерых)"#)?,
                      |_| IntegerValue::new_with_grain(19, 1)
    );
    b.rule_1_terminal("a pair",
                      b.reg(r#"пар[ауые]"#)?,
                      |_| IntegerValue::new_with_grain(2, 1)
    );
    b.rule_1_terminal("several",
                      b.reg(r#"несколько"#)?,
                      |_| IntegerValue::new_with_grain(4, 1)
    );
    // TODO: Add more forms
    b.rule_1_terminal("integer 10",
        b.reg(r#"десят[ьи]"#)?,
        |_| IntegerValue::new_with_grain(10, 1),
    );
    b.rule_1_terminal("integer 20",
        b.reg(r#"двадцат[ьи]"#)?,
        |_| IntegerValue::new_with_grain(20, 1),
    );
    b.rule_1_terminal("integer 30",
                      b.reg(r#"тридцат[ьи]"#)?,
                      |_| IntegerValue::new_with_grain(30, 1),
    );
    b.rule_1_terminal("integer 40",
                      b.reg(r#"сорока?"#)?,
                      |_| IntegerValue::new_with_grain(40, 1),
    );
    b.rule_1_terminal("integer 50",
                      b.reg(r#"пятьдесят|пятидесяти"#)?,
                      |_| IntegerValue::new_with_grain(50, 1),
    );
    b.rule_1_terminal("integer 60",
                      b.reg(r#"шестьдесят|шестидесяти"#)?,
                      |_| IntegerValue::new_with_grain(60, 1),
    );
    b.rule_1_terminal("integer 70",
                      b.reg(r#"семьдесят|семидесяти"#)?,
                      |_| IntegerValue::new_with_grain(70, 1),
    );
    b.rule_1_terminal("integer 80",
                      b.reg(r#"восемьдесят|восьмидесяти"#)?,
                      |_| IntegerValue::new_with_grain(80, 1),
    );
    b.rule_1_terminal("integer 90",
                      b.reg(r#"девяност[оа]"#)?,
                      |_| IntegerValue::new_with_grain(90, 1),
    );
    b.rule_2("integer 21..99",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             integer_check_by_range!(1, 9),
             |a, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_3("integer 21..99",
             integer_check_by_range!(10, 90, |integer: &IntegerValue| integer.value % 10 == 0),
             b.reg(r#"-"#)?,
             integer_check_by_range!(1, 9),
             |a, _, b| IntegerValue::new(a.value().value + b.value().value));
    b.rule_1_terminal("integer (numeric)",
                      b.reg(r#"(\d{1,18})"#)?,
                      |text_match| IntegerValue::new(text_match.group(0).parse()?));
    b.rule_1_terminal("integer with thousands separator ,",
                      b.reg(r#"(\d{1,3}(,\d\d\d){1,5})"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: i64 = reformatted_string.parse()?;
                          IntegerValue::new(value)
                      });
    b.rule_1_terminal("100, 1_000, 1_000_000, 1_000_000_000",
                      b.reg(r#"(сотня|сто|тысяч|миллион|миллиард)(?:ы|ов|а)?"#)?,
                      |text_match| {
                          let (value, grain) = match text_match.group(1).as_ref() {
                              "сотня" | "сто" => (100, 2),
                              "тысяч" => (1_000, 3),
                              "миллион" => (1_000_000, 6),
                              "миллиард" => (1_000_000_000, 9),
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, grain)
                      }
    );
    b.rule_1_terminal("integer (200, ..., 900)",
                      b.reg(r#"(двести|триста|четыреста|пятьсот|шестьсот|семьсот|восемьсот|девятьсот)"#)?,
                      |text_match| {
                          let value = match text_match.group(1).as_ref() {
                              "двести" => 200,
                              "триста" => 300,
                              "четыреста" => 400,
                              "пятьсот" => 500,
                              "шестьсот" => 600,
                              "семьсот" => 700,
                              "восемьсот" => 800,
                              "девятьсот" => 900,
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          IntegerValue::new_with_grain(value, 2)});
    b.rule_2("200..900, 2_000..9_000, 2_000_000..9_000_000_000",
             integer_check_by_range!(1, 999),
             b.reg(r#"(сотн|сто|тысяч|миллион|миллиард)(?:и|я|ы|ов|а)?"#)?,
             |integer, text_match| {
                 let (value, grain) = match text_match.group(1).as_ref() {
                     "сотн" => (100, 2),
                     "сто" => (100, 2),
                     "тысяч" => (1_000, 3),
                     "миллион" => (1_000_000, 6),
                     "миллиард" => (1_000_000_000, 9),
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 IntegerValue::new_with_grain(integer.value().value * value, grain)
             }
    );
    b.rule_1_terminal("dozen",
                      b.reg(r#"дюжина"#)?,
                      |_| Ok(IntegerValue {
                          value: 12,
                          grain: Some(1),
                          group: true,
                          ..IntegerValue::default()
                      })
    );
    b.rule_2("number dozen",
             integer_check_by_range!(1, 99),
             integer_check!(|integer: &IntegerValue| integer.group),
             |a, b| {
                 Ok(IntegerValue {
                     value: a.value().value * b.value().value,
                     grain: b.value().grain,
                     group: true,
                     ..IntegerValue::default()
                 })
             });

    b.rule_1("decimal number",
             b.reg(r#"(\d*\.\d+)"#)?,
             |text_match| {
                 let value: f64 = text_match.group(0).parse()?;
                 Ok(FloatValue {
                     value: value,
                     ..FloatValue::default()
                 })
             });
    b.rule_2("<integer> and a half",
             integer_check!(),
             b.reg(r#"[си] половин(?:а|ой)"#)?,
             |integer, _| FloatValue::new(integer.value().value as f64 + 0.5)
    );
    b.rule_2("<integer> and a quarter",
             integer_check!(),
             b.reg(r#"[си] четвертью?"#)?,
             |integer, _| FloatValue::new(integer.value().value as f64 + 0.25)
    );
    b.rule_3("number dot number",
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             b.reg(r#"точка|целых"#)?,
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             |a, _, b| {
                 let value: f64 = format!("{}.{}", a.value().value, b.value().value).parse()?;
                 Ok(FloatValue {
                     value,
                     ..FloatValue::default()
                 })
             });
    b.rule_4("number dot zero... number",
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             b.reg(r#"точка|целых"#)?,
             b.reg(r#"(?:(?:ноль )*(?:ноль))"#)?,
             integer_check!(|integer: &IntegerValue| !integer.prefixed),
             |a, _, zeros, b| {
                 let zeros_string =  std::iter::repeat("0").take(zeros.group(0).split_whitespace().count()).collect::<String>();
                 let value: f64 = format!("{}.{}{}", a.value().value, zeros_string, b.value().value).parse()?;
                 Ok(FloatValue {
                     value,
                     ..FloatValue::default()
                 })

             });
    b.rule_1_terminal("decimal with thousands separator",
                      b.reg(r#"(\d+(,\d\d\d)+\.\d+)"#)?,
                      |text_match| {
                          let reformatted_string = text_match.group(1).replace(",", "");
                          let value: f64 = reformatted_string.parse()?;
                          Ok(FloatValue {
                              value: value,
                              ..FloatValue::default()
                          })
                      });
    b.rule_2("numbers prefix with -, negative or minus",
             b.reg(r#"-|минус"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * -1,
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             value: float.value * -1.0,
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             });
    b.rule_2("numbers prefix with +, positive",
             b.reg(r#"\+"#)?,
             number_check!(|number: &NumberValue| !number.prefixed()),
             |_, a| -> RuleResult<NumberValue> {
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             prefixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         FloatValue {
                             prefixed: true,
                             ..float
                         }
                             .into()
                     }
                 })
             }
    );
    b.rule_2("numbers suffixes (K, M, G)",
             number_check!(|number: &NumberValue| !number.suffixed()),
             b.reg_neg_lh(r#"([kmgкмг])"#, r#"^[^\W\$€]"#)?,
             |a, text_match| -> RuleResult<NumberValue> {
                 let multiplier = match text_match.group(0).as_ref() {
                     "k" | "к" => 1000,
                     "m" | "м" => 1000000,
                     "g" | "г" => 1000000000,
                     _ => return Err(RuleError::Invalid.into()),
                 };
                 Ok(match a.value().clone() {
                     // checked
                     NumberValue::Integer(integer) => {
                         IntegerValue {
                             value: integer.value * multiplier,
                             suffixed: true,
                             ..integer
                         }
                             .into()
                     }
                     NumberValue::Float(float) => {
                         let product = float.value * (multiplier as f64);
                         if product.floor() == product {
                             IntegerValue {
                                 value: product as i64,
                                 suffixed: true,
                                 ..IntegerValue::default()
                             }
                                 .into()
                         } else {
                             FloatValue {
                                 value: product,
                                 suffixed: true,
                                 ..float
                             }
                                 .into()
                         }
                     }
                 })
             });
    b.rule_1_terminal("ordinal 0",
                      b.reg(r#"нулев(?:ой|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(0)));
    b.rule_1_terminal("ordinal 1",
                      b.reg(r#"перв(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(1)));
    b.rule_1_terminal("ordinal 2",
                      b.reg(r#"втор(?:ой|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(2)));
    b.rule_1_terminal("ordinal 3",
                      b.reg(r#"трет(?:ий|ье|ьего)"#)?,
                      |_| Ok(OrdinalValue::new(3)));
    b.rule_1_terminal("ordinal 4",
                      b.reg(r#"четверт(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(4)));
    b.rule_1_terminal("ordinal 5",
                      b.reg(r#"пят(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(5)));
    b.rule_1_terminal("ordinal 6",
                      b.reg(r#"шест(?:ой|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(6)));
    b.rule_1_terminal("ordinal 7",
                      b.reg(r#"седьм(?:ой|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(7)));
    b.rule_1_terminal("ordinal 8",
                      b.reg(r#"восьм(?:ой|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(8)));
    b.rule_1_terminal("ordinal 9",
                      b.reg(r#"девят(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(9)));
    b.rule_1_terminal("ordinal 10",
                      b.reg(r#"деся(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(10)));
    b.rule_1_terminal("ordinal 11",
                      b.reg(r#"одиннадцат(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(11)));
    b.rule_1_terminal("ordinal 12",
                      b.reg(r#"двенадцат(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(12)));
    b.rule_1_terminal("ordinal 13",
                      b.reg(r#"тринадцат(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(13)));
    b.rule_1_terminal("ordinal 14",
                      b.reg(r#"четырнадцат(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(14)));
    b.rule_1_terminal("ordinal 15",
                      b.reg(r#"пятнадцат(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(15)));
    b.rule_1_terminal("ordinal 16",
                      b.reg(r#"шестнадцат(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(16)));
    b.rule_1_terminal("ordinal 17",
                      b.reg(r#"семнадцат(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(17)));
    b.rule_1_terminal("ordinal 18",
                      b.reg(r#"восемнадцат(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(18)));
    b.rule_1_terminal("ordinal 19",
                      b.reg(r#"девятнадцат(?:ый|ое|ого)"#)?,
                      |_| Ok(OrdinalValue::new(19)));
    b.rule_1_terminal("ordinal (100, 1_000, 1_000_000)",
                      b.reg(r#"(сотый|тычячный|миллионный|миллиардный)"#)?,
                      |text_match| {
                          let (value, grain) = match text_match.group(1).as_ref() {
                              "сотый" => (100, 2),
                              "тычячный" => (1_000, 3),
                              "миллионный" => (1_000_000, 6),
                              "миллиардный" => (1_000_000_000, 9),
                              _ => return Err(RuleError::Invalid.into()),
                          };
                          Ok(OrdinalValue::new_with_grain(value, grain))
                      }
    );
    b.rule_1_terminal("ordinal (digits)",
                      b.reg(r#"0*(\d+)[ -]?(й|ой|ий|о?е|о?го)"#)?,
                      |text_match| {
                          let value: i64 = text_match.group(1).parse()?;
                          Ok(OrdinalValue::new(value))
                      });
    Ok(())
}
