# PublicSessionStreamsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeviceIds** | **[]string** | Camera device IDs for the portal. Stream URLs resolved client-side or via future API. |

## Methods

### NewPublicSessionStreamsResponse

`func NewPublicSessionStreamsResponse(deviceIds []string, ) *PublicSessionStreamsResponse`

NewPublicSessionStreamsResponse instantiates a new PublicSessionStreamsResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicSessionStreamsResponseWithDefaults

`func NewPublicSessionStreamsResponseWithDefaults() *PublicSessionStreamsResponse`

NewPublicSessionStreamsResponseWithDefaults instantiates a new PublicSessionStreamsResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeviceIds

`func (o *PublicSessionStreamsResponse) GetDeviceIds() []string`

GetDeviceIds returns the DeviceIds field if non-nil, zero value otherwise.

### GetDeviceIdsOk

`func (o *PublicSessionStreamsResponse) GetDeviceIdsOk() (*[]string, bool)`

GetDeviceIdsOk returns a tuple with the DeviceIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeviceIds

`func (o *PublicSessionStreamsResponse) SetDeviceIds(v []string)`

SetDeviceIds sets DeviceIds field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
