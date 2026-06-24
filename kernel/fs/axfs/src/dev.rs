use axdriver::prelude::*;

const BLOCK_SIZE: usize = 512;

/// A disk device with a cursor.
pub struct Disk {
    block_id: u64,
    offset: usize,
    dev: AxBlockDevice,
    read_buffer: [u8; BLOCK_SIZE],
    write_buffer: [u8; BLOCK_SIZE],
    write_buffer_block_id: u64,
    write_buffer_dirty: bool,
}

impl Disk {
    /// Create a new disk.
    pub fn new(dev: AxBlockDevice) -> Self {
        assert_eq!(BLOCK_SIZE, dev.block_size());
        Self {
            block_id: 0,
            offset: 0,
            dev,
            read_buffer: [0; BLOCK_SIZE],
            write_buffer: [0; BLOCK_SIZE],
            write_buffer_block_id: 0,
            write_buffer_dirty: false,
        }
    }

    /// Get the size of the disk.
    pub fn size(&self) -> u64 {
        self.dev.num_blocks() * BLOCK_SIZE as u64
    }

    /// Get the position of the cursor.
    pub fn position(&self) -> u64 {
        self.block_id * BLOCK_SIZE as u64 + self.offset as u64
    }

    /// Set the position of the cursor.
    pub fn set_position(&mut self, pos: u64) {
        self.block_id = pos / BLOCK_SIZE as u64;
        self.offset = pos as usize % BLOCK_SIZE;
    }

    /// Flushes the pending partial-block write buffer, if any.
    pub fn flush(&mut self) -> DevResult {
        if self.write_buffer_dirty {
            self.dev
                .write_block(self.write_buffer_block_id, &self.write_buffer)?;
            self.write_buffer_dirty = false;
        }
        Ok(())
    }

    fn advance(&mut self, count: usize) {
        self.offset += count;
        if self.offset >= BLOCK_SIZE {
            self.block_id += 1;
            self.offset -= BLOCK_SIZE;
        }
    }

    fn prepare_write_buffer(&mut self) -> DevResult {
        if self.write_buffer_dirty && self.write_buffer_block_id == self.block_id {
            return Ok(());
        }
        self.flush()?;
        self.dev
            .read_block(self.block_id, &mut self.write_buffer)?;
        self.write_buffer_block_id = self.block_id;
        self.write_buffer_dirty = true;
        Ok(())
    }

    /// Read within one block, returns the number of bytes read.
    pub fn read_one(&mut self, buf: &mut [u8]) -> DevResult<usize> {
        let read_size = if self.offset == 0 && buf.len() >= BLOCK_SIZE {
            // whole block
            if self.write_buffer_dirty && self.write_buffer_block_id == self.block_id {
                buf[..BLOCK_SIZE].copy_from_slice(&self.write_buffer);
            } else {
                self.dev
                    .read_block(self.block_id, &mut buf[0..BLOCK_SIZE])?;
            }
            self.block_id += 1;
            BLOCK_SIZE
        } else {
            // partial block
            let start = self.offset;
            let count = buf.len().min(BLOCK_SIZE - self.offset);

            if self.write_buffer_dirty && self.write_buffer_block_id == self.block_id {
                buf[..count].copy_from_slice(&self.write_buffer[start..start + count]);
            } else {
                self.dev
                    .read_block(self.block_id, &mut self.read_buffer)?;
                buf[..count].copy_from_slice(&self.read_buffer[start..start + count]);
            }

            self.advance(count);
            count
        };
        Ok(read_size)
    }

    /// Write within one block, returns the number of bytes written.
    pub fn write_one(&mut self, buf: &[u8]) -> DevResult<usize> {
        let write_size = if self.offset == 0 && buf.len() >= BLOCK_SIZE {
            // whole block
            if self.write_buffer_dirty {
                if self.write_buffer_block_id == self.block_id {
                    self.write_buffer_dirty = false;
                } else {
                    self.flush()?;
                }
            }
            self.dev.write_block(self.block_id, &buf[0..BLOCK_SIZE])?;
            self.block_id += 1;
            BLOCK_SIZE
        } else {
            // partial block
            let start = self.offset;
            let count = buf.len().min(BLOCK_SIZE - self.offset);

            self.prepare_write_buffer()?;
            self.write_buffer[start..start + count].copy_from_slice(&buf[..count]);

            self.advance(count);
            if self.offset == 0 {
                self.flush()?;
            }
            count
        };
        Ok(write_size)
    }
}
