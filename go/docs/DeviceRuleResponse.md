# DeviceRuleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Config** | **map[string]interface{}** |  |
**DeviceId** | **string** |  |
**DeviceName** | [**LocalizedString**](LocalizedString.md) |  |
**Output** | Pointer to **NullableString** | Output the rule is scoped to, or null when it binds every output of the device. | [optional]
**PolicyId** | **string** |  |
**PolicyType** | **string** |  |

## Methods

### NewDeviceRuleResponse

`func NewDeviceRuleResponse(config map[string]interface{}, deviceId string, deviceName LocalizedString, policyId string, policyType string, ) *DeviceRuleResponse`

NewDeviceRuleResponse instantiates a new DeviceRuleResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDeviceRuleResponseWithDefaults

`func NewDeviceRuleResponseWithDefaults() *DeviceRuleResponse`

NewDeviceRuleResponseWithDefaults instantiates a new DeviceRuleResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetConfig

`func (o *DeviceRuleResponse) GetConfig() map[string]interface{}`

GetConfig returns the Config field if non-nil, zero value otherwise.

### GetConfigOk

`func (o *DeviceRuleResponse) GetConfigOk() (*map[string]interface{}, bool)`

GetConfigOk returns a tuple with the Config field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfig

`func (o *DeviceRuleResponse) SetConfig(v map[string]interface{})`

SetConfig sets Config field to given value.


### GetDeviceId

`func (o *DeviceRuleResponse) GetDeviceId() string`

GetDeviceId returns the DeviceId field if non-nil, zero value otherwise.

### GetDeviceIdOk

`func (o *DeviceRuleResponse) GetDeviceIdOk() (*string, bool)`

GetDeviceIdOk returns a tuple with the DeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceId

`func (o *DeviceRuleResponse) SetDeviceId(v string)`

SetDeviceId sets DeviceId field to given value.


### GetDeviceName

`func (o *DeviceRuleResponse) GetDeviceName() LocalizedString`

GetDeviceName returns the DeviceName field if non-nil, zero value otherwise.

### GetDeviceNameOk

`func (o *DeviceRuleResponse) GetDeviceNameOk() (*LocalizedString, bool)`

GetDeviceNameOk returns a tuple with the DeviceName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceName

`func (o *DeviceRuleResponse) SetDeviceName(v LocalizedString)`

SetDeviceName sets DeviceName field to given value.


### GetOutput

`func (o *DeviceRuleResponse) GetOutput() string`

GetOutput returns the Output field if non-nil, zero value otherwise.

### GetOutputOk

`func (o *DeviceRuleResponse) GetOutputOk() (*string, bool)`

GetOutputOk returns a tuple with the Output field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutput

`func (o *DeviceRuleResponse) SetOutput(v string)`

SetOutput sets Output field to given value.

### HasOutput

`func (o *DeviceRuleResponse) HasOutput() bool`

HasOutput returns a boolean if a field has been set.

### SetOutputNil

`func (o *DeviceRuleResponse) SetOutputNil(b bool)`

 SetOutputNil sets the value for Output to be an explicit nil

### UnsetOutput
`func (o *DeviceRuleResponse) UnsetOutput()`

UnsetOutput ensures that no value is present for Output, not even an explicit nil
### GetPolicyId

`func (o *DeviceRuleResponse) GetPolicyId() string`

GetPolicyId returns the PolicyId field if non-nil, zero value otherwise.

### GetPolicyIdOk

`func (o *DeviceRuleResponse) GetPolicyIdOk() (*string, bool)`

GetPolicyIdOk returns a tuple with the PolicyId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyId

`func (o *DeviceRuleResponse) SetPolicyId(v string)`

SetPolicyId sets PolicyId field to given value.


### GetPolicyType

`func (o *DeviceRuleResponse) GetPolicyType() string`

GetPolicyType returns the PolicyType field if non-nil, zero value otherwise.

### GetPolicyTypeOk

`func (o *DeviceRuleResponse) GetPolicyTypeOk() (*string, bool)`

GetPolicyTypeOk returns a tuple with the PolicyType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyType

`func (o *DeviceRuleResponse) SetPolicyType(v string)`

SetPolicyType sets PolicyType field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
