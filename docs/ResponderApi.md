# \ResponderApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_responder_from_definition**](ResponderApi.md#create_responder_from_definition) | **POST** /responder/definition/{responderDefinitionId} | Create a new responder instance from a definition
[**delete_responder**](ResponderApi.md#delete_responder) | **DELETE** /responder/{responderId} | Delete a responder
[**find_responders**](ResponderApi.md#find_responders) | **POST** /responder/_search | Find/Search responders
[**get_responder_by_id**](ResponderApi.md#get_responder_by_id) | **GET** /responder/{responderId} | Get a specific responder by ID
[**list_responder_definitions**](ResponderApi.md#list_responder_definitions) | **GET** /responder/definition | List all available responder definitions
[**list_responders_for_type**](ResponderApi.md#list_responders_for_type) | **GET** /responder/type/{dataType} | List responders available for a specific data type
[**scan_responder_definitions**](ResponderApi.md#scan_responder_definitions) | **POST** /responder/scan | Trigger a rescan of responder definitions
[**update_responder**](ResponderApi.md#update_responder) | **PUT** /responder/{responderId} | Update a responder



## create_responder_from_definition

> models::Worker create_responder_from_definition(responder_definition_id, responder_create_request)
Create a new responder instance from a definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**responder_definition_id** | **String** | The ID of the responder definition. | [required] |
**responder_create_request** | [**ResponderCreateRequest**](ResponderCreateRequest.md) |  | [required] |

### Return type

[**models::Worker**](Worker.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_responder

> delete_responder(responder_id)
Delete a responder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**responder_id** | **String** | The ID of the responder. | [required] |

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_responders

> models::ListAnalyzersForType200Response find_responders(responder_find_request)
Find/Search responders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**responder_find_request** | Option<[**ResponderFindRequest**](ResponderFindRequest.md)> |  |  |

### Return type

[**models::ListAnalyzersForType200Response**](listAnalyzersForType_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_responder_by_id

> models::Worker get_responder_by_id(responder_id)
Get a specific responder by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**responder_id** | **String** | The ID of the responder. | [required] |

### Return type

[**models::Worker**](Worker.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_responder_definitions

> models::ListAnalyzerDefinitions200Response list_responder_definitions()
List all available responder definitions

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


## list_responders_for_type

> models::ListAnalyzersForType200Response list_responders_for_type(data_type)
List responders available for a specific data type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_type** | **String** | The data type to filter responders by. | [required] |

### Return type

[**models::ListAnalyzersForType200Response**](listAnalyzersForType_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scan_responder_definitions

> scan_responder_definitions()
Trigger a rescan of responder definitions

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


## update_responder

> models::Worker update_responder(responder_id, request_body)
Update a responder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**responder_id** | **String** | The ID of the responder. | [required] |
**request_body** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [required] |

### Return type

[**models::Worker**](Worker.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

