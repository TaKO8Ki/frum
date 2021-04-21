@echo off
cd %1
if exist .ruby-version (
    frum local
)
@echo on
