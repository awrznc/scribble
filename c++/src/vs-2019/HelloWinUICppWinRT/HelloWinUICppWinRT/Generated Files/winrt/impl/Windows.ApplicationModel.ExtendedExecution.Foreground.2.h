// WARNING: Please don't edit this file. It was generated by C++/WinRT v2.0.200703.9

#ifndef WINRT_Windows_ApplicationModel_ExtendedExecution_Foreground_2_H
#define WINRT_Windows_ApplicationModel_ExtendedExecution_Foreground_2_H
#include "winrt/impl/Windows.Foundation.1.h"
#include "winrt/impl/Windows.ApplicationModel.ExtendedExecution.Foreground.1.h"
WINRT_EXPORT namespace winrt::Windows::ApplicationModel::ExtendedExecution::Foreground
{
    struct __declspec(empty_bases) ExtendedExecutionForegroundRevokedEventArgs : Windows::ApplicationModel::ExtendedExecution::Foreground::IExtendedExecutionForegroundRevokedEventArgs
    {
        ExtendedExecutionForegroundRevokedEventArgs(std::nullptr_t) noexcept {}
        ExtendedExecutionForegroundRevokedEventArgs(void* ptr, take_ownership_from_abi_t) noexcept : Windows::ApplicationModel::ExtendedExecution::Foreground::IExtendedExecutionForegroundRevokedEventArgs(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) ExtendedExecutionForegroundSession : Windows::ApplicationModel::ExtendedExecution::Foreground::IExtendedExecutionForegroundSession
    {
        ExtendedExecutionForegroundSession(std::nullptr_t) noexcept {}
        ExtendedExecutionForegroundSession(void* ptr, take_ownership_from_abi_t) noexcept : Windows::ApplicationModel::ExtendedExecution::Foreground::IExtendedExecutionForegroundSession(ptr, take_ownership_from_abi) {}
        ExtendedExecutionForegroundSession();
    };
}
#endif
