# LanAgentBootstrapTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ApiBaseUrl** | **string** | Browser-visible API base (e.g. &#x60;https://app.example.com/api/v1&#x60;). |
**DeviceId** | Pointer to **NullableString** |  | [optional]
**IntegrationId** | Pointer to **NullableString** | Required for one-shot Waveshare tasks; omit for &#x60;serve&#x60;. | [optional]
**TaskId** | **string** |  |

## Methods

### NewLanAgentBootstrapTokenRequest

`func NewLanAgentBootstrapTokenRequest(apiBaseUrl string, taskId string, ) *LanAgentBootstrapTokenRequest`

NewLanAgentBootstrapTokenRequest instantiates a new LanAgentBootstrapTokenRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewLanAgentBootstrapTokenRequestWithDefaults

`func NewLanAgentBootstrapTokenRequestWithDefaults() *LanAgentBootstrapTokenRequest`

NewLanAgentBootstrapTokenRequestWithDefaults instantiates a new LanAgentBootstrapTokenRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetApiBaseUrl

`func (o *LanAgentBootstrapTokenRequest) GetApiBaseUrl() string`

GetApiBaseUrl returns the ApiBaseUrl field if non-nil, zero value otherwise.

### GetApiBaseUrlOk

`func (o *LanAgentBootstrapTokenRequest) GetApiBaseUrlOk() (*string, bool)`

GetApiBaseUrlOk returns a tuple with the ApiBaseUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApiBaseUrl

`func (o *LanAgentBootstrapTokenRequest) SetApiBaseUrl(v string)`

SetApiBaseUrl sets ApiBaseUrl field to given value.


### GetDeviceId

`func (o *LanAgentBootstrapTokenRequest) GetDeviceId() string`

GetDeviceId returns the DeviceId field if non-nil, zero value otherwise.

### GetDeviceIdOk

`func (o *LanAgentBootstrapTokenRequest) GetDeviceIdOk() (*string, bool)`

GetDeviceIdOk returns a tuple with the DeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceId

`func (o *LanAgentBootstrapTokenRequest) SetDeviceId(v string)`

SetDeviceId sets DeviceId field to given value.

### HasDeviceId

`func (o *LanAgentBootstrapTokenRequest) HasDeviceId() bool`

HasDeviceId returns a boolean if a field has been set.

### SetDeviceIdNil

`func (o *LanAgentBootstrapTokenRequest) SetDeviceIdNil(b bool)`

 SetDeviceIdNil sets the value for DeviceId to be an explicit nil

### UnsetDeviceId
`func (o *LanAgentBootstrapTokenRequest) UnsetDeviceId()`

UnsetDeviceId ensures that no value is present for DeviceId, not even an explicit nil
### GetIntegrationId

`func (o *LanAgentBootstrapTokenRequest) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *LanAgentBootstrapTokenRequest) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *LanAgentBootstrapTokenRequest) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.

### HasIntegrationId

`func (o *LanAgentBootstrapTokenRequest) HasIntegrationId() bool`

HasIntegrationId returns a boolean if a field has been set.

### SetIntegrationIdNil

`func (o *LanAgentBootstrapTokenRequest) SetIntegrationIdNil(b bool)`

 SetIntegrationIdNil sets the value for IntegrationId to be an explicit nil

### UnsetIntegrationId
`func (o *LanAgentBootstrapTokenRequest) UnsetIntegrationId()`

UnsetIntegrationId ensures that no value is present for IntegrationId, not even an explicit nil
### GetTaskId

`func (o *LanAgentBootstrapTokenRequest) GetTaskId() string`

GetTaskId returns the TaskId field if non-nil, zero value otherwise.

### GetTaskIdOk

`func (o *LanAgentBootstrapTokenRequest) GetTaskIdOk() (*string, bool)`

GetTaskIdOk returns a tuple with the TaskId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTaskId

`func (o *LanAgentBootstrapTokenRequest) SetTaskId(v string)`

SetTaskId sets TaskId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
