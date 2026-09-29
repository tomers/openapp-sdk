# EntityHoldItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**EntityId** | **string** |  |
**Hold** | [**HoldView**](HoldView.md) |  |

## Methods

### NewEntityHoldItem

`func NewEntityHoldItem(entityId string, hold HoldView, ) *EntityHoldItem`

NewEntityHoldItem instantiates a new EntityHoldItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewEntityHoldItemWithDefaults

`func NewEntityHoldItemWithDefaults() *EntityHoldItem`

NewEntityHoldItemWithDefaults instantiates a new EntityHoldItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEntityId

`func (o *EntityHoldItem) GetEntityId() string`

GetEntityId returns the EntityId field if non-nil, zero value otherwise.

### GetEntityIdOk

`func (o *EntityHoldItem) GetEntityIdOk() (*string, bool)`

GetEntityIdOk returns a tuple with the EntityId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityId

`func (o *EntityHoldItem) SetEntityId(v string)`

SetEntityId sets EntityId field to given value.


### GetHold

`func (o *EntityHoldItem) GetHold() HoldView`

GetHold returns the Hold field if non-nil, zero value otherwise.

### GetHoldOk

`func (o *EntityHoldItem) GetHoldOk() (*HoldView, bool)`

GetHoldOk returns a tuple with the Hold field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHold

`func (o *EntityHoldItem) SetHold(v HoldView)`

SetHold sets Hold field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
