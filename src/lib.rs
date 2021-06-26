pub trait RngJitter: rand::Rng {
    fn jitter_factor<T, S, F>(&mut self, val: T, amt: S) -> T
    where
        T: core::ops::Mul<F, Output = T> + core::ops::Add<T, Output = T> + Clone,
        S: rand::distributions::uniform::SampleRange<F>,
        F: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        let factor = self.gen_range(amt);
        val.clone() * factor + val
    }

    fn jitter_amount<T, S, F>(&mut self, val: T, amt: S) -> T
    where
        T: core::ops::Add<F, Output = T>,
        S: rand::distributions::uniform::SampleRange<F>,
        F: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        let amt = self.gen_range(amt);
        val + amt
    }
}

impl<T> RngJitter for T where T: rand::Rng {}

#[cfg(feature = "euclid")]
pub trait RngJitterEuclid: rand::Rng {
    fn jitter_point<T, S, U>(
        &mut self,
        val: euclid::Point2D<T, U>,
        len_amt: S,
        ang_amt: S,
    ) -> euclid::Point2D<T, U>
    where
        S: rand::distributions::uniform::SampleRange<T>,
        T: rand::distributions::uniform::SampleUniform
            + PartialOrd
            + euclid::Trig
            + core::ops::Mul<Output = T>
            + core::ops::Add<Output = T>
            + Copy,
    {
        let vec_len = self.gen_range(len_amt);
        let vec_radians = self.gen_range(ang_amt);
        let vector = euclid::Vector2D::<T, U>::from_angle_and_length(
            euclid::Angle::<T>::radians(vec_radians),
            vec_len,
        );
        val + vector
    }

    fn jitter_angle<T, S>(&mut self, val: euclid::Angle<T>, amt: S) -> euclid::Angle<T>
    where
        S: rand::distributions::uniform::SampleRange<T>,
        T: rand::distributions::uniform::SampleUniform + PartialOrd + core::ops::Add<Output = T>,
    {
        let radians = self.gen_range(amt);
        val + euclid::Angle::<T>::radians(radians)
    }
}

#[cfg(feature = "euclid")]
impl<T> RngJitterEuclid for T where T: rand::Rng {}
