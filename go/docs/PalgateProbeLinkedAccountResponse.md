# PalgateProbeLinkedAccountResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**LinkedAccountAdminMode** | **string** |  |
**Probe** | [**PalgateLinkedAccountProbeDto**](PalgateLinkedAccountProbeDto.md) |  |

## Methods

### NewPalgateProbeLinkedAccountResponse

`func NewPalgateProbeLinkedAccountResponse(linkedAccountAdminMode string, probe PalgateLinkedAccountProbeDto, ) *PalgateProbeLinkedAccountResponse`

NewPalgateProbeLinkedAccountResponse instantiates a new PalgateProbeLinkedAccountResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPalgateProbeLinkedAccountResponseWithDefaults

`func NewPalgateProbeLinkedAccountResponseWithDefaults() *PalgateProbeLinkedAccountResponse`

NewPalgateProbeLinkedAccountResponseWithDefaults instantiates a new PalgateProbeLinkedAccountResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLinkedAccountAdminMode

`func (o *PalgateProbeLinkedAccountResponse) GetLinkedAccountAdminMode() string`

GetLinkedAccountAdminMode returns the LinkedAccountAdminMode field if non-nil, zero value otherwise.

### GetLinkedAccountAdminModeOk

`func (o *PalgateProbeLinkedAccountResponse) GetLinkedAccountAdminModeOk() (*string, bool)`

GetLinkedAccountAdminModeOk returns a tuple with the LinkedAccountAdminMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLinkedAccountAdminMode

`func (o *PalgateProbeLinkedAccountResponse) SetLinkedAccountAdminMode(v string)`

SetLinkedAccountAdminMode sets LinkedAccountAdminMode field to given value.


### GetProbe

`func (o *PalgateProbeLinkedAccountResponse) GetProbe() PalgateLinkedAccountProbeDto`

GetProbe returns the Probe field if non-nil, zero value otherwise.

### GetProbeOk

`func (o *PalgateProbeLinkedAccountResponse) GetProbeOk() (*PalgateLinkedAccountProbeDto, bool)`

GetProbeOk returns a tuple with the Probe field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProbe

`func (o *PalgateProbeLinkedAccountResponse) SetProbe(v PalgateLinkedAccountProbeDto)`

SetProbe sets Probe field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
