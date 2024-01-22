const SAMPLE_RATE: usize = crate::SAMPLE_RATE as usize;

type Beat = usize;
type StereoSample = (f32, f32);
type MonoSample = f32;

pub fn get_beat_length(beats_in_measure: usize, sample_rate: usize) -> usize {
    sample_rate / beats_in_measure
}

pub fn get_sample_range(
    start: Beat,
    end: Beat,
    beats_in_measure: usize,
    sample_rate: usize,
) -> (usize, usize) {
    let l = get_beat_length(beats_in_measure, sample_rate);
    (start * l, end * l)
}

pub fn mix_mono_to_mono(a: &mut MonoSample, b: MonoSample) {
    *a += b
}

pub fn mix_mono_to_stereo(a: &mut StereoSample, b: MonoSample) {
    *a = (a.0 + b, a.1 + b)
}

pub fn mix_stereo_to_stereo(a: &mut StereoSample, b: StereoSample) {
    *a = (a.0 + b.0, a.1 + b.1)
}

pub fn amplify(s: &mut StereoSample, a: f32) {
    *s = (s.0 * a, s.1 * a)
}

pub fn amplify_vec(vec: &mut Vec<(f32, f32)>, a: f32) {
    for elem in vec.iter_mut() {
        elem.0 = elem.0*a;
        elem.1 = elem.1*a;
    }
}

pub fn delay(
    vec: &mut Vec<(f32, f32)>,
    start: Beat,
    end: Beat,
    delay: Beat,
    feedback: f32,
    beats_in_measure: usize,
    max: i32,
    sample_rate: usize,
) {
    let l = get_beat_length(beats_in_measure, sample_rate);
    let mut amp = feedback;
    let minus = amp / max as f32;
    let range = get_sample_range(start, end, beats_in_measure, sample_rate);
    let mut buffer = Vec::from_iter(vec[range.0..range.1].iter().cloned());
    let mut index = range.0 + delay * l;
    for _ in 0..max {
        let mut b = 0;
        amplify_vec(&mut buffer, amp);
        for i in index..(index + buffer.len()) {
            mix_stereo_to_stereo(&mut vec[i], buffer[b]);
            b += 1;
        }
        index += delay * l;
        amp -= minus;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_amplify() {
        let mut vec = vec![(1., 1.), (2., 2.)];
        amplify_vec(&mut vec, 2.);
        let assert = vec![(2., 2.), (4., 4.)];
        assert_eq!(vec, assert);
    }

    #[test]
    fn test_delay() {
        let mut vec = vec![
            (1., 1.),
            (0., 0.),
            (0., 0.),
            (0., 0.),
            (0., 0.),
            (0., 0.),
            (0., 0.),
            (0., 0.),
            (0., 0.),
            (0., 0.),
        ];
        let assert = vec![
            (1., 1.),
            (0., 0.),
            (1., 1.),
            (0., 0.),
            (0.75, 0.75),
            (0., 0.),
            (0.375, 0.375),
            (0., 0.),
            (0.25, 0.25),
            (0., 0.),
        ];
        delay(&mut vec, 0, 1, 2, 1., 1, 4, 1);
        assert_eq!(vec, assert);
    }
}
