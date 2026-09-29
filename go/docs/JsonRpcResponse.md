# JsonRpcResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Error** | Pointer to **interface{}** |  | [optional]
**Id** | Pointer to **interface{}** |  | [optional]
**Jsonrpc** | **string** |  |
**Result** | Pointer to **interface{}** |  | [optional]

## Methods

### NewJsonRpcResponse

`func NewJsonRpcResponse(jsonrpc string, ) *JsonRpcResponse`

NewJsonRpcResponse instantiates a new JsonRpcResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewJsonRpcResponseWithDefaults

`func NewJsonRpcResponseWithDefaults() *JsonRpcResponse`

NewJsonRpcResponseWithDefaults instantiates a new JsonRpcResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetError

`func (o *JsonRpcResponse) GetError() interface{}`

GetError returns the Error field if non-nil, zero value otherwise.

### GetErrorOk

`func (o *JsonRpcResponse) GetErrorOk() (*interface{}, bool)`

GetErrorOk returns a tuple with the Error field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetError

`func (o *JsonRpcResponse) SetError(v interface{})`

SetError sets Error field to given value.

### HasError

`func (o *JsonRpcResponse) HasError() bool`

HasError returns a boolean if a field has been set.

### SetErrorNil

`func (o *JsonRpcResponse) SetErrorNil(b bool)`

 SetErrorNil sets the value for Error to be an explicit nil

### UnsetError
`func (o *JsonRpcResponse) UnsetError()`

UnsetError ensures that no value is present for Error, not even an explicit nil
### GetId

`func (o *JsonRpcResponse) GetId() interface{}`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *JsonRpcResponse) GetIdOk() (*interface{}, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *JsonRpcResponse) SetId(v interface{})`

SetId sets Id field to given value.

### HasId

`func (o *JsonRpcResponse) HasId() bool`

HasId returns a boolean if a field has been set.

### SetIdNil

`func (o *JsonRpcResponse) SetIdNil(b bool)`

 SetIdNil sets the value for Id to be an explicit nil

### UnsetId
`func (o *JsonRpcResponse) UnsetId()`

UnsetId ensures that no value is present for Id, not even an explicit nil
### GetJsonrpc

`func (o *JsonRpcResponse) GetJsonrpc() string`

GetJsonrpc returns the Jsonrpc field if non-nil, zero value otherwise.

### GetJsonrpcOk

`func (o *JsonRpcResponse) GetJsonrpcOk() (*string, bool)`

GetJsonrpcOk returns a tuple with the Jsonrpc field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJsonrpc

`func (o *JsonRpcResponse) SetJsonrpc(v string)`

SetJsonrpc sets Jsonrpc field to given value.


### GetResult

`func (o *JsonRpcResponse) GetResult() interface{}`

GetResult returns the Result field if non-nil, zero value otherwise.

### GetResultOk

`func (o *JsonRpcResponse) GetResultOk() (*interface{}, bool)`

GetResultOk returns a tuple with the Result field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResult

`func (o *JsonRpcResponse) SetResult(v interface{})`

SetResult sets Result field to given value.

### HasResult

`func (o *JsonRpcResponse) HasResult() bool`

HasResult returns a boolean if a field has been set.

### SetResultNil

`func (o *JsonRpcResponse) SetResultNil(b bool)`

 SetResultNil sets the value for Result to be an explicit nil

### UnsetResult
`func (o *JsonRpcResponse) UnsetResult()`

UnsetResult ensures that no value is present for Result, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
