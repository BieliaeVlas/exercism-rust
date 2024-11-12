use time::Duration as Duration;
use time::PrimitiveDateTime as DateTime;

const ONE_BILION: i64 = 10i64.pow(9);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(ONE_BILION)
}
