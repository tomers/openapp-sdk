# PublicHoldView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Mode** | [**HoldMode**](HoldMode.md) |  |
**Reason** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPublicHoldView

`func NewPublicHoldView(mode HoldMode, ) *PublicHoldView`

NewPublicHoldView instantiates a new PublicHoldView object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicHoldViewWithDefaults

`func NewPublicHoldViewWithDefaults() *PublicHoldView`

NewPublicHoldViewWithDefaults instantiates a new PublicHoldView object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMode

`func (o *PublicHoldView) GetMode() HoldMode`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *PublicHoldView) GetModeOk() (*HoldMode, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *PublicHoldView) SetMode(v HoldMode)`

SetMode sets Mode field to given value.


### GetReason

`func (o *PublicHoldView) GetReason() string`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *PublicHoldView) GetReasonOk() (*string, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *PublicHoldView) SetReason(v string)`

SetReason sets Reason field to given value.

### HasReason

`func (o *PublicHoldView) HasReason() bool`

HasReason returns a boolean if a field has been set.

### SetReasonNil

`func (o *PublicHoldView) SetReasonNil(b bool)`

 SetReasonNil sets the value for Reason to be an explicit nil

### UnsetReason
`func (o *PublicHoldView) UnsetReason()`

UnsetReason ensures that no value is present for Reason, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
