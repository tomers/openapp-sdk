# KeyOverage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Current** | **int64** |  |
**Excess** | **int64** |  |
**Key** | [**QuotaKey**](QuotaKey.md) |  |
**Limit** | **int64** |  |
**Unit** | [**QuotaUnit**](QuotaUnit.md) |  |

## Methods

### NewKeyOverage

`func NewKeyOverage(current int64, excess int64, key QuotaKey, limit int64, unit QuotaUnit, ) *KeyOverage`

NewKeyOverage instantiates a new KeyOverage object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewKeyOverageWithDefaults

`func NewKeyOverageWithDefaults() *KeyOverage`

NewKeyOverageWithDefaults instantiates a new KeyOverage object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCurrent

`func (o *KeyOverage) GetCurrent() int64`

GetCurrent returns the Current field if non-nil, zero value otherwise.

### GetCurrentOk

`func (o *KeyOverage) GetCurrentOk() (*int64, bool)`

GetCurrentOk returns a tuple with the Current field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrent

`func (o *KeyOverage) SetCurrent(v int64)`

SetCurrent sets Current field to given value.


### GetExcess

`func (o *KeyOverage) GetExcess() int64`

GetExcess returns the Excess field if non-nil, zero value otherwise.

### GetExcessOk

`func (o *KeyOverage) GetExcessOk() (*int64, bool)`

GetExcessOk returns a tuple with the Excess field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExcess

`func (o *KeyOverage) SetExcess(v int64)`

SetExcess sets Excess field to given value.


### GetKey

`func (o *KeyOverage) GetKey() QuotaKey`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *KeyOverage) GetKeyOk() (*QuotaKey, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *KeyOverage) SetKey(v QuotaKey)`

SetKey sets Key field to given value.


### GetLimit

`func (o *KeyOverage) GetLimit() int64`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *KeyOverage) GetLimitOk() (*int64, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *KeyOverage) SetLimit(v int64)`

SetLimit sets Limit field to given value.


### GetUnit

`func (o *KeyOverage) GetUnit() QuotaUnit`

GetUnit returns the Unit field if non-nil, zero value otherwise.

### GetUnitOk

`func (o *KeyOverage) GetUnitOk() (*QuotaUnit, bool)`

GetUnitOk returns a tuple with the Unit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnit

`func (o *KeyOverage) SetUnit(v QuotaUnit)`

SetUnit sets Unit field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
