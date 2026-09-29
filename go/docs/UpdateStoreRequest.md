# UpdateStoreRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BrandingOverrides** | Pointer to **map[string]interface{}** |  | [optional]
**Currency** | Pointer to **NullableString** |  | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**Timezone** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateStoreRequest

`func NewUpdateStoreRequest(name LocalizedString, ) *UpdateStoreRequest`

NewUpdateStoreRequest instantiates a new UpdateStoreRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateStoreRequestWithDefaults

`func NewUpdateStoreRequestWithDefaults() *UpdateStoreRequest`

NewUpdateStoreRequestWithDefaults instantiates a new UpdateStoreRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBrandingOverrides

`func (o *UpdateStoreRequest) GetBrandingOverrides() map[string]interface{}`

GetBrandingOverrides returns the BrandingOverrides field if non-nil, zero value otherwise.

### GetBrandingOverridesOk

`func (o *UpdateStoreRequest) GetBrandingOverridesOk() (*map[string]interface{}, bool)`

GetBrandingOverridesOk returns a tuple with the BrandingOverrides field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBrandingOverrides

`func (o *UpdateStoreRequest) SetBrandingOverrides(v map[string]interface{})`

SetBrandingOverrides sets BrandingOverrides field to given value.

### HasBrandingOverrides

`func (o *UpdateStoreRequest) HasBrandingOverrides() bool`

HasBrandingOverrides returns a boolean if a field has been set.

### SetBrandingOverridesNil

`func (o *UpdateStoreRequest) SetBrandingOverridesNil(b bool)`

 SetBrandingOverridesNil sets the value for BrandingOverrides to be an explicit nil

### UnsetBrandingOverrides
`func (o *UpdateStoreRequest) UnsetBrandingOverrides()`

UnsetBrandingOverrides ensures that no value is present for BrandingOverrides, not even an explicit nil
### GetCurrency

`func (o *UpdateStoreRequest) GetCurrency() string`

GetCurrency returns the Currency field if non-nil, zero value otherwise.

### GetCurrencyOk

`func (o *UpdateStoreRequest) GetCurrencyOk() (*string, bool)`

GetCurrencyOk returns a tuple with the Currency field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrency

`func (o *UpdateStoreRequest) SetCurrency(v string)`

SetCurrency sets Currency field to given value.

### HasCurrency

`func (o *UpdateStoreRequest) HasCurrency() bool`

HasCurrency returns a boolean if a field has been set.

### SetCurrencyNil

`func (o *UpdateStoreRequest) SetCurrencyNil(b bool)`

 SetCurrencyNil sets the value for Currency to be an explicit nil

### UnsetCurrency
`func (o *UpdateStoreRequest) UnsetCurrency()`

UnsetCurrency ensures that no value is present for Currency, not even an explicit nil
### GetName

`func (o *UpdateStoreRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateStoreRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateStoreRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetTimezone

`func (o *UpdateStoreRequest) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *UpdateStoreRequest) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *UpdateStoreRequest) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.

### HasTimezone

`func (o *UpdateStoreRequest) HasTimezone() bool`

HasTimezone returns a boolean if a field has been set.

### SetTimezoneNil

`func (o *UpdateStoreRequest) SetTimezoneNil(b bool)`

 SetTimezoneNil sets the value for Timezone to be an explicit nil

### UnsetTimezone
`func (o *UpdateStoreRequest) UnsetTimezone()`

UnsetTimezone ensures that no value is present for Timezone, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
