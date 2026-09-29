# FlaggedResource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**FlaggedAt** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**MarkedAt** | Pointer to **NullableString** |  | [optional]
**Resource** | [**CapacityResource**](CapacityResource.md) |  |
**State** | [**ResourceState**](ResourceState.md) |  |

## Methods

### NewFlaggedResource

`func NewFlaggedResource(id string, resource CapacityResource, state ResourceState, ) *FlaggedResource`

NewFlaggedResource instantiates a new FlaggedResource object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewFlaggedResourceWithDefaults

`func NewFlaggedResourceWithDefaults() *FlaggedResource`

NewFlaggedResourceWithDefaults instantiates a new FlaggedResource object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFlaggedAt

`func (o *FlaggedResource) GetFlaggedAt() string`

GetFlaggedAt returns the FlaggedAt field if non-nil, zero value otherwise.

### GetFlaggedAtOk

`func (o *FlaggedResource) GetFlaggedAtOk() (*string, bool)`

GetFlaggedAtOk returns a tuple with the FlaggedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFlaggedAt

`func (o *FlaggedResource) SetFlaggedAt(v string)`

SetFlaggedAt sets FlaggedAt field to given value.

### HasFlaggedAt

`func (o *FlaggedResource) HasFlaggedAt() bool`

HasFlaggedAt returns a boolean if a field has been set.

### SetFlaggedAtNil

`func (o *FlaggedResource) SetFlaggedAtNil(b bool)`

 SetFlaggedAtNil sets the value for FlaggedAt to be an explicit nil

### UnsetFlaggedAt
`func (o *FlaggedResource) UnsetFlaggedAt()`

UnsetFlaggedAt ensures that no value is present for FlaggedAt, not even an explicit nil
### GetId

`func (o *FlaggedResource) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *FlaggedResource) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *FlaggedResource) SetId(v string)`

SetId sets Id field to given value.


### GetMarkedAt

`func (o *FlaggedResource) GetMarkedAt() string`

GetMarkedAt returns the MarkedAt field if non-nil, zero value otherwise.

### GetMarkedAtOk

`func (o *FlaggedResource) GetMarkedAtOk() (*string, bool)`

GetMarkedAtOk returns a tuple with the MarkedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMarkedAt

`func (o *FlaggedResource) SetMarkedAt(v string)`

SetMarkedAt sets MarkedAt field to given value.

### HasMarkedAt

`func (o *FlaggedResource) HasMarkedAt() bool`

HasMarkedAt returns a boolean if a field has been set.

### SetMarkedAtNil

`func (o *FlaggedResource) SetMarkedAtNil(b bool)`

 SetMarkedAtNil sets the value for MarkedAt to be an explicit nil

### UnsetMarkedAt
`func (o *FlaggedResource) UnsetMarkedAt()`

UnsetMarkedAt ensures that no value is present for MarkedAt, not even an explicit nil
### GetResource

`func (o *FlaggedResource) GetResource() CapacityResource`

GetResource returns the Resource field if non-nil, zero value otherwise.

### GetResourceOk

`func (o *FlaggedResource) GetResourceOk() (*CapacityResource, bool)`

GetResourceOk returns a tuple with the Resource field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResource

`func (o *FlaggedResource) SetResource(v CapacityResource)`

SetResource sets Resource field to given value.


### GetState

`func (o *FlaggedResource) GetState() ResourceState`

GetState returns the State field if non-nil, zero value otherwise.

### GetStateOk

`func (o *FlaggedResource) GetStateOk() (*ResourceState, bool)`

GetStateOk returns a tuple with the State field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetState

`func (o *FlaggedResource) SetState(v ResourceState)`

SetState sets State field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
