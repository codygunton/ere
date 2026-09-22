use std::env;

pub const ERE_IMAGE_REGISTRY: &str = "ERE_IMAGE_REGISTRY";
pub const ERE_FORCE_REBUILD_DOCKER_IMAGE: &str = "ERE_FORCE_REBUILD_DOCKER_IMAGE";
pub const ERE_GPU_DEVICES: &str = "ERE_GPU_DEVICES";
pub const ERE_DOCKER_NETWORK: &str = "ERE_DOCKER_NETWORK";
pub const ERE_ZISK_PROVING_KEY_VOLUME: &str = "ERE_ZISK_PROVING_KEY_VOLUME";
pub const ERE_IMAGE_TAG: &str = "ERE_IMAGE_TAG";
pub const ERE_DOCKER_MEMORY: &str = "ERE_DOCKER_MEMORY";

/// Returns image registry from env variable `ERE_IMAGE_REGISTRY`.
///
/// If env varialbe is valid, it will be prepended to all images. For example
/// if `ERE_IMAGE_REGISTRY=ghcr.io/eth-act/ere`, the [`base_image`] will return
/// `ghcr.io/eth-act/ere/ere-base:{image_tag}`.
///
/// [`base_image`]: crate::image::base_image
pub fn image_registry() -> Option<String> {
    env::var(ERE_IMAGE_REGISTRY).ok()
}

/// Returns whether env variable `ERE_FORCE_REBUILD_DOCKER_IMAGE` is set or not.
pub fn force_rebuild_docker_image() -> bool {
    env::var_os(ERE_FORCE_REBUILD_DOCKER_IMAGE).is_some()
}

/// Returns env variable `ERE_GPU_DEVICES`.
pub fn gpu_devices() -> Option<String> {
    env::var(ERE_GPU_DEVICES).ok()
}

/// Returns env variable `ERE_DOCKER_NETWORK`.
pub fn docker_network() -> Option<String> {
    env::var(ERE_DOCKER_NETWORK).ok()
}

/// Returns image tag from env variable `ERE_IMAGE_TAG`.
///
/// If set, it replaces [`DOCKER_IMAGE_TAG`] for all images, e.g. to run the
/// images published for another commit or release.
///
/// [`DOCKER_IMAGE_TAG`]: crate::DOCKER_IMAGE_TAG
pub fn image_tag_override() -> Option<String> {
    env::var(ERE_IMAGE_TAG).ok()
}

/// Returns env variable `ERE_DOCKER_MEMORY`, the memory limit of spawned
/// `ere-server-*` containers (passed to Docker's `--memory` flag).
pub fn docker_memory() -> Option<String> {
    env::var(ERE_DOCKER_MEMORY).ok()
}
