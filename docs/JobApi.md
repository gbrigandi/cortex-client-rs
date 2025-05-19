# \JobApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_analyzer_job**](JobApi.md#create_analyzer_job) | **POST** /analyzer/{workerId}/run | Create and run an analyzer job
[**create_responder_job**](JobApi.md#create_responder_job) | **POST** /responder/{workerId}/run | Create and run a responder job
[**delete_job**](JobApi.md#delete_job) | **DELETE** /job/{jobId} | Delete a job
[**find_jobs**](JobApi.md#find_jobs) | **POST** /job/_search | Find/Search jobs
[**get_job_by_id**](JobApi.md#get_job_by_id) | **GET** /job/{jobId} | Get a specific job by ID
[**get_job_report**](JobApi.md#get_job_report) | **GET** /job/{jobId}/report | Get the report for a job
[**get_jobs_status**](JobApi.md#get_jobs_status) | **POST** /job/status | Get the status of multiple jobs
[**list_job_artifacts**](JobApi.md#list_job_artifacts) | **POST** /job/{jobId}/artifacts/_search | List/Search artifacts for a job
[**list_jobs**](JobApi.md#list_jobs) | **GET** /job | List jobs for the user with optional filters
[**wait_job_report**](JobApi.md#wait_job_report) | **GET** /job/{jobId}/waitReport | Wait for and get the report for a job



## create_analyzer_job

> models::Job create_analyzer_job(worker_id, job_create_request)
Create and run an analyzer job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**worker_id** | **String** | ID of the analyzer worker. | [required] |
**job_create_request** | [**JobCreateRequest**](JobCreateRequest.md) |  | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_responder_job

> models::Job create_responder_job(worker_id, job_create_request)
Create and run a responder job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**worker_id** | **String** | ID of the responder worker. | [required] |
**job_create_request** | [**JobCreateRequest**](JobCreateRequest.md) |  | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_job

> delete_job(job_id)
Delete a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_jobs

> models::ListJobs200Response find_jobs(analyzer_find_request)
Find/Search jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**analyzer_find_request** | Option<[**AnalyzerFindRequest**](AnalyzerFindRequest.md)> |  |  |

### Return type

[**models::ListJobs200Response**](listJobs_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_by_id

> models::Job get_job_by_id(job_id)
Get a specific job by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::Job**](Job.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_report

> models::JobReportResponse get_job_report(job_id)
Get the report for a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::JobReportResponse**](JobReportResponse.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_jobs_status

> std::collections::HashMap<String, String> get_jobs_status(job_status_request)
Get the status of multiple jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_status_request** | [**JobStatusRequest**](JobStatusRequest.md) |  | [required] |

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## list_jobs

> models::ListJobs200Response list_jobs(data_type_filter, data_filter, worker_filter, range)
List jobs for the user with optional filters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data_type_filter** | Option<**String**> |  |  |
**data_filter** | Option<**String**> |  |  |
**worker_filter** | Option<**String**> |  |  |
**range** | Option<**String**> |  |  |

### Return type

[**models::ListJobs200Response**](listJobs_200_response.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wait_job_report

> models::JobReportResponse wait_job_report(job_id, at_most)
Wait for and get the report for a job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**at_most** | **String** | Maximum duration to wait (e.g., \"5minutes\", \"30s\"). | [required] |

### Return type

[**models::JobReportResponse**](JobReportResponse.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

