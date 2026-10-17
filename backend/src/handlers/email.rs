use crate::{AppState, error::AppError, tui::println_error, utils::send_email};

/// Sends the reservation status email to the given email address.
pub(super) async fn send_status_email(
    state: &AppState,
    email: &str,
    token: &str,
) -> Result<(), AppError> {
    let config = &state.config;
    let link = format!("{}{}", config.status_url, token);

    let subject = "予約状況の確認";
    let html_content = format!(
        r#"
            <p>ご利用ありがとうございます。</p>
            <p>以下のリンクから、予約状況や各企画のお呼び出し状況をご確認いただけます。<br/>
            <a href="{}">予約状況を確認する</a></p>
            <p>予約状況は随時更新されますので、必要に応じてご確認ください。</p>
            "#,
        link
    );

    send_email(
        config.resend_key.clone(),
        config.email.clone(),
        email.to_owned(),
        subject.to_owned(),
        html_content,
    )
    .await
    .map_err(|e| {
        println_error(format!("Failed to send status email: {e}"));
        AppError::Internal
    })?;

    Ok(())
}

pub(super) async fn send_signin_email(
    state: &AppState,
    email: &str,
    token: &str,
) -> Result<(), AppError> {
    let config = &state.config;
    let link = format!("{}{}", config.status_url, token);

    let subject = "予約管理システムへのサインイン";
    let html_content = format!(
        r#"
        <p>予約管理システムの管理画面へのサインインはこちらから行えます。
        <a href="{}">サインイン</a></p>
        <p>このリンクは発行から10分間有効です。</p>
        <p>このメールに心当たりがない場合は、リンクを開かずに破棄してください。</p>
        "#,
        link
    );

    send_email(
        config.resend_key.clone(),
        config.email.clone(),
        email.to_owned(),
        subject.to_owned(),
        html_content,
    )
    .await
    .map_err(|e| {
        println_error(format!("Failed to send status email: {e}"));
        AppError::Internal
    })?;

    Ok(())
}
