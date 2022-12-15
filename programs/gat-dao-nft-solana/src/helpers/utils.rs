pub struct Utils {}

impl Utils {
  pub fn is_happening(now: u64, start_time: u64, end_time: u64) -> bool {
    return now >= start_time && now <= end_time;
  }
}