// WARNING: Please don't edit this file. It was generated by C++/WinRT v2.0.200703.9

#ifndef WINRT_HelloWinUICppWinRT_2_H
#define WINRT_HelloWinUICppWinRT_2_H
#include "winrt/impl/Windows.UI.Composition.1.h"
#include "winrt/impl/Windows.UI.Xaml.1.h"
#include "winrt/impl/Windows.UI.Xaml.Controls.1.h"
#include "winrt/impl/Windows.UI.Xaml.Markup.1.h"
#include "winrt/impl/HelloWinUICppWinRT.1.h"
WINRT_EXPORT namespace winrt::HelloWinUICppWinRT
{
    struct __declspec(empty_bases) MainPage : HelloWinUICppWinRT::IMainPage,
        impl::base<MainPage, Windows::UI::Xaml::Controls::Page, Windows::UI::Xaml::Controls::UserControl, Windows::UI::Xaml::Controls::Control, Windows::UI::Xaml::FrameworkElement, Windows::UI::Xaml::UIElement, Windows::UI::Xaml::DependencyObject>,
        impl::require<MainPage, Windows::UI::Xaml::Controls::IPage, Windows::UI::Xaml::Controls::IPageOverrides, Windows::UI::Xaml::Controls::IUserControl, Windows::UI::Xaml::Controls::IControl, Windows::UI::Xaml::Controls::IControl2, Windows::UI::Xaml::Controls::IControl3, Windows::UI::Xaml::Controls::IControl4, Windows::UI::Xaml::Controls::IControl5, Windows::UI::Xaml::Controls::IControl7, Windows::UI::Xaml::Controls::IControlProtected, Windows::UI::Xaml::Controls::IControlOverrides, Windows::UI::Xaml::Controls::IControlOverrides6, Windows::UI::Xaml::IFrameworkElement, Windows::UI::Xaml::IFrameworkElement2, Windows::UI::Xaml::IFrameworkElement3, Windows::UI::Xaml::IFrameworkElement4, Windows::UI::Xaml::IFrameworkElement6, Windows::UI::Xaml::IFrameworkElement7, Windows::UI::Xaml::IFrameworkElementProtected7, Windows::UI::Xaml::IFrameworkElementOverrides, Windows::UI::Xaml::IFrameworkElementOverrides2, Windows::UI::Xaml::IUIElement, Windows::UI::Xaml::IUIElement2, Windows::UI::Xaml::IUIElement3, Windows::UI::Xaml::IUIElement4, Windows::UI::Xaml::IUIElement5, Windows::UI::Xaml::IUIElement7, Windows::UI::Xaml::IUIElement8, Windows::UI::Xaml::IUIElement9, Windows::UI::Xaml::IUIElement10, Windows::UI::Xaml::IUIElementOverrides, Windows::UI::Xaml::IUIElementOverrides7, Windows::UI::Xaml::IUIElementOverrides8, Windows::UI::Xaml::IUIElementOverrides9, Windows::UI::Composition::IAnimationObject, Windows::UI::Composition::IVisualElement, Windows::UI::Xaml::IDependencyObject, Windows::UI::Xaml::IDependencyObject2>
    {
        MainPage(std::nullptr_t) noexcept {}
        MainPage(void* ptr, take_ownership_from_abi_t) noexcept : HelloWinUICppWinRT::IMainPage(ptr, take_ownership_from_abi) {}
        MainPage();
    };
    struct __declspec(empty_bases) XamlMetaDataProvider : Windows::UI::Xaml::Markup::IXamlMetadataProvider
    {
        XamlMetaDataProvider(std::nullptr_t) noexcept {}
        XamlMetaDataProvider(void* ptr, take_ownership_from_abi_t) noexcept : Windows::UI::Xaml::Markup::IXamlMetadataProvider(ptr, take_ownership_from_abi) {}
        XamlMetaDataProvider();
    };
}
#endif
