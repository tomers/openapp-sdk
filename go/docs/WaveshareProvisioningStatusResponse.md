# WaveshareProvisioningStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Current** | [**WaveshareConnectionConfig**](WaveshareConnectionConfig.md) |  |
**Diff** | [**[]WaveshareProvisioningDiffEntry**](WaveshareProvisioningDiffEntry.md) |  |
**Stale** | **bool** |  |
**Stored** | [**WaveshareConnectionConfig**](WaveshareConnectionConfig.md) |  |

## Methods

### NewWaveshareProvisioningStatusResponse

`func NewWaveshareProvisioningStatusResponse(current WaveshareConnectionConfig, diff []WaveshareProvisioningDiffEntry, stale bool, stored WaveshareConnectionConfig, ) *WaveshareProvisioningStatusResponse`

NewWaveshareProvisioningStatusResponse instantiates a new WaveshareProvisioningStatusResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewWaveshareProvisioningStatusResponseWithDefaults

`func NewWaveshareProvisioningStatusResponseWithDefaults() *WaveshareProvisioningStatusResponse`

NewWaveshareProvisioningStatusResponseWithDefaults instantiates a new WaveshareProvisioningStatusResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCurrent

`func (o *WaveshareProvisioningStatusResponse) GetCurrent() WaveshareConnectionConfig`

GetCurrent returns the Current field if non-nil, zero value otherwise.

### GetCurrentOk

`func (o *WaveshareProvisioningStatusResponse) GetCurrentOk() (*WaveshareConnectionConfig, bool)`

GetCurrentOk returns a tuple with the Current field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrent

`func (o *WaveshareProvisioningStatusResponse) SetCurrent(v WaveshareConnectionConfig)`

SetCurrent sets Current field to given value.


### GetDiff

`func (o *WaveshareProvisioningStatusResponse) GetDiff() []WaveshareProvisioningDiffEntry`

GetDiff returns the Diff field if non-nil, zero value otherwise.

### GetDiffOk

`func (o *WaveshareProvisioningStatusResponse) GetDiffOk() (*[]WaveshareProvisioningDiffEntry, bool)`

GetDiffOk returns a tuple with the Diff field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDiff

`func (o *WaveshareProvisioningStatusResponse) SetDiff(v []WaveshareProvisioningDiffEntry)`

SetDiff sets Diff field to given value.


### GetStale

`func (o *WaveshareProvisioningStatusResponse) GetStale() bool`

GetStale returns the Stale field if non-nil, zero value otherwise.

### GetStaleOk

`func (o *WaveshareProvisioningStatusResponse) GetStaleOk() (*bool, bool)`

GetStaleOk returns a tuple with the Stale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStale

`func (o *WaveshareProvisioningStatusResponse) SetStale(v bool)`

SetStale sets Stale field to given value.


### GetStored

`func (o *WaveshareProvisioningStatusResponse) GetStored() WaveshareConnectionConfig`

GetStored returns the Stored field if non-nil, zero value otherwise.

### GetStoredOk

`func (o *WaveshareProvisioningStatusResponse) GetStoredOk() (*WaveshareConnectionConfig, bool)`

GetStoredOk returns a tuple with the Stored field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStored

`func (o *WaveshareProvisioningStatusResponse) SetStored(v WaveshareConnectionConfig)`

SetStored sets Stored field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
