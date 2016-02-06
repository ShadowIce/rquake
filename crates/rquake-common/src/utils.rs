#![warn(missing_docs)]

//! Small utility functions / structs.

extern crate time;

/// Timer for frame times.
pub struct Timer {
    current_time : u64,
    target_time : f32,
    lower_bound : f32,
    upper_bound: f32,
}

impl Timer {
    /// Constructs a new Timer
    ///
    /// Timer will use bounds of 0.001..0.1 and a target frame time of 1/72s (72fps).
    pub fn new() -> Timer {
        Timer {
            current_time : time::precise_time_ns(),
            target_time: 1.0 / 72.0,
            lower_bound : 0.001,
            upper_bound: 0.1,
        }
    }

    /// Sets new lower and upper bounds for the timer.
    pub fn set_bounds(&mut self, lower: f32, upper: f32) {
        debug_assert!(lower < upper);
        self.lower_bound = lower;
        self.upper_bound = upper;
    }
    
    /// Sets a new target time for the timer.
    pub fn set_target(&mut self, target : f32) {
        self.target_time = target;
    }
    
    /// Returns the time for the next frame, or 0.0 if not enough time passed.
    pub fn next(&mut self) -> Option<f32> {
        let new_time = time::precise_time_ns();
        let rt_in_s = (new_time - self.current_time) as f32 / 1000000000.0f32;

        if rt_in_s > self.target_time {
            // Only increase time if at least target_time seconds have passed.   
            self.current_time = new_time;
            if rt_in_s < self.lower_bound {
                return Some(self.lower_bound);
            } else if rt_in_s > self.upper_bound {
                return Some(self.upper_bound);
            } else {
                return Some(rt_in_s);
            }
        }
        
        None
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;
    
    #[test]
    fn timer_bounds_test() {
        let mut t = Timer::new();
        t.set_bounds(0.001f32, 0.1f32);
        // upper bounds test
        sleep(Duration::new(1, 0));
        if let Some(next_time) = t.next() {
            assert!(next_time >= 0.001f32);
            assert!(next_time <= 0.1f32);
        } else {
            assert!(false, "upper bound invalid");
        }
        
        // lower bounds test
        let mut next_time = t.next();
        while next_time.is_none() {
            next_time = t.next();
        }
        if let Some(next_time) = next_time {
            assert!(next_time >= 0.001f32);
            assert!(next_time <= 0.1f32);
        } else {
            assert!(false, "lower bound invalid");
        }
    }
}