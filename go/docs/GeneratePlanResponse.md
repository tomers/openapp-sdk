# GeneratePlanResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Create** | [**[]GeneratePlanEntry**](GeneratePlanEntry.md) |  |
**Skipped** | [**[]GeneratePlanEntry**](GeneratePlanEntry.md) |  |
**Unchanged** | [**[]GeneratePlanEntry**](GeneratePlanEntry.md) |  |
**Update** | [**[]GeneratePlanEntry**](GeneratePlanEntry.md) |  |

## Methods

### NewGeneratePlanResponse

`func NewGeneratePlanResponse(create []GeneratePlanEntry, skipped []GeneratePlanEntry, unchanged []GeneratePlanEntry, update []GeneratePlanEntry, ) *GeneratePlanResponse`

NewGeneratePlanResponse instantiates a new GeneratePlanResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGeneratePlanResponseWithDefaults

`func NewGeneratePlanResponseWithDefaults() *GeneratePlanResponse`

NewGeneratePlanResponseWithDefaults instantiates a new GeneratePlanResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreate

`func (o *GeneratePlanResponse) GetCreate() []GeneratePlanEntry`

GetCreate returns the Create field if non-nil, zero value otherwise.

### GetCreateOk

`func (o *GeneratePlanResponse) GetCreateOk() (*[]GeneratePlanEntry, bool)`

GetCreateOk returns a tuple with the Create field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreate

`func (o *GeneratePlanResponse) SetCreate(v []GeneratePlanEntry)`

SetCreate sets Create field to given value.


### GetSkipped

`func (o *GeneratePlanResponse) GetSkipped() []GeneratePlanEntry`

GetSkipped returns the Skipped field if non-nil, zero value otherwise.

### GetSkippedOk

`func (o *GeneratePlanResponse) GetSkippedOk() (*[]GeneratePlanEntry, bool)`

GetSkippedOk returns a tuple with the Skipped field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSkipped

`func (o *GeneratePlanResponse) SetSkipped(v []GeneratePlanEntry)`

SetSkipped sets Skipped field to given value.


### GetUnchanged

`func (o *GeneratePlanResponse) GetUnchanged() []GeneratePlanEntry`

GetUnchanged returns the Unchanged field if non-nil, zero value otherwise.

### GetUnchangedOk

`func (o *GeneratePlanResponse) GetUnchangedOk() (*[]GeneratePlanEntry, bool)`

GetUnchangedOk returns a tuple with the Unchanged field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnchanged

`func (o *GeneratePlanResponse) SetUnchanged(v []GeneratePlanEntry)`

SetUnchanged sets Unchanged field to given value.


### GetUpdate

`func (o *GeneratePlanResponse) GetUpdate() []GeneratePlanEntry`

GetUpdate returns the Update field if non-nil, zero value otherwise.

### GetUpdateOk

`func (o *GeneratePlanResponse) GetUpdateOk() (*[]GeneratePlanEntry, bool)`

GetUpdateOk returns a tuple with the Update field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdate

`func (o *GeneratePlanResponse) SetUpdate(v []GeneratePlanEntry)`

SetUpdate sets Update field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
