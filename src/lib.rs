#![no_std]

pub trait Hal<T> {
    fn read(&mut self) -> T;
    fn timestamp_ns(&self) -> u64;
}

pub struct Sampler<T: 'static, H: Hal<T>> {
    hal: H,
    delay_ns: u64,
    __phantom_data: T,
}

impl<T, H: Hal<T>> Sampler<T, H> {
    pub fn new(hal: H, buf: &mut [T], sample_rate_hz: u32) -> Sampler<T, H> {
        todo!()
    }

    pub fn sample_frame<F: FnMut(&mut [T], u64)>(mut self, mut buf: &mut [T], fun: F) -> ! {
        let mut batch_start = self.hal.timestamp_ns();
        let mut last_read = 0;
        let mut idx = 0;
        loop {
            if self.hal.timestamp_ns() <= last_read + self.delay_ns || last_read == 0 {
                buf[idx] = self.hal.read();
                idx += 1;

                if idx >= buf.len() {
                    fun(&mut buf, self.hal.timestamp_ns() - batch_start);
                    batch_start = self.hal.timestamp_ns();
                    idx = 0
                }
            }
        }
    }
}