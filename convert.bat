@echo off

@REM This script converts a text file dropped on it to a WAV file using coeiroink2-txt2wav

set CONVERTER="%~dp0target\release\coeiroink2-txt2wav.exe"
echo Input file: %1
echo Output file: %1.wav

%CONVERTER% predict -i %1 -o %1.wav

pause
