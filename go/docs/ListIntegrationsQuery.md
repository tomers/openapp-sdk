# ListIntegrationsQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IncludeDeleted** | **bool** |  |
**IncludeMetadata** | **bool** |  |
**OnlyDeleted** | **bool** |  |
**Limit** | Pointer to **int32** | Number of items per page. Default from config, max 200. | [optional]
**Offset** | Pointer to **int32** | Number of items to skip. Default 0. | [optional]
**ProviderType** | Pointer to **NullableString** | Optional filter: only integrations with this provider_type (e.g. virtual_access). | [optional]
**Q** | Pointer to **NullableString** | Optional case-insensitive substring match on integration &#x60;name&#x60; (localized JSON), same as devices list. | [optional]

## Methods

### NewListIntegrationsQuery

`func NewListIntegrationsQuery(includeDeleted bool, includeMetadata bool, onlyDeleted bool, ) *ListIntegrationsQuery`

NewListIntegrationsQuery instantiates a new ListIntegrationsQuery object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListIntegrationsQueryWithDefaults

`func NewListIntegrationsQueryWithDefaults() *ListIntegrationsQuery`

NewListIntegrationsQueryWithDefaults instantiates a new ListIntegrationsQuery object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIncludeDeleted

`func (o *ListIntegrationsQuery) GetIncludeDeleted() bool`

GetIncludeDeleted returns the IncludeDeleted field if non-nil, zero value otherwise.

### GetIncludeDeletedOk

`func (o *ListIntegrationsQuery) GetIncludeDeletedOk() (*bool, bool)`

GetIncludeDeletedOk returns a tuple with the IncludeDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeDeleted

`func (o *ListIntegrationsQuery) SetIncludeDeleted(v bool)`

SetIncludeDeleted sets IncludeDeleted field to given value.


### GetIncludeMetadata

`func (o *ListIntegrationsQuery) GetIncludeMetadata() bool`

GetIncludeMetadata returns the IncludeMetadata field if non-nil, zero value otherwise.

### GetIncludeMetadataOk

`func (o *ListIntegrationsQuery) GetIncludeMetadataOk() (*bool, bool)`

GetIncludeMetadataOk returns a tuple with the IncludeMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeMetadata

`func (o *ListIntegrationsQuery) SetIncludeMetadata(v bool)`

SetIncludeMetadata sets IncludeMetadata field to given value.


### GetOnlyDeleted

`func (o *ListIntegrationsQuery) GetOnlyDeleted() bool`

GetOnlyDeleted returns the OnlyDeleted field if non-nil, zero value otherwise.

### GetOnlyDeletedOk

`func (o *ListIntegrationsQuery) GetOnlyDeletedOk() (*bool, bool)`

GetOnlyDeletedOk returns a tuple with the OnlyDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOnlyDeleted

`func (o *ListIntegrationsQuery) SetOnlyDeleted(v bool)`

SetOnlyDeleted sets OnlyDeleted field to given value.


### GetLimit

`func (o *ListIntegrationsQuery) GetLimit() int32`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *ListIntegrationsQuery) GetLimitOk() (*int32, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *ListIntegrationsQuery) SetLimit(v int32)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *ListIntegrationsQuery) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### GetOffset

`func (o *ListIntegrationsQuery) GetOffset() int32`

GetOffset returns the Offset field if non-nil, zero value otherwise.

### GetOffsetOk

`func (o *ListIntegrationsQuery) GetOffsetOk() (*int32, bool)`

GetOffsetOk returns a tuple with the Offset field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffset

`func (o *ListIntegrationsQuery) SetOffset(v int32)`

SetOffset sets Offset field to given value.

### HasOffset

`func (o *ListIntegrationsQuery) HasOffset() bool`

HasOffset returns a boolean if a field has been set.

### GetProviderType

`func (o *ListIntegrationsQuery) GetProviderType() string`

GetProviderType returns the ProviderType field if non-nil, zero value otherwise.

### GetProviderTypeOk

`func (o *ListIntegrationsQuery) GetProviderTypeOk() (*string, bool)`

GetProviderTypeOk returns a tuple with the ProviderType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderType

`func (o *ListIntegrationsQuery) SetProviderType(v string)`

SetProviderType sets ProviderType field to given value.

### HasProviderType

`func (o *ListIntegrationsQuery) HasProviderType() bool`

HasProviderType returns a boolean if a field has been set.

### SetProviderTypeNil

`func (o *ListIntegrationsQuery) SetProviderTypeNil(b bool)`

 SetProviderTypeNil sets the value for ProviderType to be an explicit nil

### UnsetProviderType
`func (o *ListIntegrationsQuery) UnsetProviderType()`

UnsetProviderType ensures that no value is present for ProviderType, not even an explicit nil
### GetQ

`func (o *ListIntegrationsQuery) GetQ() string`

GetQ returns the Q field if non-nil, zero value otherwise.

### GetQOk

`func (o *ListIntegrationsQuery) GetQOk() (*string, bool)`

GetQOk returns a tuple with the Q field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQ

`func (o *ListIntegrationsQuery) SetQ(v string)`

SetQ sets Q field to given value.

### HasQ

`func (o *ListIntegrationsQuery) HasQ() bool`

HasQ returns a boolean if a field has been set.

### SetQNil

`func (o *ListIntegrationsQuery) SetQNil(b bool)`

 SetQNil sets the value for Q to be an explicit nil

### UnsetQ
`func (o *ListIntegrationsQuery) UnsetQ()`

UnsetQ ensures that no value is present for Q, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
