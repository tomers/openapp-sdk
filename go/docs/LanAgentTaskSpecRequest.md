# LanAgentTaskSpecRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**LanProtocolVersion** | **int32** |  |
**Payload** | **interface{}** |  |
**TaskId** | **string** |  |

## Methods

### NewLanAgentTaskSpecRequest

`func NewLanAgentTaskSpecRequest(lanProtocolVersion int32, payload interface{}, taskId string, ) *LanAgentTaskSpecRequest`

NewLanAgentTaskSpecRequest instantiates a new LanAgentTaskSpecRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewLanAgentTaskSpecRequestWithDefaults

`func NewLanAgentTaskSpecRequestWithDefaults() *LanAgentTaskSpecRequest`

NewLanAgentTaskSpecRequestWithDefaults instantiates a new LanAgentTaskSpecRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLanProtocolVersion

`func (o *LanAgentTaskSpecRequest) GetLanProtocolVersion() int32`

GetLanProtocolVersion returns the LanProtocolVersion field if non-nil, zero value otherwise.

### GetLanProtocolVersionOk

`func (o *LanAgentTaskSpecRequest) GetLanProtocolVersionOk() (*int32, bool)`

GetLanProtocolVersionOk returns a tuple with the LanProtocolVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLanProtocolVersion

`func (o *LanAgentTaskSpecRequest) SetLanProtocolVersion(v int32)`

SetLanProtocolVersion sets LanProtocolVersion field to given value.


### GetPayload

`func (o *LanAgentTaskSpecRequest) GetPayload() interface{}`

GetPayload returns the Payload field if non-nil, zero value otherwise.

### GetPayloadOk

`func (o *LanAgentTaskSpecRequest) GetPayloadOk() (*interface{}, bool)`

GetPayloadOk returns a tuple with the Payload field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPayload

`func (o *LanAgentTaskSpecRequest) SetPayload(v interface{})`

SetPayload sets Payload field to given value.


### SetPayloadNil

`func (o *LanAgentTaskSpecRequest) SetPayloadNil(b bool)`

 SetPayloadNil sets the value for Payload to be an explicit nil

### UnsetPayload
`func (o *LanAgentTaskSpecRequest) UnsetPayload()`

UnsetPayload ensures that no value is present for Payload, not even an explicit nil
### GetTaskId

`func (o *LanAgentTaskSpecRequest) GetTaskId() string`

GetTaskId returns the TaskId field if non-nil, zero value otherwise.

### GetTaskIdOk

`func (o *LanAgentTaskSpecRequest) GetTaskIdOk() (*string, bool)`

GetTaskIdOk returns a tuple with the TaskId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTaskId

`func (o *LanAgentTaskSpecRequest) SetTaskId(v string)`

SetTaskId sets TaskId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
