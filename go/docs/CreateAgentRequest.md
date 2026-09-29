# CreateAgentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedEntityIds** | Pointer to **[]string** |  | [optional]
**AllowedPortalIds** | Pointer to **[]string** |  | [optional]
**AllowedToolNames** | Pointer to **[]string** |  | [optional]
**ConfirmationMode** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | Pointer to **NullableString** |  | [optional]
**ExpiresIn** | Pointer to **NullableString** |  | [optional]
**Name** | **string** |  |
**Purpose** | Pointer to **NullableString** |  | [optional]

## Methods

### NewCreateAgentRequest

`func NewCreateAgentRequest(name string, ) *CreateAgentRequest`

NewCreateAgentRequest instantiates a new CreateAgentRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateAgentRequestWithDefaults

`func NewCreateAgentRequestWithDefaults() *CreateAgentRequest`

NewCreateAgentRequestWithDefaults instantiates a new CreateAgentRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedEntityIds

`func (o *CreateAgentRequest) GetAllowedEntityIds() []string`

GetAllowedEntityIds returns the AllowedEntityIds field if non-nil, zero value otherwise.

### GetAllowedEntityIdsOk

`func (o *CreateAgentRequest) GetAllowedEntityIdsOk() (*[]string, bool)`

GetAllowedEntityIdsOk returns a tuple with the AllowedEntityIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedEntityIds

`func (o *CreateAgentRequest) SetAllowedEntityIds(v []string)`

SetAllowedEntityIds sets AllowedEntityIds field to given value.

### HasAllowedEntityIds

`func (o *CreateAgentRequest) HasAllowedEntityIds() bool`

HasAllowedEntityIds returns a boolean if a field has been set.

### SetAllowedEntityIdsNil

`func (o *CreateAgentRequest) SetAllowedEntityIdsNil(b bool)`

 SetAllowedEntityIdsNil sets the value for AllowedEntityIds to be an explicit nil

### UnsetAllowedEntityIds
`func (o *CreateAgentRequest) UnsetAllowedEntityIds()`

UnsetAllowedEntityIds ensures that no value is present for AllowedEntityIds, not even an explicit nil
### GetAllowedPortalIds

`func (o *CreateAgentRequest) GetAllowedPortalIds() []string`

GetAllowedPortalIds returns the AllowedPortalIds field if non-nil, zero value otherwise.

### GetAllowedPortalIdsOk

`func (o *CreateAgentRequest) GetAllowedPortalIdsOk() (*[]string, bool)`

GetAllowedPortalIdsOk returns a tuple with the AllowedPortalIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedPortalIds

`func (o *CreateAgentRequest) SetAllowedPortalIds(v []string)`

SetAllowedPortalIds sets AllowedPortalIds field to given value.

### HasAllowedPortalIds

`func (o *CreateAgentRequest) HasAllowedPortalIds() bool`

HasAllowedPortalIds returns a boolean if a field has been set.

### SetAllowedPortalIdsNil

`func (o *CreateAgentRequest) SetAllowedPortalIdsNil(b bool)`

 SetAllowedPortalIdsNil sets the value for AllowedPortalIds to be an explicit nil

### UnsetAllowedPortalIds
`func (o *CreateAgentRequest) UnsetAllowedPortalIds()`

UnsetAllowedPortalIds ensures that no value is present for AllowedPortalIds, not even an explicit nil
### GetAllowedToolNames

`func (o *CreateAgentRequest) GetAllowedToolNames() []string`

GetAllowedToolNames returns the AllowedToolNames field if non-nil, zero value otherwise.

### GetAllowedToolNamesOk

`func (o *CreateAgentRequest) GetAllowedToolNamesOk() (*[]string, bool)`

GetAllowedToolNamesOk returns a tuple with the AllowedToolNames field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedToolNames

`func (o *CreateAgentRequest) SetAllowedToolNames(v []string)`

SetAllowedToolNames sets AllowedToolNames field to given value.

### HasAllowedToolNames

`func (o *CreateAgentRequest) HasAllowedToolNames() bool`

HasAllowedToolNames returns a boolean if a field has been set.

### SetAllowedToolNamesNil

`func (o *CreateAgentRequest) SetAllowedToolNamesNil(b bool)`

 SetAllowedToolNamesNil sets the value for AllowedToolNames to be an explicit nil

### UnsetAllowedToolNames
`func (o *CreateAgentRequest) UnsetAllowedToolNames()`

UnsetAllowedToolNames ensures that no value is present for AllowedToolNames, not even an explicit nil
### GetConfirmationMode

`func (o *CreateAgentRequest) GetConfirmationMode() string`

GetConfirmationMode returns the ConfirmationMode field if non-nil, zero value otherwise.

### GetConfirmationModeOk

`func (o *CreateAgentRequest) GetConfirmationModeOk() (*string, bool)`

GetConfirmationModeOk returns a tuple with the ConfirmationMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfirmationMode

`func (o *CreateAgentRequest) SetConfirmationMode(v string)`

SetConfirmationMode sets ConfirmationMode field to given value.

### HasConfirmationMode

`func (o *CreateAgentRequest) HasConfirmationMode() bool`

HasConfirmationMode returns a boolean if a field has been set.

### SetConfirmationModeNil

`func (o *CreateAgentRequest) SetConfirmationModeNil(b bool)`

 SetConfirmationModeNil sets the value for ConfirmationMode to be an explicit nil

### UnsetConfirmationMode
`func (o *CreateAgentRequest) UnsetConfirmationMode()`

UnsetConfirmationMode ensures that no value is present for ConfirmationMode, not even an explicit nil
### GetExpiresAt

`func (o *CreateAgentRequest) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *CreateAgentRequest) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *CreateAgentRequest) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *CreateAgentRequest) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *CreateAgentRequest) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *CreateAgentRequest) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetExpiresIn

`func (o *CreateAgentRequest) GetExpiresIn() string`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *CreateAgentRequest) GetExpiresInOk() (*string, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *CreateAgentRequest) SetExpiresIn(v string)`

SetExpiresIn sets ExpiresIn field to given value.

### HasExpiresIn

`func (o *CreateAgentRequest) HasExpiresIn() bool`

HasExpiresIn returns a boolean if a field has been set.

### SetExpiresInNil

`func (o *CreateAgentRequest) SetExpiresInNil(b bool)`

 SetExpiresInNil sets the value for ExpiresIn to be an explicit nil

### UnsetExpiresIn
`func (o *CreateAgentRequest) UnsetExpiresIn()`

UnsetExpiresIn ensures that no value is present for ExpiresIn, not even an explicit nil
### GetName

`func (o *CreateAgentRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateAgentRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateAgentRequest) SetName(v string)`

SetName sets Name field to given value.


### GetPurpose

`func (o *CreateAgentRequest) GetPurpose() string`

GetPurpose returns the Purpose field if non-nil, zero value otherwise.

### GetPurposeOk

`func (o *CreateAgentRequest) GetPurposeOk() (*string, bool)`

GetPurposeOk returns a tuple with the Purpose field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurpose

`func (o *CreateAgentRequest) SetPurpose(v string)`

SetPurpose sets Purpose field to given value.

### HasPurpose

`func (o *CreateAgentRequest) HasPurpose() bool`

HasPurpose returns a boolean if a field has been set.

### SetPurposeNil

`func (o *CreateAgentRequest) SetPurposeNil(b bool)`

 SetPurposeNil sets the value for Purpose to be an explicit nil

### UnsetPurpose
`func (o *CreateAgentRequest) UnsetPurpose()`

UnsetPurpose ensures that no value is present for Purpose, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
