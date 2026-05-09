# PlanQuotaResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**LimitValue** | Pointer to **NullableInt64** |  | [optional]
**Period** | **string** |  |
**QuotaKey** | **string** |  |

## Methods

### NewPlanQuotaResponse

`func NewPlanQuotaResponse(period string, quotaKey string, ) *PlanQuotaResponse`

NewPlanQuotaResponse instantiates a new PlanQuotaResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPlanQuotaResponseWithDefaults

`func NewPlanQuotaResponseWithDefaults() *PlanQuotaResponse`

NewPlanQuotaResponseWithDefaults instantiates a new PlanQuotaResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLimitValue

`func (o *PlanQuotaResponse) GetLimitValue() int64`

GetLimitValue returns the LimitValue field if non-nil, zero value otherwise.

### GetLimitValueOk

`func (o *PlanQuotaResponse) GetLimitValueOk() (*int64, bool)`

GetLimitValueOk returns a tuple with the LimitValue field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimitValue

`func (o *PlanQuotaResponse) SetLimitValue(v int64)`

SetLimitValue sets LimitValue field to given value.

### HasLimitValue

`func (o *PlanQuotaResponse) HasLimitValue() bool`

HasLimitValue returns a boolean if a field has been set.

### SetLimitValueNil

`func (o *PlanQuotaResponse) SetLimitValueNil(b bool)`

 SetLimitValueNil sets the value for LimitValue to be an explicit nil

### UnsetLimitValue
`func (o *PlanQuotaResponse) UnsetLimitValue()`

UnsetLimitValue ensures that no value is present for LimitValue, not even an explicit nil
### GetPeriod

`func (o *PlanQuotaResponse) GetPeriod() string`

GetPeriod returns the Period field if non-nil, zero value otherwise.

### GetPeriodOk

`func (o *PlanQuotaResponse) GetPeriodOk() (*string, bool)`

GetPeriodOk returns a tuple with the Period field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeriod

`func (o *PlanQuotaResponse) SetPeriod(v string)`

SetPeriod sets Period field to given value.


### GetQuotaKey

`func (o *PlanQuotaResponse) GetQuotaKey() string`

GetQuotaKey returns the QuotaKey field if non-nil, zero value otherwise.

### GetQuotaKeyOk

`func (o *PlanQuotaResponse) GetQuotaKeyOk() (*string, bool)`

GetQuotaKeyOk returns a tuple with the QuotaKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuotaKey

`func (o *PlanQuotaResponse) SetQuotaKey(v string)`

SetQuotaKey sets QuotaKey field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
