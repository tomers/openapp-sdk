# SetOrgTimezoneRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Timezone** | Pointer to **NullableString** | IANA timezone. Empty or null clears to UTC fallback. | [optional]

## Methods

### NewSetOrgTimezoneRequest

`func NewSetOrgTimezoneRequest() *SetOrgTimezoneRequest`

NewSetOrgTimezoneRequest instantiates a new SetOrgTimezoneRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSetOrgTimezoneRequestWithDefaults

`func NewSetOrgTimezoneRequestWithDefaults() *SetOrgTimezoneRequest`

NewSetOrgTimezoneRequestWithDefaults instantiates a new SetOrgTimezoneRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetTimezone

`func (o *SetOrgTimezoneRequest) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *SetOrgTimezoneRequest) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *SetOrgTimezoneRequest) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.

### HasTimezone

`func (o *SetOrgTimezoneRequest) HasTimezone() bool`

HasTimezone returns a boolean if a field has been set.

### SetTimezoneNil

`func (o *SetOrgTimezoneRequest) SetTimezoneNil(b bool)`

 SetTimezoneNil sets the value for Timezone to be an explicit nil

### UnsetTimezone
`func (o *SetOrgTimezoneRequest) UnsetTimezone()`

UnsetTimezone ensures that no value is present for Timezone, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
