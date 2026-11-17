// Integration test for SigmaOS-arch-shader-accountable-learning-kernel
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use arch_shader_accountable_learning_kernel as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
