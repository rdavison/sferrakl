use crate::finger::Finger;
use crate::hand_finger::HandFinger;
use crate::key::Key;
use std::collections::HashSet;

pub type Fingering = Vec<HandFinger>;

#[derive(Debug)]
struct FingeredStroke<'a> {
    stroke: &'a [Key],
    fingering: Fingering,
}

#[derive(Debug)]
enum RowChangeType {
    Good,
    WideGood,
    WeakScissor,
    BadScissor,
    SameFinger(f64),
    None,
}

impl PartialEq for RowChangeType {
    fn eq(&self, other: &Self) -> bool {
        use RowChangeType::*;
        match (self, other) {
            (Good, Good) => true,
            (WideGood, WideGood) => true,
            (WeakScissor, WeakScissor) => true,
            (BadScissor, BadScissor) => true,
            (SameFinger(d1), SameFinger(d2)) => (d1 - d2).abs() < f64::EPSILON, // Compare f64 with tolerance
            (None, None) => true,
            _ => false,
        }
    }
}
impl Eq for RowChangeType {} // Can implement Eq because PartialEq is implemented with tolerance for f64

fn classify_row_change(
    key1: &Key,
    key2: &Key,
    HandFinger((hand1, finger1)): &HandFinger,
    HandFinger((hand2, finger2)): &HandFinger,
) -> RowChangeType {
    use RowChangeType::*;
    let row1 = key1.row;
    let row2 = key2.row;
    let row_diff = (row2 as i8) - (row1 as i8);

    // Same finger, calculate travel distance
    if finger1 == finger2 {
        let distance = (key1.row as f64 - key2.row as f64).abs(); // Simple row distance
        return SameFinger(distance);
    }

    // Different hands
    if hand1 != hand2 {
        return None;
    }

    // Same row, handled by S-tier
    if row_diff == 0 {
        return None;
    }

    let is_short_finger1 = matches!(finger1, Finger::P | Finger::I);
    let is_long_finger1 = matches!(finger1, Finger::R | Finger::M);

    if row_diff.abs() == 1 {
        // Normal row change
        // curl
        if is_short_finger1 && row_diff > 0 {
            return Good;
        }
        // stretch
        if is_long_finger1 && row_diff < 0 {
            return Good;
        }
        // anti-curl
        if is_short_finger1 && row_diff < 0 {
            return WeakScissor;
        }
        // anti-stretch
        if is_long_finger1 && row_diff > 0 {
            return WeakScissor;
        }
    } else {
        // Wide or bad row change
        // wide curl
        if is_short_finger1 && row_diff > 0 {
            return WideGood;
        }
        // wide stretch
        if is_long_finger1 && row_diff < 0 {
            return WideGood;
        }
        // bad anti-curl
        if is_short_finger1 && row_diff < 0 {
            return BadScissor;
        }
        // bad anti-stretch
        if is_long_finger1 && row_diff > 0 {
            return BadScissor;
        }
    }

    panic!(
        "Failed to classify row change for keys: {:?} -> {:?} with fingerings: {} -> {}",
        key1,
        key2,
        HandFinger((*hand1, *finger1)),
        HandFinger((*hand2, *finger2))
    );
}

fn is_weak_finger_stroke(fingering: &Fingering) -> bool {
    fingering.iter().all(|hf| {
        let HandFinger((_, finger)) = hf;
        matches!(finger, Finger::P | Finger::R)
    })
}

fn is_same_row(stroke: &[Key]) -> bool {
    if stroke.len() < 2 {
        return true;
    }

    let first_row = stroke[0].row;

    stroke.iter().all(|k| k.row == first_row)
}

fn combine<'a>(fs1: FingeredStroke<'a>, fs2: FingeredStroke<'a>) -> FingeredStroke<'a> {
    let f1 = fs1.fingering;
    let s1 = fs1.stroke;
    let f2 = fs2.fingering;
    let s2 = fs2.stroke;

    let fingering = if f1.is_empty() {
        f2
    } else if f2.is_empty() {
        f1
    } else {
        let last_hf_f1 = f1
            .last()
            .expect("f1 should not be empty due to prior check");

        let first_hf_f2 = f2
            .first()
            .expect("f2 should not be empty due to prior check");

        if last_hf_f1 != first_hf_f2 {
            [f1, f2].concat()
        } else {
            // SFB detected between the two strokes.
            let key1 = &s1[s1.len() - 1];
            let key2 = &s2[0];

            // Option 1: Try to resolve by changing key2's finger
            let mut candidates2 = key2
                .id
                .alternate_fingers()
                .into_iter()
                .filter(|alt| HandFinger((key2.hand, *alt)) != *last_hf_f1)
                .collect::<Vec<_>>();

            candidates2.sort_by_key(|a| !a.is_strong());

            if let Some(alt) = candidates2.first() {
                let mut new_f2 = f2.clone();
                new_f2[0] = HandFinger((key2.hand, *alt));
                [f1, new_f2].concat()
            } else {
                // Option 2: Try to resolve by changing key1's finger
                let mut candidates1 = key1
                    .id
                    .alternate_fingers()
                    .into_iter()
                    .filter(|alt| HandFinger((key1.hand, *alt)) != *first_hf_f2)
                    .collect::<Vec<_>>();

                candidates1.sort_by_key(|a| !a.is_strong());

                if let Some(alt) = candidates1.first() {
                    if f1.len() > 1 {
                        // this alt creates a new SFB
                        if f1[f1.len() - 2] == HandFinger((key1.hand, *alt)) {
                            // Cannot resolve
                            [f1, f2].concat()
                        } else {
                            let mut new_f1 = f1.clone();
                            new_f1[f1.len() - 1] = HandFinger((key1.hand, *alt));
                            [new_f1, f2].concat()
                        }
                    } else {
                        let mut new_f1 = f1.clone();
                        new_f1[f1.len() - 1] = HandFinger((key1.hand, *alt));
                        [new_f1, f2].concat()
                    }
                } else {
                    // Cannot resolve
                    [f1, f2].concat()
                }
            }
        }
    };

    // This is safe because this function is only called from `determine_fingering`
    // with adjacent slices from `split_at`, which guarantees they are contiguous.
    let stroke = unsafe {
        std::slice::from_raw_parts(fs1.stroke.as_ptr(), fs1.stroke.len() + fs2.stroke.len())
    };

    FingeredStroke { stroke, fingering }
}

fn determine_fingering<'a>(stroke: &'a [Key]) -> FingeredStroke<'a> {
    match stroke.len() {
        0 => FingeredStroke {
            stroke,
            fingering: vec![],
        },
        1 => FingeredStroke {
            stroke,
            fingering: vec![HandFinger((stroke[0].hand, stroke[0].finger))],
        },
        2 => {
            let fingering = {
                let key1 = &stroke[0];
                let key2 = &stroke[1];
                let hf1 = HandFinger((key1.hand, key1.finger));
                let hf2 = HandFinger((key2.hand, key2.finger));

                // No SFB, use default fingering
                if hf1 != hf2 {
                    vec![hf1, hf2]
                } else {
                    // SFB detected, try to resolve it.
                    let alt_fingers1 = key1.id.alternate_fingers();
                    let alt_fingers2 = key2.id.alternate_fingers();

                    // Create a list of candidates
                    let mut candidates = vec![];

                    // Add resolutions by changing finger for key2
                    for &alt_finger in &alt_fingers2 {
                        let new_hf2 = HandFinger((key2.hand, alt_finger));
                        if hf1 != new_hf2 {
                            candidates.push((vec![hf1, new_hf2], alt_finger.is_strong()));
                        }
                    }

                    // Add resolutions by changing finger for key1
                    for &alt_finger in &alt_fingers1 {
                        let new_hf1 = HandFinger((key1.hand, alt_finger));
                        if new_hf1 != hf2 {
                            candidates.push((vec![new_hf1, hf2], alt_finger.is_strong()));
                        }
                    }

                    // Sort candidates: prefer strong fingers
                    candidates.sort_by_key(|(_, is_strong)| !is_strong);

                    if let Some((fingering_vec, _)) = candidates.first() {
                        fingering_vec.clone()
                    } else {
                        // Cannot resolve. Return the SFB.
                        vec![hf1, hf2]
                    }
                }
            };
            FingeredStroke { stroke, fingering }
        }
        n => {
            let (s1, s2_slice) = stroke.split_at(n - 1);
            let fs1 = determine_fingering(s1);
            let fs2 = determine_fingering(s2_slice);
            combine(fs1, fs2)
        }
    }
}

use core::percentage::T as Percentage;

fn get_same_finger_tier(key: &Key) -> Percentage {
    use crate::finger::Finger;

    let finger_score = match key.finger {
        Finger::M => 0, // Middle (best)
        Finger::I => 1, // Index
        Finger::R => 2, // Ring
        Finger::P => 3, // Pinky (worst for repeats)
        _ => 5,         // Thumb or other - higher penalty
    };

    let row_score = match key.row {
        3 => 0, // ASDFG (Homerow - best)
        2 => 1, // QWERTY (Top letter row)
        1 => 1, // Number row (Treat same as QWERTY for now)
        4 => 2, // ZXCVB (Bottom letter row - worst)
        _ => 3, // Other rows (Function/Spacebar row)
    };

    let combined_score = finger_score + row_score;

    match combined_score {
        0 => Percentage::new(0.8).unwrap(), // Middle, Homerow
        1 => Percentage::new(0.6).unwrap(), // Index, Homerow OR Middle, Top row
        2 => Percentage::new(0.4).unwrap(), // Ring, Homerow OR Index, Top row OR Middle, Bottom row
        3 => Percentage::new(0.2).unwrap(), // Pinky, Homerow OR Ring, Top row OR Index, Bottom row OR Middle, Other row
        4 => Percentage::new(0.2).unwrap(), // Pinky, Top row OR Index, Bottom row OR Ring, Other row
        _ => Percentage::new(0.0).unwrap(), // Anything worse
    }
}

pub fn assign_tier(stroke: &[Key]) -> Option<Percentage> {
    if stroke.len() < 2 {
        return None;
    }

    let fingering = determine_fingering(stroke).fingering;

    // S-tier
    if is_same_row(stroke) {
        let mut unique_fingers = HashSet::new();
        if fingering.iter().all(|f| unique_fingers.insert(f)) {
            if stroke.len() >= 3 {
                if fingering.iter().any(|hf_item| {
                    let HandFinger((_, finger)) = hf_item;
                    finger == &Finger::I
                }) {
                    return Some(Percentage::new(1.0).unwrap());
                }
            } else {
                return Some(Percentage::new(1.0).unwrap());
            }
        }
    }

    let mut changes = vec![];
    for i in 0..(stroke.len() - 1) {
        changes.push(classify_row_change(
            &stroke[i],
            &stroke[i + 1],
            &fingering[i],
            &fingering[i + 1],
        ));
    }

    let is_weak = is_weak_finger_stroke(&fingering);

    let mut direct_repeat_tier: Option<Percentage> = None;
    for (i, change) in changes.iter().enumerate() {
        if let RowChangeType::SameFinger(distance) = change {
            if *distance == 0.0 {
                direct_repeat_tier = Some(get_same_finger_tier(&stroke[i]));
                break;
            }
        }
    }
    if let Some(tier) = direct_repeat_tier {
        return Some(tier);
    }

    // Now, check for other SameFinger (distance > 0) - these are penalized heavily as F
    if changes
        .iter()
        .any(|c| matches!(c, RowChangeType::SameFinger(_)))
    {
        return Some(Percentage::new(0.0).unwrap());
    }

    let has_good = changes.iter().any(|c| *c == RowChangeType::Good);
    let has_wide_good = changes.iter().any(|c| *c == RowChangeType::WideGood);
    let has_weak_scissor = changes.iter().any(|c| *c == RowChangeType::WeakScissor);
    let has_bad_scissor = changes.iter().any(|c| *c == RowChangeType::BadScissor);

    if has_bad_scissor {
        if is_weak {
            return Some(Percentage::new(0.0).unwrap());
        } else {
            return Some(Percentage::new(0.2).unwrap());
        }
    }

    if has_weak_scissor {
        if is_weak {
            return Some(Percentage::new(0.2).unwrap());
        } else {
            return Some(Percentage::new(0.4).unwrap());
        }
    }

    if has_wide_good {
        if is_weak {
            return Some(Percentage::new(0.4).unwrap());
        } else {
            return Some(Percentage::new(0.6).unwrap());
        }
    }

    if has_good {
        if is_weak {
            return Some(Percentage::new(0.6).unwrap());
        } else {
            return Some(Percentage::new(0.8).unwrap());
        }
    }

    if changes.iter().all(|c| matches!(*c, RowChangeType::None)) {
        // If all transitions are alternate-hand (None), consider it a good stroke.
        // It's not S-tier (same row), but it's not penalized for same finger or bad scissors.
        return Some(Percentage::new(0.8).unwrap());
    }

    panic!(
        "Unhandled stroke: {:?}",
        stroke.iter().map(|k| k.code).collect::<Vec<_>>()
    );
}
