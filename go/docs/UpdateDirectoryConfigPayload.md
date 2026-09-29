# UpdateDirectoryConfigPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedListingKinds** | Pointer to **[]string** |  | [optional]
**DisabledMessage** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**NamingPatterns** | Pointer to **map[string]string** |  | [optional]
**State** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateDirectoryConfigPayload

`func NewUpdateDirectoryConfigPayload() *UpdateDirectoryConfigPayload`

NewUpdateDirectoryConfigPayload instantiates a new UpdateDirectoryConfigPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateDirectoryConfigPayloadWithDefaults

`func NewUpdateDirectoryConfigPayloadWithDefaults() *UpdateDirectoryConfigPayload`

NewUpdateDirectoryConfigPayloadWithDefaults instantiates a new UpdateDirectoryConfigPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedListingKinds

`func (o *UpdateDirectoryConfigPayload) GetAllowedListingKinds() []string`

GetAllowedListingKinds returns the AllowedListingKinds field if non-nil, zero value otherwise.

### GetAllowedListingKindsOk

`func (o *UpdateDirectoryConfigPayload) GetAllowedListingKindsOk() (*[]string, bool)`

GetAllowedListingKindsOk returns a tuple with the AllowedListingKinds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedListingKinds

`func (o *UpdateDirectoryConfigPayload) SetAllowedListingKinds(v []string)`

SetAllowedListingKinds sets AllowedListingKinds field to given value.

### HasAllowedListingKinds

`func (o *UpdateDirectoryConfigPayload) HasAllowedListingKinds() bool`

HasAllowedListingKinds returns a boolean if a field has been set.

### SetAllowedListingKindsNil

`func (o *UpdateDirectoryConfigPayload) SetAllowedListingKindsNil(b bool)`

 SetAllowedListingKindsNil sets the value for AllowedListingKinds to be an explicit nil

### UnsetAllowedListingKinds
`func (o *UpdateDirectoryConfigPayload) UnsetAllowedListingKinds()`

UnsetAllowedListingKinds ensures that no value is present for AllowedListingKinds, not even an explicit nil
### GetDisabledMessage

`func (o *UpdateDirectoryConfigPayload) GetDisabledMessage() LocalizedString`

GetDisabledMessage returns the DisabledMessage field if non-nil, zero value otherwise.

### GetDisabledMessageOk

`func (o *UpdateDirectoryConfigPayload) GetDisabledMessageOk() (*LocalizedString, bool)`

GetDisabledMessageOk returns a tuple with the DisabledMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisabledMessage

`func (o *UpdateDirectoryConfigPayload) SetDisabledMessage(v LocalizedString)`

SetDisabledMessage sets DisabledMessage field to given value.

### HasDisabledMessage

`func (o *UpdateDirectoryConfigPayload) HasDisabledMessage() bool`

HasDisabledMessage returns a boolean if a field has been set.

### SetDisabledMessageNil

`func (o *UpdateDirectoryConfigPayload) SetDisabledMessageNil(b bool)`

 SetDisabledMessageNil sets the value for DisabledMessage to be an explicit nil

### UnsetDisabledMessage
`func (o *UpdateDirectoryConfigPayload) UnsetDisabledMessage()`

UnsetDisabledMessage ensures that no value is present for DisabledMessage, not even an explicit nil
### GetNamingPatterns

`func (o *UpdateDirectoryConfigPayload) GetNamingPatterns() map[string]string`

GetNamingPatterns returns the NamingPatterns field if non-nil, zero value otherwise.

### GetNamingPatternsOk

`func (o *UpdateDirectoryConfigPayload) GetNamingPatternsOk() (*map[string]string, bool)`

GetNamingPatternsOk returns a tuple with the NamingPatterns field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNamingPatterns

`func (o *UpdateDirectoryConfigPayload) SetNamingPatterns(v map[string]string)`

SetNamingPatterns sets NamingPatterns field to given value.

### HasNamingPatterns

`func (o *UpdateDirectoryConfigPayload) HasNamingPatterns() bool`

HasNamingPatterns returns a boolean if a field has been set.

### GetState

`func (o *UpdateDirectoryConfigPayload) GetState() string`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *UpdateDirectoryConfigPayload) GetStateOk() (*string, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *UpdateDirectoryConfigPayload) SetState(v string)`

SetState sets State field to given value.

### HasState

`func (o *UpdateDirectoryConfigPayload) HasState() bool`

HasState returns a boolean if a field has been set.

### SetStateNil

`func (o *UpdateDirectoryConfigPayload) SetStateNil(b bool)`

 SetStateNil sets the value for State to be an explicit nil

### UnsetState
`func (o *UpdateDirectoryConfigPayload) UnsetState()`

UnsetState ensures that no value is present for State, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
