pub fn get_frame_count(l: f32, fr: u32) -> usize{
    let c = l * fr as f32;
    c as usize
}

pub fn get_frame_length(sr: u32, fr: u32) -> usize{
    let c = sr / fr;
    c as usize
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_are_returned_correctly() {
        assert_eq!(25, get_frame_count(0.5, 50));
    }

    #[test]
    fn frame_length_is_correct_44k_50() {
        assert_eq!(882, get_frame_length(44100, 50));
    }

    #[test]
    fn frame_length_is_correct_96k_50() {
        assert_eq!(1920, get_frame_length(96000, 50));
    }
}