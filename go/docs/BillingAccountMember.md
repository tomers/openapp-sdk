# BillingAccountMember

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AccountId** | **NullableString** |  |
**CanCharge** | **bool** |  |
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**UserId** | **NullableString** |  |

## Methods

### NewBillingAccountMember

`func NewBillingAccountMember(accountId NullableString, canCharge bool, userId NullableString, ) *BillingAccountMember`

NewBillingAccountMember instantiates a new BillingAccountMember object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBillingAccountMemberWithDefaults

`func NewBillingAccountMemberWithDefaults() *BillingAccountMember`

NewBillingAccountMemberWithDefaults instantiates a new BillingAccountMember object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAccountId

`func (o *BillingAccountMember) GetAccountId() string`

GetAccountId returns the AccountId field if non-nil, zero value otherwise.

### GetAccountIdOk

`func (o *BillingAccountMember) GetAccountIdOk() (*string, bool)`

GetAccountIdOk returns a tuple with the AccountId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAccountId

`func (o *BillingAccountMember) SetAccountId(v string)`

SetAccountId sets AccountId field to given value.


### SetAccountIdNil

`func (o *BillingAccountMember) SetAccountIdNil(b bool)`

 SetAccountIdNil sets the value for AccountId to be an explicit nil

### UnsetAccountId
`func (o *BillingAccountMember) UnsetAccountId()`

UnsetAccountId ensures that no value is present for AccountId, not even an explicit nil
### GetCanCharge

`func (o *BillingAccountMember) GetCanCharge() bool`

GetCanCharge returns the CanCharge field if non-nil, zero value otherwise.

### GetCanChargeOk

`func (o *BillingAccountMember) GetCanChargeOk() (*bool, bool)`

GetCanChargeOk returns a tuple with the CanCharge field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanCharge

`func (o *BillingAccountMember) SetCanCharge(v bool)`

SetCanCharge sets CanCharge field to given value.


### GetCreatedAt

`func (o *BillingAccountMember) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *BillingAccountMember) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *BillingAccountMember) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *BillingAccountMember) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *BillingAccountMember) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *BillingAccountMember) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetUserId

`func (o *BillingAccountMember) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *BillingAccountMember) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *BillingAccountMember) SetUserId(v string)`

SetUserId sets UserId field to given value.


### SetUserIdNil

`func (o *BillingAccountMember) SetUserIdNil(b bool)`

 SetUserIdNil sets the value for UserId to be an explicit nil

### UnsetUserId
`func (o *BillingAccountMember) UnsetUserId()`

UnsetUserId ensures that no value is present for UserId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
