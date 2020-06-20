winrt::import!(
    dependencies
        os
    types
        windows::ui::*
        windows::ui::notifications::*
);

use windows::ui::notifications::{
    ToastNotificationManager, ToastTemplateType, ToastNotification,
};

fn main() -> winrt::Result<()> {
    let notification_manager = ToastNotificationManager::get_default()?;

    let toast_xml = ToastNotificationManager::get_template_content(ToastTemplateType::ToastText01)?;

    // 通知の内容を設定する
    let text_node = toast_xml.get_elements_by_tag_name("text")?.item(0)?;
    let text = toast_xml.create_text_node(r"Hello from Rust/WinRT!")?;
    text_node.append_child(winrt::Param::Owned(text.into()))?;

    // // 通知のアイコンを設定したい
    // let image = toast_xml.get_elements_by_tag_name("image")?.item(0)?;
    // image.set_attribute("src", "https://xxx.yyy.zzz/image.png");
    // image.set_attribute("placement", "appLogoOverride");
    // toast_xml.document_element.append_child(image);

    // トースト通知を表示する
    // v1709 以降は AppId が必要なので, Powershell のものを使う
    let toast = ToastNotification::create_toast_notification(toast_xml)?;
    notification_manager
        .create_toast_notifier_with_id("{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe")?
        .show(toast)?;

    // この行を消すとなぜかトースト通知が表示されないらしい
    std::thread::sleep(std::time::Duration::from_millis(10));
    Ok(())
}
