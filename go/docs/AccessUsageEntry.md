# AccessUsageEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Actor** | [**UsageActor**](UsageActor.md) |  |
**DoorName** | Pointer to **interface{}** |  | [optional]
**Id** | **string** | Source audit event id (ULID). |
**InviteName** | Pointer to **interface{}** |  | [optional]
**OccurredAt** | **string** | RFC 3339 timestamp of the event. |
**Outcome** | **string** | &#x60;succeeded&#x60; | &#x60;failed&#x60; | &#x60;denied&#x60;. |

## Methods

### NewAccessUsageEntry

`func NewAccessUsageEntry(actor UsageActor, id string, occurredAt string, outcome string, ) *AccessUsageEntry`

NewAccessUsageEntry instantiates a new AccessUsageEntry object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAccessUsageEntryWithDefaults

`func NewAccessUsageEntryWithDefaults() *AccessUsageEntry`

NewAccessUsageEntryWithDefaults instantiates a new AccessUsageEntry object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetActor

`func (o *AccessUsageEntry) GetActor() UsageActor`

GetActor returns the Actor field if non-nil, zero value otherwise.

### GetActorOk

`func (o *AccessUsageEntry) GetActorOk() (*UsageActor, bool)`

GetActorOk returns a tuple with the Actor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActor

`func (o *AccessUsageEntry) SetActor(v UsageActor)`

SetActor sets Actor field to given value.


### GetDoorName

`func (o *AccessUsageEntry) GetDoorName() interface{}`

GetDoorName returns the DoorName field if non-nil, zero value otherwise.

### GetDoorNameOk

`func (o *AccessUsageEntry) GetDoorNameOk() (*interface{}, bool)`

GetDoorNameOk returns a tuple with the DoorName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoorName

`func (o *AccessUsageEntry) SetDoorName(v interface{})`

SetDoorName sets DoorName field to given value.

### HasDoorName

`func (o *AccessUsageEntry) HasDoorName() bool`

HasDoorName returns a boolean if a field has been set.

### SetDoorNameNil

`func (o *AccessUsageEntry) SetDoorNameNil(b bool)`

 SetDoorNameNil sets the value for DoorName to be an explicit nil

### UnsetDoorName
`func (o *AccessUsageEntry) UnsetDoorName()`

UnsetDoorName ensures that no value is present for DoorName, not even an explicit nil
### GetId

`func (o *AccessUsageEntry) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AccessUsageEntry) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AccessUsageEntry) SetId(v string)`

SetId sets Id field to given value.


### GetInviteName

`func (o *AccessUsageEntry) GetInviteName() interface{}`

GetInviteName returns the InviteName field if non-nil, zero value otherwise.

### GetInviteNameOk

`func (o *AccessUsageEntry) GetInviteNameOk() (*interface{}, bool)`

GetInviteNameOk returns a tuple with the InviteName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteName

`func (o *AccessUsageEntry) SetInviteName(v interface{})`

SetInviteName sets InviteName field to given value.

### HasInviteName

`func (o *AccessUsageEntry) HasInviteName() bool`

HasInviteName returns a boolean if a field has been set.

### SetInviteNameNil

`func (o *AccessUsageEntry) SetInviteNameNil(b bool)`

 SetInviteNameNil sets the value for InviteName to be an explicit nil

### UnsetInviteName
`func (o *AccessUsageEntry) UnsetInviteName()`

UnsetInviteName ensures that no value is present for InviteName, not even an explicit nil
### GetOccurredAt

`func (o *AccessUsageEntry) GetOccurredAt() string`

GetOccurredAt returns the OccurredAt field if non-nil, zero value otherwise.

### GetOccurredAtOk

`func (o *AccessUsageEntry) GetOccurredAtOk() (*string, bool)`

GetOccurredAtOk returns a tuple with the OccurredAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredAt

`func (o *AccessUsageEntry) SetOccurredAt(v string)`

SetOccurredAt sets OccurredAt field to given value.


### GetOutcome

`func (o *AccessUsageEntry) GetOutcome() string`

GetOutcome returns the Outcome field if non-nil, zero value otherwise.

### GetOutcomeOk

`func (o *AccessUsageEntry) GetOutcomeOk() (*string, bool)`

GetOutcomeOk returns a tuple with the Outcome field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcome

`func (o *AccessUsageEntry) SetOutcome(v string)`

SetOutcome sets Outcome field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
