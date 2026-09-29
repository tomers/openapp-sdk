# TransferPreviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Affected** | [**AffectedCounts**](AffectedCounts.md) |  |
**Mode** | **string** |  |
**Notes** | **[]string** | Informational notes the dashboard should display before transferring. Unlike &#x60;warnings&#x60;, these never gate &#x60;requires_confirmation&#x60; — they inform without requiring a confirm step. |
**RequiresConfirmation** | **bool** | When true, &#x60;POST /transfer&#x60; must be called with &#x60;confirm &#x3D; true&#x60;. |
**SourceOrgId** | **string** |  |
**Warnings** | **[]string** | Side-effect warning codes the dashboard should surface before confirming. |

## Methods

### NewTransferPreviewResponse

`func NewTransferPreviewResponse(affected AffectedCounts, mode string, notes []string, requiresConfirmation bool, sourceOrgId string, warnings []string, ) *TransferPreviewResponse`

NewTransferPreviewResponse instantiates a new TransferPreviewResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTransferPreviewResponseWithDefaults

`func NewTransferPreviewResponseWithDefaults() *TransferPreviewResponse`

NewTransferPreviewResponseWithDefaults instantiates a new TransferPreviewResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAffected

`func (o *TransferPreviewResponse) GetAffected() AffectedCounts`

GetAffected returns the Affected field if non-nil, zero value otherwise.

### GetAffectedOk

`func (o *TransferPreviewResponse) GetAffectedOk() (*AffectedCounts, bool)`

GetAffectedOk returns a tuple with the Affected field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAffected

`func (o *TransferPreviewResponse) SetAffected(v AffectedCounts)`

SetAffected sets Affected field to given value.


### GetMode

`func (o *TransferPreviewResponse) GetMode() string`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *TransferPreviewResponse) GetModeOk() (*string, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *TransferPreviewResponse) SetMode(v string)`

SetMode sets Mode field to given value.


### GetNotes

`func (o *TransferPreviewResponse) GetNotes() []string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *TransferPreviewResponse) GetNotesOk() (*[]string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *TransferPreviewResponse) SetNotes(v []string)`

SetNotes sets Notes field to given value.


### GetRequiresConfirmation

`func (o *TransferPreviewResponse) GetRequiresConfirmation() bool`

GetRequiresConfirmation returns the RequiresConfirmation field if non-nil, zero value otherwise.

### GetRequiresConfirmationOk

`func (o *TransferPreviewResponse) GetRequiresConfirmationOk() (*bool, bool)`

GetRequiresConfirmationOk returns a tuple with the RequiresConfirmation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRequiresConfirmation

`func (o *TransferPreviewResponse) SetRequiresConfirmation(v bool)`

SetRequiresConfirmation sets RequiresConfirmation field to given value.


### GetSourceOrgId

`func (o *TransferPreviewResponse) GetSourceOrgId() string`

GetSourceOrgId returns the SourceOrgId field if non-nil, zero value otherwise.

### GetSourceOrgIdOk

`func (o *TransferPreviewResponse) GetSourceOrgIdOk() (*string, bool)`

GetSourceOrgIdOk returns a tuple with the SourceOrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceOrgId

`func (o *TransferPreviewResponse) SetSourceOrgId(v string)`

SetSourceOrgId sets SourceOrgId field to given value.


### GetWarnings

`func (o *TransferPreviewResponse) GetWarnings() []string`

GetWarnings returns the Warnings field if non-nil, zero value otherwise.

### GetWarningsOk

`func (o *TransferPreviewResponse) GetWarningsOk() (*[]string, bool)`

GetWarningsOk returns a tuple with the Warnings field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWarnings

`func (o *TransferPreviewResponse) SetWarnings(v []string)`

SetWarnings sets Warnings field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
