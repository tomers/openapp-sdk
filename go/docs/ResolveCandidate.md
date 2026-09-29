# ResolveCandidate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**EntityType** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**Label** | **string** |  |
**ResourceType** | **string** |  |
**Score** | **float64** |  |
**Why** | Pointer to **NullableString** |  | [optional]

## Methods

### NewResolveCandidate

`func NewResolveCandidate(id string, label string, resourceType string, score float64, ) *ResolveCandidate`

NewResolveCandidate instantiates a new ResolveCandidate object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResolveCandidateWithDefaults

`func NewResolveCandidateWithDefaults() *ResolveCandidate`

NewResolveCandidateWithDefaults instantiates a new ResolveCandidate object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEntityType

`func (o *ResolveCandidate) GetEntityType() string`

GetEntityType returns the EntityType field if non-nil, zero value otherwise.

### GetEntityTypeOk

`func (o *ResolveCandidate) GetEntityTypeOk() (*string, bool)`

GetEntityTypeOk returns a tuple with the EntityType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntityType

`func (o *ResolveCandidate) SetEntityType(v string)`

SetEntityType sets EntityType field to given value.

### HasEntityType

`func (o *ResolveCandidate) HasEntityType() bool`

HasEntityType returns a boolean if a field has been set.

### SetEntityTypeNil

`func (o *ResolveCandidate) SetEntityTypeNil(b bool)`

 SetEntityTypeNil sets the value for EntityType to be an explicit nil

### UnsetEntityType
`func (o *ResolveCandidate) UnsetEntityType()`

UnsetEntityType ensures that no value is present for EntityType, not even an explicit nil
### GetId

`func (o *ResolveCandidate) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ResolveCandidate) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ResolveCandidate) SetId(v string)`

SetId sets Id field to given value.


### GetLabel

`func (o *ResolveCandidate) GetLabel() string`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *ResolveCandidate) GetLabelOk() (*string, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *ResolveCandidate) SetLabel(v string)`

SetLabel sets Label field to given value.


### GetResourceType

`func (o *ResolveCandidate) GetResourceType() string`

GetResourceType returns the ResourceType field if non-nil, zero value otherwise.

### GetResourceTypeOk

`func (o *ResolveCandidate) GetResourceTypeOk() (*string, bool)`

GetResourceTypeOk returns a tuple with the ResourceType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceType

`func (o *ResolveCandidate) SetResourceType(v string)`

SetResourceType sets ResourceType field to given value.


### GetScore

`func (o *ResolveCandidate) GetScore() float64`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *ResolveCandidate) GetScoreOk() (*float64, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *ResolveCandidate) SetScore(v float64)`

SetScore sets Score field to given value.


### GetWhy

`func (o *ResolveCandidate) GetWhy() string`

GetWhy returns the Why field if non-nil, zero value otherwise.

### GetWhyOk

`func (o *ResolveCandidate) GetWhyOk() (*string, bool)`

GetWhyOk returns a tuple with the Why field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWhy

`func (o *ResolveCandidate) SetWhy(v string)`

SetWhy sets Why field to given value.

### HasWhy

`func (o *ResolveCandidate) HasWhy() bool`

HasWhy returns a boolean if a field has been set.

### SetWhyNil

`func (o *ResolveCandidate) SetWhyNil(b bool)`

 SetWhyNil sets the value for Why to be an explicit nil

### UnsetWhy
`func (o *ResolveCandidate) UnsetWhy()`

UnsetWhy ensures that no value is present for Why, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
