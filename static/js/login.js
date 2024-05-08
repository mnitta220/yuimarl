$(document).ready(function () {
  $.ajax({
    type: "GET",
    url: "/api/firebaseConfig",
    success: function (result) {
      var ret = JSON.parse(result);
      const firebaseConfig = {
        apiKey: ret.api_key,
        authDomain: ret.auth_domain,
        projectId: ret.project_id,
        storageBucket: ret.storage_bucket,
        messagingSenderId: ret.messaging_sender_id,
        appId: ret.app_id,
        measurementId: ret.measurement_id,
      };
      firebase.initializeApp(firebaseConfig);
      const ui = new firebaseui.auth.AuthUI(firebase.auth());
      const uiConfig = {
        callbacks: {
          signInSuccessWithAuthResult: function (authResult, redirectUrl) {
            $('#display_name').val(authResult.user.multiFactor.user.displayName);
            $('#email').val(authResult.user.multiFactor.user.email);
            $('#photo_url').val(authResult.user.multiFactor.user.photoURL);
            $('#uid').val(authResult.user.multiFactor.user.uid);
            $('#locale').val(authResult.user.multiFactor.user.locale);
            $('#loginForm').submit();
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
          firebase.auth.EmailAuthProvider.PROVIDER_ID,
        ],
        tosUrl: 'static/404.html',
        privacyPolicyUrl: '/static/404.html'
      };
      ui.start('#firebaseui-auth-container', uiConfig);
    }
  });
});
