# CreateStoreRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BrandingOverrides** | Pointer to **map[string]interface{}** |  | [optional]
**Currency** | Pointer to **NullableString** |  | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**Timezone** | Pointer to **NullableString** |  | [optional]

## Methods

### NewCreateStoreRequest

`func NewCreateStoreRequest(name LocalizedString, ) *CreateStoreRequest`

NewCreateStoreRequest instantiates a new CreateStoreRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateStoreRequestWithDefaults

`func NewCreateStoreRequestWithDefaults() *CreateStoreRequest`

NewCreateStoreRequestWithDefaults instantiates a new CreateStoreRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBrandingOverrides

`func (o *CreateStoreRequest) GetBrandingOverrides() map[string]interface{}`

GetBrandingOverrides returns the BrandingOverrides field if non-nil, zero value otherwise.

### GetBrandingOverridesOk

`func (o *CreateStoreRequest) GetBrandingOverridesOk() (*map[string]interface{}, bool)`

GetBrandingOverridesOk returns a tuple with the BrandingOverrides field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBrandingOverrides

`func (o *CreateStoreRequest) SetBrandingOverrides(v map[string]interface{})`

SetBrandingOverrides sets BrandingOverrides field to given value.

### HasBrandingOverrides

`func (o *CreateStoreRequest) HasBrandingOverrides() bool`

HasBrandingOverrides returns a boolean if a field has been set.

### SetBrandingOverridesNil

`func (o *CreateStoreRequest) SetBrandingOverridesNil(b bool)`

 SetBrandingOverridesNil sets the value for BrandingOverrides to be an explicit nil

### UnsetBrandingOverrides
`func (o *CreateStoreRequest) UnsetBrandingOverrides()`

UnsetBrandingOverrides ensures that no value is present for BrandingOverrides, not even an explicit nil
### GetCurrency

`func (o *CreateStoreRequest) GetCurrency() string`

GetCurrency returns the Currency field if non-nil, zero value otherwise.

### GetCurrencyOk

`func (o *CreateStoreRequest) GetCurrencyOk() (*string, bool)`

GetCurrencyOk returns a tuple with the Currency field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrency

`func (o *CreateStoreRequest) SetCurrency(v string)`

SetCurrency sets Currency field to given value.

### HasCurrency

`func (o *CreateStoreRequest) HasCurrency() bool`

HasCurrency returns a boolean if a field has been set.

### SetCurrencyNil

`func (o *CreateStoreRequest) SetCurrencyNil(b bool)`

 SetCurrencyNil sets the value for Currency to be an explicit nil

### UnsetCurrency
`func (o *CreateStoreRequest) UnsetCurrency()`

UnsetCurrency ensures that no value is present for Currency, not even an explicit nil
### GetName

`func (o *CreateStoreRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateStoreRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateStoreRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetTimezone

`func (o *CreateStoreRequest) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *CreateStoreRequest) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *CreateStoreRequest) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.

### HasTimezone

`func (o *CreateStoreRequest) HasTimezone() bool`

HasTimezone returns a boolean if a field has been set.

### SetTimezoneNil

`func (o *CreateStoreRequest) SetTimezoneNil(b bool)`

 SetTimezoneNil sets the value for Timezone to be an explicit nil

### UnsetTimezone
`func (o *CreateStoreRequest) UnsetTimezone()`

UnsetTimezone ensures that no value is present for Timezone, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
