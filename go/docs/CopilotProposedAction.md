# CopilotProposedAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Args** | **map[string]interface{}** |  |
**Preview** | Pointer to **NullableString** |  | [optional]
**Tool** | **string** |  |

## Methods

### NewCopilotProposedAction

`func NewCopilotProposedAction(args map[string]interface{}, tool string, ) *CopilotProposedAction`

NewCopilotProposedAction instantiates a new CopilotProposedAction object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCopilotProposedActionWithDefaults

`func NewCopilotProposedActionWithDefaults() *CopilotProposedAction`

NewCopilotProposedActionWithDefaults instantiates a new CopilotProposedAction object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetArgs

`func (o *CopilotProposedAction) GetArgs() map[string]interface{}`

GetArgs returns the Args field if non-nil, zero value otherwise.

### GetArgsOk

`func (o *CopilotProposedAction) GetArgsOk() (*map[string]interface{}, bool)`

GetArgsOk returns a tuple with the Args field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetArgs

`func (o *CopilotProposedAction) SetArgs(v map[string]interface{})`

SetArgs sets Args field to given value.


### GetPreview

`func (o *CopilotProposedAction) GetPreview() string`

GetPreview returns the Preview field if non-nil, zero value otherwise.

### GetPreviewOk

`func (o *CopilotProposedAction) GetPreviewOk() (*string, bool)`

GetPreviewOk returns a tuple with the Preview field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreview

`func (o *CopilotProposedAction) SetPreview(v string)`

SetPreview sets Preview field to given value.

### HasPreview

`func (o *CopilotProposedAction) HasPreview() bool`

HasPreview returns a boolean if a field has been set.

### SetPreviewNil

`func (o *CopilotProposedAction) SetPreviewNil(b bool)`

 SetPreviewNil sets the value for Preview to be an explicit nil

### UnsetPreview
`func (o *CopilotProposedAction) UnsetPreview()`

UnsetPreview ensures that no value is present for Preview, not even an explicit nil
### GetTool

`func (o *CopilotProposedAction) GetTool() string`

GetTool returns the Tool field if non-nil, zero value otherwise.

### GetToolOk

`func (o *CopilotProposedAction) GetToolOk() (*string, bool)`

GetToolOk returns a tuple with the Tool field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTool

`func (o *CopilotProposedAction) SetTool(v string)`

SetTool sets Tool field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
