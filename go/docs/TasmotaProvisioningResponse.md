# TasmotaProvisioningResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Backlog** | **string** |  |
**Connection** | [**TasmotaConnectionConfig**](TasmotaConnectionConfig.md) |  |
**MqttCaPem** | Pointer to **NullableString** |  | [optional]

## Methods

### NewTasmotaProvisioningResponse

`func NewTasmotaProvisioningResponse(backlog string, connection TasmotaConnectionConfig, ) *TasmotaProvisioningResponse`

NewTasmotaProvisioningResponse instantiates a new TasmotaProvisioningResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTasmotaProvisioningResponseWithDefaults

`func NewTasmotaProvisioningResponseWithDefaults() *TasmotaProvisioningResponse`

NewTasmotaProvisioningResponseWithDefaults instantiates a new TasmotaProvisioningResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBacklog

`func (o *TasmotaProvisioningResponse) GetBacklog() string`

GetBacklog returns the Backlog field if non-nil, zero value otherwise.

### GetBacklogOk

`func (o *TasmotaProvisioningResponse) GetBacklogOk() (*string, bool)`

GetBacklogOk returns a tuple with the Backlog field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBacklog

`func (o *TasmotaProvisioningResponse) SetBacklog(v string)`

SetBacklog sets Backlog field to given value.


### GetConnection

`func (o *TasmotaProvisioningResponse) GetConnection() TasmotaConnectionConfig`

GetConnection returns the Connection field if non-nil, zero value otherwise.

### GetConnectionOk

`func (o *TasmotaProvisioningResponse) GetConnectionOk() (*TasmotaConnectionConfig, bool)`

GetConnectionOk returns a tuple with the Connection field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConnection

`func (o *TasmotaProvisioningResponse) SetConnection(v TasmotaConnectionConfig)`

SetConnection sets Connection field to given value.


### GetMqttCaPem

`func (o *TasmotaProvisioningResponse) GetMqttCaPem() string`

GetMqttCaPem returns the MqttCaPem field if non-nil, zero value otherwise.

### GetMqttCaPemOk

`func (o *TasmotaProvisioningResponse) GetMqttCaPemOk() (*string, bool)`

GetMqttCaPemOk returns a tuple with the MqttCaPem field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMqttCaPem

`func (o *TasmotaProvisioningResponse) SetMqttCaPem(v string)`

SetMqttCaPem sets MqttCaPem field to given value.

### HasMqttCaPem

`func (o *TasmotaProvisioningResponse) HasMqttCaPem() bool`

HasMqttCaPem returns a boolean if a field has been set.

### SetMqttCaPemNil

`func (o *TasmotaProvisioningResponse) SetMqttCaPemNil(b bool)`

 SetMqttCaPemNil sets the value for MqttCaPem to be an explicit nil

### UnsetMqttCaPem
`func (o *TasmotaProvisioningResponse) UnsetMqttCaPem()`

UnsetMqttCaPem ensures that no value is present for MqttCaPem, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
