# QuotaUsage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Current** | **int64** |  |
**Key** | [**QuotaKey**](QuotaKey.md) |  |
**Limit** | [**EffectiveLimit**](EffectiveLimit.md) |  |
**Period** | [**QuotaPeriod**](QuotaPeriod.md) |  |
**PeriodEnd** | Pointer to **NullableString** | Window end (RFC3339). &#x60;null&#x60; for &#x60;Lifetime&#x60;. | [optional]
**PeriodLabel** | **string** | Window label (e.g. &#x60;lifetime&#x60;, &#x60;month:2026-04&#x60;). Stable across reports. |
**PeriodStart** | Pointer to **NullableString** | Window start (RFC3339). &#x60;null&#x60; for &#x60;Lifetime&#x60;. | [optional]

## Methods

### NewQuotaUsage

`func NewQuotaUsage(current int64, key QuotaKey, limit EffectiveLimit, period QuotaPeriod, periodLabel string, ) *QuotaUsage`

NewQuotaUsage instantiates a new QuotaUsage object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQuotaUsageWithDefaults

`func NewQuotaUsageWithDefaults() *QuotaUsage`

NewQuotaUsageWithDefaults instantiates a new QuotaUsage object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCurrent

`func (o *QuotaUsage) GetCurrent() int64`

GetCurrent returns the Current field if non-nil, zero value otherwise.

### GetCurrentOk

`func (o *QuotaUsage) GetCurrentOk() (*int64, bool)`

GetCurrentOk returns a tuple with the Current field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrent

`func (o *QuotaUsage) SetCurrent(v int64)`

SetCurrent sets Current field to given value.


### GetKey

`func (o *QuotaUsage) GetKey() QuotaKey`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *QuotaUsage) GetKeyOk() (*QuotaKey, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *QuotaUsage) SetKey(v QuotaKey)`

SetKey sets Key field to given value.


### GetLimit

`func (o *QuotaUsage) GetLimit() EffectiveLimit`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *QuotaUsage) GetLimitOk() (*EffectiveLimit, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *QuotaUsage) SetLimit(v EffectiveLimit)`

SetLimit sets Limit field to given value.


### GetPeriod

`func (o *QuotaUsage) GetPeriod() QuotaPeriod`

GetPeriod returns the Period field if non-nil, zero value otherwise.

### GetPeriodOk

`func (o *QuotaUsage) GetPeriodOk() (*QuotaPeriod, bool)`

GetPeriodOk returns a tuple with the Period field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeriod

`func (o *QuotaUsage) SetPeriod(v QuotaPeriod)`

SetPeriod sets Period field to given value.


### GetPeriodEnd

`func (o *QuotaUsage) GetPeriodEnd() string`

GetPeriodEnd returns the PeriodEnd field if non-nil, zero value otherwise.

### GetPeriodEndOk

`func (o *QuotaUsage) GetPeriodEndOk() (*string, bool)`

GetPeriodEndOk returns a tuple with the PeriodEnd field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeriodEnd

`func (o *QuotaUsage) SetPeriodEnd(v string)`

SetPeriodEnd sets PeriodEnd field to given value.

### HasPeriodEnd

`func (o *QuotaUsage) HasPeriodEnd() bool`

HasPeriodEnd returns a boolean if a field has been set.

### SetPeriodEndNil

`func (o *QuotaUsage) SetPeriodEndNil(b bool)`

 SetPeriodEndNil sets the value for PeriodEnd to be an explicit nil

### UnsetPeriodEnd
`func (o *QuotaUsage) UnsetPeriodEnd()`

UnsetPeriodEnd ensures that no value is present for PeriodEnd, not even an explicit nil
### GetPeriodLabel

`func (o *QuotaUsage) GetPeriodLabel() string`

GetPeriodLabel returns the PeriodLabel field if non-nil, zero value otherwise.

### GetPeriodLabelOk

`func (o *QuotaUsage) GetPeriodLabelOk() (*string, bool)`

GetPeriodLabelOk returns a tuple with the PeriodLabel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeriodLabel

`func (o *QuotaUsage) SetPeriodLabel(v string)`

SetPeriodLabel sets PeriodLabel field to given value.


### GetPeriodStart

`func (o *QuotaUsage) GetPeriodStart() string`

GetPeriodStart returns the PeriodStart field if non-nil, zero value otherwise.

### GetPeriodStartOk

`func (o *QuotaUsage) GetPeriodStartOk() (*string, bool)`

GetPeriodStartOk returns a tuple with the PeriodStart field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeriodStart

`func (o *QuotaUsage) SetPeriodStart(v string)`

SetPeriodStart sets PeriodStart field to given value.

### HasPeriodStart

`func (o *QuotaUsage) HasPeriodStart() bool`

HasPeriodStart returns a boolean if a field has been set.

### SetPeriodStartNil

`func (o *QuotaUsage) SetPeriodStartNil(b bool)`

 SetPeriodStartNil sets the value for PeriodStart to be an explicit nil

### UnsetPeriodStart
`func (o *QuotaUsage) UnsetPeriodStart()`

UnsetPeriodStart ensures that no value is present for PeriodStart, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
