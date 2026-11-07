pub fn note_to_frequency(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((f32::from(note) - 69.0) / 12.0)
}

#[cfg(test)]
mod tests {
    use super::note_to_frequency;

    #[test]
    fn midi_note_69_is_a440() {
        assert!((note_to_frequency(69) - 440.0).abs() < 1e-5);
    }
}
