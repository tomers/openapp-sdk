# PublicStoreResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Currency** | **string** |  |
**IsMember** | **bool** | Whether the authenticated caller (if any) is already an active member. |
**Items** | [**[]PublicCatalogItem**](PublicCatalogItem.md) |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**PublicId** | **string** |  |
**RegistrationRequested** | **bool** | Whether the authenticated caller (if any) already has a pending registration request. |
**StoreId** | **string** |  |

## Methods

### NewPublicStoreResponse

`func NewPublicStoreResponse(currency string, isMember bool, items []PublicCatalogItem, name LocalizedString, publicId string, registrationRequested bool, storeId string, ) *PublicStoreResponse`

NewPublicStoreResponse instantiates a new PublicStoreResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicStoreResponseWithDefaults

`func NewPublicStoreResponseWithDefaults() *PublicStoreResponse`

NewPublicStoreResponseWithDefaults instantiates a new PublicStoreResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCurrency

`func (o *PublicStoreResponse) GetCurrency() string`

GetCurrency returns the Currency field if non-nil, zero value otherwise.

### GetCurrencyOk

`func (o *PublicStoreResponse) GetCurrencyOk() (*string, bool)`

GetCurrencyOk returns a tuple with the Currency field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrency

`func (o *PublicStoreResponse) SetCurrency(v string)`

SetCurrency sets Currency field to given value.


### GetIsMember

`func (o *PublicStoreResponse) GetIsMember() bool`

GetIsMember returns the IsMember field if non-nil, zero value otherwise.

### GetIsMemberOk

`func (o *PublicStoreResponse) GetIsMemberOk() (*bool, bool)`

GetIsMemberOk returns a tuple with the IsMember field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsMember

`func (o *PublicStoreResponse) SetIsMember(v bool)`

SetIsMember sets IsMember field to given value.


### GetItems

`func (o *PublicStoreResponse) GetItems() []PublicCatalogItem`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *PublicStoreResponse) GetItemsOk() (*[]PublicCatalogItem, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *PublicStoreResponse) SetItems(v []PublicCatalogItem)`

SetItems sets Items field to given value.


### GetName

`func (o *PublicStoreResponse) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PublicStoreResponse) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PublicStoreResponse) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPublicId

`func (o *PublicStoreResponse) GetPublicId() string`

GetPublicId returns the PublicId field if non-nil, zero value otherwise.

### GetPublicIdOk

`func (o *PublicStoreResponse) GetPublicIdOk() (*string, bool)`

GetPublicIdOk returns a tuple with the PublicId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicId

`func (o *PublicStoreResponse) SetPublicId(v string)`

SetPublicId sets PublicId field to given value.


### GetRegistrationRequested

`func (o *PublicStoreResponse) GetRegistrationRequested() bool`

GetRegistrationRequested returns the RegistrationRequested field if non-nil, zero value otherwise.

### GetRegistrationRequestedOk

`func (o *PublicStoreResponse) GetRegistrationRequestedOk() (*bool, bool)`

GetRegistrationRequestedOk returns a tuple with the RegistrationRequested field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRegistrationRequested

`func (o *PublicStoreResponse) SetRegistrationRequested(v bool)`

SetRegistrationRequested sets RegistrationRequested field to given value.


### GetStoreId

`func (o *PublicStoreResponse) GetStoreId() string`

GetStoreId returns the StoreId field if non-nil, zero value otherwise.

### GetStoreIdOk

`func (o *PublicStoreResponse) GetStoreIdOk() (*string, bool)`

GetStoreIdOk returns a tuple with the StoreId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStoreId

`func (o *PublicStoreResponse) SetStoreId(v string)`

SetStoreId sets StoreId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
