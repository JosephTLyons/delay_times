#[allow(unused_imports)]
#[macro_use]
extern crate assert_float_eq;

#[readonly::make]
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

impl DelayTimes {
    pub fn in_ms(beats_per_minute: f64) -> Self {
        let ms: f64 = 60_000.0 / beats_per_minute;
        DelayTimes::get_instance(ms)
    }

    pub fn in_hz(beats_per_minute: f64) -> Self {
        let hz: f64 = beats_per_minute / 60.0;
        DelayTimes::get_instance(hz)
    }

    pub fn dotted(&self) -> Self {
        self.multiply_all_values_by(1.5)
    }

    pub fn triplet(&self) -> Self {
        self.multiply_all_values_by(2.0 / 3.0)
    }

    fn get_instance(base_value: f64) -> Self {
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

    fn multiply_all_values_by(&self, multiplier: f64) -> Self {
        Self {
            v_whole: self.v_whole * multiplier,
            v_half: self.v_half * multiplier,
            v_quarter: self.v_quarter * multiplier,
            v_8th: self.v_8th * multiplier,
            v_16th: self.v_16th * multiplier,
            v_32nd: self.v_32nd * multiplier,
            v_64th: self.v_64th * multiplier,
            v_128th: self.v_128th * multiplier,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DelayTimes;

    fn assert_delay_times_instances_are_equal(
        delay_times_a: &DelayTimes,
        delay_times_b: &DelayTimes,
    ) {
        let tolerance = 0.0001;
        assert_float_absolute_eq!(delay_times_a.v_whole, delay_times_b.v_whole, tolerance);
        assert_float_absolute_eq!(delay_times_a.v_half, delay_times_b.v_half, tolerance);
        assert_float_absolute_eq!(delay_times_a.v_quarter, delay_times_b.v_quarter, tolerance);
        assert_float_absolute_eq!(delay_times_a.v_8th, delay_times_b.v_8th, tolerance);
        assert_float_absolute_eq!(delay_times_a.v_16th, delay_times_b.v_16th, tolerance);
        assert_float_absolute_eq!(delay_times_a.v_32nd, delay_times_b.v_32nd, tolerance);
        assert_float_absolute_eq!(delay_times_a.v_64th, delay_times_b.v_64th, tolerance);
        assert_float_absolute_eq!(delay_times_a.v_128th, delay_times_b.v_128th, tolerance);
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

            let actual_delay_times = DelayTimes::in_ms(120.0);

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

            let actual_delay_times = DelayTimes::in_ms(120.0).dotted();

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

            let actual_delay_times = DelayTimes::in_ms(120.0).triplet();

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

            let actual_delay_times = DelayTimes::in_hz(120.0);

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

            let actual_delay_times = DelayTimes::in_hz(120.0).dotted();

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

            let actual_delay_times = DelayTimes::in_hz(120.0).triplet();

            assert_delay_times_instances_are_equal(&expected_delay_times, &actual_delay_times)
        }
    }
}
