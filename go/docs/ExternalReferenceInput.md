# ExternalReferenceInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**RecordId** | **string** | Identifier of the record inside that system (reservation, work order, enrolment). Opaque to OpenApp; up to 128 characters. |
**Revision** | Pointer to **NullableInt64** | Monotonic revision of the external record. When present, OpenApp rejects any later write carrying a lower revision with &#x60;external_revision_stale&#x60;. | [optional]
**Source** | **string** | Slug naming the external system, e.g. &#x60;mews&#x60;, &#x60;opera&#x60;, &#x60;custom-pms&#x60;. Lowercase letters, digits, &#x60;.&#x60;, &#x60;_&#x60;, and &#x60;-&#x60;; up to 64 characters. |

## Methods

### NewExternalReferenceInput

`func NewExternalReferenceInput(recordId string, source string, ) *ExternalReferenceInput`

NewExternalReferenceInput instantiates a new ExternalReferenceInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewExternalReferenceInputWithDefaults

`func NewExternalReferenceInputWithDefaults() *ExternalReferenceInput`

NewExternalReferenceInputWithDefaults instantiates a new ExternalReferenceInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetRecordId

`func (o *ExternalReferenceInput) GetRecordId() string`

GetRecordId returns the RecordId field if non-nil, zero value otherwise.

### GetRecordIdOk

`func (o *ExternalReferenceInput) GetRecordIdOk() (*string, bool)`

GetRecordIdOk returns a tuple with the RecordId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecordId

`func (o *ExternalReferenceInput) SetRecordId(v string)`

SetRecordId sets RecordId field to given value.


### GetRevision

`func (o *ExternalReferenceInput) GetRevision() int64`

GetRevision returns the Revision field if non-nil, zero value otherwise.

### GetRevisionOk

`func (o *ExternalReferenceInput) GetRevisionOk() (*int64, bool)`

GetRevisionOk returns a tuple with the Revision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRevision

`func (o *ExternalReferenceInput) SetRevision(v int64)`

SetRevision sets Revision field to given value.

### HasRevision

`func (o *ExternalReferenceInput) HasRevision() bool`

HasRevision returns a boolean if a field has been set.

### SetRevisionNil

`func (o *ExternalReferenceInput) SetRevisionNil(b bool)`

 SetRevisionNil sets the value for Revision to be an explicit nil

### UnsetRevision
`func (o *ExternalReferenceInput) UnsetRevision()`

UnsetRevision ensures that no value is present for Revision, not even an explicit nil
### GetSource

`func (o *ExternalReferenceInput) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *ExternalReferenceInput) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *ExternalReferenceInput) SetSource(v string)`

SetSource sets Source field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
