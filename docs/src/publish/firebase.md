# Firebase

## prop

### project

`type`: String
`value`: String

## auth

## setup

```js
<!-- The core Firebase JS SDK is always required and must be listed first -->
<script src="https://www.gstatic.com/firebasejs/7.14.6/firebase-app.js"></script>

<!-- TODO: Add SDKs for Firebase products that you want to use
     https://firebase.google.com/docs/web/setup#available-libraries -->
<script src="https://www.gstatic.com/firebasejs/7.14.6/firebase-analytics.js"></script>

<script>
  // Your web app's Firebase configuration
  var firebaseConfig = {
    apiKey: "AIzaSyAIn-DA1KrMa6jaIiR6w-EO7SQ9cMXffw8",
    authDomain: "wqms-fb.firebaseapp.com",
    databaseURL: "https://wqms-fb.firebaseio.com",
    projectId: "wqms-fb",
    storageBucket: "wqms-fb.appspot.com",
    messagingSenderId: "276128813099",
    appId: "1:276128813099:web:a190a252dcf7f1dc00da6e",
    measurementId: "G-8DB8K45X6Q"
  };
  // Initialize Firebase
  firebase.initializeApp(firebaseConfig);
  firebase.analytics();
</script>

```
