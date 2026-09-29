# CurfewWindowResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**End** | **string** | Local clock &#x60;HH:MM&#x60;. Inclusive; may be earlier than &#x60;start&#x60; (midnight wrap). |
**Start** | **string** | Local clock &#x60;HH:MM&#x60;. |

## Methods

### NewCurfewWindowResponse

`func NewCurfewWindowResponse(end string, start string, ) *CurfewWindowResponse`

NewCurfewWindowResponse instantiates a new CurfewWindowResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCurfewWindowResponseWithDefaults

`func NewCurfewWindowResponseWithDefaults() *CurfewWindowResponse`

NewCurfewWindowResponseWithDefaults instantiates a new CurfewWindowResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEnd

`func (o *CurfewWindowResponse) GetEnd() string`

GetEnd returns the End field if non-nil, zero value otherwise.

### GetEndOk

`func (o *CurfewWindowResponse) GetEndOk() (*string, bool)`

GetEndOk returns a tuple with the End field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnd

`func (o *CurfewWindowResponse) SetEnd(v string)`

SetEnd sets End field to given value.


### GetStart

`func (o *CurfewWindowResponse) GetStart() string`

GetStart returns the Start field if non-nil, zero value otherwise.

### GetStartOk

`func (o *CurfewWindowResponse) GetStartOk() (*string, bool)`

GetStartOk returns a tuple with the Start field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStart

`func (o *CurfewWindowResponse) SetStart(v string)`

SetStart sets Start field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
