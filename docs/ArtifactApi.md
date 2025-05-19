# \ArtifactApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_job_artifacts**](ArtifactApi.md#list_job_artifacts) | **POST** /job/{jobId}/artifacts/_search | List/Search artifacts for a job



## list_job_artifacts

> models::ListJobArtifacts200Response list_job_artifacts(job_id, analyzer_find_request)
List/Search artifacts for a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**analyzer_find_request** | Option<[**AnalyzerFindRequest**](AnalyzerFindRequest.md)> |  |  |

### Return type

[**models::ListJobArtifacts200Response**](listJobArtifacts_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

