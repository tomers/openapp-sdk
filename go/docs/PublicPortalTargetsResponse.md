# PublicPortalTargetsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**FloorOrder** | Pointer to **[]string** | Building &#x60;floor_order&#x60; from virtual_access integration config (canonical keys). | [optional]
**Message** | Pointer to **interface{}** |  | [optional]
**Status** | **string** | One of &#x60;ready&#x60;, &#x60;empty&#x60;, &#x60;not_configured&#x60;, &#x60;disabled&#x60;. Each carries its own visitor copy, so a building with no occupants reads differently from an intercom that was never set up. |
**Targets** | [**[]PublicPortalTarget**](PublicPortalTarget.md) |  |

## Methods

### NewPublicPortalTargetsResponse

`func NewPublicPortalTargetsResponse(status string, targets []PublicPortalTarget, ) *PublicPortalTargetsResponse`

NewPublicPortalTargetsResponse instantiates a new PublicPortalTargetsResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicPortalTargetsResponseWithDefaults

`func NewPublicPortalTargetsResponseWithDefaults() *PublicPortalTargetsResponse`

NewPublicPortalTargetsResponseWithDefaults instantiates a new PublicPortalTargetsResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFloorOrder

`func (o *PublicPortalTargetsResponse) GetFloorOrder() []string`

GetFloorOrder returns the FloorOrder field if non-nil, zero value otherwise.

### GetFloorOrderOk

`func (o *PublicPortalTargetsResponse) GetFloorOrderOk() (*[]string, bool)`

GetFloorOrderOk returns a tuple with the FloorOrder field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFloorOrder

`func (o *PublicPortalTargetsResponse) SetFloorOrder(v []string)`

SetFloorOrder sets FloorOrder field to given value.

### HasFloorOrder

`func (o *PublicPortalTargetsResponse) HasFloorOrder() bool`

HasFloorOrder returns a boolean if a field has been set.

### SetFloorOrderNil

`func (o *PublicPortalTargetsResponse) SetFloorOrderNil(b bool)`

 SetFloorOrderNil sets the value for FloorOrder to be an explicit nil

### UnsetFloorOrder
`func (o *PublicPortalTargetsResponse) UnsetFloorOrder()`

UnsetFloorOrder ensures that no value is present for FloorOrder, not even an explicit nil
### GetMessage

`func (o *PublicPortalTargetsResponse) GetMessage() interface{}`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *PublicPortalTargetsResponse) GetMessageOk() (*interface{}, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *PublicPortalTargetsResponse) SetMessage(v interface{})`

SetMessage sets Message field to given value.

### HasMessage

`func (o *PublicPortalTargetsResponse) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### SetMessageNil

`func (o *PublicPortalTargetsResponse) SetMessageNil(b bool)`

 SetMessageNil sets the value for Message to be an explicit nil

### UnsetMessage
`func (o *PublicPortalTargetsResponse) UnsetMessage()`

UnsetMessage ensures that no value is present for Message, not even an explicit nil
### GetStatus

`func (o *PublicPortalTargetsResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *PublicPortalTargetsResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *PublicPortalTargetsResponse) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetTargets

`func (o *PublicPortalTargetsResponse) GetTargets() []PublicPortalTarget`

GetTargets returns the Targets field if non-nil, zero value otherwise.

### GetTargetsOk

`func (o *PublicPortalTargetsResponse) GetTargetsOk() (*[]PublicPortalTarget, bool)`

GetTargetsOk returns a tuple with the Targets field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTargets

`func (o *PublicPortalTargetsResponse) SetTargets(v []PublicPortalTarget)`

SetTargets sets Targets field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
