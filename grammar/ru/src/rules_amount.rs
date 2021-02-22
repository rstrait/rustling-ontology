use rustling::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::dimension::Precision::*;
use rustling_ontology_values::helpers;

pub fn rules_percentage(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("<number> per cent",
        number_check!(),
        b.reg(r#"%|процент(?:а|ов)?|из (?:ста|100)"#)?,
        |number, _| Ok(PercentageValue(number.value().value()))
    );
    Ok(())
}

pub fn rules_finance(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_2("intersect (X cents)",
             amount_of_money_check!(),
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, b| helpers::compose_money(a.value(), b.value()));
    b.rule_3("intersect (and X cents)",
             amount_of_money_check!(),
             b.reg(r#"и"#)?,
             amount_of_money_check!(|money: &AmountOfMoneyValue| money.unit == Some("cent")),
             |a, _, b| helpers::compose_money(&a.value(), &b.value()));
    b.rule_2("intersect",
             amount_of_money_check!(),
             number_check!(),
             |a, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_3("intersect (and number)",
             amount_of_money_check!(),
             b.reg(r#"и"#)?,
             number_check!(),
             |a, _, b| helpers::compose_money_number(&a.value(), &b.value()));
    b.rule_1_terminal("$",
                      b.reg(r#"\$|долл?ар(?:а|ов)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("$") }),
    );
    b.rule_1_terminal("USD",
                      b.reg(r#"us[d\$]|американск(?:ий|их)? долл?ар(?:а|ов)?|бакс(?:a|ов)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("USD") }),
    );
    b.rule_2_terminal("dollar US",
                      b.reg(r#"долл?ар(?:а|ов)?"#)?,
                      b.reg(r#"сша"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("USD") }),
    );
    b.rule_1_terminal("AUD",
                      b.reg(r#"au[d\$]"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("AUD") })
    );
    b.rule_2_terminal("dollar AUD",
                      b.reg(r#"австралийски(?:й|х)"#)?,
                      b.reg(r#"долл?ар(?:а|ов)?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("AUD") }),
    );
    b.rule_1_terminal("CAD",
                      b.reg(r#"ca[d\$]"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CAD") })
    );
    b.rule_2_terminal("dollar CAD",
                      b.reg(r#"канадски(?:й|х)"#)?,
                      b.reg(r#"долл?ар(?:а|ов)?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("CAD") }),
    );
    b.rule_1_terminal("HKD",
                      b.reg(r#"hk[d\$]"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("HKD") })
    );
    b.rule_2_terminal("dollar HKD",
                      b.reg(r#"hk|гонконгcки(?:х|й)"#)?,
                      b.reg(r#"долл?ар(?:а|ов)?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("HKD") }),
    );
    b.rule_1_terminal("EUR",
                      b.reg(r#"€|евро"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("EUR") }),
    );
    b.rule_1_terminal("£",
                      b.reg(r#"£|фунт(?:а|ов)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("£") })
    );
    b.rule_1_terminal("GBP",
                      b.reg(r#"gbp|стерлинг(?:а|ов)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("GBP") })
    );
    b.rule_2_terminal("british GBP",
                      b.reg(r#"британски(?:й|х)"#)?,
                      b.reg(r#"фунт(?:а|ов)?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("GBP") }),
    );
    b.rule_3_terminal("british GBP sterling",
                      b.reg(r#"британски(?:й|х)"#)?,
                      b.reg(r#"фунт(?:а|ов)?"#)?,
                      b.reg(r#"стерлинг(?:а|ов)?"#)?,
                      |_, _, _| Ok(MoneyUnitValue { unit: Some("GBP") }),
    );
    b.rule_1_terminal("CHF",
                      b.reg(r#"chf"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CHF") })
    );
    b.rule_2_terminal("swiss CHF",
                      b.reg(r#"швей?царски(?:й|х)"#)?,
                      b.reg(r#"франк(?:а|ов)?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("CHF") }),
    );
    b.rule_1_terminal("KR",
                      b.reg(r#"kr|крон(?:a|ы)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KR") })
    );
    b.rule_1_terminal("DKK",
                      b.reg(r#"dkk"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("DKK") })
    );
    b.rule_2_terminal("danish DKK",
                      b.reg(r#"датск(?:ая|их)"#)?,
                      b.reg(r#"крон(?:а|ы)?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("DKK") }),
    );
    b.rule_1_terminal("NOK",
                      b.reg(r#"nok"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("NOK") })
    );
    b.rule_2_terminal("norwegian NOK",
                      b.reg(r#"норвежск(?:ая|их)"#)?,
                      b.reg(r#"крон(?:а|ы)?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("NOK") }),
    );
    b.rule_1_terminal("SEK",
                      b.reg(r#"sek"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("SEK") })
    );
    b.rule_2_terminal("swedish SEK",
                      b.reg(r#"шведск(?:ая|их)"#)?,
                      b.reg(r#"крон(?:а|ы)?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("SEK") }),
    );
    b.rule_1_terminal("RUB",
                      b.reg(r#"₽|руб(?:лей|ля)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("RUB") })
    );
    b.rule_1_terminal("INR",
                      b.reg(r#"inr|рупи(?:й|я)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("INR") })
    );
    b.rule_2_terminal("indian INR",
                      b.reg(r#"инди[ий]?ск(?:ая|их)"#)?,
                      b.reg(r#"рупи(?:й|я)?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("INR") }),
    );
    b.rule_1_terminal("JPY",
                      b.reg(r#"jpy|[ий]ена?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("JPY") })
    );
    b.rule_1_terminal("CNY",
                      b.reg(r#"cny|cnh|rmb|юан(?:ь|я|ей)"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("CNY") })
    );
    b.rule_1_terminal("¥",
                      b.reg(r#"¥"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("¥") })
    );
    b.rule_1_terminal("KRW",
                      b.reg(r#"₩|krw|вона?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("KRW") })
    );
    b.rule_2_terminal("korean KRW",
                      b.reg(r#"(?:южно[- ]?)?корейск(?:ая|их)"#)?,
                      b.reg(r#"вона?"#)?,
                      |_, _| Ok(MoneyUnitValue { unit: Some("KRW") }),
    );
    b.rule_1_terminal("฿",
                      b.reg(r#"฿|btc|битко[ий]н(?:а|ов)?"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("฿") })
    );
    b.rule_1_terminal("cent",
                      b.reg(r#"сантим(?:a|ов)?|цент(?:а|ов)?|копе(?:ек|[йи]ка)|c|¢"#)?,
                      |_| Ok(MoneyUnitValue { unit: Some("cent") })
    );
    b.rule_2("<unit> <amount>",
             money_unit!(),
             number_check!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: b.value().value(),
                     unit: a.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("<amount> <unit>",
             number_check!(),
             money_unit!(),
             |a, b| {
                 Ok(AmountOfMoneyValue {
                     value: a.value().value(),
                     unit: b.value().unit,
                     ..AmountOfMoneyValue::default()
                 })
             });
    b.rule_2("about <amount-of-money>",
             b.reg(r#"около|примерно|почти"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Approximate,
                     ..a.value().clone()
                 })
             });
    b.rule_2("exactly <amount-of-money>",
             b.reg(r#"ровно|(?:в )?точно(?:сти)?|именно"#)?,
             amount_of_money_check!(),
             |_, a| {
                 Ok(AmountOfMoneyValue {
                     precision: Exact,
                     ..a.value().clone()
                 })
             });
    Ok(())
}

pub fn rules_temperature(b: &mut RuleSetBuilder<Dimension>) -> RustlingResult<()> {
    b.rule_1("number as temp",
             number_check!(),
             |a| {
                 Ok(TemperatureValue {
                     value: a.value().value(),
                     unit: None,
                     latent: true,
                 })
             });
    b.rule_2("<latent temp> degrees",
             temperature_check!(|temp: &TemperatureValue| temp.latent),
             b.reg(r#"градуса?|°"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("degree"),
                     latent: true,
                 })
             });
    b.rule_2("<temp> Celcius",
             temperature_check!(|temp: &TemperatureValue| temp.latent),
             b.reg(r#"(?:по )?цельс[ие][яийю]|°?c"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("celsius"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Fahrenheit",
             temperature_check!(|temp: &TemperatureValue| temp.latent),
             b.reg(r#"(?:по )?фаренге[йи]т(?:[ау])?|°?f"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("fahrenheit"),
                     latent: false,
                 })
             });
    b.rule_2("<temp> Kelvin",
             temperature_check!(),
             b.reg(r#"кель?вин(?:[ау])?|°?k"#)?,
             |a, _| {
                 Ok(TemperatureValue {
                     value: a.value().value,
                     unit: Some("kelvin"),
                     latent: false,
                 })
             });
    Ok(())
}

