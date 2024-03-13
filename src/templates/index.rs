/// Get the HTML content to render the index/root page.
///
/// # See Also
///
/// - This page is served as a response for the `/` entry point.
///
/// # Returns
///
/// A `String` version of the HTML, CSS and JS content.
pub fn get_content() -> String {
    r###"<!DOCTYPE html>
<!--suppress JSUnresolvedLibraryURL -->
<html lang="en">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <title>RuStream - Self-hosted Streaming Engine - v{{ version }}</title>
    <meta property="og:type" content="MediaStreaming">
    <meta name="keywords" content="Rust, streaming, actix, JavaScript, HTML, CSS">
    <meta name="author" content="Vignesh Rao">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <script src="https://code.jquery.com/jquery-3.6.4.min.js"></script>
    <script src="https://thevickypedia.github.io/open-source/crypto/crypto.js"></script>
    <!-- Favicon.ico and Apple Touch Icon -->
    <link rel="icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.ico">
    <link rel="apple-touch-icon" href="https://thevickypedia.github.io/open-source/images/logo/actix.png">
    <!-- Font Awesome icons -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/fontawesome.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/solid.min.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/regular.min.css">
    <style>
        @import url('https://fonts.googleapis.com/css2?family=Ubuntu:wght@400;500;700&display=swap');
        * {
            font-family: 'Ubuntu', 'PT Serif', sans-serif;
        }
        body {
            margin: 0;
            padding: 0;
            background-color: #151515;
        }

        .container {
            max-width: 400px;
            margin: 50px auto;
            background: #fff;
            padding: 20px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
            border-radius: 8px;
        }

        .header {
            text-align: center;
            margin-bottom: 20px;
        }

        .header img {
            max-width: 150px;
        }

        .content {
            display: flex;
            flex-direction: column;
        }

        form {
            display: flex;
            flex-direction: column;
        }

        label {
            margin-top: 10px;
            color: #000000;
            font-size: large;
            font-weight: normal;
        }

        input {
            padding: 10px;
            margin-bottom: 15px;
            border: 1px solid #ddd;
            border-radius: 5px;
        }

        button {
            padding: 12px;
            background-color: #000000;
            color: #fff;
            border: none;
            border-radius: 5px;
            cursor: pointer;
            font-weight: bold;
        }

        button:hover {
            background-color: #494949;
        }

    </style>
    <style>
        .password-container {
            position: relative;
        }
        .password-container input[type="password"],
        .password-container input[type="text"]{
            width: 100%;
            padding: 12px 36px 12px 12px;
            box-sizing: border-box;
        }
        .fa-eye, .fa-eye-slash {
            position: absolute;
            top: 40%;
            right: 4%;
            transform: translateY(-50%);
            cursor: pointer;
            color: lightgray;
        }
    </style>
    <noscript>
        <style>
            body {
                width: 100%;
                height: 100%;
                overflow: hidden;
            }
        </style>
        <div style="position: fixed; text-align:center; height: 100%; width: 100%; background-color: #151515;">
            <h2 style="margin-top:5%">This page requires JavaScript
                to be enabled.
                <br><br>
                Please refer <a href="https://www.enable-javascript.com/">enable-javascript</a> for how to.
            </h2>
            <form>
                <button type="submit" onClick="<meta httpEquiv='refresh' content='0'>">RETRY</button>
            </form>
        </div>
    </noscript>
</head>
<body>
<div class="container">
    <div class="header">
        <i class="fa-solid fa-user"></i>
    </div>
    <div class="content">
        <!-- <form action="{ url_for('signin') }" method="post"> -->
        <form>
            <label for="username">Username:</label>
            <input type="text" id="username" name="username" required>
            <label for="password">Password:</label>
            <div class="password-container">
                <input type="password" id="password" name="password" required>
                <i class="fa-regular fa-eye" id="eye"></i>
            </div>
            <button type="submit" onclick="submitToAPI(event)">Sign In</button>
        </form>
    </div>
</div>
</body>
<!-- control the behavior of the browser's navigation without triggering a full page reload -->
<script>
    document.addEventListener('DOMContentLoaded', function() {
        history.pushState(null, document.title, location.href);
        window.addEventListener('popstate', function (event) {
            history.pushState(null, document.title, location.href);
        });
    });
</script>
<!-- handle authentication from login page -->
<script>
    async function submitToAPI(event) {
        event.preventDefault();
        const username = $("#username").val();
        const password = $("#password").val();
        if (username === "" || password === "") {
            alert("ERROR: Username and password are required to authenticate your request!");
            return false;
        }
        async function ConvertStringToHex(str) {
            let arr = [];
            for (let i = 0; i < str.length; i++) {
                arr[i] = ("00" + str.charCodeAt(i).toString(16)).slice(-4);
            }
            return "\\u" + arr.join("\\u");
        }
        async function CalculateHash(username, password, timestamp) {
            const message = username + password + timestamp;
            const encoder = new TextEncoder();
            const data = encoder.encode(message);
            if (crypto.subtle === undefined) {
                const wordArray = CryptoJS.lib.WordArray.create(data);
                const hash = CryptoJS.SHA512(wordArray);
                // Convert the hash to a hexadecimal string and return it
                return hash.toString(CryptoJS.enc.Hex);
            } else {
                const hashBuffer = await crypto.subtle.digest('SHA-512', data);
                const hashArray = Array.from(new Uint8Array(hashBuffer));
                // Convert each byte to a hexadecimal string, pad with zeros, and join them to form the final hash
                return hashArray.map(byte => byte.toString(16).padStart(2, '0')).join('');
            }
        }
        let hex_user = await ConvertStringToHex(username);
        let hex_pass = await ConvertStringToHex(password);
        let timestamp = Math.round(new Date().getTime() / 1000);
        let hash = await CalculateHash(hex_user, hex_pass, timestamp)
        let authHeaderValue = hex_user + ',' + hash + ',' + timestamp;
        let origin = window.location.origin
        $.ajax({
            method: "POST",
            url: origin.concat("/login"),
            headers: {
                'accept': 'application/json',
                'Authorization': btoa(authHeaderValue)
            },
            crossDomain: "true",
            contentType: "application/json; charset=utf-8",
            success: function (data) {
                // Check if the response contains a redirect URL
                if (data.redirect_url) {
                    // Manually handle the redirect
                    window.location.href = data.redirect_url;
                } else {
                    console.log("Unhandled good response data")
                    // Handle success if needed
                    console.log(data);
                }
            },
            error: function(jqXHR, textStatus, errorThrown) {
                console.error(`Status: ${textStatus}, Error: ${errorThrown}`);
                if (jqXHR.hasOwnProperty("responseJSON")) {
                    alert(jqXHR.responseJSON.detail);
                } else {
                    alert(errorThrown);
                }
            }
        });
    }

</script>
<script>
    const passwordInput = document.querySelector("#password")
    const eye = document.querySelector("#eye")
    eye.addEventListener("click", function() {
        if (passwordInput.type === "password") {
            passwordInput.type = "text";
            eye.className = "fa-regular fa-eye-slash"
        } else {
            passwordInput.type = "password";
            eye.className = "fa-regular fa-eye"
        }
    });
</script>
</html>
"###.to_string()
}
