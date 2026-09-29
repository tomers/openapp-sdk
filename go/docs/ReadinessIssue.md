# ReadinessIssue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Code** | [**ReadinessIssueCode**](ReadinessIssueCode.md) |  |
**Severity** | [**ReadinessSeverity**](ReadinessSeverity.md) |  |
**Subject** | Pointer to [**NullableReadinessSubject**](ReadinessSubject.md) |  | [optional]

## Methods

### NewReadinessIssue

`func NewReadinessIssue(code ReadinessIssueCode, severity ReadinessSeverity, ) *ReadinessIssue`

NewReadinessIssue instantiates a new ReadinessIssue object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewReadinessIssueWithDefaults

`func NewReadinessIssueWithDefaults() *ReadinessIssue`

NewReadinessIssueWithDefaults instantiates a new ReadinessIssue object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCode

`func (o *ReadinessIssue) GetCode() ReadinessIssueCode`

GetCode returns the Code field if non-nil, zero value otherwise.

### GetCodeOk

`func (o *ReadinessIssue) GetCodeOk() (*ReadinessIssueCode, bool)`

GetCodeOk returns a tuple with the Code field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCode

`func (o *ReadinessIssue) SetCode(v ReadinessIssueCode)`

SetCode sets Code field to given value.


### GetSeverity

`func (o *ReadinessIssue) GetSeverity() ReadinessSeverity`

GetSeverity returns the Severity field if non-nil, zero value otherwise.

### GetSeverityOk

`func (o *ReadinessIssue) GetSeverityOk() (*ReadinessSeverity, bool)`

GetSeverityOk returns a tuple with the Severity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSeverity

`func (o *ReadinessIssue) SetSeverity(v ReadinessSeverity)`

SetSeverity sets Severity field to given value.


### GetSubject

`func (o *ReadinessIssue) GetSubject() ReadinessSubject`

GetSubject returns the Subject field if non-nil, zero value otherwise.

### GetSubjectOk

`func (o *ReadinessIssue) GetSubjectOk() (*ReadinessSubject, bool)`

GetSubjectOk returns a tuple with the Subject field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubject

`func (o *ReadinessIssue) SetSubject(v ReadinessSubject)`

SetSubject sets Subject field to given value.

### HasSubject

`func (o *ReadinessIssue) HasSubject() bool`

HasSubject returns a boolean if a field has been set.

### SetSubjectNil

`func (o *ReadinessIssue) SetSubjectNil(b bool)`

 SetSubjectNil sets the value for Subject to be an explicit nil

### UnsetSubject
`func (o *ReadinessIssue) UnsetSubject()`

UnsetSubject ensures that no value is present for Subject, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
