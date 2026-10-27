# OrgDefaultLocaleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DefaultLocale** | Pointer to **NullableString** | Supported locale code, or &#x60;null&#x60; when unset (messages then use English unless the recipient has a locale of their own). | [optional]

## Methods

### NewOrgDefaultLocaleResponse

`func NewOrgDefaultLocaleResponse() *OrgDefaultLocaleResponse`

NewOrgDefaultLocaleResponse instantiates a new OrgDefaultLocaleResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOrgDefaultLocaleResponseWithDefaults

`func NewOrgDefaultLocaleResponseWithDefaults() *OrgDefaultLocaleResponse`

NewOrgDefaultLocaleResponseWithDefaults instantiates a new OrgDefaultLocaleResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDefaultLocale

`func (o *OrgDefaultLocaleResponse) GetDefaultLocale() string`

GetDefaultLocale returns the DefaultLocale field if non-nil, zero value otherwise.

### GetDefaultLocaleOk

`func (o *OrgDefaultLocaleResponse) GetDefaultLocaleOk() (*string, bool)`

GetDefaultLocaleOk returns a tuple with the DefaultLocale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultLocale

`func (o *OrgDefaultLocaleResponse) SetDefaultLocale(v string)`

SetDefaultLocale sets DefaultLocale field to given value.

### HasDefaultLocale

`func (o *OrgDefaultLocaleResponse) HasDefaultLocale() bool`

HasDefaultLocale returns a boolean if a field has been set.

### SetDefaultLocaleNil

`func (o *OrgDefaultLocaleResponse) SetDefaultLocaleNil(b bool)`

 SetDefaultLocaleNil sets the value for DefaultLocale to be an explicit nil

### UnsetDefaultLocale
`func (o *OrgDefaultLocaleResponse) UnsetDefaultLocale()`

UnsetDefaultLocale ensures that no value is present for DefaultLocale, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
