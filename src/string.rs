use crate::sequence::{SequenceModificationDelta, SequenceModificationDeltaOwned};
use crate::{Diffable, MacroOptimizationHints, Patchable};

impl<'d> Diffable<'d, String> for String {
    type Delta = Vec<SequenceModificationDelta<'d, u8>>;
    type DeltaOwned = Vec<SequenceModificationDeltaOwned<u8>>;

    fn create_delta_towards(&self, end_state: &'d String) -> (Self::Delta, MacroOptimizationHints) {
        self.as_bytes().create_delta_towards(&end_state.as_bytes())
    }
}

impl Patchable<Vec<SequenceModificationDeltaOwned<u8>>> for String {
    fn apply_patch(&mut self, patch: Vec<SequenceModificationDeltaOwned<u8>>) {
        // TODO: More efficient implementation without copying.. Just quickly getting things working.
        let mut bytes = self.as_bytes().to_vec();

        bytes.apply_patch(patch);

        *self = String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sequence::SequenceModificationDelta;
    use crate::DiffPatchTestCase;

    /// Verify that we can diff and patch strings.
    #[test]
    fn string_dipa() {
        DiffPatchTestCase {
            label: Some("String unchanged"),
            start: "XYZ".to_string(),
            end: &"XYZ".to_string(),
            expected_delta: vec![],
            // 1 for vec length
            expected_serialized_patch_size: 1,
            expected_macro_hints: MacroOptimizationHints { did_change: false },
        }
        .test();

        DiffPatchTestCase {
            label: Some("String changed"),
            start: "ABCDE".to_string(),
            end: &"ABDE".to_string(),
            expected_delta: vec![SequenceModificationDelta::DeleteOne { index: 2 }],
            // 1 for vec length, 1 for variant, 1 for index
            expected_serialized_patch_size: 3,
            expected_macro_hints: MacroOptimizationHints { did_change: true },
        }
        .test();
    }
}