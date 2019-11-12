use ash;
use ash::vk;
use glfw;
use vulkan_state::VulkanState;

use ash::extensions::khr::XlibSurface;
use ash::extensions::nv::RayTracing;
use ash::extensions::nv;

fn main() {
    let global_state = VulkanState::new((800, 600));
}
