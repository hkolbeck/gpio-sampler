#![no_std]

pub trait Hal<T> {
    fn read(&mut self) -> T;
    fn timestamp_ns(&self) -> u64;
}

pub struct Sampler<T: 'static, H: Hal<T>, F: FnMut(&mut [T], u64)> {
    hal: H,
    buf: &'static mut [T],
    fun: F,
    delay_ns: u64,
}

impl <T, H: Hal<T>, F: FnMut(&mut [T], u64)> Sampler<T, H, F> {
    pub fn new() -> Sampler<T, H, F> {
        todo!()
    }

    pub fn sample(mut self) -> ! {
        let mut batch_start = self.hal.timestamp_ns();
        let mut last_read = batch_start;

        let mut idx = 0;
        self.buf[idx] = self.hal.read();
        idx += 1;

        loop {
            if self.hal.timestamp_ns() <= last_read + self.delay_ns {
                if idx >= self.buf.len() {
                    (self.fun)(&mut self.buf, self.hal.timestamp_ns() - batch_start);
                    batch_start = self.hal.timestamp_ns();
                    idx = 0
                }

                self.buf[idx] = self.hal.read();
                last_read = self.hal.timestamp_ns();
                idx += 1;
            }
        }
    }
}