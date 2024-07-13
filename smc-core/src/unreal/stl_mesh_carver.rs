//! An [`STLMeshCarver`] looks like the following (C++ signature found in C++ modding templates):
//!
//! ```c++
//! UCLASS(Blueprintable)
//! class FSD_API USTLMeshCarver : public UObject {
//!     GENERATED_BODY()
//! public:
//!     UPROPERTY(BlueprintReadWrite, EditAnywhere, meta=(AllowPrivateAccess=true))
//!     FBox AABB;
//!     
//!     UPROPERTY(BlueprintReadWrite, EditAnywhere, meta=(AllowPrivateAccess=true))
//!     FDeepCSGFloatTree BSPTree;
//!     
//!     UPROPERTY(BlueprintReadWrite, EditAnywhere, meta=(AllowPrivateAccess=true))
//!     TArray<FVector> Vertices;
//!     
//!     USTLMeshCarver();
//! 
//! };
//! ```
