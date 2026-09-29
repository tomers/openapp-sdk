# SiteReadiness

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Sections** | [**[]SectionReadiness**](SectionReadiness.md) |  |
**Severity** | [**ReadinessSeverity**](ReadinessSeverity.md) |  |
**SuppressedSections** | [**[]ReadinessSection**](ReadinessSection.md) |  |
**Usable** | **bool** |  |

## Methods

### NewSiteReadiness

`func NewSiteReadiness(sections []SectionReadiness, severity ReadinessSeverity, suppressedSections []ReadinessSection, usable bool, ) *SiteReadiness`

NewSiteReadiness instantiates a new SiteReadiness object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteReadinessWithDefaults

`func NewSiteReadinessWithDefaults() *SiteReadiness`

NewSiteReadinessWithDefaults instantiates a new SiteReadiness object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSections

`func (o *SiteReadiness) GetSections() []SectionReadiness`

GetSections returns the Sections field if non-nil, zero value otherwise.

### GetSectionsOk

`func (o *SiteReadiness) GetSectionsOk() (*[]SectionReadiness, bool)`

GetSectionsOk returns a tuple with the Sections field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSections

`func (o *SiteReadiness) SetSections(v []SectionReadiness)`

SetSections sets Sections field to given value.


### GetSeverity

`func (o *SiteReadiness) GetSeverity() ReadinessSeverity`

GetSeverity returns the Severity field if non-nil, zero value otherwise.

### GetSeverityOk

`func (o *SiteReadiness) GetSeverityOk() (*ReadinessSeverity, bool)`

GetSeverityOk returns a tuple with the Severity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSeverity

`func (o *SiteReadiness) SetSeverity(v ReadinessSeverity)`

SetSeverity sets Severity field to given value.


### GetSuppressedSections

`func (o *SiteReadiness) GetSuppressedSections() []ReadinessSection`

GetSuppressedSections returns the SuppressedSections field if non-nil, zero value otherwise.

### GetSuppressedSectionsOk

`func (o *SiteReadiness) GetSuppressedSectionsOk() (*[]ReadinessSection, bool)`

GetSuppressedSectionsOk returns a tuple with the SuppressedSections field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSuppressedSections

`func (o *SiteReadiness) SetSuppressedSections(v []ReadinessSection)`

SetSuppressedSections sets SuppressedSections field to given value.


### GetUsable

`func (o *SiteReadiness) GetUsable() bool`

GetUsable returns the Usable field if non-nil, zero value otherwise.

### GetUsableOk

`func (o *SiteReadiness) GetUsableOk() (*bool, bool)`

GetUsableOk returns a tuple with the Usable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsable

`func (o *SiteReadiness) SetUsable(v bool)`

SetUsable sets Usable field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
