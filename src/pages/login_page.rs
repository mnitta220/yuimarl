/// Component for rendering the login page
pub struct LoginPage {}

impl LoginPage {
    pub fn write() -> String {
        let mut buf: String = r#"<!DOCTYPE html>
        <html>
        <head>
          <meta charset="utf-8">
          <meta name="viewport" content="width=device-width, initial-scale=1">
          <title>Yuimarl ログイン</title>
          <script src="https://www.gstatic.com/firebasejs/9.9.2/firebase-app-compat.js"></script>
          <script src="https://www.gstatic.com/firebasejs/9.9.2/firebase-auth-compat.js"></script>
          <script src="https://www.gstatic.com/firebasejs/ui/6.1.0/firebase-ui-auth__ja.js"></script>
          <link type="text/css" rel="stylesheet" href="https://www.gstatic.com/firebasejs/ui/6.1.0/firebase-ui-auth.css" />
          <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
        </head>
        <body>
          <h1 style="text-align: center">Yuimarl ログイン</h1>
          <div id="firebaseui-auth-container"></div>
          <div id="loader">Loading...</div>
          <form id="loginForm" method="POST" action="/login">
            <input type=hidden id="display_name" name="display_name">
            <input type=hidden id="email" name="email">
            <input type=hidden id="photo_url" name="photo_url">
            <input type=hidden id="uid" name="uid">
            <input type=hidden id="locale" name="locale">
          </form>
          "#.to_string();

        buf += r#"
          <script>
            const firebaseConfig = {
              apiKey: ""#;
        buf += crate::API_KEY.get().unwrap();
        buf += r#"", authDomain: ""#;
        buf += crate::AUTH_DOMAIN.get().unwrap();
        buf += r#"", projectId: ""#;
        buf += crate::GOOGLE_PROJECT_ID.get().unwrap();
        buf += r#"", storageBucket: ""#;
        buf += crate::STORAGE_BUCKET.get().unwrap();
        buf += r#"", messagingSenderId: ""#;
        buf += crate::MESSAGING_SENDER_ID.get().unwrap();
        buf += r#"", appId: ""#;
        buf += crate::APP_ID.get().unwrap();
        buf += r#"", measurementId: ""#;
        buf += crate::MEASUREMENT_ID.get().unwrap();
        buf += r#""};"#;

        buf += r#"
            firebase.initializeApp(firebaseConfig);
            const ui = new firebaseui.auth.AuthUI(firebase.auth());
            const uiConfig = {
              callbacks: {
                signInSuccessWithAuthResult: function (authResult, redirectUrl) {
                  //console.log("***authResult: ", authResult);
                  let displayName = document.getElementById('display_name');
                  displayName.value = authResult.user.multiFactor.user.displayName;
                  let email = document.getElementById('email');
                  email.value = authResult.user.multiFactor.user.email;
                  let photoURL = document.getElementById('photo_url');
                  photoURL.value = authResult.user.multiFactor.user.photoURL;
                  let uid = document.getElementById('uid');
                  uid.value = authResult.user.multiFactor.user.uid;
                  let locale = document.getElementById('locale');
                  locale.value = authResult.additionalUserInfo.profile.locale;
                  let loginForm = document.getElementById('loginForm');
                  loginForm.submit();
                  return false;
                },
                uiShown: function () {
                  document.getElementById('loader').style.display = 'none';
                }
              },
              signInFlow: 'popup',
              signInSuccessUrl: 'static/success.html',
              signInOptions: [
                firebase.auth.GoogleAuthProvider.PROVIDER_ID,
                firebase.auth.FacebookAuthProvider.PROVIDER_ID,
                firebase.auth.EmailAuthProvider.PROVIDER_ID,
              ],
              tosUrl: 'static/404.html',
              privacyPolicyUrl: '/static/404.html'
            };
            ui.start('#firebaseui-auth-container', uiConfig);
          </script>
        </body>
        </html>"#;

        buf
    }
}
