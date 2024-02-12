# default_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**download_infos_v1_download_infos_get**](default_api.md#download_infos_v1_download_infos_get) | **GET** /v1/download_infos | Download Infos
**estimate_f0_v1_estimate_f0_post**](default_api.md#estimate_f0_v1_estimate_f0_post) | **POST** /v1/estimate_f0 | Estimate F0
**estimate_prosody_v1_estimate_prosody_post**](default_api.md#estimate_prosody_v1_estimate_prosody_post) | **POST** /v1/estimate_prosody | Estimate Prosody
**get_engine_info_v1_engine_info_get**](default_api.md#get_engine_info_v1_engine_info_get) | **GET** /v1/engine_info | Get Engine Info
**get_sample_voice_v1_sample_voice_get**](default_api.md#get_sample_voice_v1_sample_voice_get) | **GET** /v1/sample_voice | Get Sample Voice
**get_speaker_policy_v1_speaker_policy_get**](default_api.md#get_speaker_policy_v1_speaker_policy_get) | **GET** /v1/speaker_policy | Get Speaker Policy
**predict_v1_predict_post**](default_api.md#predict_v1_predict_post) | **POST** /v1/predict | Predict
**predict_v1_predict_with_duration_post**](default_api.md#predict_v1_predict_with_duration_post) | **POST** /v1/predict_with_duration | Predict
**process_v1_process_post**](default_api.md#process_v1_process_post) | **POST** /v1/process | Process
**process_v1_process_with_pitch_post**](default_api.md#process_v1_process_with_pitch_post) | **POST** /v1/process_with_pitch | Process
**read_root__get**](default_api.md#read_root__get) | **GET** / | Read Root
**set_dictionary_v1_set_dictionary_post**](default_api.md#set_dictionary_v1_set_dictionary_post) | **POST** /v1/set_dictionary | Set Dictionary
**speaker_folder_path_v1_query2prosody_post**](default_api.md#speaker_folder_path_v1_query2prosody_post) | **POST** /v1/query2prosody | Speaker Folder Path
**speaker_folder_path_v1_speaker_folder_path_get**](default_api.md#speaker_folder_path_v1_speaker_folder_path_get) | **GET** /v1/speaker_folder_path | Speaker Folder Path
**speaker_folder_path_v1_style_id_to_speaker_meta_post**](default_api.md#speaker_folder_path_v1_style_id_to_speaker_meta_post) | **POST** /v1/style_id_to_speaker_meta | Speaker Folder Path
**speakers_v1_speakers_get**](default_api.md#speakers_v1_speakers_get) | **GET** /v1/speakers | Speakers
**synthesis_v1_synthesis_post**](default_api.md#synthesis_v1_synthesis_post) | **POST** /v1/synthesis | Synthesis


# **download_infos_v1_download_infos_get**
> Vec<models::DownloadableModel> download_infos_v1_download_infos_get()
Download Infos

キャラクターダウンロード情報を取得。

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<models::DownloadableModel>**](DownloadableModel.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **estimate_f0_v1_estimate_f0_post**
> models::WorldF0 estimate_f0_v1_estimate_f0_post(wav_with_duration)
Estimate F0

wavの周波数情報を取得。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **wav_with_duration** | [**WavWithDuration**](WavWithDuration.md)|  | 

### Return type

[**models::WorldF0**](WorldF0.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **estimate_prosody_v1_estimate_prosody_post**
> models::Prosody estimate_prosody_v1_estimate_prosody_post(prosody_making_param)
Estimate Prosody

テキストから韻律を取得。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **prosody_making_param** | [**ProsodyMakingParam**](ProsodyMakingParam.md)|  | 

### Return type

[**models::Prosody**](Prosody.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_engine_info_v1_engine_info_get**
> models::EngineInfo get_engine_info_v1_engine_info_get()
Get Engine Info

エンジンの情報を取得する。

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::EngineInfo**](EngineInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_sample_voice_v1_sample_voice_get**
> swagger::ByteArray get_sample_voice_v1_sample_voice_get(optional)
Get Sample Voice

話者のサンプルボイスを取得する。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **speaker_uuid** | **String**|  | 
 **style_id** | **i32**|  | 
 **index** | **i32**|  | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, audio/wav

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_speaker_policy_v1_speaker_policy_get**
> models::SpeakerPolicy get_speaker_policy_v1_speaker_policy_get(optional)
Get Speaker Policy

話者のpolicyファイルを取得する

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **speaker_uuid** | **String**|  | 

### Return type

[**models::SpeakerPolicy**](SpeakerPolicy.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **predict_v1_predict_post**
> swagger::ByteArray predict_v1_predict_post(wav_making_param)
Predict

テキストから音声を予測。機械学習による推論処理なので時間がかかる。prosodyDetailなしでtextだけでも推論可能。textとprosodyDetailではprosodyDetailが優先される。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **wav_making_param** | [**WavMakingParam**](WavMakingParam.md)|  | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, audio/wav

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **predict_v1_predict_with_duration_post**
> models::WavWithDuration predict_v1_predict_with_duration_post(wav_making_param)
Predict

predictと同時にduration情報も得る。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **wav_making_param** | [**WavMakingParam**](WavMakingParam.md)|  | 

### Return type

[**models::WavWithDuration**](WavWithDuration.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **process_v1_process_post**
> swagger::ByteArray process_v1_process_post(wav, optional)
Process

音声を加工。機械学習による推論ではないため処理時間は短い。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **wav** | **swagger::ByteArray**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **wav** | **swagger::ByteArray**|  | 
 **volume_scale** | **f64**|  | [default to 1.0]
 **pitch_scale** | **f64**|  | [default to 0.0]
 **intonation_scale** | **f64**|  | [default to 1.0]
 **pre_phoneme_length** | **f64**|  | [default to 0.0]
 **post_phoneme_length** | **f64**|  | [default to 0.0]
 **output_sampling_rate** | **i32**|  | [default to 44100]

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json, audio/wav

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **process_v1_process_with_pitch_post**
> swagger::ByteArray process_v1_process_with_pitch_post(wav_processing_param)
Process

pitch調整も含めた音声の加工。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **wav_processing_param** | [**WavProcessingParam**](WavProcessingParam.md)|  | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, audio/wav

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_root__get**
> models::Status read_root__get()
Read Root

エンジンの起動状態を取得。

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::Status**](Status.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **set_dictionary_v1_set_dictionary_post**
> serde_json::Value set_dictionary_v1_set_dictionary_post(dictionary_words)
Set Dictionary

辞書の設定。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **dictionary_words** | [**DictionaryWords**](DictionaryWords.md)|  | 

### Return type

[**serde_json::Value**](AnyType.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **speaker_folder_path_v1_query2prosody_post**
> models::Prosody speaker_folder_path_v1_query2prosody_post(audio_query)
Speaker Folder Path

v1のqueryをprosodyに変換。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **audio_query** | [**AudioQuery**](AudioQuery.md)|  | 

### Return type

[**models::Prosody**](Prosody.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **speaker_folder_path_v1_speaker_folder_path_get**
> models::SpeakerFolderPath speaker_folder_path_v1_speaker_folder_path_get(optional)
Speaker Folder Path

キャラクターのフォルダパスを取得。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **speaker_uuid** | **String**|  | 

### Return type

[**models::SpeakerFolderPath**](SpeakerFolderPath.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **speaker_folder_path_v1_style_id_to_speaker_meta_post**
> models::SpeakerMetaForTextBox speaker_folder_path_v1_style_id_to_speaker_meta_post(optional)
Speaker Folder Path

style_idだけでスピーカー情報を取得。重複があった場合は後にロードしたスピーカーが優先される。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **style_id** | **i32**|  | 

### Return type

[**models::SpeakerMetaForTextBox**](SpeakerMetaForTextBox.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **speakers_v1_speakers_get**
> Vec<models::SpeakerMeta> speakers_v1_speakers_get()
Speakers

話者情報の取得。

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<models::SpeakerMeta>**](SpeakerMeta.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **synthesis_v1_synthesis_post**
> swagger::ByteArray synthesis_v1_synthesis_post(synthesis_param)
Synthesis

predict+processの処理。

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **synthesis_param** | [**SynthesisParam**](SynthesisParam.md)|  | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, audio/wav

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

