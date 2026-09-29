# SetResourceKeepRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Keep** | **bool** | &#x60;true&#x60; pins the resource (kept within cap); &#x60;false&#x60; lets it be flagged as excess. |
**Resource** | [**CapacityResource**](CapacityResource.md) | Which capacity resource the row belongs to. |
**ResourceId** | **string** | The resource id to pin or unpin. |

## Methods

### NewSetResourceKeepRequest

`func NewSetResourceKeepRequest(keep bool, resource CapacityResource, resourceId string, ) *SetResourceKeepRequest`

NewSetResourceKeepRequest instantiates a new SetResourceKeepRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSetResourceKeepRequestWithDefaults

`func NewSetResourceKeepRequestWithDefaults() *SetResourceKeepRequest`

NewSetResourceKeepRequestWithDefaults instantiates a new SetResourceKeepRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetKeep

`func (o *SetResourceKeepRequest) GetKeep() bool`

GetKeep returns the Keep field if non-nil, zero value otherwise.

### GetKeepOk

`func (o *SetResourceKeepRequest) GetKeepOk() (*bool, bool)`

GetKeepOk returns a tuple with the Keep field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKeep

`func (o *SetResourceKeepRequest) SetKeep(v bool)`

SetKeep sets Keep field to given value.


### GetResource

`func (o *SetResourceKeepRequest) GetResource() CapacityResource`

GetResource returns the Resource field if non-nil, zero value otherwise.

### GetResourceOk

`func (o *SetResourceKeepRequest) GetResourceOk() (*CapacityResource, bool)`

GetResourceOk returns a tuple with the Resource field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResource

`func (o *SetResourceKeepRequest) SetResource(v CapacityResource)`

SetResource sets Resource field to given value.


### GetResourceId

`func (o *SetResourceKeepRequest) GetResourceId() string`

GetResourceId returns the ResourceId field if non-nil, zero value otherwise.

### GetResourceIdOk

`func (o *SetResourceKeepRequest) GetResourceIdOk() (*string, bool)`

GetResourceIdOk returns a tuple with the ResourceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResourceId

`func (o *SetResourceKeepRequest) SetResourceId(v string)`

SetResourceId sets ResourceId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
