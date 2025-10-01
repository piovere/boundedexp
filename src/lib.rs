use pyo3::prelude::*;

#[pymodule]
mod boundedexp {
    use pyo3::prelude::*;
    use rand::random;
    use rayon::prelude::*;
    use std::time::Duration;

    #[pyclass]
    pub struct MeasurementIsotope {
        half_life: Duration,
        measurement_time: Duration,
    }

    #[pymethods]
    impl MeasurementIsotope {
        #[new]
        #[pyo3(text_signature = "($self, half_life, measurement_time)")]
        pub fn new(half_life: Duration, measurement_time: Duration) -> Self {
            MeasurementIsotope {
                half_life,
                measurement_time,
            }
        }

        #[pyo3(text_signature = "($self)")]
        pub fn get_time(&self) -> Duration {
            let cdf = random::<f64>();
            self.mhl()
                .mul_f64(-(1.0 - cdf * self.inv_norm_const()).ln())
        }

        #[pyo3(text_signature = "($self, num)")]
        pub fn get_times(&self, num: usize) -> Vec<Duration> {
            (0..num).into_par_iter().map(|_| self.get_time()).collect()
        }

        fn inv_norm_const(&self) -> f64 {
            let ep = -self.measurement_time.div_duration_f64(self.mhl());
            1.0 - ep.exp()
        }

        fn mhl(&self) -> Duration {
            self.half_life.div_f64((2.0_f64).ln())
        }
    }

    impl Iterator for MeasurementIsotope {
        type Item = Duration;

        fn next(&mut self) -> Option<Self::Item> {
            Some(self.get_time())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dont_exceed_measurement_time() {
            let mut isotope =
                MeasurementIsotope::new(Duration::from_secs(1000 * 86400), Duration::from_secs(10));
            let mut times = Vec::new();
            for _ in 0..100 {
                times.push(isotope.next().unwrap());
            }
            assert!(times.iter().all(|&t| t <= Duration::from_secs(10)));
        }

        #[test]
        fn sample_many_times() {
            let isotope =
                MeasurementIsotope::new(Duration::from_secs(1000 * 86400), Duration::from_secs(10));
            let times = isotope.get_times(1000);
            assert!(times.iter().all(|&t| t <= Duration::from_secs(10)));
        }
    }
}
