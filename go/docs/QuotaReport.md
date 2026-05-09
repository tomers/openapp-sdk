# QuotaReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Items** | [**[]QuotaUsage**](QuotaUsage.md) |  |
**OrgId** | **string** |  |
**TierSlug** | Pointer to **NullableString** |  | [optional]

## Methods

### NewQuotaReport

`func NewQuotaReport(items []QuotaUsage, orgId string, ) *QuotaReport`

NewQuotaReport instantiates a new QuotaReport object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQuotaReportWithDefaults

`func NewQuotaReportWithDefaults() *QuotaReport`

NewQuotaReportWithDefaults instantiates a new QuotaReport object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetItems

`func (o *QuotaReport) GetItems() []QuotaUsage`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *QuotaReport) GetItemsOk() (*[]QuotaUsage, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *QuotaReport) SetItems(v []QuotaUsage)`

SetItems sets Items field to given value.


### GetOrgId

`func (o *QuotaReport) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *QuotaReport) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *QuotaReport) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetTierSlug

`func (o *QuotaReport) GetTierSlug() string`

GetTierSlug returns the TierSlug field if non-nil, zero value otherwise.

### GetTierSlugOk

`func (o *QuotaReport) GetTierSlugOk() (*string, bool)`

GetTierSlugOk returns a tuple with the TierSlug field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTierSlug

`func (o *QuotaReport) SetTierSlug(v string)`

SetTierSlug sets TierSlug field to given value.

### HasTierSlug

`func (o *QuotaReport) HasTierSlug() bool`

HasTierSlug returns a boolean if a field has been set.

### SetTierSlugNil

`func (o *QuotaReport) SetTierSlugNil(b bool)`

 SetTierSlugNil sets the value for TierSlug to be an explicit nil

### UnsetTierSlug
`func (o *QuotaReport) UnsetTierSlug()`

UnsetTierSlug ensures that no value is present for TierSlug, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
