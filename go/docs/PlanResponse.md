# PlanResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Description** | Pointer to **interface{}** |  | [optional]
**IsDefaultForPersonal** | **bool** |  |
**IsPublished** | **bool** |  |
**Name** | **interface{}** |  |
**Quotas** | [**[]PlanQuotaResponse**](PlanQuotaResponse.md) |  |
**Slug** | **string** |  |
**SortOrder** | **int32** |  |

## Methods

### NewPlanResponse

`func NewPlanResponse(isDefaultForPersonal bool, isPublished bool, name interface{}, quotas []PlanQuotaResponse, slug string, sortOrder int32, ) *PlanResponse`

NewPlanResponse instantiates a new PlanResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPlanResponseWithDefaults

`func NewPlanResponseWithDefaults() *PlanResponse`

NewPlanResponseWithDefaults instantiates a new PlanResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDescription

`func (o *PlanResponse) GetDescription() interface{}`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *PlanResponse) GetDescriptionOk() (*interface{}, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *PlanResponse) SetDescription(v interface{})`

SetDescription sets Description field to given value.

### HasDescription

`func (o *PlanResponse) HasDescription() bool`

HasDescription returns a boolean if a field has been set.

### SetDescriptionNil

`func (o *PlanResponse) SetDescriptionNil(b bool)`

 SetDescriptionNil sets the value for Description to be an explicit nil

### UnsetDescription
`func (o *PlanResponse) UnsetDescription()`

UnsetDescription ensures that no value is present for Description, not even an explicit nil
### GetIsDefaultForPersonal

`func (o *PlanResponse) GetIsDefaultForPersonal() bool`

GetIsDefaultForPersonal returns the IsDefaultForPersonal field if non-nil, zero value otherwise.

### GetIsDefaultForPersonalOk

`func (o *PlanResponse) GetIsDefaultForPersonalOk() (*bool, bool)`

GetIsDefaultForPersonalOk returns a tuple with the IsDefaultForPersonal field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDefaultForPersonal

`func (o *PlanResponse) SetIsDefaultForPersonal(v bool)`

SetIsDefaultForPersonal sets IsDefaultForPersonal field to given value.


### GetIsPublished

`func (o *PlanResponse) GetIsPublished() bool`

GetIsPublished returns the IsPublished field if non-nil, zero value otherwise.

### GetIsPublishedOk

`func (o *PlanResponse) GetIsPublishedOk() (*bool, bool)`

GetIsPublishedOk returns a tuple with the IsPublished field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsPublished

`func (o *PlanResponse) SetIsPublished(v bool)`

SetIsPublished sets IsPublished field to given value.


### GetName

`func (o *PlanResponse) GetName() interface{}`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PlanResponse) GetNameOk() (*interface{}, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PlanResponse) SetName(v interface{})`

SetName sets Name field to given value.


### SetNameNil

`func (o *PlanResponse) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *PlanResponse) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetQuotas

`func (o *PlanResponse) GetQuotas() []PlanQuotaResponse`

GetQuotas returns the Quotas field if non-nil, zero value otherwise.

### GetQuotasOk

`func (o *PlanResponse) GetQuotasOk() (*[]PlanQuotaResponse, bool)`

GetQuotasOk returns a tuple with the Quotas field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuotas

`func (o *PlanResponse) SetQuotas(v []PlanQuotaResponse)`

SetQuotas sets Quotas field to given value.


### GetSlug

`func (o *PlanResponse) GetSlug() string`

GetSlug returns the Slug field if non-nil, zero value otherwise.

### GetSlugOk

`func (o *PlanResponse) GetSlugOk() (*string, bool)`

GetSlugOk returns a tuple with the Slug field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSlug

`func (o *PlanResponse) SetSlug(v string)`

SetSlug sets Slug field to given value.


### GetSortOrder

`func (o *PlanResponse) GetSortOrder() int32`

GetSortOrder returns the SortOrder field if non-nil, zero value otherwise.

### GetSortOrderOk

`func (o *PlanResponse) GetSortOrderOk() (*int32, bool)`

GetSortOrderOk returns a tuple with the SortOrder field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSortOrder

`func (o *PlanResponse) SetSortOrder(v int32)`

SetSortOrder sets SortOrder field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
