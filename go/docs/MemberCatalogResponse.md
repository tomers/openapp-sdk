# MemberCatalogResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Items** | [**[]MemberCatalogItem**](MemberCatalogItem.md) |  |
**Store** | [**Store**](Store.md) |  |

## Methods

### NewMemberCatalogResponse

`func NewMemberCatalogResponse(items []MemberCatalogItem, store Store, ) *MemberCatalogResponse`

NewMemberCatalogResponse instantiates a new MemberCatalogResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMemberCatalogResponseWithDefaults

`func NewMemberCatalogResponseWithDefaults() *MemberCatalogResponse`

NewMemberCatalogResponseWithDefaults instantiates a new MemberCatalogResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetItems

`func (o *MemberCatalogResponse) GetItems() []MemberCatalogItem`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *MemberCatalogResponse) GetItemsOk() (*[]MemberCatalogItem, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *MemberCatalogResponse) SetItems(v []MemberCatalogItem)`

SetItems sets Items field to given value.


### GetStore

`func (o *MemberCatalogResponse) GetStore() Store`

GetStore returns the Store field if non-nil, zero value otherwise.

### GetStoreOk

`func (o *MemberCatalogResponse) GetStoreOk() (*Store, bool)`

GetStoreOk returns a tuple with the Store field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStore

`func (o *MemberCatalogResponse) SetStore(v Store)`

SetStore sets Store field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
