# OrgTimezoneResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Timezone** | Pointer to **NullableString** | IANA timezone (e.g. &#x60;Asia/Jerusalem&#x60;). &#x60;null&#x60; means UTC fallback at evaluation. | [optional]

## Methods

### NewOrgTimezoneResponse

`func NewOrgTimezoneResponse() *OrgTimezoneResponse`

NewOrgTimezoneResponse instantiates a new OrgTimezoneResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOrgTimezoneResponseWithDefaults

`func NewOrgTimezoneResponseWithDefaults() *OrgTimezoneResponse`

NewOrgTimezoneResponseWithDefaults instantiates a new OrgTimezoneResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetTimezone

`func (o *OrgTimezoneResponse) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *OrgTimezoneResponse) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *OrgTimezoneResponse) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.

### HasTimezone

`func (o *OrgTimezoneResponse) HasTimezone() bool`

HasTimezone returns a boolean if a field has been set.

### SetTimezoneNil

`func (o *OrgTimezoneResponse) SetTimezoneNil(b bool)`

 SetTimezoneNil sets the value for Timezone to be an explicit nil

### UnsetTimezone
`func (o *OrgTimezoneResponse) UnsetTimezone()`

UnsetTimezone ensures that no value is present for Timezone, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
