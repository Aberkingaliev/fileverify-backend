pub fn activation_message(username: &String, activation_link: &String) -> String {
    return format!(
        "<html>
            <body>
            <h3>Dear {},</h3>\
                <p>Thank you for creating an account. Please click on the following link to activate your account:</p>\
                <p><a href=\"http://localhost:8000/api-v1/user/activate/{}\">http://localhost:8000/api-v1/user/activate/{}</a></p>\
                <p>Best regards,</p>\
                <p>The FileVerify Team</p>
            </body>
        </html>",
        &username, &activation_link, &activation_link
        );
}
