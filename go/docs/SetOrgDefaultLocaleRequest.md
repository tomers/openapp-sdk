# SetOrgDefaultLocaleRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DefaultLocale** | Pointer to **NullableString** | Supported locale code (&#x60;en&#x60;, &#x60;zh_cn&#x60;, &#x60;es&#x60;, &#x60;ru&#x60;, &#x60;pt_br&#x60;, &#x60;de&#x60;, &#x60;fr&#x60;, &#x60;ja&#x60;, &#x60;ar&#x60;, &#x60;it&#x60;, &#x60;he&#x60;), or &#x60;null&#x60; to clear. | [optional]

## Methods

### NewSetOrgDefaultLocaleRequest

`func NewSetOrgDefaultLocaleRequest() *SetOrgDefaultLocaleRequest`

NewSetOrgDefaultLocaleRequest instantiates a new SetOrgDefaultLocaleRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSetOrgDefaultLocaleRequestWithDefaults

`func NewSetOrgDefaultLocaleRequestWithDefaults() *SetOrgDefaultLocaleRequest`

NewSetOrgDefaultLocaleRequestWithDefaults instantiates a new SetOrgDefaultLocaleRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDefaultLocale

`func (o *SetOrgDefaultLocaleRequest) GetDefaultLocale() string`

GetDefaultLocale returns the DefaultLocale field if non-nil, zero value otherwise.

### GetDefaultLocaleOk

`func (o *SetOrgDefaultLocaleRequest) GetDefaultLocaleOk() (*string, bool)`

GetDefaultLocaleOk returns a tuple with the DefaultLocale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultLocale

`func (o *SetOrgDefaultLocaleRequest) SetDefaultLocale(v string)`

SetDefaultLocale sets DefaultLocale field to given value.

### HasDefaultLocale

`func (o *SetOrgDefaultLocaleRequest) HasDefaultLocale() bool`

HasDefaultLocale returns a boolean if a field has been set.

### SetDefaultLocaleNil

`func (o *SetOrgDefaultLocaleRequest) SetDefaultLocaleNil(b bool)`

 SetDefaultLocaleNil sets the value for DefaultLocale to be an explicit nil

### UnsetDefaultLocale
`func (o *SetOrgDefaultLocaleRequest) UnsetDefaultLocale()`

UnsetDefaultLocale ensures that no value is present for DefaultLocale, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
