# \AnalyzerApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_analyzer_from_definition**](AnalyzerApi.md#create_analyzer_from_definition) | **POST** /analyzer/definition/{analyzerDefinitionId} | Create a new analyzer instance from a definition
[**delete_analyzer**](AnalyzerApi.md#delete_analyzer) | **DELETE** /analyzer/{analyzerId} | Delete an analyzer
[**find_analyzers**](AnalyzerApi.md#find_analyzers) | **POST** /analyzer/_search | Find/Search analyzers
[**get_analyzer_by_id**](AnalyzerApi.md#get_analyzer_by_id) | **GET** /analyzer/{analyzerId} | Get a specific analyzer by ID
[**list_analyzer_definitions**](AnalyzerApi.md#list_analyzer_definitions) | **GET** /analyzer/definition | List all available analyzer definitions
[**list_analyzers_for_type**](AnalyzerApi.md#list_analyzers_for_type) | **GET** /analyzer/type/{dataType} | List analyzers available for a specific data type
[**scan_analyzer_definitions**](AnalyzerApi.md#scan_analyzer_definitions) | **POST** /analyzer/scan | Trigger a rescan of analyzer definitions
[**update_analyzer**](AnalyzerApi.md#update_analyzer) | **PUT** /analyzer/{analyzerId} | Update an analyzer



## create_analyzer_from_definition

> models::Worker create_analyzer_from_definition(analyzer_definition_id, analyzer_create_request)
Create a new analyzer instance from a definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer_definition_id** | **String** | The ID of the analyzer definition to use for creation. | [required] |
**analyzer_create_request** | [**AnalyzerCreateRequest**](AnalyzerCreateRequest.md) |  | [required] |

### Return type

[**models::Worker**](Worker.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_analyzer

> delete_analyzer(analyzer_id)
Delete an analyzer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer_id** | **String** | The ID of the analyzer | [required] |

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_analyzers

> Vec<models::Worker> find_analyzers(analyzer_find_request)
Find/Search analyzers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer_find_request** | Option<[**AnalyzerFindRequest**](AnalyzerFindRequest.md)> |  |  |

### Return type

[**Vec<models::Worker>**](Worker.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analyzer_by_id

> models::Worker get_analyzer_by_id(analyzer_id)
Get a specific analyzer by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer_id** | **String** | The ID of the analyzer | [required] |

### Return type

[**models::Worker**](Worker.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_analyzer_definitions

> models::ListAnalyzerDefinitions200Response list_analyzer_definitions()
List all available analyzer definitions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListAnalyzerDefinitions200Response**](listAnalyzerDefinitions_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_analyzers_for_type

> models::ListAnalyzersForType200Response list_analyzers_for_type(data_type)
List analyzers available for a specific data type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_type** | **String** | The data type to filter analyzers by (e.g., ip, domain, file). | [required] |

### Return type

[**models::ListAnalyzersForType200Response**](listAnalyzersForType_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scan_analyzer_definitions

> scan_analyzer_definitions()
Trigger a rescan of analyzer definitions

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_analyzer

> models::Worker update_analyzer(analyzer_id, request_body)
Update an analyzer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer_id** | **String** | The ID of the analyzer | [required] |
**request_body** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [required] |

### Return type

[**models::Worker**](Worker.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

