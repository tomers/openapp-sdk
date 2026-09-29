# HoldView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**EntityDirect** | **bool** |  |
**HeldByDoorIds** | **[]string** |  |
**Kind** | [**HoldKind**](HoldKind.md) |  |
**Mode** | [**HoldMode**](HoldMode.md) |  |
**NextWindow** | Pointer to [**NullableHoldWindowView**](HoldWindowView.md) |  | [optional]
**Reason** | Pointer to **NullableString** |  | [optional]
**Recurrence** | Pointer to **map[string]interface{}** | Authored recurrence (always &#x60;freq: weekly&#x60; and an explicit ISO day list). | [optional]
**SetAt** | **string** |  |
**SetBy** | Pointer to **NullableString** |  | [optional]
**SetByDisplayName** | Pointer to **NullableString** |  | [optional]
**Timezone** | **string** | IANA timezone for authoring: the spec&#39;s zone when recurring, otherwise the org zone (&#x60;UTC&#x60; if the org has none). Never the caller&#39;s browser zone. |
**Until** | Pointer to **NullableString** |  | [optional]

## Methods

### NewHoldView

`func NewHoldView(entityDirect bool, heldByDoorIds []string, kind HoldKind, mode HoldMode, setAt string, timezone string, ) *HoldView`

NewHoldView instantiates a new HoldView object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewHoldViewWithDefaults

`func NewHoldViewWithDefaults() *HoldView`

NewHoldViewWithDefaults instantiates a new HoldView object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEntityDirect

`func (o *HoldView) GetEntityDirect() bool`

GetEntityDirect returns the EntityDirect field if non-nil, zero value otherwise.

### GetEntityDirectOk

`func (o *HoldView) GetEntityDirectOk() (*bool, bool)`

GetEntityDirectOk returns a tuple with the EntityDirect field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityDirect

`func (o *HoldView) SetEntityDirect(v bool)`

SetEntityDirect sets EntityDirect field to given value.


### GetHeldByDoorIds

`func (o *HoldView) GetHeldByDoorIds() []string`

GetHeldByDoorIds returns the HeldByDoorIds field if non-nil, zero value otherwise.

### GetHeldByDoorIdsOk

`func (o *HoldView) GetHeldByDoorIdsOk() (*[]string, bool)`

GetHeldByDoorIdsOk returns a tuple with the HeldByDoorIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHeldByDoorIds

`func (o *HoldView) SetHeldByDoorIds(v []string)`

SetHeldByDoorIds sets HeldByDoorIds field to given value.


### GetKind

`func (o *HoldView) GetKind() HoldKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *HoldView) GetKindOk() (*HoldKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *HoldView) SetKind(v HoldKind)`

SetKind sets Kind field to given value.


### GetMode

`func (o *HoldView) GetMode() HoldMode`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *HoldView) GetModeOk() (*HoldMode, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *HoldView) SetMode(v HoldMode)`

SetMode sets Mode field to given value.


### GetNextWindow

`func (o *HoldView) GetNextWindow() HoldWindowView`

GetNextWindow returns the NextWindow field if non-nil, zero value otherwise.

### GetNextWindowOk

`func (o *HoldView) GetNextWindowOk() (*HoldWindowView, bool)`

GetNextWindowOk returns a tuple with the NextWindow field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextWindow

`func (o *HoldView) SetNextWindow(v HoldWindowView)`

SetNextWindow sets NextWindow field to given value.

### HasNextWindow

`func (o *HoldView) HasNextWindow() bool`

HasNextWindow returns a boolean if a field has been set.

### SetNextWindowNil

`func (o *HoldView) SetNextWindowNil(b bool)`

 SetNextWindowNil sets the value for NextWindow to be an explicit nil

### UnsetNextWindow
`func (o *HoldView) UnsetNextWindow()`

UnsetNextWindow ensures that no value is present for NextWindow, not even an explicit nil
### GetReason

`func (o *HoldView) GetReason() string`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *HoldView) GetReasonOk() (*string, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *HoldView) SetReason(v string)`

SetReason sets Reason field to given value.

### HasReason

`func (o *HoldView) HasReason() bool`

HasReason returns a boolean if a field has been set.

### SetReasonNil

`func (o *HoldView) SetReasonNil(b bool)`

 SetReasonNil sets the value for Reason to be an explicit nil

### UnsetReason
`func (o *HoldView) UnsetReason()`

UnsetReason ensures that no value is present for Reason, not even an explicit nil
### GetRecurrence

`func (o *HoldView) GetRecurrence() map[string]interface{}`

GetRecurrence returns the Recurrence field if non-nil, zero value otherwise.

### GetRecurrenceOk

`func (o *HoldView) GetRecurrenceOk() (*map[string]interface{}, bool)`

GetRecurrenceOk returns a tuple with the Recurrence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecurrence

`func (o *HoldView) SetRecurrence(v map[string]interface{})`

SetRecurrence sets Recurrence field to given value.

### HasRecurrence

`func (o *HoldView) HasRecurrence() bool`

HasRecurrence returns a boolean if a field has been set.

### SetRecurrenceNil

`func (o *HoldView) SetRecurrenceNil(b bool)`

 SetRecurrenceNil sets the value for Recurrence to be an explicit nil

### UnsetRecurrence
`func (o *HoldView) UnsetRecurrence()`

UnsetRecurrence ensures that no value is present for Recurrence, not even an explicit nil
### GetSetAt

`func (o *HoldView) GetSetAt() string`

GetSetAt returns the SetAt field if non-nil, zero value otherwise.

### GetSetAtOk

`func (o *HoldView) GetSetAtOk() (*string, bool)`

GetSetAtOk returns a tuple with the SetAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSetAt

`func (o *HoldView) SetSetAt(v string)`

SetSetAt sets SetAt field to given value.


### GetSetBy

`func (o *HoldView) GetSetBy() string`

GetSetBy returns the SetBy field if non-nil, zero value otherwise.

### GetSetByOk

`func (o *HoldView) GetSetByOk() (*string, bool)`

GetSetByOk returns a tuple with the SetBy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSetBy

`func (o *HoldView) SetSetBy(v string)`

SetSetBy sets SetBy field to given value.

### HasSetBy

`func (o *HoldView) HasSetBy() bool`

HasSetBy returns a boolean if a field has been set.

### SetSetByNil

`func (o *HoldView) SetSetByNil(b bool)`

 SetSetByNil sets the value for SetBy to be an explicit nil

### UnsetSetBy
`func (o *HoldView) UnsetSetBy()`

UnsetSetBy ensures that no value is present for SetBy, not even an explicit nil
### GetSetByDisplayName

`func (o *HoldView) GetSetByDisplayName() string`

GetSetByDisplayName returns the SetByDisplayName field if non-nil, zero value otherwise.

### GetSetByDisplayNameOk

`func (o *HoldView) GetSetByDisplayNameOk() (*string, bool)`

GetSetByDisplayNameOk returns a tuple with the SetByDisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSetByDisplayName

`func (o *HoldView) SetSetByDisplayName(v string)`

SetSetByDisplayName sets SetByDisplayName field to given value.

### HasSetByDisplayName

`func (o *HoldView) HasSetByDisplayName() bool`

HasSetByDisplayName returns a boolean if a field has been set.

### SetSetByDisplayNameNil

`func (o *HoldView) SetSetByDisplayNameNil(b bool)`

 SetSetByDisplayNameNil sets the value for SetByDisplayName to be an explicit nil

### UnsetSetByDisplayName
`func (o *HoldView) UnsetSetByDisplayName()`

UnsetSetByDisplayName ensures that no value is present for SetByDisplayName, not even an explicit nil
### GetTimezone

`func (o *HoldView) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *HoldView) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *HoldView) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.


### GetUntil

`func (o *HoldView) GetUntil() string`

GetUntil returns the Until field if non-nil, zero value otherwise.

### GetUntilOk

`func (o *HoldView) GetUntilOk() (*string, bool)`

GetUntilOk returns a tuple with the Until field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUntil

`func (o *HoldView) SetUntil(v string)`

SetUntil sets Until field to given value.

### HasUntil

`func (o *HoldView) HasUntil() bool`

HasUntil returns a boolean if a field has been set.

### SetUntilNil

`func (o *HoldView) SetUntilNil(b bool)`

 SetUntilNil sets the value for Until to be an explicit nil

### UnsetUntil
`func (o *HoldView) UnsetUntil()`

UnsetUntil ensures that no value is present for Until, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
