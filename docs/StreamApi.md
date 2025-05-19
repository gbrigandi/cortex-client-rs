# \StreamApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_stream**](StreamApi.md#create_stream) | **POST** /stream | Create a new event stream
[**get_stream_events**](StreamApi.md#get_stream_events) | **GET** /stream/{id} | Get events from a stream
[**get_stream_session_status**](StreamApi.md#get_stream_session_status) | **GET** /stream/status | Get the status of the current session/token for streaming



## create_stream

> String create_stream()
Create a new event stream

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stream_events

> models::StreamMessagesResponse get_stream_events(id)
Get events from a stream

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the stream. | [required] |

### Return type

[**models::StreamMessagesResponse**](StreamMessagesResponse.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stream_session_status

> models::StreamStatusResponse get_stream_session_status()
Get the status of the current session/token for streaming

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::StreamStatusResponse**](StreamStatusResponse.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

