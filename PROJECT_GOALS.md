# The goals of the AGE engine
> Even though this is a small project, I think it's good to know what do you actually want to achive

## Core goals
- Providing a simple game engine with support for:
  - asset management
  - 2d (and maybe 3d) graphics rendering on a native backend as well as graphics manipulation
  - audio input, output and manipulation
  - some extra utility like maths
  - maybe more, idk
- Quickstart capability
- Extensive options for those wanting to customize everything
- Taking advantage of Rust features like `Option<T>`, `Result<T,E>` or `Traits`
- Cross-platform capabilities (If the users doesn't tell the engine anything about the platform, it should be possible to get it working on all supported platforms)
- Grouping optional features provided by wgpu by backends to make it easier to take advantage of their full features
- Supported platforms
  - Vulkan
  - DirectX 12
  - Metal
  - **No** OpenGL

## Opional goals
- Having bindings fo other languages

## More coming soon!
