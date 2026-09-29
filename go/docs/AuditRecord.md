# AuditRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ActorAgentId** | Pointer to **NullableString** |  | [optional]
**ActorApiKeyId** | Pointer to **NullableString** |  | [optional]
**ActorGuestId** | Pointer to **NullableString** | Stable, client-supplied per-device guest fingerprint (ULID). Deliberately kept in its own field, NEVER &#x60;actor_user_id&#x60;: a guest id is spoofable and must never be readable as a trusted (registered) user identity, nor collide with the user-id population. | [optional]
**ActorKind** | **string** |  |
**ActorUserId** | Pointer to **NullableString** |  | [optional]
**CorrelationId** | Pointer to **NullableString** |  | [optional]
**Details** | **interface{}** |  |
**EventType** | **string** |  |
**Id** | **string** | ULID; also the webhook idempotency key. |
**OccurredAt** | **time.Time** |  |
**OrgId** | **string** |  |
**Outcome** | [**AuditOutcome**](AuditOutcome.md) |  |
**ResourceId** | Pointer to **NullableString** |  | [optional]
**ResourceType** | Pointer to **NullableString** |  | [optional]
**Source** | [**AuditSource**](AuditSource.md) |  |

## Methods

### NewAuditRecord

`func NewAuditRecord(actorKind string, details interface{}, eventType string, id string, occurredAt time.Time, orgId string, outcome AuditOutcome, source AuditSource, ) *AuditRecord`

NewAuditRecord instantiates a new AuditRecord object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAuditRecordWithDefaults

`func NewAuditRecordWithDefaults() *AuditRecord`

NewAuditRecordWithDefaults instantiates a new AuditRecord object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetActorAgentId

`func (o *AuditRecord) GetActorAgentId() string`

GetActorAgentId returns the ActorAgentId field if non-nil, zero value otherwise.

### GetActorAgentIdOk

`func (o *AuditRecord) GetActorAgentIdOk() (*string, bool)`

GetActorAgentIdOk returns a tuple with the ActorAgentId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorAgentId

`func (o *AuditRecord) SetActorAgentId(v string)`

SetActorAgentId sets ActorAgentId field to given value.

### HasActorAgentId

`func (o *AuditRecord) HasActorAgentId() bool`

HasActorAgentId returns a boolean if a field has been set.

### SetActorAgentIdNil

`func (o *AuditRecord) SetActorAgentIdNil(b bool)`

 SetActorAgentIdNil sets the value for ActorAgentId to be an explicit nil

### UnsetActorAgentId
`func (o *AuditRecord) UnsetActorAgentId()`

UnsetActorAgentId ensures that no value is present for ActorAgentId, not even an explicit nil
### GetActorApiKeyId

`func (o *AuditRecord) GetActorApiKeyId() string`

GetActorApiKeyId returns the ActorApiKeyId field if non-nil, zero value otherwise.

### GetActorApiKeyIdOk

`func (o *AuditRecord) GetActorApiKeyIdOk() (*string, bool)`

GetActorApiKeyIdOk returns a tuple with the ActorApiKeyId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorApiKeyId

`func (o *AuditRecord) SetActorApiKeyId(v string)`

SetActorApiKeyId sets ActorApiKeyId field to given value.

### HasActorApiKeyId

`func (o *AuditRecord) HasActorApiKeyId() bool`

HasActorApiKeyId returns a boolean if a field has been set.

### SetActorApiKeyIdNil

`func (o *AuditRecord) SetActorApiKeyIdNil(b bool)`

 SetActorApiKeyIdNil sets the value for ActorApiKeyId to be an explicit nil

### UnsetActorApiKeyId
`func (o *AuditRecord) UnsetActorApiKeyId()`

UnsetActorApiKeyId ensures that no value is present for ActorApiKeyId, not even an explicit nil
### GetActorGuestId

`func (o *AuditRecord) GetActorGuestId() string`

GetActorGuestId returns the ActorGuestId field if non-nil, zero value otherwise.

### GetActorGuestIdOk

`func (o *AuditRecord) GetActorGuestIdOk() (*string, bool)`

GetActorGuestIdOk returns a tuple with the ActorGuestId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorGuestId

`func (o *AuditRecord) SetActorGuestId(v string)`

SetActorGuestId sets ActorGuestId field to given value.

### HasActorGuestId

`func (o *AuditRecord) HasActorGuestId() bool`

HasActorGuestId returns a boolean if a field has been set.

### SetActorGuestIdNil

`func (o *AuditRecord) SetActorGuestIdNil(b bool)`

 SetActorGuestIdNil sets the value for ActorGuestId to be an explicit nil

### UnsetActorGuestId
`func (o *AuditRecord) UnsetActorGuestId()`

UnsetActorGuestId ensures that no value is present for ActorGuestId, not even an explicit nil
### GetActorKind

`func (o *AuditRecord) GetActorKind() string`

GetActorKind returns the ActorKind field if non-nil, zero value otherwise.

### GetActorKindOk

`func (o *AuditRecord) GetActorKindOk() (*string, bool)`

GetActorKindOk returns a tuple with the ActorKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorKind

`func (o *AuditRecord) SetActorKind(v string)`

SetActorKind sets ActorKind field to given value.


### GetActorUserId

`func (o *AuditRecord) GetActorUserId() string`

GetActorUserId returns the ActorUserId field if non-nil, zero value otherwise.

### GetActorUserIdOk

`func (o *AuditRecord) GetActorUserIdOk() (*string, bool)`

GetActorUserIdOk returns a tuple with the ActorUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorUserId

`func (o *AuditRecord) SetActorUserId(v string)`

SetActorUserId sets ActorUserId field to given value.

### HasActorUserId

`func (o *AuditRecord) HasActorUserId() bool`

HasActorUserId returns a boolean if a field has been set.

### SetActorUserIdNil

`func (o *AuditRecord) SetActorUserIdNil(b bool)`

 SetActorUserIdNil sets the value for ActorUserId to be an explicit nil

### UnsetActorUserId
`func (o *AuditRecord) UnsetActorUserId()`

UnsetActorUserId ensures that no value is present for ActorUserId, not even an explicit nil
### GetCorrelationId

`func (o *AuditRecord) GetCorrelationId() string`

GetCorrelationId returns the CorrelationId field if non-nil, zero value otherwise.

### GetCorrelationIdOk

`func (o *AuditRecord) GetCorrelationIdOk() (*string, bool)`

GetCorrelationIdOk returns a tuple with the CorrelationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCorrelationId

`func (o *AuditRecord) SetCorrelationId(v string)`

SetCorrelationId sets CorrelationId field to given value.

### HasCorrelationId

`func (o *AuditRecord) HasCorrelationId() bool`

HasCorrelationId returns a boolean if a field has been set.

### SetCorrelationIdNil

`func (o *AuditRecord) SetCorrelationIdNil(b bool)`

 SetCorrelationIdNil sets the value for CorrelationId to be an explicit nil

### UnsetCorrelationId
`func (o *AuditRecord) UnsetCorrelationId()`

UnsetCorrelationId ensures that no value is present for CorrelationId, not even an explicit nil
### GetDetails

`func (o *AuditRecord) GetDetails() interface{}`

GetDetails returns the Details field if non-nil, zero value otherwise.

### GetDetailsOk

`func (o *AuditRecord) GetDetailsOk() (*interface{}, bool)`

GetDetailsOk returns a tuple with the Details field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDetails

`func (o *AuditRecord) SetDetails(v interface{})`

SetDetails sets Details field to given value.


### SetDetailsNil

`func (o *AuditRecord) SetDetailsNil(b bool)`

 SetDetailsNil sets the value for Details to be an explicit nil

### UnsetDetails
`func (o *AuditRecord) UnsetDetails()`

UnsetDetails ensures that no value is present for Details, not even an explicit nil
### GetEventType

`func (o *AuditRecord) GetEventType() string`

GetEventType returns the EventType field if non-nil, zero value otherwise.

### GetEventTypeOk

`func (o *AuditRecord) GetEventTypeOk() (*string, bool)`

GetEventTypeOk returns a tuple with the EventType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventType

`func (o *AuditRecord) SetEventType(v string)`

SetEventType sets EventType field to given value.


### GetId

`func (o *AuditRecord) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AuditRecord) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AuditRecord) SetId(v string)`

SetId sets Id field to given value.


### GetOccurredAt

`func (o *AuditRecord) GetOccurredAt() time.Time`

GetOccurredAt returns the OccurredAt field if non-nil, zero value otherwise.

### GetOccurredAtOk

`func (o *AuditRecord) GetOccurredAtOk() (*time.Time, bool)`

GetOccurredAtOk returns a tuple with the OccurredAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredAt

`func (o *AuditRecord) SetOccurredAt(v time.Time)`

SetOccurredAt sets OccurredAt field to given value.


### GetOrgId

`func (o *AuditRecord) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *AuditRecord) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *AuditRecord) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetOutcome

`func (o *AuditRecord) GetOutcome() AuditOutcome`

GetOutcome returns the Outcome field if non-nil, zero value otherwise.

### GetOutcomeOk

`func (o *AuditRecord) GetOutcomeOk() (*AuditOutcome, bool)`

GetOutcomeOk returns a tuple with the Outcome field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcome

`func (o *AuditRecord) SetOutcome(v AuditOutcome)`

SetOutcome sets Outcome field to given value.


### GetResourceId

`func (o *AuditRecord) GetResourceId() string`

GetResourceId returns the ResourceId field if non-nil, zero value otherwise.

### GetResourceIdOk

`func (o *AuditRecord) GetResourceIdOk() (*string, bool)`

GetResourceIdOk returns a tuple with the ResourceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceId

`func (o *AuditRecord) SetResourceId(v string)`

SetResourceId sets ResourceId field to given value.

### HasResourceId

`func (o *AuditRecord) HasResourceId() bool`

HasResourceId returns a boolean if a field has been set.

### SetResourceIdNil

`func (o *AuditRecord) SetResourceIdNil(b bool)`

 SetResourceIdNil sets the value for ResourceId to be an explicit nil

### UnsetResourceId
`func (o *AuditRecord) UnsetResourceId()`

UnsetResourceId ensures that no value is present for ResourceId, not even an explicit nil
### GetResourceType

`func (o *AuditRecord) GetResourceType() string`

GetResourceType returns the ResourceType field if non-nil, zero value otherwise.

### GetResourceTypeOk

`func (o *AuditRecord) GetResourceTypeOk() (*string, bool)`

GetResourceTypeOk returns a tuple with the ResourceType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceType

`func (o *AuditRecord) SetResourceType(v string)`

SetResourceType sets ResourceType field to given value.

### HasResourceType

`func (o *AuditRecord) HasResourceType() bool`

HasResourceType returns a boolean if a field has been set.

### SetResourceTypeNil

`func (o *AuditRecord) SetResourceTypeNil(b bool)`

 SetResourceTypeNil sets the value for ResourceType to be an explicit nil

### UnsetResourceType
`func (o *AuditRecord) UnsetResourceType()`

UnsetResourceType ensures that no value is present for ResourceType, not even an explicit nil
### GetSource

`func (o *AuditRecord) GetSource() AuditSource`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *AuditRecord) GetSourceOk() (*AuditSource, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *AuditRecord) SetSource(v AuditSource)`

SetSource sets Source field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
