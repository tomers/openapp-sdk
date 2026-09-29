# CreateSkillRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AgentId** | **string** |  |
**Enabled** | Pointer to **NullableBool** |  | [optional]
**Name** | **string** |  |
**Steps** | [**[]SkillStep**](SkillStep.md) |  |
**TriggerEventTypes** | **[]string** |  |

## Methods

### NewCreateSkillRequest

`func NewCreateSkillRequest(agentId string, name string, steps []SkillStep, triggerEventTypes []string, ) *CreateSkillRequest`

NewCreateSkillRequest instantiates a new CreateSkillRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateSkillRequestWithDefaults

`func NewCreateSkillRequestWithDefaults() *CreateSkillRequest`

NewCreateSkillRequestWithDefaults instantiates a new CreateSkillRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAgentId

`func (o *CreateSkillRequest) GetAgentId() string`

GetAgentId returns the AgentId field if non-nil, zero value otherwise.

### GetAgentIdOk

`func (o *CreateSkillRequest) GetAgentIdOk() (*string, bool)`

GetAgentIdOk returns a tuple with the AgentId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAgentId

`func (o *CreateSkillRequest) SetAgentId(v string)`

SetAgentId sets AgentId field to given value.


### GetEnabled

`func (o *CreateSkillRequest) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *CreateSkillRequest) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *CreateSkillRequest) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.

### HasEnabled

`func (o *CreateSkillRequest) HasEnabled() bool`

HasEnabled returns a boolean if a field has been set.

### SetEnabledNil

`func (o *CreateSkillRequest) SetEnabledNil(b bool)`

 SetEnabledNil sets the value for Enabled to be an explicit nil

### UnsetEnabled
`func (o *CreateSkillRequest) UnsetEnabled()`

UnsetEnabled ensures that no value is present for Enabled, not even an explicit nil
### GetName

`func (o *CreateSkillRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateSkillRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateSkillRequest) SetName(v string)`

SetName sets Name field to given value.


### GetSteps

`func (o *CreateSkillRequest) GetSteps() []SkillStep`

GetSteps returns the Steps field if non-nil, zero value otherwise.

### GetStepsOk

`func (o *CreateSkillRequest) GetStepsOk() (*[]SkillStep, bool)`

GetStepsOk returns a tuple with the Steps field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSteps

`func (o *CreateSkillRequest) SetSteps(v []SkillStep)`

SetSteps sets Steps field to given value.


### GetTriggerEventTypes

`func (o *CreateSkillRequest) GetTriggerEventTypes() []string`

GetTriggerEventTypes returns the TriggerEventTypes field if non-nil, zero value otherwise.

### GetTriggerEventTypesOk

`func (o *CreateSkillRequest) GetTriggerEventTypesOk() (*[]string, bool)`

GetTriggerEventTypesOk returns a tuple with the TriggerEventTypes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTriggerEventTypes

`func (o *CreateSkillRequest) SetTriggerEventTypes(v []string)`

SetTriggerEventTypes sets TriggerEventTypes field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
