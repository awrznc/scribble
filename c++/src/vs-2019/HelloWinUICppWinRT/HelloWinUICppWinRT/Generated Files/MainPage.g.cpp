// WARNING: Please don't edit this file. It was generated by C++/WinRT v2.0.200703.9

void* winrt_make_HelloWinUICppWinRT_MainPage()
{
    return winrt::detach_abi(winrt::make<winrt::HelloWinUICppWinRT::factory_implementation::MainPage>());
}
WINRT_EXPORT namespace winrt::HelloWinUICppWinRT
{
    MainPage::MainPage() :
        MainPage(make<HelloWinUICppWinRT::implementation::MainPage>())
    {
    }
}
