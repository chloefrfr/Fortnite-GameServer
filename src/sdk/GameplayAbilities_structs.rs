#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayEffectGrantedAbilityRemovePolicy {
    CancelAbilityImmediately = 0,
    RemoveAbilityOnEnd = 1,
    DoNothing = 2,
    EGameplayEffectGrantedAbilityRemovePolicy_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayEffectAttributeCaptureSource {
    Source = 0,
    Target = 1,
    EGameplayEffectAttributeCaptureSource_MAX = 2,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayAbilityActivationMode {
    Authority = 0,
    NonAuthority = 1,
    Predicting = 2,
    Confirmed = 3,
    Rejected = 4,
    EGameplayAbilityActivationMode_MAX = 5,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EAbilityGenericReplicatedEvent {
    GenericConfirm = 0,
    GenericCancel = 1,
    InputPressed = 2,
    InputReleased = 3,
    GenericSignalFromClient = 4,
    GenericSignalFromServer = 5,
    GameCustom1 = 6,
    GameCustom2 = 7,
    GameCustom3 = 8,
    GameCustom4 = 9,
    GameCustom5 = 10,
    GameCustom6 = 11,
    MAX = 12,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayEffectReplicationMode {
    Minimal = 0,
    Mixed = 1,
    Full = 2,
    EGameplayEffectReplicationMode_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EAbilityTaskWaitState {
    WaitingOnGame = 1,
    WaitingOnUser = 2,
    WaitingOnAvatar = 4,
    EAbilityTaskWaitState_MAX = 5,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ERootMotionMoveToActorTargetOffsetType {
    AlignFromTargetToSource = 0,
    AlignToTargetForward = 1,
    AlignToWorldSpace = 2,
    ERootMotionMoveToActorTargetOffsetType_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EAbilityTaskNetSyncType {
    BothWait = 0,
    OnlyServerWait = 1,
    OnlyClientWait = 2,
    EAbilityTaskNetSyncType_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EWaitAttributeChangeComparison {
    None = 0,
    GreaterThan = 1,
    LessThan = 2,
    GreaterThanOrEqualTo = 3,
    LessThanOrEqualTo = 4,
    NotEqualTo = 5,
    ExactlyEqualTo = 6,
    MAX = 7,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayAbilityInputBinds {
    Ability1 = 0,
    Ability2 = 1,
    Ability3 = 2,
    Ability4 = 3,
    Ability5 = 4,
    Ability6 = 5,
    Ability7 = 6,
    Ability8 = 7,
    Ability9 = 8,
    EGameplayAbilityInputBinds_MAX = 9,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ETargetDataFilterSelf {
    TDFS_Any = 0,
    TDFS_NoSelf = 1,
    TDFS_NoOthers = 2,
    TDFS_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayAbilityTargetingLocationType {
    LiteralTransform = 0,
    ActorTransform = 1,
    SocketTransform = 2,
    EGameplayAbilityTargetingLocationType_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayTargetingConfirmation {
    Instant = 0,
    UserConfirmed = 1,
    Custom = 2,
    CustomMulti = 3,
    EGameplayTargetingConfirmation_MAX = 4,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayAbilityTriggerSource {
    GameplayEvent = 0,
    OwnedTagAdded = 1,
    OwnedTagPresent = 2,
    EGameplayAbilityTriggerSource_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayAbilityReplicationPolicy {
    ReplicateNo = 0,
    ReplicateYes = 1,
    EGameplayAbilityReplicationPolicy_MAX = 2,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayAbilityNetExecutionPolicy {
    LocalPredicted = 0,
    LocalOnly = 1,
    ServerInitiated = 2,
    ServerOnly = 3,
    EGameplayAbilityNetExecutionPolicy_MAX = 4,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayAbilityInstancingPolicy {
    NonInstanced = 0,
    InstancedPerActor = 1,
    InstancedPerExecution = 2,
    EGameplayAbilityInstancingPolicy_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayCuePayloadType {
    EffectContext = 0,
    CueParameters = 1,
    FromSpec = 2,
    EGameplayCuePayloadType_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayEffectStackingExpirationPolicy {
    ClearEntireStack = 0,
    RemoveSingleStackAndRefreshDuration = 1,
    RefreshDuration = 2,
    EGameplayEffectStackingExpirationPolicy_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayEffectStackingPeriodPolicy {
    ResetOnSuccessfulApplication = 0,
    NeverReset = 1,
    EGameplayEffectStackingPeriodPolicy_MAX = 2,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayEffectStackingDurationPolicy {
    RefreshOnSuccessfulApplication = 0,
    NeverRefresh = 1,
    EGameplayEffectStackingDurationPolicy_MAX = 2,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayEffectDurationType {
    Instant = 0,
    Infinite = 1,
    HasDuration = 2,
    EGameplayEffectDurationType_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EAttributeBasedFloatCalculationType {
    AttributeMagnitude = 0,
    AttributeBaseValue = 1,
    AttributeBonusMagnitude = 2,
    AttributeMagnitudeEvaluatedUpToChannel = 3,
    EAttributeBasedFloatCalculationType_MAX = 4,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayEffectMagnitudeCalculation {
    ScalableFloat = 0,
    AttributeBased = 1,
    CustomCalculationClass = 2,
    SetByCaller = 3,
    EGameplayEffectMagnitudeCalculation_MAX = 4,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayTagEventType {
    NewOrRemoved = 0,
    AnyCountChange = 1,
    EGameplayTagEventType_MAX = 2,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayCueEvent {
    OnActive = 0,
    WhileActive = 1,
    Executed = 2,
    Removed = 3,
    EGameplayCueEvent_MAX = 4,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayEffectStackingType {
    None = 0,
    AggregateBySource = 1,
    AggregateByTarget = 2,
    EGameplayEffectStackingType_MAX = 3,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayModOp {
    Additive = 0,
    Multiplicitive = 1,
    Division = 2,
    Override = 3,
    Max = 4,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum EGameplayModEvaluationChannel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4,
    Channel5 = 5,
    Channel6 = 6,
    Channel7 = 7,
    Channel8 = 8,
    Channel9 = 9,
    Channel_MAX = 10,
    EGameplayModEvaluationChannel_MAX = 11,
}

#[derive(Debug, Clone)]
pub struct FGameplayAttribute {
    pub attribute_name: String,
    pub attribute: Option<*mut u8>,
    pub attribute_owner: Option<*mut u8>,
}

#[derive(Debug, Clone)]
pub struct FGameplayEffectModifiedAttribute {
    pub attribute: FGameplayAttribute,
    pub total_magnitude: f32,
}

#[derive(Debug, Clone)]
pub struct FScalableFloat {
    pub value: f32,
    pub curve: Option<*mut u8>,
}

#[derive(Debug, Clone)]
pub struct FGameplayAbilitySpecHandle {
    pub handle: i32,
}

#[derive(Debug, Clone)]
pub struct FGameplayAbilitySpecDef {
    pub ability: Option<*mut u8>,
    pub level_scalable_float: FScalableFloat,
    pub input_id: i32,
    pub removal_policy: EGameplayEffectGrantedAbilityRemovePolicy,
    pub source_object: Option<*mut u8>,
    pub assigned_handle: FGameplayAbilitySpecHandle,
}

#[derive(Debug, Clone)]
pub struct FGameplayEffectAttributeCaptureDefinition {
    pub attribute_to_capture: FGameplayAttribute,
    pub attribute_source: EGameplayEffectAttributeCaptureSource,
    pub b_snapshot: bool,
}

#[derive(Debug, Clone)]
pub struct FGameplayEffectAttributeCaptureSpec {
    pub backing_definition: FGameplayEffectAttributeCaptureDefinition,
}

#[derive(Debug, Clone)]
pub struct FGameplayEffectAttributeCaptureSpecContainer {
    pub source_attributes: Vec<FGameplayEffectAttributeCaptureSpec>,
    pub target_attributes: Vec<FGameplayEffectAttributeCaptureSpec>,
    pub b_has_non_snapshotted_attributes: bool,
}

#[derive(Debug, Clone)]
pub struct FTagContainerAggregator {
    pub captured_actor_tags: FGameplayTagContainer,
    pub captured_spec_tags: FGameplayTagContainer,
    pub scoped_tags: FGameplayTagContainer,
}

#[derive(Debug, Clone)]
pub struct FModifierSpec {
    pub evaluated_magnitude: f32,
}

#[derive(Debug, Clone)]
pub struct FGameplayEffectContextHandle {
    pub _pad: [u8; 0x18],
}

#[derive(Debug, Clone)]
pub struct FGameplayEffectSpec {
    pub def: Option<*mut u8>,
    pub modified_attributes: Vec<FGameplayEffectModifiedAttribute>,
    pub captured_relevant_attributes: FGameplayEffectAttributeCaptureSpecContainer,
    pub duration: f32,
    pub period: f32,
    pub chance_to_apply_to_target: f32,
    pub captured_source_tags: FTagContainerAggregator,
    pub captured_target_tags: FTagContainerAggregator,
    pub dynamic_granted_tags: FGameplayTagContainer,
    pub dynamic_asset_tags: FGameplayTagContainer,
    pub modifiers: Vec<FModifierSpec>,
    pub stack_count: i32,
    pub b_completed_source_attribute_capture: bool,
    pub b_completed_target_attribute_capture: bool,
    pub b_duration_locked: bool,
    pub granted_ability_specs: Vec<FGameplayAbilitySpecDef>,
    pub effect_context: FGameplayEffectContextHandle,
    pub level: f32,
}

#[derive(Debug, Clone)]
pub struct FPredictionKey {
    pub current: i16,
    pub base: i16,
    pub predictive_connection: Option<*mut u8>,
    pub b_is_stale: bool,
    pub b_is_server_initiated: bool,
}

#[repr(C)]
pub struct FActiveGameplayEffect {
    _pad: [u8; 0xC],
    spec: FGameplayEffectSpec,           // 0x18(0x298)
    prediction_key: FPredictionKey,      // 0x2B0(0x18)
    start_server_world_time: f32,        // 0x2C8(0x4)
    cached_start_server_world_time: f32, // 0x2CC(0x4)
    start_world_time: f32,               // 0x2D0(0x4)
    b_is_inhibited: bool,                // 0x2D4(0x1)
    _pad2: [u8; 0x9B],
}

#[repr(C)]
pub struct FActiveGameplayEffectHandle {
    handle: i32,                             // 0x0(0x4)
    b_passed_filters_and_was_executed: bool, // 0x4(0x1)
    _pad: [u8; 0x3],
}

#[repr(C)]
pub struct FGameplayEffectSpecHandle {
    _pad: [u8; 0x10],
}

#[repr(C)]
pub struct FGameplayEffectRemovalInfo {
    b_premature_removal: bool, // 0x0(0x1)
    _pad: [u8; 0x3],
    stack_count: i32,                             // 0x4(0x4)
    effect_context: FGameplayEffectContextHandle, // 0x8(0x18)
}

#[repr(C)]
pub struct FGameplayAbilityTargetDataHandle {
    _pad: [u8; 0x20],
}

#[repr(C)]
pub struct FGameplayEventData {
    event_tag: FGameplayTag,                      // 0x0(0x8)
    instigator: Option<*mut AActor>,              // 0x8(0x8)
    target: Option<*mut AActor>,                  // 0x10(0x8)
    optional_object: Option<*mut UObject>,        // 0x18(0x8)
    optional_object2: Option<*mut UObject>,       // 0x20(0x8)
    context_handle: FGameplayEffectContextHandle, // 0x28(0x18)
    instigator_tags: FGameplayTagContainer,       // 0x40(0x20)
    target_tags: FGameplayTagContainer,           // 0x60(0x20)
    event_magnitude: f32,                         // 0x80(0x4)
    _pad: [u8; 0x4],
    target_data: FGameplayAbilityTargetDataHandle, // 0x88(0x20)
}

#[repr(C)]
pub struct FGameplayAbilityActivationInfo {
    activation_mode: EGameplayAbilityActivationMode, // 0x0(0x1)
    b_can_be_ended_by_other_instance: bool,          // Masked: 0x1
    _bit_pad: u8,                                    // 7 bits of padding
    _pad: [u8; 0x6],                                 // Padding after
    prediction_key_when_activated: FPredictionKey,   // 0x8(0x18)
}

#[repr(C)]
pub struct FGameplayAbilityActivationInfo {
    activation_mode: EGameplayAbilityActivationMode, // 0x0(0x1)
    b_can_be_ended_by_other_instance: bool,          // Masked: 0x1
    _bit_pad: u8,                                    // 7 bits of padding
    _pad: [u8; 0x6],                                 // Padding after
    prediction_key_when_activated: FPredictionKey,   // 0x8(0x18)
}

#[repr(C)]
pub struct FGameplayEffectQuery {
    _pad: [u8; 0x10],
    custom_match_delegate_bp: Option<*mut UDelegateProperty>, // 0x10(0x10)
    owning_tag_query: FGameplayTagQuery,                      // 0x20(0x48)
    effect_tag_query: FGameplayTagQuery,                      // 0x68(0x48)
    source_tag_query: FGameplayTagQuery,                      // 0xB0(0x48)
    modifying_attribute: FGameplayAttribute,                  // 0xF8(0x20)
    effect_source: Option<*mut UObject>,                      // 0x118(0x8)
    effect_definition: Option<*mut UGameplayEffect>,          // 0x120(0x8)
    _pad2: [u8; 0x10],
}

#[repr(C)]
pub struct FGameplayCueParameters {
    normalized_magnitude: f32,                             // 0x0(0x4)
    raw_magnitude: f32,                                    // 0x4(0x4)
    effect_context: FGameplayEffectContextHandle,          // 0x8(0x18)
    matched_tag_name: FGameplayTag,                        // 0x20(0x8)
    original_tag: FGameplayTag,                            // 0x28(0x8)
    aggregated_source_tags: FGameplayTagContainer,         // 0x30(0x20)
    aggregated_target_tags: FGameplayTagContainer,         // 0x50(0x20)
    location: FVector_NetQuantize10,                       // 0x70(0xC)
    normal: FVector_NetQuantizeNormal,                     // 0x7C(0xC)
    instigator: Option<*mut AActor>,                       // 0x88(0x8)
    effect_causer: Option<*mut AActor>,                    // 0x90(0x8)
    source_object: Option<*mut UObject>,                   // 0x98(0x8)
    physical_material: Option<*mut UPhysicalMaterial>,     // 0xA0(0x8)
    gameplay_effect_level: i32,                            // 0xA8(0x4)
    ability_level: i32,                                    // 0xAC(0x4)
    target_attach_component: Option<*mut USceneComponent>, // 0xB0(0x8)
}

#[repr(C)]
pub struct FGameplayEffectSpecForRPC {
    def: Option<*mut UGameplayEffect>, // 0x0(0x8)
    modified_attributes: Vec<FGameplayEffectModifiedAttribute>, // 0x8(0x10)
    effect_context: FGameplayEffectContextHandle, // 0x18(0x18)
    aggregated_source_tags: FGameplayTagContainer, // 0x30(0x20)
    aggregated_target_tags: FGameplayTagContainer, // 0x50(0x20)
    level: f32,                        // 0x70(0x4)
    ability_level: f32,                // 0x74(0x4)
}

#[repr(C)]
pub struct FServerAbilityRPCBatch {
    ability_spec_handle: FGameplayAbilitySpecHandle, // 0x0(0x4)
    _pad: [u8; 0x4],
    prediction_key: FPredictionKey,                // 0x8(0x18)
    target_data: FGameplayAbilityTargetDataHandle, // 0x20(0x20)
    input_pressed: bool,                           // 0x40(0x1)
    ended: bool,                                   // 0x41(0x1)
    started: bool,                                 // 0x42(0x1)
    _pad2: [u8; 0x5],
}

#[repr(C)]
pub struct FReplicatedPredictionKeyItem {
    _pad: [u8; 0x4],
    prediction_key: FPredictionKey, // 0x10(0x18)
}

#[repr(C)]
pub struct FReplicatedPredictionKeyMap {
    prediction_keys: Vec<FReplicatedPredictionKeyItem>, // 0xB0(0x10)
}

#[repr(C)]
pub struct FMinimalReplicationTagCountMap {
    pad_23b1: [u8; 0x50],
    owner: Option<*mut UAbilitySystemComponent>, // 0x50(0x8)(ExportObject, ZeroConstructor, InstancedReference, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    pad_23b2: [u8; 0x8],
}

#[repr(C)]
pub struct FActiveGameplayCue {
    pad_23b3: [u8; 0x4],
    gameplay_cue_tag: FGameplayTag, // 0x10(0x8)(NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    prediction_key: FPredictionKey, // 0x18(0x18)(NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    parameters: FGameplayCueParameters, // 0x30(0xB8)(ContainsInstancedReference, NativeAccessSpecifierPublic)
    b_predictively_removed: bool, // 0xE8(0x1)(ZeroConstructor, IsPlainOldData, RepSkip, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    pad_23b4: [u8; 0x7],
}

#[repr(C)]
pub struct FActiveGameplayCueContainer {
    gameplay_cues: Vec<FActiveGameplayCue>, // 0xB0(0x10)(ZeroConstructor, ContainsInstancedReference, NativeAccessSpecifierPublic)
    pad_23b5: [u8; 0x8],
    owner: Option<*mut UAbilitySystemComponent>, // 0xC8(0x8)(ExportObject, ZeroConstructor, InstancedReference, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPrivate)
}

#[repr(C)]
pub struct FActiveGameplayEffectsContainer {
    pad_23b6: [u8; 0x30],
    gameplay_effects_internal: Vec<FActiveGameplayEffect>, // 0xE0(0x10)(ZeroConstructor, NativeAccessSpecifierPrivate)
    pad_23b7: [u8; 0x310],
    application_immunity_query_effects: Vec<*mut UGameplayEffect>, // 0x400(0x10)(ZeroConstructor, NativeAccessSpecifierPrivate)
    pad_23b8: [u8; 0x18],
}

#[repr(C)]
pub struct FGameplayAbilityLocalAnimMontage {
    anim_montage: Option<*mut UAnimMontage>, // 0x0(0x8)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    play_bit: bool, // 0x8(0x1)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    pad_23b9: [u8; 0x7],
    prediction_key: FPredictionKey, // 0x10(0x18)(NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    animating_ability: Option<*mut UGameplayAbility>, // 0x28(0x8)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
}

#[repr(C)]
pub struct FGameplayAbilityRepAnimMontage {
    anim_montage: Option<*mut UAnimMontage>, // 0x0(0x8)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    play_rate: f32, // 0x8(0x4)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    position: f32, // 0xC(0x4)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    blend_time: f32, // 0x10(0x4)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    next_section_id: u8, // 0x14(0x1)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    is_stopped: u8, // Mask: 0x1, PropSize: 0x10x15(0x1)(NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    force_play_bit: u8, // Mask: 0x2, PropSize: 0x10x15(0x1)(NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    skip_position_correction: u8, // Mask: 0x4, PropSize: 0x10x15(0x1)(NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    b_skip_play_rate: u8, // Mask: 0x8, PropSize: 0x10x15(0x1)(NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    bit_pad_205: u8,
    pad_23ba: [u8; 0x2],
    prediction_key: FPredictionKey, // 0x18(0x18)(NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
}

#[repr(C)]
pub struct FGameplayAbilitySpec {
    handle: FGameplayAbilitySpecHandle, // 0xC(0x4)(NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    ability: Option<*mut UGameplayAbility>, // 0x10(0x8)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    level: i32, // 0x18(0x4)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    input_id: i32, // 0x1C(0x4)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    source_object: Option<*mut UObject>, // 0x20(0x8)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    active_count: u8, // 0x28(0x1)(ZeroConstructor, IsPlainOldData, RepSkip, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    input_pressed: u8, // Mask: 0x1, PropSize: 0x10x29(0x1)(RepSkip, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    remove_after_activation: u8, // Mask: 0x2, PropSize: 0x10x29(0x1)(RepSkip, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    pending_remove: u8, // Mask: 0x4, PropSize: 0x10x29(0x1)(RepSkip, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    bit_pad_206: u8,
    pad_23bb: [u8; 0x6],
    activation_info: FGameplayAbilityActivationInfo, // 0x30(0x20)(RepSkip, NoDestructor, NativeAccessSpecifierPublic)
    non_replicated_instances: Vec<Option<*mut UGameplayAbility>>, // 0x50(0x10)(ZeroConstructor, RepSkip, NativeAccessSpecifierPublic)
    replicated_instances: Vec<Option<*mut UGameplayAbility>>, // 0x60(0x10)(ZeroConstructor, NativeAccessSpecifierPublic)
    gameplay_effect_handle: FActiveGameplayEffectHandle, // 0x70(0x8)(RepSkip, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPublic)
    pad_23bc: [u8; 0xA0],
    owner: Option<*mut UAbilitySystemComponent>, // 0x10(0x8)(ZeroConstructor, IsPlainOldData, NoDestructor, HasGetValueTypeHash, NativeAccessSpecifierPrivate)
}

#[repr(C)]
pub struct FGameplayAbilityTargetingLocationInfo {
    _pad: [u8; 0x10],
    pub location_type: EGameplayAbilityTargetingLocationType, // 0x10 (0x1)
    _pad2: [u8; 0xF],
    pub literal_transform: FTransform,     // 0x20 (0x30)
    pub source_actor: Option<*mut AActor>, // 0x50 (0x8)
    pub source_component: Option<*mut UMeshComponent>, // 0x58 (0x8)
    pub source_ability: Option<*mut UGameplayAbility>, // 0x60 (0x8)
    pub source_socket_name: FName,         // 0x68 (0x8)
}

#[repr(C)]
pub struct FGameplayAbilityTargetData_ActorArray {
    _pad: [u8; 0x8],
    pub source_location: FGameplayAbilityTargetingLocationInfo, // 0x10 (0x70)
    pub target_actor_array: Vec<TWeakObjectPtr<AActor>>,        // 0x80 (0x10)
}

#[repr(C)]
pub struct FGameplayAbilityTargetData_LocationInfo {
    _pad: [u8; 0x8],
    pub source_location: FGameplayAbilityTargetingLocationInfo, // 0x10 (0x70)
    pub target_location: FGameplayAbilityTargetingLocationInfo, // 0x80 (0x70)
}

#[repr(C)]
pub struct FAbilityTaskDebugMessage {
    pub from_task: Option<*mut UGameplayTask>, // 0x0 (0x8)
    pub message: FString,                      // 0x8 (0x10)
}

#[repr(C)]
pub struct FAbilityEndedData {
    pub ability_that_ended: Option<*mut UGameplayAbility>, // 0x0 (0x8)
    pub ability_spec_handle: FGameplayAbilitySpecHandle,   // 0x8 (0x4)
    pub b_replicate_end_ability: bool,                     // 0xC (0x1)
    pub b_was_cancelled: bool,                             // 0xD (0x1)
    _pad: [u8; 0x2],
}

#[repr(C)]
pub struct FGameplayAbilitySpecHandleAndPredictionKey {
    pub ability_handle: FGameplayAbilitySpecHandle, // 0x0 (0x4)
    pub prediction_key_at_creation: i32,            // 0x4 (0x4)
}

#[repr(C)]
pub struct FGameplayAbilityActorInfo {
    _pad: [u8; 0x8],
    pub owner_actor: Option<TWeakObjectPtr<AActor>>, // 0x8 (0x8)
    pub avatar_actor: Option<TWeakObjectPtr<AActor>>, // 0x10 (0x8)
    pub player_controller: Option<TWeakObjectPtr<APlayerController>>, // 0x18 (0x8)
    pub ability_system_component: Option<TWeakObjectPtr<UAbilitySystemComponent>>, // 0x20 (0x8)
    pub skeletal_mesh_component: Option<TWeakObjectPtr<USkeletalMeshComponent>>, // 0x28 (0x8)
    pub anim_instance: Option<TWeakObjectPtr<UAnimInstance>>, // 0x30 (0x8)
    pub movement_component: Option<TWeakObjectPtr<UMovementComponent>>, // 0x38 (0x8)
}

#[repr(C)]
pub struct FWorldReticleParameters {
    pub aoe_scale: FVector, // 0x0 (0xC)
}
#[repr(C)]
pub struct FPreallocationInfo {
    _pad: [u8; 0x50],
    pub classes_needing_preallocation: Vec<TSubclassOf<AGameplayCueNotify_Actor>>, // 0x50 (0x10)
    _pad2: [u8; 0x8],
}

#[repr(C)]
pub struct FGameplayCuePendingExecute {
    _pad: [u8; 0x18],
    pub prediction_key: FPredictionKey,        // 0x18 (0x18)
    pub payload_type: EGameplayCuePayloadType, // 0x30 (0x1)
    _pad2: [u8; 0x7],
    pub owning_component: Option<*mut UAbilitySystemComponent>, // 0x38 (0x8)
    pub from_spec: FGameplayEffectSpecForRPC,                   // 0x40 (0x78)
    pub cue_parameters: FGameplayCueParameters,                 // 0xB8 (0xB8)
}

#[repr(C)]
pub struct FMinimalGameplayCueReplicationProxy {
    _pad: [u8; 0x1C0],
    pub owner: Option<*mut UAbilitySystemComponent>, // 0x1C0 (0x8)
    _pad2: [u8; 0x8],
}

#[repr(C)]
pub struct FGameplayCueTag {
    pub gameplay_cue_tag: FGameplayTag, // 0x0 (0x8)
}

#[repr(C)]
pub struct FGameplayCueObjectLibrary {
    pub paths: Vec<FString>, // 0x0 (0x10)
    _pad: [u8; 0x20],
    pub actor_object_library: Option<*mut UObjectLibrary>, // 0x30 (0x8)
    pub static_object_library: Option<*mut UObjectLibrary>, // 0x38 (0x8)
    pub cue_set: Option<*mut UGameplayCueSet>,             // 0x40 (0x8)
    _pad2: [u8; 0x4],
    pub b_should_sync_scan: bool,     // 0x4C (0x1)
    pub b_should_async_load: bool,    // 0x4D (0x1)
    pub b_should_sync_load: bool,     // 0x4E (0x1)
    pub b_has_been_initialized: bool, // 0x4F (0x1)
}

#[repr(C)]
pub struct FGameplayCueNotifyData {
    pub gameplay_cue_tag: FGameplayTag,                 // 0x0 (0x8)
    pub gameplay_cue_notify_obj: FSoftObjectPath,       // 0x8 (0x18)
    pub loaded_gameplay_cue_class: Option<*mut UClass>, // 0x20 (0x8)
    _pad: [u8; 0x8],
}

#[repr(C)]
pub struct FGameplayCueTranslationLink {
    pub rules_cdo: Option<*mut UGameplayCueTranslator>, // 0x0 (0x8)
    _pad: [u8; 0x10],
}

#[derive(Debug, Clone, Copy)]
pub struct GameplayCueTranslatorNodeIndex {
    pub index: i32,
}

#[derive(Debug, Clone)]
pub struct GameplayCueTranslatorNode {
    pub links: Vec<GameplayCueTranslationLink>,
    pub cached_index: GameplayCueTranslatorNodeIndex,
    pub cached_gameplay_tag: GameplayTag,
    pub cached_gameplay_tag_name: String,
}

#[derive(Debug, Clone)]
pub struct InheritedTagContainer {
    pub combined_tags: GameplayTagContainer,
    pub added: GameplayTagContainer,
    pub removed: GameplayTagContainer,
}

#[derive(Debug, Clone)]
pub struct GameplayTagContainer {
    pub tags: Vec<GameplayTag>,
}

#[derive(Debug, Clone)]
pub struct GameplayModEvaluationChannelSettings {
    pub channel: EGameplayModEvaluationChannel,
}

#[derive(Debug, Clone)]
pub struct CustomCalculationBasedFloat {
    pub calculation_class_magnitude: Option<String>,
    pub coefficient: ScalableFloat,
    pub pre_multiply_additive_value: ScalableFloat,
    pub post_multiply_additive_value: ScalableFloat,
    pub final_lookup_curve: Option<CurveTableRowHandle>,
}

#[derive(Debug, Clone)]
pub struct SetByCallerFloat {
    pub data_name: String,
    pub data_tag: GameplayTag,
}

#[derive(Debug, Clone)]
pub struct GameplayEffectModifierMagnitude {
    pub magnitude_calculation_type: EGameplayEffectMagnitudeCalculation,
    pub scalable_float_magnitude: FScalableFloat,
    pub attribute_based_magnitude: AttributeBasedFloat,
    pub custom_magnitude: CustomCalculationBasedFloat,
    pub set_by_caller_magnitude: SetByCallerFloat,
}

#[derive(Debug, Clone)]
pub struct GameplayModifierInfo {
    pub attribute: FGameplayAttribute,
    pub modifier_op: EGameplayModOp,
    pub magnitude: FScalableFloat,
    pub modifier_magnitude: FGameplayEffectModifierMagnitude,
    pub evaluation_channel_settings: FGameplayModEvaluationChannelSettings,
    _pad: [u8; 0x7],
    pub source_tags: FGameplayTagRequirements,
    pub target_tags: FGameplayTagRequirements,
}
