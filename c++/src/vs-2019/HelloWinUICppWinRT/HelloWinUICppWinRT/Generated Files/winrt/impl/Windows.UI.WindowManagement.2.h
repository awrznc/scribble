// WARNING: Please don't edit this file. It was generated by C++/WinRT v2.0.200703.9

#ifndef WINRT_Windows_UI_WindowManagement_2_H
#define WINRT_Windows_UI_WindowManagement_2_H
#include "winrt/impl/Windows.UI.WindowManagement.1.h"
WINRT_EXPORT namespace winrt::Windows::UI::WindowManagement
{
    struct __declspec(empty_bases) AppWindow : Windows::UI::WindowManagement::IAppWindow
    {
        AppWindow(std::nullptr_t) noexcept {}
        AppWindow(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindow(ptr, take_ownership_from_abi) {}
        static auto TryCreateAsync();
        static auto ClearAllPersistedState();
        static auto ClearPersistedState(param::hstring const& key);
    };
    struct __declspec(empty_bases) AppWindowChangedEventArgs : Windows::UI::WindowManagement::IAppWindowChangedEventArgs
    {
        AppWindowChangedEventArgs(std::nullptr_t) noexcept {}
        AppWindowChangedEventArgs(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindowChangedEventArgs(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) AppWindowCloseRequestedEventArgs : Windows::UI::WindowManagement::IAppWindowCloseRequestedEventArgs
    {
        AppWindowCloseRequestedEventArgs(std::nullptr_t) noexcept {}
        AppWindowCloseRequestedEventArgs(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindowCloseRequestedEventArgs(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) AppWindowClosedEventArgs : Windows::UI::WindowManagement::IAppWindowClosedEventArgs
    {
        AppWindowClosedEventArgs(std::nullptr_t) noexcept {}
        AppWindowClosedEventArgs(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindowClosedEventArgs(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) AppWindowFrame : Windows::UI::WindowManagement::IAppWindowFrame,
        impl::require<AppWindowFrame, Windows::UI::WindowManagement::IAppWindowFrameStyle>
    {
        AppWindowFrame(std::nullptr_t) noexcept {}
        AppWindowFrame(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindowFrame(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) AppWindowPlacement : Windows::UI::WindowManagement::IAppWindowPlacement
    {
        AppWindowPlacement(std::nullptr_t) noexcept {}
        AppWindowPlacement(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindowPlacement(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) AppWindowPresentationConfiguration : Windows::UI::WindowManagement::IAppWindowPresentationConfiguration
    {
        AppWindowPresentationConfiguration(std::nullptr_t) noexcept {}
        AppWindowPresentationConfiguration(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindowPresentationConfiguration(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) AppWindowPresenter : Windows::UI::WindowManagement::IAppWindowPresenter
    {
        AppWindowPresenter(std::nullptr_t) noexcept {}
        AppWindowPresenter(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindowPresenter(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) AppWindowTitleBar : Windows::UI::WindowManagement::IAppWindowTitleBar,
        impl::require<AppWindowTitleBar, Windows::UI::WindowManagement::IAppWindowTitleBarVisibility>
    {
        AppWindowTitleBar(std::nullptr_t) noexcept {}
        AppWindowTitleBar(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindowTitleBar(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) AppWindowTitleBarOcclusion : Windows::UI::WindowManagement::IAppWindowTitleBarOcclusion
    {
        AppWindowTitleBarOcclusion(std::nullptr_t) noexcept {}
        AppWindowTitleBarOcclusion(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IAppWindowTitleBarOcclusion(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) CompactOverlayPresentationConfiguration : Windows::UI::WindowManagement::ICompactOverlayPresentationConfiguration,
        impl::base<CompactOverlayPresentationConfiguration, Windows::UI::WindowManagement::AppWindowPresentationConfiguration>,
        impl::require<CompactOverlayPresentationConfiguration, Windows::UI::WindowManagement::IAppWindowPresentationConfiguration>
    {
        CompactOverlayPresentationConfiguration(std::nullptr_t) noexcept {}
        CompactOverlayPresentationConfiguration(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::ICompactOverlayPresentationConfiguration(ptr, take_ownership_from_abi) {}
        CompactOverlayPresentationConfiguration();
    };
    struct __declspec(empty_bases) DefaultPresentationConfiguration : Windows::UI::WindowManagement::IDefaultPresentationConfiguration,
        impl::base<DefaultPresentationConfiguration, Windows::UI::WindowManagement::AppWindowPresentationConfiguration>,
        impl::require<DefaultPresentationConfiguration, Windows::UI::WindowManagement::IAppWindowPresentationConfiguration>
    {
        DefaultPresentationConfiguration(std::nullptr_t) noexcept {}
        DefaultPresentationConfiguration(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IDefaultPresentationConfiguration(ptr, take_ownership_from_abi) {}
        DefaultPresentationConfiguration();
    };
    struct __declspec(empty_bases) DisplayRegion : Windows::UI::WindowManagement::IDisplayRegion
    {
        DisplayRegion(std::nullptr_t) noexcept {}
        DisplayRegion(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IDisplayRegion(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) FullScreenPresentationConfiguration : Windows::UI::WindowManagement::IFullScreenPresentationConfiguration,
        impl::base<FullScreenPresentationConfiguration, Windows::UI::WindowManagement::AppWindowPresentationConfiguration>,
        impl::require<FullScreenPresentationConfiguration, Windows::UI::WindowManagement::IAppWindowPresentationConfiguration>
    {
        FullScreenPresentationConfiguration(std::nullptr_t) noexcept {}
        FullScreenPresentationConfiguration(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IFullScreenPresentationConfiguration(ptr, take_ownership_from_abi) {}
        FullScreenPresentationConfiguration();
    };
    struct __declspec(empty_bases) WindowingEnvironment : Windows::UI::WindowManagement::IWindowingEnvironment
    {
        WindowingEnvironment(std::nullptr_t) noexcept {}
        WindowingEnvironment(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IWindowingEnvironment(ptr, take_ownership_from_abi) {}
        static auto FindAll();
        static auto FindAll(Windows::UI::WindowManagement::WindowingEnvironmentKind const& kind);
    };
    struct __declspec(empty_bases) WindowingEnvironmentAddedEventArgs : Windows::UI::WindowManagement::IWindowingEnvironmentAddedEventArgs
    {
        WindowingEnvironmentAddedEventArgs(std::nullptr_t) noexcept {}
        WindowingEnvironmentAddedEventArgs(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IWindowingEnvironmentAddedEventArgs(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) WindowingEnvironmentChangedEventArgs : Windows::UI::WindowManagement::IWindowingEnvironmentChangedEventArgs
    {
        WindowingEnvironmentChangedEventArgs(std::nullptr_t) noexcept {}
        WindowingEnvironmentChangedEventArgs(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IWindowingEnvironmentChangedEventArgs(ptr, take_ownership_from_abi) {}
    };
    struct __declspec(empty_bases) WindowingEnvironmentRemovedEventArgs : Windows::UI::WindowManagement::IWindowingEnvironmentRemovedEventArgs
    {
        WindowingEnvironmentRemovedEventArgs(std::nullptr_t) noexcept {}
        WindowingEnvironmentRemovedEventArgs(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::WindowManagement::IWindowingEnvironmentRemovedEventArgs(ptr, take_ownership_from_abi) {}
    };
}
#endif
