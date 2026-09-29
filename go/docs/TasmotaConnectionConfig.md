# TasmotaConnectionConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CommandTopic** | **string** |  |
**MqttClientId** | **string** |  |
**MqttHost** | **string** |  |
**MqttPassword** | **string** |  |
**MqttPort** | **int32** |  |
**MqttTls** | **bool** |  |
**MqttUsername** | **string** |  |
**PowerTopics** | Pointer to [**[]TasmotaPowerTopic**](TasmotaPowerTopic.md) |  | [optional]
**WebPassword** | **string** |  |

## Methods

### NewTasmotaConnectionConfig

`func NewTasmotaConnectionConfig(commandTopic string, mqttClientId string, mqttHost string, mqttPassword string, mqttPort int32, mqttTls bool, mqttUsername string, webPassword string, ) *TasmotaConnectionConfig`

NewTasmotaConnectionConfig instantiates a new TasmotaConnectionConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTasmotaConnectionConfigWithDefaults

`func NewTasmotaConnectionConfigWithDefaults() *TasmotaConnectionConfig`

NewTasmotaConnectionConfigWithDefaults instantiates a new TasmotaConnectionConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCommandTopic

`func (o *TasmotaConnectionConfig) GetCommandTopic() string`

GetCommandTopic returns the CommandTopic field if non-nil, zero value otherwise.

### GetCommandTopicOk

`func (o *TasmotaConnectionConfig) GetCommandTopicOk() (*string, bool)`

GetCommandTopicOk returns a tuple with the CommandTopic field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCommandTopic

`func (o *TasmotaConnectionConfig) SetCommandTopic(v string)`

SetCommandTopic sets CommandTopic field to given value.


### GetMqttClientId

`func (o *TasmotaConnectionConfig) GetMqttClientId() string`

GetMqttClientId returns the MqttClientId field if non-nil, zero value otherwise.

### GetMqttClientIdOk

`func (o *TasmotaConnectionConfig) GetMqttClientIdOk() (*string, bool)`

GetMqttClientIdOk returns a tuple with the MqttClientId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMqttClientId

`func (o *TasmotaConnectionConfig) SetMqttClientId(v string)`

SetMqttClientId sets MqttClientId field to given value.


### GetMqttHost

`func (o *TasmotaConnectionConfig) GetMqttHost() string`

GetMqttHost returns the MqttHost field if non-nil, zero value otherwise.

### GetMqttHostOk

`func (o *TasmotaConnectionConfig) GetMqttHostOk() (*string, bool)`

GetMqttHostOk returns a tuple with the MqttHost field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMqttHost

`func (o *TasmotaConnectionConfig) SetMqttHost(v string)`

SetMqttHost sets MqttHost field to given value.


### GetMqttPassword

`func (o *TasmotaConnectionConfig) GetMqttPassword() string`

GetMqttPassword returns the MqttPassword field if non-nil, zero value otherwise.

### GetMqttPasswordOk

`func (o *TasmotaConnectionConfig) GetMqttPasswordOk() (*string, bool)`

GetMqttPasswordOk returns a tuple with the MqttPassword field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMqttPassword

`func (o *TasmotaConnectionConfig) SetMqttPassword(v string)`

SetMqttPassword sets MqttPassword field to given value.


### GetMqttPort

`func (o *TasmotaConnectionConfig) GetMqttPort() int32`

GetMqttPort returns the MqttPort field if non-nil, zero value otherwise.

### GetMqttPortOk

`func (o *TasmotaConnectionConfig) GetMqttPortOk() (*int32, bool)`

GetMqttPortOk returns a tuple with the MqttPort field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMqttPort

`func (o *TasmotaConnectionConfig) SetMqttPort(v int32)`

SetMqttPort sets MqttPort field to given value.


### GetMqttTls

`func (o *TasmotaConnectionConfig) GetMqttTls() bool`

GetMqttTls returns the MqttTls field if non-nil, zero value otherwise.

### GetMqttTlsOk

`func (o *TasmotaConnectionConfig) GetMqttTlsOk() (*bool, bool)`

GetMqttTlsOk returns a tuple with the MqttTls field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMqttTls

`func (o *TasmotaConnectionConfig) SetMqttTls(v bool)`

SetMqttTls sets MqttTls field to given value.


### GetMqttUsername

`func (o *TasmotaConnectionConfig) GetMqttUsername() string`

GetMqttUsername returns the MqttUsername field if non-nil, zero value otherwise.

### GetMqttUsernameOk

`func (o *TasmotaConnectionConfig) GetMqttUsernameOk() (*string, bool)`

GetMqttUsernameOk returns a tuple with the MqttUsername field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMqttUsername

`func (o *TasmotaConnectionConfig) SetMqttUsername(v string)`

SetMqttUsername sets MqttUsername field to given value.


### GetPowerTopics

`func (o *TasmotaConnectionConfig) GetPowerTopics() []TasmotaPowerTopic`

GetPowerTopics returns the PowerTopics field if non-nil, zero value otherwise.

### GetPowerTopicsOk

`func (o *TasmotaConnectionConfig) GetPowerTopicsOk() (*[]TasmotaPowerTopic, bool)`

GetPowerTopicsOk returns a tuple with the PowerTopics field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPowerTopics

`func (o *TasmotaConnectionConfig) SetPowerTopics(v []TasmotaPowerTopic)`

SetPowerTopics sets PowerTopics field to given value.

### HasPowerTopics

`func (o *TasmotaConnectionConfig) HasPowerTopics() bool`

HasPowerTopics returns a boolean if a field has been set.

### GetWebPassword

`func (o *TasmotaConnectionConfig) GetWebPassword() string`

GetWebPassword returns the WebPassword field if non-nil, zero value otherwise.

### GetWebPasswordOk

`func (o *TasmotaConnectionConfig) GetWebPasswordOk() (*string, bool)`

GetWebPasswordOk returns a tuple with the WebPassword field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebPassword

`func (o *TasmotaConnectionConfig) SetWebPassword(v string)`

SetWebPassword sets WebPassword field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
