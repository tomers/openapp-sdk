# PostMeCallReportPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Details** | Pointer to **NullableString** |  | [optional]
**PublicPortalId** | Pointer to **NullableString** |  | [optional]
**Reason** | **string** |  |
**SessionId** | **string** |  |

## Methods

### NewPostMeCallReportPayload

`func NewPostMeCallReportPayload(reason string, sessionId string, ) *PostMeCallReportPayload`

NewPostMeCallReportPayload instantiates a new PostMeCallReportPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPostMeCallReportPayloadWithDefaults

`func NewPostMeCallReportPayloadWithDefaults() *PostMeCallReportPayload`

NewPostMeCallReportPayloadWithDefaults instantiates a new PostMeCallReportPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDetails

`func (o *PostMeCallReportPayload) GetDetails() string`

GetDetails returns the Details field if non-nil, zero value otherwise.

### GetDetailsOk

`func (o *PostMeCallReportPayload) GetDetailsOk() (*string, bool)`

GetDetailsOk returns a tuple with the Details field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDetails

`func (o *PostMeCallReportPayload) SetDetails(v string)`

SetDetails sets Details field to given value.

### HasDetails

`func (o *PostMeCallReportPayload) HasDetails() bool`

HasDetails returns a boolean if a field has been set.

### SetDetailsNil

`func (o *PostMeCallReportPayload) SetDetailsNil(b bool)`

 SetDetailsNil sets the value for Details to be an explicit nil

### UnsetDetails
`func (o *PostMeCallReportPayload) UnsetDetails()`

UnsetDetails ensures that no value is present for Details, not even an explicit nil
### GetPublicPortalId

`func (o *PostMeCallReportPayload) GetPublicPortalId() string`

GetPublicPortalId returns the PublicPortalId field if non-nil, zero value otherwise.

### GetPublicPortalIdOk

`func (o *PostMeCallReportPayload) GetPublicPortalIdOk() (*string, bool)`

GetPublicPortalIdOk returns a tuple with the PublicPortalId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublicPortalId

`func (o *PostMeCallReportPayload) SetPublicPortalId(v string)`

SetPublicPortalId sets PublicPortalId field to given value.

### HasPublicPortalId

`func (o *PostMeCallReportPayload) HasPublicPortalId() bool`

HasPublicPortalId returns a boolean if a field has been set.

### SetPublicPortalIdNil

`func (o *PostMeCallReportPayload) SetPublicPortalIdNil(b bool)`

 SetPublicPortalIdNil sets the value for PublicPortalId to be an explicit nil

### UnsetPublicPortalId
`func (o *PostMeCallReportPayload) UnsetPublicPortalId()`

UnsetPublicPortalId ensures that no value is present for PublicPortalId, not even an explicit nil
### GetReason

`func (o *PostMeCallReportPayload) GetReason() string`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *PostMeCallReportPayload) GetReasonOk() (*string, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *PostMeCallReportPayload) SetReason(v string)`

SetReason sets Reason field to given value.


### GetSessionId

`func (o *PostMeCallReportPayload) GetSessionId() string`

GetSessionId returns the SessionId field if non-nil, zero value otherwise.

### GetSessionIdOk

`func (o *PostMeCallReportPayload) GetSessionIdOk() (*string, bool)`

GetSessionIdOk returns a tuple with the SessionId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSessionId

`func (o *PostMeCallReportPayload) SetSessionId(v string)`

SetSessionId sets SessionId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
