# Store

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BrandingOverrides** | Pointer to **map[string]interface{}** |  | [optional]
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**Currency** | **string** | ISO 4217 currency code for prices/statements. |
**DeletedAt** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**OrgId** | **string** |  |
**Timezone** | **string** | IANA timezone used to derive monthly billing cycles. |
**UpdatedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewStore

`func NewStore(currency string, id string, name LocalizedString, orgId string, timezone string, ) *Store`

NewStore instantiates a new Store object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewStoreWithDefaults

`func NewStoreWithDefaults() *Store`

NewStoreWithDefaults instantiates a new Store object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBrandingOverrides

`func (o *Store) GetBrandingOverrides() map[string]interface{}`

GetBrandingOverrides returns the BrandingOverrides field if non-nil, zero value otherwise.

### GetBrandingOverridesOk

`func (o *Store) GetBrandingOverridesOk() (*map[string]interface{}, bool)`

GetBrandingOverridesOk returns a tuple with the BrandingOverrides field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBrandingOverrides

`func (o *Store) SetBrandingOverrides(v map[string]interface{})`

SetBrandingOverrides sets BrandingOverrides field to given value.

### HasBrandingOverrides

`func (o *Store) HasBrandingOverrides() bool`

HasBrandingOverrides returns a boolean if a field has been set.

### SetBrandingOverridesNil

`func (o *Store) SetBrandingOverridesNil(b bool)`

 SetBrandingOverridesNil sets the value for BrandingOverrides to be an explicit nil

### UnsetBrandingOverrides
`func (o *Store) UnsetBrandingOverrides()`

UnsetBrandingOverrides ensures that no value is present for BrandingOverrides, not even an explicit nil
### GetCreatedAt

`func (o *Store) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *Store) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *Store) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *Store) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *Store) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *Store) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetCurrency

`func (o *Store) GetCurrency() string`

GetCurrency returns the Currency field if non-nil, zero value otherwise.

### GetCurrencyOk

`func (o *Store) GetCurrencyOk() (*string, bool)`

GetCurrencyOk returns a tuple with the Currency field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrency

`func (o *Store) SetCurrency(v string)`

SetCurrency sets Currency field to given value.


### GetDeletedAt

`func (o *Store) GetDeletedAt() string`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *Store) GetDeletedAtOk() (*string, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *Store) SetDeletedAt(v string)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *Store) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### SetDeletedAtNil

`func (o *Store) SetDeletedAtNil(b bool)`

 SetDeletedAtNil sets the value for DeletedAt to be an explicit nil

### UnsetDeletedAt
`func (o *Store) UnsetDeletedAt()`

UnsetDeletedAt ensures that no value is present for DeletedAt, not even an explicit nil
### GetId

`func (o *Store) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *Store) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *Store) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *Store) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *Store) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *Store) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *Store) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *Store) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *Store) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetTimezone

`func (o *Store) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *Store) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *Store) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.


### GetUpdatedAt

`func (o *Store) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *Store) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *Store) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *Store) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *Store) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *Store) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
