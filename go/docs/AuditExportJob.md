# AuditExportJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CompletedAt** | Pointer to **NullableString** |  | [optional]
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**DownloadUrl** | Pointer to **NullableString** | Signed download URL, present only when &#x60;status &#x3D; completed&#x60;. | [optional]
**ErrorMessage** | Pointer to **NullableString** |  | [optional]
**Format** | **string** |  |
**Id** | **string** |  |
**OccurredAfter** | Pointer to **NullableString** |  | [optional]
**OccurredBefore** | Pointer to **NullableString** |  | [optional]
**OrgId** | **string** |  |
**RowCount** | Pointer to **NullableInt64** |  | [optional]
**Status** | **string** |  |

## Methods

### NewAuditExportJob

`func NewAuditExportJob(format string, id string, orgId string, status string, ) *AuditExportJob`

NewAuditExportJob instantiates a new AuditExportJob object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAuditExportJobWithDefaults

`func NewAuditExportJobWithDefaults() *AuditExportJob`

NewAuditExportJobWithDefaults instantiates a new AuditExportJob object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCompletedAt

`func (o *AuditExportJob) GetCompletedAt() string`

GetCompletedAt returns the CompletedAt field if non-nil, zero value otherwise.

### GetCompletedAtOk

`func (o *AuditExportJob) GetCompletedAtOk() (*string, bool)`

GetCompletedAtOk returns a tuple with the CompletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompletedAt

`func (o *AuditExportJob) SetCompletedAt(v string)`

SetCompletedAt sets CompletedAt field to given value.

### HasCompletedAt

`func (o *AuditExportJob) HasCompletedAt() bool`

HasCompletedAt returns a boolean if a field has been set.

### SetCompletedAtNil

`func (o *AuditExportJob) SetCompletedAtNil(b bool)`

 SetCompletedAtNil sets the value for CompletedAt to be an explicit nil

### UnsetCompletedAt
`func (o *AuditExportJob) UnsetCompletedAt()`

UnsetCompletedAt ensures that no value is present for CompletedAt, not even an explicit nil
### GetCreatedAt

`func (o *AuditExportJob) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *AuditExportJob) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *AuditExportJob) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *AuditExportJob) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *AuditExportJob) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *AuditExportJob) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDownloadUrl

`func (o *AuditExportJob) GetDownloadUrl() string`

GetDownloadUrl returns the DownloadUrl field if non-nil, zero value otherwise.

### GetDownloadUrlOk

`func (o *AuditExportJob) GetDownloadUrlOk() (*string, bool)`

GetDownloadUrlOk returns a tuple with the DownloadUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDownloadUrl

`func (o *AuditExportJob) SetDownloadUrl(v string)`

SetDownloadUrl sets DownloadUrl field to given value.

### HasDownloadUrl

`func (o *AuditExportJob) HasDownloadUrl() bool`

HasDownloadUrl returns a boolean if a field has been set.

### SetDownloadUrlNil

`func (o *AuditExportJob) SetDownloadUrlNil(b bool)`

 SetDownloadUrlNil sets the value for DownloadUrl to be an explicit nil

### UnsetDownloadUrl
`func (o *AuditExportJob) UnsetDownloadUrl()`

UnsetDownloadUrl ensures that no value is present for DownloadUrl, not even an explicit nil
### GetErrorMessage

`func (o *AuditExportJob) GetErrorMessage() string`

GetErrorMessage returns the ErrorMessage field if non-nil, zero value otherwise.

### GetErrorMessageOk

`func (o *AuditExportJob) GetErrorMessageOk() (*string, bool)`

GetErrorMessageOk returns a tuple with the ErrorMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorMessage

`func (o *AuditExportJob) SetErrorMessage(v string)`

SetErrorMessage sets ErrorMessage field to given value.

### HasErrorMessage

`func (o *AuditExportJob) HasErrorMessage() bool`

HasErrorMessage returns a boolean if a field has been set.

### SetErrorMessageNil

`func (o *AuditExportJob) SetErrorMessageNil(b bool)`

 SetErrorMessageNil sets the value for ErrorMessage to be an explicit nil

### UnsetErrorMessage
`func (o *AuditExportJob) UnsetErrorMessage()`

UnsetErrorMessage ensures that no value is present for ErrorMessage, not even an explicit nil
### GetFormat

`func (o *AuditExportJob) GetFormat() string`

GetFormat returns the Format field if non-nil, zero value otherwise.

### GetFormatOk

`func (o *AuditExportJob) GetFormatOk() (*string, bool)`

GetFormatOk returns a tuple with the Format field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormat

`func (o *AuditExportJob) SetFormat(v string)`

SetFormat sets Format field to given value.


### GetId

`func (o *AuditExportJob) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AuditExportJob) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AuditExportJob) SetId(v string)`

SetId sets Id field to given value.


### GetOccurredAfter

`func (o *AuditExportJob) GetOccurredAfter() string`

GetOccurredAfter returns the OccurredAfter field if non-nil, zero value otherwise.

### GetOccurredAfterOk

`func (o *AuditExportJob) GetOccurredAfterOk() (*string, bool)`

GetOccurredAfterOk returns a tuple with the OccurredAfter field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredAfter

`func (o *AuditExportJob) SetOccurredAfter(v string)`

SetOccurredAfter sets OccurredAfter field to given value.

### HasOccurredAfter

`func (o *AuditExportJob) HasOccurredAfter() bool`

HasOccurredAfter returns a boolean if a field has been set.

### SetOccurredAfterNil

`func (o *AuditExportJob) SetOccurredAfterNil(b bool)`

 SetOccurredAfterNil sets the value for OccurredAfter to be an explicit nil

### UnsetOccurredAfter
`func (o *AuditExportJob) UnsetOccurredAfter()`

UnsetOccurredAfter ensures that no value is present for OccurredAfter, not even an explicit nil
### GetOccurredBefore

`func (o *AuditExportJob) GetOccurredBefore() string`

GetOccurredBefore returns the OccurredBefore field if non-nil, zero value otherwise.

### GetOccurredBeforeOk

`func (o *AuditExportJob) GetOccurredBeforeOk() (*string, bool)`

GetOccurredBeforeOk returns a tuple with the OccurredBefore field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredBefore

`func (o *AuditExportJob) SetOccurredBefore(v string)`

SetOccurredBefore sets OccurredBefore field to given value.

### HasOccurredBefore

`func (o *AuditExportJob) HasOccurredBefore() bool`

HasOccurredBefore returns a boolean if a field has been set.

### SetOccurredBeforeNil

`func (o *AuditExportJob) SetOccurredBeforeNil(b bool)`

 SetOccurredBeforeNil sets the value for OccurredBefore to be an explicit nil

### UnsetOccurredBefore
`func (o *AuditExportJob) UnsetOccurredBefore()`

UnsetOccurredBefore ensures that no value is present for OccurredBefore, not even an explicit nil
### GetOrgId

`func (o *AuditExportJob) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *AuditExportJob) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *AuditExportJob) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetRowCount

`func (o *AuditExportJob) GetRowCount() int64`

GetRowCount returns the RowCount field if non-nil, zero value otherwise.

### GetRowCountOk

`func (o *AuditExportJob) GetRowCountOk() (*int64, bool)`

GetRowCountOk returns a tuple with the RowCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRowCount

`func (o *AuditExportJob) SetRowCount(v int64)`

SetRowCount sets RowCount field to given value.

### HasRowCount

`func (o *AuditExportJob) HasRowCount() bool`

HasRowCount returns a boolean if a field has been set.

### SetRowCountNil

`func (o *AuditExportJob) SetRowCountNil(b bool)`

 SetRowCountNil sets the value for RowCount to be an explicit nil

### UnsetRowCount
`func (o *AuditExportJob) UnsetRowCount()`

UnsetRowCount ensures that no value is present for RowCount, not even an explicit nil
### GetStatus

`func (o *AuditExportJob) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *AuditExportJob) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *AuditExportJob) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
