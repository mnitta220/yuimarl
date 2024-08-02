document.addEventListener("DOMContentLoaded", () => {
  fetch(`/api/firebaseConfig`, {
    method: "GET",
  })
    .then((response) => response.json())
    .then((ret) => {
      const firebaseConfig = {
        apiKey: ret.api_key,
        authDomain: ret.auth_domain,
        projectId: ret.project_id,
        storageBucket: ret.storage_bucket,
        messagingSenderId: ret.messaging_sender_id,
        appId: ret.app_id,
      };
      firebase.initializeApp(firebaseConfig);
      const ui = new firebaseui.auth.AuthUI(firebase.auth());
      const uiConfig = {
        callbacks: {
          signInSuccessWithAuthResult: function (authResult, _) {
            const displayName =
              document.querySelector(`#display_name`);
            if (displayName)
              displayName.value = authResult.user.multiFactor.user.displayName;
            const email = document.querySelector(`#email`);
            if (email) email.value = authResult.user.multiFactor.user.email;
            const photoUrl =
              document.querySelector(`#photo_url`);
            if (photoUrl)
              photoUrl.value = authResult.user.multiFactor.user.photoURL;
            const uid = document.querySelector(`#uid`);
            if (uid) uid.value = authResult.user.multiFactor.user.uid;
            const locale = document.querySelector(`#locale`);
            if (locale) locale.value = authResult.user.multiFactor.user.locale;
            const loginForm =
              document.querySelector(`#loginForm`);
            if (loginForm) loginForm.submit();
            return false;
          },
          uiShown: function () {
            const loader = document.querySelector(`#loader`);
            if (loader) loader.style.display = "none";
          },
        },
        signInFlow: "popup",
        signInSuccessUrl: "static/success.html",
        signInOptions: [
          {
            provider: firebase.auth.GoogleAuthProvider.PROVIDER_ID,
          },
          {
            provider: firebase.auth.GithubAuthProvider.PROVIDER_ID,
          },
          {
            provider: firebase.auth.EmailAuthProvider.PROVIDER_ID,
            signInMethod:
              firebase.auth.EmailAuthProvider.EMAIL_LINK_SIGN_IN_METHOD,
          },
        ],
        tosUrl: "https://mnitta220.github.io/yuimarl/agreement.html",
        privacyPolicyUrl: "https://mnitta220.github.io/privacy.html",
      };
      ui.start("#firebaseui-auth-container", uiConfig);
    })
    .catch((e) => window.alert(`エラーが発生しました。: ${e.message}`));
});

