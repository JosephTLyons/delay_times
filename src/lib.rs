#[allow(unused_imports)]
#[macro_use]
extern crate assert_float_eq;

#[readonly::make]
#[derive(Clone)]
pub struct DelayTimes {
    pub v_whole: f64,
    pub v_half: f64,
    pub v_quarter: f64,
    pub v_8th: f64,
    pub v_16th: f64,
    pub v_32nd: f64,
    pub v_64th: f64,
    pub v_128th: f64,
}

// Using a state-pattern version of a builder-pattern design to make sure the user can't repeatedly call certain functions, like `triplet()`

impl DelayTimes {
    #[allow(clippy::new_ret_no_self)] // Maybe turn this off and think of a differnt name other than `new`
    pub fn new(beats_per_minute: f64) -> DelayTimesPeriodicUnit {
        DelayTimesPeriodicUnit::new(beats_per_minute)
    }

    fn get_instance(quarter_note_delay_value: f64, multiplier: f64) -> DelayTimes {
        DelayTimes {
            v_whole: (quarter_note_delay_value * 4.0) * multiplier,
            v_half: (quarter_note_delay_value * 2.0) * multiplier,
            v_quarter: (quarter_note_delay_value) * multiplier,
            v_8th: (quarter_note_delay_value / 2.0) * multiplier,
            v_16th: (quarter_note_delay_value / 4.0) * multiplier,
            v_32nd: (quarter_note_delay_value / 8.0) * multiplier,
            v_64th: (quarter_note_delay_value / 16.0) * multiplier,
            v_128th: (quarter_note_delay_value / 32.0) * multiplier,
        }
    }
}

//  TODO: Better name
pub struct DelayTimesPeriodicUnit {
    beats_per_minute: f64,
}

impl DelayTimesPeriodicUnit {
    fn new(beats_per_minute: f64) -> Self {
        Self { beats_per_minute }
    }

    pub fn in_ms(&self) -> DelayTimesNoteModifier {
        let quarter_note_in_ms: f64 = 60_000.0 / self.beats_per_minute;
        DelayTimesNoteModifier::new(quarter_note_in_ms)
    }

    pub fn in_hz(&self) -> DelayTimesNoteModifier {
        let quarter_note_in_hz: f64 = self.beats_per_minute / 60.0;
        DelayTimesNoteModifier::new(quarter_note_in_hz)
    }
}

pub struct DelayTimesNoteModifier {
    quarter_note_delay_value: f64,
}

impl DelayTimesNoteModifier {
    fn new(quarter_note_delay_value: f64) -> Self {
        Self {
            quarter_note_delay_value,
        }
    }

    pub fn normal(&self) -> DelayTimes {
        DelayTimes::get_instance(self.quarter_note_delay_value, 1.0)
    }

    pub fn dotted(&self) -> DelayTimes {
        DelayTimes::get_instance(self.quarter_note_delay_value, 1.5)
    }

    pub fn triplet(&self) -> DelayTimes {
        DelayTimes::get_instance(self.quarter_note_delay_value, 2.0 / 3.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::DelayTimes;

    // This function might be able to go away if there's some sort of macro we can put on the struct that makes the structs auto-comparable
    fn assert_delay_times_instances_are_equal(
        expected_delay_times: &DelayTimes,
        actual_delay_times: &DelayTimes,
    ) {
        let tolerance = 0.0001;

        let expected_and_actual_values = [
            (expected_delay_times.v_whole, actual_delay_times.v_whole),
            (expected_delay_times.v_half, actual_delay_times.v_half),
            (expected_delay_times.v_quarter, actual_delay_times.v_quarter),
            (expected_delay_times.v_8th, actual_delay_times.v_8th),
            (expected_delay_times.v_16th, actual_delay_times.v_16th),
            (expected_delay_times.v_32nd, actual_delay_times.v_32nd),
            (expected_delay_times.v_64th, actual_delay_times.v_64th),
            (expected_delay_times.v_128th, actual_delay_times.v_128th),
        ];

        for (expected_value, actual_value) in expected_and_actual_values {
            assert_float_absolute_eq!(expected_value, actual_value, tolerance);
        }
    }

    mod ms_tests {
        use super::assert_delay_times_instances_are_equal;
        use super::DelayTimes;

        #[test]
        fn test_normal() {
            let expected_delay_times = DelayTimes {
                v_whole: 2000.0,
                v_half: 1000.0,
                v_quarter: 500.0,
                v_8th: 250.0,
                v_16th: 125.0,
                v_32nd: 62.5,
                v_64th: 31.25,
                v_128th: 15.625,
            };

            let actual_delay_times = DelayTimes::new(120.0).in_ms().normal();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }

        #[test]
        fn test_dotted() {
            let expected_delay_times = DelayTimes {
                v_whole: 3000.0,
                v_half: 1500.0,
                v_quarter: 750.0,
                v_8th: 375.0,
                v_16th: 187.5,
                v_32nd: 93.75,
                v_64th: 46.875,
                v_128th: 23.4375,
            };

            let actual_delay_times = DelayTimes::new(120.0).in_ms().dotted();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }

        #[test]
        fn test_triplet() {
            let expected_delay_times = DelayTimes {
                v_whole: 1333.3333,
                v_half: 666.6666,
                v_quarter: 333.3333,
                v_8th: 166.6666,
                v_16th: 83.3333,
                v_32nd: 41.6666,
                v_64th: 20.8333,
                v_128th: 10.4166,
            };

            let actual_delay_times = DelayTimes::new(120.0).in_ms().triplet();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }
    }

    mod hz_tests {
        use super::assert_delay_times_instances_are_equal;
        use super::DelayTimes;

        #[test]
        fn test_normal() {
            let expected_delay_times = DelayTimes {
                v_whole: 8.0,
                v_half: 4.0,
                v_quarter: 2.0,
                v_8th: 1.0,
                v_16th: 0.5,
                v_32nd: 0.25,
                v_64th: 0.125,
                v_128th: 0.0625,
            };

            let actual_delay_times = DelayTimes::new(120.0).in_hz().normal();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }

        #[test]
        fn test_dotted() {
            let expected_delay_times = DelayTimes {
                v_whole: 12.0,
                v_half: 6.0,
                v_quarter: 3.0,
                v_8th: 1.5,
                v_16th: 0.75,
                v_32nd: 0.375,
                v_64th: 0.1875,
                v_128th: 0.0937,
            };

            let actual_delay_times = DelayTimes::new(120.0).in_hz().dotted();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }

        #[test]
        fn test_triplet() {
            let expected_delay_times = DelayTimes {
                v_whole: 5.3333,
                v_half: 2.6666,
                v_quarter: 1.3333,
                v_8th: 0.6666,
                v_16th: 0.3333,
                v_32nd: 0.1666,
                v_64th: 0.0833,
                v_128th: 0.0416,
            };

            let actual_delay_times = DelayTimes::new(120.0).in_hz().triplet();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }
    }

    // Here are a couple of weird tests that just ensure we don't break the interface
    // They can't fail, but at least the code won't compile if something about the inferface changes
    mod interface_tests {
        #[test]
        fn test_single_shot() {
            use super::DelayTimes;

            let _delay_times = DelayTimes::new(120.0).in_ms().normal();

            assert!(true)
        }

        #[test]
        fn test_reusability() {
            use super::DelayTimes;

            let delay_times_modifier = DelayTimes::new(120.0);

            let _delay_times_normal = delay_times_modifier.in_ms().normal();
            let _delay_times_dotted = delay_times_modifier.in_ms().dotted();
            let _delay_times_triplet = delay_times_modifier.in_ms().triplet();

            let _delay_times_normal = delay_times_modifier.in_hz().normal();
            let _delay_times_dotted = delay_times_modifier.in_hz().dotted();
            let _delay_times_triplet = delay_times_modifier.in_hz().triplet();

            assert!(true)
        }
    }
}
