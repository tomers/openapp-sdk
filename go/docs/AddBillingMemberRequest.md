# AddBillingMemberRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CanCharge** | Pointer to **NullableBool** |  | [optional]
**UserId** | **string** |  |

## Methods

### NewAddBillingMemberRequest

`func NewAddBillingMemberRequest(userId string, ) *AddBillingMemberRequest`

NewAddBillingMemberRequest instantiates a new AddBillingMemberRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAddBillingMemberRequestWithDefaults

`func NewAddBillingMemberRequestWithDefaults() *AddBillingMemberRequest`

NewAddBillingMemberRequestWithDefaults instantiates a new AddBillingMemberRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCanCharge

`func (o *AddBillingMemberRequest) GetCanCharge() bool`

GetCanCharge returns the CanCharge field if non-nil, zero value otherwise.

### GetCanChargeOk

`func (o *AddBillingMemberRequest) GetCanChargeOk() (*bool, bool)`

GetCanChargeOk returns a tuple with the CanCharge field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanCharge

`func (o *AddBillingMemberRequest) SetCanCharge(v bool)`

SetCanCharge sets CanCharge field to given value.

### HasCanCharge

`func (o *AddBillingMemberRequest) HasCanCharge() bool`

HasCanCharge returns a boolean if a field has been set.

### SetCanChargeNil

`func (o *AddBillingMemberRequest) SetCanChargeNil(b bool)`

 SetCanChargeNil sets the value for CanCharge to be an explicit nil

### UnsetCanCharge
`func (o *AddBillingMemberRequest) UnsetCanCharge()`

UnsetCanCharge ensures that no value is present for CanCharge, not even an explicit nil
### GetUserId

`func (o *AddBillingMemberRequest) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *AddBillingMemberRequest) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *AddBillingMemberRequest) SetUserId(v string)`

SetUserId sets UserId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
