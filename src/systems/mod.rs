//! ECS systems.
//!
//! Define reusable systems here. Example:
//!
//! ```ignore
//! pub fn apply_velocity(
//!     time: Res<Time>,
//!     mut query: Query<(&Velocity, &mut Transform)>,
//! ) {
//!     for (velocity, mut transform) in &mut query {
//!         transform.translation += velocity.0.extend(0.0) * time.delta_secs();
//!     }
//! }
//! ```
