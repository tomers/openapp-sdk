# ResolveResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Ambiguous** | **bool** |  |
**Candidates** | [**[]ResolveCandidate**](ResolveCandidate.md) |  |
**Query** | **string** |  |

## Methods

### NewResolveResponse

`func NewResolveResponse(ambiguous bool, candidates []ResolveCandidate, query string, ) *ResolveResponse`

NewResolveResponse instantiates a new ResolveResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResolveResponseWithDefaults

`func NewResolveResponseWithDefaults() *ResolveResponse`

NewResolveResponseWithDefaults instantiates a new ResolveResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAmbiguous

`func (o *ResolveResponse) GetAmbiguous() bool`

GetAmbiguous returns the Ambiguous field if non-nil, zero value otherwise.

### GetAmbiguousOk

`func (o *ResolveResponse) GetAmbiguousOk() (*bool, bool)`

GetAmbiguousOk returns a tuple with the Ambiguous field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAmbiguous

`func (o *ResolveResponse) SetAmbiguous(v bool)`

SetAmbiguous sets Ambiguous field to given value.


### GetCandidates

`func (o *ResolveResponse) GetCandidates() []ResolveCandidate`

GetCandidates returns the Candidates field if non-nil, zero value otherwise.

### GetCandidatesOk

`func (o *ResolveResponse) GetCandidatesOk() (*[]ResolveCandidate, bool)`

GetCandidatesOk returns a tuple with the Candidates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCandidates

`func (o *ResolveResponse) SetCandidates(v []ResolveCandidate)`

SetCandidates sets Candidates field to given value.


### GetQuery

`func (o *ResolveResponse) GetQuery() string`

GetQuery returns the Query field if non-nil, zero value otherwise.

### GetQueryOk

`func (o *ResolveResponse) GetQueryOk() (*string, bool)`

GetQueryOk returns a tuple with the Query field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuery

`func (o *ResolveResponse) SetQuery(v string)`

SetQuery sets Query field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
