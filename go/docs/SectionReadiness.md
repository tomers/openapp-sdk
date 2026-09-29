# SectionReadiness

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BlockingCount** | **int32** | Issues whose subject cannot be used, including site-scoped gaps. |
**IncompleteCount** | **int32** |  |
**Issues** | [**[]ReadinessIssue**](ReadinessIssue.md) |  |
**Section** | [**ReadinessSection**](ReadinessSection.md) |  |
**Severity** | [**ReadinessSeverity**](ReadinessSeverity.md) | Aggregate severity of the section, not the worst issue severity: see the module docs for how subject-scoped issues degrade a section to [&#x60;ReadinessSeverity::Partial&#x60;]. |

## Methods

### NewSectionReadiness

`func NewSectionReadiness(blockingCount int32, incompleteCount int32, issues []ReadinessIssue, section ReadinessSection, severity ReadinessSeverity, ) *SectionReadiness`

NewSectionReadiness instantiates a new SectionReadiness object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSectionReadinessWithDefaults

`func NewSectionReadinessWithDefaults() *SectionReadiness`

NewSectionReadinessWithDefaults instantiates a new SectionReadiness object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBlockingCount

`func (o *SectionReadiness) GetBlockingCount() int32`

GetBlockingCount returns the BlockingCount field if non-nil, zero value otherwise.

### GetBlockingCountOk

`func (o *SectionReadiness) GetBlockingCountOk() (*int32, bool)`

GetBlockingCountOk returns a tuple with the BlockingCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBlockingCount

`func (o *SectionReadiness) SetBlockingCount(v int32)`

SetBlockingCount sets BlockingCount field to given value.


### GetIncompleteCount

`func (o *SectionReadiness) GetIncompleteCount() int32`

GetIncompleteCount returns the IncompleteCount field if non-nil, zero value otherwise.

### GetIncompleteCountOk

`func (o *SectionReadiness) GetIncompleteCountOk() (*int32, bool)`

GetIncompleteCountOk returns a tuple with the IncompleteCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncompleteCount

`func (o *SectionReadiness) SetIncompleteCount(v int32)`

SetIncompleteCount sets IncompleteCount field to given value.


### GetIssues

`func (o *SectionReadiness) GetIssues() []ReadinessIssue`

GetIssues returns the Issues field if non-nil, zero value otherwise.

### GetIssuesOk

`func (o *SectionReadiness) GetIssuesOk() (*[]ReadinessIssue, bool)`

GetIssuesOk returns a tuple with the Issues field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIssues

`func (o *SectionReadiness) SetIssues(v []ReadinessIssue)`

SetIssues sets Issues field to given value.


### GetSection

`func (o *SectionReadiness) GetSection() ReadinessSection`

GetSection returns the Section field if non-nil, zero value otherwise.

### GetSectionOk

`func (o *SectionReadiness) GetSectionOk() (*ReadinessSection, bool)`

GetSectionOk returns a tuple with the Section field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSection

`func (o *SectionReadiness) SetSection(v ReadinessSection)`

SetSection sets Section field to given value.


### GetSeverity

`func (o *SectionReadiness) GetSeverity() ReadinessSeverity`

GetSeverity returns the Severity field if non-nil, zero value otherwise.

### GetSeverityOk

`func (o *SectionReadiness) GetSeverityOk() (*ReadinessSeverity, bool)`

GetSeverityOk returns a tuple with the Severity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSeverity

`func (o *SectionReadiness) SetSeverity(v ReadinessSeverity)`

SetSeverity sets Severity field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
