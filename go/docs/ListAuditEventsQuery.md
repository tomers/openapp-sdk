# ListAuditEventsQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ActorGuestId** | Pointer to **NullableString** | Filter to a single guest fingerprint. Disjoint from &#x60;actor_user_id&#x60;. | [optional]
**ActorKind** | Pointer to **NullableString** |  | [optional]
**ActorUserId** | Pointer to **NullableString** |  | [optional]
**CorrelationId** | Pointer to **NullableString** |  | [optional]
**Cursor** | Pointer to **NullableString** | Opaque cursor from a previous page&#39;s &#x60;next_cursor&#x60;. | [optional]
**DoorName** | Pointer to **NullableString** | Case-insensitive substring match on the snapshotted door name (&#x60;details.door_name&#x60;), e.g. &#x60;lobby&#x60;. Lets the Activity table answer \&quot;who opened this door\&quot; across every event that names it. | [optional]
**EventType** | Pointer to **NullableString** |  | [optional]
**Limit** | Pointer to **NullableInt32** | Page size (default 50, max 200). | [optional]
**OccurredAfter** | Pointer to **NullableString** | RFC 3339 lower bound (inclusive). Must be within the 30-day hot window. | [optional]
**OccurredBefore** | Pointer to **NullableString** | RFC 3339 upper bound (inclusive). | [optional]
**OrgScope** | Pointer to **NullableString** | Organization scope for resource events. Descendants are included by default. | [optional]
**Outcome** | Pointer to **NullableString** |  | [optional]
**ResourceId** | Pointer to **NullableString** |  | [optional]
**ResourceName** | Pointer to **NullableString** | Case-insensitive substring match on the resource&#39;s snapshotted display name (&#x60;details.resource_name&#x60;), e.g. &#x60;shelly&#x60;. | [optional]
**ResourceType** | Pointer to **NullableString** |  | [optional]
**TargetId** | Pointer to **NullableString** | Exact match against the snapshotted portal target ID (&#x60;details.public_portal_id&#x60;). | [optional]

## Methods

### NewListAuditEventsQuery

`func NewListAuditEventsQuery() *ListAuditEventsQuery`

NewListAuditEventsQuery instantiates a new ListAuditEventsQuery object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListAuditEventsQueryWithDefaults

`func NewListAuditEventsQueryWithDefaults() *ListAuditEventsQuery`

NewListAuditEventsQueryWithDefaults instantiates a new ListAuditEventsQuery object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetActorGuestId

`func (o *ListAuditEventsQuery) GetActorGuestId() string`

GetActorGuestId returns the ActorGuestId field if non-nil, zero value otherwise.

### GetActorGuestIdOk

`func (o *ListAuditEventsQuery) GetActorGuestIdOk() (*string, bool)`

GetActorGuestIdOk returns a tuple with the ActorGuestId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorGuestId

`func (o *ListAuditEventsQuery) SetActorGuestId(v string)`

SetActorGuestId sets ActorGuestId field to given value.

### HasActorGuestId

`func (o *ListAuditEventsQuery) HasActorGuestId() bool`

HasActorGuestId returns a boolean if a field has been set.

### SetActorGuestIdNil

`func (o *ListAuditEventsQuery) SetActorGuestIdNil(b bool)`

 SetActorGuestIdNil sets the value for ActorGuestId to be an explicit nil

### UnsetActorGuestId
`func (o *ListAuditEventsQuery) UnsetActorGuestId()`

UnsetActorGuestId ensures that no value is present for ActorGuestId, not even an explicit nil
### GetActorKind

`func (o *ListAuditEventsQuery) GetActorKind() string`

GetActorKind returns the ActorKind field if non-nil, zero value otherwise.

### GetActorKindOk

`func (o *ListAuditEventsQuery) GetActorKindOk() (*string, bool)`

GetActorKindOk returns a tuple with the ActorKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorKind

`func (o *ListAuditEventsQuery) SetActorKind(v string)`

SetActorKind sets ActorKind field to given value.

### HasActorKind

`func (o *ListAuditEventsQuery) HasActorKind() bool`

HasActorKind returns a boolean if a field has been set.

### SetActorKindNil

`func (o *ListAuditEventsQuery) SetActorKindNil(b bool)`

 SetActorKindNil sets the value for ActorKind to be an explicit nil

### UnsetActorKind
`func (o *ListAuditEventsQuery) UnsetActorKind()`

UnsetActorKind ensures that no value is present for ActorKind, not even an explicit nil
### GetActorUserId

`func (o *ListAuditEventsQuery) GetActorUserId() string`

GetActorUserId returns the ActorUserId field if non-nil, zero value otherwise.

### GetActorUserIdOk

`func (o *ListAuditEventsQuery) GetActorUserIdOk() (*string, bool)`

GetActorUserIdOk returns a tuple with the ActorUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorUserId

`func (o *ListAuditEventsQuery) SetActorUserId(v string)`

SetActorUserId sets ActorUserId field to given value.

### HasActorUserId

`func (o *ListAuditEventsQuery) HasActorUserId() bool`

HasActorUserId returns a boolean if a field has been set.

### SetActorUserIdNil

`func (o *ListAuditEventsQuery) SetActorUserIdNil(b bool)`

 SetActorUserIdNil sets the value for ActorUserId to be an explicit nil

### UnsetActorUserId
`func (o *ListAuditEventsQuery) UnsetActorUserId()`

UnsetActorUserId ensures that no value is present for ActorUserId, not even an explicit nil
### GetCorrelationId

`func (o *ListAuditEventsQuery) GetCorrelationId() string`

GetCorrelationId returns the CorrelationId field if non-nil, zero value otherwise.

### GetCorrelationIdOk

`func (o *ListAuditEventsQuery) GetCorrelationIdOk() (*string, bool)`

GetCorrelationIdOk returns a tuple with the CorrelationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCorrelationId

`func (o *ListAuditEventsQuery) SetCorrelationId(v string)`

SetCorrelationId sets CorrelationId field to given value.

### HasCorrelationId

`func (o *ListAuditEventsQuery) HasCorrelationId() bool`

HasCorrelationId returns a boolean if a field has been set.

### SetCorrelationIdNil

`func (o *ListAuditEventsQuery) SetCorrelationIdNil(b bool)`

 SetCorrelationIdNil sets the value for CorrelationId to be an explicit nil

### UnsetCorrelationId
`func (o *ListAuditEventsQuery) UnsetCorrelationId()`

UnsetCorrelationId ensures that no value is present for CorrelationId, not even an explicit nil
### GetCursor

`func (o *ListAuditEventsQuery) GetCursor() string`

GetCursor returns the Cursor field if non-nil, zero value otherwise.

### GetCursorOk

`func (o *ListAuditEventsQuery) GetCursorOk() (*string, bool)`

GetCursorOk returns a tuple with the Cursor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCursor

`func (o *ListAuditEventsQuery) SetCursor(v string)`

SetCursor sets Cursor field to given value.

### HasCursor

`func (o *ListAuditEventsQuery) HasCursor() bool`

HasCursor returns a boolean if a field has been set.

### SetCursorNil

`func (o *ListAuditEventsQuery) SetCursorNil(b bool)`

 SetCursorNil sets the value for Cursor to be an explicit nil

### UnsetCursor
`func (o *ListAuditEventsQuery) UnsetCursor()`

UnsetCursor ensures that no value is present for Cursor, not even an explicit nil
### GetDoorName

`func (o *ListAuditEventsQuery) GetDoorName() string`

GetDoorName returns the DoorName field if non-nil, zero value otherwise.

### GetDoorNameOk

`func (o *ListAuditEventsQuery) GetDoorNameOk() (*string, bool)`

GetDoorNameOk returns a tuple with the DoorName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorName

`func (o *ListAuditEventsQuery) SetDoorName(v string)`

SetDoorName sets DoorName field to given value.

### HasDoorName

`func (o *ListAuditEventsQuery) HasDoorName() bool`

HasDoorName returns a boolean if a field has been set.

### SetDoorNameNil

`func (o *ListAuditEventsQuery) SetDoorNameNil(b bool)`

 SetDoorNameNil sets the value for DoorName to be an explicit nil

### UnsetDoorName
`func (o *ListAuditEventsQuery) UnsetDoorName()`

UnsetDoorName ensures that no value is present for DoorName, not even an explicit nil
### GetEventType

`func (o *ListAuditEventsQuery) GetEventType() string`

GetEventType returns the EventType field if non-nil, zero value otherwise.

### GetEventTypeOk

`func (o *ListAuditEventsQuery) GetEventTypeOk() (*string, bool)`

GetEventTypeOk returns a tuple with the EventType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventType

`func (o *ListAuditEventsQuery) SetEventType(v string)`

SetEventType sets EventType field to given value.

### HasEventType

`func (o *ListAuditEventsQuery) HasEventType() bool`

HasEventType returns a boolean if a field has been set.

### SetEventTypeNil

`func (o *ListAuditEventsQuery) SetEventTypeNil(b bool)`

 SetEventTypeNil sets the value for EventType to be an explicit nil

### UnsetEventType
`func (o *ListAuditEventsQuery) UnsetEventType()`

UnsetEventType ensures that no value is present for EventType, not even an explicit nil
### GetLimit

`func (o *ListAuditEventsQuery) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *ListAuditEventsQuery) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *ListAuditEventsQuery) SetLimit(v int32)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *ListAuditEventsQuery) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### SetLimitNil

`func (o *ListAuditEventsQuery) SetLimitNil(b bool)`

 SetLimitNil sets the value for Limit to be an explicit nil

### UnsetLimit
`func (o *ListAuditEventsQuery) UnsetLimit()`

UnsetLimit ensures that no value is present for Limit, not even an explicit nil
### GetOccurredAfter

`func (o *ListAuditEventsQuery) GetOccurredAfter() string`

GetOccurredAfter returns the OccurredAfter field if non-nil, zero value otherwise.

### GetOccurredAfterOk

`func (o *ListAuditEventsQuery) GetOccurredAfterOk() (*string, bool)`

GetOccurredAfterOk returns a tuple with the OccurredAfter field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredAfter

`func (o *ListAuditEventsQuery) SetOccurredAfter(v string)`

SetOccurredAfter sets OccurredAfter field to given value.

### HasOccurredAfter

`func (o *ListAuditEventsQuery) HasOccurredAfter() bool`

HasOccurredAfter returns a boolean if a field has been set.

### SetOccurredAfterNil

`func (o *ListAuditEventsQuery) SetOccurredAfterNil(b bool)`

 SetOccurredAfterNil sets the value for OccurredAfter to be an explicit nil

### UnsetOccurredAfter
`func (o *ListAuditEventsQuery) UnsetOccurredAfter()`

UnsetOccurredAfter ensures that no value is present for OccurredAfter, not even an explicit nil
### GetOccurredBefore

`func (o *ListAuditEventsQuery) GetOccurredBefore() string`

GetOccurredBefore returns the OccurredBefore field if non-nil, zero value otherwise.

### GetOccurredBeforeOk

`func (o *ListAuditEventsQuery) GetOccurredBeforeOk() (*string, bool)`

GetOccurredBeforeOk returns a tuple with the OccurredBefore field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredBefore

`func (o *ListAuditEventsQuery) SetOccurredBefore(v string)`

SetOccurredBefore sets OccurredBefore field to given value.

### HasOccurredBefore

`func (o *ListAuditEventsQuery) HasOccurredBefore() bool`

HasOccurredBefore returns a boolean if a field has been set.

### SetOccurredBeforeNil

`func (o *ListAuditEventsQuery) SetOccurredBeforeNil(b bool)`

 SetOccurredBeforeNil sets the value for OccurredBefore to be an explicit nil

### UnsetOccurredBefore
`func (o *ListAuditEventsQuery) UnsetOccurredBefore()`

UnsetOccurredBefore ensures that no value is present for OccurredBefore, not even an explicit nil
### GetOrgScope

`func (o *ListAuditEventsQuery) GetOrgScope() string`

GetOrgScope returns the OrgScope field if non-nil, zero value otherwise.

### GetOrgScopeOk

`func (o *ListAuditEventsQuery) GetOrgScopeOk() (*string, bool)`

GetOrgScopeOk returns a tuple with the OrgScope field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgScope

`func (o *ListAuditEventsQuery) SetOrgScope(v string)`

SetOrgScope sets OrgScope field to given value.

### HasOrgScope

`func (o *ListAuditEventsQuery) HasOrgScope() bool`

HasOrgScope returns a boolean if a field has been set.

### SetOrgScopeNil

`func (o *ListAuditEventsQuery) SetOrgScopeNil(b bool)`

 SetOrgScopeNil sets the value for OrgScope to be an explicit nil

### UnsetOrgScope
`func (o *ListAuditEventsQuery) UnsetOrgScope()`

UnsetOrgScope ensures that no value is present for OrgScope, not even an explicit nil
### GetOutcome

`func (o *ListAuditEventsQuery) GetOutcome() string`

GetOutcome returns the Outcome field if non-nil, zero value otherwise.

### GetOutcomeOk

`func (o *ListAuditEventsQuery) GetOutcomeOk() (*string, bool)`

GetOutcomeOk returns a tuple with the Outcome field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcome

`func (o *ListAuditEventsQuery) SetOutcome(v string)`

SetOutcome sets Outcome field to given value.

### HasOutcome

`func (o *ListAuditEventsQuery) HasOutcome() bool`

HasOutcome returns a boolean if a field has been set.

### SetOutcomeNil

`func (o *ListAuditEventsQuery) SetOutcomeNil(b bool)`

 SetOutcomeNil sets the value for Outcome to be an explicit nil

### UnsetOutcome
`func (o *ListAuditEventsQuery) UnsetOutcome()`

UnsetOutcome ensures that no value is present for Outcome, not even an explicit nil
### GetResourceId

`func (o *ListAuditEventsQuery) GetResourceId() string`

GetResourceId returns the ResourceId field if non-nil, zero value otherwise.

### GetResourceIdOk

`func (o *ListAuditEventsQuery) GetResourceIdOk() (*string, bool)`

GetResourceIdOk returns a tuple with the ResourceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceId

`func (o *ListAuditEventsQuery) SetResourceId(v string)`

SetResourceId sets ResourceId field to given value.

### HasResourceId

`func (o *ListAuditEventsQuery) HasResourceId() bool`

HasResourceId returns a boolean if a field has been set.

### SetResourceIdNil

`func (o *ListAuditEventsQuery) SetResourceIdNil(b bool)`

 SetResourceIdNil sets the value for ResourceId to be an explicit nil

### UnsetResourceId
`func (o *ListAuditEventsQuery) UnsetResourceId()`

UnsetResourceId ensures that no value is present for ResourceId, not even an explicit nil
### GetResourceName

`func (o *ListAuditEventsQuery) GetResourceName() string`

GetResourceName returns the ResourceName field if non-nil, zero value otherwise.

### GetResourceNameOk

`func (o *ListAuditEventsQuery) GetResourceNameOk() (*string, bool)`

GetResourceNameOk returns a tuple with the ResourceName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceName

`func (o *ListAuditEventsQuery) SetResourceName(v string)`

SetResourceName sets ResourceName field to given value.

### HasResourceName

`func (o *ListAuditEventsQuery) HasResourceName() bool`

HasResourceName returns a boolean if a field has been set.

### SetResourceNameNil

`func (o *ListAuditEventsQuery) SetResourceNameNil(b bool)`

 SetResourceNameNil sets the value for ResourceName to be an explicit nil

### UnsetResourceName
`func (o *ListAuditEventsQuery) UnsetResourceName()`

UnsetResourceName ensures that no value is present for ResourceName, not even an explicit nil
### GetResourceType

`func (o *ListAuditEventsQuery) GetResourceType() string`

GetResourceType returns the ResourceType field if non-nil, zero value otherwise.

### GetResourceTypeOk

`func (o *ListAuditEventsQuery) GetResourceTypeOk() (*string, bool)`

GetResourceTypeOk returns a tuple with the ResourceType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceType

`func (o *ListAuditEventsQuery) SetResourceType(v string)`

SetResourceType sets ResourceType field to given value.

### HasResourceType

`func (o *ListAuditEventsQuery) HasResourceType() bool`

HasResourceType returns a boolean if a field has been set.

### SetResourceTypeNil

`func (o *ListAuditEventsQuery) SetResourceTypeNil(b bool)`

 SetResourceTypeNil sets the value for ResourceType to be an explicit nil

### UnsetResourceType
`func (o *ListAuditEventsQuery) UnsetResourceType()`

UnsetResourceType ensures that no value is present for ResourceType, not even an explicit nil
### GetTargetId

`func (o *ListAuditEventsQuery) GetTargetId() string`

GetTargetId returns the TargetId field if non-nil, zero value otherwise.

### GetTargetIdOk

`func (o *ListAuditEventsQuery) GetTargetIdOk() (*string, bool)`

GetTargetIdOk returns a tuple with the TargetId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTargetId

`func (o *ListAuditEventsQuery) SetTargetId(v string)`

SetTargetId sets TargetId field to given value.

### HasTargetId

`func (o *ListAuditEventsQuery) HasTargetId() bool`

HasTargetId returns a boolean if a field has been set.

### SetTargetIdNil

`func (o *ListAuditEventsQuery) SetTargetIdNil(b bool)`

 SetTargetIdNil sets the value for TargetId to be an explicit nil

### UnsetTargetId
`func (o *ListAuditEventsQuery) UnsetTargetId()`

UnsetTargetId ensures that no value is present for TargetId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
