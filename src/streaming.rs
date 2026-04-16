use std::str::FromStr;

/// Streaming latency mode controlling the encoder chunk size.
///
/// These modes match NVIDIA's documented Nemotron / Multitalker
/// `att_context_size` presets:
///
/// - `Normal`: `[70, 13]`
/// - `Low`: `[70, 6]`
/// - `VeryLow`: `[70, 1]`
/// - `Ultra`: `[70, 0]`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LatencyMode {
    /// `[70, 13]` -- 14 encoded frames, 112 mel frames, 1.12s latency.
    /// Highest accuracy. This is the default.
    #[default]
    Normal,
    /// `[70, 6]` -- 7 encoded frames, 56 mel frames, 0.56s latency.
    Low,
    /// `[70, 1]` -- 2 encoded frames, 16 mel frames, 0.16s latency.
    VeryLow,
    /// `[70, 0]` -- 1 encoded frame, 8 mel frames, 0.08s latency.
    /// Lowest accuracy.
    Ultra,
}

impl LatencyMode {
    /// Number of mel spectrogram frames per encoder chunk.
    pub const fn chunk_mel_frames(self) -> usize {
        match self {
            Self::Normal => 112,
            Self::Low => 56,
            Self::VeryLow => 16,
            Self::Ultra => 8,
        }
    }

    /// Number of encoded frames per chunk (after 8x subsampling).
    pub const fn encoded_frames(self) -> usize {
        match self {
            Self::Normal => 14,
            Self::Low => 7,
            Self::VeryLow => 2,
            Self::Ultra => 1,
        }
    }

    /// Approximate latency in seconds.
    pub const fn latency_secs(self) -> f32 {
        match self {
            Self::Normal => 1.12,
            Self::Low => 0.56,
            Self::VeryLow => 0.16,
            Self::Ultra => 0.08,
        }
    }
}

impl FromStr for LatencyMode {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "normal" => Ok(Self::Normal),
            "low" => Ok(Self::Low),
            "very-low" => Ok(Self::VeryLow),
            "ultra" => Ok(Self::Ultra),
            _ => Err("expected one of: normal, low, very-low, ultra"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LatencyMode;
    use std::str::FromStr;

    #[test]
    fn latency_modes_match_documented_chunk_sizes() {
        assert_eq!(LatencyMode::Normal.chunk_mel_frames(), 112);
        assert_eq!(LatencyMode::Low.chunk_mel_frames(), 56);
        assert_eq!(LatencyMode::VeryLow.chunk_mel_frames(), 16);
        assert_eq!(LatencyMode::Ultra.chunk_mel_frames(), 8);
    }

    #[test]
    fn latency_modes_parse_documented_cli_names() {
        assert_eq!(
            LatencyMode::from_str("normal").unwrap(),
            LatencyMode::Normal
        );
        assert_eq!(LatencyMode::from_str("low").unwrap(), LatencyMode::Low);
        assert_eq!(
            LatencyMode::from_str("very-low").unwrap(),
            LatencyMode::VeryLow
        );
        assert_eq!(LatencyMode::from_str("ultra").unwrap(), LatencyMode::Ultra);
    }
}
