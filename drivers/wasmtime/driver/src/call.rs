// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 The Hyperlight Authors.

/// A borrowed Postcard FunctionCall from /dev/hlcall.
pub struct Call<'a> {
    name: &'a str,
    strings: Vec<Option<&'a str>>,
}

impl<'a> Call<'a> {
    pub fn parse(buf: &'a [u8]) -> Option<Self> {
        let mut reader = Reader(buf);
        let name = reader.string()?;
        if reader.take(1)? != [0] || reader.uint(32)? > 10 {
            return None;
        }
        let count = usize::try_from(reader.uint(64)?).ok()?;
        if count > reader.0.len() {
            return None;
        }
        let mut strings = Vec::with_capacity(count);
        let mut external = 0usize;
        for _ in 0..count {
            let string = match reader.uint(32)? {
                0 | 1 => {
                    reader.uint(32)?;
                    None
                }
                2 | 3 => {
                    reader.uint(64)?;
                    None
                }
                4 => {
                    reader.take(4)?;
                    None
                }
                5 => {
                    reader.take(8)?;
                    None
                }
                6 => Some(reader.string()?),
                7 => {
                    if reader.take(1)?[0] > 1 {
                        return None;
                    }
                    None
                }
                9 => {
                    external = external.checked_add(usize::try_from(reader.uint(64)?).ok()?)?;
                    if reader.take(1)?[0] > 1 {
                        return None;
                    }
                    None
                }
                _ => return None,
            };
            strings.push(string);
        }
        (reader.0.len() == external).then_some(Self { name, strings })
    }

    pub fn name(&self) -> Option<&'a str> {
        Some(self.name)
    }

    pub fn string_arg(&self, index: usize) -> Option<&'a str> {
        self.strings.get(index).copied().flatten()
    }
}

struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        let (value, rest) = self.0.split_at_checked(len)?;
        self.0 = rest;
        Some(value)
    }

    fn uint(&mut self, bits: u32) -> Option<u64> {
        let mut value = 0;
        for shift in (0..bits).step_by(7) {
            let byte = self.take(1)?[0];
            if bits - shift < 7 && u32::from(byte) >= (1 << (bits - shift)) {
                return None;
            }
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Some(value);
            }
        }
        None
    }

    fn string(&mut self) -> Option<&'a str> {
        let len = usize::try_from(self.uint(64)?).ok()?;
        std::str::from_utf8(self.take(len)?).ok()
    }
}
