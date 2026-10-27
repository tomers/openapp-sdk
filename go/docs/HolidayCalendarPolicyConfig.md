# HolidayCalendarPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Dates** | **[]string** | Organization-local ISO dates in &#x60;YYYY-MM-DD&#x60; format. |
**Output** | Pointer to [**NullablePolicyOutputSelector**](PolicyOutputSelector.md) | Provider-derived output/channel selector: the row binds only actions on that output. | [optional]
**ZoneId** | Pointer to **NullableString** | Zone the row is scoped to (org and integration rows only): it binds only when the acting effective zone is this zone or a descendant. Must be a live zone of the policy&#39;s org. | [optional]

## Methods

### NewHolidayCalendarPolicyConfig

`func NewHolidayCalendarPolicyConfig(dates []*string, ) *HolidayCalendarPolicyConfig`

NewHolidayCalendarPolicyConfig instantiates a new HolidayCalendarPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewHolidayCalendarPolicyConfigWithDefaults

`func NewHolidayCalendarPolicyConfigWithDefaults() *HolidayCalendarPolicyConfig`

NewHolidayCalendarPolicyConfigWithDefaults instantiates a new HolidayCalendarPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDates

`func (o *HolidayCalendarPolicyConfig) GetDates() []*string`

GetDates returns the Dates field if non-nil, zero value otherwise.

### GetDatesOk

`func (o *HolidayCalendarPolicyConfig) GetDatesOk() (*[]*string, bool)`

GetDatesOk returns a tuple with the Dates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDates

`func (o *HolidayCalendarPolicyConfig) SetDates(v []*string)`

SetDates sets Dates field to given value.


### GetOutput

`func (o *HolidayCalendarPolicyConfig) GetOutput() PolicyOutputSelector`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *HolidayCalendarPolicyConfig) GetOutputOk() (*PolicyOutputSelector, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *HolidayCalendarPolicyConfig) SetOutput(v PolicyOutputSelector)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *HolidayCalendarPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *HolidayCalendarPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *HolidayCalendarPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil
### GetZoneId

`func (o *HolidayCalendarPolicyConfig) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *HolidayCalendarPolicyConfig) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *HolidayCalendarPolicyConfig) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *HolidayCalendarPolicyConfig) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### SetZoneIdNil

`func (o *HolidayCalendarPolicyConfig) SetZoneIdNil(b bool)`

 SetZoneIdNil sets the value for ZoneId to be an explicit nil

### UnsetZoneId
`func (o *HolidayCalendarPolicyConfig) UnsetZoneId()`

UnsetZoneId ensures that no value is present for ZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
