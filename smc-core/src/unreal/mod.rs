//! This module is responsible for dealing with the specialized handling of Unreal Engine 4.27
//! .uasset files which contain `STLMeshCarver` instances in Deep Rock Galactic game files.
//!
//! There are two aspects to this:
//!
//! 1. The *generic* `.uasset` archive format, which we will learn from multiple sources, including
//!    [jorgenpt/uasset-rs][uasset], [UAssetAPI], [`PackageFileSummary.h`] and [`PackageFileSummary.cpp`].
//! 2. Specific `STLMeshCarver` instance (de)serialization format employed by DRG through Unreal's
//!    archive mechanisms.
//!
//! This module does *not* seek to replace [uasset], nor does it seek to achieve feature parity,
//! because this module is only intended to be responsible for handling Unreal 4.27 DRG
//! `STLMeshCarver` assets *specifically*. Unfortunately at the time of writing, [uasset] does not
//! have serialization support, so here we hand roll our version that supports both serialization
//! and deserialization for `STLMeshCarver`-containing `.uasset` files.
//!
//! [uasset]: https://github.com/jorgenpt/uasset-rs
//! [UAssetAPI]: https://github.com/atenfyr/UAssetAPI
//! [`PackageFileSummary.h`]: https://github.com/EpicGames/UnrealEngine/blob/4.27/Engine/Source/Runtime/CoreUObject/Public/UObject/PackageFileSummary.h
//! [`PackageFileSummary.cpp`]: https://github.com/EpicGames/UnrealEngine/blob/4.27/Engine/Source/Runtime/CoreUObject/Private/UObject/PackageFileSummary.cpp

pub mod stl_mesh_carver;
