# InviteScheduleEntryInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresIn** | Pointer to **NullableString** | Duration per [RFC 5545 §3.3.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.6) (ISO 8601 &#x60;P1D&#x60;, &#x60;PT1H&#x60;, …; no months/years) or compact tokens &#x60;s&#x60;/&#x60;m&#x60;/&#x60;h&#x60;/&#x60;d&#x60;/&#x60;w&#x60; (&#x60;M&#x60; is minutes). Max 3650d (10 years). &#x60;1y&#x60; is rejected. Measured from resolved &#x60;valid_from&#x60;. Mutually exclusive with &#x60;valid_to&#x60;. | [optional]
**Id** | Pointer to **NullableString** |  | [optional]
**InviteRecurrence** | Pointer to **interface{}** |  | [optional]
**IsEnabled** | Pointer to **bool** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**StartsIn** | Pointer to **NullableString** | Duration per [RFC 5545 §3.3.6](https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.6) (ISO 8601 &#x60;P1D&#x60;, &#x60;PT1H&#x60;, …; no months/years) or compact tokens &#x60;s&#x60;/&#x60;m&#x60;/&#x60;h&#x60;/&#x60;d&#x60;/&#x60;w&#x60; (&#x60;M&#x60; is minutes). Max 3650d (10 years). &#x60;1y&#x60; is rejected. Mutually exclusive with &#x60;valid_from&#x60;. | [optional]
**ValidFrom** | Pointer to **NullableString** | [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) start (UTC). Mutually exclusive with &#x60;starts_in&#x60;. | [optional]
**ValidTo** | Pointer to **NullableString** | [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339) end (UTC). Mutually exclusive with &#x60;expires_in&#x60;. | [optional]

## Methods

### NewInviteScheduleEntryInput

`func NewInviteScheduleEntryInput() *InviteScheduleEntryInput`

NewInviteScheduleEntryInput instantiates a new InviteScheduleEntryInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInviteScheduleEntryInputWithDefaults

`func NewInviteScheduleEntryInputWithDefaults() *InviteScheduleEntryInput`

NewInviteScheduleEntryInputWithDefaults instantiates a new InviteScheduleEntryInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresIn

`func (o *InviteScheduleEntryInput) GetExpiresIn() string`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *InviteScheduleEntryInput) GetExpiresInOk() (*string, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *InviteScheduleEntryInput) SetExpiresIn(v string)`

SetExpiresIn sets ExpiresIn field to given value.

### HasExpiresIn

`func (o *InviteScheduleEntryInput) HasExpiresIn() bool`

HasExpiresIn returns a boolean if a field has been set.

### SetExpiresInNil

`func (o *InviteScheduleEntryInput) SetExpiresInNil(b bool)`

 SetExpiresInNil sets the value for ExpiresIn to be an explicit nil

### UnsetExpiresIn
`func (o *InviteScheduleEntryInput) UnsetExpiresIn()`

UnsetExpiresIn ensures that no value is present for ExpiresIn, not even an explicit nil
### GetId

`func (o *InviteScheduleEntryInput) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *InviteScheduleEntryInput) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *InviteScheduleEntryInput) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *InviteScheduleEntryInput) HasId() bool`

HasId returns a boolean if a field has been set.

### SetIdNil

`func (o *InviteScheduleEntryInput) SetIdNil(b bool)`

 SetIdNil sets the value for Id to be an explicit nil

### UnsetId
`func (o *InviteScheduleEntryInput) UnsetId()`

UnsetId ensures that no value is present for Id, not even an explicit nil
### GetInviteRecurrence

`func (o *InviteScheduleEntryInput) GetInviteRecurrence() interface{}`

GetInviteRecurrence returns the InviteRecurrence field if non-nil, zero value otherwise.

### GetInviteRecurrenceOk

`func (o *InviteScheduleEntryInput) GetInviteRecurrenceOk() (*interface{}, bool)`

GetInviteRecurrenceOk returns a tuple with the InviteRecurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInviteRecurrence

`func (o *InviteScheduleEntryInput) SetInviteRecurrence(v interface{})`

SetInviteRecurrence sets InviteRecurrence field to given value.

### HasInviteRecurrence

`func (o *InviteScheduleEntryInput) HasInviteRecurrence() bool`

HasInviteRecurrence returns a boolean if a field has been set.

### SetInviteRecurrenceNil

`func (o *InviteScheduleEntryInput) SetInviteRecurrenceNil(b bool)`

 SetInviteRecurrenceNil sets the value for InviteRecurrence to be an explicit nil

### UnsetInviteRecurrence
`func (o *InviteScheduleEntryInput) UnsetInviteRecurrence()`

UnsetInviteRecurrence ensures that no value is present for InviteRecurrence, not even an explicit nil
### GetIsEnabled

`func (o *InviteScheduleEntryInput) GetIsEnabled() bool`

GetIsEnabled returns the IsEnabled field if non-nil, zero value otherwise.

### GetIsEnabledOk

`func (o *InviteScheduleEntryInput) GetIsEnabledOk() (*bool, bool)`

GetIsEnabledOk returns a tuple with the IsEnabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsEnabled

`func (o *InviteScheduleEntryInput) SetIsEnabled(v bool)`

SetIsEnabled sets IsEnabled field to given value.

### HasIsEnabled

`func (o *InviteScheduleEntryInput) HasIsEnabled() bool`

HasIsEnabled returns a boolean if a field has been set.

### GetName

`func (o *InviteScheduleEntryInput) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *InviteScheduleEntryInput) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *InviteScheduleEntryInput) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *InviteScheduleEntryInput) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *InviteScheduleEntryInput) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *InviteScheduleEntryInput) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetStartsIn

`func (o *InviteScheduleEntryInput) GetStartsIn() string`

GetStartsIn returns the StartsIn field if non-nil, zero value otherwise.

### GetStartsInOk

`func (o *InviteScheduleEntryInput) GetStartsInOk() (*string, bool)`

GetStartsInOk returns a tuple with the StartsIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStartsIn

`func (o *InviteScheduleEntryInput) SetStartsIn(v string)`

SetStartsIn sets StartsIn field to given value.

### HasStartsIn

`func (o *InviteScheduleEntryInput) HasStartsIn() bool`

HasStartsIn returns a boolean if a field has been set.

### SetStartsInNil

`func (o *InviteScheduleEntryInput) SetStartsInNil(b bool)`

 SetStartsInNil sets the value for StartsIn to be an explicit nil

### UnsetStartsIn
`func (o *InviteScheduleEntryInput) UnsetStartsIn()`

UnsetStartsIn ensures that no value is present for StartsIn, not even an explicit nil
### GetValidFrom

`func (o *InviteScheduleEntryInput) GetValidFrom() string`

GetValidFrom returns the ValidFrom field if non-nil, zero value otherwise.

### GetValidFromOk

`func (o *InviteScheduleEntryInput) GetValidFromOk() (*string, bool)`

GetValidFromOk returns a tuple with the ValidFrom field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidFrom

`func (o *InviteScheduleEntryInput) SetValidFrom(v string)`

SetValidFrom sets ValidFrom field to given value.

### HasValidFrom

`func (o *InviteScheduleEntryInput) HasValidFrom() bool`

HasValidFrom returns a boolean if a field has been set.

### SetValidFromNil

`func (o *InviteScheduleEntryInput) SetValidFromNil(b bool)`

 SetValidFromNil sets the value for ValidFrom to be an explicit nil

### UnsetValidFrom
`func (o *InviteScheduleEntryInput) UnsetValidFrom()`

UnsetValidFrom ensures that no value is present for ValidFrom, not even an explicit nil
### GetValidTo

`func (o *InviteScheduleEntryInput) GetValidTo() string`

GetValidTo returns the ValidTo field if non-nil, zero value otherwise.

### GetValidToOk

`func (o *InviteScheduleEntryInput) GetValidToOk() (*string, bool)`

GetValidToOk returns a tuple with the ValidTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidTo

`func (o *InviteScheduleEntryInput) SetValidTo(v string)`

SetValidTo sets ValidTo field to given value.

### HasValidTo

`func (o *InviteScheduleEntryInput) HasValidTo() bool`

HasValidTo returns a boolean if a field has been set.

### SetValidToNil

`func (o *InviteScheduleEntryInput) SetValidToNil(b bool)`

 SetValidToNil sets the value for ValidTo to be an explicit nil

### UnsetValidTo
`func (o *InviteScheduleEntryInput) UnsetValidTo()`

UnsetValidTo ensures that no value is present for ValidTo, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
