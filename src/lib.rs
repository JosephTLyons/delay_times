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

// Using a state pattern design with `DelayTimes` and `DelayTimesModifier` to make sure the user
// can't repeatedly call certain functions, like `tripler()`

pub enum NoteModifier {
    Normal,
    Dotted,
    Triplet,
}

pub enum DelayUnit {
    Hertz,
    Milliseconds,
}

impl DelayTimes {
    pub fn new(beats_per_minute: f64, delay_unit: DelayUnit, note_modifier: NoteModifier) -> Self {
        let delay_unit_value = match delay_unit {
            DelayUnit::Hertz => beats_per_minute / 60.0,
            DelayUnit::Milliseconds => 60_000.0 / beats_per_minute,
        };

        let note_modifier_value = match note_modifier {
            NoteModifier::Normal => 1.0,
            NoteModifier::Dotted => 1.5,
            NoteModifier::Triplet => 2.0 / 3.0,
        };

        let base_value = delay_unit_value * note_modifier_value;

        Self {
            v_whole: base_value * 4.0,
            v_half: base_value * 2.0,
            v_quarter: base_value,
            v_8th: base_value / 2.0,
            v_16th: base_value / 4.0,
            v_32nd: base_value / 8.0,
            v_64th: base_value / 16.0,
            v_128th: base_value / 32.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DelayTimes;

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
        use crate::{DelayTimes, DelayUnit, NoteModifier};

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

            let actual_delay_times =
                DelayTimes::new(120.0, DelayUnit::Milliseconds, NoteModifier::Normal);

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

            let actual_delay_times =
                DelayTimes::new(120.0, DelayUnit::Milliseconds, NoteModifier::Dotted);

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

            let actual_delay_times =
                DelayTimes::new(120.0, DelayUnit::Milliseconds, NoteModifier::Triplet);

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }
    }

    mod hz_tests {
        use super::assert_delay_times_instances_are_equal;
        use crate::{DelayTimes, DelayUnit, NoteModifier};

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

            let actual_delay_times = DelayTimes::new(120.0, DelayUnit::Hertz, NoteModifier::Normal);

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

            let actual_delay_times = DelayTimes::new(120.0, DelayUnit::Hertz, NoteModifier::Dotted);

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

            let actual_delay_times =
                DelayTimes::new(120.0, DelayUnit::Hertz, NoteModifier::Triplet);

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }
    }
}
