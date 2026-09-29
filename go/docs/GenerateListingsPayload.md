# GenerateListingsPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ListingKind** | Pointer to **NullableString** |  | [optional]
**NamingPattern** | Pointer to **NullableString** |  | [optional]
**SourceGroupIds** | **[]string** |  |

## Methods

### NewGenerateListingsPayload

`func NewGenerateListingsPayload(sourceGroupIds []string, ) *GenerateListingsPayload`

NewGenerateListingsPayload instantiates a new GenerateListingsPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGenerateListingsPayloadWithDefaults

`func NewGenerateListingsPayloadWithDefaults() *GenerateListingsPayload`

NewGenerateListingsPayloadWithDefaults instantiates a new GenerateListingsPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetListingKind

`func (o *GenerateListingsPayload) GetListingKind() string`

GetListingKind returns the ListingKind field if non-nil, zero value otherwise.

### GetListingKindOk

`func (o *GenerateListingsPayload) GetListingKindOk() (*string, bool)`

GetListingKindOk returns a tuple with the ListingKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListingKind

`func (o *GenerateListingsPayload) SetListingKind(v string)`

SetListingKind sets ListingKind field to given value.

### HasListingKind

`func (o *GenerateListingsPayload) HasListingKind() bool`

HasListingKind returns a boolean if a field has been set.

### SetListingKindNil

`func (o *GenerateListingsPayload) SetListingKindNil(b bool)`

 SetListingKindNil sets the value for ListingKind to be an explicit nil

### UnsetListingKind
`func (o *GenerateListingsPayload) UnsetListingKind()`

UnsetListingKind ensures that no value is present for ListingKind, not even an explicit nil
### GetNamingPattern

`func (o *GenerateListingsPayload) GetNamingPattern() string`

GetNamingPattern returns the NamingPattern field if non-nil, zero value otherwise.

### GetNamingPatternOk

`func (o *GenerateListingsPayload) GetNamingPatternOk() (*string, bool)`

GetNamingPatternOk returns a tuple with the NamingPattern field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNamingPattern

`func (o *GenerateListingsPayload) SetNamingPattern(v string)`

SetNamingPattern sets NamingPattern field to given value.

### HasNamingPattern

`func (o *GenerateListingsPayload) HasNamingPattern() bool`

HasNamingPattern returns a boolean if a field has been set.

### SetNamingPatternNil

`func (o *GenerateListingsPayload) SetNamingPatternNil(b bool)`

 SetNamingPatternNil sets the value for NamingPattern to be an explicit nil

### UnsetNamingPattern
`func (o *GenerateListingsPayload) UnsetNamingPattern()`

UnsetNamingPattern ensures that no value is present for NamingPattern, not even an explicit nil
### GetSourceGroupIds

`func (o *GenerateListingsPayload) GetSourceGroupIds() []string`

GetSourceGroupIds returns the SourceGroupIds field if non-nil, zero value otherwise.

### GetSourceGroupIdsOk

`func (o *GenerateListingsPayload) GetSourceGroupIdsOk() (*[]string, bool)`

GetSourceGroupIdsOk returns a tuple with the SourceGroupIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceGroupIds

`func (o *GenerateListingsPayload) SetSourceGroupIds(v []string)`

SetSourceGroupIds sets SourceGroupIds field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
