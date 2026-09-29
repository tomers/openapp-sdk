# TransferIntegrationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Confirm** | Pointer to **bool** | Must be &#x60;true&#x60; when the preview reports side-effect warnings; otherwise the request is rejected with &#x60;error_code &#x3D; \&quot;confirmation_required\&quot;&#x60;. | [optional]
**Mode** | Pointer to [**TransferModeDto**](TransferModeDto.md) |  | [optional]
**TargetOrgId** | **string** | Destination organization id (must differ from the integration&#39;s current org). |

## Methods

### NewTransferIntegrationRequest

`func NewTransferIntegrationRequest(targetOrgId string, ) *TransferIntegrationRequest`

NewTransferIntegrationRequest instantiates a new TransferIntegrationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTransferIntegrationRequestWithDefaults

`func NewTransferIntegrationRequestWithDefaults() *TransferIntegrationRequest`

NewTransferIntegrationRequestWithDefaults instantiates a new TransferIntegrationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetConfirm

`func (o *TransferIntegrationRequest) GetConfirm() bool`

GetConfirm returns the Confirm field if non-nil, zero value otherwise.

### GetConfirmOk

`func (o *TransferIntegrationRequest) GetConfirmOk() (*bool, bool)`

GetConfirmOk returns a tuple with the Confirm field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfirm

`func (o *TransferIntegrationRequest) SetConfirm(v bool)`

SetConfirm sets Confirm field to given value.

### HasConfirm

`func (o *TransferIntegrationRequest) HasConfirm() bool`

HasConfirm returns a boolean if a field has been set.

### GetMode

`func (o *TransferIntegrationRequest) GetMode() TransferModeDto`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *TransferIntegrationRequest) GetModeOk() (*TransferModeDto, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *TransferIntegrationRequest) SetMode(v TransferModeDto)`

SetMode sets Mode field to given value.

### HasMode

`func (o *TransferIntegrationRequest) HasMode() bool`

HasMode returns a boolean if a field has been set.

### GetTargetOrgId

`func (o *TransferIntegrationRequest) GetTargetOrgId() string`

GetTargetOrgId returns the TargetOrgId field if non-nil, zero value otherwise.

### GetTargetOrgIdOk

`func (o *TransferIntegrationRequest) GetTargetOrgIdOk() (*string, bool)`

GetTargetOrgIdOk returns a tuple with the TargetOrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTargetOrgId

`func (o *TransferIntegrationRequest) SetTargetOrgId(v string)`

SetTargetOrgId sets TargetOrgId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
