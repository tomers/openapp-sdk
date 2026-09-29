# ExternalReferenceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CorrelationId** | Pointer to **NullableString** | Correlation id of the request that last wrote this reference. | [optional]
**RecordId** | **NullableString** |  |
**Revision** | Pointer to **NullableInt64** |  | [optional]
**Source** | **NullableString** |  |

## Methods

### NewExternalReferenceResponse

`func NewExternalReferenceResponse(recordId NullableString, source NullableString, ) *ExternalReferenceResponse`

NewExternalReferenceResponse instantiates a new ExternalReferenceResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewExternalReferenceResponseWithDefaults

`func NewExternalReferenceResponseWithDefaults() *ExternalReferenceResponse`

NewExternalReferenceResponseWithDefaults instantiates a new ExternalReferenceResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCorrelationId

`func (o *ExternalReferenceResponse) GetCorrelationId() string`

GetCorrelationId returns the CorrelationId field if non-nil, zero value otherwise.

### GetCorrelationIdOk

`func (o *ExternalReferenceResponse) GetCorrelationIdOk() (*string, bool)`

GetCorrelationIdOk returns a tuple with the CorrelationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCorrelationId

`func (o *ExternalReferenceResponse) SetCorrelationId(v string)`

SetCorrelationId sets CorrelationId field to given value.

### HasCorrelationId

`func (o *ExternalReferenceResponse) HasCorrelationId() bool`

HasCorrelationId returns a boolean if a field has been set.

### SetCorrelationIdNil

`func (o *ExternalReferenceResponse) SetCorrelationIdNil(b bool)`

 SetCorrelationIdNil sets the value for CorrelationId to be an explicit nil

### UnsetCorrelationId
`func (o *ExternalReferenceResponse) UnsetCorrelationId()`

UnsetCorrelationId ensures that no value is present for CorrelationId, not even an explicit nil
### GetRecordId

`func (o *ExternalReferenceResponse) GetRecordId() string`

GetRecordId returns the RecordId field if non-nil, zero value otherwise.

### GetRecordIdOk

`func (o *ExternalReferenceResponse) GetRecordIdOk() (*string, bool)`

GetRecordIdOk returns a tuple with the RecordId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecordId

`func (o *ExternalReferenceResponse) SetRecordId(v string)`

SetRecordId sets RecordId field to given value.


### SetRecordIdNil

`func (o *ExternalReferenceResponse) SetRecordIdNil(b bool)`

 SetRecordIdNil sets the value for RecordId to be an explicit nil

### UnsetRecordId
`func (o *ExternalReferenceResponse) UnsetRecordId()`

UnsetRecordId ensures that no value is present for RecordId, not even an explicit nil
### GetRevision

`func (o *ExternalReferenceResponse) GetRevision() int64`

GetRevision returns the Revision field if non-nil, zero value otherwise.

### GetRevisionOk

`func (o *ExternalReferenceResponse) GetRevisionOk() (*int64, bool)`

GetRevisionOk returns a tuple with the Revision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRevision

`func (o *ExternalReferenceResponse) SetRevision(v int64)`

SetRevision sets Revision field to given value.

### HasRevision

`func (o *ExternalReferenceResponse) HasRevision() bool`

HasRevision returns a boolean if a field has been set.

### SetRevisionNil

`func (o *ExternalReferenceResponse) SetRevisionNil(b bool)`

 SetRevisionNil sets the value for Revision to be an explicit nil

### UnsetRevision
`func (o *ExternalReferenceResponse) UnsetRevision()`

UnsetRevision ensures that no value is present for Revision, not even an explicit nil
### GetSource

`func (o *ExternalReferenceResponse) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *ExternalReferenceResponse) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *ExternalReferenceResponse) SetSource(v string)`

SetSource sets Source field to given value.


### SetSourceNil

`func (o *ExternalReferenceResponse) SetSourceNil(b bool)`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *ExternalReferenceResponse) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
