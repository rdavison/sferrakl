use crate::finger::Finger;
use crate::hand_finger::HandFinger;
use crate::key::Key;
use std::collections::HashSet;

pub type Fingering = Vec<HandFinger>;

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
    hf1: &HandFinger,
    hf2: &HandFinger,
) -> RowChangeType {
    use RowChangeType::*;
    let HandFinger((hand1, finger1)) = hf1;
    let HandFinger((hand2, finger2)) = hf2;
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
        key1, key2, hf1, hf2
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

fn combine(f1: Fingering, f2: Fingering, s1: &[Key], s2: &[Key]) -> Fingering {
    if f1.is_empty() {
        return f2;
    }

    if f2.is_empty() {
        return f1;
    }

    let last_hf_f1 = f1.last().unwrap();

    let first_hf_f2 = f2.first().unwrap();

    if last_hf_f1 != first_hf_f2 {
        return [f1, f2].concat();
    }

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

        return [f1, new_f2].concat();
    }

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
                return [f1, f2].concat();
            }
        }

        let mut new_f1 = f1.clone();

        new_f1[f1.len() - 1] = HandFinger((key1.hand, *alt));

        return [new_f1, f2].concat();
    }

    // Cannot resolve
    [f1, f2].concat()
}

fn determine_fingering(stroke: &[Key]) -> Fingering {
    match stroke.len() {
        0 => vec![],
        1 => vec![HandFinger((stroke[0].hand, stroke[0].finger))],
        2 => {
            let key1 = &stroke[0];
            let key2 = &stroke[1];
            let hf1 = HandFinger((key1.hand, key1.finger));
            let hf2 = HandFinger((key2.hand, key2.finger));

            // No SFB, use default fingering
            if hf1 != hf2 {
                return vec![hf1, hf2];
            }

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
                return fingering_vec.clone();
            }

            // Cannot resolve. Return the SFB.
            vec![hf1, hf2]
        }

        n => {
            let (s1, s2_slice) = stroke.split_at(n - 1);
            let f1 = determine_fingering(s1);
            let f2 = determine_fingering(s2_slice);
            combine(f1, f2, s1, s2_slice)
        }
    }
}

// Examples of str
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
    F,
}

pub fn assign_tier(stroke: &[Key]) -> Option<Tier> {
    if stroke.len() < 2 {
        return None;
    }

    let fingering = determine_fingering(stroke);

    // S-tier
    if is_same_row(stroke) {
        let mut unique_fingers = HashSet::new();
        if fingering.iter().all(|f| unique_fingers.insert(f)) {
            if stroke.len() >= 3 {
                if fingering.iter().any(|hf_item| {
                    let HandFinger((_, finger)) = hf_item;
                    finger == &Finger::I
                }) {
                    return Some(Tier::S);
                }
            } else {
                return Some(Tier::S);
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

    let has_good = changes.iter().any(|c| *c == RowChangeType::Good);
    let has_wide_good = changes.iter().any(|c| *c == RowChangeType::WideGood);
    let has_weak_scissor = changes.iter().any(|c| *c == RowChangeType::WeakScissor);
    let has_bad_scissor = changes.iter().any(|c| *c == RowChangeType::BadScissor);
    let has_same_finger = changes
        .iter()
        .any(|c| matches!(c, RowChangeType::SameFinger(_))); // New check

    if has_same_finger {
        return Some(Tier::F); // Penalize SFB heavily
    }

    if has_bad_scissor {
        if is_weak {
            return Some(Tier::F);
        } else {
            return Some(Tier::D);
        }
    }

    if has_weak_scissor {
        if is_weak {
            return Some(Tier::D);
        } else {
            return Some(Tier::C);
        }
    }

    if has_wide_good {
        if is_weak {
            return Some(Tier::C);
        } else {
            return Some(Tier::B);
        }
    }

    if has_good {
        if is_weak {
            return Some(Tier::B);
        } else {
            return Some(Tier::A);
        }
    }

    panic!(
        "Unhandled stroke: {:?}",
        stroke.iter().map(|k| k.code).collect::<Vec<_>>()
    );
}
