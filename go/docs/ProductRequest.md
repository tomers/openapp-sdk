# ProductRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExternalUrl** | Pointer to **NullableString** |  | [optional]
**ImageMediaAssetId** | Pointer to **NullableString** |  | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) |  |

## Methods

### NewProductRequest

`func NewProductRequest(name LocalizedString, ) *ProductRequest`

NewProductRequest instantiates a new ProductRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewProductRequestWithDefaults

`func NewProductRequestWithDefaults() *ProductRequest`

NewProductRequestWithDefaults instantiates a new ProductRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExternalUrl

`func (o *ProductRequest) GetExternalUrl() string`

GetExternalUrl returns the ExternalUrl field if non-nil, zero value otherwise.

### GetExternalUrlOk

`func (o *ProductRequest) GetExternalUrlOk() (*string, bool)`

GetExternalUrlOk returns a tuple with the ExternalUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalUrl

`func (o *ProductRequest) SetExternalUrl(v string)`

SetExternalUrl sets ExternalUrl field to given value.

### HasExternalUrl

`func (o *ProductRequest) HasExternalUrl() bool`

HasExternalUrl returns a boolean if a field has been set.

### SetExternalUrlNil

`func (o *ProductRequest) SetExternalUrlNil(b bool)`

 SetExternalUrlNil sets the value for ExternalUrl to be an explicit nil

### UnsetExternalUrl
`func (o *ProductRequest) UnsetExternalUrl()`

UnsetExternalUrl ensures that no value is present for ExternalUrl, not even an explicit nil
### GetImageMediaAssetId

`func (o *ProductRequest) GetImageMediaAssetId() string`

GetImageMediaAssetId returns the ImageMediaAssetId field if non-nil, zero value otherwise.

### GetImageMediaAssetIdOk

`func (o *ProductRequest) GetImageMediaAssetIdOk() (*string, bool)`

GetImageMediaAssetIdOk returns a tuple with the ImageMediaAssetId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImageMediaAssetId

`func (o *ProductRequest) SetImageMediaAssetId(v string)`

SetImageMediaAssetId sets ImageMediaAssetId field to given value.

### HasImageMediaAssetId

`func (o *ProductRequest) HasImageMediaAssetId() bool`

HasImageMediaAssetId returns a boolean if a field has been set.

### SetImageMediaAssetIdNil

`func (o *ProductRequest) SetImageMediaAssetIdNil(b bool)`

 SetImageMediaAssetIdNil sets the value for ImageMediaAssetId to be an explicit nil

### UnsetImageMediaAssetId
`func (o *ProductRequest) UnsetImageMediaAssetId()`

UnsetImageMediaAssetId ensures that no value is present for ImageMediaAssetId, not even an explicit nil
### GetName

`func (o *ProductRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ProductRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ProductRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
