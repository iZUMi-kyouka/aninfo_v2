use crate::prelude::*;

#[function_component(PrivacyPolicy)]
pub fn privacy_policy() -> Html {
    let cx = use_context::<AppContext>().unwrap();
    use_title("Privacy Policy | ANiNFO".to_string());

    html! {
        <div class={format!("written-content {}", handle_theme(&cx))}>
        <h2 class="content-header">{"Privacy Policy for ANiNFO"}</h2>

        <p>{"ANiNFO respects your privacy and is committed to protecting any information you provide while using our application. This Privacy Policy outlines how ANiNFO handles information collected from users."}</p>

        <h3>{"Information Collection and Use"}</h3>

        <p>{"ANiNFO does not collect any personal user data. We do not require you to provide any personal information to use our application."}</p>

        <p>{"However, ANiNFO utilizes Google Fonts API to enhance the visual experience of our application. When you use ANiNFO, your device may automatically send certain information to Google, including, but not limited to, your device's IP address and the date and time of your request. This information is solely used by Google to deliver the requested font resources and is subject to Google's Privacy Policy."}</p>

        <h3>{"Children's Privacy"}</h3>

        <p>{"ANiNFO does not knowingly collect any personally identifiable information from children under the age of 13. If you are a parent or guardian and you are aware that your child has provided us with personal information, please contact us so that we can take appropriate action."}</p>

        <h3>{"Changes to This Privacy Policy"}</h3>

        <p>{"ANiNFO may update this Privacy Policy from time to time. Thus, you are advised to review this page periodically for any changes. We will notify you of any changes by posting the new Privacy Policy on this page. These changes are effective immediately after they are posted on this page."}</p>

        <h3>{"Contact Us"}</h3>

        <p>{"If you have any questions or suggestions about our Privacy Policy, do not hesitate to contact us at "}<a href={"mailto:contact@aninfo.com"}>{"contact@aninfo.com"}</a>{"."}</p>
        <br/>
        <p>{"This Privacy Policy was last updated on March 18, 2024."}</p>
        </div>
    }
}
