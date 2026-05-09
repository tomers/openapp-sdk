# LanAgentTaskSpecResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**LanProtocolVersion** | **int32** |  |
**Spec** | **interface{}** |  |
**TaskId** | **string** |  |

## Methods

### NewLanAgentTaskSpecResponse

`func NewLanAgentTaskSpecResponse(lanProtocolVersion int32, spec interface{}, taskId string, ) *LanAgentTaskSpecResponse`

NewLanAgentTaskSpecResponse instantiates a new LanAgentTaskSpecResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewLanAgentTaskSpecResponseWithDefaults

`func NewLanAgentTaskSpecResponseWithDefaults() *LanAgentTaskSpecResponse`

NewLanAgentTaskSpecResponseWithDefaults instantiates a new LanAgentTaskSpecResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLanProtocolVersion

`func (o *LanAgentTaskSpecResponse) GetLanProtocolVersion() int32`

GetLanProtocolVersion returns the LanProtocolVersion field if non-nil, zero value otherwise.

### GetLanProtocolVersionOk

`func (o *LanAgentTaskSpecResponse) GetLanProtocolVersionOk() (*int32, bool)`

GetLanProtocolVersionOk returns a tuple with the LanProtocolVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLanProtocolVersion

`func (o *LanAgentTaskSpecResponse) SetLanProtocolVersion(v int32)`

SetLanProtocolVersion sets LanProtocolVersion field to given value.


### GetSpec

`func (o *LanAgentTaskSpecResponse) GetSpec() interface{}`

GetSpec returns the Spec field if non-nil, zero value otherwise.

### GetSpecOk

`func (o *LanAgentTaskSpecResponse) GetSpecOk() (*interface{}, bool)`

GetSpecOk returns a tuple with the Spec field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSpec

`func (o *LanAgentTaskSpecResponse) SetSpec(v interface{})`

SetSpec sets Spec field to given value.


### SetSpecNil

`func (o *LanAgentTaskSpecResponse) SetSpecNil(b bool)`

 SetSpecNil sets the value for Spec to be an explicit nil

### UnsetSpec
`func (o *LanAgentTaskSpecResponse) UnsetSpec()`

UnsetSpec ensures that no value is present for Spec, not even an explicit nil
### GetTaskId

`func (o *LanAgentTaskSpecResponse) GetTaskId() string`

GetTaskId returns the TaskId field if non-nil, zero value otherwise.

### GetTaskIdOk

`func (o *LanAgentTaskSpecResponse) GetTaskIdOk() (*string, bool)`

GetTaskIdOk returns a tuple with the TaskId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTaskId

`func (o *LanAgentTaskSpecResponse) SetTaskId(v string)`

SetTaskId sets TaskId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
