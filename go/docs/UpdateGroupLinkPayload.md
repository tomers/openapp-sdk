# UpdateGroupLinkPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AdmissionMode** | Pointer to **NullableString** | approve waits for each member to be admitted; auto admits them immediately; pinned holds a fixed member set until an explicit re-sync. | [optional]
**Resync** | Pointer to **NullableBool** | With pinned mode, re-reads the group&#39;s current membership and admits the difference. | [optional]

## Methods

### NewUpdateGroupLinkPayload

`func NewUpdateGroupLinkPayload() *UpdateGroupLinkPayload`

NewUpdateGroupLinkPayload instantiates a new UpdateGroupLinkPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateGroupLinkPayloadWithDefaults

`func NewUpdateGroupLinkPayloadWithDefaults() *UpdateGroupLinkPayload`

NewUpdateGroupLinkPayloadWithDefaults instantiates a new UpdateGroupLinkPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAdmissionMode

`func (o *UpdateGroupLinkPayload) GetAdmissionMode() string`

GetAdmissionMode returns the AdmissionMode field if non-nil, zero value otherwise.

### GetAdmissionModeOk

`func (o *UpdateGroupLinkPayload) GetAdmissionModeOk() (*string, bool)`

GetAdmissionModeOk returns a tuple with the AdmissionMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAdmissionMode

`func (o *UpdateGroupLinkPayload) SetAdmissionMode(v string)`

SetAdmissionMode sets AdmissionMode field to given value.

### HasAdmissionMode

`func (o *UpdateGroupLinkPayload) HasAdmissionMode() bool`

HasAdmissionMode returns a boolean if a field has been set.

### SetAdmissionModeNil

`func (o *UpdateGroupLinkPayload) SetAdmissionModeNil(b bool)`

 SetAdmissionModeNil sets the value for AdmissionMode to be an explicit nil

### UnsetAdmissionMode
`func (o *UpdateGroupLinkPayload) UnsetAdmissionMode()`

UnsetAdmissionMode ensures that no value is present for AdmissionMode, not even an explicit nil
### GetResync

`func (o *UpdateGroupLinkPayload) GetResync() bool`

GetResync returns the Resync field if non-nil, zero value otherwise.

### GetResyncOk

`func (o *UpdateGroupLinkPayload) GetResyncOk() (*bool, bool)`

GetResyncOk returns a tuple with the Resync field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResync

`func (o *UpdateGroupLinkPayload) SetResync(v bool)`

SetResync sets Resync field to given value.

### HasResync

`func (o *UpdateGroupLinkPayload) HasResync() bool`

HasResync returns a boolean if a field has been set.

### SetResyncNil

`func (o *UpdateGroupLinkPayload) SetResyncNil(b bool)`

 SetResyncNil sets the value for Resync to be an explicit nil

### UnsetResync
`func (o *UpdateGroupLinkPayload) UnsetResync()`

UnsetResync ensures that no value is present for Resync, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
