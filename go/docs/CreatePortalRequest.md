# CreatePortalRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BrandingOverrides** | Pointer to **map[string]interface{}** |  | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) |  |

## Methods

### NewCreatePortalRequest

`func NewCreatePortalRequest(name LocalizedString, ) *CreatePortalRequest`

NewCreatePortalRequest instantiates a new CreatePortalRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreatePortalRequestWithDefaults

`func NewCreatePortalRequestWithDefaults() *CreatePortalRequest`

NewCreatePortalRequestWithDefaults instantiates a new CreatePortalRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBrandingOverrides

`func (o *CreatePortalRequest) GetBrandingOverrides() map[string]interface{}`

GetBrandingOverrides returns the BrandingOverrides field if non-nil, zero value otherwise.

### GetBrandingOverridesOk

`func (o *CreatePortalRequest) GetBrandingOverridesOk() (*map[string]interface{}, bool)`

GetBrandingOverridesOk returns a tuple with the BrandingOverrides field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBrandingOverrides

`func (o *CreatePortalRequest) SetBrandingOverrides(v map[string]interface{})`

SetBrandingOverrides sets BrandingOverrides field to given value.

### HasBrandingOverrides

`func (o *CreatePortalRequest) HasBrandingOverrides() bool`

HasBrandingOverrides returns a boolean if a field has been set.

### SetBrandingOverridesNil

`func (o *CreatePortalRequest) SetBrandingOverridesNil(b bool)`

 SetBrandingOverridesNil sets the value for BrandingOverrides to be an explicit nil

### UnsetBrandingOverrides
`func (o *CreatePortalRequest) UnsetBrandingOverrides()`

UnsetBrandingOverrides ensures that no value is present for BrandingOverrides, not even an explicit nil
### GetName

`func (o *CreatePortalRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreatePortalRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreatePortalRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
