@echo off
cd %1
if exist .ruby-version (
    farm local
)
@echo on
