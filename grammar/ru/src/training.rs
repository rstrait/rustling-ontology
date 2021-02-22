use rustling_ontology_values::check::*;
use rustling_ontology_moment::*;
use rustling_ontology_values::dimension::*;
use rustling_ontology_values::ResolverContext;

pub fn examples_percentage(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_percentage(0.3), "0.3%", "ноль целых три процента");
    example!(v, check_percentage(15.0), "15%", "15 %", "+15%", "пятнадцать процентов");
    example!(v, check_percentage(202.0), "202%", "202 процента");
}

pub fn examples_temperature(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_temperature(3.0, Some("degree")), "три градуса", "3 градуса", "3°", "+3°", "3 °");
    example!(v, check_temperature(32.0, Some("celsius")), "тридцать два градуса по Цельсию", "тридцать два градуса цельсия", "32°C", "32 °c");
    example!(v, check_temperature(-27.0, Some("celsius")), "минус 27 по Цельсию", "-27C", "- 27 c");
    example!(v, check_temperature(-5.0, Some("fahrenheit")), "-5 °F", "- 5°f");
    example!(v, check_temperature(168.0, Some("fahrenheit")), "168 F", "168f");
    example!(v, check_temperature(10.0, Some("kelvin")), "10 °K", "10°k");
    example!(v, check_temperature(21.0, Some("kelvin")), "21 кельвин", "21 K", "21k");
}

pub fn examples_finance(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    example!(v, check_finance(800.0, Some("$"), Precision::Exact), "800$", "восемьсот долларов", "+800$");
    example!(v, check_finance(10.0, Some("USD"), Precision::Approximate), "около десяти долларов США", "почти 10 usd");
    example!(v, check_finance(3.0, Some("AUD"), Precision::Exact), "ровно 3 австралийских доллара", "именно 3 AUD");
    example!(v, check_finance(0.0, Some("HKD"), Precision::Exact), "ноль hk долларов");
    example!(v, check_finance(125.0, Some("CAD"), Precision::Exact), "125 cad");
    example!(v, check_finance(45.0, Some("EUR"), Precision::Exact), "сорок пять евро", "45 €", "45 евро");
    example!(v, check_finance(2.0, Some("£"), Precision::Exact), "2 фунта", "два £");
    example!(v, check_finance(20.0, Some("GBP"), Precision::Exact), "двадцать британских фунтов", "20 стерлингов", "20 GBP");
    example!(v, check_finance(38.0, Some("CHF"), Precision::Exact), "38 швейцарских франков");
    example!(v, check_finance(447.0, Some("KR"), Precision::Exact), "447 крон");
    example!(v, check_finance(10000.0, Some("DKK"), Precision::Exact), "10,000 DKK", "10,000 датских кроны");
    example!(v, check_finance(100.0, Some("NOK"), Precision::Exact), "сто норвежских крон", "100 норвежских крон");
    example!(v, check_finance(2005.0, Some("SEK"), Precision::Exact), "2005 SEK", "2005 шведских крон");
    example!(v, check_finance(96.0, Some("INR"), Precision::Exact), "96 индийских рупий", "96 рупий");
    example!(v, check_finance(5.0, Some("RUB"), Precision::Exact), "пять рублей");
    example!(v, check_finance(3.0, Some("RUB"), Precision::Exact), "3 рубля");
    example!(v, check_finance(89.0, Some("JPY"), Precision::Approximate), "около 89 иен");
    example!(v, check_finance(200.0, Some("CNY"), Precision::Exact), "двести юаней");
    example!(v, check_finance(7.0, Some("KRW"), Precision::Exact), "7 вон", "7₩");
    example!(v, check_finance(3.0, Some("฿"), Precision::Exact), "3฿", "3 ฿", "три биткойна");
    example!(v, check_finance(2.05, Some("EUR"), Precision::Exact), "2 евро и 5 центов", "два евро пять сантимов", "2.05€");
    example!(v, check_finance(5.0, Some("cent"), Precision::Exact), "5 центов", "пять сантимов", "5¢", "5 копеек");
    example!(v, check_finance(1.0, Some("cent"), Precision::Exact), "один цент", "одна копейка", "1 сантим", "1 ¢");
}


// TODO: Sort out and split by datetime subtype
pub fn examples_datetime(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    let c = ResolverContext::new(Interval::starting_at(Moment(Local.ymd(2013, 2, 12).and_hms(4, 30, 0)), Grain::Second));
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 0]), "сейчас", "прямо сейчас", "в этот момент");
    example!(v, check_moment!(c, [2013, 2, 12]), "сегодня");
    example!(v, check_moment!(c, [2013, 2, 11]), "вчера");
    example!(v, check_moment!(c, [2013, 2, 13]), "завтра");
    example!(v, check_moment!(c, [2013, 2, 18]), "понедельник", "пн.", "этот понедельник");
    example!(v, check_moment!(c, [2013, 2, 18]), "18 февраля");
    example!(v, check_moment!(c, [2013, 2, 19]), "вторник");
    example!(v, check_moment!(c, [2013, 2, 14]), "четверг", "чт", "чт.");
    example!(v, check_moment!(c, [2013, 2, 15]), "пятница", "пт", "пт.");
    example!(v, check_moment!(c, [2013, 2, 16]), "суббота", "сб", "сб.");
    example!(v, check_moment!(c, [2013, 2, 17]), "воскресенье", "вс", "вс.");
    example!(v, check_moment!(c, [2013, 3, 1]), "1е марта", "первое марта");
    example!(v, check_moment!(c, [2013, 3, 3]), "3 марта");
    example!(v, check_moment!(c, [2015, 3, 3]), "3 марта 2015");
    example!(v, check_moment!(c, [2015, 3, 3]), "3/3/2015", "3/3/15", "2015-3-3", "2015-03-03");
    example!(v, check_moment!(c, [2015, 8, 31]), "31/08/2015", "31/08/15", "2015-08-31", "2015-08-31");
    example!(v, check_moment!(c, [2015, 8, 31]), "08/31/2015", "08/31/15", "2015-08-31", "2015-08-31");
    example!(v, check_moment!(c, [2013, 3, 3]), "3/3");
    example!(v, check_moment!(c, [2013, 8, 31]), "31/08");
    example!(v, check_moment!(c, [2013, 8, 31]), "08/31");
    example!(v, check_moment!(c, [2013, 2, 15]), "на 15-е");
    example!(v, check_moment!(c, [2013, 2, 15]), "15 февраля", "2/15");
    example!(v, check_moment!(c, [2013, 8, 8]), "8 августа");
    example!(v, check_moment!(c, [2014, 10]), "октябрь 2014");
    example!(v, check_moment!(c, [2015, 4, 14]), "14 апреля 2015");
    example!(v, check_moment!(c, [2013, 2, 19]), "следующий вторник");
    example!(v, check_moment!(c, [2013, 2, 22]), "пятница после следующей");
    example!(v, check_moment!(c, [2013, 3]), "следующий Март");
    example!(v, check_moment!(c, [2013, 2, 10]), "10 фев.");
    example!(v, check_moment!(c, [2013, 2, 11], Grain::Week), "эта неделя");
    example!(v, check_moment!(c, [2013, 2, 4], Grain::Week), "последняя неделя", "прошедшая неделя", "предыдущая неделя");
    example!(v, check_moment!(c, [2013, 2, 18], Grain::Week), "следующая неделя");
    example!(v, check_moment!(c, [2013, 1]), "прошлый месяц");
    example!(v, check_moment!(c, [2013, 3]), "следующий месяц");
    example!(v, check_moment!(c, [2013, 1, 1], Grain::Quarter), "текущий квартал");
    example!(v, check_moment!(c, [2013, 4, 1], Grain::Quarter), "следующий квартал");
    example!(v, check_moment!(c, [2013, 7, 1], Grain::Quarter), "третий квартал", "3й квартал");
    example!(v, check_moment!(c, [2018, 10, 1], Grain::Quarter), "4й квартал 2018");
    example!(v, check_moment!(c, [2012]), "предыдущий год");
    example!(v, check_moment!(c, [2013]), "текущий год", "этот год");
    example!(v, check_moment!(c, [2014]), "следующий год");
    example!(v, check_moment!(c, [2013, 2, 10]), "предыдущее воскресенье", "прошлое воскресенье", "последнее воскресенье");
    example!(v, check_moment!(c, [2013, 2, 5]), "предыдущий вторник");
    example!(v, check_moment!(c, [2013, 2, 13]), "следующая среда");
    example!(v, check_moment!(c, [2013, 2, 14]), "послезавтра");
    example!(v, check_moment!(c, [2013, 2, 14, 17]), "послезавтра в 5 вечера");
    example!(v, check_moment!(c, [2013, 2, 10]), "позавчера");
    example!(v, check_moment!(c, [2013, 2, 10, 8]), "позавчера 8 утра");
    example!(v, check_moment!(c, [2013, 2, 13, 3]), "в 3 утра", "3 часа утра", "в три утра");
    example!(v, check_moment!(c, [2013, 2, 13, 3, 18]), "3:18", "3:18 утра");
    example!(v, check_moment!(c, [2013, 2, 12, 11, 10]), "пятьдесят минут до полудня", "50 минут до 12");
    example!(v, check_moment_with_precision!(c, [2013, 2, 12, 11, 10], Precision::Approximate), "около 50 минут до 12", "примерно 50 минут до 12");
    example!(v, check_moment!(c, [2013, 2, 10, 6, 15]), "два дня тому назад в 6:15 утра");
    example!(v, check_moment!(c, [2013, 2, 12, 15]), "в 3 дня", "3 часа дня");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 15]), "3:15 дня");
    example!(v, check_moment!(c, [2013, 2, 12, 15, 23, 24]), "15:23:24");
    example!(v, check_moment!(c, [2013, 2, 12, 20]), "8 вечера сегодня", "восемь вечера сегодня", "8 вечера");
    // // Mixing date and time
    example!(v, check_moment!(c, [2013, 9, 20, 19, 30]), "в 7:30 вечера в Пт, 20 сентября");
    example!(v, check_moment!(c, [2013, 2, 16, 9]), "в 9 утра в субботу", "в субботу в 9 утра");
    example!(v, check_moment!(c, [2014, 7, 18, 19, 0]), "18 июля, 2014 19:00");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 30, 1]), "через секунду");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 31, 0]), "через минуту", "через одну минуту");
    example!(v, check_moment!(c, [2013, 2, 12, 4, 32, 0]), "через 2 минуты");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30, 0]), "через шестьдесят минут");
    example!(v, check_moment!(c, [2013, 2, 12, 5, 30]), "через час", "через 1ч");
    // example!(v, check_moment!(c, [2013, 2, 12, 5, 0, 0]), "in half an hour", "in 1/2h", "in 1/2 h", "in 1/2 hour");
    // example!(v, check_moment!(c, [2013, 2, 12, 7, 0, 0]), "in 2.5 hours", "in 2 and a half hours");
    // example!(v, check_moment!(c, [2013, 2, 12, 7, 30]), "in a few hours", "in few hours");
    // example!(v, check_moment!(c, [2013, 2, 13, 4, 30]), "in 24 hours", "in 24hrs", "in 24 hrs");
    // example!(v, check_moment!(c, [2013, 2, 13]), "in a day", "a day from now");
    // example!(v, check_moment!(c, [2016, 2]), "3 years from today");
    // example!(v, check_moment!(c, [2013, 2, 19]), "in 7 days");
    // example!(v, check_moment!(c, [2013, 2, 19]), "in 1 week", "in a week");
    // example!(v, check_moment!(c, [2013, 2, 5]), "7 days ago");
    // example!(v, check_moment!(c, [2013, 1, 29]), "14 days ago", "a fortnight ago");
    // example!(v, check_moment!(c, [2013, 2, 5]), "a week ago", "one week ago", "1 week ago");
    // example!(v, check_moment!(c, [2013, 1, 22]), "three weeks ago");
    // example!(v, check_moment!(c, [2012, 11, 12]), "three months ago");
    // example!(v, check_moment!(c, [2011, 2]), "two years ago");
    // example!(v, check_moment!(c, [2001]), "2001");
    // example!(v, check_moment!(c, [2013, 2, 19]), "7 days hence");
    // example!(v, check_moment!(c, [2013, 2, 26]), "14 days hence", "a fortnight hence");
    // example!(v, check_moment!(c, [2013, 2, 19]), "a week hence", "one week hence", "1 week hence");
    // example!(v, check_moment!(c, [2013, 3, 5]), "three weeks hence");
    // example!(v, check_moment!(c, [2013, 5, 12]), "three months hence");
    // example!(v, check_moment!(c, [2015, 2]), "two years hence");
    // example!(v, check_moment!(c, [2013, 12]), "one year after christmas");
    // example!(v, check_moment!(c, [2014, 3, 1], Grain::Month), "march 2014", "in march 2014", "for march 2014");
    // example!(v, check_moment!(c, [2005, 5, 1], Grain::Month), "may 2005", "in may 2005", "for may 2005");
    // example!(v, check_moment_span!(c, [2014, 6, 21], [2014, 9, 24]), "summer 2014", "in summer 2014", "for summer 2014");
    // example!(v, check_moment_span!(c, [2014, 12, 21], [2015, 3, 21]), "winter 2014", "in winter 2014", "for winter 2014");
    // example!(v, check_moment_span!(c, [2013, 6, 21], [2013, 9, 24]), "this summer", "current summer");
    // example!(v, check_moment_span!(c, [2012, 12, 21], [2013, 3, 21]), "this winter");
    // example!(v, check_moment!(c, [2013, 12, 25]), "xmas", "christmas", "christmas day");
    // example!(v, check_moment!(c, [2013, 12, 31]), "new year's eve", "new years eve");
    // example!(v, check_moment!(c, [2014, 1, 1]), "new year's day", "new years day");
    // example!(v, check_moment!(c, [2013, 2, 14]), "valentine's day", "valentine day");
    // example!(v, check_moment!(c, [2013, 5, 27]), "memorial day");
    // example!(v, check_moment!(c, [2013, 5, 12]), "Mother's Day");
    // example!(v, check_moment!(c, [2013, 6, 16]), "Father's Day");
    // example!(v, check_moment_span!(c, [2013, 5, 24, 18], [2013, 5, 28, 0]), "memorial day week-end");
    // example!(v, check_moment!(c, [2013, 7, 4]), "independence day", "4th of July", "4 of july");
    // example!(v, check_moment!(c, [2013, 9, 2]), "labor day");
    // example!(v, check_moment_span!(c, [2013, 8, 30, 18], [2013, 9, 3, 0]), "labor day weekend");
    // example!(v, check_moment!(c, [2013, 10, 31]), "halloween");
    // example!(v, check_moment!(c, [2013, 11, 28]), "thanksgiving day", "thanksgiving");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 18], [2013, 2, 13, 0]), "this evening", "tonight");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 4, 29, 58], [2013, 2, 12, 4, 30, 00]), "last 2 seconds", "last two seconds");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 01], [2013, 2, 12, 4, 30, 04]), "next 3 seconds", "next three seconds");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 4, 28], [2013, 2, 12, 4, 30]), "last 2 minutes", "last two minutes");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 4, 31], [2013, 2, 12, 4, 34]), "next 3 minutes", "next three minutes");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 3], [2013, 2, 12, 4]), "last 1 hour", "last 1 hr", "last one hour");
    // example!(v, check_moment_span!(c, [2013, 2, 11, 4], [2013, 2, 12, 4]), "last 24 hours", "last twenty four hours", "last twenty four hrs", "last 24 hrs", "last 24hrs");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 5], [2013, 2, 12, 8]), "next 3 hours", "next three hours");
    // example!(v, check_moment_span!(c, [2013, 2, 10], [2013, 2, 12]), "last 2 days", "last two days", "past 2 days");
    // example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "next 3 days", "next three days");
    // example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 16]), "next few days");
    // example!(v, check_moment_span!(c, [2013, 1, 28], [2013, 2, 11], Grain::Week), "last 2 weeks", "last two weeks", "past 2 weeks");
    // example!(v, check_moment_span!(c, [2013, 2, 18], [2013, 3, 11], Grain::Week), "next 3 weeks", "next three weeks");
    // example!(v, check_moment_span!(c, [2012, 12], [2013, 02]), "last 2 months", "last two months");
    // example!(v, check_moment_span!(c, [2013, 3], [2013, 6]), "next 3 months", "next three months");
    // example!(v, check_moment_span!(c, [2011], [2013]), "last 2 years", "last two years");
    // example!(v, check_moment_span!(c, [2014], [2017]), "next 3 years", "next three years");
    // example!(v, check_moment_span!(c, [2013, 7, 13], [2013, 7, 16]), "July 13-15", "July 13 to 15", "July 13 thru 15", "July 13 through 15", "July 13 - July 15", "from July 13 to July 15");
    // example!(v, check_moment_span!(c, [2013, 8, 8], [2013, 8, 13]), "Aug 8 - Aug 12");
    // example!(v, check_moment_span!(c, [2008, 8, 8], [2008, 12, 13]), "Aug 8 to december 12 2008");
    // example!(v, check_moment_span!(c, [2013, 1, 8], [2013, 12, 13]), "jan 8 to december 12 2013");
    // example!(v, check_moment_span!(c, [2019, 1, 8], [2019, 12, 13]), "jan 8 to december 12 2019"); // this helps correct resolution of year in such intervals, for year current and +
    // example!(v, check_moment_span!(c, [2013, 2, 12, 9, 30], [2013, 2, 12, 11, 0]), "9:30 - 11:00");
    // example!(v, check_moment_span!(c, [2013, 2, 14, 9, 30], [2013, 2, 14, 11, 0]), "from 9:30 - 11:00 on Thursday", "between 9:30 and 11:00 on thursday", "9:30 - 11:00 on Thursday", "later than 9:30 but before 11:00 on Thursday", "Thursday from 9:30 to 11:00","from 9:30 untill 11:00 on thursday", "Thursday from 9:30 untill 11:00", "9:30 till 11:00 on Thursday");
    // example!(v, check_moment_span!(c, [2013, 2, 14, 9], [2013, 2, 14, 11]), "Thursday from 9a to 11a");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 11, 30], [2013, 2, 12, 13, 30]), "11:30-1:30");
    // example!(v, check_moment!(c, [2013, 9, 21, 13, 30]), "1:30 PM on Sat, Sep 21");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 26]), "within 2 weeks");
    // example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14, 0], Direction::After), "from 2:00pm");
    // example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14, 0], Direction::Before), "until 2:00pm", "through 2:00pm");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 14]), "by 2:00pm");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 4, 30, 0], [2013, 2, 12, 20]), "by EOD");
    // example!(v, check_moment_span!(c, [2013, 2, 12], [2013, 3, 1, 0]), "by EOM");
    // example!(v, check_moment_span!(c, [2013, 2, 12], [2013, 4, 1, 0]), "by the end of next month");
    // example!(v, check_moment!(c, [2013, 2, 12, 14]), "today at 2pm", "at 2pm");
    // example!(v, check_moment!(c, [2013, 4, 25, 16, 0]), "4/25 at 4:00pm");
    // example!(v, check_moment!(c, [2013, 2, 13, 15]), "3pm tomorrow");
    // example!(v, check_moment_with_direction!(c, [2013, 2, 12, 14], Direction::After), "after 2 pm");
    // example!(v, check_moment_with_direction!(c, [2013, 2, 17], Direction::After), "after 5 days");
    // example!(v, check_moment_with_direction!(c, [2013, 2, 12, 11], Direction::Before), "before 11 am");
    // example!(v, check_moment_span!(c, [2013, 2, 13], [2013, 2, 13, 11]), "tomorrow before 11 am", "13th feb. 2013 until 11am");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 12], [2013, 2, 12, 19]), "in the afternoon");
    // example!(v, check_moment!(c, [2013, 2, 12, 13, 30]), "at 1:30pm", "1:30pm", "at thirteen thirty");
    // example!(v, check_moment!(c, [2013, 2, 12, 4, 45, 0]), "in 15 minutes");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 13], [2013, 2, 12, 17]), "after lunch");
    // example!(v, check_moment!(c, [2013, 2, 12, 10, 30]), "10:30");
    // example!(v, check_moment_span!(c, [2013, 2, 12, 4], [2013, 2, 12, 12]), "morning");
    // example!(v, check_moment!(c, [2013, 2, 18]), "next monday");
    // example!(v, check_moment!(c, [2013, 2, 12, 12]), "at 12pm", "at noon");
    // example!(v, check_moment!(c, [2013, 2, 13, 0]), "at 12am", "at midnight");
    // example!(v, check_moment!(c, [2013, 3]), "March", "in March");
    // example!(v, check_moment!(c, [2016, 12, 15]), "12.15.2016", "12.15.16");
    // example!(v, check_moment!(c, [2017, 05, 10]), "wednesday the 10th of may");
    // example!(v, check_moment!(c, [2013, 2, 12, 9, 9]), "at nine o nine", "at nine o nine am", "at nine o nine in the morning");
    // example!(v, check_moment!(c, [2013, 2, 12, 8, 25]), "at eight twenty-five", "at eight twenty-five am", "at eight twenty-five in the morning");
    // example!(v, check_moment_span!(c, [2017, 05, 12, 10, 32], [2017, 06, 7, 18, 23]), "friday the 12th of may, 10:32 am to wednesday the 7th of june, 6:22 pm");
}

pub fn examples_durations(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    // example!(v, check_duration!([0, 0, 0, 0, 2]), "during two hours", "for 2 hours");
    // example!(v, check_duration!([0, 0, 0, 1], Precision::Approximate), "about one day", "approximately 1 day");
    // example!(v, check_duration!([0, 2, 0]), "during two months", "for 2 months");
    // example!(v, check_duration!([1]), "during a year");
    // example!(v, check_duration!([0, 0, 0, 0, 0, 1, 3]), "during one minute and three seconds", "for 1 minute and 3 seconds", "for 1min3sec");
    // example!(v, check_duration!([0, 0, 0, 0, 7, 15], Precision::Approximate), "during about seven hours and a quarter");
    // example!(v, check_duration!([0, 0, 0, 0, 3, 30], Precision::Approximate), "about three and a half hours", "around 3 hours and a half");
    // example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "about one hour and a half");
    // example!(v, check_duration!([0, 0, 0, 0, 1, 30], Precision::Approximate), "about 1h30", "for around one hour and thirty minutes");
    // example!(v, check_duration!([0, 0, 0, 0, 0, 15], Precision::Approximate), "during about a quarter of an hour", "for approximately 1/4hour", "around a quarter-hour");
    // example!(v, check_duration!([0, 0, 0, 0, 0, 45]), "for three-quarters of an hour", "for 3/4h", "for 3/4 h", "for 3/4 hour");
    // example!(v, check_duration!([0, 0, 0, 0, 1]), "during one hour", "for 1h");
    // example!(v, check_duration!([0, 0, 2]), "for 2 weeks");
    // example!(v, check_duration!([0, 0, 0, 2], Precision::Approximate), "around two days");
}

pub fn examples_numbers(v: &mut Vec<::rustling::train::Example<Dimension>>) {
    // example!(v, check_integer(0), "0", "ноль");
    // example!(v, check_integer(1), "1", "+1", "один");
    // example!(v, check_integer(2), "2", "два", "пара");
    // example!(v, check_integer(33), "33", "тридцать три", "0033");
    // example!(v, check_integer(14), "14", "четырнадцать");
    // example!(v, check_integer(16), "16", "шестнадцать");
    // example!(v, check_integer(17), "17", "семнадцать");
    // example!(v, check_integer(18), "18", "восемнадцать");
    // example!(v, check_float(1.1), "1.1", "1.10", "01.10");
    // example!(v, check_float(0.3), "0.3", "0.30");
    // example!(v, check_float(0.5), "0.5", "0.50");
    // example!(v, check_float(0.05), "0.05");
    // example!(v, check_float(32.75), "32.75");
    // example!(v, check_float(10.08), "10.08");
    // example!(v,
    //          check_integer(100000),
    //          "100,000",
    //          "100000",
    //          "100K",
    //          "100k",
    //          "100к");
    // example!(v,
    //          check_integer(3000000),
    //          "3M",
    //          "3М",
    //          "3000K",
    //          "3000К",
    //          "3000000",
    //          "3,000,000");
    // example!(v,
    //          check_integer(1200000),
    //          "1,200,000",
    //          "1200000",
    //          "1.2M",
    //          "1.2М",
    //          "1200K",
    //          ".0012G");
    // example!(v,
    //          check_integer(-1200000),
    //          "- 1,200,000",
    //          "-1200000",
    //          "минус 1,200,000",
    //          "-1.2M",
    //          "-1200K",
    //          "-.0012G");
    // example!(v, check_integer(5000), "5 тысяч", "пять тысяч");
    // example!(v, check_integer(200000), "двести тысяч");
    // example!(v, check_ordinal(4), "4й", "4-й", "четвертый");
    // example!(v, check_ordinal(3), "3й", "3-й", "третий");
    // example!(v, check_ordinal(2), "2й", "2 й", "второй");
    // // TODO: example!(v, check_ordinal(21), "двадцать первый");
}
