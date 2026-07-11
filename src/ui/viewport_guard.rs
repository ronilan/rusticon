use crate::core::model::State;
use incredible::*;
use incredible_elements::ViewportGuard;

pub fn build() -> ViewportGuard<State> {
    let guard = ViewportGuard::default();
    guard.min_width(80).min_height(24).showed(false).fused(true);

    guard
}
