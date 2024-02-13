# coeiroink2-txt2wav

This is a cli tool to convert long text file to wav file using COEIROINK.

## Usage

You need to run this tool on powershell or cmd.
Before using this tool, you need to launch the COEIROINK.

```powershell
PS> coeiroink2-txt2wav.exe list
つくよみちゃん: 3c37646f-3881-5374-2a83-149267990abc
        れいせい: 0
PS> coeiroink2-txt2wav.exe predict -i input.txt -o output.wav -u "3c37646f-3881-5374-2a83-149267990abc" -s 0
```

## OpenAPI

This cli tool uses
[openapi-generator-cli](https://github.com/OpenAPITools/openapi-generator)
to generate a rust client for COEIROINK.

```
$ openapi-generator-cli generate -g rust-server -i openapi.json -o coeiroink2
```
