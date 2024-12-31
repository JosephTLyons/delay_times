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
    pub fn new(beats_per_minute: f64) -> DelayTimesBuilder {
        DelayTimesBuilder::new(beats_per_minute)
    }

    fn get_instance(beats_per_minute: f64, unit: TimeUnit, modifier: RhythmicModifier) -> Self {
        Self {
            v_whole: calculate(beats_per_minute, unit, modifier, 4.0),
            v_half: calculate(beats_per_minute, unit, modifier, 2.0),
            v_quarter: calculate(beats_per_minute, unit, modifier, 1.0),
            v_8th: calculate(beats_per_minute, unit, modifier, 0.5),
            v_16th: calculate(beats_per_minute, unit, modifier, 0.25),
            v_32nd: calculate(beats_per_minute, unit, modifier, 1.0 / 8.0),
            v_64th: calculate(beats_per_minute, unit, modifier, 1.0 / 16.0),
            v_128th: calculate(beats_per_minute, unit, modifier, 1.0 / 32.0),
        }
    }
}

fn calculate(
    beats_per_minute: f64,
    unit: TimeUnit,
    modifier: RhythmicModifier,
    note_value: f64,
) -> f64 {
    let modified_value = note_value * modifier.value();
    match unit {
        TimeUnit::Ms => (60_000.0 / beats_per_minute) * modified_value,
        TimeUnit::Hz => (beats_per_minute / 60.0) / modified_value,
    }
}

#[derive(Clone, Copy)]
enum TimeUnit {
    Ms,
    Hz,
}

#[derive(Clone, Copy)]
enum RhythmicModifier {
    Normal,
    Dotted,
    Triplet,
}

impl RhythmicModifier {
    fn value(&self) -> f64 {
        match self {
            RhythmicModifier::Normal => 1.0,
            RhythmicModifier::Dotted => 1.5,
            RhythmicModifier::Triplet => 2.0 / 3.0,
        }
    }
}

pub struct DelayTimesBuilder {
    beats_per_minute: f64,
}

impl DelayTimesBuilder {
    fn new(beats_per_minute: f64) -> Self {
        Self { beats_per_minute }
    }

    pub fn in_ms(&self) -> DelayTimesNoteModifier {
        DelayTimesNoteModifier::new(self.beats_per_minute, TimeUnit::Ms)
    }

    pub fn in_hz(&self) -> DelayTimesNoteModifier {
        DelayTimesNoteModifier::new(self.beats_per_minute, TimeUnit::Hz)
    }
}

pub struct DelayTimesNoteModifier {
    beats_per_minute: f64,
    unit: TimeUnit,
}

impl DelayTimesNoteModifier {
    fn new(beats_per_minute: f64, unit: TimeUnit) -> Self {
        Self {
            beats_per_minute,
            unit,
        }
    }

    pub fn normal(&self) -> DelayTimes {
        DelayTimes::get_instance(self.beats_per_minute, self.unit, RhythmicModifier::Normal)
    }

    pub fn dotted(&self) -> DelayTimes {
        DelayTimes::get_instance(self.beats_per_minute, self.unit, RhythmicModifier::Dotted)
    }

    pub fn triplet(&self) -> DelayTimes {
        DelayTimes::get_instance(self.beats_per_minute, self.unit, RhythmicModifier::Triplet)
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;

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
                v_whole: 0.5,
                v_half: 1.0,
                v_quarter: 2.0,
                v_8th: 4.0,
                v_16th: 8.0,
                v_32nd: 16.0,
                v_64th: 32.0,
                v_128th: 64.0,
            };

            let actual_delay_times = DelayTimes::new(120.0).in_hz().normal();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }

        #[test]
        fn test_dotted() {
            let expected_delay_times = DelayTimes {
                v_whole: 0.3333,
                v_half: 0.6666,
                v_quarter: 1.3333,
                v_8th: 2.6666,
                v_16th: 5.3333,
                v_32nd: 10.6666,
                v_64th: 21.3333,
                v_128th: 42.6666,
            };

            let actual_delay_times = DelayTimes::new(120.0).in_hz().dotted();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }

        #[test]
        fn test_triplet() {
            let expected_delay_times = DelayTimes {
                v_whole: 0.75,
                v_half: 1.5,
                v_quarter: 3.0,
                v_8th: 6.0,
                v_16th: 12.0,
                v_32nd: 24.0,
                v_64th: 48.0,
                v_128th: 96.0,
            };

            let actual_delay_times = DelayTimes::new(120.0).in_hz().triplet();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }
    }

    // Here are a couple of weird tests that just ensure we don't break the interface
    // They can't fail, but at least the code won't compile if something about the interface changes
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
