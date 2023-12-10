use std::{
    io::{ self, ErrorKind, Read, Seek, SeekFrom, Cursor },
    pin::Pin,
    task::{ Context, Poll },
};

use async_trait::async_trait;
use songbird::input::{ AsyncMediaSource, AudioStreamError };
use symphonia::core::io::MediaSource;
use tokio::io::{ AsyncRead, ReadBuf, AsyncSeek };

pub struct GoogleTtsMediaSource {
    pub audio_data: Vec<u8>,
    pub position: usize,
}

impl GoogleTtsMediaSource {
    pub fn new(base64_audio_content: String) -> io::Result<Self> {
        // Convert the base64 string to bytes
        let base64_bytes = base64_audio_content.into_bytes();
        let mut cursor = Cursor::new(base64_bytes);

        // Decode the base64 content
        let mut audio_data = Vec::new();
        let engine = base64::engine::GeneralPurpose::new(
            &base64::alphabet::STANDARD,
            base64::engine::general_purpose::PAD
        );
        let mut decoder = base64::read::DecoderReader::new(&mut cursor, &engine);
        decoder
            .read_to_end(&mut audio_data)
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Base64 decode error"))?;

        Ok(GoogleTtsMediaSource {
            audio_data,
            position: 0,
        })
    }
}

// Implement Read, Seek, MediaSource, AsyncRead, AsyncSeek for GoogleTtsMediaSource
impl Read for GoogleTtsMediaSource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let available_data = &self.audio_data[self.position..];
        let to_read = buf.len().min(available_data.len());
        buf[..to_read].copy_from_slice(&available_data[..to_read]);
        self.position += to_read;
        Ok(to_read)
    }
}

impl Seek for GoogleTtsMediaSource {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let new_pos = match pos {
            SeekFrom::Start(offset) => offset as usize,
            SeekFrom::End(offset) => {
                let pos = (self.audio_data.len() as isize) + (offset as isize);
                if pos < 0 {
                    return Err(
                        io::Error::new(
                            ErrorKind::InvalidInput,
                            "Invalid seek to a negative position"
                        )
                    );
                }
                pos as usize
            }
            SeekFrom::Current(offset) => {
                let pos = (self.position as isize) + (offset as isize);
                if pos < 0 {
                    return Err(
                        io::Error::new(
                            ErrorKind::InvalidInput,
                            "Invalid seek to a negative position"
                        )
                    );
                }
                pos as usize
            }
        };

        if new_pos > self.audio_data.len() {
            return Err(io::Error::new(ErrorKind::InvalidInput, "Invalid seek beyond EOF"));
        }

        self.position = new_pos;
        Ok(new_pos as u64)
    }
}

impl MediaSource for GoogleTtsMediaSource {
    fn byte_len(&self) -> Option<u64> {
        Some(self.audio_data.len() as u64)
    }

    fn is_seekable(&self) -> bool {
        true
    }
}

impl Clone for GoogleTtsMediaSource {
    fn clone(&self) -> Self {
        GoogleTtsMediaSource {
            audio_data: self.audio_data.clone(),
            position: self.position,
        }
    }
}

#[async_trait]
impl AsyncMediaSource for GoogleTtsMediaSource {
    fn is_seekable(&self) -> bool {
        true
    }

    async fn byte_len(&self) -> Option<u64> {
        Some(self.audio_data.len() as u64)
    }

    async fn try_resume(
        &mut self,
        _offset: u64
    ) -> Result<Box<dyn AsyncMediaSource>, AudioStreamError> {
        // This implementation depends on how you can handle resuming in your application.
        // The basic idea is to recreate or reset the media source to resume from the given offset.
        // For now, it returns an error indicating that resuming is not supported.
        Err(AudioStreamError::Unsupported)
    }
}

impl AsyncRead for GoogleTtsMediaSource {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>
    ) -> Poll<Result<(), std::io::Error>> {
        let available_data = &self.audio_data[self.position..];
        let to_read = std::cmp::min(buf.remaining(), available_data.len());
        buf.put_slice(&available_data[..to_read]);
        self.position += to_read;
        Poll::Ready(Ok(()))
    }
}

impl AsyncSeek for GoogleTtsMediaSource {
    fn start_seek(mut self: Pin<&mut Self>, position: SeekFrom) -> Result<(), std::io::Error> {
        self.position = match position {
            SeekFrom::Start(pos) => pos as usize,
            SeekFrom::End(pos) => {
                // Ensure no underflow for negative values
                if pos.is_negative() {
                    let pos = -pos as usize;
                    if pos > self.audio_data.len() {
                        return Err(
                            io::Error::new(ErrorKind::InvalidInput, "Seek before the start")
                        );
                    }
                    self.audio_data.len() - pos
                } else {
                    let pos = pos as usize;
                    if pos > usize::MAX - self.audio_data.len() {
                        return Err(io::Error::new(ErrorKind::InvalidInput, "Seek beyond the end"));
                    }
                    self.audio_data.len() + pos
                }
            }
            SeekFrom::Current(pos) => {
                // Handle negative offset
                if pos.is_negative() {
                    let pos = -pos as usize;
                    if pos > self.position {
                        return Err(
                            io::Error::new(ErrorKind::InvalidInput, "Seek before the start")
                        );
                    }
                    self.position - pos
                } else {
                    let pos = pos as usize;
                    if pos > usize::MAX - self.position {
                        return Err(io::Error::new(ErrorKind::InvalidInput, "Seek beyond the end"));
                    }
                    self.position + pos
                }
            }
        };
        Ok(())
    }

    fn poll_complete(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>
    ) -> Poll<Result<u64, std::io::Error>> {
        Poll::Ready(Ok(self.position as u64))
    }
}
