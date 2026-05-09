# PatchEntityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Metadata** | **map[string]interface{}** | Keys to merge into persisted metadata. Omitted keys are left unchanged. JSON &#x60;null&#x60; removes a key (e.g. clear &#x60;floor&#x60; when setting &#x60;floor_number&#x60;). |

## Methods

### NewPatchEntityRequest

`func NewPatchEntityRequest(metadata map[string]interface{}, ) *PatchEntityRequest`

NewPatchEntityRequest instantiates a new PatchEntityRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPatchEntityRequestWithDefaults

`func NewPatchEntityRequestWithDefaults() *PatchEntityRequest`

NewPatchEntityRequestWithDefaults instantiates a new PatchEntityRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMetadata

`func (o *PatchEntityRequest) GetMetadata() map[string]interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *PatchEntityRequest) GetMetadataOk() (*map[string]interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *PatchEntityRequest) SetMetadata(v map[string]interface{})`

SetMetadata sets Metadata field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
