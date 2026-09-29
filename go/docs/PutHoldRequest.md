# PutHoldRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IncludeLights** | Pointer to **NullableBool** |  | [optional]
**Kind** | Pointer to **NullableString** |  | [optional]
**Mode** | **string** |  |
**Reason** | Pointer to **NullableString** |  | [optional]
**Recurrence** | Pointer to **map[string]interface{}** |  | [optional]
**Until** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPutHoldRequest

`func NewPutHoldRequest(mode string, ) *PutHoldRequest`

NewPutHoldRequest instantiates a new PutHoldRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPutHoldRequestWithDefaults

`func NewPutHoldRequestWithDefaults() *PutHoldRequest`

NewPutHoldRequestWithDefaults instantiates a new PutHoldRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIncludeLights

`func (o *PutHoldRequest) GetIncludeLights() bool`

GetIncludeLights returns the IncludeLights field if non-nil, zero value otherwise.

### GetIncludeLightsOk

`func (o *PutHoldRequest) GetIncludeLightsOk() (*bool, bool)`

GetIncludeLightsOk returns a tuple with the IncludeLights field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeLights

`func (o *PutHoldRequest) SetIncludeLights(v bool)`

SetIncludeLights sets IncludeLights field to given value.

### HasIncludeLights

`func (o *PutHoldRequest) HasIncludeLights() bool`

HasIncludeLights returns a boolean if a field has been set.

### SetIncludeLightsNil

`func (o *PutHoldRequest) SetIncludeLightsNil(b bool)`

 SetIncludeLightsNil sets the value for IncludeLights to be an explicit nil

### UnsetIncludeLights
`func (o *PutHoldRequest) UnsetIncludeLights()`

UnsetIncludeLights ensures that no value is present for IncludeLights, not even an explicit nil
### GetKind

`func (o *PutHoldRequest) GetKind() string`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *PutHoldRequest) GetKindOk() (*string, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *PutHoldRequest) SetKind(v string)`

SetKind sets Kind field to given value.

### HasKind

`func (o *PutHoldRequest) HasKind() bool`

HasKind returns a boolean if a field has been set.

### SetKindNil

`func (o *PutHoldRequest) SetKindNil(b bool)`

 SetKindNil sets the value for Kind to be an explicit nil

### UnsetKind
`func (o *PutHoldRequest) UnsetKind()`

UnsetKind ensures that no value is present for Kind, not even an explicit nil
### GetMode

`func (o *PutHoldRequest) GetMode() string`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *PutHoldRequest) GetModeOk() (*string, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *PutHoldRequest) SetMode(v string)`

SetMode sets Mode field to given value.


### GetReason

`func (o *PutHoldRequest) GetReason() string`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *PutHoldRequest) GetReasonOk() (*string, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *PutHoldRequest) SetReason(v string)`

SetReason sets Reason field to given value.

### HasReason

`func (o *PutHoldRequest) HasReason() bool`

HasReason returns a boolean if a field has been set.

### SetReasonNil

`func (o *PutHoldRequest) SetReasonNil(b bool)`

 SetReasonNil sets the value for Reason to be an explicit nil

### UnsetReason
`func (o *PutHoldRequest) UnsetReason()`

UnsetReason ensures that no value is present for Reason, not even an explicit nil
### GetRecurrence

`func (o *PutHoldRequest) GetRecurrence() map[string]interface{}`

GetRecurrence returns the Recurrence field if non-nil, zero value otherwise.

### GetRecurrenceOk

`func (o *PutHoldRequest) GetRecurrenceOk() (*map[string]interface{}, bool)`

GetRecurrenceOk returns a tuple with the Recurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecurrence

`func (o *PutHoldRequest) SetRecurrence(v map[string]interface{})`

SetRecurrence sets Recurrence field to given value.

### HasRecurrence

`func (o *PutHoldRequest) HasRecurrence() bool`

HasRecurrence returns a boolean if a field has been set.

### SetRecurrenceNil

`func (o *PutHoldRequest) SetRecurrenceNil(b bool)`

 SetRecurrenceNil sets the value for Recurrence to be an explicit nil

### UnsetRecurrence
`func (o *PutHoldRequest) UnsetRecurrence()`

UnsetRecurrence ensures that no value is present for Recurrence, not even an explicit nil
### GetUntil

`func (o *PutHoldRequest) GetUntil() string`

GetUntil returns the Until field if non-nil, zero value otherwise.

### GetUntilOk

`func (o *PutHoldRequest) GetUntilOk() (*string, bool)`

GetUntilOk returns a tuple with the Until field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUntil

`func (o *PutHoldRequest) SetUntil(v string)`

SetUntil sets Until field to given value.

### HasUntil

`func (o *PutHoldRequest) HasUntil() bool`

HasUntil returns a boolean if a field has been set.

### SetUntilNil

`func (o *PutHoldRequest) SetUntilNil(b bool)`

 SetUntilNil sets the value for Until to be an explicit nil

### UnsetUntil
`func (o *PutHoldRequest) UnsetUntil()`

UnsetUntil ensures that no value is present for Until, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
