# PaginatedResponseStoreMemberItemsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BillingAccountId** | Pointer to **NullableString** | Account purchases are charged to. &#x60;None&#x60; falls back to the member&#39;s personal account. | [optional]
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**RemovedAt** | Pointer to **NullableString** |  | [optional]
**Role** | [**StoreMemberRole**](StoreMemberRole.md) |  |
**Status** | [**StoreMemberStatus**](StoreMemberStatus.md) |  |
**StoreId** | **string** |  |
**UpdatedAt** | Pointer to **NullableString** |  | [optional]
**UserId** | **string** |  |

## Methods

### NewPaginatedResponseStoreMemberItemsInner

`func NewPaginatedResponseStoreMemberItemsInner(id string, role StoreMemberRole, status StoreMemberStatus, storeId string, userId string, ) *PaginatedResponseStoreMemberItemsInner`

NewPaginatedResponseStoreMemberItemsInner instantiates a new PaginatedResponseStoreMemberItemsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPaginatedResponseStoreMemberItemsInnerWithDefaults

`func NewPaginatedResponseStoreMemberItemsInnerWithDefaults() *PaginatedResponseStoreMemberItemsInner`

NewPaginatedResponseStoreMemberItemsInnerWithDefaults instantiates a new PaginatedResponseStoreMemberItemsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBillingAccountId

`func (o *PaginatedResponseStoreMemberItemsInner) GetBillingAccountId() string`

GetBillingAccountId returns the BillingAccountId field if non-nil, zero value otherwise.

### GetBillingAccountIdOk

`func (o *PaginatedResponseStoreMemberItemsInner) GetBillingAccountIdOk() (*string, bool)`

GetBillingAccountIdOk returns a tuple with the BillingAccountId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBillingAccountId

`func (o *PaginatedResponseStoreMemberItemsInner) SetBillingAccountId(v string)`

SetBillingAccountId sets BillingAccountId field to given value.

### HasBillingAccountId

`func (o *PaginatedResponseStoreMemberItemsInner) HasBillingAccountId() bool`

HasBillingAccountId returns a boolean if a field has been set.

### SetBillingAccountIdNil

`func (o *PaginatedResponseStoreMemberItemsInner) SetBillingAccountIdNil(b bool)`

 SetBillingAccountIdNil sets the value for BillingAccountId to be an explicit nil

### UnsetBillingAccountId
`func (o *PaginatedResponseStoreMemberItemsInner) UnsetBillingAccountId()`

UnsetBillingAccountId ensures that no value is present for BillingAccountId, not even an explicit nil
### GetCreatedAt

`func (o *PaginatedResponseStoreMemberItemsInner) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PaginatedResponseStoreMemberItemsInner) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PaginatedResponseStoreMemberItemsInner) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *PaginatedResponseStoreMemberItemsInner) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *PaginatedResponseStoreMemberItemsInner) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *PaginatedResponseStoreMemberItemsInner) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetId

`func (o *PaginatedResponseStoreMemberItemsInner) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PaginatedResponseStoreMemberItemsInner) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PaginatedResponseStoreMemberItemsInner) SetId(v string)`

SetId sets Id field to given value.


### GetRemovedAt

`func (o *PaginatedResponseStoreMemberItemsInner) GetRemovedAt() string`

GetRemovedAt returns the RemovedAt field if non-nil, zero value otherwise.

### GetRemovedAtOk

`func (o *PaginatedResponseStoreMemberItemsInner) GetRemovedAtOk() (*string, bool)`

GetRemovedAtOk returns a tuple with the RemovedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRemovedAt

`func (o *PaginatedResponseStoreMemberItemsInner) SetRemovedAt(v string)`

SetRemovedAt sets RemovedAt field to given value.

### HasRemovedAt

`func (o *PaginatedResponseStoreMemberItemsInner) HasRemovedAt() bool`

HasRemovedAt returns a boolean if a field has been set.

### SetRemovedAtNil

`func (o *PaginatedResponseStoreMemberItemsInner) SetRemovedAtNil(b bool)`

 SetRemovedAtNil sets the value for RemovedAt to be an explicit nil

### UnsetRemovedAt
`func (o *PaginatedResponseStoreMemberItemsInner) UnsetRemovedAt()`

UnsetRemovedAt ensures that no value is present for RemovedAt, not even an explicit nil
### GetRole

`func (o *PaginatedResponseStoreMemberItemsInner) GetRole() StoreMemberRole`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *PaginatedResponseStoreMemberItemsInner) GetRoleOk() (*StoreMemberRole, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *PaginatedResponseStoreMemberItemsInner) SetRole(v StoreMemberRole)`

SetRole sets Role field to given value.


### GetStatus

`func (o *PaginatedResponseStoreMemberItemsInner) GetStatus() StoreMemberStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *PaginatedResponseStoreMemberItemsInner) GetStatusOk() (*StoreMemberStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *PaginatedResponseStoreMemberItemsInner) SetStatus(v StoreMemberStatus)`

SetStatus sets Status field to given value.


### GetStoreId

`func (o *PaginatedResponseStoreMemberItemsInner) GetStoreId() string`

GetStoreId returns the StoreId field if non-nil, zero value otherwise.

### GetStoreIdOk

`func (o *PaginatedResponseStoreMemberItemsInner) GetStoreIdOk() (*string, bool)`

GetStoreIdOk returns a tuple with the StoreId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStoreId

`func (o *PaginatedResponseStoreMemberItemsInner) SetStoreId(v string)`

SetStoreId sets StoreId field to given value.


### GetUpdatedAt

`func (o *PaginatedResponseStoreMemberItemsInner) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PaginatedResponseStoreMemberItemsInner) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PaginatedResponseStoreMemberItemsInner) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *PaginatedResponseStoreMemberItemsInner) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *PaginatedResponseStoreMemberItemsInner) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *PaginatedResponseStoreMemberItemsInner) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil
### GetUserId

`func (o *PaginatedResponseStoreMemberItemsInner) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *PaginatedResponseStoreMemberItemsInner) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *PaginatedResponseStoreMemberItemsInner) SetUserId(v string)`

SetUserId sets UserId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
