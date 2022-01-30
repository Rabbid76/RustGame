use crate::context::ContextData;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub trait Time {
    fn get_ticks(&self) -> f64;
    fn new_clock(&self) -> Clock;
}

pub struct TimeStd {
    context_data: Arc<ContextData>,
}

impl TimeStd {
    pub fn from(context: Arc<ContextData>) -> Result<TimeStd, Box<dyn Error>> {
        Ok(TimeStd {
            context_data: context.clone(),
        })
    }
}

impl Time for TimeStd {
    fn get_ticks(&self) -> f64 {
        let duration = Instant::now() - self.context_data.initialization_time;
        duration.as_micros() as f64 / 1000.0
    }

    fn new_clock(&self) -> Clock {
        let time_now = Instant::now();
        Clock {
            start_time: time_now,
            previous_time: time_now,
            frames: 0,
        }
    }
}

pub struct Clock {
    start_time: Instant,
    previous_time: Instant,
    frames: u64,
}

impl Clock {
    fn initialize_tick(&mut self) -> f64 {
        self.frames = 1;
        let time_now = Instant::now();
        self.start_time = time_now;
        self.previous_time = time_now;
        0.0
    }

    fn update_tick(&mut self) -> f64 {
        self.frames += 1;
        let previous_time = self.previous_time;
        let time_now = Instant::now();
        self.previous_time = time_now;
        (time_now - previous_time).as_micros() as f64 / 1000.0
    }

    pub fn tick(&mut self) -> f64 {
        if self.frames == 0 {
            return self.initialize_tick();
        }
        self.update_tick()
    }

    pub fn tick_frame_rate(&mut self, frame_rate: u16) -> f64 {
        if self.frames == 0 {
            return self.initialize_tick();
        }

        let expected_duration = self.frames as i128 * 1000 / frame_rate as i128;
        let actual_duration = (Instant::now() - self.start_time).as_millis() as i128;
        let delta_duration = expected_duration - actual_duration;
        if delta_duration > 0 {
            ::std::thread::sleep(Duration::from_millis((delta_duration) as u64));
        }
        self.update_tick()
    }
}

#[cfg(test)]
mod context_test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_get_ticks() {
        let time = TimeStd::from(Arc::new(ContextData::new())).unwrap();
        ::std::thread::sleep(Duration::from_millis(10));
        let ticks = time.get_ticks();
        assert!(ticks >= 10.0 && ticks < 100.0);
    }

    #[test]
    fn test_clock_tick() {
        let time = TimeStd::from(Arc::new(ContextData::new())).unwrap();
        let mut clock = time.new_clock();
        let ticks = clock.tick();
        assert_eq!(0.0, ticks);
        ::std::thread::sleep(Duration::from_millis(10));
        let ticks = clock.tick();
        assert!(ticks >= 9.9 && ticks < 30.0);
    }

    #[test]
    fn test_clock_tick_frame_rate() {
        let time = TimeStd::from(Arc::new(ContextData::new())).unwrap();
        let mut clock = time.new_clock();
        let ticks = clock.tick();
        assert_eq!(0.0, ticks);
        let ticks = clock.tick_frame_rate(100);
        assert!(ticks >= 9.9 && ticks < 30.0);
    }
}
