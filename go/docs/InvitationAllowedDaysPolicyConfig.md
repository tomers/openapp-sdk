# InvitationAllowedDaysPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BlackoutDates** | Pointer to **[]string** | Organization-local ISO dates that invitations cannot use. | [optional]
**HolidayCalendar** | Pointer to **NullableBool** | Also block the dates from applicable &#x60;holiday_calendar&#x60; policies. | [optional]
**Output** | Pointer to [**NullablePolicyOutputSelector**](PolicyOutputSelector.md) | Provider-derived output/channel selector: the row binds only actions on that output. | [optional]
**Weekdays** | Pointer to **[]int32** | Sunday is 0; Saturday is 6. Omit to leave weekdays unrestricted for this row. | [optional]
**ZoneId** | Pointer to **NullableString** | Zone the row is scoped to (org and integration rows only): it binds only when the acting effective zone is this zone or a descendant. Must be a live zone of the policy&#39;s org. | [optional]

## Methods

### NewInvitationAllowedDaysPolicyConfig

`func NewInvitationAllowedDaysPolicyConfig() *InvitationAllowedDaysPolicyConfig`

NewInvitationAllowedDaysPolicyConfig instantiates a new InvitationAllowedDaysPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationAllowedDaysPolicyConfigWithDefaults

`func NewInvitationAllowedDaysPolicyConfigWithDefaults() *InvitationAllowedDaysPolicyConfig`

NewInvitationAllowedDaysPolicyConfigWithDefaults instantiates a new InvitationAllowedDaysPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBlackoutDates

`func (o *InvitationAllowedDaysPolicyConfig) GetBlackoutDates() []*string`

GetBlackoutDates returns the BlackoutDates field if non-nil, zero value otherwise.

### GetBlackoutDatesOk

`func (o *InvitationAllowedDaysPolicyConfig) GetBlackoutDatesOk() (*[]*string, bool)`

GetBlackoutDatesOk returns a tuple with the BlackoutDates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBlackoutDates

`func (o *InvitationAllowedDaysPolicyConfig) SetBlackoutDates(v []*string)`

SetBlackoutDates sets BlackoutDates field to given value.

### HasBlackoutDates

`func (o *InvitationAllowedDaysPolicyConfig) HasBlackoutDates() bool`

HasBlackoutDates returns a boolean if a field has been set.

### SetBlackoutDatesNil

`func (o *InvitationAllowedDaysPolicyConfig) SetBlackoutDatesNil(b bool)`

 SetBlackoutDatesNil sets the value for BlackoutDates to be an explicit nil

### UnsetBlackoutDates
`func (o *InvitationAllowedDaysPolicyConfig) UnsetBlackoutDates()`

UnsetBlackoutDates ensures that no value is present for BlackoutDates, not even an explicit nil
### GetHolidayCalendar

`func (o *InvitationAllowedDaysPolicyConfig) GetHolidayCalendar() bool`

GetHolidayCalendar returns the HolidayCalendar field if non-nil, zero value otherwise.

### GetHolidayCalendarOk

`func (o *InvitationAllowedDaysPolicyConfig) GetHolidayCalendarOk() (*bool, bool)`

GetHolidayCalendarOk returns a tuple with the HolidayCalendar field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHolidayCalendar

`func (o *InvitationAllowedDaysPolicyConfig) SetHolidayCalendar(v bool)`

SetHolidayCalendar sets HolidayCalendar field to given value.

### HasHolidayCalendar

`func (o *InvitationAllowedDaysPolicyConfig) HasHolidayCalendar() bool`

HasHolidayCalendar returns a boolean if a field has been set.

### SetHolidayCalendarNil

`func (o *InvitationAllowedDaysPolicyConfig) SetHolidayCalendarNil(b bool)`

 SetHolidayCalendarNil sets the value for HolidayCalendar to be an explicit nil

### UnsetHolidayCalendar
`func (o *InvitationAllowedDaysPolicyConfig) UnsetHolidayCalendar()`

UnsetHolidayCalendar ensures that no value is present for HolidayCalendar, not even an explicit nil
### GetOutput

`func (o *InvitationAllowedDaysPolicyConfig) GetOutput() PolicyOutputSelector`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *InvitationAllowedDaysPolicyConfig) GetOutputOk() (*PolicyOutputSelector, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *InvitationAllowedDaysPolicyConfig) SetOutput(v PolicyOutputSelector)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *InvitationAllowedDaysPolicyConfig) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *InvitationAllowedDaysPolicyConfig) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *InvitationAllowedDaysPolicyConfig) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil
### GetWeekdays

`func (o *InvitationAllowedDaysPolicyConfig) GetWeekdays() []int32`

GetWeekdays returns the Weekdays field if non-nil, zero value otherwise.

### GetWeekdaysOk

`func (o *InvitationAllowedDaysPolicyConfig) GetWeekdaysOk() (*[]int32, bool)`

GetWeekdaysOk returns a tuple with the Weekdays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWeekdays

`func (o *InvitationAllowedDaysPolicyConfig) SetWeekdays(v []int32)`

SetWeekdays sets Weekdays field to given value.

### HasWeekdays

`func (o *InvitationAllowedDaysPolicyConfig) HasWeekdays() bool`

HasWeekdays returns a boolean if a field has been set.

### SetWeekdaysNil

`func (o *InvitationAllowedDaysPolicyConfig) SetWeekdaysNil(b bool)`

 SetWeekdaysNil sets the value for Weekdays to be an explicit nil

### UnsetWeekdays
`func (o *InvitationAllowedDaysPolicyConfig) UnsetWeekdays()`

UnsetWeekdays ensures that no value is present for Weekdays, not even an explicit nil
### GetZoneId

`func (o *InvitationAllowedDaysPolicyConfig) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *InvitationAllowedDaysPolicyConfig) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *InvitationAllowedDaysPolicyConfig) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.

### HasZoneId

`func (o *InvitationAllowedDaysPolicyConfig) HasZoneId() bool`

HasZoneId returns a boolean if a field has been set.

### SetZoneIdNil

`func (o *InvitationAllowedDaysPolicyConfig) SetZoneIdNil(b bool)`

 SetZoneIdNil sets the value for ZoneId to be an explicit nil

### UnsetZoneId
`func (o *InvitationAllowedDaysPolicyConfig) UnsetZoneId()`

UnsetZoneId ensures that no value is present for ZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
