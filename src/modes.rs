//! This module contains the [`AFE4404`] lighting modes.

/// Uninitialized mode.
#[derive(Debug)]
pub struct UninitializedMode;

/// Three LEDs mode.
#[derive(Debug)]
pub struct ThreeLedsMode;

/// Two LEDs mode.
#[derive(Debug)]
pub struct TwoLedsMode;

/// Represents the lighting mode of the [`AFE4404`].
pub trait LedMode {}

impl LedMode for UninitializedMode {}
impl LedMode for ThreeLedsMode {}
impl LedMode for TwoLedsMode {}
