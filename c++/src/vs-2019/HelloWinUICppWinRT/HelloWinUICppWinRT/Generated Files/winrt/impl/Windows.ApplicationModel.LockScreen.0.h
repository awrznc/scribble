// WARNING: Please don't edit this file. It was generated by C++/WinRT v2.0.200703.9

#ifndef WINRT_Windows_ApplicationModel_LockScreen_0_H
#define WINRT_Windows_ApplicationModel_LockScreen_0_H
WINRT_EXPORT namespace winrt::Windows::Foundation
{
    struct EventRegistrationToken;
    template <typename T> struct __declspec(empty_bases) IReference;
    template <typename TSender, typename TResult> struct __declspec(empty_bases) TypedEventHandler;
}
WINRT_EXPORT namespace winrt::Windows::Foundation::Collections
{
    template <typename T> struct __declspec(empty_bases) IVectorView;
}
WINRT_EXPORT namespace winrt::Windows::Storage::Streams
{
    struct IRandomAccessStream;
}
WINRT_EXPORT namespace winrt::Windows::ApplicationModel::LockScreen
{
    struct ILockApplicationHost;
    struct ILockApplicationHostStatics;
    struct ILockScreenBadge;
    struct ILockScreenInfo;
    struct ILockScreenUnlockingDeferral;
    struct ILockScreenUnlockingEventArgs;
    struct LockApplicationHost;
    struct LockScreenBadge;
    struct LockScreenInfo;
    struct LockScreenUnlockingDeferral;
    struct LockScreenUnlockingEventArgs;
}
namespace winrt::impl
{
    template <> struct category<Windows::ApplicationModel::LockScreen::ILockApplicationHost>{ using type = interface_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::ILockApplicationHostStatics>{ using type = interface_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::ILockScreenBadge>{ using type = interface_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::ILockScreenInfo>{ using type = interface_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingDeferral>{ using type = interface_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingEventArgs>{ using type = interface_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::LockApplicationHost>{ using type = class_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::LockScreenBadge>{ using type = class_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::LockScreenInfo>{ using type = class_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::LockScreenUnlockingDeferral>{ using type = class_category; };
    template <> struct category<Windows::ApplicationModel::LockScreen::LockScreenUnlockingEventArgs>{ using type = class_category; };
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::LockApplicationHost> = L"Windows.ApplicationModel.LockScreen.LockApplicationHost";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::LockScreenBadge> = L"Windows.ApplicationModel.LockScreen.LockScreenBadge";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::LockScreenInfo> = L"Windows.ApplicationModel.LockScreen.LockScreenInfo";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::LockScreenUnlockingDeferral> = L"Windows.ApplicationModel.LockScreen.LockScreenUnlockingDeferral";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::LockScreenUnlockingEventArgs> = L"Windows.ApplicationModel.LockScreen.LockScreenUnlockingEventArgs";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::ILockApplicationHost> = L"Windows.ApplicationModel.LockScreen.ILockApplicationHost";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::ILockApplicationHostStatics> = L"Windows.ApplicationModel.LockScreen.ILockApplicationHostStatics";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::ILockScreenBadge> = L"Windows.ApplicationModel.LockScreen.ILockScreenBadge";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::ILockScreenInfo> = L"Windows.ApplicationModel.LockScreen.ILockScreenInfo";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingDeferral> = L"Windows.ApplicationModel.LockScreen.ILockScreenUnlockingDeferral";
    template <> inline constexpr auto& name_v<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingEventArgs> = L"Windows.ApplicationModel.LockScreen.ILockScreenUnlockingEventArgs";
    template <> inline constexpr guid guid_v<Windows::ApplicationModel::LockScreen::ILockApplicationHost>{ 0x38EE31AD,0xD94F,0x4E7C,{ 0x81,0xFA,0x4F,0x44,0x36,0x50,0x62,0x81 } }; // 38EE31AD-D94F-4E7C-81FA-4F4436506281
    template <> inline constexpr guid guid_v<Windows::ApplicationModel::LockScreen::ILockApplicationHostStatics>{ 0xF48FAB8E,0x23D7,0x4E63,{ 0x96,0xA1,0x66,0x6F,0xF5,0x2D,0x3B,0x2C } }; // F48FAB8E-23D7-4E63-96A1-666FF52D3B2C
    template <> inline constexpr guid guid_v<Windows::ApplicationModel::LockScreen::ILockScreenBadge>{ 0xE95105D9,0x2BFF,0x4DB0,{ 0x9B,0x4F,0x38,0x24,0x77,0x8B,0x9C,0x9A } }; // E95105D9-2BFF-4DB0-9B4F-3824778B9C9A
    template <> inline constexpr guid guid_v<Windows::ApplicationModel::LockScreen::ILockScreenInfo>{ 0xF59AA65C,0x9711,0x4DC9,{ 0xA6,0x30,0x95,0xB6,0xCB,0x8C,0xDA,0xD0 } }; // F59AA65C-9711-4DC9-A630-95B6CB8CDAD0
    template <> inline constexpr guid guid_v<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingDeferral>{ 0x7E7D1AD6,0x5203,0x43E7,{ 0x9B,0xD6,0x7C,0x39,0x47,0xD1,0xE3,0xFE } }; // 7E7D1AD6-5203-43E7-9BD6-7C3947D1E3FE
    template <> inline constexpr guid guid_v<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingEventArgs>{ 0x44E6C007,0x75FB,0x4ABB,{ 0x9F,0x8B,0x82,0x47,0x48,0x90,0x0C,0x71 } }; // 44E6C007-75FB-4ABB-9F8B-824748900C71
    template <> struct default_interface<Windows::ApplicationModel::LockScreen::LockApplicationHost>{ using type = Windows::ApplicationModel::LockScreen::ILockApplicationHost; };
    template <> struct default_interface<Windows::ApplicationModel::LockScreen::LockScreenBadge>{ using type = Windows::ApplicationModel::LockScreen::ILockScreenBadge; };
    template <> struct default_interface<Windows::ApplicationModel::LockScreen::LockScreenInfo>{ using type = Windows::ApplicationModel::LockScreen::ILockScreenInfo; };
    template <> struct default_interface<Windows::ApplicationModel::LockScreen::LockScreenUnlockingDeferral>{ using type = Windows::ApplicationModel::LockScreen::ILockScreenUnlockingDeferral; };
    template <> struct default_interface<Windows::ApplicationModel::LockScreen::LockScreenUnlockingEventArgs>{ using type = Windows::ApplicationModel::LockScreen::ILockScreenUnlockingEventArgs; };
    template <> struct abi<Windows::ApplicationModel::LockScreen::ILockApplicationHost>
    {
        struct __declspec(novtable) type : inspectable_abi
        {
            virtual int32_t __stdcall RequestUnlock() noexcept = 0;
            virtual int32_t __stdcall add_Unlocking(void*, winrt::event_token*) noexcept = 0;
            virtual int32_t __stdcall remove_Unlocking(winrt::event_token) noexcept = 0;
        };
    };
    template <> struct abi<Windows::ApplicationModel::LockScreen::ILockApplicationHostStatics>
    {
        struct __declspec(novtable) type : inspectable_abi
        {
            virtual int32_t __stdcall GetForCurrentView(void**) noexcept = 0;
        };
    };
    template <> struct abi<Windows::ApplicationModel::LockScreen::ILockScreenBadge>
    {
        struct __declspec(novtable) type : inspectable_abi
        {
            virtual int32_t __stdcall get_Logo(void**) noexcept = 0;
            virtual int32_t __stdcall get_Glyph(void**) noexcept = 0;
            virtual int32_t __stdcall get_Number(void**) noexcept = 0;
            virtual int32_t __stdcall get_AutomationName(void**) noexcept = 0;
            virtual int32_t __stdcall LaunchApp() noexcept = 0;
        };
    };
    template <> struct abi<Windows::ApplicationModel::LockScreen::ILockScreenInfo>
    {
        struct __declspec(novtable) type : inspectable_abi
        {
            virtual int32_t __stdcall add_LockScreenImageChanged(void*, winrt::event_token*) noexcept = 0;
            virtual int32_t __stdcall remove_LockScreenImageChanged(winrt::event_token) noexcept = 0;
            virtual int32_t __stdcall get_LockScreenImage(void**) noexcept = 0;
            virtual int32_t __stdcall add_BadgesChanged(void*, winrt::event_token*) noexcept = 0;
            virtual int32_t __stdcall remove_BadgesChanged(winrt::event_token) noexcept = 0;
            virtual int32_t __stdcall get_Badges(void**) noexcept = 0;
            virtual int32_t __stdcall add_DetailTextChanged(void*, winrt::event_token*) noexcept = 0;
            virtual int32_t __stdcall remove_DetailTextChanged(winrt::event_token) noexcept = 0;
            virtual int32_t __stdcall get_DetailText(void**) noexcept = 0;
            virtual int32_t __stdcall add_AlarmIconChanged(void*, winrt::event_token*) noexcept = 0;
            virtual int32_t __stdcall remove_AlarmIconChanged(winrt::event_token) noexcept = 0;
            virtual int32_t __stdcall get_AlarmIcon(void**) noexcept = 0;
        };
    };
    template <> struct abi<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingDeferral>
    {
        struct __declspec(novtable) type : inspectable_abi
        {
            virtual int32_t __stdcall Complete() noexcept = 0;
        };
    };
    template <> struct abi<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingEventArgs>
    {
        struct __declspec(novtable) type : inspectable_abi
        {
            virtual int32_t __stdcall GetDeferral(void**) noexcept = 0;
            virtual int32_t __stdcall get_Deadline(int64_t*) noexcept = 0;
        };
    };
    template <typename D>
    struct consume_Windows_ApplicationModel_LockScreen_ILockApplicationHost
    {
        WINRT_IMPL_AUTO(void) RequestUnlock() const;
        WINRT_IMPL_AUTO(winrt::event_token) Unlocking(Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockApplicationHost, Windows::ApplicationModel::LockScreen::LockScreenUnlockingEventArgs> const& handler) const;
        using Unlocking_revoker = impl::event_revoker<Windows::ApplicationModel::LockScreen::ILockApplicationHost, &impl::abi_t<Windows::ApplicationModel::LockScreen::ILockApplicationHost>::remove_Unlocking>;
        [[nodiscard]] Unlocking_revoker Unlocking(auto_revoke_t, Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockApplicationHost, Windows::ApplicationModel::LockScreen::LockScreenUnlockingEventArgs> const& handler) const;
        WINRT_IMPL_AUTO(void) Unlocking(winrt::event_token const& token) const noexcept;
    };
    template <> struct consume<Windows::ApplicationModel::LockScreen::ILockApplicationHost>
    {
        template <typename D> using type = consume_Windows_ApplicationModel_LockScreen_ILockApplicationHost<D>;
    };
    template <typename D>
    struct consume_Windows_ApplicationModel_LockScreen_ILockApplicationHostStatics
    {
        WINRT_IMPL_AUTO(Windows::ApplicationModel::LockScreen::LockApplicationHost) GetForCurrentView() const;
    };
    template <> struct consume<Windows::ApplicationModel::LockScreen::ILockApplicationHostStatics>
    {
        template <typename D> using type = consume_Windows_ApplicationModel_LockScreen_ILockApplicationHostStatics<D>;
    };
    template <typename D>
    struct consume_Windows_ApplicationModel_LockScreen_ILockScreenBadge
    {
        [[nodiscard]] WINRT_IMPL_AUTO(Windows::Storage::Streams::IRandomAccessStream) Logo() const;
        [[nodiscard]] WINRT_IMPL_AUTO(Windows::Storage::Streams::IRandomAccessStream) Glyph() const;
        [[nodiscard]] WINRT_IMPL_AUTO(Windows::Foundation::IReference<uint32_t>) Number() const;
        [[nodiscard]] WINRT_IMPL_AUTO(hstring) AutomationName() const;
        WINRT_IMPL_AUTO(void) LaunchApp() const;
    };
    template <> struct consume<Windows::ApplicationModel::LockScreen::ILockScreenBadge>
    {
        template <typename D> using type = consume_Windows_ApplicationModel_LockScreen_ILockScreenBadge<D>;
    };
    template <typename D>
    struct consume_Windows_ApplicationModel_LockScreen_ILockScreenInfo
    {
        WINRT_IMPL_AUTO(winrt::event_token) LockScreenImageChanged(Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockScreenInfo, Windows::Foundation::IInspectable> const& handler) const;
        using LockScreenImageChanged_revoker = impl::event_revoker<Windows::ApplicationModel::LockScreen::ILockScreenInfo, &impl::abi_t<Windows::ApplicationModel::LockScreen::ILockScreenInfo>::remove_LockScreenImageChanged>;
        [[nodiscard]] LockScreenImageChanged_revoker LockScreenImageChanged(auto_revoke_t, Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockScreenInfo, Windows::Foundation::IInspectable> const& handler) const;
        WINRT_IMPL_AUTO(void) LockScreenImageChanged(winrt::event_token const& token) const noexcept;
        [[nodiscard]] WINRT_IMPL_AUTO(Windows::Storage::Streams::IRandomAccessStream) LockScreenImage() const;
        WINRT_IMPL_AUTO(winrt::event_token) BadgesChanged(Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockScreenInfo, Windows::Foundation::IInspectable> const& handler) const;
        using BadgesChanged_revoker = impl::event_revoker<Windows::ApplicationModel::LockScreen::ILockScreenInfo, &impl::abi_t<Windows::ApplicationModel::LockScreen::ILockScreenInfo>::remove_BadgesChanged>;
        [[nodiscard]] BadgesChanged_revoker BadgesChanged(auto_revoke_t, Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockScreenInfo, Windows::Foundation::IInspectable> const& handler) const;
        WINRT_IMPL_AUTO(void) BadgesChanged(winrt::event_token const& token) const noexcept;
        [[nodiscard]] WINRT_IMPL_AUTO(Windows::Foundation::Collections::IVectorView<Windows::ApplicationModel::LockScreen::LockScreenBadge>) Badges() const;
        WINRT_IMPL_AUTO(winrt::event_token) DetailTextChanged(Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockScreenInfo, Windows::Foundation::IInspectable> const& handler) const;
        using DetailTextChanged_revoker = impl::event_revoker<Windows::ApplicationModel::LockScreen::ILockScreenInfo, &impl::abi_t<Windows::ApplicationModel::LockScreen::ILockScreenInfo>::remove_DetailTextChanged>;
        [[nodiscard]] DetailTextChanged_revoker DetailTextChanged(auto_revoke_t, Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockScreenInfo, Windows::Foundation::IInspectable> const& handler) const;
        WINRT_IMPL_AUTO(void) DetailTextChanged(winrt::event_token const& token) const noexcept;
        [[nodiscard]] WINRT_IMPL_AUTO(Windows::Foundation::Collections::IVectorView<hstring>) DetailText() const;
        WINRT_IMPL_AUTO(winrt::event_token) AlarmIconChanged(Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockScreenInfo, Windows::Foundation::IInspectable> const& handler) const;
        using AlarmIconChanged_revoker = impl::event_revoker<Windows::ApplicationModel::LockScreen::ILockScreenInfo, &impl::abi_t<Windows::ApplicationModel::LockScreen::ILockScreenInfo>::remove_AlarmIconChanged>;
        [[nodiscard]] AlarmIconChanged_revoker AlarmIconChanged(auto_revoke_t, Windows::Foundation::TypedEventHandler<Windows::ApplicationModel::LockScreen::LockScreenInfo, Windows::Foundation::IInspectable> const& handler) const;
        WINRT_IMPL_AUTO(void) AlarmIconChanged(winrt::event_token const& token) const noexcept;
        [[nodiscard]] WINRT_IMPL_AUTO(Windows::Storage::Streams::IRandomAccessStream) AlarmIcon() const;
    };
    template <> struct consume<Windows::ApplicationModel::LockScreen::ILockScreenInfo>
    {
        template <typename D> using type = consume_Windows_ApplicationModel_LockScreen_ILockScreenInfo<D>;
    };
    template <typename D>
    struct consume_Windows_ApplicationModel_LockScreen_ILockScreenUnlockingDeferral
    {
        WINRT_IMPL_AUTO(void) Complete() const;
    };
    template <> struct consume<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingDeferral>
    {
        template <typename D> using type = consume_Windows_ApplicationModel_LockScreen_ILockScreenUnlockingDeferral<D>;
    };
    template <typename D>
    struct consume_Windows_ApplicationModel_LockScreen_ILockScreenUnlockingEventArgs
    {
        WINRT_IMPL_AUTO(Windows::ApplicationModel::LockScreen::LockScreenUnlockingDeferral) GetDeferral() const;
        [[nodiscard]] WINRT_IMPL_AUTO(Windows::Foundation::DateTime) Deadline() const;
    };
    template <> struct consume<Windows::ApplicationModel::LockScreen::ILockScreenUnlockingEventArgs>
    {
        template <typename D> using type = consume_Windows_ApplicationModel_LockScreen_ILockScreenUnlockingEventArgs<D>;
    };
}
#endif
