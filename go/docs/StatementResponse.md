# StatementResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Currency** | **string** |  |
**Items** | [**[]StorePurchase**](StorePurchase.md) |  |
**Month** | **string** |  |
**PurchaseCount** | **int64** |  |
**TotalCents** | **int64** |  |

## Methods

### NewStatementResponse

`func NewStatementResponse(currency string, items []StorePurchase, month string, purchaseCount int64, totalCents int64, ) *StatementResponse`

NewStatementResponse instantiates a new StatementResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewStatementResponseWithDefaults

`func NewStatementResponseWithDefaults() *StatementResponse`

NewStatementResponseWithDefaults instantiates a new StatementResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCurrency

`func (o *StatementResponse) GetCurrency() string`

GetCurrency returns the Currency field if non-nil, zero value otherwise.

### GetCurrencyOk

`func (o *StatementResponse) GetCurrencyOk() (*string, bool)`

GetCurrencyOk returns a tuple with the Currency field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCurrency

`func (o *StatementResponse) SetCurrency(v string)`

SetCurrency sets Currency field to given value.


### GetItems

`func (o *StatementResponse) GetItems() []StorePurchase`

GetItems returns the Items field if non-nil, zero value otherwise.

### GetItemsOk

`func (o *StatementResponse) GetItemsOk() (*[]StorePurchase, bool)`

GetItemsOk returns a tuple with the Items field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetItems

`func (o *StatementResponse) SetItems(v []StorePurchase)`

SetItems sets Items field to given value.


### GetMonth

`func (o *StatementResponse) GetMonth() string`

GetMonth returns the Month field if non-nil, zero value otherwise.

### GetMonthOk

`func (o *StatementResponse) GetMonthOk() (*string, bool)`

GetMonthOk returns a tuple with the Month field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMonth

`func (o *StatementResponse) SetMonth(v string)`

SetMonth sets Month field to given value.


### GetPurchaseCount

`func (o *StatementResponse) GetPurchaseCount() int64`

GetPurchaseCount returns the PurchaseCount field if non-nil, zero value otherwise.

### GetPurchaseCountOk

`func (o *StatementResponse) GetPurchaseCountOk() (*int64, bool)`

GetPurchaseCountOk returns a tuple with the PurchaseCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurchaseCount

`func (o *StatementResponse) SetPurchaseCount(v int64)`

SetPurchaseCount sets PurchaseCount field to given value.


### GetTotalCents

`func (o *StatementResponse) GetTotalCents() int64`

GetTotalCents returns the TotalCents field if non-nil, zero value otherwise.

### GetTotalCentsOk

`func (o *StatementResponse) GetTotalCentsOk() (*int64, bool)`

GetTotalCentsOk returns a tuple with the TotalCents field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalCents

`func (o *StatementResponse) SetTotalCents(v int64)`

SetTotalCents sets TotalCents field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
