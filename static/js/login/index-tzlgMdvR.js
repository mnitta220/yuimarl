(function(){const o=document.createElement("link").relList;if(o&&o.supports&&o.supports("modulepreload"))return;for(const e of document.querySelectorAll('link[rel="modulepreload"]'))s(e);new MutationObserver(e=>{for(const t of e)if(t.type==="childList")for(const i of t.addedNodes)i.tagName==="LINK"&&i.rel==="modulepreload"&&s(i)}).observe(document,{childList:!0,subtree:!0});function n(e){const t={};return e.integrity&&(t.integrity=e.integrity),e.referrerPolicy&&(t.referrerPolicy=e.referrerPolicy),e.crossOrigin==="use-credentials"?t.credentials="include":e.crossOrigin==="anonymous"?t.credentials="omit":t.credentials="same-origin",t}function s(e){if(e.ep)return;e.ep=!0;const t=n(e);fetch(e.href,t)}})();document.addEventListener("DOMContentLoaded",()=>{fetch("/api/firebaseConfig",{method:"GET"}).then(r=>r.json()).then(r=>{const o={apiKey:r.api_key,authDomain:r.auth_domain,projectId:r.project_id,storageBucket:r.storage_bucket,messagingSenderId:r.messaging_sender_id,appId:r.app_id};firebase.initializeApp(o);const n=new firebaseui.auth.AuthUI(firebase.auth()),s={callbacks:{signInSuccessWithAuthResult:function(e,t){const i=document.querySelector("#display_name");i&&(i.value=e.user.multiFactor.user.displayName);const u=document.querySelector("#email");u&&(u.value=e.user.multiFactor.user.email);const a=document.querySelector("#photo_url");a&&(a.value=e.user.multiFactor.user.photoURL);const c=document.querySelector("#uid");c&&(c.value=e.user.multiFactor.user.uid);const l=document.querySelector("#locale");l&&(l.value=e.user.multiFactor.user.locale);const d=document.querySelector("#loginForm");return d&&d.submit(),!1},uiShown:function(){const e=document.querySelector("#loader");e&&(e.style.display="none")}},signInFlow:"popup",signInSuccessUrl:"static/success.html",signInOptions:[{provider:firebase.auth.GoogleAuthProvider.PROVIDER_ID},{provider:firebase.auth.GithubAuthProvider.PROVIDER_ID},{provider:firebase.auth.EmailAuthProvider.PROVIDER_ID,signInMethod:firebase.auth.EmailAuthProvider.EMAIL_LINK_SIGN_IN_METHOD}],tosUrl:"https://mnitta220.github.io/yuimarl/agreement.html",privacyPolicyUrl:"https://mnitta220.github.io/privacy.html"};n.start("#firebaseui-auth-container",s)}).catch(r=>window.alert(`エラーが発生しました。: ${r.message}`))});
