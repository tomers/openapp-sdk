# CreateDeviceRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeviceMetadata** | Pointer to **interface{}** |  | [optional]
**ExternalId** | Pointer to **NullableString** |  | [optional]
**IntegrationId** | **string** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**OrgId** | **string** |  |

## Methods

### NewCreateDeviceRequest

`func NewCreateDeviceRequest(integrationId string, name LocalizedString, orgId string, ) *CreateDeviceRequest`

NewCreateDeviceRequest instantiates a new CreateDeviceRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateDeviceRequestWithDefaults

`func NewCreateDeviceRequestWithDefaults() *CreateDeviceRequest`

NewCreateDeviceRequestWithDefaults instantiates a new CreateDeviceRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeviceMetadata

`func (o *CreateDeviceRequest) GetDeviceMetadata() interface{}`

GetDeviceMetadata returns the DeviceMetadata field if non-nil, zero value otherwise.

### GetDeviceMetadataOk

`func (o *CreateDeviceRequest) GetDeviceMetadataOk() (*interface{}, bool)`

GetDeviceMetadataOk returns a tuple with the DeviceMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceMetadata

`func (o *CreateDeviceRequest) SetDeviceMetadata(v interface{})`

SetDeviceMetadata sets DeviceMetadata field to given value.

### HasDeviceMetadata

`func (o *CreateDeviceRequest) HasDeviceMetadata() bool`

HasDeviceMetadata returns a boolean if a field has been set.

### SetDeviceMetadataNil

`func (o *CreateDeviceRequest) SetDeviceMetadataNil(b bool)`

 SetDeviceMetadataNil sets the value for DeviceMetadata to be an explicit nil

### UnsetDeviceMetadata
`func (o *CreateDeviceRequest) UnsetDeviceMetadata()`

UnsetDeviceMetadata ensures that no value is present for DeviceMetadata, not even an explicit nil
### GetExternalId

`func (o *CreateDeviceRequest) GetExternalId() string`

GetExternalId returns the ExternalId field if non-nil, zero value otherwise.

### GetExternalIdOk

`func (o *CreateDeviceRequest) GetExternalIdOk() (*string, bool)`

GetExternalIdOk returns a tuple with the ExternalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExternalId

`func (o *CreateDeviceRequest) SetExternalId(v string)`

SetExternalId sets ExternalId field to given value.

### HasExternalId

`func (o *CreateDeviceRequest) HasExternalId() bool`

HasExternalId returns a boolean if a field has been set.

### SetExternalIdNil

`func (o *CreateDeviceRequest) SetExternalIdNil(b bool)`

 SetExternalIdNil sets the value for ExternalId to be an explicit nil

### UnsetExternalId
`func (o *CreateDeviceRequest) UnsetExternalId()`

UnsetExternalId ensures that no value is present for ExternalId, not even an explicit nil
### GetIntegrationId

`func (o *CreateDeviceRequest) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *CreateDeviceRequest) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *CreateDeviceRequest) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### GetName

`func (o *CreateDeviceRequest) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateDeviceRequest) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateDeviceRequest) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *CreateDeviceRequest) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *CreateDeviceRequest) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *CreateDeviceRequest) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
