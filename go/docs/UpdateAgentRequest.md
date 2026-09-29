# UpdateAgentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedEntityIds** | Pointer to **[]string** |  | [optional]
**AllowedPortalIds** | Pointer to **[]string** |  | [optional]
**AllowedToolNames** | Pointer to **[]string** |  | [optional]
**ConfirmationMode** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | Pointer to **NullableString** |  | [optional]
**ExpiresIn** | Pointer to **NullableString** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Purpose** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateAgentRequest

`func NewUpdateAgentRequest() *UpdateAgentRequest`

NewUpdateAgentRequest instantiates a new UpdateAgentRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateAgentRequestWithDefaults

`func NewUpdateAgentRequestWithDefaults() *UpdateAgentRequest`

NewUpdateAgentRequestWithDefaults instantiates a new UpdateAgentRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedEntityIds

`func (o *UpdateAgentRequest) GetAllowedEntityIds() []string`

GetAllowedEntityIds returns the AllowedEntityIds field if non-nil, zero value otherwise.

### GetAllowedEntityIdsOk

`func (o *UpdateAgentRequest) GetAllowedEntityIdsOk() (*[]string, bool)`

GetAllowedEntityIdsOk returns a tuple with the AllowedEntityIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedEntityIds

`func (o *UpdateAgentRequest) SetAllowedEntityIds(v []string)`

SetAllowedEntityIds sets AllowedEntityIds field to given value.

### HasAllowedEntityIds

`func (o *UpdateAgentRequest) HasAllowedEntityIds() bool`

HasAllowedEntityIds returns a boolean if a field has been set.

### SetAllowedEntityIdsNil

`func (o *UpdateAgentRequest) SetAllowedEntityIdsNil(b bool)`

 SetAllowedEntityIdsNil sets the value for AllowedEntityIds to be an explicit nil

### UnsetAllowedEntityIds
`func (o *UpdateAgentRequest) UnsetAllowedEntityIds()`

UnsetAllowedEntityIds ensures that no value is present for AllowedEntityIds, not even an explicit nil
### GetAllowedPortalIds

`func (o *UpdateAgentRequest) GetAllowedPortalIds() []string`

GetAllowedPortalIds returns the AllowedPortalIds field if non-nil, zero value otherwise.

### GetAllowedPortalIdsOk

`func (o *UpdateAgentRequest) GetAllowedPortalIdsOk() (*[]string, bool)`

GetAllowedPortalIdsOk returns a tuple with the AllowedPortalIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedPortalIds

`func (o *UpdateAgentRequest) SetAllowedPortalIds(v []string)`

SetAllowedPortalIds sets AllowedPortalIds field to given value.

### HasAllowedPortalIds

`func (o *UpdateAgentRequest) HasAllowedPortalIds() bool`

HasAllowedPortalIds returns a boolean if a field has been set.

### SetAllowedPortalIdsNil

`func (o *UpdateAgentRequest) SetAllowedPortalIdsNil(b bool)`

 SetAllowedPortalIdsNil sets the value for AllowedPortalIds to be an explicit nil

### UnsetAllowedPortalIds
`func (o *UpdateAgentRequest) UnsetAllowedPortalIds()`

UnsetAllowedPortalIds ensures that no value is present for AllowedPortalIds, not even an explicit nil
### GetAllowedToolNames

`func (o *UpdateAgentRequest) GetAllowedToolNames() []string`

GetAllowedToolNames returns the AllowedToolNames field if non-nil, zero value otherwise.

### GetAllowedToolNamesOk

`func (o *UpdateAgentRequest) GetAllowedToolNamesOk() (*[]string, bool)`

GetAllowedToolNamesOk returns a tuple with the AllowedToolNames field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedToolNames

`func (o *UpdateAgentRequest) SetAllowedToolNames(v []string)`

SetAllowedToolNames sets AllowedToolNames field to given value.

### HasAllowedToolNames

`func (o *UpdateAgentRequest) HasAllowedToolNames() bool`

HasAllowedToolNames returns a boolean if a field has been set.

### SetAllowedToolNamesNil

`func (o *UpdateAgentRequest) SetAllowedToolNamesNil(b bool)`

 SetAllowedToolNamesNil sets the value for AllowedToolNames to be an explicit nil

### UnsetAllowedToolNames
`func (o *UpdateAgentRequest) UnsetAllowedToolNames()`

UnsetAllowedToolNames ensures that no value is present for AllowedToolNames, not even an explicit nil
### GetConfirmationMode

`func (o *UpdateAgentRequest) GetConfirmationMode() string`

GetConfirmationMode returns the ConfirmationMode field if non-nil, zero value otherwise.

### GetConfirmationModeOk

`func (o *UpdateAgentRequest) GetConfirmationModeOk() (*string, bool)`

GetConfirmationModeOk returns a tuple with the ConfirmationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfirmationMode

`func (o *UpdateAgentRequest) SetConfirmationMode(v string)`

SetConfirmationMode sets ConfirmationMode field to given value.

### HasConfirmationMode

`func (o *UpdateAgentRequest) HasConfirmationMode() bool`

HasConfirmationMode returns a boolean if a field has been set.

### SetConfirmationModeNil

`func (o *UpdateAgentRequest) SetConfirmationModeNil(b bool)`

 SetConfirmationModeNil sets the value for ConfirmationMode to be an explicit nil

### UnsetConfirmationMode
`func (o *UpdateAgentRequest) UnsetConfirmationMode()`

UnsetConfirmationMode ensures that no value is present for ConfirmationMode, not even an explicit nil
### GetExpiresAt

`func (o *UpdateAgentRequest) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *UpdateAgentRequest) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *UpdateAgentRequest) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *UpdateAgentRequest) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *UpdateAgentRequest) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *UpdateAgentRequest) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetExpiresIn

`func (o *UpdateAgentRequest) GetExpiresIn() string`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *UpdateAgentRequest) GetExpiresInOk() (*string, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *UpdateAgentRequest) SetExpiresIn(v string)`

SetExpiresIn sets ExpiresIn field to given value.

### HasExpiresIn

`func (o *UpdateAgentRequest) HasExpiresIn() bool`

HasExpiresIn returns a boolean if a field has been set.

### SetExpiresInNil

`func (o *UpdateAgentRequest) SetExpiresInNil(b bool)`

 SetExpiresInNil sets the value for ExpiresIn to be an explicit nil

### UnsetExpiresIn
`func (o *UpdateAgentRequest) UnsetExpiresIn()`

UnsetExpiresIn ensures that no value is present for ExpiresIn, not even an explicit nil
### GetName

`func (o *UpdateAgentRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateAgentRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateAgentRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *UpdateAgentRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdateAgentRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdateAgentRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetPurpose

`func (o *UpdateAgentRequest) GetPurpose() string`

GetPurpose returns the Purpose field if non-nil, zero value otherwise.

### GetPurposeOk

`func (o *UpdateAgentRequest) GetPurposeOk() (*string, bool)`

GetPurposeOk returns a tuple with the Purpose field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurpose

`func (o *UpdateAgentRequest) SetPurpose(v string)`

SetPurpose sets Purpose field to given value.

### HasPurpose

`func (o *UpdateAgentRequest) HasPurpose() bool`

HasPurpose returns a boolean if a field has been set.

### SetPurposeNil

`func (o *UpdateAgentRequest) SetPurposeNil(b bool)`

 SetPurposeNil sets the value for Purpose to be an explicit nil

### UnsetPurpose
`func (o *UpdateAgentRequest) UnsetPurpose()`

UnsetPurpose ensures that no value is present for Purpose, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
