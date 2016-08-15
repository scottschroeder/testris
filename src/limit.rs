#[derive(Debug, PartialEq)]
enum LimiterState {
    Off,
    First,
    Repeat,
}

#[derive(Debug)]
pub struct RateLimiter {
    time: f64,
    state: LimiterState,
    repeat_delay: Option<f64>,
    repeat_rate: f64,
}

impl RateLimiter {
    pub fn new(rate: f64, delay: Option<f64>) -> Self {
        if let Some(d) = delay {
            if d < 0f64 {
                panic!("Cannot wait a negative number. Delay={}", d)
            }
        }
        if rate < 0f64 {
            panic!("Cannot wait a negative number. Rate={}", rate)
        }

        RateLimiter {
            time: 0f64,
            repeat_delay: delay,
            repeat_rate: rate,
            state: LimiterState::Off,
        }
    }

    pub fn elapsed(&mut self, dt: f64) {
        if self.state == LimiterState::Off {
            return;
        }
        self.time += dt;
    }

    pub fn reset(&mut self) {
        self.state = LimiterState::Off;
    }

    pub fn get_event(&mut self) -> Option<()> {
        match self.is_ready() {
            true => {
                self.do_event();
                Some(())
            }
            false => None,
        }
    }

    fn do_event(&mut self) {
        self.state = match self.state {
            LimiterState::Off => LimiterState::First,
            LimiterState::First => LimiterState::Repeat,
            LimiterState::Repeat => LimiterState::Repeat,
        };
        self.time = 0f64;
    }

    pub fn is_ready(&self) -> bool {
        match self.state {
            LimiterState::Off => true,
            LimiterState::First => {
                match self.repeat_delay {
                    Some(delay) => {
                        if self.time > delay {
                            true
                        } else {
                            false
                        }
                    }
                    None => {
                        if self.time > self.repeat_rate {
                            true
                        } else {
                            false
                        }
                    }
                }
            }
            LimiterState::Repeat => {
                if self.time > self.repeat_rate {
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[test]
fn create_limiter() {
    RateLimiter::new(1f64, None);
    RateLimiter::new(1f64, Some(1f64));
}

#[test]
#[should_panic]
fn create_bad_repeat_limiter() {
    RateLimiter::new(-1f64, None);
}

#[test]
#[should_panic]
fn create_bad_delay_limiter() {
    RateLimiter::new(1f64, Some(-1f64));
}

#[test]
fn starts_ready() {
    let mut limit = RateLimiter::new(1f64, None);
    assert_eq!(limit.is_ready(), true);
    assert_eq!(limit.get_event(), Some(()));
}

#[test]
fn deny_two_attempts() {
    let mut limit = RateLimiter::new(1f64, None);
    assert_eq!(limit.is_ready(), true);
    assert_eq!(limit.get_event(), Some(()));
    assert_eq!(limit.is_ready(), false);
    assert_eq!(limit.get_event(), None);
}

#[test]
fn requset_works_after_wait() {
    let mut limit = RateLimiter::new(1f64, None);
    assert_eq!(limit.is_ready(), true);
    assert_eq!(limit.get_event(), Some(()));
    limit.elapsed(2f64);
    assert_eq!(limit.is_ready(), true);
    assert_eq!(limit.get_event(), Some(()));
}

#[test]
fn delay_vs_repeat() {
    let mut limit = RateLimiter::new(1f64, Some(2f64));
    assert_eq!(limit.is_ready(), true);
    assert_eq!(limit.get_event(), Some(()));
    assert_eq!(limit.is_ready(), false);
    assert_eq!(limit.get_event(), None);
    limit.elapsed(1.1f64);
    assert_eq!(limit.is_ready(), false);
    assert_eq!(limit.get_event(), None);
    limit.elapsed(1.1f64);
    assert_eq!(limit.is_ready(), true);
    assert_eq!(limit.get_event(), Some(()));
    limit.elapsed(1.1f64);
    assert_eq!(limit.is_ready(), true);
    assert_eq!(limit.get_event(), Some(()));
}

#[test]
fn ready_after_reset() {
    let mut limit = RateLimiter::new(1f64, Some(2f64));
    assert_eq!(limit.is_ready(), true);
    assert_eq!(limit.get_event(), Some(()));
    assert_eq!(limit.is_ready(), false);
    assert_eq!(limit.get_event(), None);
    limit.reset();
    assert_eq!(limit.is_ready(), true);
    assert_eq!(limit.get_event(), Some(()));
}
